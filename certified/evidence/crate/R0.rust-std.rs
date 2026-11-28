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
    F77(u8, Bytes),
    F190(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F77(k0, k1) => f77(*k0, k1.clone(), p0),
            Fn0::F190(k0) => f190(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F121,
    F243,
    F358,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F121 => f121(p0, p1),
            Fn1::F243 => f243(p0, p1),
            Fn1::F358 => f358(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F213(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F213(k0) => f213(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F327(Bytes),
    F328(Bytes),
    F430(Bytes),
    F431(u64, Bytes),
    F438(Bytes),
    F446(Bytes, u64, Bytes, u64),
    F510(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F327(k0) => f327(k0.clone(), p0),
            Fn3::F328(k0) => f328(k0.clone(), p0),
            Fn3::F430(k0) => Ok(f430(k0.clone(), p0)),
            Fn3::F431(k0, k1) => f431(*k0, k1.clone(), p0),
            Fn3::F438(k0) => f438(k0.clone(), p0),
            Fn3::F446(k0, k1, k2, k3) => f446(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F510(k0) => f510(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F331(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F331(k0) => f331(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F422,
    F617,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F422 => f422(p0, p1),
            Fn5::F617 => f617(p0, p1),
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
    F490(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt23)) -> R<(u64, Adt23)> {
        match self {
            Fn11::F490(k0, k1, k2) => f490(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F521,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F521 => f521(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F524,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F524 => f524(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F546(u64, Bytes),
    F557(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn14::F546(k0, k1) => Ok(f546(*k0, k1.clone(), p0)),
            Fn14::F557(k0) => f557(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F554,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F554 => f554(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F556,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F556 => f556(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F560(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F560(k0) => f560(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F569,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn18::F569 => f569(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F576,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F576 => f576(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F581,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F581 => f581(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F600,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F600 => f600(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F603,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F603 => f603(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F610,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F610 => f610(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F615,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F615 => f615(p0),
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
        Adt12::C2(f8(Str::lit("bash")), f137(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f51(f52(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h463, _) => {
                h463
            }
        })) {
            Adt12::C2(f8(Str::lit("bash")), f137(Str::lit("build")), f8(Str::lit("prism-code build")))
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
                    Ok(Adt12::C3(f154(f155(v0.clone())?, List::cons((f156(), f157()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f158()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f159()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f160()), List::cons((f8(Str::lit(".github/scripts/readmit.sh")), f161()), List::cons((f8(Str::lit(".github/scripts/merge-entries.py")), f162()), List::cons((f8(Str::lit(".github/scripts/prism-acceptance.py")), f163()), List::cons((f8(Str::lit(".github/scripts/prepare-change.py")), f164()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f165()), List::cons((f8(Str::lit(".github/lexlean.pin")), f166()), List::cons((f8(Str::lit("docs/catalog.md")), f167(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f168(v0.clone())?), List::cons((f8(Str::lit("app/src/Studio/Prism/Closure/Register.lex.tex")), f169(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f170(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f171()), List::cons((f8(Str::lit("surface/mcp-tools.json")), f172()?), List::cons((f8(Str::lit(".github/scripts/surface-check.py")), f173()), List::cons((f8(Str::lit(".github/scripts/gui-tree.py")), f174()), List::<(Bytes, Bytes)>::nil()))))))))))))))))))), f8(Str::lit("published the workflows, the export, readmit, entry-merge, change-preparing, Prism acceptance and surface-check and GUI-tree scripts, CODEOWNERS, the lexlean pin, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md, docs/owner-decisions.md, the Prism closure register and the MCP tools of surface/mcp-tools.json\u{a}"))))
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
        f175(v3.clone(), v1.clone(), f176(v3, v1, false)?)
    } else {
        if f32(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f175(v3.clone(), v1.clone(), f177(v3, v1)?)
        } else {
            if f32(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f175(v3.clone(), v1.clone(), f176(v3, v1, true)?)
            } else {
                if f32(v0.clone(), f8(Str::lit("stage-registries"))) {
                    f175(v3.clone(), v1.clone(), f178(v3, v1)?)
                } else {
                    if f32(v0.clone(), f8(Str::lit("stage-encodings"))) {
                        f175(v3.clone(), v1.clone(), f179(v3, v1)?)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("stage-mappings"))) {
                            f175(v3.clone(), v1.clone(), f180(v3, v1, false)?)
                        } else {
                            if f32(v0, f8(Str::lit("stage-matrix"))) {
                                f175(v3.clone(), v1.clone(), f180(v3, v1, true)?)
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
            f181(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("ingest-index"))) {
                f182(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("merge-gate"))) {
                    f183(v3, v1)
                } else {
                    if f32(v0.clone(), f8(Str::lit("record-decision"))) {
                        f184(v3, v1)
                    } else {
                        if f32(v0, f8(Str::lit("record-error"))) {
                            f185(v3, v1, v2)
                        } else {
                            Ok(Adt12::C4(f79()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f88(v0: Bytes) -> R<bool> {
    f186(v0.clone(), 0u64, {
        let a480 = v0;
        length_bytes(a480)
    })
}

pub fn f89(p485: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
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

pub fn f90(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
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

pub fn f91(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m490 = f187(f188(v0.clone(), v1.clone()), v1.clone());
    match m490 {
        None => {
            f43(f64(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f92(v0: Option<Adt2>) -> Option<Bytes> {
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

pub fn f93(p498: Adt2, v1: Bytes) -> Option<Adt2> {
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

pub fn f94(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m499 = v0;
    match m499.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m500 = f189(v2, v1.clone())?;
            match m500 {
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
        let a501 = v0.clone();
        length_bytes(a501)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h502, _) = f76(Fn0::F190(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
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
        Ok(f78(List::cons(f46(v0, v3, {
            let a509 = v1;
            let a510 = v3;
            nat_sub(a509, a510)
        }), v4)))
    } else {
        Ok(f78(v4))
    }
}

pub fn f96(v0: Adt1, v1: Bytes) -> R<List<Adt2>> {
    Ok(f191(f57(v0, v1)?))
}

pub fn f97(v0: List<Adt2>, v1: Bytes) -> List<Adt2> {
    let m511 = v0;
    match m511.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f192(v2, v1.clone()), f97(v3, v1))
        }
    }
}

pub fn f98(p516: List<Bytes>, p517: List<Bytes>) -> bool {
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

pub fn f99(v0: List<Adt2>) -> List<Bytes> {
    let m518 = v0;
    match m518.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m519 = f93(v1, f8(Str::lit("std")));
            match m519 {
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
    let a520 = f193(v0)?;
    let a521 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a520, a521))
}

pub fn f101(v0: List<Adt2>) -> Adt2 {
    let m522 = v0;
    match m522.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(v1), std::rc::Rc::new(f101(v2)))
        }
    }
}

pub fn f102(v0: Adt13) -> Bytes {
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

pub fn f103() -> Bytes {
    f8(Str::lit("target/engine/ref.json"))
}

pub fn f104() -> Bytes {
    f8(Str::lit("target/engine/tag.json"))
}

pub fn f105(v0: Adt1, v1: Bytes, v2: Bytes) -> R<Adt12> {
    let a525 = {
        let a524 = v2.clone();
        length_bytes(a524)
    };
    let a526 = 0u64;
    if nat_eq(a525, a526) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m527 = f194(v0.clone(), f60())?;
        match m527 {
            None => {
                if f51(f52(f197(), match v0.clone() {
                    Adt1::C0(_, _, h528, _) => {
                        h528
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f197()))
                } else {
                    let m529 = f56(v0.clone(), List::cons(f198(), f199(f60())?));
                    match m529.uncons() {
                        None => {
                            Ok(Adt12::C3(f200(v0.clone(), f60(), v1.clone(), v2.clone(), f201(f64(v0, f198()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f195(v3.clone())?, f196(v3, v2)))
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

pub fn f114(v0: Bytes, p557: u64, p558: u64, p559: bool, p560: bool, p561: u64, p562: Bytes) -> R<Bytes> {
    let m556 = (p557, (p558, (p559, (p560, (p561, p562)))));
    {
        let mut m556 = m556;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m556;
                let m530 = v2;
                if m530 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f202(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m530 - 1;
                    let m533 = {
                        let a531 = v0.clone();
                        let a532 = v1;
                        index_bytes(a531, a532)
                    };
                    match m533 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f202(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f72(v8)?;
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
                                    }, f202(v0.clone(), v5, v1, v6)))))))
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

pub fn f115(v0: Bytes, v1: u64) -> R<u64> {
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

pub fn f116(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f203(v0.clone(), v1, {
        let a613 = {
            let a612 = v0.clone();
            length_bytes(a612)
        };
        let a614 = 1u64;
        nat_add(a613, a614)?
    })?;
    let a615 = f115(v0.clone(), v2)?;
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
        f204(v0.clone(), v1, {
            let a622 = {
                let a621 = v0;
                length_bytes(a621)
            };
            let a623 = 1u64;
            nat_add(a622, a623)?
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
        let a624 = f115(v0.clone(), v1)?;
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
    let v4: u64 = f205(v0.clone(), v3)?;
    let v5: u64 = {
        let a628 = v4;
        let a629 = v3;
        nat_sub(a628, a629)
    };
    let v6: bool = {
        let a630 = f115(v0.clone(), v4)?;
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
        f205(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f115(v0.clone(), v8)?;
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
    let v11: u64 = f115(v0.clone(), {
        let a638 = v8;
        let a639 = 1u64;
        nat_add(a638, a639)?
    })?;
    let v12: bool = f117(v10, f206({
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
        f205(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a648 = v14;
        let a649 = v13;
        nat_sub(a648, a649)
    });
    let v16: u64 = f207(v15.clone())?;
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
        if f117({
            let a655 = 1u64;
            let a656 = v5;
            nat_lt(a655, a656)
        }, {
            let a657 = f115(v0.clone(), v3)?;
            let a658 = 48u64;
            nat_eq(a657, a658)
        }) {
            true
        } else {
            if f117(v6, {
                let a659 = v8;
                let a660 = v7;
                nat_eq(a659, a660)
            }) {
                true
            } else {
                f206(if v10 {
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
                f208(v17)?
            }
        };
        match m669 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m674 = f209(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a670 = v8;
                    let a671 = v7;
                    nat_sub(a670, a671)
                }), f117(v12, {
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
                            let m680 = f208({
                                let (h679, _) = v19.clone();
                                h679
                            })?;
                            match m680 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f117(v2, {
                                        let a681 = 0u64;
                                        let a682 = v20;
                                        nat_lt(a681, a682)
                                    });
                                    if f210(v21, v20) {
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

pub fn f119(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
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
    let m692 = f211(v0.clone(), v3);
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

pub fn f120(v0: Fn1, p706: List<Bytes>, p707: List<Bytes>) -> List<Bytes> {
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

pub fn f121(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f122(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m708 = f212(Fn2::F213(v1), None::<Bytes>, v0);
    match m708 {
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
        Adt1::C0(_, h709, _, _) => {
            h709
        }
    }, f214()) {
        if f138(match v0 {
            Adt1::C0(_, h710, _, _) => {
                h710
            }
        }, f215()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f215(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f214(), List::<Bytes>::nil())
    }
}

pub fn f125(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m711 = v0;
    match m711.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f125(v3, v1))
        }
    }
}

pub fn f126(v0: Adt1) -> R<List<Bytes>> {
    Ok(f56(v0.clone(), List::cons(f216(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f136(v0)?)))))
}

pub fn f127(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f217(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f142(f96(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f128(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m712 = v0;
    match m712.uncons() {
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
    let m713 = v0;
    match m713.uncons() {
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
    let m714 = v0;
    match m714.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f125(f218(f191(f93(v1, f8(Str::lit("defers"))))), f130(v2))
        }
    }
}

pub fn f131(v0: List<Adt2>) -> List<Bytes> {
    let m715 = v0;
    match m715.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m716 = f93(v1, f8(Str::lit("std")));
            match m716 {
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
    let m717 = v0;
    match m717.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f154(f219(v2.clone()), f220(f221(v0, v1.clone(), v2)?));
    f222(f145(f223(v1))?, v3.clone(), {
        let a719 = {
            let a718 = v3;
            length_list(a718)
        };
        let a720 = 1u64;
        nat_add(a719, a720)?
    })
}

pub fn f134(v0: Adt14, v1: List<Adt2>) -> List<Adt2> {
    let m721 = v1;
    match m721.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            if if f224(v2.clone(), f8(Str::lit("subject"))) {
                f225(v0.clone(), v2.clone())
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
    let m722 = v1;
    match m722.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let m723 = f226(f95(f64(v0.clone(), v2.clone()))?, f85(v2, f215()))?;
            match m723 {
                None => {
                    f135(v0, v3)
                }
                Some(v4) => {
                    Ok(f227(v4, f135(v0, v3)?))
                }
            }
        }
    }
}

pub fn f136(v0: Adt1) -> R<List<Bytes>> {
    Ok(f125(f145(f228(f214(), f147(match v0.clone() {
        Adt1::C0(_, h724, _, _) => {
            h724
        }
    }, f214())))?, f145(f228(f215(), f147(match v0 {
        Adt1::C0(_, h725, _, _) => {
            h725
        }
    }, f215())))?))
}

pub fn f137(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("-c")), List::cons(f229(), List::cons(f8(Str::lit("prism-code")), List::cons(f8(Str::lit("UOR-Foundation/prismpm")), List::cons(f230(), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil())))))))))
}

pub fn f138(p729: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
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

pub fn f139() -> Bytes {
    f8(Str::lit("model/registries"))
}

pub fn f140(v0: Adt1) -> R<List<Bytes>> {
    f145(f228(f139(), f147(match v0 {
        Adt1::C0(_, h730, _, _) => {
            h730
        }
    }, f139())))
}

pub fn f141(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m731 = v1;
    match m731.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f125(if f52(v4.clone(), v3.clone()) {
                f231(v4.clone(), f95(f64(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f232(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
            }, f141(v0, v5, v2, v3)?))
        }
    }
}

pub fn f142(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m732 = v0;
    match m732.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f142(v3, v1))
        }
    }
}

pub fn f143(v0: Adt1) -> R<Adt12> {
    let m733 = f56(v0.clone(), f233());
    match m733.uncons() {
        None => {
            let m734 = f234(v0.clone(), f233())?;
            match m734.uncons() {
                None => {
                    let m737 = f235(match v0.clone() {
                        Adt1::C0(_, h735, _, _) => {
                            h735
                        }
                    }, f125(f236(), f237(match v0.clone() {
                        Adt1::C0(_, h736, _, _) => {
                            h736
                        }
                    }, false)));
                    match m737.uncons() {
                        None => {
                            let m739 = f56(v0.clone(), f237(match v0.clone() {
                                Adt1::C0(_, h738, _, _) => {
                                    h738
                                }
                            }, true));
                            match m739.uncons() {
                                None => {
                                    let m740 = f56(v0.clone(), f238());
                                    match m740.uncons() {
                                        None => {
                                            let m741 = f239(v0.clone())?;
                                            match m741 {
                                                None => {
                                                    let m743 = f240(v0.clone(), f237(match v0.clone() {
                                                        Adt1::C0(_, h742, _, _) => {
                                                            h742
                                                        }
                                                    }, true))?;
                                                    match m743 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f241(Adt16::C0(Adt15::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f217(v0.clone())?, f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f96(v0.clone(), f8(Str::lit("model/strata.json")))?, f218(f96(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f237(match v0 {
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

pub fn f144() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f145(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f242(f120(Fn1::F243, List::<Bytes>::nil(), f244(v0)?)))
}

pub fn f146(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m748 = v1;
    match m748.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f245(v0.clone(), v2), f146(v0, v3))
            } else {
                f146(v0, v3)
            }
        }
    }
}

pub fn f147(p753: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
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

pub fn f148(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, p756: List<Bytes>) -> R<Option<Bytes>> {
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
                        if f246(v0.clone(), v1.clone(), v3.clone())? {
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

pub fn f149(v0: List<List<Bytes>>) -> R<List<(List<Bytes>, List<Bytes>)>> {
    let m757 = v0;
    match m757.uncons() {
        None => {
            Ok(List::<(List<Bytes>, List<Bytes>)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f247(v1.clone())?, v1), f149(v2)?))
        }
    }
}

pub fn f150(p760: List<Bytes>, p761: List<Bytes>, p762: List<List<Bytes>>) -> List<List<Bytes>> {
    let m759 = (p760, (p761, p762));
    {
        let mut m759 = m759;
        loop {
            let step = {
                let (v0, (v1, v2)) = m759;
                let m758 = v0;
                match m758.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f248(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f248(v1, List::<Bytes>::nil()), v2))))
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

pub fn f151(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m763 = v0;
    match m763.uncons() {
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
        Adt1::C0(_, h764, _, _) => {
            h764
        }
    }, f249())) {
        Ok(Adt12::C1(List::cons(f249(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f146(f249(), f147(match v0.clone() {
            Adt1::C0(_, h765, _, _) => {
                h765
            }
        }, f249())))?, v1.clone());
        let m766 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m766.uncons() {
            None => {
                let v5: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m767 = f56(v0.clone(), f250(v0.clone(), v2.clone(), v5.clone())?);
                match m767.uncons() {
                    None => {
                        let m768 = f251(v0.clone(), v2, v5)?;
                        match m768 {
                            None => {
                                f252(v0, v1)
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
    let m769 = v0;
    match m769.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f154(v3, v1))
        }
    }
}

pub fn f155(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m770 = f96(v0.clone(), f153())?;
    match m770.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f253(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f156() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f157() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}            target/release/verdant_xml\u{a}            target/release/verdant_admit\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      # Restored on every run, not only when certified/ changed: the end-to-end propose-command step below\u{a}      # exports with the attested lexlean binary, and a fresh cargo install does not reproduce its hash.\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers certified-xml certified-admit)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/, certified-readers/, certified-xml/ or certified-admit/:\"\u{a}            git status --short -- certified certified-readers certified-xml certified-admit | head -20\u{a}            exit 1\u{a}          fi\u{a}      # verify-artifact replays a bundle but does not compare it with the sources. So each LexLean project's\u{a}      # semantic id must equal the one its bundle records. Lock-level ids move with every admission to app/,\u{a}      # which readers depends on without using it, so they are not compared.\u{a}      - name: the certified bundles were exported from these sources (semantic ids, no Lean)\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The lexlean cache is keyed by .github/lexlean.pin, so after the pin moves the replay above may have been skipped with no binary restored.\u{a}          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          for pair in studio:certified readers:certified-readers readers/xml:certified-xml admit:certified-admit; do\u{a}            project=$(cut -d : -f 1 <<< \"$pair\")\u{a}            bundle=$(cut -d : -f 2 <<< \"$pair\")\u{a}            if [ ! -f \"$project/lexlean.toml\" ]; then continue; fi\u{a}            if [ ! -f \"$bundle/result.json\" ]; then echo \"::warning::$project has no committed bundle $bundle, so nothing records what it was exported from\"; continue; fi\u{a}            now=$(cd \"$project\" && \"$ll\" lock >/dev/null && \"$ll\" check | sed -n 's/.*semantic \\([0-9a-f]*\\)).*/\\1/p')\u{a}            git checkout -q -- \"$project/lexlean.lock\"\u{a}            was=$(jq -r .source.semantic_id \"$bundle/result.json\")\u{a}            if [ \"$now\" != \"$was\" ]; then echo \"::error::$project now has semantic id $now, but $bundle was exported from $was: export it again through propose-command\"; exit 1; fi\u{a}            echo \"$project matches $bundle (semantic $now)\"\u{a}          done\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          # Each encoding admitted through admit-translated recorded the reader command that printed it: run it again and fail if the output differs.\u{a}          target/release/verdant_readers verify-translations model/admissions.json\u{a}          if [ -x target/release/verdant_xml ]; then target/release/verdant_xml verify-translations model/admissions.json; fi\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept the Prism of the seed Verdant Studio, every facet at its media type and its generator's fidelity\u{a}        run: python3 .github/scripts/prism-acceptance.py node target/prismpm/sdk/npm/prism-code/bin/prism-code.mjs\u{a}      - name: the workspace API's operations are MCP tools valid against the bound MCP schema (Studio.Surface)\u{a}        run: python3 .github/scripts/surface-check.py\u{a}      - name: the Studio UI Prism's view tree is published as MCP resources valid against the bound MCP schema (Studio.Agent.Interface.Resource)\u{a}        run: python3 .github/scripts/gui-tree.py node target/prismpm/sdk/npm/prism-code/bin/prism-code.mjs\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          # certified-xml/ and certified-admit/ join the comparison once main holds them: the export that first writes it is the one that adds it.\u{a}          bundles=\"certified certified-readers\"\u{a}          if [ -n \"$(git ls-files certified-xml | head -n 1)\" ]; then bundles=\"$bundles certified-xml\"; fi\u{a}          if [ -n \"$(git ls-files certified-admit | head -n 1)\" ]; then bundles=\"$bundles certified-admit\"; fi\u{a}          if [ -n \"$(git status --porcelain -- $bundles)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- $bundles | head -20\u{a}            attested=$(git show HEAD:certified/evidence/attestation.json | jq -c .lexlean); used=$(jq -c .lexlean certified/evidence/attestation.json)\u{a}            [ \"$attested\" = \"$used\" ] || echo \"::error::the export ran with lexlean $used, but certified/ attests lexlean $attested. lexlean is built from .github/lexlean.pin with rust 1.97.0 and its cargo home remapped, so a different executable means the toolchain or the runner image changed\"\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f158() -> Bytes {
    f7(List::cons(f254(), List::cons(f255(), List::cons(f256(), List::cons(f257(), List::cons(f258(), List::cons(f259(), List::<Bytes>::nil())))))))
}

pub fn f159() -> Bytes {
    f7(List::cons(f260(), List::cons(f261(), List::cons(f262(), List::cons(f263(), List::<Bytes>::nil())))))
}

pub fn f160() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports under its own limits: studio, readers, admit and readers/xml when they exist, largest first.\u{a}# Lean's proof of a project grows with the square of its function count, so the readers are split by family and admission is its own project.\u{a}# Each export runs two Lean phases at once, so at most half as many exports as cores run together: more slows every phase.\u{a}slots=$(( $(nproc) / 2 ))\u{a}if [ \"$slots\" -lt 1 ]; then slots=1; fi\u{a}running=0\u{a}exported=' '\u{a}for project in studio readers admit readers/xml; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    # A project whose semantic id is the one its committed bundle was exported from, under the same lexlean, is not\u{a}    # exported again: the studio check compares exactly these ids, so its bundle stays the bundle of its sources.\u{a}    case \"$project\" in studio) bundle=certified;; readers/xml) bundle=certified-xml;; *) bundle=\"certified-$project\";; esac\u{a}    if [ -f \"$bundle/result.json\" ] && git diff --quiet HEAD -- .github/lexlean.pin; then\u{a}      now=$(cd \"$project\" && \"$ll\" lock >/dev/null && \"$ll\" check | sed -n 's/.*semantic \\([0-9a-f]*\\)).*/\\1/p')\u{a}      if [ -n \"$now\" ] && [ \"$now\" = \"$(jq -r .source.semantic_id \"$bundle/result.json\")\" ]; then\u{a}        echo \"$project is unchanged since $bundle was exported (semantic $now), so it is not exported again\"\u{a}        continue\u{a}      fi\u{a}    fi\u{a}    exported=\"$exported$project \"\u{a}    if [ \"$running\" -ge \"$slots\" ]; then wait -n || status=$?; running=$((running - 1)); fi\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.${project//\\//-}\" 2>&1 &\u{a}    running=$((running + 1))\u{a}  fi\u{a}done\u{a}while [ \"$running\" -gt 0 ]; do wait -n || status=$?; running=$((running - 1)); done\u{a}: > \"$log\"\u{a}for part in studio readers readers-xml admit; do\u{a}  if [ -f \"$log.$part\" ]; then cat \"$log.$part\" >> \"$log\"; fi\u{a}done\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}if [[ \"$exported\" == *\" studio \"* ]]; then\u{a}  rm -rf certified\u{a}  cp -r studio/.lexlean/production/*/ certified\u{a}  rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified\u{a}fi\u{a}if [[ \"$exported\" == *\" readers \"* ]]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}if [[ \"$exported\" == *\" readers/xml \"* ]]; then\u{a}  rm -rf certified-xml\u{a}  cp -r readers/xml/.lexlean/production/*/ certified-xml\u{a}  rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-xml\u{a}fi\u{a}if [[ \"$exported\" == *\" admit \"* ]]; then\u{a}  rm -rf certified-admit\u{a}  cp -r admit/.lexlean/production/*/ certified-admit\u{a}  rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-admit\u{a}fi\u{a}"))
}

pub fn f161() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env bash\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# A change to a reader can change what it prints for an encoding already admitted, and the studio check\u{a}")), List::cons(f8(Str::lit("# replays every admitted reader command (verify-translations). So the change re-derives those encodings\u{a}")), List::cons(f8(Str::lit("# itself: each Catalog encoding whose replay now differs is admitted again through read-admit, with the\u{a}")), List::cons(f8(Str::lit("# command and citation its admission row records, and the pull request carries both (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("# It prints each construct it admitted again, one per line.\u{a}")), List::cons(f8(Str::lit("set -euo pipefail\u{a}")), List::cons(f8(Str::lit("readers=$1\u{a}")), List::cons(f8(Str::lit("mkdir -p target\u{a}")), List::cons(f8(Str::lit("rows=$(jq length model/admissions.json)\u{a}")), List::cons(f8(Str::lit("for try in $(seq 1 \"$rows\"); do\u{a}")), List::cons(f8(Str::lit("  if \"$readers\" verify-translations model/admissions.json > target/replay.txt 2>&1; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("  construct=$(sed -n 's/^\\([^:]*\\): its reader command now prints .*/\\1/p' target/replay.txt | head -n 1)\u{a}")), List::cons(f8(Str::lit("  case \"$construct\" in\u{a}")), List::cons(f8(Str::lit("    Catalog.*) ;;\u{a}")), List::cons(f8(Str::lit("    *) cat target/replay.txt >&2; echo \"readmit: only a Catalog encoding whose reader now prints other output is admitted again\" >&2; exit 1;;\u{a}")), List::cons(f8(Str::lit("  esac\u{a}")), List::cons(f8(Str::lit("  row=$(jq -c --arg construct \"$construct\" '[.[] | select(.construct == $construct and .translation != null)][0]' model/admissions.json)\u{a}")), List::cons(f8(Str::lit("  authority=$(jq -r '.std.authority // \"\"' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  cited=$(jq -r '.std.row // \"\"' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  if [ -z \"$authority\" ]; then echo \"readmit: $construct cites no std authority, so read-admit cannot cite it again\" >&2; exit 1; fi\u{a}")), List::cons(f8(Str::lit("  mapfile -t command < <(jq -r '.translation.command[]' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  \"$readers\" read-admit \"${construct#Catalog.}\" \"${command[@]}\" std \"$authority\" $cited >/dev/null\u{a}")), List::cons(f8(Str::lit("  echo \"$construct\"\u{a}")), List::cons(f8(Str::lit("done\u{a}")), List::cons(f8(Str::lit("echo \"readmit: the replay did not settle after $rows admissions\" >&2\u{a}")), List::cons(f8(Str::lit("exit 1\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))
}

pub fn f162() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# app/src/Contract.lex.tex and model/strata.json each keep their entries on one long line, so two changes that\u{a}")), List::cons(f8(Str::lit("# each change entries there always conflict, whatever the entries. A change that only adds, removes or replaces\u{a}")), List::cons(f8(Str::lit("# entries is merged into main's file by entry instead: Contract's importmodule lines, dispatch routes and\u{a}")), List::cons(f8(Str::lit("# acceptance vectors, and the strata rows keyed by path (VS-05). Anything else is left to git apply.\u{a}")), List::cons(f8(Str::lit("#   merge BASE NEW MAIN PATH  prints NEW's entry changes merged into MAIN, or exits 1 when NEW changes more.\u{a}")), List::cons(f8(Str::lit("#   patch FILE                merges each of the two files a patch changes that way, and leaves the rest in FILE.\u{a}")), List::cons(f8(Str::lit("import json, os, re, subprocess, sys, tempfile\u{a}")), List::cons(f8(Str::lit("CONTRACT = 'app/src/Contract.lex.tex'\u{a}")), List::cons(f8(Str::lit("STRATA = 'model/strata.json'\u{a}")), List::cons(f8(Str::lit("NIL = '(nil Foundation.View.Text.V1.Model::AcceptanceVector)'\u{a}")), List::cons(f8(Str::lit("ROUTE = re.compile(r'[(]if [(]routed value (0x[0-9a-f]+)[)] then [(][A-Za-z0-9_.]+::[A-Za-z0-9_]+ [(]routedValue value \\1[)][)] else ')\u{a}")), List::cons(f8(Str::lit("VECTOR = re.compile(r'[(]cons [(]record Foundation[.]View[.]Text[.]V1[.]Model::AcceptanceVector [{]request := 0x[0-9a-f]*, response := 0x[0-9a-f]*[}][)] ')\u{a}")), List::cons(f8(Str::lit("IMPORT = re.compile(r'(?m)^[\\\\]importmodule[{][^}]+[}]$')\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def entries(text):\u{a}")), List::cons(f8(Str::lit("    return IMPORT.findall(text), [m.group(0) for m in ROUTE.finditer(text)], VECTOR.findall(text)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def dispatch_line(text):\u{a}")), List::cons(f8(Str::lit("    at = text.index('body: ', text.index('name: dispatch')) + 6\u{a}")), List::cons(f8(Str::lit("    return at, text.index(chr(10), at)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def skeleton(text):\u{a}")), List::cons(f8(Str::lit("    text = re.sub('[' + chr(10) + ']+', chr(10), IMPORT.sub('', ROUTE.sub('', VECTOR.sub('', text))))\u{a}")), List::cons(f8(Str::lit("    at, end = dispatch_line(text)\u{a}")), List::cons(f8(Str::lit("    line = text[at:end].rstrip(',').rstrip(')')\u{a}")), List::cons(f8(Str::lit("    text = text[:at] + line + text[end:]\u{a}")), List::cons(f8(Str::lit("    n = text.index(NIL) + len(NIL)\u{a}")), List::cons(f8(Str::lit("    return text[:n] + text[n:].lstrip(')')\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def drop(text, gone):\u{a}")), List::cons(f8(Str::lit("    for entry in gone:\u{a}")), List::cons(f8(Str::lit("        text = text.replace(entry, '', 1) if not entry.startswith(chr(92)) else text.replace(entry + chr(10), '', 1)\u{a}")), List::cons(f8(Str::lit("    return text\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def edit_contract(text, removed, added):\u{a}")), List::cons(f8(Str::lit("    imports, routes, vectors = added\u{a}")), List::cons(f8(Str::lit("    at, end = dispatch_line(text)\u{a}")), List::cons(f8(Str::lit("    line = drop(text[at:end], removed[1])\u{a}")), List::cons(f8(Str::lit("    tail = ',' if line.endswith(',') else ''\u{a}")), List::cons(f8(Str::lit("    line = line[:len(line) - len(tail)]\u{a}")), List::cons(f8(Str::lit("    line = ''.join(routes) + line[:len(line) - len(removed[1])] + ')' * len(routes) + tail\u{a}")), List::cons(f8(Str::lit("    text = drop(drop(text[:at] + line + text[end:], removed[2]), removed[0])\u{a}")), List::cons(f8(Str::lit("    if imports:\u{a}")), List::cons(f8(Str::lit("        last = list(IMPORT.finditer(text))[-1].end()\u{a}")), List::cons(f8(Str::lit("        text = text[:last] + ''.join(chr(10) + i for i in imports) + text[last:]\u{a}")), List::cons(f8(Str::lit("    n = text.index(NIL)\u{a}")), List::cons(f8(Str::lit("    after = text[n + len(NIL):]\u{a}")), List::cons(f8(Str::lit("    if not after.startswith(')' * len(removed[2])):\u{a}")), List::cons(f8(Str::lit("        raise ValueError('the vectors are not closed after their list')\u{a}")), List::cons(f8(Str::lit("    return text[:n] + ''.join(vectors) + NIL + ')' * len(vectors) + after[len(removed[2]):]\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def merge_contract(base, new, main):\u{a}")), List::cons(f8(Str::lit("    if skeleton(base) != skeleton(new) or skeleton(base) != skeleton(main):\u{a}")), List::cons(f8(Str::lit("        return None\u{a}")), List::cons(f8(Str::lit("    b, n, m = entries(base), entries(new), entries(main)\u{a}")), List::cons(f8(Str::lit("    removed = [[e for e in old if e not in now and e in have] for old, now, have in zip(b, n, m)]\u{a}")), List::cons(f8(Str::lit("    added = [[e for e in now if e not in old and e not in have] for old, now, have in zip(b, n, m)]\u{a}")), List::cons(f8(Str::lit("    return edit_contract(main, removed, added)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def merge_strata(base, new, main):\u{a}")), List::cons(f8(Str::lit("    b, n, m = json.loads(base), json.loads(new), json.loads(main)\u{a}")), List::cons(f8(Str::lit("    kept = [r for r in m if r in n or r not in b]\u{a}")), List::cons(f8(Str::lit("    have = set(r['path'] for r in kept)\u{a}")), List::cons(f8(Str::lit("    rows = kept + [r for r in n if r not in b and r['path'] not in have]\u{a}")), List::cons(f8(Str::lit("    for ascii in (True, False):\u{a}")), List::cons(f8(Str::lit("        if json.dumps(m, separators=(',', ':'), ensure_ascii=ascii) + chr(10) == main:\u{a}")), List::cons(f8(Str::lit("            return json.dumps(rows, separators=(',', ':'), ensure_ascii=ascii) + chr(10)\u{a}")), List::cons(f8(Str::lit("    return None\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def merge(base, new, main, path):\u{a}")), List::cons(f8(Str::lit("    return merge_contract(base, new, main) if path == CONTRACT else merge_strata(base, new, main)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def blob(sha):\u{a}")), List::cons(f8(Str::lit("    got = subprocess.run(['git', 'cat-file', '-p', sha], capture_output=True, text=True)\u{a}")), List::cons(f8(Str::lit("    if got.returncode == 0:\u{a}")), List::cons(f8(Str::lit("        return got.stdout\u{a}")), List::cons(f8(Str::lit("    return subprocess.run(['gh', 'api', '-H', 'Accept: application/vnd.github.raw', 'repos/' + os.environ['GITHUB_REPOSITORY'] + chr(47) + 'git/blobs/' + sha], capture_output=True, text=True, check=True).stdout\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def changed(path, base, section):\u{a}")), List::cons(f8(Str::lit("    with tempfile.TemporaryDirectory() as work:\u{a}")), List::cons(f8(Str::lit("        subprocess.run(['git', 'init', '-q', work], check=True)\u{a}")), List::cons(f8(Str::lit("        os.makedirs(os.path.join(work, os.path.dirname(path)), exist_ok=True)\u{a}")), List::cons(f8(Str::lit("        open(os.path.join(work, path), 'w').write(base)\u{a}")), List::cons(f8(Str::lit("        subprocess.run(['git', 'apply', '-'], cwd=work, input=section, text=True, check=True)\u{a}")), List::cons(f8(Str::lit("        return open(os.path.join(work, path)).read()\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def patch(name):\u{a}")), List::cons(f8(Str::lit("    text = open(name).read()\u{a}")), List::cons(f8(Str::lit("    parts = re.split('(?m)^(?=diff --git )', text)\u{a}")), List::cons(f8(Str::lit("    kept = [parts[0]]\u{a}")), List::cons(f8(Str::lit("    for section in parts[1:]:\u{a}")), List::cons(f8(Str::lit("        path = section.split(chr(10), 1)[0].split(' ')[-1][2:]\u{a}")), List::cons(f8(Str::lit("        index = re.search('(?m)^index ([0-9a-f]{40})[.][.]', section)\u{a}")), List::cons(f8(Str::lit("        merged = None\u{a}")), List::cons(f8(Str::lit("        if path in (CONTRACT, STRATA) and index:\u{a}")), List::cons(f8(Str::lit("            try:\u{a}")), List::cons(f8(Str::lit("                base = blob(index.group(1))\u{a}")), List::cons(f8(Str::lit("                merged = merge(base, changed(path, base, section), open(path).read(), path)\u{a}")), List::cons(f8(Str::lit("            except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as failure:\u{a}")), List::cons(f8(Str::lit("                print('::warning::' + path + ' is left to git apply: ' + str(failure).splitlines()[0])\u{a}")), List::cons(f8(Str::lit("        if merged is None:\u{a}")), List::cons(f8(Str::lit("            kept.append(section)\u{a}")), List::cons(f8(Str::lit("        else:\u{a}")), List::cons(f8(Str::lit("            open(path, 'w').write(merged)\u{a}")), List::cons(f8(Str::lit("            print('::notice::merged the entries this change adds to ' + path + ' into main by entry')\u{a}")), List::cons(f8(Str::lit("    open(name, 'w').write(''.join(kept))\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("if sys.argv[1] == 'merge':\u{a}")), List::cons(f8(Str::lit("    merged = merge(*[open(p).read() for p in sys.argv[2:5]], sys.argv[5])\u{a}")), List::cons(f8(Str::lit("    if merged is None:\u{a}")), List::cons(f8(Str::lit("        sys.exit(1)\u{a}")), List::cons(f8(Str::lit("    sys.stdout.write(merged)\u{a}")), List::cons(f8(Str::lit("else:\u{a}")), List::cons(f8(Str::lit("    patch(sys.argv[2])\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f163() -> Bytes {
    f7(List::cons(f264(), List::cons(f265(), List::cons(f266(), List::<Bytes>::nil()))))
}

pub fn f164() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# Parallel lanes each register the next VS IDs they see, so two changes can claim the same ones, and each\u{a}")), List::cons(f8(Str::lit("# appends its rows at the end of model/ids.toml and of a suite. So each VS ID a change adds that main already\u{a}")), List::cons(f8(Str::lit("# has becomes the next free one throughout the change, and a file the change only appends to is appended to on\u{a}")), List::cons(f8(Str::lit("# main instead of patched, so a change made against an older main still applies. propose-command and refresh\u{a}")), List::cons(f8(Str::lit("# both take this path.\u{a}")), List::cons(f8(Str::lit("#   prepare PATCH APPENDS  renumbers PATCH in place and moves its pure appends into APPENDS\u{a}")), List::cons(f8(Str::lit("#   append APPENDS         appends them, after the rest of the patch is applied\u{a}")), List::cons(f8(Str::lit("#   lint PATCH             refuses an added string literal the Lean export would refuse as an absolute path\u{a}")), List::cons(f8(Str::lit("import re, json, sys\u{a}")), List::cons(f8(Str::lit("nl = chr(10)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def prepare(name, appends_name):\u{a}")), List::cons(f8(Str::lit("    text = open(name).read()\u{a}")), List::cons(f8(Str::lit("    parts = re.split('(?m)^(?=diff --git )', text)\u{a}")), List::cons(f8(Str::lit("    head, files = parts[0], parts[1:]\u{a}")), List::cons(f8(Str::lit("    have = set(int(n) for n in re.findall('(?m)^id = .VS-([0-9]+).', open('model/ids.toml').read()))\u{a}")), List::cons(f8(Str::lit("    added = []\u{a}")), List::cons(f8(Str::lit("    for f in files:\u{a}")), List::cons(f8(Str::lit("        added += [int(n) for n in re.findall('(?m)^[+]id = .VS-([0-9]+).', f)]\u{a}")), List::cons(f8(Str::lit("    top = max(have | set(added) | {0})\u{a}")), List::cons(f8(Str::lit("    ren = {}\u{a}")), List::cons(f8(Str::lit("    for n in added:\u{a}")), List::cons(f8(Str::lit("        if n in have and n not in ren:\u{a}")), List::cons(f8(Str::lit("            top += 1\u{a}")), List::cons(f8(Str::lit("            ren[n] = top\u{a}")), List::cons(f8(Str::lit("    def fix(line):\u{a}")), List::cons(f8(Str::lit("        return re.sub('VS-([0-9]+)(?![0-9])', lambda m: 'VS-' + str(ren[int(m.group(1))]).zfill(2) if int(m.group(1)) in ren else m.group(0), line)\u{a}")), List::cons(f8(Str::lit("    appends = {}\u{a}")), List::cons(f8(Str::lit("    kept = []\u{a}")), List::cons(f8(Str::lit("    for f in files:\u{a}")), List::cons(f8(Str::lit("        lines = [fix(l) if l.startswith('+') and not l.startswith('+++') else l for l in f.split(nl)]\u{a}")), List::cons(f8(Str::lit("        hunks = [i for i, l in enumerate(lines) if l.startswith('@@')]\u{a}")), List::cons(f8(Str::lit("        # The last hunk is a pure append when it removes nothing and ends on an added line, so no\u{a}")), List::cons(f8(Str::lit("        # context follows it: it reaches the end of the file. Any hunks before it are patched as usual.\u{a}")), List::cons(f8(Str::lit("        body = [l for l in lines[hunks[-1] + 1:] if l != ''] if hunks else []\u{a}")), List::cons(f8(Str::lit("        plain = 'new file mode' not in f and 'GIT binary patch' not in f\u{a}")), List::cons(f8(Str::lit("        if plain and body and body[-1].startswith('+') and not any(l[:1] in '-' + chr(92) for l in body):\u{a}")), List::cons(f8(Str::lit("            appends[lines[0].split(' ')[2][2:]] = [l[1:] for l in body if l.startswith('+')]\u{a}")), List::cons(f8(Str::lit("            if len(hunks) > 1:\u{a}")), List::cons(f8(Str::lit("                kept.append(nl.join(lines[:hunks[-1]]) + nl)\u{a}")), List::cons(f8(Str::lit("        else:\u{a}")), List::cons(f8(Str::lit("            kept.append(nl.join(lines))\u{a}")), List::cons(f8(Str::lit("    open(name, 'w').write(head + ''.join(kept))\u{a}")), List::cons(f8(Str::lit("    json.dump(appends, open(appends_name, 'w'))\u{a}")), List::cons(f8(Str::lit("    for n in sorted(ren):\u{a}")), List::cons(f8(Str::lit("        print('::notice::VS-' + str(n).zfill(2) + ' is already on main, so this change registers it as VS-' + str(ren[n]).zfill(2))\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def append(appends_name):\u{a}")), List::cons(f8(Str::lit("    for path, add in json.load(open(appends_name)).items():\u{a}")), List::cons(f8(Str::lit("        old = open(path).read()\u{a}")), List::cons(f8(Str::lit("        if old and not old.endswith(nl):\u{a}")), List::cons(f8(Str::lit("            old += nl\u{a}")), List::cons(f8(Str::lit("        open(path, 'w').write(old + nl.join(add) + nl)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def rooted(text):\u{a}")), List::cons(f8(Str::lit("    # LexLean's own test for an absolute path in Lean's output (first_unexpected_absolute_path in its verifier),\u{a}")), List::cons(f8(Str::lit("    # run over each added string literal as written, which is how Lean prints it: escapes stay escaped, so a line\u{a}")), List::cons(f8(Str::lit("    # ending in a one-letter name and a colon reads as a Windows path.\u{a}")), List::cons(f8(Str::lit("    slash, colon, backslash = chr(47), chr(58), chr(92)\u{a}")), List::cons(f8(Str::lit("    boundary = lambda char: char.isspace() or char in chr(34) + chr(39) + chr(96) + '([{<,;='\u{a}")), List::cons(f8(Str::lit("    segment = lambda char: char.isalnum() or char in '_.'\u{a}")), List::cons(f8(Str::lit("    for at, char in enumerate(text):\u{a}")), List::cons(f8(Str::lit("        before = at == 0 or boundary(text[at - 1])\u{a}")), List::cons(f8(Str::lit("        ahead = text[at + 1:at + 4]\u{a}")), List::cons(f8(Str::lit("        posix = char == slash and before and len(ahead) > 0 and segment(ahead[0])\u{a}")), List::cons(f8(Str::lit("        windows = char.isascii() and char.isalpha() and before and len(ahead) == 3 and ahead[0] == colon and ahead[1] in slash + backslash and segment(ahead[2])\u{a}")), List::cons(f8(Str::lit("        if posix or windows:\u{a}")), List::cons(f8(Str::lit("            return text[at:at + 40]\u{a}")), List::cons(f8(Str::lit("    return None\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def lint(patch_name):\u{a}")), List::cons(f8(Str::lit("    found = []\u{a}")), List::cons(f8(Str::lit("    for line in open(patch_name).read().split(nl):\u{a}")), List::cons(f8(Str::lit("        if line.startswith('+') and not line.startswith('+++'):\u{a}")), List::cons(f8(Str::lit("            for literal in re.findall(chr(34) + '((?:[^' + chr(34) + chr(92) * 2 + ']|' + chr(92) * 2 + '.)*)' + chr(34), line):\u{a}")), List::cons(f8(Str::lit("                hit = rooted(literal)\u{a}")), List::cons(f8(Str::lit("                if hit is not None:\u{a}")), List::cons(f8(Str::lit("                    found.append(hit)\u{a}")), List::cons(f8(Str::lit("    for hit in found:\u{a}")), List::cons(f8(Str::lit("        print('::error::LLV7006: an added string literal holds ' + repr(hit) + ', which the Lean export refuses as an absolute path')\u{a}")), List::cons(f8(Str::lit("    return 1 if found else 0\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("if sys.argv[1] == 'prepare':\u{a}")), List::cons(f8(Str::lit("    prepare(sys.argv[2], sys.argv[3])\u{a}")), List::cons(f8(Str::lit("elif sys.argv[1] == 'lint':\u{a}")), List::cons(f8(Str::lit("    sys.exit(lint(sys.argv[2]))\u{a}")), List::cons(f8(Str::lit("else:\u{a}")), List::cons(f8(Str::lit("    append(sys.argv[2])\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f165() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f166() -> Bytes {
    f8(Str::lit("--git https://github.com/UOR-Foundation/lexlean --rev 82a32815c45762ea8931c84cb9465a5aac902866\u{a}"))
}

pub fn f167(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f127(v0.clone())?;
    let v2: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt2> = f134(v1.clone(), f135(v0.clone(), f136(v0.clone())?)?);
    let v4: List<List<Bytes>> = f267(f268(f64(v0, f216()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f269(v4.clone(), f270(v4), f221(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f133(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f168(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f271(f127(v0.clone())?, f135(v0.clone(), f136(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f169(v0: Adt1) -> R<Bytes> {
    let v1: List<List<Bytes>> = f267(f268(f64(v0.clone(), f216()))?);
    let v2: List<(Bytes, Bytes)> = f272(f270(v1.clone()), f135(v0.clone(), f136(v0)?)?);
    let v3: Bytes = f7(f273(v1, 0u64, 26u64)?);
    let v4: Bytes = f7(f145(f274(v2.clone(), v2)?)?);
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Studio.Prism.Closure.Register}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}definition{\u{a}  name: cells,\u{a}  parameters: [],\u{a}  result: Bytes,\u{a}  body: 0x")), List::cons(f275(v3.clone(), 0u64, {
        let a771 = v3;
        length_bytes(a771)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\u{a}}\u{a}\u{a}definition{\u{a}  name: pairs,\u{a}  parameters: [],\u{a}  result: Bytes,\u{a}  body: 0x")), List::cons(f275(v4.clone(), 0u64, {
        let a772 = v4;
        length_bytes(a772)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\u{a}}\u{a}\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f170(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f276(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f277(f96(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f171() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f172() -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"cacheScope\":\"public\",\"resultType\":\"complete\",\"ttlMs\":0,\"tools\":[")), List::cons(f122(f278(f123())?, f8(Str::lit(","))), List::cons(f8(Str::lit("]}\u{a}")), List::<Bytes>::nil())))))
}

pub fn f173() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# Studio.Surface (REQ-AGENT-INTERFACE): the workspace API's operations, published as MCP Tool definitions in\u{a}")), List::cons(f8(Str::lit("# surface/mcp-tools.json, must be a ListToolsResult of the bound MCP-SCHEMA-2026-07-28. This validates it against\u{a}")), List::cons(f8(Str::lit("# that schema's own definitions (type, required, properties, items, enum, const, additionalProperties, $ref), and\u{a}")), List::cons(f8(Str::lit("# requires every tool's inputSchema to be a JSON Schema object, one tool for each operation the usage lists.\u{a}")), List::cons(f8(Str::lit("import json, sys\u{a}")), List::cons(f8(Str::lit("TYPES = {'object': dict, 'array': list, 'string': str, 'boolean': bool, 'null': type(None)}\u{a}")), List::cons(f8(Str::lit("schema = json.load(open('authorities' + chr(47) + 'MCP-SCHEMA-2026-07-28' + chr(47) + 'schema.json'))\u{a}")), List::cons(f8(Str::lit("defs = schema['$defs']\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def numeric(value):\u{a}")), List::cons(f8(Str::lit("    return isinstance(value, (int, float)) and not isinstance(value, bool)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def check(value, rule, where, problems):\u{a}")), List::cons(f8(Str::lit("    if not isinstance(rule, dict):\u{a}")), List::cons(f8(Str::lit("        return\u{a}")), List::cons(f8(Str::lit("    if '$ref' in rule:\u{a}")), List::cons(f8(Str::lit("        check(value, defs[rule['$ref'].split(chr(47))[-1]], where, problems)\u{a}")), List::cons(f8(Str::lit("    kinds = rule.get('type')\u{a}")), List::cons(f8(Str::lit("    if kinds is not None:\u{a}")), List::cons(f8(Str::lit("        kinds = kinds if isinstance(kinds, list) else [kinds]\u{a}")), List::cons(f8(Str::lit("        fits = any((kind in ('number', 'integer') and numeric(value)) or (kind in TYPES and isinstance(value, TYPES[kind])) for kind in kinds)\u{a}")), List::cons(f8(Str::lit("        if not fits:\u{a}")), List::cons(f8(Str::lit("            problems.append(where + ' is not of type ' + repr(kinds))\u{a}")), List::cons(f8(Str::lit("            return\u{a}")), List::cons(f8(Str::lit("    if 'enum' in rule and value not in rule['enum']:\u{a}")), List::cons(f8(Str::lit("        problems.append(where + ' is not one of ' + repr(rule['enum']))\u{a}")), List::cons(f8(Str::lit("    if 'const' in rule and value != rule['const']:\u{a}")), List::cons(f8(Str::lit("        problems.append(where + ' is not ' + repr(rule['const']))\u{a}")), List::cons(f8(Str::lit("    if isinstance(value, dict):\u{a}")), List::cons(f8(Str::lit("        for name in rule.get('required', []):\u{a}")), List::cons(f8(Str::lit("            if name not in value:\u{a}")), List::cons(f8(Str::lit("                problems.append(where + ' lacks ' + name)\u{a}")), List::cons(f8(Str::lit("        known = rule.get('properties', {})\u{a}")), List::cons(f8(Str::lit("        for name, item in value.items():\u{a}")), List::cons(f8(Str::lit("            if name in known:\u{a}")), List::cons(f8(Str::lit("                check(item, known[name], where + '.' + name, problems)\u{a}")), List::cons(f8(Str::lit("            elif rule.get('additionalProperties') is False:\u{a}")), List::cons(f8(Str::lit("                problems.append(where + ' has ' + name + ', which the schema does not allow')\u{a}")), List::cons(f8(Str::lit("            else:\u{a}")), List::cons(f8(Str::lit("                check(item, rule.get('additionalProperties'), where + '.' + name, problems)\u{a}")), List::cons(f8(Str::lit("    if isinstance(value, list) and 'items' in rule:\u{a}")), List::cons(f8(Str::lit("        for index, item in enumerate(value):\u{a}")), List::cons(f8(Str::lit("            check(item, rule['items'], where + '[' + str(index) + ']', problems)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("result = json.load(open('surface' + chr(47) + 'mcp-tools.json'))\u{a}")), List::cons(f8(Str::lit("problems = []\u{a}")), List::cons(f8(Str::lit("check(result, defs['ListToolsResult'], 'tools-list', problems)\u{a}")), List::cons(f8(Str::lit("names = [tool.get('name') for tool in result.get('tools', [])]\u{a}")), List::cons(f8(Str::lit("if len(names) != len(set(names)):\u{a}")), List::cons(f8(Str::lit("    problems.append('tools-list names a tool twice')\u{a}")), List::cons(f8(Str::lit("for tool in result.get('tools', []):\u{a}")), List::cons(f8(Str::lit("    shape = tool.get('inputSchema', {})\u{a}")), List::cons(f8(Str::lit("    for name in shape.get('required', []):\u{a}")), List::cons(f8(Str::lit("        if name not in shape.get('properties', {}):\u{a}")), List::cons(f8(Str::lit("            problems.append(tool.get('name', '?') + ' requires ' + name + ', which it does not define')\u{a}")), List::cons(f8(Str::lit("for problem in problems:\u{a}")), List::cons(f8(Str::lit("    print('::error::surface' + chr(47) + 'mcp-tools.json ' + problem)\u{a}")), List::cons(f8(Str::lit("if not problems:\u{a}")), List::cons(f8(Str::lit("    print('accepted: surface' + chr(47) + 'mcp-tools.json holds ' + str(len(names)) + ' MCP tools, valid against MCP-SCHEMA-2026-07-28')\u{a}")), List::cons(f8(Str::lit("sys.exit(1 if problems else 0)\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f174() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# Studio.Agent.Interface.Resource (REQ-UI-PRISM, REQ-AGENT-INTERFACE): the view tree of the Studio UI Prism, published\u{a}")), List::cons(f8(Str::lit("# as MCP resources. Each journey's routes, views and actions are read from the app's own acceptance vectors, the\u{a}")), List::cons(f8(Str::lit("# answers its model gives, and its guards from the build rows of its suite in model/ids.toml, so nothing here\u{a}")), List::cons(f8(Str::lit("# restates the model. The resource list must be a ListResourcesResult, and each journey a ReadResourceResult, of\u{a}")), List::cons(f8(Str::lit("# the bound MCP-SCHEMA-2026-07-28. gui-tree.py COMMAND... runs COMMAND record on the app's holo.\u{a}")), List::cons(f8(Str::lit("import glob, json, subprocess, sys, tomllib\u{a}")), List::cons(f8(Str::lit("SLASH = chr(47)\u{a}")), List::cons(f8(Str::lit("TYPES = {'object': dict, 'array': list, 'string': str, 'boolean': bool, 'null': type(None)}\u{a}")), List::cons(f8(Str::lit("schema = json.load(open(SLASH.join(['authorities', 'MCP-SCHEMA-2026-07-28', 'schema.json'])))\u{a}")), List::cons(f8(Str::lit("defs = schema['$defs']\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def numeric(value):\u{a}")), List::cons(f8(Str::lit("    return isinstance(value, (int, float)) and not isinstance(value, bool)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def check(value, rule, where, problems):\u{a}")), List::cons(f8(Str::lit("    if not isinstance(rule, dict):\u{a}")), List::cons(f8(Str::lit("        return\u{a}")), List::cons(f8(Str::lit("    if '$ref' in rule:\u{a}")), List::cons(f8(Str::lit("        check(value, defs[rule['$ref'].split(SLASH)[-1]], where, problems)\u{a}")), List::cons(f8(Str::lit("    kinds = rule.get('type')\u{a}")), List::cons(f8(Str::lit("    if kinds is not None:\u{a}")), List::cons(f8(Str::lit("        kinds = kinds if isinstance(kinds, list) else [kinds]\u{a}")), List::cons(f8(Str::lit("        fits = any((kind in ('number', 'integer') and numeric(value)) or (kind in TYPES and isinstance(value, TYPES[kind])) for kind in kinds)\u{a}")), List::cons(f8(Str::lit("        if not fits:\u{a}")), List::cons(f8(Str::lit("            problems.append(where + ' is not of type ' + repr(kinds))\u{a}")), List::cons(f8(Str::lit("            return\u{a}")), List::cons(f8(Str::lit("    if 'enum' in rule and value not in rule['enum']:\u{a}")), List::cons(f8(Str::lit("        problems.append(where + ' is not one of ' + repr(rule['enum']))\u{a}")), List::cons(f8(Str::lit("    if 'const' in rule and value != rule['const']:\u{a}")), List::cons(f8(Str::lit("        problems.append(where + ' is not ' + repr(rule['const']))\u{a}")), List::cons(f8(Str::lit("    if isinstance(value, dict):\u{a}")), List::cons(f8(Str::lit("        for name in rule.get('required', []):\u{a}")), List::cons(f8(Str::lit("            if name not in value:\u{a}")), List::cons(f8(Str::lit("                problems.append(where + ' lacks ' + name)\u{a}")), List::cons(f8(Str::lit("        known = rule.get('properties', {})\u{a}")), List::cons(f8(Str::lit("        for name, item in value.items():\u{a}")), List::cons(f8(Str::lit("            if name in known:\u{a}")), List::cons(f8(Str::lit("                check(item, known[name], where + '.' + name, problems)\u{a}")), List::cons(f8(Str::lit("            elif rule.get('additionalProperties') is False:\u{a}")), List::cons(f8(Str::lit("                problems.append(where + ' has ' + name + ', which the schema does not allow')\u{a}")), List::cons(f8(Str::lit("            else:\u{a}")), List::cons(f8(Str::lit("                check(item, rule.get('additionalProperties'), where + '.' + name, problems)\u{a}")), List::cons(f8(Str::lit("    if isinstance(value, list) and 'items' in rule:\u{a}")), List::cons(f8(Str::lit("        for index, item in enumerate(value):\u{a}")), List::cons(f8(Str::lit("            check(item, rule['items'], where + '[' + str(index) + ']', problems)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("holo = sorted(glob.glob(SLASH.join(['app', '.prism', 'build', '*.holo'])))[0]\u{a}")), List::cons(f8(Str::lit("record = SLASH.join(['target', 'gui-record.json'])\u{a}")), List::cons(f8(Str::lit("subprocess.run(sys.argv[1:] + ['record', holo, '--output', record], check=True, capture_output=True)\u{a}")), List::cons(f8(Str::lit("events = json.load(open(record))['events']\u{a}")), List::cons(f8(Str::lit("rows = tomllib.load(open(SLASH.join(['model', 'ids.toml']), 'rb'))['id']\u{a}")), List::cons(f8(Str::lit("prefix = 'studio' + chr(9) + 'gui' + chr(9)\u{a}")), List::cons(f8(Str::lit("journeys = {}\u{a}")), List::cons(f8(Str::lit("for event in events:\u{a}")), List::cons(f8(Str::lit("    request = bytes.fromhex(event['request']).decode('utf-8', 'replace')\u{a}")), List::cons(f8(Str::lit("    if not request.startswith(prefix):\u{a}")), List::cons(f8(Str::lit("        continue\u{a}")), List::cons(f8(Str::lit("    route = request[len(prefix):].split(chr(9))[0]\u{a}")), List::cons(f8(Str::lit("    entry = journeys.setdefault(route, {'route': prefix + route + chr(9), 'views': [], 'actions': {}})\u{a}")), List::cons(f8(Str::lit("    for line in bytes.fromhex(event['response']).decode('utf-8', 'replace').split(chr(10)):\u{a}")), List::cons(f8(Str::lit("        kind, _, rest = line.partition(' ')\u{a}")), List::cons(f8(Str::lit("        name, _, text = rest.partition(': ')\u{a}")), List::cons(f8(Str::lit("        if kind == 'journey':\u{a}")), List::cons(f8(Str::lit("            entry['journey'] = name\u{a}")), List::cons(f8(Str::lit("            entry['title'] = text\u{a}")), List::cons(f8(Str::lit("        elif kind == 'view' and name not in entry['views']:\u{a}")), List::cons(f8(Str::lit("            entry['views'].append(name)\u{a}")), List::cons(f8(Str::lit("        elif kind == 'action':\u{a}")), List::cons(f8(Str::lit("            states = entry['actions'].setdefault(name, [])\u{a}")), List::cons(f8(Str::lit("            state = text.split(',')[0]\u{a}")), List::cons(f8(Str::lit("            if state not in states:\u{a}")), List::cons(f8(Str::lit("                states.append(state)\u{a}")), List::cons(f8(Str::lit("problems = []\u{a}")), List::cons(f8(Str::lit("resources = []\u{a}")), List::cons(f8(Str::lit("reads = {}\u{a}")), List::cons(f8(Str::lit("for route, entry in sorted(journeys.items()):\u{a}")), List::cons(f8(Str::lit("    suite = 'gui-' + route\u{a}")), List::cons(f8(Str::lit("    rules = [{'id': row['id'], 'statement': row['statement']} for row in rows if row.get('suite') == suite and row.get('level') == 'build']\u{a}")), List::cons(f8(Str::lit("    journey = entry.get('journey', route)\u{a}")), List::cons(f8(Str::lit("    tree = {'journey': journey, 'title': entry.get('title', ''), 'route': entry['route'], 'views': entry['views'],\u{a}")), List::cons(f8(Str::lit("            'actions': [{'name': name, 'states': sorted(states)} for name, states in entry['actions'].items()], 'rules': rules}\u{a}")), List::cons(f8(Str::lit("    if not tree['views'] or not tree['actions'] or not rules:\u{a}")), List::cons(f8(Str::lit("        problems.append(journey + ' needs a view, an action and a build rule of suite ' + suite)\u{a}")), List::cons(f8(Str::lit("    uri = 'verdant-studio:' + SLASH + SLASH + 'gui' + SLASH + journey\u{a}")), List::cons(f8(Str::lit("    resources.append({'uri': uri, 'name': journey, 'title': tree['title'], 'mimeType': 'application' + SLASH + 'json',\u{a}")), List::cons(f8(Str::lit("                      'description': 'The ' + journey + ' journey of the Studio UI Prism: its route, views, actions and guards'})\u{a}")), List::cons(f8(Str::lit("    reads[journey] = {'cacheScope': 'public', 'resultType': 'complete', 'ttlMs': 0,\u{a}")), List::cons(f8(Str::lit("                      'contents': [{'uri': uri, 'mimeType': 'application' + SLASH + 'json', 'text': json.dumps(tree, sort_keys=True)}]}\u{a}")), List::cons(f8(Str::lit("listing = {'cacheScope': 'public', 'resultType': 'complete', 'ttlMs': 0, 'resources': resources}\u{a}")), List::cons(f8(Str::lit("check(listing, defs['ListResourcesResult'], 'resources-list', problems)\u{a}")), List::cons(f8(Str::lit("for journey, read in reads.items():\u{a}")), List::cons(f8(Str::lit("    check(read, defs['ReadResourceResult'], 'resources-read ' + journey, problems)\u{a}")), List::cons(f8(Str::lit("if not resources:\u{a}")), List::cons(f8(Str::lit("    problems.append('the app answers no journey of the Studio UI Prism')\u{a}")), List::cons(f8(Str::lit("for problem in problems:\u{a}")), List::cons(f8(Str::lit("    print('::error::the GUI view tree ' + problem)\u{a}")), List::cons(f8(Str::lit("if problems:\u{a}")), List::cons(f8(Str::lit("    sys.exit(1)\u{a}")), List::cons(f8(Str::lit("site = SLASH.join(['app', '.prism', 'site', 'surface'])\u{a}")), List::cons(f8(Str::lit("subprocess.run(['mkdir', '-p', site + SLASH + 'gui'], check=True)\u{a}")), List::cons(f8(Str::lit("json.dump(listing, open(site + SLASH + 'mcp-resources.json', 'w'), sort_keys=True)\u{a}")), List::cons(f8(Str::lit("for journey, read in reads.items():\u{a}")), List::cons(f8(Str::lit("    json.dump(read, open(site + SLASH + 'gui' + SLASH + journey + '.json', 'w'), sort_keys=True)\u{a}")), List::cons(f8(Str::lit("subprocess.run(['cp', SLASH.join(['surface', 'mcp-tools.json']), site], check=True)\u{a}")), List::cons(f8(Str::lit("print('accepted: ' + str(len(resources)) + ' journeys of the Studio UI Prism as MCP resources, valid against MCP-SCHEMA-2026-07-28: ' + ', '.join(sorted(reads)))\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f175(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m773 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m773.uncons() {
        None => {
            if f279(f95(f64(v0, v1))?)? {
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

pub fn f176(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m774 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m774.uncons() {
        None => {
            let v5: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f280(v5)?
                } else {
                    f281(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f245(f282(), f283(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f245(f282(), f283(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f284()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f177(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m775 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m775.uncons() {
        None => {
            let m776 = f285(f95(f64(v0.clone(), v1.clone()))?)?;
            match m776 {
                None => {
                    Ok(Adt12::C4(f286()))
                }
                Some(v4) => {
                    let m777 = f287(f95(f64(v0, v1))?)?;
                    match m777 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f288(v5)))
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

pub fn f178(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let v2: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
        Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
    } else {
        if f117(f289(f290(v2.clone())?), f51({
            let a779 = {
                let a778 = f291(f290(v2)?);
                length_list(a778)
            };
            let a780 = 0u64;
            nat_eq(a779, a780)
        })) {
            Ok(Adt12::C3(List::cons((f245(f282(), f283(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f245(f282(), f283(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt12::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f179(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m781 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m781.uncons() {
        None => {
            let m782 = f292(f95(f64(v0, v1))?)?;
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

pub fn f180(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m783 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m783.uncons() {
        None => {
            let v5: Bytes = f245(if v2 {
                f215()
            } else {
                f214()
            }, f283(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m784 = f226(f95(f64(v0.clone(), v1.clone()))?, v2)?;
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

pub fn f181(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m788 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m788.uncons() {
        None => {
            let m789 = f57(v0.clone(), v1.clone())?;
            match m789 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f293())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt2 = {
                            let m790 = f93(v4.clone(), f8(Str::lit("source")));
                            match m790 {
                                None => {
                                    Adt2::C9
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f92(f93(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f245(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f245(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m791 = v8;
                        match m791 {
                            None => {
                                f294(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f294(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f245(v9, f283(f58(v7.clone(), f8(Str::lit("url"))))?);
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
                                                if f32(f295(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f294(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f182(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m794 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m794.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f268(v4.clone())?;
            if f51(f98(f296(v5.clone()), f297())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f298(Str::lit("id"), v6.clone())?, List::cons(f298(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f298(Str::lit("artifact"), f216())?, List::cons(f298(Str::lit("checksum"), f295(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f216(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f299(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f300(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f122(f270(f267(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f183(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m795 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m795.uncons() {
        None => {
            let m796 = f57(v0, v1)?;
            match m796 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f301(f191(Some(v4))) {
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

pub fn f184(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m797 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f153(), List::<Bytes>::nil())));
    match m797.uncons() {
        None => {
            let m798 = f302(f95(f64(v0.clone(), v1))?)?;
            match m798 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt2> = f96(v0, f153())?;
                    let v6: List<Bytes> = f142(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f145(f142(v5.clone(), f8(Str::lit("at"))))?;
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
                                let a802 = f145(f125(v6.clone(), v7.clone()))?;
                                length_list(a802)
                            };
                            let a805 = {
                                let a803 = f125(v6.clone(), v7);
                                length_list(a803)
                            };
                            nat_eq(a804, a805)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f303(f153(), f227(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f122(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f185(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m806 = v2;
    match m806.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m807 = v4;
            match m807.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m808 = v6.clone();
                    match m808.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((_, _)) => {
                            let m809 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f304(), List::<Bytes>::nil())));
                            match m809.uncons() {
                                None => {
                                    let v11: Bytes = f122(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt2> = f96(v0, f304())?;
                                    if f305(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f303(f304(), f227(v13, List::cons(f306(v11.clone(), v12, v5, v3), List::<Adt2>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f186(v0: Bytes, p863: u64, p864: u64) -> R<bool> {
    let m862 = (p863, p864);
    {
        let mut m862 = m862;
        loop {
            let step = {
                let (v1, v2) = m862;
                let m810 = v2;
                if m810 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m810 - 1;
                    let m813 = {
                        let a811 = v0.clone();
                        let a812 = v1;
                        index_bytes(a811, a812)
                    };
                    match m813 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f307({
                                let v5: u8 = v4;
                                let a858 = {
                                    let a834 = {
                                        let a822 = {
                                            let a816 = {
                                                let a814 = v5;
                                                let a815 = 128u8;
                                                fixed_u8::bit_and(a814, a815)
                                            };
                                            let a817 = 128u8;
                                            if equal(a816, a817) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a823 = {
                                            let a820 = {
                                                let a818 = v5;
                                                let a819 = 64u8;
                                                fixed_u8::bit_and(a818, a819)
                                            };
                                            let a821 = 64u8;
                                            if equal(a820, a821) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a822, a823)?
                                    };
                                    let a835 = {
                                        let a832 = {
                                            let a826 = {
                                                let a824 = v5;
                                                let a825 = 32u8;
                                                fixed_u8::bit_and(a824, a825)
                                            };
                                            let a827 = 32u8;
                                            if equal(a826, a827) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a833 = {
                                            let a830 = {
                                                let a828 = v5;
                                                let a829 = 16u8;
                                                fixed_u8::bit_and(a828, a829)
                                            };
                                            let a831 = 16u8;
                                            if equal(a830, a831) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a832, a833)?
                                    };
                                    nat_add(a834, a835)?
                                };
                                let a859 = {
                                    let a856 = {
                                        let a844 = {
                                            let a838 = {
                                                let a836 = v5;
                                                let a837 = 8u8;
                                                fixed_u8::bit_and(a836, a837)
                                            };
                                            let a839 = 8u8;
                                            if equal(a838, a839) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a845 = {
                                            let a842 = {
                                                let a840 = v5;
                                                let a841 = 4u8;
                                                fixed_u8::bit_and(a840, a841)
                                            };
                                            let a843 = 4u8;
                                            if equal(a842, a843) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a844, a845)?
                                    };
                                    let a857 = {
                                        let a854 = {
                                            let a848 = {
                                                let a846 = v5;
                                                let a847 = 2u8;
                                                fixed_u8::bit_and(a846, a847)
                                            };
                                            let a849 = 2u8;
                                            if equal(a848, a849) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a855 = {
                                            let a852 = {
                                                let a850 = v5;
                                                let a851 = 1u8;
                                                fixed_u8::bit_and(a850, a851)
                                            };
                                            let a853 = 1u8;
                                            if equal(a852, a853) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a854, a855)?
                                    };
                                    nat_add(a856, a857)?
                                };
                                nat_add(a858, a859)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a860 = v1;
                                    let a861 = 1u64;
                                    nat_add(a860, a861)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m862 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(p869: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m868 = p869;
    {
        let mut m868 = m868;
        loop {
            let step = {
                let v0 = m868;
                let m865 = v0;
                match m865.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h866, _) = v2;
                            h866
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h867) = v2;
                                h867
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m868 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m873 = {
        let a871 = match v0 {
            Adt1::C0(_, _, _, h870) => {
                h870
            }
        };
        let a872 = f40(v1);
        index_list(a871, a872)
    };
    match m873 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f189(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), v1) {
        let m874 = f44(v0, 34u8)?;
        match m874.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m875 = v3;
                match m875.uncons() {
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

pub fn f190(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f308(v0, v1)
}

pub fn f191(v0: Option<Adt2>) -> List<Adt2> {
    let m876 = v0;
    match m876 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f309(v1)
        }
    }
}

pub fn f192(v0: Adt2, v1: Bytes) -> Adt2 {
    let m877 = v0;
    match m877 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r878) => {
            let v12 = (*r878).clone();
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
        Adt2::C7(r879) => {
            let v10 = (*r879).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r880) => {
            let v5 = (*r880).clone();
            Adt2::C8(std::rc::Rc::new(f192(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r881, r882) => {
            let v18 = (*r881).clone();
            let v19 = (*r882).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r883, r884) => {
            let v3 = (*r883).clone();
            let v4 = (*r884).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt2::C11(v2, std::rc::Rc::new(f310(v3, v1.clone())), std::rc::Rc::new(f192(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f192(v4, v1)))
            }
        }
    }
}

pub fn f193(v0: Adt2) -> R<Bytes> {
    let m885 = v0;
    match m885 {
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
            f311(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f312(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f313(v9, v10)
        }
        Adt2::C6(v11) => {
            f314(v11)
        }
        Adt2::C7(r886) => {
            let v12 = (*r886).clone();
            Ok(f315(f316(v12)?))
        }
        Adt2::C8(r887) => {
            let v13 = (*r887).clone();
            f317(f318(v13)?)
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

pub fn f194(v0: Adt1, p891: List<Adt13>) -> R<Option<Adt13>> {
    let m890 = p891;
    {
        let mut m890 = m890;
        loop {
            let step = {
                let v1 = m890;
                let m888 = v1;
                match m888.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f195(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h889, _) => {
                                h889
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
                Ok(next) => { m890 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Adt13) -> R<Bytes> {
    Ok(f245(f319(), f283(f320(v0))?))
}

pub fn f196(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f320(v0), List::<Bytes>::nil())))))
}

pub fn f197() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f198() -> Bytes {
    f321()
}

pub fn f199(v0: List<Adt13>) -> R<List<Bytes>> {
    let m892 = v0;
    match m892.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f195(v1)?, f199(v2)?))
        }
    }
}

pub fn f200(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m893 = v1;
    match m893.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f322(v5.clone(), v2.clone(), v3.clone(), f295(f64(v0.clone(), f195(v5)?))?, v4.clone())?), f200(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f201(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f323(v0.clone())?;
    let v2: u64 = f324(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a894 = v1;
        let a895 = v2;
        nat_sub(a894, a895)
    }))
}

pub fn f202(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a898 = v3;
    let a899 = f46(v0, v1, {
        let a896 = v2;
        let a897 = v1;
        nat_sub(a896, a897)
    });
    append_bytes(a898, a899)
}

pub fn f203(v0: Bytes, p912: u64, p913: u64) -> R<u64> {
    let m911 = (p912, p913);
    {
        let mut m911 = m911;
        loop {
            let step = {
                let (v1, v2) = m911;
                let m900 = v2;
                if m900 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m900 - 1;
                    let v4: u64 = f115(v0.clone(), v1)?;
                    let a901 = v4;
                    let a902 = 34u64;
                    if nat_eq(a901, a902) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a903 = v4;
                        let a904 = 92u64;
                        if nat_eq(a903, a904) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a905 = v4;
                            let a906 = 32u64;
                            if nat_lt(a905, a906) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a907 = v4;
                                let a908 = 256u64;
                                if nat_eq(a907, a908) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a909 = v1;
                                        let a910 = 1u64;
                                        nat_add(a909, a910)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m911 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Bytes, p939: u64, p940: u64, p941: Bytes) -> R<Option<(u64, Bytes)>> {
    let m938 = (p939, (p940, p941));
    {
        let mut m938 = m938;
        loop {
            let step = {
                let (v1, (v2, v3)) = m938;
                let m914 = v2;
                if m914 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m914 - 1;
                    let v5: u64 = f203(v0.clone(), v1, {
                        let a918 = {
                            let a916 = {
                                let a915 = v0.clone();
                                length_bytes(a915)
                            };
                            let a917 = v1;
                            nat_sub(a916, a917)
                        };
                        let a919 = 1u64;
                        nat_add(a918, a919)?
                    })?;
                    let v6: Bytes = {
                        let a922 = v3;
                        let a923 = f46(v0.clone(), v1, {
                            let a920 = v5;
                            let a921 = v1;
                            nat_sub(a920, a921)
                        });
                        append_bytes(a922, a923)
                    };
                    let v7: u64 = f115(v0.clone(), v5)?;
                    let a924 = v7;
                    let a925 = 34u64;
                    if nat_eq(a924, a925) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a926 = v5;
                            let a927 = 1u64;
                            nat_add(a926, a927)?
                        }, v6))))
                    } else {
                        let a928 = v7;
                        let a929 = 92u64;
                        if nat_eq(a928, a929) {
                            let m930 = f325(v0.clone(), v5, v6)?;
                            match m930 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a933 = {
                                        let (_, h931) = {
                                            let (_, h932) = v8.clone();
                                            h932
                                        };
                                        h931
                                    };
                                    let a934 = 0u64;
                                    if nat_eq(a933, a934) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h935, _) = v8;
                                            h935
                                        }, (v4, {
                                            let (h936, _) = {
                                                let (_, h937) = v8;
                                                h937
                                            };
                                            h936
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
                Ok(next) => { m938 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Bytes, v1: u64) -> R<u64> {
    let (h942, _) = f326(Fn3::F327(v0.clone()), {
        let a944 = {
            let a943 = v0;
            length_bytes(a943)
        };
        let a945 = 1u64;
        nat_add(a944, a945)?
    }, v1)?;
    Ok(h942)
}

pub fn f206(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f207(v0: Bytes) -> R<u64> {
    let (h946, _) = f326(Fn3::F328(v0.clone()), {
        let a948 = {
            let a947 = v0;
            length_bytes(a947)
        };
        let a949 = 1u64;
        nat_add(a948, a949)?
    }, 0u64)?;
    Ok(h946)
}

pub fn f208(v0: Bytes) -> R<Option<u64>> {
    if f329(v0.clone())? {
        Ok(Some({
            let (_, h950) = {
                let (h951, _) = f330(Fn4::F331(v0.clone()), {
                    let a952 = v0;
                    length_bytes(a952)
                }, (0u64, 0u64))?;
                h951
            };
            h950
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f209(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a954 = {
            let a953 = v1.clone();
            length_bytes(a953)
        };
        let a955 = f332(v1)?;
        nat_sub(a954, a955)
    });
    let v5: Bytes = {
        let a956 = v0;
        let a957 = v4.clone();
        append_bytes(a956, a957)
    };
    let v6: u64 = f207(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a959 = {
            let a958 = v5;
            length_bytes(a958)
        };
        let a960 = v6;
        nat_sub(a959, a960)
    });
    let v8: u64 = {
        let a961 = v4;
        length_bytes(a961)
    };
    let v9: u64 = {
        let a962 = v7.clone();
        length_bytes(a962)
    };
    let a963 = v9;
    let a964 = 0u64;
    if nat_eq(a963, a964) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f206(v2, {
            let a965 = v3;
            let a966 = v8;
            nat_lt(a965, a966)
        }) {
            let v10: u64 = if v2 {
                let a967 = v3;
                let a968 = v8;
                nat_add(a967, a968)?
            } else {
                let a969 = v8;
                let a970 = v3;
                nat_sub(a969, a970)
            };
            let v11: u64 = f332(v7.clone())?;
            let a971 = v10;
            let a972 = v11;
            if nat_le(a971, a972) {
                Ok(Some((f46(v7, 0u64, {
                    let a973 = v9;
                    let a974 = v10;
                    nat_sub(a973, a974)
                }), 0u64)))
            } else {
                let a977 = {
                    let a975 = v10;
                    let a976 = v11;
                    nat_sub(a975, a976)
                };
                let a978 = 19u64;
                if nat_le(a977, a978) {
                    Ok(Some((f46(v7, 0u64, {
                        let a979 = v9;
                        let a980 = v11;
                        nat_sub(a979, a980)
                    }), {
                        let a981 = v10;
                        let a982 = v11;
                        nat_sub(a981, a982)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a983 = v3;
                let a984 = v8;
                nat_sub(a983, a984)
            };
            let a987 = {
                let a985 = v9;
                let a986 = v12;
                nat_add(a985, a986)?
            };
            let a988 = 19u64;
            if nat_le(a987, a988) {
                Ok(Some(({
                    let a989 = v7;
                    let a990 = f333(v12);
                    append_bytes(a989, a990)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f210(v0: bool, v1: u64) -> bool {
    if v0 {
        let a991 = 0u64;
        let a992 = v1;
        if nat_lt(a991, a992) {
            let a993 = v1;
            let a994 = 9223372036854775808u64;
            nat_le(a993, a994)
        } else {
            false
        }
    } else {
        let a995 = v1;
        let a996 = 9223372036854775807u64;
        nat_le(a995, a996)
    }
}

pub fn f211(v0: Bytes, v1: u64) -> Option<u8> {
    let a997 = v0;
    let a998 = v1;
    index_bytes(a997, a998)
}

pub fn f212(v0: Fn2, p1002: Option<Bytes>, p1003: List<Bytes>) -> Option<Bytes> {
    let m1001 = (p1002, p1003);
    {
        let mut m1001 = m1001;
        loop {
            let step = {
                let (v1, v2) = m1001;
                let m999 = v2;
                match m999.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1000: Fn2 = v0.clone();
                            c1000.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1001 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1004 = v1;
    match m1004 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1007 = {
                    let a1005 = v3;
                    let a1006 = v0;
                    append_bytes(a1005, a1006)
                };
                let a1008 = v2;
                append_bytes(a1007, a1008)
            })
        }
    }
}

pub fn f214() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f215() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f216() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f217(v0: Adt1) -> R<List<Bytes>> {
    let m1009 = f268(f64(v0, f216()))?;
    match m1009.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f270(v2))
        }
    }
}

pub fn f218(v0: List<Adt2>) -> List<Bytes> {
    let m1010 = v0;
    match m1010.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1011 = f92(Some(v1));
            match m1011 {
                None => {
                    f218(v2)
                }
                Some(v3) => {
                    List::cons(v3, f218(v2))
                }
            }
        }
    }
}

pub fn f219(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1012 = v0;
    match m1012.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f219(v2))
        }
    }
}

pub fn f220(v0: List<Adt17>) -> List<(Bytes, Bytes)> {
    let m1013 = v0;
    match m1013.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt17::C0(h1014, _, _) => {
                    h1014
                }
            }, match v1 {
                Adt17::C0(_, _, h1015) => {
                    h1015
                }
            }), f220(v2))
        }
    }
}

pub fn f221(v0: Adt14, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Adt17>> {
    let v3: List<(Bytes, Bytes)> = f334(v1);
    let v4: List<(Bytes, Bytes)> = f335(v0, v2);
    f336(v3.clone(), f154(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f222(p1022: List<Bytes>, v1: List<(Bytes, Bytes)>, p1023: u64) -> R<List<Bytes>> {
    let m1021 = (p1022, p1023);
    {
        let mut m1021 = m1021;
        loop {
            let step = {
                let (v0, v2) = m1021;
                let m1016 = v2;
                if m1016 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1016 - 1;
                    let v4: List<Bytes> = f145(f125(v0.clone(), f337(v0.clone(), v1.clone())))?;
                    let a1019 = {
                        let a1017 = v4.clone();
                        length_list(a1017)
                    };
                    let a1020 = {
                        let a1018 = v0.clone();
                        length_list(a1018)
                    };
                    if nat_eq(a1019, a1020) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1021 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: List<Adt2>) -> List<Bytes> {
    let m1024 = v0;
    match m1024.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1025 = f93(v1, f8(Str::lit("std")));
            match m1025 {
                None => {
                    f223(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f224(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f223(v2))
                    } else {
                        f223(v2)
                    })
                }
            }
        }
    }
}

pub fn f224(v0: Adt2, v1: Bytes) -> bool {
    f51({
        let a1027 = {
            let a1026 = f58(v0, v1);
            length_bytes(a1026)
        };
        let a1028 = 0u64;
        nat_eq(a1027, a1028)
    })
}

pub fn f225(v0: Adt14, v1: Adt2) -> bool {
    if f338(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f338(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f226(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt2>>> {
    let m1029 = v0;
    match m1029.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f201(v2)?;
            let a1031 = {
                let a1030 = v4.clone();
                length_bytes(a1030)
            };
            let a1032 = 0u64;
            if nat_eq(a1031, a1032) {
                f226(v3, v1)
            } else {
                let m1033 = f43(v4)?;
                match m1033 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f339(v5.clone())
                        } else {
                            f340(v5.clone())
                        } {
                            let m1034 = f226(v3, v1)?;
                            match m1034 {
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

pub fn f227(v0: List<Adt2>, v1: List<Adt2>) -> List<Adt2> {
    let m1035 = v0;
    match m1035.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f227(v3, v1))
        }
    }
}

pub fn f228(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1036 = v1;
    match m1036.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f245(v0.clone(), v2), f228(v0, v3))
            } else {
                f228(v0, v3)
            }
        }
    }
}

pub fn f229() -> Bytes {
    f8(Str::lit("set -euo pipefail; repo=$1; rev=$2; shift 2; d=target/prismpm; if [ \"$(git -C $d rev-parse HEAD 2>/dev/null)\" != \"$rev\" ]; then rm -rf $d; git init -q $d; git -C $d remote add origin \"https://github.com/$repo\"; git -C $d sparse-checkout set --no-cone sdk/npm/prism-code; git -C $d fetch -q --depth=1 --filter=blob:none origin \"$rev\"; git -C $d checkout -q FETCH_HEAD; fi; exec node $d/sdk/npm/prism-code/bin/prism-code.mjs \"$@\""))
}

pub fn f230() -> Bytes {
    f8(Str::lit("047c868227ca45c5837bf07c1e7ed3b762d0f490"))
}

pub fn f231(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1037 = v1;
    match m1037.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f125({
                let a1039 = {
                    let a1038 = f201(v4.clone())?;
                    length_bytes(a1038)
                };
                let a1040 = 0u64;
                if nat_eq(a1039, a1040) {
                    List::<Bytes>::nil()
                } else {
                    f341(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f231(v0, v5, v2, v3)?))
        }
    }
}

pub fn f232(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f233() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f216(), List::<Bytes>::nil())))))))
}

pub fn f234(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1041 = v1;
    match m1041.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m1042 = f57(v0.clone(), v2.clone())?;
                match m1042 {
                    None => {
                        Ok(List::cons(v2, f234(v0, v3)?))
                    }
                    Some(_) => {
                        f234(v0, v3)
                    }
                }
            } else {
                f234(v0, v3)
            }
        }
    }
}

pub fn f235(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m1043 = v1;
    match m1043.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f138(v0.clone(), v2.clone()) {
                f235(v0, v3)
            } else {
                List::cons(v2, f235(v0, v3))
            }
        }
    }
}

pub fn f236() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f237(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m1044 = v0;
    match m1044.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f125(f342({
                let (h1045, _) = v2;
                h1045
            }, {
                let (_, h1046) = v2;
                h1046
            }, v1), f237(v3, v1))
        }
    }
}

pub fn f238() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::cons(f8(Str::lit(".github/scripts/readmit.sh")), List::<Bytes>::nil())))))
}

pub fn f239(v0: Adt1) -> R<Option<Bytes>> {
    f343(f344(v0, f238())?)
}

pub fn f240(v0: Adt1, p1050: List<Bytes>) -> R<Option<Bytes>> {
    let m1049 = p1050;
    {
        let mut m1049 = m1049;
        loop {
            let step = {
                let v1 = m1049;
                let m1047 = v1;
                match m1047.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1048 = f345(f64(v0.clone(), v2.clone()), f346())?;
                        match m1048 {
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
                Ok(next) => { m1049 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h1051, _, _, _, _, _) => {
            h1051
        }
    };
    let v2: Adt18 = f347(v1.clone())?;
    let v3: List<Bytes> = f348(v1.clone());
    let v4: List<Bytes> = f142(match v0.clone() {
        Adt16::C0(_, _, _, h1052, _, _) => {
            h1052
        }
    }, f8(Str::lit("path")));
    let m1054 = f349(v2, List::<u64>::nil(), f350(v1.clone(), match v0.clone() {
        Adt16::C0(_, h1053, _, _, _, _) => {
            h1053
        }
    }, 0u64)?);
    match m1054 {
        None => {
            if f352(f353(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1056) => {
                    h1056
                }
            }, f125(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1057) => {
                    h1057
                }
            }, v4.clone()))?, f353(v4.clone(), f125(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1058) => {
                    h1058
                }
            }, v4.clone()))?, f354(match v0.clone() {
                Adt16::C0(_, _, _, h1059, _, _) => {
                    h1059
                }
            })) {
                let m1063 = f355(f353(match v0.clone() {
                    Adt16::C0(_, _, _, _, h1060, _) => {
                        h1060
                    }
                }, f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, _, h1061, _) => {
                        h1061
                    }
                }))?, f353(v3.clone(), f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, _, h1062, _) => {
                        h1062
                    }
                }))?);
                match m1063 {
                    None => {
                        let m1066 = f356(f142(match v0.clone() {
                            Adt16::C0(_, h1065, _, _, _, _) => {
                                h1065
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m1066 {
                            None => {
                                let m1067 = f356(v4, List::<Bytes>::nil());
                                match m1067 {
                                    None => {
                                        let m1069 = f357(v1, match v0 {
                                            Adt16::C0(_, _, h1068, _, _, _) => {
                                                h1068
                                            }
                                        });
                                        match m1069 {
                                            None => {
                                                Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f122(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f351(f125(v3, match v0 {
                            Adt16::C0(_, _, _, _, h1064, _) => {
                                h1064
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f351(match v1 {
                Adt15::C0(_, _, _, _, h1055) => {
                    h1055
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f242(v0: List<Bytes>) -> List<Bytes> {
    f120(Fn1::F358, List::<Bytes>::nil(), v0)
}

pub fn f243(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f359(v0, v1)
}

pub fn f244(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f360(f361(f362(v0))?))
}

pub fn f245(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f246(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1070 = f57(v0, v2)?;
    match m1070 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f363(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1071 = v4.clone();
            match m1071.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1073 = f92(f93({
                        let m1072 = f93(v3.clone(), f8(Str::lit("source")));
                        match m1072 {
                            None => {
                                Adt2::C9
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1073 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f364(f365(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f247(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1074 = v0;
    match m1074.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f366(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1078 = f44(f46(v1.clone(), 6u64, {
                    let a1076 = {
                        let a1075 = v1;
                        length_bytes(a1075)
                    };
                    let a1077 = 6u64;
                    nat_sub(a1076, a1077)
                }), 34u8)?;
                match m1078.uncons() {
                    None => {
                        f247(v2)
                    }
                    Some((v3, v4)) => {
                        let m1079 = v4;
                        match m1079.uncons() {
                            None => {
                                f247(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f247(v2)?))
                            }
                        }
                    }
                }
            } else {
                f247(v2)
            }
        }
    }
}

pub fn f248(p1082: List<Bytes>, p1083: List<Bytes>) -> List<Bytes> {
    let m1081 = (p1082, p1083);
    {
        let mut m1081 = m1081;
        loop {
            let step = {
                let (v0, v1) = m1081;
                let m1080 = v0;
                match m1080.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f249() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f250(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1084 = v1;
    match m1084.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f367(v0.clone(), v3)?;
            if f52(f368(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f369(v5)?, f250(v0, v4, v2)?))
            } else {
                f250(v0, v4, v2)
            }
        }
    }
}

pub fn f251(v0: Adt1, p1088: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1087 = p1088;
    {
        let mut m1087 = m1087;
        loop {
            let step = {
                let v1 = m1087;
                let m1085 = v1;
                match m1085.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f367(v0.clone(), v3.clone())?;
                        if if f52(f368(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f100({
                                let m1086 = f93(v5.clone(), f8(Str::lit("schema")));
                                match m1086 {
                                    None => {
                                        Adt2::C9
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f369(v5)?))
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
                Ok(next) => { m1087 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h1089, _, _) => {
            h1089
        }
    }, f282())) {
        Ok(Adt12::C1(List::cons(f282(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f370(f147(match v0.clone() {
            Adt1::C0(_, h1090, _, _) => {
                h1090
            }
        }, f282())))?, v1.clone());
        let m1091 = f56(v0.clone(), List::cons(f153(), List::cons(f216(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1091.uncons() {
            None => {
                let m1092 = f56(v0.clone(), f371(v0.clone(), v2.clone())?);
                match m1092.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f372(v0.clone(), v2, v1)?;
                        let (h1093, _) = v7;
                        if h1093 {
                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                let (_, h1094) = v7;
                                h1094
                            }))
                        } else {
                            let m1095 = f124(v0.clone());
                            match m1095.uncons() {
                                None => {
                                    let m1096 = f126(v0.clone())?;
                                    match m1096.uncons() {
                                        None => {
                                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                                let (_, h1097) = v7;
                                                h1097
                                            }, List::cons(f7(f145(f373(f127(v0.clone())?, f135(v0.clone(), f136(v0.clone())?)?))?), List::cons(f374(v0)?, List::<Bytes>::nil()))))))
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

pub fn f253(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f375(f96(v0, f153())?)?)))
}

pub fn f254() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# The one way an agent changes this repository: dispatch this workflow with an\u{a}")), List::cons(f8(Str::lit("# operation of the API. It runs the certified command, and opens a pull request\u{a}")), List::cons(f8(Str::lit("# as the Verdant Studio GitHub App, which merges it once the studio check passes.\u{a}")), List::cons(f8(Str::lit("name: operate\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}")), List::cons(f8(Str::lit("# that defers to an open advance pull request always has a successor when that merges.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("    inputs:\u{a}")), List::cons(f8(Str::lit("      args:\u{a}")), List::cons(f8(Str::lit("        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}")), List::cons(f8(Str::lit("        required: true\u{a}")), List::cons(f8(Str::lit("        default: 'advance'\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input:\u{a}")), List::cons(f8(Str::lit("        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input_name:\u{a}")), List::cons(f8(Str::lit("        description: 'File name for input'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("        default: 'input'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Only operations that write the registers share one group. GitHub keeps one\u{a}")), List::cons(f8(Str::lit("# pending run per group, so a staging or propose-command run gets its own.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  # The wait is its own job under its own limit, so time spent waiting never comes out of the 30 minutes the operation has.\u{a}  wait:\u{a}    if: ${{ !startsWith(inputs.args, 'revert-merge ') }}\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 12\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      # Main moves about every 10 minutes, and refresh's regeneration of an API change plus its check takes\u{a}")), List::cons(f8(Str::lit("      # about 12, so an operation merged in that window pushes the change behind main again (#309, #315).\u{a}")), List::cons(f8(Str::lit("      # refresh keeps a regenerated change ready with its auto-merge set, so this run waits for every such\u{a}")), List::cons(f8(Str::lit("      # change whose studio check has not failed to merge first, then starts from the new main. It waits at\u{a}")), List::cons(f8(Str::lit("      # most 10 minutes, so neither queue stalls. A new propose-command waits too, so it never merges ahead of an\u{a}")), List::cons(f8(Str::lit("      # older change that is being regenerated; revert-merge, the repair of a broken path, never waits.\u{a}")), List::cons(f8(Str::lit("      - name: wait for the API changes that are next to merge (every operation but revert-merge)\u{a}")), List::cons(f8(Str::lit("")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          for try in $(seq 1 20); do\u{a}")), List::cons(f8(Str::lit("            next=''\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio propose-command\")) and .auto_merge != null and (.draft | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then next=\"$next #${pr}\"; fi\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f255() -> Bytes {
    f7(List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -z \"$next\" ]; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("            echo \"waiting for${next} to merge (check $try of 20)\"\u{a}")), List::cons(f8(Str::lit("            sleep 30\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          echo \"::warning::${next# } still open after 10 minutes, so this run goes on\"\u{a}  operate:\u{a}    needs: wait\u{a}    if: ${{ !cancelled() && (needs.wait.result == 'success' || needs.wait.result == 'skipped') }}\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}")), List::cons(f8(Str::lit("        if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          mkdir -p target/input\u{a}")), List::cons(f8(Str::lit("          cat > target/input/change.patch <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          # A sender can name the sha256 of the patch it means to send on a first line of its own, sha256:HEX, the digest of\u{a}")), List::cons(f8(Str::lit("          # the patch text as applied (or of the gzip's content). A slip in transcription then fails here in seconds.\u{a}")), List::cons(f8(Str::lit("          want=''\u{a}")), List::cons(f8(Str::lit("          if head -n 1 target/input/change.patch | grep -qE '^sha256:[0-9a-f]{64}$'; then want=$(head -n 1 target/input/change.patch | cut -c8-); tail -n +2 target/input/change.patch > target/input/change.body; mv target/input/change.body target/input/change.patch; fi\u{a}")), List::cons(f8(Str::lit("          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}")), List::cons(f8(Str::lit("          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$want\" ]; then got=$(sha256sum target/input/change.patch | cut -c1-64); if [ \"$got\" != \"$want\" ]; then echo \"::error::the patch decodes to sha256 $got, not the $want its sender named, so it changed in transcription: send it again\"; exit 1; fi; echo \"the patch matches the sha256 its sender named\"; fi\u{a}")), List::cons(f8(Str::lit("          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^admit/' -e '^lexlean-workspace.toml$' -e '^studio/lexlean.toml$' -e '^studio/production-request.json$' -e '^app/lexlean.toml$' -e '^stdlib/lexlean.toml$' -e '^.github/lexlean.pin$' || true)\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers, admit, lexlean-workspace.toml, the studio, app and stdlib lexlean.toml, studio/production-request.json and .github/lexlean.pin, not: $outside\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          # studio/lexlean.lock, readers/lexlean.lock, readers/xml/lexlean.lock and admit/lexlean.lock are generated, and the export below writes them again.\u{a}")), List::cons(f8(Str::lit("          # A VS ID this change adds that main already has becomes the next free one, and a pure append is\u{a}          # appended on main instead of patched (prepare-change.py, which refresh runs too).\u{a}          python3 .github/scripts/prepare-change.py prepare target/input/change.patch target/input/appends.json\u{a}          # git apply drops a file whose binary patch is corrupt, says so on one error line and still exits 0, so any error line stops the run here, before the export.\u{a}          # Contract and strata keep their entries on one line, so a change that only adds entries there is merged\u{a}          # into main by entry, and never conflicts with another change that adds others (merge-entries.py).\u{a}          python3 .github/scripts/merge-entries.py patch target/input/change.patch\u{a}          : > target/apply.err\u{a}          if grep -q '^diff --git' target/input/change.patch; then git apply --exclude=studio/lexlean.lock --exclude=readers/lexlean.lock --exclude=readers/xml/lexlean.lock --exclude=admit/lexlean.lock target/input/change.patch 2> target/apply.err || { sed 's/^/::error::/' target/apply.err; exit 1; }; fi\u{a}          if grep -q '^error' target/apply.err; then sed 's/^/::error::the patch did not apply whole: /' target/apply.err; exit 1; fi\u{a}          python3 .github/scripts/prepare-change.py append target/input/appends.json\u{a}")), List::cons(f8(Str::lit("          # The owner reads this pull request, so its body leads with the proposer's own account (the prose\u{a}")), List::cons(f8(Str::lit("          # before the first diff), then the definitions it changes. Checked before the export, which takes minutes.\u{a}")), List::cons(f8(Str::lit("          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}")), List::cons(f8(Str::lit("          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}")), List::cons(f8(Str::lit("            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The export refuses Lean output holding an absolute path (LLV7006) only at its end, so refuse here, in seconds,\u{a}")), List::cons(f8(Str::lit("          # an added string literal that LexLean's own test would refuse, read as Lean prints it (prepare-change.py lint).\u{a}")), List::cons(f8(Str::lit("          python3 .github/scripts/prepare-change.py lint target/input/change.patch\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          # The one export, shared with refresh and run end to end by the studio check.\u{a}")), List::cons(f8(Str::lit("          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f376(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("            tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}")), List::cons(f8(Str::lit("          {\u{a}")), List::cons(f8(Str::lit("            cat target/summary.md\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"### Definitions this adds or changes\"\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f256() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            {\u{a}")), List::cons(f8(Str::lit("              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}")), List::cons(f8(Str::lit("              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}")), List::cons(f8(Str::lit("            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            slash=/\u{a}")), List::cons(f8(Str::lit("            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            head -n 8 target/export.log.phases || true\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"<${slash}details>\"\u{a}")), List::cons(f8(Str::lit("          } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("      - id: command\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: |\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_readers\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_xml\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_admit\u{a}")), List::cons(f8(Str::lit("          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}")), List::cons(f8(Str::lit("        if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          target/lexlean/bin/lexlean production verify-artifact certified\u{a}")), List::cons(f8(Str::lit("          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}")), List::cons(f8(Str::lit("          if [ -d certified-readers ]; then\u{a}")), List::cons(f8(Str::lit("            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}")), List::cons(f8(Str::lit("            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}")), List::cons(f8(Str::lit("      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("          INPUT_NAME: ${{ inputs.input_name }}\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail -f\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          # read-* operations run in their reader project's command when it is built (the XML ones in verdant_xml), the admission operations in verdant_admit, every other in the studio command.\u{a}")), List::cons(f8(Str::lit("          vs() { case \"$1\" in admit|admit-encoding|admit-registry|admit-translated|propose-encoding) if [ -x target/release/verdant_admit ]; then target/release/verdant_admit \"$@\"; return; fi;; read-xml|read-xsd|read-xsd-components) if [ -x target/release/verdant_xml ]; then target/release/verdant_xml \"$@\"; return; fi;; read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f257() -> Bytes {
    f7(List::cons(f8(Str::lit("          # An App pull request whose files already hold on main has nothing left to merge,\u{a}")), List::cons(f8(Str::lit("          # so it is closed; the propose-command ones change the API and stay open until they merge.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}")), List::cons(f8(Str::lit("              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}")), List::cons(f8(Str::lit("              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}")), List::cons(f8(Str::lit("              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}")), List::cons(f8(Str::lit("                exit 0\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            # A change to a reader re-derives the encodings it printed before the studio check replays them.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then\u{a}")), List::cons(f8(Str::lit("              readmitted=$(bash .github/scripts/readmit.sh target/release/verdant_readers; if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi)\u{a}")), List::cons(f8(Str::lit("              if [ -n \"$readmitted\" ]; then\u{a}")), List::cons(f8(Str::lit("                { echo; echo \"### Encodings admitted again\"; echo; echo \"This change alters what a reader prints for these admitted encodings, so each was admitted again through read-admit with the command and citation its row records:\"; echo; sed 's/^/- /' <<< \"$readmitted\"; } >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            cat target/operation.txt\u{a}")), List::cons(f8(Str::lit("          elif [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            : > target/operation.txt\u{a}")), List::cons(f8(Str::lit("            last=''\u{a}")), List::cons(f8(Str::lit("            skipped=''\u{a}")), List::cons(f8(Str::lit("            # next prints one operation it can dispatch, or else the lines that say why the\u{a}")), List::cons(f8(Str::lit("            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}")), List::cons(f8(Str::lit("            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}")), List::cons(f8(Str::lit("              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("              if vs $op > target/attempt.txt 2>&1; then\u{a}")), List::cons(f8(Str::lit("                tee -a target/operation.txt < target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                last=$op\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                cat target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                binding=${op#* }\u{a}")), List::cons(f8(Str::lit("                echo \"::warning::$op was refused; the other bindings continue\"\u{a}")), List::cons(f8(Str::lit("                echo \"refused: $op\" >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}")), List::cons(f8(Str::lit("                skipped=\"$skipped $binding\"\u{a}")), List::cons(f8(Str::lit("                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            t0=$EPOCHREALTIME\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}")), List::cons(f8(Str::lit("            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}")), List::cons(f8(Str::lit("            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}")), List::cons(f8(Str::lit("              echo \"waiting merge: #${pr} changes the API, and merges once the studio check passes\" >> target/waiting.txt\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}")), List::cons(f8(Str::lit("          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f258() -> Bytes {
    f7(List::cons(f8(Str::lit("            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}")), List::cons(f8(Str::lit("            # good main commit, it reverts the merge on main as the App, and merges once the studio check passes.\u{a}")), List::cons(f8(Str::lit("            merge=${ARGS#revert-merge }\u{a}")), List::cons(f8(Str::lit("            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B revert origin/main\u{a}")), List::cons(f8(Str::lit("            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("            git revert -m 1 --no-commit \"$merge\"\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::cons(f8(Str::lit("            vs $ARGS | tee target/operation.txt\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("      - name: open a pull request as the Verdant Studio app\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          git add -A\u{a}")), List::cons(f8(Str::lit("          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}")), List::cons(f8(Str::lit("          verdant_studio $ARGS\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("          $(cat target/operation.txt)\u{a}")), List::cons(f8(Str::lit("          MSG\u{a}")), List::cons(f8(Str::lit("          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}")), List::cons(f8(Str::lit("          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}")), List::cons(f8(Str::lit("            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          title=\"verdant_studio $ARGS\"\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}")), List::cons(f8(Str::lit("          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}")), List::cons(f8(Str::lit("          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}")), List::cons(f8(Str::lit("          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}")), List::cons(f8(Str::lit("          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}")), List::cons(f8(Str::lit("          pr=\"\"\u{a}")), List::cons(f8(Str::lit("          for try in 1 2 3 4; do\u{a}")), List::cons(f8(Str::lit("            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            pr=\"\"\u{a}")), List::cons(f8(Str::lit("            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}")), List::cons(f8(Str::lit("            sleep $((2 ** try))\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          echo \"opened #${pr}\"\u{a}")), List::cons(f8(Str::lit("          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("          # A change to the API itself (propose-command, revert-merge) merges as every App pull request does:\u{a}")), List::cons(f8(Str::lit("          # the owner automated these approvals (2026-11-26), so the studio check, which runs the API's own\u{a}")), List::cons(f8(Str::lit("          # end-to-end propose run, is its gate. refresh keeps its auto-merge while it regenerates it.\u{a}")), List::cons(f8(Str::lit("          # An API change merges before every change opened after it, however long each check takes: #478 and\u{a}")), List::cons(f8(Str::lit("          # #479 merged ahead of #474 after the wait above gave up, and each pushed it behind main again.\u{a}")), List::cons(f8(Str::lit("          # So while an older propose-command pull request keeps its auto-merge and its studio check has not\u{a}")), List::cons(f8(Str::lit("          # failed, this one is left ready without auto-merge, and refresh sets it once that one merges.\u{a}")), List::cons(f8(Str::lit("          # revert-merge, the repair of a broken path, never waits.\u{a}")), List::cons(f8(Str::lit("          ahead=''\u{a}")), List::cons(f8(Str::lit("          if [ \"${ARGS#revert-merge }\" = \"$ARGS\" ]; then\u{a}")), List::cons(f8(Str::lit("            for older in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select(.user.type == \\\"Bot\\\" and (.title | startswith(\\\"verdant_studio propose-command\\\")) and .auto_merge != null and .number < ${pr}) | .number\"); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${older}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then ahead=\"$ahead #${older}\"; fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}")), List::cons(f8(Str::lit("          if [ \"$gate\" = merge ] && [ -n \"$ahead\" ]; then\u{a}")), List::cons(f8(Str::lit("            echo \"::notice::#${pr} merges after${ahead}: refresh sets its auto-merge once they merge\"\u{a}")), List::cons(f8(Str::lit("          elif [ \"$gate\" = merge ]; then\u{a}")), List::cons(f8(Str::lit("            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f259() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::<Bytes>::nil())))
}

pub fn f260() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# When main moves, every open propose-command pull request that no longer merges\u{a}")), List::cons(f8(Str::lit("# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}")), List::cons(f8(Str::lit("# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}")), List::cons(f8(Str::lit("# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}")), List::cons(f8(Str::lit("# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}")), List::cons(f8(Str::lit("# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}")), List::cons(f8(Str::lit("# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("# The exception is one whose auto-merge is set, the next change to merge: it stays ready and keeps its\u{a}")), List::cons(f8(Str::lit("# auto-merge, so it merges as soon as its regeneration passes the studio check. It is\u{a}")), List::cons(f8(Str::lit("# regenerated first, and only the oldest such one at a time, since two changes to the\u{a}")), List::cons(f8(Str::lit("# API conflict on their export (#309 and #315 were pushed behind main this way).\u{a}")), List::cons(f8(Str::lit("name: refresh\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}")), List::cons(f8(Str::lit("# operate run completes, which needs no token that may dispatch workflows, so every\u{a}")), List::cons(f8(Str::lit("# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_run:\u{a}")), List::cons(f8(Str::lit("    workflows: [operate]\u{a}")), List::cons(f8(Str::lit("    types: [completed]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A running refresh always finishes, because an export takes minutes and main moves more\u{a}")), List::cons(f8(Str::lit("# often than that; GitHub keeps only the newest pending run behind it.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: refresh\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  find:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 15\u{a}")), List::cons(f8(Str::lit("    outputs:\u{a}")), List::cons(f8(Str::lit("      stale: ${{ steps.stale.outputs.stale }}\u{a}")), List::cons(f8(Str::lit("      matrix: ${{ steps.stale.outputs.matrix }}\u{a}")), List::cons(f8(Str::lit("      auto: ${{ steps.stale.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - id: stale\u{a}")), List::cons(f8(Str::lit("        name: find the propose-command pull requests that conflict with main\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          stale=''\u{a}")), List::cons(f8(Str::lit("          auto=''\u{a}")), List::cons(f8(Str::lit("          # Oldest first, so the oldest one whose auto-merge is set is the one regenerated now.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}")), List::cons(f8(Str::lit("            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}")), List::cons(f8(Str::lit("            # which a cancelled run may have left behind.\u{a}")), List::cons(f8(Str::lit("            mergeable=null\u{a}")), List::cons(f8(Str::lit("            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}")), List::cons(f8(Str::lit("              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f261() -> Bytes {
    f7(List::cons(f8(Str::lit("              if [ \"$mergeable\" != null ]; then break; fi\u{a}")), List::cons(f8(Str::lit("              sleep 10\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            # One that changes .github is exported again by the studio check, byte for byte, so it also fails\u{a}")), List::cons(f8(Str::lit("            # when main changed its export inputs since it was exported, even without a conflict: readers\u{a}")), List::cons(f8(Str::lit("            # depends on app, so admissions merged during its export (#357 and #361 under #362) turn it red.\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = true ] && [ \"$(gh api --paginate \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files\" --jq '.[].filename' | grep -c '^\\.github/' || true)\" != 0 ]; then\u{a}")), List::cons(f8(Str::lit("              base=$(gh api \"repos/${GITHUB_REPOSITORY}/compare/main...$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\" --jq .merge_base_commit.sha)\u{a}")), List::cons(f8(Str::lit("              moved=$(gh api \"repos/${GITHUB_REPOSITORY}/compare/${base}...main\" --jq '[.files[]?.filename | select(test(\"^(app|stdlib|studio|readers)/\"))] | length')\u{a}")), List::cons(f8(Str::lit("              if [ \"$moved\" != 0 ]; then echo \"#${pr}: main changed ${moved} of its export inputs since it was exported\"; mergeable=false; fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = false ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              # Drafting would drop its auto-merge, so it stays ready. A later one waits: it is\u{a}")), List::cons(f8(Str::lit("              # regenerated once this one merges, when it would conflict again anyway.\u{a}")), List::cons(f8(Str::lit("              if [ -z \"$auto\" ]; then auto=$pr; else echo \"#${pr} waits for #${auto} to merge\"; fi\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = false ]; then\u{a}")), List::cons(f8(Str::lit("              stale=\"$stale $pr\"\u{a}")), List::cons(f8(Str::lit("              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}")), List::cons(f8(Str::lit("              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          # A change operate left without auto-merge, behind an older API change, gets it here once no\u{a}")), List::cons(f8(Str::lit("          # propose-command pull request keeps its auto-merge with a studio check that has not failed: oldest\u{a}")), List::cons(f8(Str::lit("          # first, up to and including the first API change, so API changes still merge one at a time in order.\u{a}")), List::cons(f8(Str::lit("          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > \"$RUNNER_TEMP/rules.json\"\u{a}")), List::cons(f8(Str::lit("          if [ \"$(jq '[.[] | select(.type == \"required_status_checks\") | .parameters.required_status_checks[]?.context] | index(\"studio\") != null' \"$RUNNER_TEMP/rules.json\")\" = true ]; then\u{a}")), List::cons(f8(Str::lit("            armed=''\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select(.user.login == \\\"${SLUG}[bot]\\\" and (.title | startswith(\\\"verdant_studio propose-command\\\")) and .auto_merge != null) | .number\"); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then armed=\"$armed #${pr}\"; fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -z \"$armed\" ]; then\u{a}")), List::cons(f8(Str::lit("              for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select(.user.login == \\\"${SLUG}[bot]\\\" and .auto_merge == null and (.draft | not)) | .number\"); do\u{a}")), List::cons(f8(Str::lit("                if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\" = true ]; then\u{a}")), List::cons(f8(Str::lit("                  gh pr merge \"$pr\" --auto --squash --delete-branch --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("                  echo \"#${pr}: auto-merge set\"\u{a}")), List::cons(f8(Str::lit("                  if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.title | startswith(\"verdant_studio propose-command\")')\" = true ]; then break; fi\u{a}")), List::cons(f8(Str::lit("                fi\u{a}")), List::cons(f8(Str::lit("              done\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$auto\" ]; then stale=\" $auto$stale\"; fi\u{a}")), List::cons(f8(Str::lit("          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"auto=$auto\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("  regenerate:\u{a}")), List::cons(f8(Str::lit("    needs: find\u{a}")), List::cons(f8(Str::lit("    if: needs.find.outputs.stale != ''\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 60\u{a}")), List::cons(f8(Str::lit("    strategy:\u{a}")), List::cons(f8(Str::lit("      fail-fast: false\u{a}")), List::cons(f8(Str::lit("      matrix:\u{a}")), List::cons(f8(Str::lit("        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          fetch-depth: 0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          STALE: ${{ matrix.pr }}\u{a}")), List::cons(f8(Str::lit("          AUTO: ${{ needs.find.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A regeneration of the one whose auto-merge is set that fails leaves it a draft, which drops\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f262() -> Bytes {
    f7(List::cons(f8(Str::lit("          # its auto-merge, so it is no longer the next change to merge and nothing is handed over red.\u{a}")), List::cons(f8(Str::lit("          trap 'rc=$?; if [ \"$rc\" != 0 ] && [ \"$STALE\" = \"$AUTO\" ]; then gh pr ready --undo \"$STALE\" || true; fi' EXIT\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            mkdir -p target\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          failed=''\u{a}")), List::cons(f8(Str::lit("          for pr in $STALE; do\u{a}")), List::cons(f8(Str::lit("            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}")), List::cons(f8(Str::lit("            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}")), List::cons(f8(Str::lit("            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("            # The one whose auto-merge is set stays ready, so that it keeps its auto-merge.\u{a}")), List::cons(f8(Str::lit("            if [ \"$pr\" != \"$AUTO\" ]; then gh pr ready --undo \"$pr\" || true; fi\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin \"$branch\" main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B refresh origin/main\u{a}")), List::cons(f8(Str::lit("            git clean -fdq\u{a}")), List::cons(f8(Str::lit("            # readers/lexlean.lock and readers/xml/lexlean.lock are generated: the export below writes them again, so they are left out of the\u{a}")), List::cons(f8(Str::lit("            # change, and two reader pull requests never conflict on it.\u{a}")), List::cons(f8(Str::lit("            git diff --full-index \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml ':!readers/lexlean.lock' ':!readers/xml/lexlean.lock' ':!admit/lexlean.lock' > target/change.patch\u{a}")), List::cons(f8(Str::lit("            # app/src/Contract.lex.tex and model/strata.json each keep their entries on one line, so two changes\u{a}")), List::cons(f8(Str::lit("            # that each add entries there always conflict. A conflict on those two files alone is merged by entry\u{a}")), List::cons(f8(Str::lit("            # (merge-entries.py): main's file, then each entry the pull request adds that main does not have.\u{a}")), List::cons(f8(Str::lit("            # git apply drops a file whose binary patch is corrupt and still exits 0, so that error counts as not applying.\u{a}            # The same path as propose-command: renumber colliding VS IDs, move pure appends aside, and merge\u{a}            # Contract's and strata's entries by entry, so a lane that loses an ID or entry race never dispatches again.\u{a}            python3 .github/scripts/prepare-change.py prepare target/change.patch target/appends.json\u{a}            python3 .github/scripts/merge-entries.py patch target/change.patch\u{a}            applied=yes\u{a}            : > target/apply.err\u{a}            if grep -q '^diff --git' target/change.patch; then git apply --3way target/change.patch 2> target/apply.err || applied=no; fi\u{a}            if grep -q '^error: corrupt' target/apply.err; then applied=no; fi\u{a}            if [ \"$applied\" = no ]; then\u{a}")), List::cons(f8(Str::lit("              merged=no\u{a}")), List::cons(f8(Str::lit("              if [ -n \"$(git diff --name-only --diff-filter=U)\" ] && [ -z \"$(git diff --name-only --diff-filter=U | grep -v -x -e app/src/Contract.lex.tex -e model/strata.json || true)\" ]; then\u{a}")), List::cons(f8(Str::lit("                merged=yes\u{a}")), List::cons(f8(Str::lit("                base=$(git merge-base origin/main \"origin/$branch\")\u{a}")), List::cons(f8(Str::lit("                for f in $(git diff --name-only --diff-filter=U); do\u{a}")), List::cons(f8(Str::lit("                  git show \"$base:$f\" > target/entries-base\u{a}")), List::cons(f8(Str::lit("                  git show \"origin/$branch:$f\" > target/entries-pr\u{a}")), List::cons(f8(Str::lit("                  git show \"origin/main:$f\" > target/entries-main\u{a}")), List::cons(f8(Str::lit("                  if python3 .github/scripts/merge-entries.py merge target/entries-base target/entries-pr target/entries-main \"$f\" > target/entries-merged; then cp target/entries-merged \"$f\"; git add \"$f\"; else merged=no; fi\u{a}")), List::cons(f8(Str::lit("                done\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("              if [ \"$merged\" = yes ]; then\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::#${pr}: merged the entries it adds to Contract and strata into main's by entry\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}")), List::cons(f8(Str::lit("                git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("                failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}")), List::cons(f8(Str::lit("                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("                continue\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            python3 .github/scripts/prepare-change.py append target/appends.json\u{a}")), List::cons(f8(Str::lit("            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f376(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("              tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("              git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("              failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("              continue\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            rm -rf studio/.lexlean/production readers/.lexlean/production readers/xml/.lexlean/production admit/.lexlean/production\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            # The regenerated change re-derives the encodings whose reader output it changes, as propose-command does.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then bash .github/scripts/readmit.sh target/release/verdant_readers; fi\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("            git add -A\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f263() -> Bytes {
    f7(List::cons(f8(Str::lit("            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml; } > target/message.txt\u{a}")), List::cons(f8(Str::lit("            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}")), List::cons(f8(Str::lit("            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}")), List::cons(f8(Str::lit("            gh pr ready \"$pr\" || true\u{a}")), List::cons(f8(Str::lit("            echo \"::notice::regenerated #${pr} on the new main\"\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))))))
}

pub fn f264() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# The owner's acceptance test for the whole pipeline (REQ-SELF-HOSTED, REQ-PRISM-GENERATION): prompt Studio\u{a}")), List::cons(f8(Str::lit("# with the seed \"Verdant Studio\" and require every facet the Prism lists to have its media type at the\u{a}")), List::cons(f8(Str::lit("# fidelity its generator's theorems state. Run from the repository root after the build, so the app is not\u{a}")), List::cons(f8(Str::lit("# proved again: prism-acceptance.py COMMAND... runs COMMAND run app --input REQUEST for each request.\u{a}")), List::cons(f8(Str::lit("import math, struct, subprocess, sys, xml.etree.ElementTree as tree\u{a}")), List::cons(f8(Str::lit("SEED = 'Verdant Studio'\u{a}")), List::cons(f8(Str::lit("MIDI_HEADER = bytes.fromhex('4d54686400000006000000010060')\u{a}")), List::cons(f8(Str::lit("FMT = bytes.fromhex('0100010080bb00000077010002001000')\u{a}")), List::cons(f8(Str::lit("# SONY-ASWG-R001 (the AswgLoudnessTarget catalog module), the home-based platform target REQ-ENGINE-FIDELITY's high-end gaming\u{a}")), List::cons(f8(Str::lit("# systems name: -24 (+-2) LKFS integrated, at most -1 dBTP true peak, both measured as ITU-R-BS-1770-5 states.\u{a}")), List::cons(f8(Str::lit("TARGET, TOLERANCE, TRUE_PEAK = -24.0, 2.0, -1.0\u{a}")), List::cons(f8(Str::lit("# BS.1770-5 Annex 1 pre-filter and RLB filter at 48 kHz, as (b0, b1, b2, a1, a2).\u{a}")), List::cons(f8(Str::lit("K_WEIGHTING = ((1.53512485958697, -2.69169618940638, 1.19839281085285, -1.69065929318241, 0.73248077421585),\u{a}")), List::cons(f8(Str::lit("               (1.0, -2.0, 1.0, -1.99004745483398, 0.99007225036621))\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def loudness(x):\u{a}")), List::cons(f8(Str::lit("    # Annex 1: K-weighting, 400 ms blocks overlapping by three quarters, absolute gate -70 LKFS, relative gate -10 LU.\u{a}")), List::cons(f8(Str::lit("    for b0, b1, b2, a1, a2 in K_WEIGHTING:\u{a}")), List::cons(f8(Str::lit("        y, x1, x2, y1, y2 = [], 0.0, 0.0, 0.0, 0.0\u{a}")), List::cons(f8(Str::lit("        for v in list(x):\u{a}")), List::cons(f8(Str::lit("            o = b0 * v + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2\u{a}")), List::cons(f8(Str::lit("            x2, x1, y2, y1 = x1, v, y1, o\u{a}")), List::cons(f8(Str::lit("            y.append(o)\u{a}")), List::cons(f8(Str::lit("        x = y\u{a}")), List::cons(f8(Str::lit("    squares = [0.0]\u{a}")), List::cons(f8(Str::lit("    for v in list(x):\u{a}")), List::cons(f8(Str::lit("        squares.append(squares[-1] + v * v)\u{a}")), List::cons(f8(Str::lit("    size, hop = 19200, 4800\u{a}")), List::cons(f8(Str::lit("    blocks = [(squares[at + size] - squares[at]) * size ** -1 for at in range(0, len(x) - size + 1, hop)]\u{a}")), List::cons(f8(Str::lit("    level = lambda power: -0.691 + 10 * math.log10(power) if power > 0 else -math.inf\u{a}")), List::cons(f8(Str::lit("    gated = [power for power in blocks if level(power) > -70]\u{a}")), List::cons(f8(Str::lit("    if not gated:\u{a}")), List::cons(f8(Str::lit("        return -math.inf\u{a}")), List::cons(f8(Str::lit("    relative = level(sum(gated) * len(gated) ** -1) - 10\u{a}")), List::cons(f8(Str::lit("    gated = [power for power in gated if level(power) > relative]\u{a}")), List::cons(f8(Str::lit("    return level(sum(gated) * len(gated) ** -1)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def true_peak(x):\u{a}")), List::cons(f8(Str::lit("    # Annex 2: oversample by 4 and take the greatest magnitude, here with a 48-tap Hann-windowed sinc interpolator.\u{a}")), List::cons(f8(Str::lit("    taps = [[(math.sin(math.pi * (k - 24 + phase * 0.25)) * (math.pi * (k - 24 + phase * 0.25)) ** -1 if k - 24 + phase * 0.25 else 1.0)\u{a}")), List::cons(f8(Str::lit("             * (0.5 - 0.5 * math.cos(2 * math.pi * (k + phase * 0.25) * 48 ** -1)) for k in range(48)] for phase in range(1, 4)]\u{a}")), List::cons(f8(Str::lit("    peak = max(map(abs, x), default=0.0)\u{a}")), List::cons(f8(Str::lit("    for at in range(len(x)):\u{a}")), List::cons(f8(Str::lit("        if abs(x[at]) < peak * 0.7:\u{a}")), List::cons(f8(Str::lit("            continue\u{a}")), List::cons(f8(Str::lit("        window = x[max(0, at - 23):at + 25]\u{a}")), List::cons(f8(Str::lit("        for row in taps:\u{a}")), List::cons(f8(Str::lit("            peak = max(peak, abs(sum(c * v for c, v in zip(row[23 - (at - max(0, at - 23)):], window))))\u{a}")), List::cons(f8(Str::lit("    return 20 * math.log10(peak) if peak > 0 else -math.inf\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("SVG_ROOT = '{http:' + chr(47) * 2 + 'www.w3.org' + chr(47) + '2000' + chr(47) + 'svg}svg'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def run(request):\u{a}")), List::cons(f8(Str::lit("    got = subprocess.run(sys.argv[1:] + ['run', 'app', '--input', request, '--rm'], capture_output=True)\u{a}")), List::cons(f8(Str::lit("    if got.returncode != 0:\u{a}")), List::cons(f8(Str::lit("        raise SystemExit('::error::' + repr(request) + ' failed: ' + got.stderr.decode(errors='replace')[-400:])\u{a}")), List::cons(f8(Str::lit("    return got.stdout\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f265() -> Bytes {
    f7(List::cons(f8(Str::lit("def midi(data):\u{a}")), List::cons(f8(Str::lit("    # Midi.lex.tex writes_a_format_0_file: format 0, one track, 96 ticks per quarter note.\u{a}")), List::cons(f8(Str::lit("    if not data.startswith(MIDI_HEADER) or data[14:18] != b'MTrk':\u{a}")), List::cons(f8(Str::lit("        return 'is no format 0 SMF with one track at 96 ticks per quarter note'\u{a}")), List::cons(f8(Str::lit("    if struct.unpack('>I', data[18:22])[0] != len(data) - 22:\u{a}")), List::cons(f8(Str::lit("        return 'has a track length that is not the rest of the file'\u{a}")), List::cons(f8(Str::lit("    # Coverage (Der from REQ-PRISM-GENERATION): every facet carries every feature. The SMF carries each as a Marker\u{a}")), List::cons(f8(Str::lit("    # meta event (FF 06, MIDI-SMF-RP-001-1.0-PAGE), in the order the features route lists them.\u{a}")), List::cons(f8(Str::lit("    markers, at = [], 22\u{a}")), List::cons(f8(Str::lit("    while at < len(data):\u{a}")), List::cons(f8(Str::lit("        while data[at] & 128:\u{a}")), List::cons(f8(Str::lit("            at += 1\u{a}")), List::cons(f8(Str::lit("        at += 1\u{a}")), List::cons(f8(Str::lit("        if data[at] == 255:\u{a}")), List::cons(f8(Str::lit("            kind, size, at = data[at + 1], 0, at + 2\u{a}")), List::cons(f8(Str::lit("            while data[at] & 128:\u{a}")), List::cons(f8(Str::lit("                size, at = size * 128 + (data[at] & 127), at + 1\u{a}")), List::cons(f8(Str::lit("            size, at = size * 128 + data[at], at + 1\u{a}")), List::cons(f8(Str::lit("            if kind == 6:\u{a}")), List::cons(f8(Str::lit("                markers.append(data[at:at + size].decode(errors='replace'))\u{a}")), List::cons(f8(Str::lit("            at += size\u{a}")), List::cons(f8(Str::lit("        else:\u{a}")), List::cons(f8(Str::lit("            at += 2 if data[at] >> 4 in (12, 13) else 3\u{a}")), List::cons(f8(Str::lit("    if markers != FEATURES:\u{a}")), List::cons(f8(Str::lit("        return 'marks the features ' + repr(markers) + ', not ' + repr(FEATURES)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def wave(data):\u{a}")), List::cons(f8(Str::lit("    # Bwf.lex.tex format_is_pcm_mono_48000_16 and wave_orders_bext_fmt_data: bext (646 bytes), fmt, data.\u{a}")), List::cons(f8(Str::lit("    if data[:4] != b'RIFF' or data[8:12] != b'WAVE' or struct.unpack('<I', data[4:8])[0] != len(data) - 8:\u{a}")), List::cons(f8(Str::lit("        return 'is no RIFF WAVE file of its own length'\u{a}")), List::cons(f8(Str::lit("    chunks, at = [], 12\u{a}")), List::cons(f8(Str::lit("    while at + 8 <= len(data):\u{a}")), List::cons(f8(Str::lit("        name, size = data[at:at + 4], struct.unpack('<I', data[at + 4:at + 8])[0]\u{a}")), List::cons(f8(Str::lit("        chunks.append((name, data[at + 8:at + 8 + size]))\u{a}")), List::cons(f8(Str::lit("        at += 8 + size + (size & 1)\u{a}")), List::cons(f8(Str::lit("    if [name for name, body in chunks] != [b'bext', b'fmt ', b'data']:\u{a}")), List::cons(f8(Str::lit("        return 'has chunks ' + repr([name for name, body in chunks]) + ', not bext, fmt and data'\u{a}")), List::cons(f8(Str::lit("    if len(chunks[0][1]) != 646:\u{a}")), List::cons(f8(Str::lit("        return 'has a bext chunk of ' + str(len(chunks[0][1])) + ' bytes, not 646'\u{a}")), List::cons(f8(Str::lit("    if chunks[1][1] != FMT:\u{a}")), List::cons(f8(Str::lit("        return 'is not PCM, mono, 48000 Hz, 16-bit'\u{a}")), List::cons(f8(Str::lit("    if not chunks[2][1]:\u{a}")), List::cons(f8(Str::lit("        return 'has no samples'\u{a}")), List::cons(f8(Str::lit("    samples = [value * 2.0 ** -15 for value in struct.unpack('<' + str(len(chunks[2][1]) >> 1) + 'h', chunks[2][1])]\u{a}")), List::cons(f8(Str::lit("    measured, peak = loudness(samples), true_peak(samples)\u{a}")), List::cons(f8(Str::lit("    if abs(measured - TARGET) > TOLERANCE:\u{a}")), List::cons(f8(Str::lit("        return 'reads ' + format(measured, '.1f') + ' LKFS, outside the -24 (+-2) LKFS home-platform target of ASWG-R001'\u{a}")), List::cons(f8(Str::lit("    if peak > TRUE_PEAK:\u{a}")), List::cons(f8(Str::lit("        return 'peaks at ' + format(peak, '.2f') + ' dBTP, above the -1 dBTP of ASWG-R001'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def svg(data):\u{a}")), List::cons(f8(Str::lit("    try:\u{a}")), List::cons(f8(Str::lit("        root = tree.fromstring(data)\u{a}")), List::cons(f8(Str::lit("    except tree.ParseError as failure:\u{a}")), List::cons(f8(Str::lit("        return 'is not well-formed XML: ' + str(failure)\u{a}")), List::cons(f8(Str::lit("    if root.tag != SVG_ROOT:\u{a}")), List::cons(f8(Str::lit("        return 'has the root ' + root.tag + ', not svg in the SVG namespace'\u{a}")), List::cons(f8(Str::lit("    # Coverage (Der from REQ-PRISM-GENERATION): the artwork's text carries every feature of the Prism.\u{a}")), List::cons(f8(Str::lit("    words = ' '.join(root.itertext()).split()\u{a}")), List::cons(f8(Str::lit("    missing = [feature for feature in FEATURES if feature not in words]\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f266() -> Bytes {
    f7(List::cons(f8(Str::lit("    if missing:\u{a}")), List::cons(f8(Str::lit("        return 'leaves out the features ' + repr(missing)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def bundle(data):\u{a}")), List::cons(f8(Str::lit("    if data[:4] != b'PK' + bytes([3, 4]) or b'bagit.txt' not in data:\u{a}")), List::cons(f8(Str::lit("        return 'is no ZIP holding a BagIt bag'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("RULES = {'audio/midi': midi, 'audio/wave': wave, 'image/svg+xml': svg, 'application/zip': bundle}\u{a}")), List::cons(f8(Str::lit("FEATURES = [line for line in run('prism' + chr(9) + 'features' + chr(9) + SEED).decode().split(chr(10)) if line]\u{a}")), List::cons(f8(Str::lit("facets = [line for line in run('prism' + chr(9) + chr(9) + SEED).decode().split(chr(10)) if line]\u{a}")), List::cons(f8(Str::lit("if not facets:\u{a}")), List::cons(f8(Str::lit("    raise SystemExit('::error::the Prism of ' + SEED + ' lists no facet')\u{a}")), List::cons(f8(Str::lit("failed = False\u{a}")), List::cons(f8(Str::lit("for facet in facets:\u{a}")), List::cons(f8(Str::lit("    data = run('prism' + chr(9) + facet + chr(9) + SEED)\u{a}")), List::cons(f8(Str::lit("    rule = RULES.get(facet)\u{a}")), List::cons(f8(Str::lit("    problem = 'has no acceptance rule, so its fidelity is unchecked' if rule is None else rule(data)\u{a}")), List::cons(f8(Str::lit("    if problem:\u{a}")), List::cons(f8(Str::lit("        failed = True\u{a}")), List::cons(f8(Str::lit("        print('::error::the ' + facet + ' facet of ' + SEED + ' ' + problem)\u{a}")), List::cons(f8(Str::lit("    else:\u{a}")), List::cons(f8(Str::lit("        print('accepted: the ' + facet + ' facet of ' + SEED + ', ' + str(len(data)) + ' bytes')\u{a}")), List::cons(f8(Str::lit("sys.exit(1 if failed else 0)\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))
}

pub fn f267(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1098 = v0;
    match m1098.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f268(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt19 = f377(v0.clone(), 0u64, {
        let a1099 = v0;
        length_bytes(a1099)
    }, Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt19 = {
        let a1102 = {
            let a1101 = match v1.clone() {
                Adt19::C0(_, _, h1100, _, _) => {
                    h1100
                }
            };
            length_bytes(a1101)
        };
        let a1103 = 0u64;
        if if nat_eq(a1102, a1103) {
            let m1105 = match v1.clone() {
                Adt19::C0(_, _, _, h1104, _) => {
                    h1104
                }
            };
            match m1105.uncons() {
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
            f378(v1)
        }
    };
    Ok(f379(match v4 {
        Adt19::C0(_, _, _, _, h1106) => {
            h1106
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f269(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt17>, v3: List<Adt2>, v4: List<Bytes>) -> List<Bytes> {
    let m1107 = v1;
    match m1107.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f125(f380(v5.clone(), v2.clone()), f381(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f382(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1109 = {
                    let a1108 = v7.clone();
                    length_list(a1108)
                };
                let a1110 = 0u64;
                if nat_eq(a1109, a1110) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f122(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f269(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f270(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1111 = v0;
    match m1111.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1112 = v1;
            match m1112.uncons() {
                None => {
                    f270(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f270(v2))
                }
            }
        }
    }
}

pub fn f271(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1113 = v1;
    match m1113.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f224(v2.clone(), f8(Str::lit("domain"))) {
                f383(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1114 = f93(v2.clone(), f8(Str::lit("role")));
                    match m1114 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f384(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1115 = f93(v2.clone(), f8(Str::lit("origin")));
                    match m1115 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1116 = f93(v2, f8(Str::lit("evidence")));
                    match m1116 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f271(v0, v3))
            } else {
                f271(v0, v3)
            }
        }
    }
}

pub fn f272(v0: List<Bytes>, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1117 = v1;
    match m1117.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            if f224(v2.clone(), f8(Str::lit("subject"))) {
                if f52(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                    List::cons(({
                        let a1118 = f8(Str::lit("a:"));
                        let a1119 = f58(v2.clone(), f8(Str::lit("object")));
                        append_bytes(a1118, a1119)
                    }, f46(f58(v2, f8(Str::lit("subject"))), 0u64, 1u64)), f272(v0, v3))
                } else {
                    f272(v0, v3)
                }
            } else {
                let v5: Bytes = {
                    let m1120 = f93(v2.clone(), f8(Str::lit("role")));
                    match m1120 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                };
                if f117(f52(f58(v2.clone(), f8(Str::lit("row"))), v0.clone()), f51({
                    let a1122 = {
                        let a1121 = v5.clone();
                        length_bytes(a1121)
                    };
                    let a1123 = 0u64;
                    nat_eq(a1122, a1123)
                })) {
                    List::cons(({
                        let a1124 = f8(Str::lit("r:"));
                        let a1125 = v5;
                        append_bytes(a1124, a1125)
                    }, f46(f58(v2, f8(Str::lit("row"))), 0u64, 1u64)), f272(v0, v3))
                } else {
                    f272(v0, v3)
                }
            }
        }
    }
}

pub fn f273(v0: List<List<Bytes>>, v1: u64, v2: u64) -> R<List<Bytes>> {
    let m1126 = v2;
    if m1126 == 0 {
        Ok(List::<Bytes>::nil())
    } else {
        let v3 = m1126 - 1;
        Ok(f125(f385(v0.clone(), f46(f386(), v1, 1u64), f387()?)?, f273(v0, {
            let a1127 = v1;
            let a1128 = 1u64;
            nat_add(a1127, a1128)?
        }, v3)?))
    }
}

pub fn f274(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1129 = v0;
    match m1129.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f388({
                let (h1130, _) = v2;
                h1130
            }, {
                let (_, h1131) = v2;
                h1131
            }, v1.clone())?, f274(v3, v1)?))
        }
    }
}

pub fn f275(v0: Bytes, p1149: u64, p1150: u64, p1151: Bytes) -> R<Bytes> {
    let m1148 = (p1149, (p1150, p1151));
    {
        let mut m1148 = m1148;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1148;
                let m1132 = v2;
                if m1132 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1132 - 1;
                    let v6: u64 = {
                        let m1135 = {
                            let a1133 = v0.clone();
                            let a1134 = v1;
                            index_bytes(a1133, a1134)
                        };
                        match m1135 {
                            None => {
                                0u64
                            }
                            Some(v5) => {
                                f72(v5)?
                            }
                        }
                    };
                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                        let a1136 = v1;
                        let a1137 = 1u64;
                        nat_add(a1136, a1137)?
                    }, (v4, {
                        let a1146 = {
                            let a1141 = v3;
                            let a1142 = f46(f8(Str::lit("0123456789abcdef")), {
                                let a1138 = v6;
                                let a1139 = 16u64;
                                let a1140 = 0u64;
                                nat_quot(a1138, a1139, a1140)
                            }, 1u64);
                            append_bytes(a1141, a1142)
                        };
                        let a1147 = f46(f8(Str::lit("0123456789abcdef")), {
                            let a1143 = v6;
                            let a1144 = 16u64;
                            let a1145 = 0u64;
                            nat_rem(a1143, a1144, a1145)
                        }, 1u64);
                        append_bytes(a1146, a1147)
                    })))
                }
            };
            match step {
                Ok(next) => { m1148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: List<Adt2>) -> List<Bytes> {
    let m1152 = v0;
    match m1152.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f276(v2))
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
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f389(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f277(v2))
        }
    }
}

pub fn f278(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1154 = v0;
    match m1154.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f390(v1)?, f278(v2)?))
        }
    }
}

pub fn f279(p1160: List<Bytes>) -> R<bool> {
    let m1159 = p1160;
    {
        let mut m1159 = m1159;
        loop {
            let step = {
                let v0 = m1159;
                let m1155 = v0;
                match m1155.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1157 = {
                            let a1156 = f201(v1)?;
                            length_bytes(a1156)
                        };
                        let a1158 = 0u64;
                        if nat_eq(a1157, a1158) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(v0: List<Bytes>) -> R<bool> {
    if f289(f302(v0.clone())?) {
        Ok(f51({
            let a1162 = {
                let a1161 = f291(f302(v0)?);
                length_list(a1161)
            };
            let a1163 = 0u64;
            nat_eq(a1162, a1163)
        }))
    } else {
        Ok(false)
    }
}

pub fn f281(v0: List<Bytes>) -> R<bool> {
    if f289(f391(v0.clone(), f8(Str::lit("requirement")))?) {
        if f289(f391(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1165 = {
                    let a1164 = f227(f291(f391(v0.clone(), f8(Str::lit("requirement")))?), f291(f391(v0, f8(Str::lit("admission")))?));
                    length_list(a1164)
                };
                let a1166 = 0u64;
                nat_eq(a1165, a1166)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f282() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f283(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f284() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f285(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1167 = v0;
    match m1167.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f201(v1)?;
            let a1169 = {
                let a1168 = v3.clone();
                length_bytes(a1168)
            };
            let a1170 = 0u64;
            if nat_eq(a1169, a1170) {
                f285(v2)
            } else {
                let m1171 = f43(v3.clone())?;
                match m1171 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f392(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1172 = f285(v2)?;
                            match m1172 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f393(v5), {
                                        let a1173 = v3;
                                        let a1174 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1173, a1174)
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

pub fn f286() -> Bytes {
    let a1175 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1175)
}

pub fn f287(p1180: List<Bytes>) -> R<Option<Bytes>> {
    let m1179 = p1180;
    {
        let mut m1179 = m1179;
        loop {
            let step = {
                let v0 = m1179;
                let m1176 = v0;
                match m1176.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1177 = f43(f201(v1)?)?;
                        match m1177 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1178 = f93(v3.clone(), f8(Str::lit("source")));
                                match m1178 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f394(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m1179 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: Bytes) -> Bytes {
    let a1185 = {
        let a1181 = Str::lit("binding ");
        utf8_encode(a1181)
    };
    let a1186 = {
        let a1183 = v0;
        let a1184 = {
            let a1182 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1182)
        };
        append_bytes(a1183, a1184)
    };
    append_bytes(a1185, a1186)
}

pub fn f289(v0: Option<List<Adt2>>) -> bool {
    let m1187 = v0;
    match m1187 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f290(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1188 = v0;
    match m1188.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f201(v1)?;
            let a1190 = {
                let a1189 = v3.clone();
                length_bytes(a1189)
            };
            let a1191 = 0u64;
            if nat_eq(a1190, a1191) {
                f290(v2)
            } else {
                let m1192 = f43(v3)?;
                match m1192 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if f117(f117(f224(v4.clone(), f8(Str::lit("authority"))), f224(v4.clone(), f8(Str::lit("row")))), f117(f117(f224(v4.clone(), f8(Str::lit("construct"))), f224(v4.clone(), f8(Str::lit("artifact")))), f395(f58(v4.clone(), f8(Str::lit("reader"))), f58(v4.clone(), f8(Str::lit("construct")))))) {
                            let m1193 = f290(v2)?;
                            match m1193 {
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

pub fn f291(v0: Option<List<Adt2>>) -> List<Adt2> {
    let m1194 = v0;
    match m1194 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f292(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1195 = v0;
    match m1195.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f201(v1)?;
            let a1197 = {
                let a1196 = v3.clone();
                length_bytes(a1196)
            };
            let a1198 = 0u64;
            if nat_eq(a1197, a1198) {
                f292(v2)
            } else {
                let m1199 = f43(v3.clone())?;
                match m1199 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f396(v4.clone()) {
                            let m1200 = f292(v2)?;
                            match m1200 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f249(), List::cons(f8(Str::lit("/")), List::cons(f397(f58(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1201 = v3;
                                        let a1202 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1201, a1202)
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

pub fn f293() -> Bytes {
    f398()
}

pub fn f294(v0: Adt1, v1: Adt2, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f299(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f399(v1, v4)?)?), List::cons(f300(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f295(v0: Bytes) -> R<Bytes> {
    Ok(f400(f401(v0)?))
}

pub fn f296(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1203 = v0;
    match m1203.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f297() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f298(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f364(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f299(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f402(f150(f95(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f403(f404(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f300(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f100(Adt2::C7(std::rc::Rc::new(f405(f145(f125(v1, f218(f96(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f301(p1206: List<Adt2>) -> bool {
    let m1205 = p1206;
    {
        let mut m1205 = m1205;
        loop {
            let step = {
                let v0 = m1205;
                let m1204 = v0;
                match m1204.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f142(f406(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1205 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1207 = v0;
    match m1207.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f201(v1)?;
            let a1209 = {
                let a1208 = v3.clone();
                length_bytes(a1208)
            };
            let a1210 = 0u64;
            if nat_eq(a1209, a1210) {
                f302(v2)
            } else {
                let m1211 = f43(v3)?;
                match m1211 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if if f224(v4.clone(), f8(Str::lit("agent"))) {
                            if f224(v4.clone(), f8(Str::lit("at"))) {
                                if f224(v4.clone(), f8(Str::lit("quote"))) {
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
                            let m1212 = f302(v2)?;
                            match m1212 {
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

pub fn f303(v0: Bytes, v1: List<Adt2>) -> R<(Bytes, Bytes)> {
    Ok((v0, f100(Adt2::C7(std::rc::Rc::new(f101(v1))))?))
}

pub fn f304() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f305(p1215: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m1214 = p1215;
    {
        let mut m1214 = m1214;
        loop {
            let step = {
                let v0 = m1214;
                let m1213 = v0;
                match m1213.uncons() {
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
                Ok(next) => { m1214 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt2 {
    Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt2::C6(v2)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt2::C6(v0)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9))))))))))))
}

pub fn f307(v0: u64) -> bool {
    let a1216 = v0;
    let a1217 = 127u64;
    nat_le(a1216, a1217)
}

pub fn f308(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1218, _) = v1.clone();
        h1218
    };
    let v3: u64 = {
        let (h1219, _) = {
            let (_, h1220) = v1.clone();
            h1220
        };
        h1219
    };
    let v4: List<Bytes> = {
        let (_, h1221) = {
            let (_, h1222) = v1;
            h1222
        };
        h1221
    };
    let m1223 = f211(v0.clone(), v2);
    match m1223 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1224 = v5;
            let a1225 = 10u8;
            if equal(a1224, a1225) {
                Ok(Some(({
                    let a1226 = v2;
                    let a1227 = 1u64;
                    nat_add(a1226, a1227)?
                }, ({
                    let a1228 = v2;
                    let a1229 = 1u64;
                    nat_add(a1228, a1229)?
                }, List::cons(f408(f46(v0, v3, {
                    let a1230 = v2;
                    let a1231 = v3;
                    nat_sub(a1230, a1231)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1232 = v2;
                    let a1233 = 1u64;
                    nat_add(a1232, a1233)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f309(v0: Adt2) -> List<Adt2> {
    let m1234 = v0;
    match m1234 {
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
        Adt2::C7(r1235) => {
            let v3 = (*r1235).clone();
            f309(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r1236, r1237) => {
            let v1 = (*r1236).clone();
            let v2 = (*r1237).clone();
            List::cons(v1, f309(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f310(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1238 = v0;
    match m1238 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r1239) => {
            let v12 = (*r1239).clone();
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
        Adt2::C7(r1240) => {
            let v10 = (*r1240).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r1241) => {
            let v5 = (*r1241).clone();
            Adt2::C8(std::rc::Rc::new(f310(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1242, r1243) => {
            let v18 = (*r1242).clone();
            let v19 = (*r1243).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r1244, r1245) => {
            let v3 = (*r1244).clone();
            let v4 = (*r1245).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt2::C11(v2, std::rc::Rc::new(f409(v3, v1.clone())), std::rc::Rc::new(f310(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f310(v4, v1)))
            }
        }
    }
}

pub fn f311(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1246 = 0u64;
        let a1247 = v1;
        nat_lt(a1246, a1247)
    } else {
        false
    } {
        let a1248 = Bytes::lit(&[0x2d]);
        let a1249 = f410(v1)?;
        Ok(append_bytes(a1248, a1249))
    } else {
        f410(v1)
    }
}

pub fn f312(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f410(v1)?;
    let v4: u64 = {
        let a1250 = v3.clone();
        length_bytes(a1250)
    };
    let v5: Bytes = {
        let a1251 = v2;
        let a1252 = v4;
        if nat_lt(a1251, a1252) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1253 = v4;
                let a1254 = v2;
                nat_sub(a1253, a1254)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1255 = v4;
                let a1256 = v2;
                nat_sub(a1255, a1256)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f333({
                let a1257 = v2;
                let a1258 = v4;
                nat_sub(a1257, a1258)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1259 = f8(Str::lit("-"));
        let a1260 = v5;
        Ok(append_bytes(a1259, a1260))
    } else {
        Ok(v5)
    }
}

pub fn f313(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f411(v0)?;
    let v3: bool = {
        let a1261 = f412(v0)?;
        let a1262 = v1;
        nat_le(a1261, a1262)
    };
    let v4: u64 = {
        let a1266 = {
            let a1263 = v1;
            let a1264 = v2;
            let a1265 = 0u64;
            nat_quot(a1263, a1264, a1265)
        };
        let a1267 = f413(f414(v0))?;
        let a1268 = 0u64;
        nat_rem(a1266, a1267, a1268)
    };
    let v5: u64 = {
        let a1269 = v1;
        let a1270 = v2;
        let a1271 = 0u64;
        nat_rem(a1269, a1270, a1271)
    };
    let a1272 = v4;
    let a1273 = f415(v0)?;
    if nat_eq(a1272, a1273) {
        let a1274 = v5;
        let a1275 = 0u64;
        if nat_eq(a1274, a1275) {
            Ok(f416(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f416(v3, {
                let a1279 = {
                    let a1276 = v2;
                    let a1277 = 2u64;
                    let a1278 = 0u64;
                    nat_quot(a1276, a1277, a1278)
                };
                let a1280 = v5;
                if nat_le(a1279, a1280) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f117({
            let a1281 = v4;
            let a1282 = 0u64;
            nat_eq(a1281, a1282)
        }, {
            let a1283 = v5;
            let a1284 = 0u64;
            nat_eq(a1283, a1284)
        }) {
            Ok(f416(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1285 = v4;
                let a1286 = 0u64;
                if nat_eq(a1285, a1286) {
                    v5
                } else {
                    let a1287 = v2;
                    let a1288 = v5;
                    nat_add(a1287, a1288)?
                }
            };
            let v7: u64 = {
                let a1289 = v4;
                let a1290 = 0u64;
                if nat_eq(a1289, a1290) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1297 = {
                    let a1293 = {
                        let a1291 = v7;
                        let a1292 = 2000u64;
                        nat_add(a1291, a1292)?
                    };
                    let a1294 = f417(v0);
                    nat_sub(a1293, a1294)
                };
                let a1298 = {
                    let a1295 = f418(v0);
                    let a1296 = 1u64;
                    nat_sub(a1295, a1296)
                };
                nat_sub(a1297, a1298)
            };
            Ok(f416(v3, {
                let a1299 = 2000u64;
                let a1300 = v8;
                if nat_le(a1299, a1300) {
                    f419(v6, true, {
                        let a1301 = v8;
                        let a1302 = 2000u64;
                        nat_sub(a1301, a1302)
                    })?
                } else {
                    f419(v6, false, {
                        let a1303 = 2000u64;
                        let a1304 = v8;
                        nat_sub(a1303, a1304)
                    })?
                }
            }))
        }
    }
}

pub fn f314(v0: Bytes) -> R<Bytes> {
    let a1309 = f203(v0.clone(), 0u64, {
        let a1306 = {
            let a1305 = v0.clone();
            length_bytes(a1305)
        };
        let a1307 = 1u64;
        nat_add(a1306, a1307)?
    })?;
    let a1310 = {
        let a1308 = v0.clone();
        length_bytes(a1308)
    };
    if nat_eq(a1309, a1310) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1316 = {
            let a1314 = Bytes::lit(&[0x22]);
            let a1315 = f420(v0.clone(), 0u64, {
                let a1312 = {
                    let a1311 = v0;
                    length_bytes(a1311)
                };
                let a1313 = 1u64;
                nat_add(a1312, a1313)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1314, a1315)
        };
        let a1317 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1316, a1317))
    }
}

pub fn f315(v0: List<Bytes>) -> Bytes {
    let a1320 = {
        let a1318 = Bytes::lit(&[0x5b]);
        let a1319 = f122(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1318, a1319)
    };
    let a1321 = Bytes::lit(&[0x5d]);
    append_bytes(a1320, a1321)
}

pub fn f316(v0: Adt2) -> R<List<Bytes>> {
    let m1322 = v0;
    match m1322 {
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
        Adt2::C10(r1323, r1324) => {
            let v14 = (*r1323).clone();
            let v15 = (*r1324).clone();
            Ok(List::cons(f193(v14)?, f316(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f317(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f421(Fn5::F422, List::<(Bytes, Bytes)>::nil(), f423(v0)?);
    let v2: List<Bytes> = f424(Fn6::F425, List::<Bytes>::nil(), v1)?;
    let a1327 = {
        let a1325 = Bytes::lit(&[0x7b]);
        let a1326 = f122(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1325, a1326)
    };
    let a1328 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1327, a1328))
}

pub fn f318(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1329 = v0;
    match m1329 {
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
        Adt2::C11(v16, r1330, r1331) => {
            let v17 = (*r1330).clone();
            let v18 = (*r1331).clone();
            Ok(List::cons((v16, f193(v17)?), f318(v18)?))
        }
    }
}

pub fn f319() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f320(v0: Adt13) -> Bytes {
    let m1332 = v0;
    match m1332 {
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

pub fn f321() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f322(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f426(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f426(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f426(f8(Str::lit("name")), f427(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f426(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f426(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f426(f8(Str::lit("statement")), f428(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f426(f8(Str::lit("media_type")), f429(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f426(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f426(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f426(f8(Str::lit("url")), f196(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f323(v0: Bytes) -> R<u64> {
    let (h1333, _) = f326(Fn3::F430(v0.clone()), {
        let a1334 = v0.clone();
        length_bytes(a1334)
    }, {
        let a1335 = v0;
        length_bytes(a1335)
    })?;
    Ok(h1333)
}

pub fn f324(v0: Bytes, v1: u64) -> R<u64> {
    let (h1336, _) = f326(Fn3::F431(v1, v0), v1, 0u64)?;
    Ok(h1336)
}

pub fn f325(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f115(v0.clone(), {
        let a1337 = v1;
        let a1338 = 1u64;
        nat_add(a1337, a1338)?
    })?;
    let v4: Bytes = {
        let a1339 = v3;
        let a1340 = 117u64;
        if nat_eq(a1339, a1340) {
            f432(v0, v1)?
        } else {
            f433(v3)
        }
    };
    let a1341 = v4.clone();
    let a1342 = Bytes::lit(&[]);
    if equal(a1341, a1342) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1343 = v3;
            let a1344 = 117u64;
            if nat_eq(a1343, a1344) {
                let a1345 = v1;
                let a1346 = 6u64;
                nat_add(a1345, a1346)?
            } else {
                let a1347 = v1;
                let a1348 = 2u64;
                nat_add(a1347, a1348)?
            }
        }, ({
            let a1349 = v2;
            let a1350 = v4;
            append_bytes(a1349, a1350)
        }, 0u64))))
    }
}

pub fn f326(v0: Fn3, p1355: u64, p1356: u64) -> R<(u64, bool)> {
    let m1354 = (p1355, p1356);
    {
        let mut m1354 = m1354;
        loop {
            let step = {
                let (v1, v2) = m1354;
                let m1351 = v1;
                if m1351 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1351 - 1;
                    let m1353 = {
                        let c1352: Fn3 = v0.clone();
                        c1352.apply(v2)?
                    };
                    match m1353 {
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
                Ok(next) => { m1354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f434(v0, v1)
}

pub fn f328(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f435(v0, v1)
}

pub fn f329(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1357 = v0.clone();
        length_bytes(a1357)
    };
    let a1358 = 0u64;
    let a1359 = v1;
    if nat_lt(a1358, a1359) {
        let a1360 = f436(v0.clone())?;
        let a1361 = v1;
        if nat_eq(a1360, a1361) {
            let a1362 = 1u64;
            let a1363 = v1;
            if nat_lt(a1362, a1363) {
                let m1366 = {
                    let a1364 = v0;
                    let a1365 = 0u64;
                    index_bytes(a1364, a1365)
                };
                match m1366 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1367 = v2;
                        let a1368 = 48u8;
                        Ok(!equal(a1367, a1368))
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

pub fn f330(v0: Fn4, p1373: u64, p1374: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1372 = (p1373, p1374);
    {
        let mut m1372 = m1372;
        loop {
            let step = {
                let (v1, v2) = m1372;
                let m1369 = v1;
                if m1369 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1369 - 1;
                    let m1371 = {
                        let c1370: Fn4 = v0.clone();
                        c1370.apply(v2)?
                    };
                    match m1371 {
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
                Ok(next) => { m1372 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f437(v0, v1)
}

pub fn f332(v0: Bytes) -> R<u64> {
    let (h1375, _) = f326(Fn3::F438(v0.clone()), {
        let a1377 = {
            let a1376 = v0;
            length_bytes(a1376)
        };
        let a1378 = 1u64;
        nat_add(a1377, a1378)?
    }, 0u64)?;
    Ok(h1375)
}

pub fn f333(v0: u64) -> Bytes {
    let m1379 = v0;
    if m1379 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1379 - 1;
        let a1380 = f8(Str::lit("0"));
        let a1381 = f333(v1);
        append_bytes(a1380, a1381)
    }
}

pub fn f334(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1382 = v0;
    match m1382.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1383 = f93(v1, f8(Str::lit("std")));
            match m1383 {
                None => {
                    f334(v2)
                }
                Some(v3) => {
                    if f224(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f334(v2))
                    } else {
                        f334(v2)
                    }
                }
            }
        }
    }
}

pub fn f335(v0: Adt14, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1384 = v1;
    match m1384.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt14::C0(h1385, _, _) => {
                    h1385
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h1386, _) => {
                        h1386
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f335(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt14::C0(h1387, _, _) => {
                        h1387
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1388, _) => {
                            h1388
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f335(v0, v3))
                } else {
                    f335(v0, v3)
                }
            }
        }
    }
}

pub fn f336(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt17>> {
    let m1389 = v0;
    match m1389.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v4, v5)) => {
            if if f439(v4.clone(), v2.clone()) {
                true
            } else {
                f439(v4.clone(), v3.clone())
            } {
                f336(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt17::C0({
                    let (h1390, _) = v4.clone();
                    h1390
                }, {
                    let a1393 = {
                        let a1392 = f145(f440({
                            let (h1391, _) = v4.clone();
                            h1391
                        }, v1.clone()))?;
                        length_list(a1392)
                    };
                    let a1394 = 1u64;
                    if nat_eq(a1393, a1394) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1395) = v4.clone();
                    h1395
                }), f336(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f337(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1396 = v1;
    match m1396.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1397, _) = v2.clone();
                h1397
            }, v0.clone()) {
                List::cons({
                    let (_, h1398) = v2;
                    h1398
                }, f337(v0, v3))
            } else {
                if f52({
                    let (_, h1399) = v2;
                    h1399
                }, v0.clone()) {
                    List::cons({
                        let (h1400, _) = v2;
                        h1400
                    }, f337(v0, v3))
                } else {
                    f337(v0, v3)
                }
            }
        }
    }
}

pub fn f338(v0: Adt14, v1: Adt2, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt14::C0(h1401, _, _) => {
            h1401
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1402, _) => {
                h1402
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt14::C0(_, _, h1403) => {
                    h1403
                }
            }) {
                true
            } else {
                if f224(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1404, _) => {
                            h1404
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f339(v0: Adt2) -> bool {
    if f224(v0.clone(), f8(Str::lit("domain"))) {
        if f224(v0.clone(), f8(Str::lit("row"))) {
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

pub fn f340(v0: Adt2) -> bool {
    if f224(v0.clone(), f8(Str::lit("subject"))) {
        if f224(v0.clone(), f8(Str::lit("object"))) {
            if f224(v0.clone(), f8(Str::lit("evidence"))) {
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

pub fn f341(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1405 = f43(v1)?;
    match m1405 {
        None => {
            Ok(f232(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f218(f191(f93(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1406 = f92(f93(v4.clone(), f8(Str::lit("construct"))));
                match m1406 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1407 = v5;
            match m1407.uncons() {
                None => {
                    Ok(f232(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1408 = v9;
                    match m1408.uncons() {
                        None => {
                            Ok(f232(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f51(f32(v8, f8(Str::lit("std")))) {
                                Ok(f232(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f51(f52(v10.clone(), v2)) {
                                    Ok(f232(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f117(f51(f32(v7.clone(), f8(Str::lit("none")))), f51(f52(v7.clone(), v3))) {
                                        Ok(f232(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1409 = f92(f93(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1409 {
                                            None => {
                                                Ok(f232(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1410 = f93(v4.clone(), f8(Str::lit("reader")));
                                                match m1410 {
                                                    None => {
                                                        let m1411 = f93(v4, f8(Str::lit("value")));
                                                        match m1411 {
                                                            None => {
                                                                Ok(f232(v0, f8(Str::lit("a row names neither the reader of its artifact nor a value"))))
                                                            }
                                                            Some(_) => {
                                                                Ok(List::<Bytes>::nil())
                                                            }
                                                        }
                                                    }
                                                    Some(_) => {
                                                        if f224(v4, f8(Str::lit("artifact"))) {
                                                            Ok(List::<Bytes>::nil())
                                                        } else {
                                                            Ok(f232(v0, f8(Str::lit("a row names its reader but not its artifact"))))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f342(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1412 = v1;
    match m1412.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f444(v3.clone())
            } else {
                f51(f444(v3.clone()))
            } {
                List::cons(f245(v0.clone(), v3), f342(v0, v4, v2))
            } else {
                f342(v0, v4, v2)
            }
        }
    }
}

pub fn f343(p1415: List<Bytes>) -> R<Option<Bytes>> {
    let m1414 = p1415;
    {
        let mut m1414 = m1414;
        loop {
            let step = {
                let v0 = m1414;
                let m1413 = v0;
                match m1413.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f445(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f201(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1416 = v1;
    match m1416.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f95(f64(v0.clone(), v2))?, f344(v0, v3)?))
        }
    }
}

pub fn f345(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1417 = v0.clone();
        length_bytes(a1417)
    };
    let v3: u64 = {
        let a1418 = v1.clone();
        length_bytes(a1418)
    };
    let v4: u64 = {
        let (h1419, _) = f326(Fn3::F446(v1.clone(), v3, v0.clone(), v2), {
            let a1420 = v2;
            let a1421 = 1u64;
            nat_add(a1420, a1421)?
        }, 0u64)?;
        h1419
    };
    if f447(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f346() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f347(v0: Adt15) -> R<Adt18> {
    Ok(Adt18::C0(f448(match v0.clone() {
        Adt15::C0(h1422, _, _, _, _) => {
            h1422
        }
    }, 0u64)?, f448(match v0.clone() {
        Adt15::C0(_, h1423, _, _, _) => {
            h1423
        }
    }, 0u64)?, f448(match v0.clone() {
        Adt15::C0(_, _, h1424, _, _) => {
            h1424
        }
    }, 0u64)?, f448(match v0 {
        Adt15::C0(_, _, _, h1425, _) => {
            h1425
        }
    }, 0u64)?))
}

pub fn f348(v0: Adt15) -> List<Bytes> {
    f125(match v0.clone() {
        Adt15::C0(h1426, _, _, _, _) => {
            h1426
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, h1427, _, _) => {
            h1427
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, _, h1428, _) => {
            h1428
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1429) => {
            h1429
        }
    })))
}

pub fn f349(v0: Adt18, p1435: List<u64>, p1436: List<Adt20>) -> Option<u64> {
    let m1434 = (p1435, p1436);
    {
        let mut m1434 = m1434;
        loop {
            let step = {
                let (v1, v2) = m1434;
                let m1430 = v2;
                match m1430.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f449(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1431) => {
                                h1431
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1432, _) => {
                                    h1432
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1433, _) => {
                                    h1433
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Adt15, v1: List<Adt2>, v2: u64) -> R<List<Adt20>> {
    let m1437 = v1;
    match m1437.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f450(v0.clone(), v3)?), f350(v0, v4, {
                let a1438 = v2;
                let a1439 = 1u64;
                nat_add(a1438, a1439)?
            })?))
        }
    }
}

pub fn f351(p1443: List<Bytes>, p1444: u64) -> Bytes {
    let m1442 = (p1443, p1444);
    {
        let mut m1442 = m1442;
        loop {
            let step = {
                let (v0, v1) = m1442;
                let m1440 = v0;
                match m1440.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1441 = v1;
                        if m1441 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1441 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
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

pub fn f352(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
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

pub fn f353(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1445 = v0;
    match m1445.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f453(v2, v1.clone())?, f353(v3, v1)?))
        }
    }
}

pub fn f354(v0: List<Adt2>) -> List<u64> {
    let m1446 = v0;
    match m1446.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f454(v1), f354(v2))
        }
    }
}

pub fn f355(p1449: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1448 = p1449;
    {
        let mut m1448 = m1448;
        loop {
            let step = {
                let v0 = m1448;
                let m1447 = v0;
                match m1447.uncons() {
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
                Ok(next) => { m1448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f356(p1452: List<Bytes>, p1453: List<Bytes>) -> Option<Bytes> {
    let m1451 = (p1452, p1453);
    {
        let mut m1451 = m1451;
        loop {
            let step = {
                let (v0, v1) = m1451;
                let m1450 = v0;
                match m1450.uncons() {
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
                Ok(next) => { m1451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(v0: Adt15, p1461: List<Adt2>) -> Option<Bytes> {
    let m1460 = p1461;
    {
        let mut m1460 = m1460;
        loop {
            let step = {
                let v1 = m1460;
                let m1454 = v1;
                match m1454.uncons() {
                    None => {
                        Err::<List<Adt2>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1455 = f93(v2.clone(), f8(Str::lit("evidence")));
                        match m1455 {
                            None => {
                                Ok::<List<Adt2>, Option<Bytes>>(v3)
                            }
                            Some(v4) => {
                                let m1456 = f93(v4, f8(Str::lit("constraint")));
                                match m1456 {
                                    None => {
                                        Ok::<List<Adt2>, Option<Bytes>>(v3)
                                    }
                                    Some(v5) => {
                                        if f52(f58(v5.clone(), f8(Str::lit("authority"))), match v0.clone() {
                                            Adt15::C0(h1457, _, _, _, _) => {
                                                h1457
                                            }
                                        }) {
                                            let m1458 = f92(f93(v5, f8(Str::lit("row"))));
                                            match m1458 {
                                                None => {
                                                    Ok::<List<Adt2>, Option<Bytes>>(v3)
                                                }
                                                Some(v6) => {
                                                    if f52(v6, match v0.clone() {
                                                        Adt15::C0(_, h1459, _, _, _) => {
                                                            h1459
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
                Ok(next) => { m1460 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f359(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1462 = v0.clone();
    match m1462.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1463 = v2;
            let a1464 = v1.clone();
            if equal(a1463, a1464) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f360(v0: List<(Bytes, ())>) -> List<Bytes> {
    f242(f456(Fn7::F457, List::<Bytes>::nil(), v0))
}

pub fn f361(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f458(f459(Fn8::F460, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1469 = {
        let (h1465, _) = f461(Fn9::F462, {
            let a1467 = {
                let a1466 = v0;
                length_list(a1466)
            };
            let a1468 = 1u64;
            nat_add(a1467, a1468)?
        }, v1)?;
        h1465
    };
    match m1469.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f362(v0: List<Bytes>) -> List<(Bytes, ())> {
    f463(f464(Fn10::F465, List::<(Bytes, ())>::nil(), v0))
}

pub fn f363(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1470 = f44(v1, 34u8)?;
    match m1470.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1471 = v3;
            match m1471.uncons() {
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

pub fn f364(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f467(v0.clone(), 0u64, {
        let a1472 = v0;
        length_bytes(a1472)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f365(v0: Adt2, v1: Bytes) -> Bytes {
    if f32(f468(f93(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f366(v0: Bytes, v1: Bytes, p1485: u64, p1486: u64) -> R<bool> {
    let m1484 = (p1485, p1486);
    {
        let mut m1484 = m1484;
        loop {
            let step = {
                let (v2, v3) = m1484;
                let m1473 = v3;
                if m1473 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1473 - 1;
                    let m1476 = {
                        let a1474 = v1.clone();
                        let a1475 = v2;
                        index_bytes(a1474, a1475)
                    };
                    match m1476 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1479 = {
                                let a1477 = v0.clone();
                                let a1478 = v2;
                                index_bytes(a1477, a1478)
                            };
                            match m1479 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1480 = v5;
                                    let a1481 = v6;
                                    if equal(a1480, a1481) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1482 = v2;
                                            let a1483 = 1u64;
                                            nat_add(a1482, a1483)?
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
                Ok(next) => { m1484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f367(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1487 = f91(v0, v1)?;
    match m1487 {
        None => {
            Ok(Adt2::C9)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f368(v0: Bytes) -> R<Bytes> {
    let a1488 = f8(Str::lit("Catalog."));
    let a1489 = f397(v0)?;
    Ok(append_bytes(a1488, a1489))
}

pub fn f369(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f397(f58(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f370(v0: List<Bytes>) -> List<Bytes> {
    let m1490 = v0;
    match m1490.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f245(f282(), v1), f370(v2))
            } else {
                f370(v2)
            }
        }
    }
}

pub fn f371(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1491 = v1;
    match m1491.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f469(f470(f95(f64(v0.clone(), v2))?)?), f371(v0, v3)?))
        }
    }
}

pub fn f372(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1492 = v1;
    match m1492.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f471(v0.clone(), v3, v2.clone())?;
            let (h1493, _) = v5.clone();
            if h1493 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f372(v0, v4, v2)?;
                let (h1494, _) = v6.clone();
                if h1494 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1497 = {
                            let (_, h1495) = v5;
                            h1495
                        };
                        let a1498 = {
                            let (_, h1496) = v6;
                            h1496
                        };
                        append_bytes(a1497, a1498)
                    }))
                }
            }
        }
    }
}

pub fn f373(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1499 = v1;
    match m1499.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f224(v2.clone(), f8(Str::lit("subject"))) {
                if f225(v0.clone(), v2.clone()) {
                    f373(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f373(v0, v3))
                }
            } else {
                if f383(v0.clone(), v2.clone()) {
                    f373(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f384(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f373(v0, v3))
                }
            }
        }
    }
}

pub fn f374(v0: Adt1) -> R<Bytes> {
    let a1501 = {
        let a1500 = f128(f129(f270(f267(f268(f64(v0.clone(), f216()))?)), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f131(f96(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1500)
    };
    let a1502 = 0u64;
    if nat_eq(a1501, a1502) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f375(v0: List<Adt2>) -> R<List<Bytes>> {
    let m1503 = v0;
    match m1503.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f122(f218(f191(f93(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f472(f95(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f375(v2)?))
        }
    }
}

pub fn f376() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f377(v0: Bytes, p1561: u64, p1562: u64, p1563: Adt19) -> R<Adt19> {
    let m1560 = (p1561, (p1562, p1563));
    {
        let mut m1560 = m1560;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1560;
                let m1504 = v2;
                if m1504 == 0 {
                    Err::<(u64, (u64, Adt19)), R<Adt19>>(Ok(v3))
                } else {
                    let v4 = m1504 - 1;
                    let v5: u64 = f473(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt19::C0(_, h1505, _, _, _) => {
                            h1505
                        }
                    } {
                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                            let a1506 = v1;
                            let a1507 = 1u64;
                            nat_add(a1506, a1507)?
                        }, (v4, Adt19::C0(true, false, match v3.clone() {
                            Adt19::C0(_, _, h1508, _, _) => {
                                h1508
                            }
                        }, match v3.clone() {
                            Adt19::C0(_, _, _, h1509, _) => {
                                h1509
                            }
                        }, match v3 {
                            Adt19::C0(_, _, _, _, h1510) => {
                                h1510
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt19::C0(h1511, _, _, _, _) => {
                                h1511
                            }
                        } {
                            let a1512 = v5;
                            let a1513 = 34u64;
                            if nat_eq(a1512, a1513) {
                                let a1516 = f473(v0.clone(), {
                                    let a1514 = v1;
                                    let a1515 = 1u64;
                                    nat_add(a1514, a1515)?
                                })?;
                                let a1517 = 34u64;
                                if nat_eq(a1516, a1517) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1518 = v1;
                                        let a1519 = 1u64;
                                        nat_add(a1518, a1519)?
                                    }, (v4, Adt19::C0(true, true, f474(match v3.clone() {
                                        Adt19::C0(_, _, h1520, _, _) => {
                                            h1520
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt19::C0(_, _, _, h1521, _) => {
                                            h1521
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1522) => {
                                            h1522
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1523 = v1;
                                        let a1524 = 1u64;
                                        nat_add(a1523, a1524)?
                                    }, (v4, Adt19::C0(false, false, match v3.clone() {
                                        Adt19::C0(_, _, h1525, _, _) => {
                                            h1525
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1526, _) => {
                                            h1526
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1527) => {
                                            h1527
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1528 = v1;
                                    let a1529 = 1u64;
                                    nat_add(a1528, a1529)?
                                }, (v4, Adt19::C0(true, false, f474(match v3.clone() {
                                    Adt19::C0(_, _, h1530, _, _) => {
                                        h1530
                                    }
                                }, v0.clone(), v1), match v3.clone() {
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
                            let a1533 = v5;
                            let a1534 = 34u64;
                            if nat_eq(a1533, a1534) {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1535 = v1;
                                    let a1536 = 1u64;
                                    nat_add(a1535, a1536)?
                                }, (v4, Adt19::C0(true, false, match v3.clone() {
                                    Adt19::C0(_, _, h1537, _, _) => {
                                        h1537
                                    }
                                }, match v3.clone() {
                                    Adt19::C0(_, _, _, h1538, _) => {
                                        h1538
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1539) => {
                                        h1539
                                    }
                                }))))
                            } else {
                                let a1540 = v5;
                                let a1541 = 44u64;
                                if nat_eq(a1540, a1541) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1542 = v1;
                                        let a1543 = 1u64;
                                        nat_add(a1542, a1543)?
                                    }, (v4, Adt19::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt19::C0(_, _, h1544, _, _) => {
                                            h1544
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1545, _) => {
                                            h1545
                                        }
                                    }), match v3 {
                                        Adt19::C0(_, _, _, _, h1546) => {
                                            h1546
                                        }
                                    }))))
                                } else {
                                    let a1547 = v5;
                                    let a1548 = 13u64;
                                    if nat_eq(a1547, a1548) {
                                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                            let a1549 = v1;
                                            let a1550 = 1u64;
                                            nat_add(a1549, a1550)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1551 = v5;
                                        let a1552 = 10u64;
                                        if nat_eq(a1551, a1552) {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1553 = v1;
                                                let a1554 = 1u64;
                                                nat_add(a1553, a1554)?
                                            }, (v4, f378(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1555 = v1;
                                                let a1556 = 1u64;
                                                nat_add(a1555, a1556)?
                                            }, (v4, Adt19::C0(false, false, f474(match v3.clone() {
                                                Adt19::C0(_, _, h1557, _, _) => {
                                                    h1557
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt19::C0(_, _, _, h1558, _) => {
                                                    h1558
                                                }
                                            }, match v3 {
                                                Adt19::C0(_, _, _, _, h1559) => {
                                                    h1559
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
                Ok(next) => { m1560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(v0: Adt19) -> Adt19 {
    Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f475(List::cons(match v0.clone() {
        Adt19::C0(_, _, h1564, _, _) => {
            h1564
        }
    }, match v0.clone() {
        Adt19::C0(_, _, _, h1565, _) => {
            h1565
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt19::C0(_, _, _, _, h1566) => {
            h1566
        }
    }))
}

pub fn f379(p1569: List<List<Bytes>>, p1570: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1568 = (p1569, p1570);
    {
        let mut m1568 = m1568;
        loop {
            let step = {
                let (v0, v1) = m1568;
                let m1567 = v0;
                match m1567.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: Bytes, v1: List<Adt17>) -> List<Bytes> {
    let m1571 = v1;
    match m1571.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt17::C0(h1572, _, _) => {
                    h1572
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt17::C0(_, h1573, _) => {
                        h1573
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt17::C0(_, _, h1574) => {
                        h1574
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f380(v0, v3))
            } else {
                f380(v0, v3)
            }
        }
    }
}

pub fn f381(v0: Bytes, v1: List<Adt2>) -> List<Bytes> {
    let m1575 = v1;
    match m1575.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f381(v0, v3))
            } else {
                if f32(f58(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f58(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f381(v0, v3))
                } else {
                    f381(v0, v3)
                }
            }
        }
    }
}

pub fn f382(p1580: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1579 = p1580;
    {
        let mut m1579 = m1579;
        loop {
            let step = {
                let v0 = m1579;
                let m1576 = v0;
                match m1576.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1577 = v2;
                        match m1577.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1578 = v5;
                                    match m1578.uncons() {
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
                Ok(next) => { m1579 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(v0: Adt14, v1: Adt2) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1581, _, _) => {
            h1581
        }
    }) {
        f52(f384(v1), match v0 {
            Adt14::C0(_, h1582, _) => {
                h1582
            }
        })
    } else {
        false
    }
}

pub fn f384(v0: Adt2) -> Bytes {
    let m1583 = f93(v0, f8(Str::lit("evidence")));
    match m1583 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f385(v0: List<List<Bytes>>, v1: Bytes, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1584 = v2;
    match m1584.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            if f477(v0.clone(), v1.clone(), v3.clone())? {
                Ok(List::cons(f7(List::cons(v1.clone(), List::cons(f8(Str::lit("/")), List::cons(v3, List::cons(f8(Str::lit("\u{9}\u{a}")), List::<Bytes>::nil()))))), f385(v0, v1, v4)?))
            } else {
                f385(v0, v1, v4)
            }
        }
    }
}

pub fn f386() -> Bytes {
    f8(Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))
}

pub fn f387() -> R<List<Bytes>> {
    f44(f8(Str::lit("CON PRE PRO QA LCH OPS MNT EOL")), 32u8)
}

pub fn f388(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1585 = v2;
    match m1585.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            if f117(f32({
                let (h1586, _) = v3.clone();
                h1586
            }, v0.clone()), {
                let a1588 = f478(v1.clone())?;
                let a1589 = f478({
                    let (_, h1587) = v3.clone();
                    h1587
                })?;
                nat_lt(a1588, a1589)
            }) {
                Ok(List::cons({
                    let a1591 = v1.clone();
                    let a1592 = {
                        let (_, h1590) = v3;
                        h1590
                    };
                    append_bytes(a1591, a1592)
                }, f388(v0, v1, v4)?))
            } else {
                f388(v0, v1, v4)
            }
        }
    }
}

pub fn f389(v0: Adt2) -> Bytes {
    if f224(v0.clone(), f8(Str::lit("req"))) {
        let a1593 = f8(Str::lit("req "));
        let a1594 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1593, a1594)
    } else {
        let m1595 = f93(v0.clone(), f8(Str::lit("std")));
        match m1595 {
            None => {
                let m1598 = f93(v0, f8(Str::lit("der")));
                match m1598 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f122(f218(f191(f93(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f224(v1.clone(), f8(Str::lit("row"))) {
                    let a1596 = f8(Str::lit(" row "));
                    let a1597 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1596, a1597)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f390(v0: Bytes) -> R<Bytes> {
    let m1599 = f44(v0.clone(), 32u8)?;
    match m1599.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            Ok(f7(List::cons(f8(Str::lit("{\"name\":")), List::cons(f314(v1)?, List::cons(f8(Str::lit(",\"description\":")), List::cons(f314({
                let a1600 = f8(Str::lit("verdant_studio "));
                let a1601 = v0;
                append_bytes(a1600, a1601)
            })?, List::cons(f8(Str::lit(",\"inputSchema\":{\"type\":\"object\",\"properties\":{")), List::cons(f122(f479(v2.clone())?, f8(Str::lit(","))), List::cons(f8(Str::lit("},\"required\":[")), List::cons(f122(f480(v2)?, f8(Str::lit(","))), List::cons(f8(Str::lit("],\"additionalProperties\":false}}")), List::<Bytes>::nil())))))))))))
        }
    }
}

pub fn f391(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt2>>> {
    let m1602 = v0;
    match m1602.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f201(v2)?;
            let a1604 = {
                let a1603 = v4.clone();
                length_bytes(a1603)
            };
            let a1605 = 0u64;
            if nat_eq(a1604, a1605) {
                f391(v3, v1)
            } else {
                let m1606 = f43(v4)?;
                match m1606 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f481(v5.clone());
                        let a1608 = {
                            let a1607 = v6.clone();
                            length_bytes(a1607)
                        };
                        let a1609 = 0u64;
                        if nat_eq(a1608, a1609) {
                            Ok(None::<List<Adt2>>)
                        } else {
                            let m1610 = f391(v3, v1.clone())?;
                            match m1610 {
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

pub fn f392(v0: Bytes, v1: Bytes) -> bool {
    let a1611 = v0;
    let a1612 = f398();
    if equal(a1611, a1612) {
        let a1616 = {
            let a1614 = {
                let a1613 = v1;
                length_bytes(a1613)
            };
            let a1615 = 0u64;
            nat_eq(a1614, a1615)
        };
        bool_not(a1616)
    } else {
        false
    }
}

pub fn f393(v0: Bytes) -> Bytes {
    f7(List::cons(f144(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f394(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1618 = v0;
    let a1619 = {
        let a1617 = Str::lit("redistributable");
        utf8_encode(a1617)
    };
    if equal(a1618, a1619) {
        f482(v1)
    } else {
        Ok(false)
    }
}

pub fn f395(v0: Bytes, v1: Bytes) -> bool {
    f206(f85(v0.clone(), f8(Str::lit("read-"))), f117(f32(v0, f8(Str::lit("check-yaml"))), f32(v1, f8(Str::lit("none")))))
}

pub fn f396(v0: Adt2) -> bool {
    if f483(v0.clone(), f8(Str::lit("name"))) {
        if f484(v0.clone(), f8(Str::lit("schema"))) {
            let m1620 = f485(f218(f191(f93(v0, f8(Str::lit("citation"))))));
            match m1620 {
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

pub fn f397(v0: Bytes) -> R<Bytes> {
    f486(v0.clone(), 0u64, {
        let a1621 = v0;
        length_bytes(a1621)
    }, true, Bytes::lit(&[]))
}

pub fn f398() -> Bytes {
    let a1622 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1622)
}

pub fn f399(v0: Adt2, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt2> = f93(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f298(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f298(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f298(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f298(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f298(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f298(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f298(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f298(Str::lit("url"), f468(v2.clone(), f8(Str::lit("url"))))?, List::cons(f298(Str::lit("media_type"), f468(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f298(Str::lit("license"), f468(v2.clone(), f8(Str::lit("license"))))?, List::cons(f298(Str::lit("redistribution"), f468(v2, f8(Str::lit("redistribution"))))?, List::cons(f298(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1623 = v1;
        match m1623 {
            None => {
                f7(List::cons(f298(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f298(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f298(Str::lit("artifact"), {
                    let (h1624, _) = v3;
                    h1624
                })?, List::cons(f298(Str::lit("checksum"), {
                    let (_, h1625) = v3;
                    h1625
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f487(f218(f191(f93(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f400(v0: Adt23) -> Bytes {
    let a1646 = f488(match v0.clone() {
        Adt23::C0(h1626, _, _, _, _, _, _, _) => {
            h1626
        }
    });
    let a1647 = {
        let a1644 = f488(match v0.clone() {
            Adt23::C0(_, h1627, _, _, _, _, _, _) => {
                h1627
            }
        });
        let a1645 = {
            let a1642 = f488(match v0.clone() {
                Adt23::C0(_, _, h1628, _, _, _, _, _) => {
                    h1628
                }
            });
            let a1643 = {
                let a1640 = f488(match v0.clone() {
                    Adt23::C0(_, _, _, h1629, _, _, _, _) => {
                        h1629
                    }
                });
                let a1641 = {
                    let a1638 = f488(match v0.clone() {
                        Adt23::C0(_, _, _, _, h1630, _, _, _) => {
                            h1630
                        }
                    });
                    let a1639 = {
                        let a1636 = f488(match v0.clone() {
                            Adt23::C0(_, _, _, _, _, h1631, _, _) => {
                                h1631
                            }
                        });
                        let a1637 = {
                            let a1634 = f488(match v0.clone() {
                                Adt23::C0(_, _, _, _, _, _, h1632, _) => {
                                    h1632
                                }
                            });
                            let a1635 = f488(match v0 {
                                Adt23::C0(_, _, _, _, _, _, _, h1633) => {
                                    h1633
                                }
                            });
                            append_bytes(a1634, a1635)
                        };
                        append_bytes(a1636, a1637)
                    };
                    append_bytes(a1638, a1639)
                };
                append_bytes(a1640, a1641)
            };
            append_bytes(a1642, a1643)
        };
        append_bytes(a1644, a1645)
    };
    append_bytes(a1646, a1647)
}

pub fn f401(v0: Bytes) -> R<Adt23> {
    let v1: u64 = {
        let a1648 = v0.clone();
        length_bytes(a1648)
    };
    let v2: u64 = {
        let a1654 = {
            let a1651 = {
                let a1649 = v1;
                let a1650 = 72u64;
                nat_add(a1649, a1650)?
            };
            let a1652 = 64u64;
            let a1653 = 0u64;
            nat_quot(a1651, a1652, a1653)
        };
        let a1655 = 64u64;
        nat_mul(a1654, a1655)?
    };
    let (_, h1656) = f489(Fn11::F490(v0, v1, v2), {
        let a1657 = v2;
        let a1658 = 64u64;
        let a1659 = 0u64;
        nat_quot(a1657, a1658, a1659)
    }, (0u64, f491()))?;
    Ok(h1656)
}

pub fn f402(p1662: List<List<Bytes>>, v1: Bytes, p1663: List<Bytes>) -> R<List<Bytes>> {
    let m1661 = (p1662, p1663);
    {
        let mut m1661 = m1661;
        loop {
            let step = {
                let (v0, v2) = m1661;
                let m1660 = v0;
                match m1660.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f492(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f404(f493(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f323(v0)?))
}

pub fn f404(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1664 = v0;
    match m1664.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1665 = v3.clone();
            match m1665.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1668 = v2;
                    let a1669 = {
                        let a1666 = v1.clone();
                        let a1667 = f404(v3, v1);
                        append_bytes(a1666, a1667)
                    };
                    append_bytes(a1668, a1669)
                }
            }
        }
    }
}

pub fn f405(v0: List<Bytes>) -> Adt2 {
    let m1670 = v0;
    match m1670.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f405(v2)))
        }
    }
}

pub fn f406(v0: Adt2, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1671 = f93(v0, v1);
    match m1671 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v3) => {
            f191(f93(v3, v2))
        }
    }
}

pub fn f407(v0: Adt2, v1: Bytes) -> bool {
    let m1672 = f93(v0, v1);
    match m1672 {
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
            let a1674 = {
                let a1673 = v0;
                length_bytes(a1673)
            };
            let a1675 = 1u64;
            nat_sub(a1674, a1675)
        })
    } else {
        v0
    }
}

pub fn f409(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1676 = v0;
    match m1676 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v8, r1677) => {
            let v9 = (*r1677).clone();
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
            Adt2::C6(f494(f60(), v2, v1))
        }
        Adt2::C7(r1678) => {
            let v6 = (*r1678).clone();
            Adt2::C7(std::rc::Rc::new(v6))
        }
        Adt2::C8(r1679) => {
            let v7 = (*r1679).clone();
            Adt2::C8(std::rc::Rc::new(v7))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1680, r1681) => {
            let v15 = (*r1680).clone();
            let v16 = (*r1681).clone();
            Adt2::C10(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt2::C11(v17, r1682, r1683) => {
            let v18 = (*r1682).clone();
            let v19 = (*r1683).clone();
            Adt2::C11(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
    }
}

pub fn f410(v0: u64) -> R<Bytes> {
    f495(f496(), 10u64, v0)
}

pub fn f411(v0: bool) -> R<u64> {
    f413({
        let a1684 = f418(v0);
        let a1685 = 1u64;
        nat_sub(a1684, a1685)
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
    let m1686 = v0;
    if m1686 == 0 {
        Ok(1u64)
    } else {
        let v1 = m1686 - 1;
        let a1687 = 2u64;
        let a1688 = f413(v1)?;
        nat_mul(a1687, a1688)
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
    let a1689 = f413(f414(v0))?;
    let a1690 = 1u64;
    Ok(nat_sub(a1689, a1690))
}

pub fn f416(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a1691 = Bytes::lit(&[0x2d]);
        let a1692 = v1;
        append_bytes(a1691, a1692)
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
        f497(f498(v2, f499(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f497(f498(v2, f499(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a1693 = v3.clone();
            length_bytes(a1693)
        };
        let v5: Bytes = {
            let a1694 = v2;
            let a1695 = v4;
            if nat_lt(a1694, a1695) {
                f46(v3.clone(), 0u64, {
                    let a1696 = v4;
                    let a1697 = v2;
                    nat_sub(a1696, a1697)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f500({
            let a1698 = v2;
            let a1699 = v4;
            if nat_lt(a1698, a1699) {
                f46(v3, {
                    let a1700 = v4;
                    let a1701 = v2;
                    nat_sub(a1700, a1701)
                }, v2)
            } else {
                let a1704 = f501({
                    let a1702 = v2;
                    let a1703 = v4;
                    nat_sub(a1702, a1703)
                });
                let a1705 = v3;
                append_bytes(a1704, a1705)
            }
        }, {
            let a1706 = v2;
            let a1707 = 1u64;
            nat_add(a1706, a1707)?
        })?;
        let a1709 = {
            let a1708 = v6.clone();
            length_bytes(a1708)
        };
        let a1710 = 0u64;
        if nat_eq(a1709, a1710) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f420(v0: Bytes, p1729: u64, p1730: u64, p1731: Bytes) -> R<Bytes> {
    let m1728 = (p1729, (p1730, p1731));
    {
        let mut m1728 = m1728;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1728;
                let m1711 = v2;
                if m1711 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1711 - 1;
                    let v5: u64 = f203(v0.clone(), v1, {
                        let a1715 = {
                            let a1713 = {
                                let a1712 = v0.clone();
                                length_bytes(a1712)
                            };
                            let a1714 = v1;
                            nat_sub(a1713, a1714)
                        };
                        let a1716 = 1u64;
                        nat_add(a1715, a1716)?
                    })?;
                    let v6: Bytes = {
                        let a1719 = v3;
                        let a1720 = f46(v0.clone(), v1, {
                            let a1717 = v5;
                            let a1718 = v1;
                            nat_sub(a1717, a1718)
                        });
                        append_bytes(a1719, a1720)
                    };
                    let m1723 = {
                        let a1721 = v0.clone();
                        let a1722 = v5;
                        index_bytes(a1721, a1722)
                    };
                    match m1723 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a1724 = v5;
                                let a1725 = 1u64;
                                nat_add(a1724, a1725)?
                            }, (v4, {
                                let a1726 = v6;
                                let a1727 = f502(v0.clone(), v5, v7)?;
                                append_bytes(a1726, a1727)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1728 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421(v0: Fn5, p1735: List<(Bytes, Bytes)>, p1736: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1734 = (p1735, p1736);
    {
        let mut m1734 = m1734;
        loop {
            let step = {
                let (v1, v2) = m1734;
                let m1732 = v2;
                match m1732.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1733: Fn5 = v0.clone();
                            c1733.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f422(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f503(v0, v1)
}

pub fn f423(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f504(v0)
}

pub fn f424(v0: Fn6, p1740: List<Bytes>, p1741: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1739 = (p1740, p1741);
    {
        let mut m1739 = m1739;
        loop {
            let step = {
                let (v1, v2) = m1739;
                let m1737 = v2;
                match m1737.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1738: Fn6 = v0.clone();
                            c1738.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1739 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f505(v1)?, v0))
}

pub fn f426(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f506(v0)?, List::cons(f8(Str::lit(":")), List::cons(f506(v1)?, List::<Bytes>::nil())))))
}

pub fn f427(v0: Adt13) -> Bytes {
    let m1742 = v0;
    match m1742 {
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
    let m1743 = v0;
    match m1743 {
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
    let m1744 = v0;
    match m1744 {
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
    let a1745 = 0u64;
    let a1746 = v1;
    if nat_lt(a1745, a1746) {
        if f507(v0, {
            let a1747 = v1;
            let a1748 = 1u64;
            nat_sub(a1747, a1748)
        }) {
            Some({
                let a1749 = v1;
                let a1750 = 1u64;
                nat_sub(a1749, a1750)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f431(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1751 = v2;
    let a1752 = v0;
    if nat_lt(a1751, a1752) {
        if f507(v1, v2) {
            Ok(Some({
                let a1753 = v2;
                let a1754 = 1u64;
                nat_add(a1753, a1754)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f432(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1757 = f46(v0.clone(), {
        let a1755 = v1;
        let a1756 = 2u64;
        nat_add(a1755, a1756)?
    }, 2u64);
    let a1758 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1757, a1758) {
        let m1761 = f508(f46(v0, {
            let a1759 = v1;
            let a1760 = 4u64;
            nat_add(a1759, a1760)?
        }, 2u64))?;
        match m1761 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f115(v2.clone(), 0u64)?;
                let a1762 = v3;
                let a1763 = 32u64;
                if nat_lt(a1762, a1763) {
                    if f509(v3) {
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
    let a1764 = v0;
    let a1765 = 34u64;
    if nat_eq(a1764, a1765) {
        Bytes::lit(&[0x22])
    } else {
        let a1766 = v0;
        let a1767 = 92u64;
        if nat_eq(a1766, a1767) {
            Bytes::lit(&[0x5c])
        } else {
            let a1768 = v0;
            let a1769 = 98u64;
            if nat_eq(a1768, a1769) {
                Bytes::lit(&[0x08])
            } else {
                let a1770 = v0;
                let a1771 = 102u64;
                if nat_eq(a1770, a1771) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1772 = v0;
                    let a1773 = 110u64;
                    if nat_eq(a1772, a1773) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1774 = v0;
                        let a1775 = 114u64;
                        if nat_eq(a1774, a1775) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1776 = v0;
                            let a1777 = 116u64;
                            if nat_eq(a1776, a1777) {
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
    let a1778 = 47u64;
    let a1779 = v2;
    if if nat_lt(a1778, a1779) {
        let a1780 = v2;
        let a1781 = 58u64;
        nat_lt(a1780, a1781)
    } else {
        false
    } {
        Ok(Some({
            let a1782 = v1;
            let a1783 = 1u64;
            nat_add(a1782, a1783)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f435(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1784 = f115(v0, v1)?;
    let a1785 = 48u64;
    if nat_eq(a1784, a1785) {
        Ok(Some({
            let a1786 = v1;
            let a1787 = 1u64;
            nat_add(a1786, a1787)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f436(v0: Bytes) -> R<u64> {
    let (h1788, _) = f326(Fn3::F510(v0.clone()), {
        let a1789 = v0;
        length_bytes(a1789)
    }, 0u64)?;
    Ok(h1788)
}

pub fn f437(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1793 = {
        let a1791 = v0;
        let a1792 = {
            let (h1790, _) = v1;
            h1790
        };
        index_bytes(a1791, a1792)
    };
    match m1793 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1795 = {
                    let (h1794, _) = v1;
                    h1794
                };
                let a1796 = 1u64;
                nat_add(a1795, a1796)?
            }, {
                let a1800 = {
                    let a1798 = {
                        let (_, h1797) = v1;
                        h1797
                    };
                    let a1799 = 10u64;
                    nat_mul(a1798, a1799)?
                };
                let a1801 = f511(v2)?;
                nat_add(a1800, a1801)?
            })))
        }
    }
}

pub fn f438(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f512(v0, v1)
}

pub fn f439(v0: (Bytes, Bytes), p1804: List<(Bytes, Bytes)>) -> bool {
    let m1803 = p1804;
    {
        let mut m1803 = m1803;
        loop {
            let step = {
                let v1 = m1803;
                let m1802 = v1;
                match m1802.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f513(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1803 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1805 = v1;
    match m1805.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1806, _) = v2;
                h1806
            }, v0.clone()) {
                List::cons({
                    let (_, h1807) = v2;
                    h1807
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
    let m1808 = f93(v0, v1);
    match m1808 {
        None => {
            false
        }
        Some(v3) => {
            f224(v3, v2)
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
    let v1: Bytes = f201(v0)?;
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
        let a1810 = {
            let a1809 = v1;
            length_bytes(a1809)
        };
        let a1811 = v2;
        nat_sub(a1810, a1811)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1812 = f345(v3.clone(), f8(Str::lit(")")))?;
        match m1812 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1813 = v4;
                    let a1814 = 1u64;
                    nat_add(a1813, a1814)?
                }, {
                    let a1818 = {
                        let a1815 = v3;
                        length_bytes(a1815)
                    };
                    let a1819 = {
                        let a1816 = v4;
                        let a1817 = 1u64;
                        nat_add(a1816, a1817)?
                    };
                    nat_sub(a1818, a1819)
                })
            }
        }
    } else {
        v3
    };
    let a1820 = 0u64;
    let a1821 = v2;
    if nat_lt(a1820, a1821) {
        f514(v5)
    } else {
        Ok(false)
    }
}

pub fn f446(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f447(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1824 = {
            let a1822 = v4;
            let a1823 = v1;
            nat_add(a1822, a1823)?
        };
        let a1825 = v3;
        if nat_lt(a1824, a1825) {
            Ok(Some({
                let a1826 = v4;
                let a1827 = 1u64;
                nat_add(a1826, a1827)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f447(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1832 = {
        let a1829 = v0;
        let a1830 = v2;
        let a1831 = {
            let a1828 = v1.clone();
            length_bytes(a1828)
        };
        slice_bytes(a1829, a1830, a1831)
    };
    match m1832 {
        None => {
            false
        }
        Some(v3) => {
            let a1833 = v3;
            let a1834 = v1;
            equal(a1833, a1834)
        }
    }
}

pub fn f448(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1835 = v0;
    match m1835.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f448(v3, {
                let a1836 = v1;
                let a1837 = 1u64;
                nat_add(a1836, a1837)?
            })?))
        }
    }
}

pub fn f449(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1838 = v2;
    match m1838 {
        Adt21::C0(v3, v4) => {
            if f455(v3, match v0.clone() {
                Adt18::C0(h1839, _, _, _) => {
                    h1839
                }
            }) {
                f515(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f455(v5, match v0 {
                Adt18::C0(_, _, h1840, _) => {
                    h1840
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f451(v7, v1) {
                f455(v8, match v0 {
                    Adt18::C0(_, _, _, h1841) => {
                        h1841
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f450(v0: Adt15, v1: Adt2) -> R<Adt21> {
    let m1842 = f93(v1.clone(), f8(Str::lit("std")));
    match m1842 {
        None => {
            let m1847 = f92(f93(v1.clone(), f8(Str::lit("req"))));
            match m1847 {
                None => {
                    let m1849 = f93(v1, f8(Str::lit("der")));
                    match m1849 {
                        None => {
                            Ok(f516(v0))
                        }
                        Some(v5) => {
                            let m1850 = f517(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1850 {
                                None => {
                                    Ok(f516(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f353(f218(f191(f93(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1851) => {
                                            h1851
                                        }
                                    })?, f453(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1852, _) => {
                                            h1852
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f453(v4, match v0 {
                        Adt15::C0(_, _, h1848, _, _) => {
                            h1848
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1843 = f92(f93(v2.clone(), f8(Str::lit("row"))));
            match m1843 {
                None => {
                    Ok(Adt21::C0(f453(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1844, _, _, _, _) => {
                            h1844
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f453(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1845, _, _, _, _) => {
                            h1845
                        }
                    })?, Some(f453(v3, match v0 {
                        Adt15::C0(_, h1846, _, _, _) => {
                            h1846
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f451(p1855: List<u64>, v1: List<u64>) -> bool {
    let m1854 = p1855;
    {
        let mut m1854 = m1854;
        loop {
            let step = {
                let v0 = m1854;
                let m1853 = v0;
                match m1853.uncons() {
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
                Ok(next) => { m1854 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(p1860: List<u64>) -> bool {
    let m1859 = p1860;
    {
        let mut m1859 = m1859;
        loop {
            let step = {
                let v0 = m1859;
                let m1856 = v0;
                match m1856.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1857 = v1;
                        let a1858 = 7u64;
                        if nat_le(a1857, a1858) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1859 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1861 = f518(v0, v1.clone(), 0u64)?;
    match m1861 {
        None => {
            let a1862 = v1;
            Ok(length_list(a1862))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f454(v0: Adt2) -> u64 {
    let m1863 = f519(f93(v0, f8(Str::lit("stratum"))));
    match m1863 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f455(v0: u64, p1868: List<u64>) -> bool {
    let m1867 = p1868;
    {
        let mut m1867 = m1867;
        loop {
            let step = {
                let v1 = m1867;
                let m1864 = v1;
                match m1864.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1865 = v2;
                        let a1866 = v0;
                        if nat_eq(a1865, a1866) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1867 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: Fn7, p1872: List<Bytes>, p1873: List<(Bytes, ())>) -> List<Bytes> {
    let m1871 = (p1872, p1873);
    {
        let mut m1871 = m1871;
        loop {
            let step = {
                let (v1, v2) = m1871;
                let m1869 = v2;
                match m1869.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1870: Fn7 = v0.clone();
                            c1870.apply(v1, v3)
                        }, v4))
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

pub fn f457(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1874, _) = v1;
        h1874
    }, v0)
}

pub fn f458(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f520(Fn12::F521, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f459(v0: Fn8, p1878: List<List<(Bytes, ())>>, p1879: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1877 = (p1878, p1879);
    {
        let mut m1877 = m1877;
        loop {
            let step = {
                let (v1, v2) = m1877;
                let m1875 = v2;
                match m1875.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1876: Fn8 = v0.clone();
                            c1876.apply(v1, v3)
                        }, v4))
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

pub fn f460(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f461(v0: Fn9, p1884: u64, p1885: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1883 = (p1884, p1885);
    {
        let mut m1883 = m1883;
        loop {
            let step = {
                let (v1, v2) = m1883;
                let m1880 = v1;
                if m1880 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1880 - 1;
                    let m1882 = {
                        let c1881: Fn9 = v0.clone();
                        c1881.apply(v2.clone())?
                    };
                    match m1882 {
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
                Ok(next) => { m1883 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f522(v0)
}

pub fn f463(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f523(Fn13::F524, List::<(Bytes, ())>::nil(), v0)
}

pub fn f464(v0: Fn10, p1889: List<(Bytes, ())>, p1890: List<Bytes>) -> List<(Bytes, ())> {
    let m1888 = (p1889, p1890);
    {
        let mut m1888 = m1888;
        loop {
            let step = {
                let (v1, v2) = m1888;
                let m1886 = v2;
                match m1886.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1887: Fn10 = v0.clone();
                            c1887.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f466(p1895: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1894 = p1895;
    {
        let mut m1894 = m1894;
        loop {
            let step = {
                let v0 = m1894;
                let m1891 = v0;
                match m1891.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1892, _) = v2;
                            h1892
                        }) {
                            let (_, h1893) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1893)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
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

pub fn f467(v0: Bytes, p1921: u64, p1922: u64, p1923: Bytes) -> R<Bytes> {
    let m1920 = (p1921, (p1922, p1923));
    {
        let mut m1920 = m1920;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1920;
                let m1896 = v2;
                if m1896 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1896 - 1;
                    let m1899 = {
                        let a1897 = v0.clone();
                        let a1898 = v1;
                        index_bytes(a1897, a1898)
                    };
                    match m1899 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f206({
                                let a1900 = v6;
                                let a1901 = 34u64;
                                nat_eq(a1900, a1901)
                            }, {
                                let a1902 = v6;
                                let a1903 = 92u64;
                                nat_eq(a1902, a1903)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1904 = v1;
                                    let a1905 = 1u64;
                                    nat_add(a1904, a1905)?
                                }, (v4, {
                                    let a1908 = v3;
                                    let a1909 = {
                                        let a1906 = f8(Str::lit("\\"));
                                        let a1907 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1906, a1907)
                                    };
                                    append_bytes(a1908, a1909)
                                })))
                            } else {
                                let a1910 = v6;
                                let a1911 = 10u64;
                                if nat_eq(a1910, a1911) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1912 = v1;
                                        let a1913 = 1u64;
                                        nat_add(a1912, a1913)?
                                    }, (v4, {
                                        let a1914 = v3;
                                        let a1915 = f8(Str::lit("\\n"));
                                        append_bytes(a1914, a1915)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1916 = v1;
                                        let a1917 = 1u64;
                                        nat_add(a1916, a1917)?
                                    }, (v4, {
                                        let a1918 = v3;
                                        let a1919 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1918, a1919)
                                    })))
                                }
                            }
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

pub fn f468(v0: Option<Adt2>, v1: Bytes) -> Bytes {
    let m1924 = v0;
    match m1924 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f469(v0: List<Adt2>) -> List<Bytes> {
    let m1925 = v0;
    match m1925.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f525(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f526(f58(v1, f8(Str::lit("authority")))), f469(v2)))
        }
    }
}

pub fn f470(v0: List<Bytes>) -> R<List<Adt2>> {
    let m1926 = f302(v0.clone())?;
    match m1926 {
        None => {
            Ok(f291(f290(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt2>::nil())
        }
    }
}

pub fn f471(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    let m1927 = f302(v3.clone())?;
    match m1927 {
        None => {
            let m1928 = f290(v3.clone())?;
            match m1928 {
                None => {
                    f529(v0, v1, v3)
                }
                Some(v5) => {
                    f528(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f527(v0, v1, v4)
        }
    }
}

pub fn f472(v0: List<Bytes>) -> List<Bytes> {
    let m1929 = v0;
    match m1929.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1931 = {
                    let a1930 = v1.clone();
                    length_bytes(a1930)
                };
                let a1932 = 0u64;
                if nat_eq(a1931, a1932) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f472(v2))
        }
    }
}

pub fn f473(v0: Bytes, v1: u64) -> R<u64> {
    let m1935 = {
        let a1933 = v0;
        let a1934 = v1;
        index_bytes(a1933, a1934)
    };
    match m1935 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f474(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1939 = {
        let a1936 = v1;
        let a1937 = v2;
        let a1938 = 1u64;
        slice_bytes(a1936, a1937, a1938)
    };
    match m1939 {
        None => {
            v0
        }
        Some(v3) => {
            let a1940 = v0;
            let a1941 = v3;
            append_bytes(a1940, a1941)
        }
    }
}

pub fn f475(p1944: List<Bytes>, p1945: List<Bytes>) -> List<Bytes> {
    let m1943 = (p1944, p1945);
    {
        let mut m1943 = m1943;
        loop {
            let step = {
                let (v0, v1) = m1943;
                let m1942 = v0;
                match m1942.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1943 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(v0: List<Bytes>) -> Bytes {
    let m1946 = v0;
    match m1946.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f477(p1949: List<List<Bytes>>, v1: Bytes, v2: Bytes) -> R<bool> {
    let m1948 = p1949;
    {
        let mut m1948 = m1948;
        loop {
            let step = {
                let v0 = m1948;
                let m1947 = v0;
                match m1947.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        if f117(f85(f530(v3.clone(), 0u64), v1.clone()), f52(v2.clone(), f44(f530(v3, 4u64), 32u8)?)) {
                            Err::<List<List<Bytes>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<Bytes>>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1948 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(v0: Bytes) -> R<u64> {
    let m1952 = {
        let a1950 = v0;
        let a1951 = 0u64;
        index_bytes(a1950, a1951)
    };
    match m1952 {
        None => {
            Ok(256u64)
        }
        Some(v1) => {
            f72(v1)
        }
    }
}

pub fn f479(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1953 = v0;
    match m1953.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f314(f531(v1.clone())?)?, List::cons(f8(Str::lit(":")), List::cons(f532(v1), List::<Bytes>::nil())))), f479(v2)?))
        }
    }
}

pub fn f480(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1954 = v0;
    match m1954.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f85(v1.clone(), f8(Str::lit("["))) {
                f480(v2)
            } else {
                Ok(List::cons(f314(f531(v1)?)?, f480(v2)?))
            }
        }
    }
}

pub fn f481(v0: Adt2) -> Bytes {
    if f224(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f224(v0.clone(), f8(Str::lit("id"))) {
            if f224(v0.clone(), f8(Str::lit("statement"))) {
                if f224(v0.clone(), f8(Str::lit("stakeholder"))) {
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

pub fn f482(v0: Bytes) -> R<bool> {
    let m1956 = {
        let a1955 = v0.clone();
        length_bytes(a1955)
    };
    if m1956 == 0 {
        Ok(true)
    } else {
        let v1 = m1956 - 1;
        let m1959 = {
            let a1957 = v0;
            let a1958 = v1;
            index_bytes(a1957, a1958)
        };
        match m1959 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f533({
                    let v3: u8 = v2;
                    let a2004 = {
                        let a1980 = {
                            let a1968 = {
                                let a1962 = {
                                    let a1960 = v3;
                                    let a1961 = 128u8;
                                    fixed_u8::bit_and(a1960, a1961)
                                };
                                let a1963 = 128u8;
                                if equal(a1962, a1963) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1969 = {
                                let a1966 = {
                                    let a1964 = v3;
                                    let a1965 = 64u8;
                                    fixed_u8::bit_and(a1964, a1965)
                                };
                                let a1967 = 64u8;
                                if equal(a1966, a1967) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1968, a1969)?
                        };
                        let a1981 = {
                            let a1978 = {
                                let a1972 = {
                                    let a1970 = v3;
                                    let a1971 = 32u8;
                                    fixed_u8::bit_and(a1970, a1971)
                                };
                                let a1973 = 32u8;
                                if equal(a1972, a1973) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1979 = {
                                let a1976 = {
                                    let a1974 = v3;
                                    let a1975 = 16u8;
                                    fixed_u8::bit_and(a1974, a1975)
                                };
                                let a1977 = 16u8;
                                if equal(a1976, a1977) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1978, a1979)?
                        };
                        nat_add(a1980, a1981)?
                    };
                    let a2005 = {
                        let a2002 = {
                            let a1990 = {
                                let a1984 = {
                                    let a1982 = v3;
                                    let a1983 = 8u8;
                                    fixed_u8::bit_and(a1982, a1983)
                                };
                                let a1985 = 8u8;
                                if equal(a1984, a1985) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1991 = {
                                let a1988 = {
                                    let a1986 = v3;
                                    let a1987 = 4u8;
                                    fixed_u8::bit_and(a1986, a1987)
                                };
                                let a1989 = 4u8;
                                if equal(a1988, a1989) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1990, a1991)?
                        };
                        let a2003 = {
                            let a2000 = {
                                let a1994 = {
                                    let a1992 = v3;
                                    let a1993 = 2u8;
                                    fixed_u8::bit_and(a1992, a1993)
                                };
                                let a1995 = 2u8;
                                if equal(a1994, a1995) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2001 = {
                                let a1998 = {
                                    let a1996 = v3;
                                    let a1997 = 1u8;
                                    fixed_u8::bit_and(a1996, a1997)
                                };
                                let a1999 = 1u8;
                                if equal(a1998, a1999) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2000, a2001)?
                        };
                        nat_add(a2002, a2003)?
                    };
                    nat_add(a2004, a2005)?
                }))
            }
        }
    }
}

pub fn f483(v0: Adt2, v1: Bytes) -> bool {
    let a2007 = {
        let a2006 = f58(v0, v1);
        length_bytes(a2006)
    };
    let a2008 = 0u64;
    !nat_eq(a2007, a2008)
}

pub fn f484(v0: Adt2, v1: Bytes) -> bool {
    let m2009 = f93(v0, v1);
    match m2009 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f485(v0: List<Bytes>) -> Option<Adt2> {
    let m2010 = v0;
    match m2010.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m2011 = v2;
                match m2011.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m2012 = v2;
                    match m2012.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m2013 = v6;
                                match m2013.uncons() {
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
                        let m2014 = v2;
                        match m2014.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m2015 = v10;
                                match m2015.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C11(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C6(v11)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f405(v12)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C6(v9)), std::rc::Rc::new(Adt2::C9))))))))), std::rc::Rc::new(Adt2::C9)))
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

pub fn f486(v0: Bytes, p2036: u64, p2037: u64, p2038: bool, p2039: Bytes) -> R<Bytes> {
    let m2035 = (p2036, (p2037, (p2038, p2039)));
    {
        let mut m2035 = m2035;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m2035;
                let m2016 = v2;
                if m2016 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m2016 - 1;
                    let m2019 = {
                        let a2017 = v0.clone();
                        let a2018 = v1;
                        index_bytes(a2017, a2018)
                    };
                    match m2019 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f534(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a2020 = v1;
                                    let a2021 = 1u64;
                                    nat_add(a2020, a2021)?
                                }, (v5, (true, v4))))
                            } else {
                                if f117(f535(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a2022 = v1;
                                        let a2023 = 1u64;
                                        nat_add(a2022, a2023)?
                                    }, (v5, (false, {
                                        let a2027 = v4;
                                        let a2028 = f46({
                                            let a2024 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a2024)
                                        }, {
                                            let a2025 = v7;
                                            let a2026 = 97u64;
                                            nat_sub(a2025, a2026)
                                        }, 1u64);
                                        append_bytes(a2027, a2028)
                                    }))))
                                } else {
                                    if f206(f535(v7), f206(f536(v7), f537(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2029 = v1;
                                            let a2030 = 1u64;
                                            nat_add(a2029, a2030)?
                                        }, (v5, (false, {
                                            let a2031 = v4;
                                            let a2032 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a2031, a2032)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2033 = v1;
                                            let a2034 = 1u64;
                                            nat_add(a2033, a2034)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
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

pub fn f487(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f404(f538(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f488(v0: u32) -> Bytes {
    let a2068 = f539({
        let a2040 = f540(v0, 28u32);
        let a2041 = 15u32;
        fixed_u32::bit_and(a2040, a2041)
    });
    let a2069 = {
        let a2066 = f539({
            let a2042 = f540(v0, 24u32);
            let a2043 = 15u32;
            fixed_u32::bit_and(a2042, a2043)
        });
        let a2067 = {
            let a2064 = f539({
                let a2044 = f540(v0, 20u32);
                let a2045 = 15u32;
                fixed_u32::bit_and(a2044, a2045)
            });
            let a2065 = {
                let a2062 = f539({
                    let a2046 = f540(v0, 16u32);
                    let a2047 = 15u32;
                    fixed_u32::bit_and(a2046, a2047)
                });
                let a2063 = {
                    let a2060 = f539({
                        let a2048 = f540(v0, 12u32);
                        let a2049 = 15u32;
                        fixed_u32::bit_and(a2048, a2049)
                    });
                    let a2061 = {
                        let a2058 = f539({
                            let a2050 = f540(v0, 8u32);
                            let a2051 = 15u32;
                            fixed_u32::bit_and(a2050, a2051)
                        });
                        let a2059 = {
                            let a2056 = f539({
                                let a2052 = f540(v0, 4u32);
                                let a2053 = 15u32;
                                fixed_u32::bit_and(a2052, a2053)
                            });
                            let a2057 = f539({
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

pub fn f489(v0: Fn11, p2073: u64, p2074: (u64, Adt23)) -> R<(u64, Adt23)> {
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

pub fn f490(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt23)) -> R<(u64, Adt23)> {
    Ok(({
        let a2076 = {
            let (h2075, _) = v3.clone();
            h2075
        };
        let a2077 = 64u64;
        nat_add(a2076, a2077)?
    }, f541({
        let (_, h2078) = v3;
        h2078
    }, f542(v0, v1, v2, {
        let (h2079, _) = v3;
        h2079
    })?)?))
}

pub fn f491() -> Adt23 {
    Adt23::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f492(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2080 = f44(v1, 34u8)?;
    match m2080.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2081 = v3;
            match m2081.uncons() {
                None => {
                    Ok(f543(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f493(v0: List<Bytes>) -> List<Bytes> {
    let m2082 = v0;
    match m2082.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f493(v2)
            } else {
                List::cons(v1, f493(v2))
            }
        }
    }
}

pub fn f494(p2085: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2084 = p2085;
    {
        let mut m2084 = m2084;
        loop {
            let step = {
                let v0 = m2084;
                let m2083 = v0;
                match m2083.uncons() {
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
                Ok(next) => { m2084 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2086 = v2;
    let a2087 = 0u64;
    if nat_eq(a2086, a2087) {
        Ok(f544(v0, 0u64))
    } else {
        let (_, h2088) = {
            let (h2089, _) = f545(Fn14::F546(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2089
        };
        Ok(h2088)
    }
}

pub fn f496() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f497(v0: List<u64>) -> R<Bytes> {
    let m2090 = f547(v0, List::<u64>::nil());
    match m2090.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a2091 = f410(v1)?;
            let a2092 = f548(v2)?;
            Ok(append_bytes(a2091, a2092))
        }
    }
}

pub fn f498(p2095: u64, p2096: List<u64>, v2: u64) -> R<List<u64>> {
    let m2094 = (p2095, p2096);
    {
        let mut m2094 = m2094;
        loop {
            let step = {
                let (v0, v1) = m2094;
                let m2093 = v0;
                if m2093 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m2093 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f549(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m2094 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f499(v0: u64, v1: u64) -> List<u64> {
    let m2097 = v0;
    if m2097 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m2097 - 1;
        let a2098 = v1;
        let a2099 = 0u64;
        if nat_eq(a2098, a2099) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a2100 = v1;
                let a2101 = f550();
                let a2102 = 0u64;
                nat_rem(a2100, a2101, a2102)
            }, f499(v2, {
                let a2103 = v1;
                let a2104 = f550();
                let a2105 = 0u64;
                nat_quot(a2103, a2104, a2105)
            }))
        }
    }
}

pub fn f500(p2116: Bytes, p2117: u64) -> R<Bytes> {
    let m2115 = (p2116, p2117);
    {
        let mut m2115 = m2115;
        loop {
            let step = {
                let (v0, v1) = m2115;
                let m2106 = v1;
                if m2106 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m2106 - 1;
                    let a2110 = f115(v0.clone(), {
                        let a2108 = {
                            let a2107 = v0.clone();
                            length_bytes(a2107)
                        };
                        let a2109 = 1u64;
                        nat_sub(a2108, a2109)
                    })?;
                    let a2111 = 48u64;
                    if nat_eq(a2110, a2111) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a2113 = {
                                let a2112 = v0;
                                length_bytes(a2112)
                            };
                            let a2114 = 1u64;
                            nat_sub(a2113, a2114)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m2115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(v0: u64) -> Bytes {
    let m2118 = v0;
    if m2118 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2118 - 1;
        let a2119 = Bytes::lit(&[0x30]);
        let a2120 = f501(v1);
        append_bytes(a2119, a2120)
    }
}

pub fn f502(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2165 = {
            let a2141 = {
                let a2129 = {
                    let a2123 = {
                        let a2121 = v3;
                        let a2122 = 128u8;
                        fixed_u8::bit_and(a2121, a2122)
                    };
                    let a2124 = 128u8;
                    if equal(a2123, a2124) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2130 = {
                    let a2127 = {
                        let a2125 = v3;
                        let a2126 = 64u8;
                        fixed_u8::bit_and(a2125, a2126)
                    };
                    let a2128 = 64u8;
                    if equal(a2127, a2128) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2129, a2130)?
            };
            let a2142 = {
                let a2139 = {
                    let a2133 = {
                        let a2131 = v3;
                        let a2132 = 32u8;
                        fixed_u8::bit_and(a2131, a2132)
                    };
                    let a2134 = 32u8;
                    if equal(a2133, a2134) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2140 = {
                    let a2137 = {
                        let a2135 = v3;
                        let a2136 = 16u8;
                        fixed_u8::bit_and(a2135, a2136)
                    };
                    let a2138 = 16u8;
                    if equal(a2137, a2138) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2139, a2140)?
            };
            nat_add(a2141, a2142)?
        };
        let a2166 = {
            let a2163 = {
                let a2151 = {
                    let a2145 = {
                        let a2143 = v3;
                        let a2144 = 8u8;
                        fixed_u8::bit_and(a2143, a2144)
                    };
                    let a2146 = 8u8;
                    if equal(a2145, a2146) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2152 = {
                    let a2149 = {
                        let a2147 = v3;
                        let a2148 = 4u8;
                        fixed_u8::bit_and(a2147, a2148)
                    };
                    let a2150 = 4u8;
                    if equal(a2149, a2150) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2151, a2152)?
            };
            let a2164 = {
                let a2161 = {
                    let a2155 = {
                        let a2153 = v3;
                        let a2154 = 2u8;
                        fixed_u8::bit_and(a2153, a2154)
                    };
                    let a2156 = 2u8;
                    if equal(a2155, a2156) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2162 = {
                    let a2159 = {
                        let a2157 = v3;
                        let a2158 = 1u8;
                        fixed_u8::bit_and(a2157, a2158)
                    };
                    let a2160 = 1u8;
                    if equal(a2159, a2160) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2161, a2162)?
            };
            nat_add(a2163, a2164)?
        };
        nat_add(a2165, a2166)?
    };
    let a2167 = v4;
    let a2168 = 34u64;
    if nat_eq(a2167, a2168) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2169 = v4;
        let a2170 = 92u64;
        if nat_eq(a2169, a2170) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2171 = v4;
            let a2172 = 8u64;
            if nat_eq(a2171, a2172) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2173 = v4;
                let a2174 = 12u64;
                if nat_eq(a2173, a2174) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2175 = v4;
                    let a2176 = 10u64;
                    if nat_eq(a2175, a2176) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2177 = v4;
                        let a2178 = 13u64;
                        if nat_eq(a2177, a2178) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2179 = v4;
                            let a2180 = 9u64;
                            if nat_eq(a2179, a2180) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2181 = v4;
                                let a2182 = 32u64;
                                if nat_lt(a2181, a2182) {
                                    let a2183 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2184 = f551(v2)?;
                                    Ok(append_bytes(a2183, a2184))
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

pub fn f503(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2185 = v0.clone();
    match m2185.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2188 = {
                let (h2186, _) = v2;
                h2186
            };
            let a2189 = {
                let (h2187, _) = v1.clone();
                h2187
            };
            if equal(a2188, a2189) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f504(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f552(f553(Fn15::F554, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2194 = {
        let (h2190, _) = f555(Fn16::F556, {
            let a2192 = {
                let a2191 = v0;
                length_list(a2191)
            };
            let a2193 = 1u64;
            nat_add(a2192, a2193)?
        }, v1)?;
        h2190
    };
    match m2194.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f505(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2199 = {
        let a2196 = f314({
            let (h2195, _) = v0;
            h2195
        })?;
        let a2197 = Bytes::lit(&[0x3a]);
        append_bytes(a2196, a2197)
    };
    let a2200 = {
        let (_, h2198) = v0;
        h2198
    };
    Ok(append_bytes(a2199, a2200))
}

pub fn f506(v0: Bytes) -> R<Bytes> {
    let a2206 = {
        let a2204 = Bytes::lit(&[0x22]);
        let a2205 = {
            let (_, h2201) = {
                let (h2202, _) = f545(Fn14::F557(v0.clone()), {
                    let a2203 = v0;
                    length_bytes(a2203)
                }, (0u64, Bytes::lit(&[])))?;
                h2202
            };
            h2201
        };
        append_bytes(a2204, a2205)
    };
    let a2207 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2206, a2207))
}

pub fn f507(v0: Bytes, v1: u64) -> bool {
    let m2208 = f211(v0, v1);
    match m2208 {
        None => {
            false
        }
        Some(v2) => {
            f558(v2)
        }
    }
}

pub fn f508(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2209 = v0.clone();
        length_bytes(a2209)
    };
    let a2213 = {
        let a2210 = v1;
        let a2211 = 2u64;
        let a2212 = 0u64;
        nat_rem(a2210, a2211, a2212)
    };
    let a2214 = 0u64;
    if nat_eq(a2213, a2214) {
        let (_, h2215) = {
            let (h2216, _) = f559(Fn17::F560(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2216
        };
        Ok(h2215)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f509(v0: u64) -> bool {
    let a2217 = v0;
    let a2218 = 8u64;
    if nat_eq(a2217, a2218) {
        true
    } else {
        let a2219 = v0;
        let a2220 = 9u64;
        if nat_eq(a2219, a2220) {
            true
        } else {
            let a2221 = v0;
            let a2222 = 10u64;
            if nat_eq(a2221, a2222) {
                true
            } else {
                let a2223 = v0;
                let a2224 = 12u64;
                if nat_eq(a2223, a2224) {
                    true
                } else {
                    let a2225 = v0;
                    let a2226 = 13u64;
                    nat_eq(a2225, a2226)
                }
            }
        }
    }
}

pub fn f510(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2229 = {
        let a2227 = v0;
        let a2228 = v1;
        index_bytes(a2227, a2228)
    };
    match m2229 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f561(v2)? {
                Ok(Some({
                    let a2230 = v1;
                    let a2231 = 1u64;
                    nat_add(a2230, a2231)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f511(v0: u8) -> R<u64> {
    let a2278 = {
        let v1: u8 = v0;
        let a2276 = {
            let a2252 = {
                let a2240 = {
                    let a2234 = {
                        let a2232 = v1;
                        let a2233 = 128u8;
                        fixed_u8::bit_and(a2232, a2233)
                    };
                    let a2235 = 128u8;
                    if equal(a2234, a2235) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2241 = {
                    let a2238 = {
                        let a2236 = v1;
                        let a2237 = 64u8;
                        fixed_u8::bit_and(a2236, a2237)
                    };
                    let a2239 = 64u8;
                    if equal(a2238, a2239) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2240, a2241)?
            };
            let a2253 = {
                let a2250 = {
                    let a2244 = {
                        let a2242 = v1;
                        let a2243 = 32u8;
                        fixed_u8::bit_and(a2242, a2243)
                    };
                    let a2245 = 32u8;
                    if equal(a2244, a2245) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2251 = {
                    let a2248 = {
                        let a2246 = v1;
                        let a2247 = 16u8;
                        fixed_u8::bit_and(a2246, a2247)
                    };
                    let a2249 = 16u8;
                    if equal(a2248, a2249) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2250, a2251)?
            };
            nat_add(a2252, a2253)?
        };
        let a2277 = {
            let a2274 = {
                let a2262 = {
                    let a2256 = {
                        let a2254 = v1;
                        let a2255 = 8u8;
                        fixed_u8::bit_and(a2254, a2255)
                    };
                    let a2257 = 8u8;
                    if equal(a2256, a2257) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2263 = {
                    let a2260 = {
                        let a2258 = v1;
                        let a2259 = 4u8;
                        fixed_u8::bit_and(a2258, a2259)
                    };
                    let a2261 = 4u8;
                    if equal(a2260, a2261) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2262, a2263)?
            };
            let a2275 = {
                let a2272 = {
                    let a2266 = {
                        let a2264 = v1;
                        let a2265 = 2u8;
                        fixed_u8::bit_and(a2264, a2265)
                    };
                    let a2267 = 2u8;
                    if equal(a2266, a2267) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2273 = {
                    let a2270 = {
                        let a2268 = v1;
                        let a2269 = 1u8;
                        fixed_u8::bit_and(a2268, a2269)
                    };
                    let a2271 = 1u8;
                    if equal(a2270, a2271) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2272, a2273)?
            };
            nat_add(a2274, a2275)?
        };
        nat_add(a2276, a2277)?
    };
    let a2279 = 48u64;
    Ok(nat_sub(a2278, a2279))
}

pub fn f512(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2280 = v0.clone();
        length_bytes(a2280)
    };
    let a2281 = v1;
    let a2282 = v2;
    if if nat_lt(a2281, a2282) {
        let a2287 = f115(v0, {
            let a2285 = v2;
            let a2286 = {
                let a2283 = v1;
                let a2284 = 1u64;
                nat_add(a2283, a2284)?
            };
            nat_sub(a2285, a2286)
        })?;
        let a2288 = 48u64;
        nat_eq(a2287, a2288)
    } else {
        false
    } {
        Ok(Some({
            let a2289 = v1;
            let a2290 = 1u64;
            nat_add(a2289, a2290)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f513(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f117(f32({
        let (h2291, _) = v0;
        h2291
    }, {
        let (h2292, _) = v1;
        h2292
    }), f32({
        let (_, h2293) = v0;
        h2293
    }, {
        let (_, h2294) = v1;
        h2294
    }))
}

pub fn f514(v0: Bytes) -> R<bool> {
    let m2295 = f345(v0.clone(), f8(Str::lit("&&")))?;
    if match m2295 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2296 = f345(v0, f8(Str::lit("||")))?;
        match m2296 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f515(v0: Adt18, v1: Option<u64>) -> bool {
    let m2297 = v1;
    match m2297 {
        None => {
            true
        }
        Some(v2) => {
            f455(v2, match v0 {
                Adt18::C0(_, h2298, _, _) => {
                    h2298
                }
            })
        }
    }
}

pub fn f516(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a2300 = match v0 {
            Adt15::C0(_, _, h2299, _, _) => {
                h2299
            }
        };
        length_list(a2300)
    })
}

pub fn f517(v0: Bytes) -> Option<Adt22> {
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

pub fn f518(v0: Bytes, p2305: List<Bytes>, p2306: u64) -> R<Option<u64>> {
    let m2304 = (p2305, p2306);
    {
        let mut m2304 = m2304;
        loop {
            let step = {
                let (v1, v2) = m2304;
                let m2301 = v1;
                match m2301.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2302 = v2;
                                let a2303 = 1u64;
                                nat_add(a2302, a2303)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f519(v0: Option<Adt2>) -> Option<u64> {
    let m2307 = v0;
    match m2307 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2308 = v1;
            match m2308 {
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

pub fn f520(v0: Fn12, p2312: List<List<(Bytes, ())>>, p2313: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2311 = (p2312, p2313);
    {
        let mut m2311 = m2311;
        loop {
            let step = {
                let (v1, v2) = m2311;
                let m2309 = v2;
                match m2309.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2310: Fn12 = v0.clone();
                            c2310.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f522(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2315 = {
        let a2314 = v0.clone();
        length_list(a2314)
    };
    let a2316 = 1u64;
    if nat_le(a2315, a2316) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f562(v0)?))
    }
}

pub fn f523(v0: Fn13, p2320: List<(Bytes, ())>, p2321: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2319 = (p2320, p2321);
    {
        let mut m2319 = m2319;
        loop {
            let step = {
                let (v1, v2) = m2319;
                let m2317 = v2;
                match m2317.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2318: Fn13 = v0.clone();
                            c2318.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f524(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f525(v0: Bytes) -> Bytes {
    f7(List::cons(f139(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f526(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f527(v0: Adt1, v1: Bytes, v2: List<Adt2>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f142(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f142(f96(v0, f153())?, f8(Str::lit("at"))));
    let a2323 = {
        let a2322 = v4.clone();
        length_list(a2322)
    };
    let a2324 = 0u64;
    if nat_eq(a2323, a2324) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2327 = {
            let a2325 = v4;
            length_list(a2325)
        };
        let a2328 = {
            let a2326 = v3;
            length_list(a2326)
        };
        if nat_eq(a2327, a2328) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f563(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f528(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt2>) -> R<(bool, Bytes)> {
    let m2329 = v3;
    match m2329.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f206(f564(v0.clone(), v6.clone())?, f52(v6.clone(), v2.clone())) {
                f528(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f526(v6.clone()))) {
                    let v9: (bool, Bytes) = f528(v0, v1.clone(), v2, v5)?;
                    let (h2330, _) = v9.clone();
                    if h2330 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2332 = f563(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2333 = {
                                let (_, h2331) = v9;
                                h2331
                            };
                            append_bytes(a2332, a2333)
                        }))
                    }
                } else {
                    if f117(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f565(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2334 = f8(Str::lit("Catalog."));
                        let a2335 = v8.clone();
                        append_bytes(a2334, a2335)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f528(v0, v1.clone(), v2, v5)?;
                        let (h2336, _) = v10.clone();
                        if h2336 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2338 = f563(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2339 = {
                                    let (_, h2337) = v10;
                                    h2337
                                };
                                append_bytes(a2338, a2339)
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

pub fn f529(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt2> = f291(f391(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt2> = f291(f391(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f125(f142(v3.clone(), f8(Str::lit("id"))), f142(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f125(v6.clone(), f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2341 = {
        let a2340 = v7.clone();
        length_list(a2340)
    };
    let a2342 = 0u64;
    if nat_eq(a2341, a2342) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2345 = {
                let a2343 = v7;
                length_list(a2343)
            };
            let a2346 = {
                let a2344 = v5;
                length_list(a2344)
            };
            nat_eq(a2345, a2346)
        }) {
            Ok((false, f563(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f145(f125(f62(f566(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f567(v4), f125(v6, f142(v3, f8(Str::lit("id")))))))?;
            let a2348 = {
                let a2347 = v8.clone();
                length_list(a2347)
            };
            let a2349 = 0u64;
            if nat_eq(a2348, a2349) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f563(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f122(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f530(p2356: List<Bytes>, p2357: u64) -> Bytes {
    let m2355 = (p2356, p2357);
    {
        let mut m2355 = m2355;
        loop {
            let step = {
                let (v0, v1) = m2355;
                let m2350 = v0;
                match m2350.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a2351 = v1;
                        let a2352 = 0u64;
                        if nat_eq(a2351, a2352) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a2353 = v1;
                                let a2354 = 1u64;
                                nat_sub(a2353, a2354)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f531(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = if f85(v0.clone(), f8(Str::lit("["))) {
        f46(v0.clone(), 1u64, {
            let a2359 = {
                let a2358 = v0;
                length_bytes(a2358)
            };
            let a2360 = 2u64;
            nat_sub(a2359, a2360)
        })
    } else {
        v0
    };
    let v2: Bytes = if f41(v1.clone(), f8(Str::lit("..."))) {
        f46(v1.clone(), 0u64, {
            let a2362 = {
                let a2361 = v1;
                length_bytes(a2361)
            };
            let a2363 = 3u64;
            nat_sub(a2362, a2363)
        })
    } else {
        v1
    };
    let m2364 = f44(v2.clone(), 124u8)?;
    match m2364.uncons() {
        None => {
            Ok(v2)
        }
        Some((v3, _)) => {
            Ok(v3)
        }
    }
}

pub fn f532(v0: Bytes) -> Bytes {
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

pub fn f533(v0: u64) -> bool {
    let a2365 = v0;
    let a2366 = 47u64;
    nat_eq(a2365, a2366)
}

pub fn f534(v0: u64) -> bool {
    f206({
        let a2367 = v0;
        let a2368 = 95u64;
        nat_eq(a2367, a2368)
    }, f206({
        let a2369 = v0;
        let a2370 = 45u64;
        nat_eq(a2369, a2370)
    }, f206({
        let a2371 = v0;
        let a2372 = 32u64;
        nat_eq(a2371, a2372)
    }, f206({
        let a2373 = v0;
        let a2374 = 46u64;
        nat_eq(a2373, a2374)
    }, {
        let a2375 = v0;
        let a2376 = 47u64;
        nat_eq(a2375, a2376)
    }))))
}

pub fn f535(v0: u64) -> bool {
    f117({
        let a2377 = 97u64;
        let a2378 = v0;
        nat_le(a2377, a2378)
    }, {
        let a2379 = v0;
        let a2380 = 122u64;
        nat_le(a2379, a2380)
    })
}

pub fn f536(v0: u64) -> bool {
    f117({
        let a2381 = 65u64;
        let a2382 = v0;
        nat_le(a2381, a2382)
    }, {
        let a2383 = v0;
        let a2384 = 90u64;
        nat_le(a2383, a2384)
    })
}

pub fn f537(v0: u64) -> bool {
    f117({
        let a2385 = 48u64;
        let a2386 = v0;
        nat_le(a2385, a2386)
    }, {
        let a2387 = v0;
        let a2388 = 57u64;
        nat_le(a2387, a2388)
    })
}

pub fn f538(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2389 = v0;
    match m2389.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f364(v1)?, f538(v2)?))
        }
    }
}

pub fn f539(v0: u32) -> Bytes {
    let a2392 = {
        let a2390 = v0;
        let a2391 = 8u32;
        fixed_u32::bit_and(a2390, a2391)
    };
    let a2393 = 0u32;
    if equal(a2392, a2393) {
        let a2396 = {
            let a2394 = v0;
            let a2395 = 4u32;
            fixed_u32::bit_and(a2394, a2395)
        };
        let a2397 = 0u32;
        if equal(a2396, a2397) {
            let a2400 = {
                let a2398 = v0;
                let a2399 = 2u32;
                fixed_u32::bit_and(a2398, a2399)
            };
            let a2401 = 0u32;
            if equal(a2400, a2401) {
                let a2404 = {
                    let a2402 = v0;
                    let a2403 = 1u32;
                    fixed_u32::bit_and(a2402, a2403)
                };
                let a2405 = 0u32;
                if equal(a2404, a2405) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2408 = {
                    let a2406 = v0;
                    let a2407 = 1u32;
                    fixed_u32::bit_and(a2406, a2407)
                };
                let a2409 = 0u32;
                if equal(a2408, a2409) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2412 = {
                let a2410 = v0;
                let a2411 = 2u32;
                fixed_u32::bit_and(a2410, a2411)
            };
            let a2413 = 0u32;
            if equal(a2412, a2413) {
                let a2416 = {
                    let a2414 = v0;
                    let a2415 = 1u32;
                    fixed_u32::bit_and(a2414, a2415)
                };
                let a2417 = 0u32;
                if equal(a2416, a2417) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2420 = {
                    let a2418 = v0;
                    let a2419 = 1u32;
                    fixed_u32::bit_and(a2418, a2419)
                };
                let a2421 = 0u32;
                if equal(a2420, a2421) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2424 = {
            let a2422 = v0;
            let a2423 = 4u32;
            fixed_u32::bit_and(a2422, a2423)
        };
        let a2425 = 0u32;
        if equal(a2424, a2425) {
            let a2428 = {
                let a2426 = v0;
                let a2427 = 2u32;
                fixed_u32::bit_and(a2426, a2427)
            };
            let a2429 = 0u32;
            if equal(a2428, a2429) {
                let a2432 = {
                    let a2430 = v0;
                    let a2431 = 1u32;
                    fixed_u32::bit_and(a2430, a2431)
                };
                let a2433 = 0u32;
                if equal(a2432, a2433) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2436 = {
                    let a2434 = v0;
                    let a2435 = 1u32;
                    fixed_u32::bit_and(a2434, a2435)
                };
                let a2437 = 0u32;
                if equal(a2436, a2437) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2440 = {
                let a2438 = v0;
                let a2439 = 2u32;
                fixed_u32::bit_and(a2438, a2439)
            };
            let a2441 = 0u32;
            if equal(a2440, a2441) {
                let a2444 = {
                    let a2442 = v0;
                    let a2443 = 1u32;
                    fixed_u32::bit_and(a2442, a2443)
                };
                let a2445 = 0u32;
                if equal(a2444, a2445) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2448 = {
                    let a2446 = v0;
                    let a2447 = 1u32;
                    fixed_u32::bit_and(a2446, a2447)
                };
                let a2449 = 0u32;
                if equal(a2448, a2449) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f540(v0: u32, v1: u32) -> u32 {
    let m2452 = {
        let a2450 = v0;
        let a2451 = v1;
        fixed_u32::shift_right(a2450, a2451)
    };
    match m2452 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f541(v0: Adt23, v1: Adt24) -> R<Adt23> {
    let v2: Adt25 = f568(Fn18::F569, 64u64, Adt25::C0(match v0.clone() {
        Adt23::C0(h2453, _, _, _, _, _, _, _) => {
            h2453
        }
    }, match v0.clone() {
        Adt23::C0(_, h2454, _, _, _, _, _, _) => {
            h2454
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h2455, _, _, _, _, _) => {
            h2455
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h2456, _, _, _, _) => {
            h2456
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, h2457, _, _, _) => {
            h2457
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2458, _, _) => {
            h2458
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2459, _) => {
            h2459
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h2460) => {
            h2460
        }
    }, v1, 0u64))?;
    Ok(Adt23::C0(f570(match v0.clone() {
        Adt23::C0(h2461, _, _, _, _, _, _, _) => {
            h2461
        }
    }, match v2.clone() {
        Adt25::C0(h2462, _, _, _, _, _, _, _, _, _) => {
            h2462
        }
    }), f570(match v0.clone() {
        Adt23::C0(_, h2463, _, _, _, _, _, _) => {
            h2463
        }
    }, match v2.clone() {
        Adt25::C0(_, h2464, _, _, _, _, _, _, _, _) => {
            h2464
        }
    }), f570(match v0.clone() {
        Adt23::C0(_, _, h2465, _, _, _, _, _) => {
            h2465
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h2466, _, _, _, _, _, _, _) => {
            h2466
        }
    }), f570(match v0.clone() {
        Adt23::C0(_, _, _, h2467, _, _, _, _) => {
            h2467
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h2468, _, _, _, _, _, _) => {
            h2468
        }
    }), f570(match v0.clone() {
        Adt23::C0(_, _, _, _, h2469, _, _, _) => {
            h2469
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h2470, _, _, _, _, _) => {
            h2470
        }
    }), f570(match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2471, _, _) => {
            h2471
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h2472, _, _, _, _) => {
            h2472
        }
    }), f570(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2473, _) => {
            h2473
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h2474, _, _, _) => {
            h2474
        }
    }), f570(match v0 {
        Adt23::C0(_, _, _, _, _, _, _, h2475) => {
            h2475
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h2476, _, _) => {
            h2476
        }
    })))
}

pub fn f542(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f571(v0.clone(), v1, v2, v3)?, f571(v0.clone(), v1, v2, {
        let a2477 = v3;
        let a2478 = 4u64;
        nat_add(a2477, a2478)?
    })?, f571(v0.clone(), v1, v2, {
        let a2479 = v3;
        let a2480 = 8u64;
        nat_add(a2479, a2480)?
    })?, f571(v0.clone(), v1, v2, {
        let a2481 = v3;
        let a2482 = 12u64;
        nat_add(a2481, a2482)?
    })?, f571(v0.clone(), v1, v2, {
        let a2483 = v3;
        let a2484 = 16u64;
        nat_add(a2483, a2484)?
    })?, f571(v0.clone(), v1, v2, {
        let a2485 = v3;
        let a2486 = 20u64;
        nat_add(a2485, a2486)?
    })?, f571(v0.clone(), v1, v2, {
        let a2487 = v3;
        let a2488 = 24u64;
        nat_add(a2487, a2488)?
    })?, f571(v0.clone(), v1, v2, {
        let a2489 = v3;
        let a2490 = 28u64;
        nat_add(a2489, a2490)?
    })?, f571(v0.clone(), v1, v2, {
        let a2491 = v3;
        let a2492 = 32u64;
        nat_add(a2491, a2492)?
    })?, f571(v0.clone(), v1, v2, {
        let a2493 = v3;
        let a2494 = 36u64;
        nat_add(a2493, a2494)?
    })?, f571(v0.clone(), v1, v2, {
        let a2495 = v3;
        let a2496 = 40u64;
        nat_add(a2495, a2496)?
    })?, f571(v0.clone(), v1, v2, {
        let a2497 = v3;
        let a2498 = 44u64;
        nat_add(a2497, a2498)?
    })?, f571(v0.clone(), v1, v2, {
        let a2499 = v3;
        let a2500 = 48u64;
        nat_add(a2499, a2500)?
    })?, f571(v0.clone(), v1, v2, {
        let a2501 = v3;
        let a2502 = 52u64;
        nat_add(a2501, a2502)?
    })?, f571(v0.clone(), v1, v2, {
        let a2503 = v3;
        let a2504 = 56u64;
        nat_add(a2503, a2504)?
    })?, f571(v0, v1, v2, {
        let a2505 = v3;
        let a2506 = 60u64;
        nat_add(a2505, a2506)?
    })?))
}

pub fn f543(p2509: List<Bytes>, v1: Bytes) -> bool {
    let m2508 = p2509;
    {
        let mut m2508 = m2508;
        loop {
            let step = {
                let v0 = m2508;
                let m2507 = v0;
                match m2507.uncons() {
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
                Ok(next) => { m2508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f544(v0: Bytes, v1: u64) -> Bytes {
    let m2513 = {
        let a2510 = v0;
        let a2511 = v1;
        let a2512 = 1u64;
        slice_bytes(a2510, a2511, a2512)
    };
    match m2513 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f545(v0: Fn14, p2518: u64, p2519: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m2517 = (p2518, p2519);
    {
        let mut m2517 = m2517;
        loop {
            let step = {
                let (v1, v2) = m2517;
                let m2514 = v1;
                if m2514 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2514 - 1;
                    let m2516 = {
                        let c2515: Fn14 = v0.clone();
                        c2515.apply(v2.clone())?
                    };
                    match m2516 {
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
                Ok(next) => { m2517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f546(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f572(v1, v0, v2)
}

pub fn f547(p2522: List<u64>, p2523: List<u64>) -> List<u64> {
    let m2521 = (p2522, p2523);
    {
        let mut m2521 = m2521;
        loop {
            let step = {
                let (v0, v1) = m2521;
                let m2520 = v0;
                match m2520.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2521 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f548(v0: List<u64>) -> R<Bytes> {
    let m2524 = v0;
    match m2524.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a2525 = f573(v1)?;
            let a2526 = f548(v2)?;
            Ok(append_bytes(a2525, a2526))
        }
    }
}

pub fn f549(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m2527 = v0;
    match m2527.uncons() {
        None => {
            let a2528 = v2;
            let a2529 = 0u64;
            if nat_eq(a2528, a2529) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a2532 = {
                    let a2530 = v3;
                    let a2531 = v1;
                    nat_mul(a2530, a2531)?
                };
                let a2533 = v2;
                nat_add(a2532, a2533)?
            };
            Ok(List::cons({
                let a2534 = v5;
                let a2535 = f550();
                let a2536 = 0u64;
                nat_rem(a2534, a2535, a2536)
            }, f549(v4, v1, {
                let a2537 = v5;
                let a2538 = f550();
                let a2539 = 0u64;
                nat_quot(a2537, a2538, a2539)
            })?))
        }
    }
}

pub fn f550() -> u64 {
    1000000000u64
}

pub fn f551(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2584 = {
            let a2560 = {
                let a2548 = {
                    let a2542 = {
                        let a2540 = v1;
                        let a2541 = 128u8;
                        fixed_u8::bit_and(a2540, a2541)
                    };
                    let a2543 = 128u8;
                    if equal(a2542, a2543) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2549 = {
                    let a2546 = {
                        let a2544 = v1;
                        let a2545 = 64u8;
                        fixed_u8::bit_and(a2544, a2545)
                    };
                    let a2547 = 64u8;
                    if equal(a2546, a2547) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2548, a2549)?
            };
            let a2561 = {
                let a2558 = {
                    let a2552 = {
                        let a2550 = v1;
                        let a2551 = 32u8;
                        fixed_u8::bit_and(a2550, a2551)
                    };
                    let a2553 = 32u8;
                    if equal(a2552, a2553) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2559 = {
                    let a2556 = {
                        let a2554 = v1;
                        let a2555 = 16u8;
                        fixed_u8::bit_and(a2554, a2555)
                    };
                    let a2557 = 16u8;
                    if equal(a2556, a2557) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2558, a2559)?
            };
            nat_add(a2560, a2561)?
        };
        let a2585 = {
            let a2582 = {
                let a2570 = {
                    let a2564 = {
                        let a2562 = v1;
                        let a2563 = 8u8;
                        fixed_u8::bit_and(a2562, a2563)
                    };
                    let a2565 = 8u8;
                    if equal(a2564, a2565) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2571 = {
                    let a2568 = {
                        let a2566 = v1;
                        let a2567 = 4u8;
                        fixed_u8::bit_and(a2566, a2567)
                    };
                    let a2569 = 4u8;
                    if equal(a2568, a2569) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2570, a2571)?
            };
            let a2583 = {
                let a2580 = {
                    let a2574 = {
                        let a2572 = v1;
                        let a2573 = 2u8;
                        fixed_u8::bit_and(a2572, a2573)
                    };
                    let a2575 = 2u8;
                    if equal(a2574, a2575) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2581 = {
                    let a2578 = {
                        let a2576 = v1;
                        let a2577 = 1u8;
                        fixed_u8::bit_and(a2576, a2577)
                    };
                    let a2579 = 1u8;
                    if equal(a2578, a2579) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2580, a2581)?
            };
            nat_add(a2582, a2583)?
        };
        nat_add(a2584, a2585)?
    };
    let a2592 = f544(f574(), {
        let a2586 = v2;
        let a2587 = 16u64;
        let a2588 = 0u64;
        nat_quot(a2586, a2587, a2588)
    });
    let a2593 = f544(f574(), {
        let a2589 = v2;
        let a2590 = 16u64;
        let a2591 = 0u64;
        nat_rem(a2589, a2590, a2591)
    });
    Ok(append_bytes(a2592, a2593))
}

pub fn f552(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f575(Fn19::F576, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f553(v0: Fn15, p2597: List<List<(Bytes, Bytes)>>, p2598: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2596 = (p2597, p2598);
    {
        let mut m2596 = m2596;
        loop {
            let step = {
                let (v1, v2) = m2596;
                let m2594 = v2;
                match m2594.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2595: Fn15 = v0.clone();
                            c2595.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2596 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f555(v0: Fn16, p2603: u64, p2604: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2602 = (p2603, p2604);
    {
        let mut m2602 = m2602;
        loop {
            let step = {
                let (v1, v2) = m2602;
                let m2599 = v1;
                if m2599 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2599 - 1;
                    let m2601 = {
                        let c2600: Fn16 = v0.clone();
                        c2600.apply(v2.clone())?
                    };
                    match m2601 {
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
                Ok(next) => { m2602 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f556(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f577(v0)
}

pub fn f557(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f578(v0, v1)
}

pub fn f558(v0: u8) -> bool {
    let a2605 = v0;
    let a2606 = 32u8;
    if if if equal(a2605, a2606) {
        true
    } else {
        let a2607 = v0;
        let a2608 = 9u8;
        equal(a2607, a2608)
    } {
        true
    } else {
        let a2609 = v0;
        let a2610 = 10u8;
        equal(a2609, a2610)
    } {
        true
    } else {
        let a2611 = v0;
        let a2612 = 12u8;
        if equal(a2611, a2612) {
            true
        } else {
            let a2613 = v0;
            let a2614 = 13u8;
            equal(a2613, a2614)
        }
    }
}

pub fn f559(v0: Fn17, p2619: u64, p2620: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2618 = (p2619, p2620);
    {
        let mut m2618 = m2618;
        loop {
            let step = {
                let (v1, v2) = m2618;
                let m2615 = v1;
                if m2615 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2615 - 1;
                    let m2617 = {
                        let c2616: Fn17 = v0.clone();
                        c2616.apply(v2.clone())?
                    };
                    match m2617 {
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
                Ok(next) => { m2618 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f560(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f579(v0, v1)
}

pub fn f561(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2665 = {
            let a2641 = {
                let a2629 = {
                    let a2623 = {
                        let a2621 = v1;
                        let a2622 = 128u8;
                        fixed_u8::bit_and(a2621, a2622)
                    };
                    let a2624 = 128u8;
                    if equal(a2623, a2624) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2630 = {
                    let a2627 = {
                        let a2625 = v1;
                        let a2626 = 64u8;
                        fixed_u8::bit_and(a2625, a2626)
                    };
                    let a2628 = 64u8;
                    if equal(a2627, a2628) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2629, a2630)?
            };
            let a2642 = {
                let a2639 = {
                    let a2633 = {
                        let a2631 = v1;
                        let a2632 = 32u8;
                        fixed_u8::bit_and(a2631, a2632)
                    };
                    let a2634 = 32u8;
                    if equal(a2633, a2634) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2640 = {
                    let a2637 = {
                        let a2635 = v1;
                        let a2636 = 16u8;
                        fixed_u8::bit_and(a2635, a2636)
                    };
                    let a2638 = 16u8;
                    if equal(a2637, a2638) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2639, a2640)?
            };
            nat_add(a2641, a2642)?
        };
        let a2666 = {
            let a2663 = {
                let a2651 = {
                    let a2645 = {
                        let a2643 = v1;
                        let a2644 = 8u8;
                        fixed_u8::bit_and(a2643, a2644)
                    };
                    let a2646 = 8u8;
                    if equal(a2645, a2646) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2652 = {
                    let a2649 = {
                        let a2647 = v1;
                        let a2648 = 4u8;
                        fixed_u8::bit_and(a2647, a2648)
                    };
                    let a2650 = 4u8;
                    if equal(a2649, a2650) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2651, a2652)?
            };
            let a2664 = {
                let a2661 = {
                    let a2655 = {
                        let a2653 = v1;
                        let a2654 = 2u8;
                        fixed_u8::bit_and(a2653, a2654)
                    };
                    let a2656 = 2u8;
                    if equal(a2655, a2656) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2662 = {
                    let a2659 = {
                        let a2657 = v1;
                        let a2658 = 1u8;
                        fixed_u8::bit_and(a2657, a2658)
                    };
                    let a2660 = 1u8;
                    if equal(a2659, a2660) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2661, a2662)?
            };
            nat_add(a2663, a2664)?
        };
        nat_add(a2665, a2666)?
    };
    let a2667 = 48u64;
    let a2668 = v2;
    if nat_le(a2667, a2668) {
        let a2669 = v2;
        let a2670 = 57u64;
        Ok(nat_le(a2669, a2670))
    } else {
        Ok(false)
    }
}

pub fn f562(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f458({
        let (_, h2671) = {
            let (h2672, _) = f580(Fn20::F581, {
                let a2674 = {
                    let a2673 = v0.clone();
                    length_list(a2673)
                };
                let a2675 = 1u64;
                nat_add(a2674, a2675)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2672
        };
        h2671
    }))
}

pub fn f563(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f564(v0: Adt1, v1: Bytes) -> R<bool> {
    if f67(v0.clone(), f525(v1.clone())) {
        let m2676 = f95(f64(v0, f525(v1)))?;
        match m2676.uncons() {
            None => {
                Ok(false)
            }
            Some((v2, _)) => {
                let m2677 = f43(v2)?;
                match m2677 {
                    None => {
                        Ok(false)
                    }
                    Some(v4) => {
                        let m2678 = f93(v4, f8(Str::lit("reader")));
                        match m2678 {
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

pub fn f565(p2682: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m2681 = p2682;
    {
        let mut m2681 = m2681;
        loop {
            let step = {
                let v0 = m2681;
                let m2679 = v0;
                match m2679.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f117(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2680 = f93(v3, f8(Str::lit("std")));
                            match m2680 {
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
                Ok(next) => { m2681 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f566(v0: List<Adt2>) -> List<Bytes> {
    let m2683 = v0;
    match m2683.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2684 = f93(v1, f8(Str::lit("std")));
            match m2684 {
                None => {
                    f566(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f566(v2))
                }
            }
        }
    }
}

pub fn f567(v0: List<Adt2>) -> List<Bytes> {
    let m2685 = v0;
    match m2685.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2686 = f92(f93(v1, f8(Str::lit("req"))));
            match m2686 {
                None => {
                    f567(v2)
                }
                Some(v3) => {
                    List::cons(v3, f567(v2))
                }
            }
        }
    }
}

pub fn f568(v0: Fn18, p2690: u64, p2691: Adt25) -> R<Adt25> {
    let m2689 = (p2690, p2691);
    {
        let mut m2689 = m2689;
        loop {
            let step = {
                let (v1, v2) = m2689;
                let m2687 = v1;
                if m2687 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m2687 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c2688: Fn18 = v0.clone();
                        c2688.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2689 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f569(v0: Adt25) -> R<Adt25> {
    f582(v0)
}

pub fn f570(v0: u32, v1: u32) -> u32 {
    let m2694 = {
        let a2692 = f583(v0);
        let a2693 = f583(v1);
        fixed_u64::checked_add(a2692, a2693)
    };
    match m2694 {
        None => {
            0u32
        }
        Some(v2) => {
            f584(v2)
        }
    }
}

pub fn f571(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2697 = {
        let a2695 = v3;
        let a2696 = 8u64;
        nat_add(a2695, a2696)?
    };
    let a2698 = v2;
    if nat_eq(a2697, a2698) {
        Ok(f585({
            let a2701 = {
                let a2699 = v1;
                let a2700 = 8u64;
                nat_mul(a2699, a2700)?
            };
            let a2702 = 4294967296u64;
            let a2703 = 0u64;
            nat_quot(a2701, a2702, a2703)
        }))
    } else {
        let a2706 = {
            let a2704 = v3;
            let a2705 = 4u64;
            nat_add(a2704, a2705)?
        };
        let a2707 = v2;
        if nat_eq(a2706, a2707) {
            Ok(f585({
                let a2710 = {
                    let a2708 = v1;
                    let a2709 = 8u64;
                    nat_mul(a2708, a2709)?
                };
                let a2711 = 4294967296u64;
                let a2712 = 0u64;
                nat_rem(a2710, a2711, a2712)
            }))
        } else {
            Ok(f586(f587(v0.clone(), v1, v3), f587(v0.clone(), v1, {
                let a2713 = v3;
                let a2714 = 1u64;
                nat_add(a2713, a2714)?
            }), f587(v0.clone(), v1, {
                let a2715 = v3;
                let a2716 = 2u64;
                nat_add(a2715, a2716)?
            }), f587(v0, v1, {
                let a2717 = v3;
                let a2718 = 3u64;
                nat_add(a2717, a2718)?
            })))
        }
    }
}

pub fn f572(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2719, _) = v2;
        h2719
    };
    let a2720 = v3;
    let a2721 = 0u64;
    if nat_eq(a2720, a2721) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2722 = v3;
            let a2723 = v1;
            let a2724 = 0u64;
            nat_quot(a2722, a2723, a2724)
        }, {
            let a2729 = f544(v0, {
                let a2725 = v3;
                let a2726 = v1;
                let a2727 = 0u64;
                nat_rem(a2725, a2726, a2727)
            });
            let a2730 = {
                let (_, h2728) = v2;
                h2728
            };
            append_bytes(a2729, a2730)
        }))
    }
}

pub fn f573(v0: u64) -> R<Bytes> {
    let v1: Bytes = f410(v0)?;
    let a2734 = f501({
        let a2732 = 9u64;
        let a2733 = {
            let a2731 = v1.clone();
            length_bytes(a2731)
        };
        nat_sub(a2732, a2733)
    });
    let a2735 = v1;
    Ok(append_bytes(a2734, a2735))
}

pub fn f574() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f575(v0: Fn19, p2739: List<List<(Bytes, Bytes)>>, p2740: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2738 = (p2739, p2740);
    {
        let mut m2738 = m2738;
        loop {
            let step = {
                let (v1, v2) = m2738;
                let m2736 = v2;
                match m2736.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2737: Fn19 = v0.clone();
                            c2737.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2738 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f576(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f577(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2742 = {
        let a2741 = v0.clone();
        length_list(a2741)
    };
    let a2743 = 1u64;
    if nat_le(a2742, a2743) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f588(v0)?))
    }
}

pub fn f578(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2747 = {
        let a2745 = v0.clone();
        let a2746 = {
            let (h2744, _) = v1.clone();
            h2744
        };
        index_bytes(a2745, a2746)
    };
    match m2747 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2749 = {
                    let (h2748, _) = v1.clone();
                    h2748
                };
                let a2750 = 1u64;
                nat_add(a2749, a2750)?
            }, {
                let a2753 = {
                    let (_, h2751) = v1;
                    h2751
                };
                let a2754 = f502(v0, {
                    let (h2752, _) = v1;
                    h2752
                }, v2)?;
                append_bytes(a2753, a2754)
            })))
        }
    }
}

pub fn f579(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2755, _) = v1;
        h2755
    };
    let m2757 = {
        let (_, h2756) = v1;
        h2756
    };
    match m2757 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2760 = {
                let a2758 = v0.clone();
                let a2759 = v2;
                index_bytes(a2758, a2759)
            };
            match m2760 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2765 = {
                        let a2763 = v0;
                        let a2764 = {
                            let a2761 = v2;
                            let a2762 = 1u64;
                            nat_add(a2761, a2762)?
                        };
                        index_bytes(a2763, a2764)
                    };
                    match m2765 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f589(v4)? {
                                f589(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2766 = v2;
                                    let a2767 = 2u64;
                                    nat_add(a2766, a2767)?
                                }, Some({
                                    let a2772 = v3;
                                    let a2773 = f544(f590(), {
                                        let a2770 = {
                                            let a2768 = f591(v4)?;
                                            let a2769 = 16u64;
                                            nat_mul(a2768, a2769)?
                                        };
                                        let a2771 = f591(v5)?;
                                        nat_add(a2770, a2771)?
                                    });
                                    append_bytes(a2772, a2773)
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

pub fn f580(v0: Fn20, p2778: u64, p2779: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2777 = (p2778, p2779);
    {
        let mut m2777 = m2777;
        loop {
            let step = {
                let (v1, v2) = m2777;
                let m2774 = v1;
                if m2774 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2774 - 1;
                    let m2776 = {
                        let c2775: Fn20 = v0.clone();
                        c2775.apply(v2.clone())?
                    };
                    match m2776 {
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
                Ok(next) => { m2777 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f592(v0)
}

pub fn f582(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f593(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h2780) => {
            h2780
        }
    });
    let v2: u32 = f570(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h2781, _, _) => {
            h2781
        }
    }, f570(f594(match v0.clone() {
        Adt25::C0(_, _, _, _, h2782, _, _, _, _, _) => {
            h2782
        }
    }), f570(f595(match v0.clone() {
        Adt25::C0(_, _, _, _, h2783, _, _, _, _, _) => {
            h2783
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2784, _, _, _, _) => {
            h2784
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2785, _, _, _) => {
            h2785
        }
    }), f570(v1, {
        let m2788 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h2787, _) => {
                h2787
            }
        };
        match m2788 {
            Adt24::C0(h2786, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2786
            }
        }
    }))));
    let v3: u32 = f570(f596(match v0.clone() {
        Adt25::C0(h2789, _, _, _, _, _, _, _, _, _) => {
            h2789
        }
    }), f597(match v0.clone() {
        Adt25::C0(h2790, _, _, _, _, _, _, _, _, _) => {
            h2790
        }
    }, match v0.clone() {
        Adt25::C0(_, h2791, _, _, _, _, _, _, _, _) => {
            h2791
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2792, _, _, _, _, _, _, _) => {
            h2792
        }
    }));
    Ok(Adt25::C0(f570(v2, v3), match v0.clone() {
        Adt25::C0(h2793, _, _, _, _, _, _, _, _, _) => {
            h2793
        }
    }, match v0.clone() {
        Adt25::C0(_, h2794, _, _, _, _, _, _, _, _) => {
            h2794
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2795, _, _, _, _, _, _, _) => {
            h2795
        }
    }, f570(match v0.clone() {
        Adt25::C0(_, _, _, h2796, _, _, _, _, _, _) => {
            h2796
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h2797, _, _, _, _, _) => {
            h2797
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2798, _, _, _, _) => {
            h2798
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2799, _, _, _) => {
            h2799
        }
    }, f598(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h2800, _) => {
            h2800
        }
    }), {
        let a2802 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h2801) => {
                h2801
            }
        };
        let a2803 = 1u64;
        nat_add(a2802, a2803)?
    }))
}

pub fn f583(v0: u32) -> u64 {
    let m2805 = {
        let a2804 = v0;
        fixed_u64::convert(i128::from(a2804))
    };
    match m2805 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f584(v0: u64) -> u32 {
    let m2809 = {
        let a2808 = {
            let a2806 = v0;
            let a2807 = 4294967295u64;
            fixed_u64::bit_and(a2806, a2807)
        };
        fixed_u32::convert(i128::from(a2808))
    };
    match m2809 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f585(v0: u64) -> u32 {
    let (_, h2810) = {
        let (_, h2811) = f599(Fn21::F600, 32u64, (v0, (1u32, 0u32)));
        h2811
    };
    h2810
}

pub fn f586(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2816 = f601(v0, 24u32);
    let a2817 = {
        let a2814 = f601(v1, 16u32);
        let a2815 = {
            let a2812 = f601(v2, 8u32);
            let a2813 = v3;
            fixed_u32::bit_or(a2812, a2813)
        };
        fixed_u32::bit_or(a2814, a2815)
    };
    fixed_u32::bit_or(a2816, a2817)
}

pub fn f587(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2818 = v2;
    let a2819 = v1;
    if nat_lt(a2818, a2819) {
        let m2822 = {
            let a2820 = v0;
            let a2821 = v2;
            index_bytes(a2820, a2821)
        };
        match m2822 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2824 = {
                    let a2823 = v3;
                    fixed_u32::convert(i128::from(a2823))
                };
                match m2824 {
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
        let a2825 = v2;
        let a2826 = v1;
        if nat_eq(a2825, a2826) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f588(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f552({
        let (_, h2827) = {
            let (h2828, _) = f602(Fn22::F603, {
                let a2830 = {
                    let a2829 = v0.clone();
                    length_list(a2829)
                };
                let a2831 = 1u64;
                nat_add(a2830, a2831)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2828
        };
        h2827
    }))
}

pub fn f589(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2876 = {
            let a2852 = {
                let a2840 = {
                    let a2834 = {
                        let a2832 = v1;
                        let a2833 = 128u8;
                        fixed_u8::bit_and(a2832, a2833)
                    };
                    let a2835 = 128u8;
                    if equal(a2834, a2835) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2841 = {
                    let a2838 = {
                        let a2836 = v1;
                        let a2837 = 64u8;
                        fixed_u8::bit_and(a2836, a2837)
                    };
                    let a2839 = 64u8;
                    if equal(a2838, a2839) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2840, a2841)?
            };
            let a2853 = {
                let a2850 = {
                    let a2844 = {
                        let a2842 = v1;
                        let a2843 = 32u8;
                        fixed_u8::bit_and(a2842, a2843)
                    };
                    let a2845 = 32u8;
                    if equal(a2844, a2845) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2851 = {
                    let a2848 = {
                        let a2846 = v1;
                        let a2847 = 16u8;
                        fixed_u8::bit_and(a2846, a2847)
                    };
                    let a2849 = 16u8;
                    if equal(a2848, a2849) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2850, a2851)?
            };
            nat_add(a2852, a2853)?
        };
        let a2877 = {
            let a2874 = {
                let a2862 = {
                    let a2856 = {
                        let a2854 = v1;
                        let a2855 = 8u8;
                        fixed_u8::bit_and(a2854, a2855)
                    };
                    let a2857 = 8u8;
                    if equal(a2856, a2857) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2863 = {
                    let a2860 = {
                        let a2858 = v1;
                        let a2859 = 4u8;
                        fixed_u8::bit_and(a2858, a2859)
                    };
                    let a2861 = 4u8;
                    if equal(a2860, a2861) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2862, a2863)?
            };
            let a2875 = {
                let a2872 = {
                    let a2866 = {
                        let a2864 = v1;
                        let a2865 = 2u8;
                        fixed_u8::bit_and(a2864, a2865)
                    };
                    let a2867 = 2u8;
                    if equal(a2866, a2867) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2873 = {
                    let a2870 = {
                        let a2868 = v1;
                        let a2869 = 1u8;
                        fixed_u8::bit_and(a2868, a2869)
                    };
                    let a2871 = 1u8;
                    if equal(a2870, a2871) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2872, a2873)?
            };
            nat_add(a2874, a2875)?
        };
        nat_add(a2876, a2877)?
    };
    let a2878 = 48u64;
    let a2879 = v2;
    if if nat_le(a2878, a2879) {
        let a2880 = v2;
        let a2881 = 57u64;
        nat_le(a2880, a2881)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2882 = 97u64;
        let a2883 = v2;
        if nat_le(a2882, a2883) {
            let a2884 = v2;
            let a2885 = 102u64;
            Ok(nat_le(a2884, a2885))
        } else {
            Ok(false)
        }
    }
}

pub fn f590() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f591(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2930 = {
            let a2906 = {
                let a2894 = {
                    let a2888 = {
                        let a2886 = v1;
                        let a2887 = 128u8;
                        fixed_u8::bit_and(a2886, a2887)
                    };
                    let a2889 = 128u8;
                    if equal(a2888, a2889) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2895 = {
                    let a2892 = {
                        let a2890 = v1;
                        let a2891 = 64u8;
                        fixed_u8::bit_and(a2890, a2891)
                    };
                    let a2893 = 64u8;
                    if equal(a2892, a2893) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2894, a2895)?
            };
            let a2907 = {
                let a2904 = {
                    let a2898 = {
                        let a2896 = v1;
                        let a2897 = 32u8;
                        fixed_u8::bit_and(a2896, a2897)
                    };
                    let a2899 = 32u8;
                    if equal(a2898, a2899) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2905 = {
                    let a2902 = {
                        let a2900 = v1;
                        let a2901 = 16u8;
                        fixed_u8::bit_and(a2900, a2901)
                    };
                    let a2903 = 16u8;
                    if equal(a2902, a2903) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2904, a2905)?
            };
            nat_add(a2906, a2907)?
        };
        let a2931 = {
            let a2928 = {
                let a2916 = {
                    let a2910 = {
                        let a2908 = v1;
                        let a2909 = 8u8;
                        fixed_u8::bit_and(a2908, a2909)
                    };
                    let a2911 = 8u8;
                    if equal(a2910, a2911) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2917 = {
                    let a2914 = {
                        let a2912 = v1;
                        let a2913 = 4u8;
                        fixed_u8::bit_and(a2912, a2913)
                    };
                    let a2915 = 4u8;
                    if equal(a2914, a2915) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2916, a2917)?
            };
            let a2929 = {
                let a2926 = {
                    let a2920 = {
                        let a2918 = v1;
                        let a2919 = 2u8;
                        fixed_u8::bit_and(a2918, a2919)
                    };
                    let a2921 = 2u8;
                    if equal(a2920, a2921) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2927 = {
                    let a2924 = {
                        let a2922 = v1;
                        let a2923 = 1u8;
                        fixed_u8::bit_and(a2922, a2923)
                    };
                    let a2925 = 1u8;
                    if equal(a2924, a2925) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2926, a2927)?
            };
            nat_add(a2928, a2929)?
        };
        nat_add(a2930, a2931)?
    };
    let a2932 = v2;
    let a2933 = 57u64;
    if nat_le(a2932, a2933) {
        let a2934 = v2;
        let a2935 = 48u64;
        Ok(nat_sub(a2934, a2935))
    } else {
        let a2936 = v2;
        let a2937 = 87u64;
        Ok(nat_sub(a2936, a2937))
    }
}

pub fn f592(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2938, _) = v0;
        h2938
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2939) = v0;
        h2939
    };
    let m2940 = v1;
    match m2940.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2941 = v4;
            match m2941.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f604(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f593(v0: u64) -> u32 {
    let a2942 = v0;
    let a2943 = 32u64;
    if nat_lt(a2942, a2943) {
        let a2944 = v0;
        let a2945 = 16u64;
        if nat_lt(a2944, a2945) {
            let a2946 = v0;
            let a2947 = 8u64;
            if nat_lt(a2946, a2947) {
                let a2948 = v0;
                let a2949 = 4u64;
                if nat_lt(a2948, a2949) {
                    let a2950 = v0;
                    let a2951 = 2u64;
                    if nat_lt(a2950, a2951) {
                        let a2952 = v0;
                        let a2953 = 1u64;
                        if nat_lt(a2952, a2953) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2954 = v0;
                        let a2955 = 3u64;
                        if nat_lt(a2954, a2955) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2956 = v0;
                    let a2957 = 6u64;
                    if nat_lt(a2956, a2957) {
                        let a2958 = v0;
                        let a2959 = 5u64;
                        if nat_lt(a2958, a2959) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2960 = v0;
                        let a2961 = 7u64;
                        if nat_lt(a2960, a2961) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2962 = v0;
                let a2963 = 12u64;
                if nat_lt(a2962, a2963) {
                    let a2964 = v0;
                    let a2965 = 10u64;
                    if nat_lt(a2964, a2965) {
                        let a2966 = v0;
                        let a2967 = 9u64;
                        if nat_lt(a2966, a2967) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2968 = v0;
                        let a2969 = 11u64;
                        if nat_lt(a2968, a2969) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2970 = v0;
                    let a2971 = 14u64;
                    if nat_lt(a2970, a2971) {
                        let a2972 = v0;
                        let a2973 = 13u64;
                        if nat_lt(a2972, a2973) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2974 = v0;
                        let a2975 = 15u64;
                        if nat_lt(a2974, a2975) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2976 = v0;
            let a2977 = 24u64;
            if nat_lt(a2976, a2977) {
                let a2978 = v0;
                let a2979 = 20u64;
                if nat_lt(a2978, a2979) {
                    let a2980 = v0;
                    let a2981 = 18u64;
                    if nat_lt(a2980, a2981) {
                        let a2982 = v0;
                        let a2983 = 17u64;
                        if nat_lt(a2982, a2983) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2984 = v0;
                        let a2985 = 19u64;
                        if nat_lt(a2984, a2985) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2986 = v0;
                    let a2987 = 22u64;
                    if nat_lt(a2986, a2987) {
                        let a2988 = v0;
                        let a2989 = 21u64;
                        if nat_lt(a2988, a2989) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2990 = v0;
                        let a2991 = 23u64;
                        if nat_lt(a2990, a2991) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2992 = v0;
                let a2993 = 28u64;
                if nat_lt(a2992, a2993) {
                    let a2994 = v0;
                    let a2995 = 26u64;
                    if nat_lt(a2994, a2995) {
                        let a2996 = v0;
                        let a2997 = 25u64;
                        if nat_lt(a2996, a2997) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2998 = v0;
                        let a2999 = 27u64;
                        if nat_lt(a2998, a2999) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3000 = v0;
                    let a3001 = 30u64;
                    if nat_lt(a3000, a3001) {
                        let a3002 = v0;
                        let a3003 = 29u64;
                        if nat_lt(a3002, a3003) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3004 = v0;
                        let a3005 = 31u64;
                        if nat_lt(a3004, a3005) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3006 = v0;
        let a3007 = 48u64;
        if nat_lt(a3006, a3007) {
            let a3008 = v0;
            let a3009 = 40u64;
            if nat_lt(a3008, a3009) {
                let a3010 = v0;
                let a3011 = 36u64;
                if nat_lt(a3010, a3011) {
                    let a3012 = v0;
                    let a3013 = 34u64;
                    if nat_lt(a3012, a3013) {
                        let a3014 = v0;
                        let a3015 = 33u64;
                        if nat_lt(a3014, a3015) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3016 = v0;
                        let a3017 = 35u64;
                        if nat_lt(a3016, a3017) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3018 = v0;
                    let a3019 = 38u64;
                    if nat_lt(a3018, a3019) {
                        let a3020 = v0;
                        let a3021 = 37u64;
                        if nat_lt(a3020, a3021) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3022 = v0;
                        let a3023 = 39u64;
                        if nat_lt(a3022, a3023) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3024 = v0;
                let a3025 = 44u64;
                if nat_lt(a3024, a3025) {
                    let a3026 = v0;
                    let a3027 = 42u64;
                    if nat_lt(a3026, a3027) {
                        let a3028 = v0;
                        let a3029 = 41u64;
                        if nat_lt(a3028, a3029) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3030 = v0;
                        let a3031 = 43u64;
                        if nat_lt(a3030, a3031) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3032 = v0;
                    let a3033 = 46u64;
                    if nat_lt(a3032, a3033) {
                        let a3034 = v0;
                        let a3035 = 45u64;
                        if nat_lt(a3034, a3035) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3036 = v0;
                        let a3037 = 47u64;
                        if nat_lt(a3036, a3037) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3038 = v0;
            let a3039 = 56u64;
            if nat_lt(a3038, a3039) {
                let a3040 = v0;
                let a3041 = 52u64;
                if nat_lt(a3040, a3041) {
                    let a3042 = v0;
                    let a3043 = 50u64;
                    if nat_lt(a3042, a3043) {
                        let a3044 = v0;
                        let a3045 = 49u64;
                        if nat_lt(a3044, a3045) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3046 = v0;
                        let a3047 = 51u64;
                        if nat_lt(a3046, a3047) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3048 = v0;
                    let a3049 = 54u64;
                    if nat_lt(a3048, a3049) {
                        let a3050 = v0;
                        let a3051 = 53u64;
                        if nat_lt(a3050, a3051) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3052 = v0;
                        let a3053 = 55u64;
                        if nat_lt(a3052, a3053) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3054 = v0;
                let a3055 = 60u64;
                if nat_lt(a3054, a3055) {
                    let a3056 = v0;
                    let a3057 = 58u64;
                    if nat_lt(a3056, a3057) {
                        let a3058 = v0;
                        let a3059 = 57u64;
                        if nat_lt(a3058, a3059) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3060 = v0;
                        let a3061 = 59u64;
                        if nat_lt(a3060, a3061) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3062 = v0;
                    let a3063 = 62u64;
                    if nat_lt(a3062, a3063) {
                        let a3064 = v0;
                        let a3065 = 61u64;
                        if nat_lt(a3064, a3065) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3066 = v0;
                        let a3067 = 63u64;
                        if nat_lt(a3066, a3067) {
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

pub fn f594(v0: u32) -> u32 {
    let a3070 = {
        let a3068 = f605(v0, 6u32, 26u32);
        let a3069 = f605(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3068, a3069)
    };
    let a3071 = f605(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3070, a3071)
}

pub fn f595(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3077 = {
        let a3072 = v0;
        let a3073 = v1;
        fixed_u32::bit_and(a3072, a3073)
    };
    let a3078 = {
        let a3075 = {
            let a3074 = v0;
            fixed_u32::bit_not(a3074)
        };
        let a3076 = v2;
        fixed_u32::bit_and(a3075, a3076)
    };
    fixed_u32::bit_xor(a3077, a3078)
}

pub fn f596(v0: u32) -> u32 {
    let a3081 = {
        let a3079 = f605(v0, 2u32, 30u32);
        let a3080 = f605(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3079, a3080)
    };
    let a3082 = f605(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3081, a3082)
}

pub fn f597(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3091 = {
        let a3087 = {
            let a3083 = v0;
            let a3084 = v1;
            fixed_u32::bit_and(a3083, a3084)
        };
        let a3088 = {
            let a3085 = v0;
            let a3086 = v2;
            fixed_u32::bit_and(a3085, a3086)
        };
        fixed_u32::bit_xor(a3087, a3088)
    };
    let a3092 = {
        let a3089 = v1;
        let a3090 = v2;
        fixed_u32::bit_and(a3089, a3090)
    };
    fixed_u32::bit_xor(a3091, a3092)
}

pub fn f598(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h3093, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3093
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h3094, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3094
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h3095, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3095
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h3096, _, _, _, _, _, _, _, _, _, _, _) => {
            h3096
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h3097, _, _, _, _, _, _, _, _, _, _) => {
            h3097
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h3098, _, _, _, _, _, _, _, _, _) => {
            h3098
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h3099, _, _, _, _, _, _, _, _) => {
            h3099
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h3100, _, _, _, _, _, _, _) => {
            h3100
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h3101, _, _, _, _, _, _) => {
            h3101
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h3102, _, _, _, _, _) => {
            h3102
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h3103, _, _, _, _) => {
            h3103
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3104, _, _, _) => {
            h3104
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3105, _, _) => {
            h3105
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3106, _) => {
            h3106
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3107) => {
            h3107
        }
    }, f570(f606(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3108, _) => {
            h3108
        }
    }), f570(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h3109, _, _, _, _, _, _) => {
            h3109
        }
    }, f570(f607(match v0.clone() {
        Adt24::C0(_, h3110, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3110
        }
    }), match v0 {
        Adt24::C0(h3111, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3111
        }
    }))))
}

pub fn f599(v0: Fn21, p3115: u64, p3116: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3114 = (p3115, p3116);
    {
        let mut m3114 = m3114;
        loop {
            let step = {
                let (v1, v2) = m3114;
                let m3112 = v1;
                if m3112 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3112 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3113: Fn21 = v0.clone();
                        c3113.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3114 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3117, _) = v0;
        h3117
    };
    let v2: u32 = {
        let (h3118, _) = {
            let (_, h3119) = v0;
            h3119
        };
        h3118
    };
    let v3: u32 = {
        let (_, h3120) = {
            let (_, h3121) = v0;
            h3121
        };
        h3120
    };
    ({
        let a3122 = v1;
        let a3123 = 2u64;
        let a3124 = 0u64;
        nat_quot(a3122, a3123, a3124)
    }, (f601(v2, 1u32), {
        let a3128 = {
            let a3125 = v1;
            let a3126 = 2u64;
            let a3127 = 0u64;
            nat_rem(a3125, a3126, a3127)
        };
        let a3129 = 1u64;
        if nat_eq(a3128, a3129) {
            let a3130 = v3;
            let a3131 = v2;
            fixed_u32::bit_or(a3130, a3131)
        } else {
            v3
        }
    }))
}

pub fn f601(v0: u32, v1: u32) -> u32 {
    let m3134 = {
        let a3132 = v0;
        let a3133 = v1;
        fixed_u32::shift_left(a3132, a3133)
    };
    match m3134 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f602(v0: Fn22, p3139: u64, p3140: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3138 = (p3139, p3140);
    {
        let mut m3138 = m3138;
        loop {
            let step = {
                let (v1, v2) = m3138;
                let m3135 = v1;
                if m3135 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3135 - 1;
                    let m3137 = {
                        let c3136: Fn22 = v0.clone();
                        c3136.apply(v2.clone())?
                    };
                    match m3137 {
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
                Ok(next) => { m3138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f603(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f608(v0)
}

pub fn f604(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3145 = {
            let a3143 = {
                let a3141 = v0.clone();
                length_list(a3141)
            };
            let a3144 = {
                let a3142 = v1.clone();
                length_list(a3142)
            };
            nat_add(a3143, a3144)?
        };
        let a3146 = 1u64;
        nat_add(a3145, a3146)?
    };
    Ok(f463({
        let (_, h3147) = {
            let (_, h3148) = {
                let (h3149, _) = f609(Fn23::F610, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3149
            };
            h3148
        };
        h3147
    }))
}

pub fn f605(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3150 = f540(v0, v1);
    let a3151 = f601(v0, v2);
    fixed_u32::bit_or(a3150, a3151)
}

pub fn f606(v0: u32) -> u32 {
    let a3154 = {
        let a3152 = f605(v0, 17u32, 15u32);
        let a3153 = f605(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3152, a3153)
    };
    let a3155 = f540(v0, 10u32);
    fixed_u32::bit_xor(a3154, a3155)
}

pub fn f607(v0: u32) -> u32 {
    let a3158 = {
        let a3156 = f605(v0, 7u32, 25u32);
        let a3157 = f605(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3156, a3157)
    };
    let a3159 = f540(v0, 3u32);
    fixed_u32::bit_xor(a3158, a3159)
}

pub fn f608(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3160, _) = v0;
        h3160
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3161) = v0;
        h3161
    };
    let m3162 = v1;
    match m3162.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3163 = v4;
            match m3163.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f611(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f609(v0: Fn23, p3168: u64, p3169: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3167 = (p3168, p3169);
    {
        let mut m3167 = m3167;
        loop {
            let step = {
                let (v1, v2) = m3167;
                let m3164 = v1;
                if m3164 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3164 - 1;
                    let m3166 = {
                        let c3165: Fn23 = v0.clone();
                        c3165.apply(v2.clone())
                    };
                    match m3166 {
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
                Ok(next) => { m3167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f610(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f612(v0)
}

pub fn f611(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3174 = {
            let a3172 = {
                let a3170 = v0.clone();
                length_list(a3170)
            };
            let a3173 = {
                let a3171 = v1.clone();
                length_list(a3171)
            };
            nat_add(a3172, a3173)?
        };
        let a3175 = 1u64;
        nat_add(a3174, a3175)?
    };
    Ok(f613({
        let (_, h3176) = {
            let (_, h3177) = {
                let (h3178, _) = f614(Fn24::F615, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3178
            };
            h3177
        };
        h3176
    }))
}

pub fn f612(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3179, _) = v0.clone();
        h3179
    };
    let v2: List<(Bytes, ())> = {
        let (h3180, _) = {
            let (_, h3181) = v0.clone();
            h3181
        };
        h3180
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3182) = {
            let (_, h3183) = v0;
            h3183
        };
        h3182
    };
    let m3184 = v1.clone();
    match m3184.uncons() {
        None => {
            let m3185 = v2;
            match m3185.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3186 = v2.clone();
            match m3186.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f616({
                        let (h3187, _) = v8.clone();
                        h3187
                    }, {
                        let (h3188, _) = v6.clone();
                        h3188
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

pub fn f613(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f421(Fn5::F617, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f614(v0: Fn24, p3193: u64, p3194: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3192 = (p3193, p3194);
    {
        let mut m3192 = m3192;
        loop {
            let step = {
                let (v1, v2) = m3192;
                let m3189 = v1;
                if m3189 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3189 - 1;
                    let m3191 = {
                        let c3190: Fn24 = v0.clone();
                        c3190.apply(v2.clone())
                    };
                    match m3191 {
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
                Ok(next) => { m3192 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f615(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f618(v0)
}

pub fn f616(v0: Bytes, v1: Bytes) -> bool {
    let a3199 = {
        let a3195 = v0;
        let a3196 = v1;
        compare_bytes(a3195, a3196)
    };
    let a3200 = {
        let a3197 = Bytes::lit(&[]);
        let a3198 = Bytes::lit(&[0x01]);
        compare_bytes(a3197, a3198)
    };
    equal(a3199, a3200)
}

pub fn f617(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f618(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3201, _) = v0.clone();
        h3201
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3202, _) = {
            let (_, h3203) = v0.clone();
            h3203
        };
        h3202
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3204) = {
            let (_, h3205) = v0;
            h3205
        };
        h3204
    };
    let m3206 = v1.clone();
    match m3206.uncons() {
        None => {
            let m3207 = v2;
            match m3207.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3208 = v2.clone();
            match m3208.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f616({
                        let (h3209, _) = v8.clone();
                        h3209
                    }, {
                        let (h3210, _) = v6.clone();
                        h3210
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
