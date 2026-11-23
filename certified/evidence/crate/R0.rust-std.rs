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
    F84(u8, Bytes),
    F140(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F84(k0, k1) => f84(*k0, k1.clone(), p0),
            Fn0::F140(k0) => f140(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F116(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F116(k0) => f116(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F129,
    F154,
    F188,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F129 => f129(p0, p1),
            Fn2::F154 => f154(p0, p1),
            Fn2::F188 => f188(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F148(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt21)) -> R<(u64, Adt21)> {
        match self {
            Fn3::F148(k0, k1, k2) => f148(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F187(Bytes),
    F252(Bytes),
    F295(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F187(k0) => Ok(f187(k0.clone(), p0)),
            Fn4::F252(k0) => f252(k0.clone(), p0),
            Fn4::F295(k0) => f295(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F203(Bytes),
    F260(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F203(k0) => f203(k0.clone(), p0),
            Fn5::F260(k0, k1) => Ok(f260(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F205,
    F326,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F205 => f205(p0, p1),
            Fn6::F326 => f326(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F208,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F208 => f208(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F211,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt23) -> R<Adt23> {
        match self {
            Fn8::F211 => f211(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F216,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn9::F216 => f216(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F219,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn10::F219 => f219(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F221,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn11::F221 => f221(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F224,
}

impl Fn12 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn12::F224 => f224(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F226(Bytes),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn13::F226(k0) => f226(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F247,
}

impl Fn14 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn14::F247 => f247(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F250,
}

impl Fn15 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn15::F250 => f250(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F255(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F255(k0) => f255(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F264,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F264 => f264(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F266,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F266 => f266(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F274,
}

impl Fn19 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn19::F274 => f274(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F286,
}

impl Fn20 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn20::F286 => f286(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F292,
}

impl Fn21 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn21::F292 => f292(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F309,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F309 => f309(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F312(Bytes),
}

impl Fn23 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn23::F312(k0) => f312(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F315,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn24::F315 => f315(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F324,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F324 => f324(p0),
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
                if f26(v2.clone(), f8(Str::lit("build"))) {
                    Ok(f28(v1))
                } else {
                    if f26(v2.clone(), f8(Str::lit("publish"))) {
                        Ok(Adt11::C3(List::cons((f29(), f30()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f31()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f32()), List::<(Bytes, Bytes)>::nil()))), f8(Str::lit("published .github/workflows/pages.yml, .github/workflows/operate.yml and .github/CODEOWNERS\u{a}"))))
                    } else {
                        let m128 = v3;
                        match m128.uncons() {
                            None => {
                                Ok(Adt11::C4(f25()))
                            }
                            Some((v4, v5)) => {
                                if f26(v2.clone(), f8(Str::lit("ingest-index"))) {
                                    f33(v1, v4)
                                } else {
                                    if f26(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                        f34(v1, v4)
                                    } else {
                                        if f26(v2, f8(Str::lit("propose-encoding"))) {
                                            let m129 = v5;
                                            match m129.uncons() {
                                                None => {
                                                    Ok(Adt11::C4(f25()))
                                                }
                                                Some((v6, v7)) => {
                                                    f35(v1, v4, v6, v7)
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
    let v1: List<Bytes> = f36(v0.clone(), 47u8)?;
    let v2: Bytes = f37(v1);
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
        Ok(f38(v0.clone(), 0u64, {
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
    f8(Str::lit("usage: verdant_studio check | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f26(v0: Bytes, v1: Bytes) -> bool {
    let a152 = v0;
    let a153 = v1;
    equal(a152, a153)
}

pub fn f27(v0: Adt1) -> R<Adt11> {
    let m154 = f39(v0.clone(), f40());
    match m154.uncons() {
        None => {
            let m155 = f41(v0.clone(), f40())?;
            match m155.uncons() {
                None => {
                    let m158 = f42(match v0.clone() {
                        Adt1::C0(_, h156, _) => {
                            h156
                        }
                    }, f43(f44(), f45(match v0.clone() {
                        Adt1::C0(_, h157, _) => {
                            h157
                        }
                    }, false)));
                    match m158.uncons() {
                        None => {
                            let v7: (bool, Bytes) = f46(Adt13::C0(Adt12::C0(f47(f48(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f49(v0.clone())?, f50(f51(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f47(f48(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f50(f51(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f51(v0.clone(), f8(Str::lit("model/admissions.json")))?, f51(v0.clone(), f8(Str::lit("model/strata.json")))?, f52(f51(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f45(match v0 {
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

pub fn f28(v0: Adt1) -> Adt11 {
    if f53(f54(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h163) => {
            h163
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f55(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f53(f54(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h164) => {
                h164
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f55(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f29() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f30() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            ~/.cargo/registry\u{a}            ~/.cargo/bin\u{a}            target\u{a}            ~/.npm\u{a}          key: studio-${{ hashFiles('certified/result.json', 'studio/lexlean.lock') }}\u{a}      - name: replay the certified bundle, then build its command\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x ~/.cargo/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root ~/.cargo\u{a}          ~/.cargo/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          git diff --exit-code\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f31() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"ingest-authority target/input/binding.json\"'\u{a}        required: true\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: operate\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            ~/.cargo/registry\u{a}            ~/.cargo/bin\u{a}            target\u{a}            ~/.npm\u{a}          key: studio-${{ hashFiles('certified/result.json', 'studio/lexlean.lock') }}\u{a}      - name: replay the certified bundle, then build its command\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x ~/.cargo/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root ~/.cargo\u{a}          ~/.cargo/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check and build through the API\u{a}        env:\u{a}          ARGS: ${{ inputs.args }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}          if [ -n \"$INPUT\" ]; then mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"; fi\u{a}          target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio check\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          git push origin \"HEAD:refs/heads/${branch}\"\u{a}          gh pr create --head \"$branch\" --base main --title \"verdant_studio $ARGS\" --body-file target/operation.txt\u{a}"))
}

pub fn f32() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f33(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m165 = f39(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m165.uncons() {
        None => {
            let v4: Bytes = f48(v0.clone(), v1);
            let v5: List<List<Bytes>> = f56(v4.clone())?;
            if f53(f57(f58(v5.clone()), f59())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f60(Str::lit("id"), v6.clone())?, List::cons(f60(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f60(Str::lit("artifact"), f61())?, List::cons(f60(Str::lit("checksum"), f62(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f61(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f63(f48(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f64(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f65(f66(f67(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f34(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m166 = f39(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m166.uncons() {
        None => {
            let m167 = f68(v0.clone(), v1.clone())?;
            match m167 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f53(f26(f69(v4.clone(), f8(Str::lit("schema"))), f70())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f69(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m168 = f71(v4.clone(), f8(Str::lit("source")));
                            match m168 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f72(f71(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f73(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f73(v9.clone(), f8(Str::lit("binding.json"))), f48(v0.clone(), v1));
                        let m169 = v8;
                        match m169 {
                            None => {
                                f74(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f53(f26(f69(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f74(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f73(v9, f75(f69(v7.clone(), f8(Str::lit("url"))))?);
                                    if f53(f54(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h170) => {
                                            h170
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f69(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m171 = f39(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m171.uncons() {
                                            None => {
                                                if f26(f62(f48(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f74(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f35(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m172 = f39(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil()))))));
    match m172.uncons() {
        None => {
            let m173 = f76(v3);
            match m173 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v6) => {
                    let m174 = f68(v0.clone(), v1.clone())?;
                    match m174 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                        }
                        Some(v7) => {
                            let v8: Bytes = {
                                let a175 = f8(Str::lit("Catalog."));
                                let a176 = f77(v2.clone())?;
                                append_bytes(a175, a176)
                            };
                            let v9: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f77(v2.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                            let m177 = f78(v7, v8.clone(), f77(v2.clone())?)?;
                            match m177 {
                                Ok(v11) => {
                                    Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f77(v2)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), f48(v0.clone(), v1)), List::cons((v9.clone(), v11), List::cons((f8(Str::lit("model/admissions.json")), f79(Adt14::C3(std::rc::Rc::new(f80(f81(f51(v0.clone(), f8(Str::lit("model/admissions.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v8.clone())), std::rc::Rc::new(v6)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f79(Adt14::C3(std::rc::Rc::new(f80(f81(f51(v0.clone(), f8(Str::lit("model/strata.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                        let a181 = f82(f48(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f38(v9.clone(), 4u64, {
                                            let a179 = {
                                                let a178 = v9;
                                                length_bytes(a178)
                                            };
                                            let a180 = 4u64;
                                            nat_sub(a179, a180)
                                        }))?;
                                        let a182 = Bytes::lit(&[0x0a]);
                                        append_bytes(a181, a182)
                                    }), List::cons(f64(v0, List::cons(v8.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v8, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f36(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a183 = v0.clone();
        length_bytes(a183)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h184, _) = f83(Fn0::F84(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h184
    };
    let v4: u64 = {
        let (h185, _) = {
            let (_, h186) = v3.clone();
            h186
        };
        h185
    };
    Ok(f85(List::cons(f38(v0, v4, {
        let a187 = v2;
        let a188 = v4;
        nat_sub(a187, a188)
    }), {
        let (_, h189) = {
            let (_, h190) = v3;
            h190
        };
        h189
    })))
}

pub fn f37(p194: List<Bytes>) -> Bytes {
    let m193 = p194;
    {
        let mut m193 = m193;
        loop {
            let step = {
                let v0 = m193;
                let m191 = v0;
                match m191.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m192 = v2.clone();
                        match m192.uncons() {
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
                Ok(next) => { m193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m198 = {
        let a195 = v0;
        let a196 = v1;
        let a197 = v2;
        slice_bytes(a195, a196, a197)
    };
    match m198 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f39(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m199 = v1;
    match m199.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m201 = f86(match v0.clone() {
                Adt1::C0(h200, _, _) => {
                    h200
                }
            }, v2.clone());
            match m201 {
                None => {
                    List::cons(v2, f39(v0, v3))
                }
                Some(_) => {
                    f39(v0, v3)
                }
            }
        }
    }
}

pub fn f40() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f61(), List::<Bytes>::nil())))))))
}

pub fn f41(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m202 = v1;
    match m202.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f87(v0.clone(), v2.clone()) {
                f88(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m203 = f68(v0.clone(), v2.clone())?;
                match m203 {
                    None => {
                        Ok(List::cons(v2, f41(v0, v3)?))
                    }
                    Some(_) => {
                        f41(v0, v3)
                    }
                }
            } else {
                f41(v0, v3)
            }
        }
    }
}

pub fn f42(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m204 = v1;
    match m204.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f89(v0.clone(), v2.clone()) {
                f42(v0, v3)
            } else {
                List::cons(v2, f42(v0, v3))
            }
        }
    }
}

pub fn f43(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m205 = v0;
    match m205.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f43(v3, v1))
        }
    }
}

pub fn f44() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f45(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m206 = v0;
    match m206.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f43(f90({
                let (h207, _) = v2;
                h207
            }, {
                let (_, h208) = v2;
                h208
            }, v1), f45(v3, v1))
        }
    }
}

pub fn f46(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h209, _, _, _, _) => {
            h209
        }
    };
    let v2: Adt15 = f91(v1.clone())?;
    let v3: List<Bytes> = f92(v1.clone());
    let v4: List<Bytes> = f50(match v0.clone() {
        Adt13::C0(_, _, h210, _, _) => {
            h210
        }
    }, f8(Str::lit("path")));
    let m212 = f93(v2, List::<u64>::nil(), f94(v1.clone(), match v0.clone() {
        Adt13::C0(_, h211, _, _, _) => {
            h211
        }
    }, 0u64)?);
    match m212 {
        None => {
            if f96(f97(match v0.clone() {
                Adt13::C0(_, _, _, _, h214) => {
                    h214
                }
            }, f43(match v0.clone() {
                Adt13::C0(_, _, _, _, h215) => {
                    h215
                }
            }, v4.clone()))?, f97(v4.clone(), f43(match v0.clone() {
                Adt13::C0(_, _, _, _, h216) => {
                    h216
                }
            }, v4))?, f98(match v0.clone() {
                Adt13::C0(_, _, h217, _, _) => {
                    h217
                }
            })) {
                let m221 = f99(f97(match v0.clone() {
                    Adt13::C0(_, _, _, h218, _) => {
                        h218
                    }
                }, f43(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h219, _) => {
                        h219
                    }
                }))?, f97(v3.clone(), f43(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h220, _) => {
                        h220
                    }
                }))?);
                match m221 {
                    None => {
                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f65(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f95(f43(v3, match v0 {
                            Adt13::C0(_, _, _, h222, _) => {
                                h222
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f95(match v1 {
                Adt12::C0(_, _, _, _, h213) => {
                    h213
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f47(v0: Bytes) -> R<List<Bytes>> {
    f100(f101(v0)?, f8(Str::lit("id = \"")))
}

pub fn f48(v0: Adt1, v1: Bytes) -> Bytes {
    let m224 = f86(match v0 {
        Adt1::C0(h223, _, _) => {
            h223
        }
    }, v1);
    match m224 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m225 = v2;
            match m225 {
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

pub fn f49(v0: Adt1) -> R<List<Bytes>> {
    let m226 = f56(f48(v0, f61()))?;
    match m226.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f66(v2))
        }
    }
}

pub fn f50(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m227 = v0;
    match m227.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f69(v2, v1.clone()), f50(v3, v1))
        }
    }
}

pub fn f51(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f102(f68(v0, v1)?))
}

pub fn f52(v0: List<Adt14>) -> List<Bytes> {
    let m228 = v0;
    match m228.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m229 = f72(Some(v1));
            match m229 {
                None => {
                    f52(v2)
                }
                Some(v3) => {
                    List::cons(v3, f52(v2))
                }
            }
        }
    }
}

pub fn f53(v0: bool) -> bool {
    !v0
}

pub fn f54(v0: Bytes, p232: List<Bytes>) -> bool {
    let m231 = p232;
    {
        let mut m231 = m231;
        loop {
            let step = {
                let v1 = m231;
                let m230 = v1;
                match m230.uncons() {
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
                Ok(next) => { m231 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f56(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt16 = f103(v0.clone(), 0u64, {
        let a233 = v0;
        length_bytes(a233)
    }, Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt16 = {
        let a236 = {
            let a235 = match v1.clone() {
                Adt16::C0(_, _, h234, _, _) => {
                    h234
                }
            };
            length_bytes(a235)
        };
        let a237 = 0u64;
        if if nat_eq(a236, a237) {
            let m239 = match v1.clone() {
                Adt16::C0(_, _, _, h238, _) => {
                    h238
                }
            };
            match m239.uncons() {
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
            f104(v1)
        }
    };
    Ok(f105(match v4 {
        Adt16::C0(_, _, _, _, h240) => {
            h240
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f57(p245: List<Bytes>, p246: List<Bytes>) -> bool {
    let m244 = (p245, p246);
    {
        let mut m244 = m244;
        loop {
            let step = {
                let (v0, v1) = m244;
                let m241 = v0;
                match m241.uncons() {
                    None => {
                        let m242 = v1;
                        match m242.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m243 = v1;
                        match m243.uncons() {
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
                Ok(next) => { m244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: List<List<Bytes>>) -> List<Bytes> {
    let m247 = v0;
    match m247.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f59() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f60(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f106(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f61() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f62(v0: Bytes) -> R<Bytes> {
    Ok(f107(f108(v0)?))
}

pub fn f63(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f109(f110(f101(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f111(f112(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f64(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f79(Adt14::C3(std::rc::Rc::new(f113(f114(f43(v1, f52(f51(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f65(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m248 = f115(Fn1::F116(v1), None::<Bytes>, v0);
    match m248 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f66(v0: List<List<Bytes>>) -> List<Bytes> {
    let m249 = v0;
    match m249.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m250 = v1;
            match m250.uncons() {
                None => {
                    f66(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f66(v2))
                }
            }
        }
    }
}

pub fn f67(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m251 = v0;
    match m251.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f68(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f117(f48(v0, v1))
}

pub fn f69(v0: Adt14, v1: Bytes) -> Bytes {
    let m252 = f72(f71(v0, v1));
    match m252 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f70() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f71(p258: Adt14, v1: Bytes) -> Option<Adt14> {
    let m257 = p258;
    {
        let mut m257 = m257;
        loop {
            let step = {
                let v0 = m257;
                let m253 = v0;
                match m253 {
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
                    Adt14::C4(r254) => {
                        let v5 = (*r254).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r255, r256) => {
                        let v3 = (*r255).clone();
                        let v4 = (*r256).clone();
                        if f26(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m257 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Option<Adt14>) -> Option<Bytes> {
    let m259 = v0;
    match m259 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m260 = v1;
            match m260 {
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

pub fn f73(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f74(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f63(f48(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f118(v1, v4)?)?), List::cons(f64(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f75(v0: Bytes) -> R<Bytes> {
    Ok(f37(f36(v0, 47u8)?))
}

pub fn f76(v0: List<Bytes>) -> Option<Adt14> {
    let m261 = v0;
    match m261.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f26(v1.clone(), f8(Str::lit("req"))) {
                let m262 = v2;
                match m262.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f26(v1.clone(), f8(Str::lit("std"))) {
                    let m263 = v2;
                    match m263.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m264 = v6;
                                match m264.uncons() {
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
                        let m265 = v2;
                        match m265.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m266 = v10;
                                match m266.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f113(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f77(v0: Bytes) -> R<Bytes> {
    f119(v0.clone(), 0u64, {
        let a267 = v0;
        length_bytes(a267)
    }, true, Bytes::lit(&[]))
}

pub fn f78(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m268 = f120(4096u64, f121(f122(f123(f71(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt17::C0(v2, v0), List::<Adt17>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m268 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f112(f124(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f79(v0: Adt14) -> R<Bytes> {
    let a269 = f125(v0)?;
    let a270 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a269, a270))
}

pub fn f80(v0: List<Adt14>) -> Adt14 {
    let m271 = v0;
    match m271.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f80(v2)))
        }
    }
}

pub fn f81(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    let m272 = v0;
    match m272.uncons() {
        None => {
            List::cons(v1, List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            List::cons(v2, f81(v3, v1))
        }
    }
}

pub fn f82(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f112(f126(f101(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f83(v0: Fn0, p277: u64, p278: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m276 = (p277, p278);
    {
        let mut m276 = m276;
        loop {
            let step = {
                let (v1, v2) = m276;
                let m273 = v1;
                if m273 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m273 - 1;
                    let m275 = {
                        let c274: Fn0 = v0.clone();
                        c274.apply(v2.clone())?
                    };
                    match m275 {
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
                Ok(next) => { m276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f127(v1, v0, v2)
}

pub fn f85(v0: List<Bytes>) -> List<Bytes> {
    f128(Fn2::F129, List::<Bytes>::nil(), v0)
}

pub fn f86(p283: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m282 = p283;
    {
        let mut m282 = m282;
        loop {
            let step = {
                let v0 = m282;
                let m279 = v0;
                match m279.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h280, _) = v2;
                            h280
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h281) = v2;
                                h281
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m282 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Adt1, v1: Bytes) -> bool {
    let m285 = f86(match v0 {
        Adt1::C0(h284, _, _) => {
            h284
        }
    }, v1);
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

pub fn f88(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a287 = v0.clone();
        length_bytes(a287)
    };
    let v3: u64 = {
        let a288 = v1.clone();
        length_bytes(a288)
    };
    let a289 = v3;
    let a290 = v2;
    if nat_le(a289, a290) {
        let m296 = {
            let a293 = v0;
            let a294 = {
                let a291 = v2;
                let a292 = v3;
                nat_sub(a291, a292)
            };
            let a295 = v3;
            slice_bytes(a293, a294, a295)
        };
        match m296 {
            None => {
                false
            }
            Some(v4) => {
                let a297 = v4;
                let a298 = v1;
                equal(a297, a298)
            }
        }
    } else {
        false
    }
}

pub fn f89(p302: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m301 = p302;
    {
        let mut m301 = m301;
        loop {
            let step = {
                let v0 = m301;
                let m299 = v0;
                match m299.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h300, _) = v2;
                            h300
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m301 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m303 = v1;
    match m303.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f130(v3.clone())
            } else {
                f53(f130(v3.clone()))
            } {
                List::cons(f73(v0.clone(), v3), f90(v0, v4, v2))
            } else {
                f90(v0, v4, v2)
            }
        }
    }
}

pub fn f91(v0: Adt12) -> R<Adt15> {
    Ok(Adt15::C0(f131(match v0.clone() {
        Adt12::C0(h304, _, _, _, _) => {
            h304
        }
    }, 0u64)?, f131(match v0.clone() {
        Adt12::C0(_, h305, _, _, _) => {
            h305
        }
    }, 0u64)?, f131(match v0.clone() {
        Adt12::C0(_, _, h306, _, _) => {
            h306
        }
    }, 0u64)?, f131(match v0 {
        Adt12::C0(_, _, _, h307, _) => {
            h307
        }
    }, 0u64)?))
}

pub fn f92(v0: Adt12) -> List<Bytes> {
    f43(match v0.clone() {
        Adt12::C0(h308, _, _, _, _) => {
            h308
        }
    }, f43(match v0.clone() {
        Adt12::C0(_, _, h309, _, _) => {
            h309
        }
    }, f43(match v0.clone() {
        Adt12::C0(_, _, _, h310, _) => {
            h310
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h311) => {
            h311
        }
    })))
}

pub fn f93(v0: Adt15, p317: List<u64>, p318: List<Adt18>) -> Option<u64> {
    let m316 = (p317, p318);
    {
        let mut m316 = m316;
        loop {
            let step = {
                let (v1, v2) = m316;
                let m312 = v2;
                match m312.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f132(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h313) => {
                                h313
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h314, _) => {
                                    h314
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h315, _) => {
                                    h315
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m319 = v1;
    match m319.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f133(v0.clone(), v3)?), f94(v0, v4, {
                let a320 = v2;
                let a321 = 1u64;
                nat_add(a320, a321)?
            })?))
        }
    }
}

pub fn f95(p325: List<Bytes>, p326: u64) -> Bytes {
    let m324 = (p325, p326);
    {
        let mut m324 = m324;
        loop {
            let step = {
                let (v0, v1) = m324;
                let m322 = v0;
                match m322.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m323 = v1;
                        if m323 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m323 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f134(v0.clone(), v1.clone()) {
        if f134(v1, v0) {
            f135(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f97(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m327 = v0;
    match m327.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f136(v2, v1.clone())?, f97(v3, v1)?))
        }
    }
}

pub fn f98(v0: List<Adt14>) -> List<u64> {
    let m328 = v0;
    match m328.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f137(v1), f98(v2))
        }
    }
}

pub fn f99(p331: List<u64>, v1: List<u64>) -> Option<u64> {
    let m330 = p331;
    {
        let mut m330 = m330;
        loop {
            let step = {
                let v0 = m330;
                let m329 = v0;
                match m329.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f138(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f100(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m332 = v0;
    match m332.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m333 = f139(v2, v1.clone())?;
            match m333 {
                None => {
                    f100(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f100(v3, v1)?))
                }
            }
        }
    }
}

pub fn f101(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a334 = v0.clone();
        length_bytes(a334)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h335, _) = f83(Fn0::F140(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h335
    };
    let v3: u64 = {
        let (h336, _) = {
            let (_, h337) = v2.clone();
            h337
        };
        h336
    };
    let v4: List<Bytes> = {
        let (_, h338) = {
            let (_, h339) = v2;
            h339
        };
        h338
    };
    let a340 = v3;
    let a341 = v1;
    if nat_lt(a340, a341) {
        Ok(f85(List::cons(f38(v0, v3, {
            let a342 = v1;
            let a343 = v3;
            nat_sub(a342, a343)
        }), v4)))
    } else {
        Ok(f85(v4))
    }
}

pub fn f102(v0: Option<Adt14>) -> List<Adt14> {
    let m344 = v0;
    match m344 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f141(v1)
        }
    }
}

pub fn f103(v0: Bytes, p402: u64, p403: u64, p404: Adt16) -> R<Adt16> {
    let m401 = (p402, (p403, p404));
    {
        let mut m401 = m401;
        loop {
            let step = {
                let (v1, (v2, v3)) = m401;
                let m345 = v2;
                if m345 == 0 {
                    Err::<(u64, (u64, Adt16)), R<Adt16>>(Ok(v3))
                } else {
                    let v4 = m345 - 1;
                    let v5: u64 = f142(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt16::C0(_, h346, _, _, _) => {
                            h346
                        }
                    } {
                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                            let a347 = v1;
                            let a348 = 1u64;
                            nat_add(a347, a348)?
                        }, (v4, Adt16::C0(true, false, match v3.clone() {
                            Adt16::C0(_, _, h349, _, _) => {
                                h349
                            }
                        }, match v3.clone() {
                            Adt16::C0(_, _, _, h350, _) => {
                                h350
                            }
                        }, match v3 {
                            Adt16::C0(_, _, _, _, h351) => {
                                h351
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt16::C0(h352, _, _, _, _) => {
                                h352
                            }
                        } {
                            let a353 = v5;
                            let a354 = 34u64;
                            if nat_eq(a353, a354) {
                                let a357 = f142(v0.clone(), {
                                    let a355 = v1;
                                    let a356 = 1u64;
                                    nat_add(a355, a356)?
                                })?;
                                let a358 = 34u64;
                                if nat_eq(a357, a358) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a359 = v1;
                                        let a360 = 1u64;
                                        nat_add(a359, a360)?
                                    }, (v4, Adt16::C0(true, true, f143(match v3.clone() {
                                        Adt16::C0(_, _, h361, _, _) => {
                                            h361
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt16::C0(_, _, _, h362, _) => {
                                            h362
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h363) => {
                                            h363
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a364 = v1;
                                        let a365 = 1u64;
                                        nat_add(a364, a365)?
                                    }, (v4, Adt16::C0(false, false, match v3.clone() {
                                        Adt16::C0(_, _, h366, _, _) => {
                                            h366
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h367, _) => {
                                            h367
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h368) => {
                                            h368
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a369 = v1;
                                    let a370 = 1u64;
                                    nat_add(a369, a370)?
                                }, (v4, Adt16::C0(true, false, f143(match v3.clone() {
                                    Adt16::C0(_, _, h371, _, _) => {
                                        h371
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt16::C0(_, _, _, h372, _) => {
                                        h372
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h373) => {
                                        h373
                                    }
                                }))))
                            }
                        } else {
                            let a374 = v5;
                            let a375 = 34u64;
                            if nat_eq(a374, a375) {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a376 = v1;
                                    let a377 = 1u64;
                                    nat_add(a376, a377)?
                                }, (v4, Adt16::C0(true, false, match v3.clone() {
                                    Adt16::C0(_, _, h378, _, _) => {
                                        h378
                                    }
                                }, match v3.clone() {
                                    Adt16::C0(_, _, _, h379, _) => {
                                        h379
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h380) => {
                                        h380
                                    }
                                }))))
                            } else {
                                let a381 = v5;
                                let a382 = 44u64;
                                if nat_eq(a381, a382) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a383 = v1;
                                        let a384 = 1u64;
                                        nat_add(a383, a384)?
                                    }, (v4, Adt16::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt16::C0(_, _, h385, _, _) => {
                                            h385
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h386, _) => {
                                            h386
                                        }
                                    }), match v3 {
                                        Adt16::C0(_, _, _, _, h387) => {
                                            h387
                                        }
                                    }))))
                                } else {
                                    let a388 = v5;
                                    let a389 = 13u64;
                                    if nat_eq(a388, a389) {
                                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                            let a390 = v1;
                                            let a391 = 1u64;
                                            nat_add(a390, a391)?
                                        }, (v4, v3)))
                                    } else {
                                        let a392 = v5;
                                        let a393 = 10u64;
                                        if nat_eq(a392, a393) {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a394 = v1;
                                                let a395 = 1u64;
                                                nat_add(a394, a395)?
                                            }, (v4, f104(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a396 = v1;
                                                let a397 = 1u64;
                                                nat_add(a396, a397)?
                                            }, (v4, Adt16::C0(false, false, f143(match v3.clone() {
                                                Adt16::C0(_, _, h398, _, _) => {
                                                    h398
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt16::C0(_, _, _, h399, _) => {
                                                    h399
                                                }
                                            }, match v3 {
                                                Adt16::C0(_, _, _, _, h400) => {
                                                    h400
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
                Ok(next) => { m401 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104(v0: Adt16) -> Adt16 {
    Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f144(List::cons(match v0.clone() {
        Adt16::C0(_, _, h405, _, _) => {
            h405
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h406, _) => {
            h406
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt16::C0(_, _, _, _, h407) => {
            h407
        }
    }))
}

pub fn f105(p410: List<List<Bytes>>, p411: List<List<Bytes>>) -> List<List<Bytes>> {
    let m409 = (p410, p411);
    {
        let mut m409 = m409;
        loop {
            let step = {
                let (v0, v1) = m409;
                let m408 = v0;
                match m408.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f145(v0.clone(), 0u64, {
        let a412 = v0;
        length_bytes(a412)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f107(v0: Adt21) -> Bytes {
    let a433 = f146(match v0.clone() {
        Adt21::C0(h413, _, _, _, _, _, _, _) => {
            h413
        }
    });
    let a434 = {
        let a431 = f146(match v0.clone() {
            Adt21::C0(_, h414, _, _, _, _, _, _) => {
                h414
            }
        });
        let a432 = {
            let a429 = f146(match v0.clone() {
                Adt21::C0(_, _, h415, _, _, _, _, _) => {
                    h415
                }
            });
            let a430 = {
                let a427 = f146(match v0.clone() {
                    Adt21::C0(_, _, _, h416, _, _, _, _) => {
                        h416
                    }
                });
                let a428 = {
                    let a425 = f146(match v0.clone() {
                        Adt21::C0(_, _, _, _, h417, _, _, _) => {
                            h417
                        }
                    });
                    let a426 = {
                        let a423 = f146(match v0.clone() {
                            Adt21::C0(_, _, _, _, _, h418, _, _) => {
                                h418
                            }
                        });
                        let a424 = {
                            let a421 = f146(match v0.clone() {
                                Adt21::C0(_, _, _, _, _, _, h419, _) => {
                                    h419
                                }
                            });
                            let a422 = f146(match v0 {
                                Adt21::C0(_, _, _, _, _, _, _, h420) => {
                                    h420
                                }
                            });
                            append_bytes(a421, a422)
                        };
                        append_bytes(a423, a424)
                    };
                    append_bytes(a425, a426)
                };
                append_bytes(a427, a428)
            };
            append_bytes(a429, a430)
        };
        append_bytes(a431, a432)
    };
    append_bytes(a433, a434)
}

pub fn f108(v0: Bytes) -> R<Adt21> {
    let v1: u64 = {
        let a435 = v0.clone();
        length_bytes(a435)
    };
    let v2: u64 = {
        let a441 = {
            let a438 = {
                let a436 = v1;
                let a437 = 72u64;
                nat_add(a436, a437)?
            };
            let a439 = 64u64;
            let a440 = 0u64;
            nat_quot(a438, a439, a440)
        };
        let a442 = 64u64;
        nat_mul(a441, a442)?
    };
    let (_, h443) = f147(Fn3::F148(v0, v1, v2), {
        let a444 = v2;
        let a445 = 64u64;
        let a446 = 0u64;
        nat_quot(a444, a445, a446)
    }, (0u64, f149()))?;
    Ok(h443)
}

pub fn f109(p449: List<List<Bytes>>, v1: Bytes, p450: List<Bytes>) -> R<List<Bytes>> {
    let m448 = (p449, p450);
    {
        let mut m448 = m448;
        loop {
            let step = {
                let (v0, v2) = m448;
                let m447 = v0;
                match m447.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f150(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f112(f151(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(p453: List<Bytes>, p454: List<Bytes>, p455: List<List<Bytes>>) -> List<List<Bytes>> {
    let m452 = (p453, (p454, p455));
    {
        let mut m452 = m452;
        loop {
            let step = {
                let (v0, (v1, v2)) = m452;
                let m451 = v0;
                match m451.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f124(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f26(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f124(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: Bytes) -> R<Bytes> {
    Ok(f38(v0.clone(), 0u64, f152(v0)?))
}

pub fn f112(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m456 = v0;
    match m456.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m457 = v3.clone();
            match m457.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a460 = v2;
                    let a461 = {
                        let a458 = v1.clone();
                        let a459 = f112(v3, v1);
                        append_bytes(a458, a459)
                    };
                    append_bytes(a460, a461)
                }
            }
        }
    }
}

pub fn f113(v0: List<Bytes>) -> Adt14 {
    let m462 = v0;
    match m462.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f113(v2)))
        }
    }
}

pub fn f114(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f153(f128(Fn2::F154, List::<Bytes>::nil(), f155(v0)?)))
}

pub fn f115(v0: Fn1, p466: Option<Bytes>, p467: List<Bytes>) -> Option<Bytes> {
    let m465 = (p466, p467);
    {
        let mut m465 = m465;
        loop {
            let step = {
                let (v1, v2) = m465;
                let m463 = v2;
                match m463.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c464: Fn1 = v0.clone();
                            c464.apply(v1, v3)
                        }, v4))
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

pub fn f116(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m468 = v1;
    match m468 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a471 = {
                    let a469 = v3;
                    let a470 = v0;
                    append_bytes(a469, a470)
                };
                let a472 = v2;
                append_bytes(a471, a472)
            })
        }
    }
}

pub fn f117(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f156(v0)?;
    let m476 = f157(v1.clone(), 0u64, {
        let a474 = {
            let a473 = v1.clone();
            length_bytes(a473)
        };
        let a475 = 2u64;
        nat_add(a474, a475)?
    }, 0u64)?;
    match m476 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a479 = {
                let (_, h477) = v2;
                h477
            };
            let a480 = {
                let a478 = v1;
                length_bytes(a478)
            };
            if nat_eq(a479, a480) {
                Ok(Some({
                    let (h481, _) = v2;
                    h481
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f118(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f71(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f60(Str::lit("id"), f69(v0.clone(), f8(Str::lit("id"))))?, List::cons(f60(Str::lit("name"), f69(v0.clone(), f8(Str::lit("name"))))?, List::cons(f60(Str::lit("issuer"), f69(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f60(Str::lit("edition"), f69(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f60(Str::lit("canonical_id"), f69(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f60(Str::lit("source_role"), f69(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f60(Str::lit("retrieval_date"), f69(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f60(Str::lit("url"), f158(v2.clone(), f8(Str::lit("url"))))?, List::cons(f60(Str::lit("media_type"), f158(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f60(Str::lit("license"), f158(v2.clone(), f8(Str::lit("license"))))?, List::cons(f60(Str::lit("redistribution"), f158(v2, f8(Str::lit("redistribution"))))?, List::cons(f60(Str::lit("statement"), f69(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m482 = v1;
        match m482 {
            None => {
                f7(List::cons(f60(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f60(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f60(Str::lit("artifact"), {
                    let (h483, _) = v3;
                    h483
                })?, List::cons(f60(Str::lit("checksum"), {
                    let (_, h484) = v3;
                    h484
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f159(f52(f102(f71(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f119(v0: Bytes, p551: u64, p552: u64, p553: bool, p554: Bytes) -> R<Bytes> {
    let m550 = (p551, (p552, (p553, p554)));
    {
        let mut m550 = m550;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m550;
                let m485 = v2;
                if m485 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m485 - 1;
                    let m488 = {
                        let a486 = v0.clone();
                        let a487 = v1;
                        index_bytes(a486, a487)
                    };
                    match m488 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a533 = {
                                    let a509 = {
                                        let a497 = {
                                            let a491 = {
                                                let a489 = v7;
                                                let a490 = 128u8;
                                                fixed_u8::bit_and(a489, a490)
                                            };
                                            let a492 = 128u8;
                                            if equal(a491, a492) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a498 = {
                                            let a495 = {
                                                let a493 = v7;
                                                let a494 = 64u8;
                                                fixed_u8::bit_and(a493, a494)
                                            };
                                            let a496 = 64u8;
                                            if equal(a495, a496) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a497, a498)?
                                    };
                                    let a510 = {
                                        let a507 = {
                                            let a501 = {
                                                let a499 = v7;
                                                let a500 = 32u8;
                                                fixed_u8::bit_and(a499, a500)
                                            };
                                            let a502 = 32u8;
                                            if equal(a501, a502) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a508 = {
                                            let a505 = {
                                                let a503 = v7;
                                                let a504 = 16u8;
                                                fixed_u8::bit_and(a503, a504)
                                            };
                                            let a506 = 16u8;
                                            if equal(a505, a506) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a507, a508)?
                                    };
                                    nat_add(a509, a510)?
                                };
                                let a534 = {
                                    let a531 = {
                                        let a519 = {
                                            let a513 = {
                                                let a511 = v7;
                                                let a512 = 8u8;
                                                fixed_u8::bit_and(a511, a512)
                                            };
                                            let a514 = 8u8;
                                            if equal(a513, a514) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a520 = {
                                            let a517 = {
                                                let a515 = v7;
                                                let a516 = 4u8;
                                                fixed_u8::bit_and(a515, a516)
                                            };
                                            let a518 = 4u8;
                                            if equal(a517, a518) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a519, a520)?
                                    };
                                    let a532 = {
                                        let a529 = {
                                            let a523 = {
                                                let a521 = v7;
                                                let a522 = 2u8;
                                                fixed_u8::bit_and(a521, a522)
                                            };
                                            let a524 = 2u8;
                                            if equal(a523, a524) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a530 = {
                                            let a527 = {
                                                let a525 = v7;
                                                let a526 = 1u8;
                                                fixed_u8::bit_and(a525, a526)
                                            };
                                            let a528 = 1u8;
                                            if equal(a527, a528) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a529, a530)?
                                    };
                                    nat_add(a531, a532)?
                                };
                                nat_add(a533, a534)?
                            };
                            if f160(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a535 = v1;
                                    let a536 = 1u64;
                                    nat_add(a535, a536)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f161(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a537 = v1;
                                        let a538 = 1u64;
                                        nat_add(a537, a538)?
                                    }, (v5, (false, {
                                        let a542 = v4;
                                        let a543 = f38({
                                            let a539 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a539)
                                        }, {
                                            let a540 = v8;
                                            let a541 = 97u64;
                                            nat_sub(a540, a541)
                                        }, 1u64);
                                        append_bytes(a542, a543)
                                    }))))
                                } else {
                                    if if f161(v8) {
                                        true
                                    } else {
                                        if f162(v8) {
                                            true
                                        } else {
                                            f163(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a544 = v1;
                                            let a545 = 1u64;
                                            nat_add(a544, a545)?
                                        }, (v5, (false, {
                                            let a546 = v4;
                                            let a547 = f38(v0.clone(), v1, 1u64);
                                            append_bytes(a546, a547)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a548 = v1;
                                            let a549 = 1u64;
                                            nat_add(a548, a549)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(p562: u64, p563: List<Adt17>, p564: List<Bytes>, p565: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m561 = (p562, (p563, (p564, p565)));
    {
        let mut m561 = m561;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m561;
                let m555 = v0;
                if m555 == 0 {
                    Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m555 - 1;
                    let m556 = v1;
                    match m556.uncons() {
                        None => {
                            Err::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m557 = v5;
                            match m557 {
                                Adt17::C0(v8, v9) => {
                                    if f54(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m558 = f164(v8.clone(), v9)?;
                                        match m558 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt17>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f121({
                                                    let (_, h559) = v11;
                                                    h559
                                                }, List::cons(Adt17::C1({
                                                    let (h560, _) = v11;
                                                    h560
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
                Ok(next) => { m561 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: List<Adt17>, v1: List<Adt17>) -> List<Adt17> {
    let m566 = v0;
    match m566.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f121(v3, v1))
        }
    }
}

pub fn f122(v0: List<(Bytes, Adt14)>) -> R<List<Adt17>> {
    let m567 = v0;
    match m567.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt17::C0(f77({
                let (h568, _) = v1;
                h568
            })?, {
                let (_, h569) = v1;
                h569
            }), f122(v2)?))
        }
    }
}

pub fn f123(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m570 = v0;
    match m570 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f165(v1)
        }
    }
}

pub fn f124(p573: List<Bytes>, p574: List<Bytes>) -> List<Bytes> {
    let m572 = (p573, p574);
    {
        let mut m572 = m572;
        loop {
            let step = {
                let (v0, v1) = m572;
                let m571 = v0;
                match m571.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f125(v0: Adt14) -> R<Bytes> {
    let m575 = v0;
    match m575 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f166(v2, v3)
        }
        Adt14::C2(v4) => {
            f167(v4)
        }
        Adt14::C3(r576) => {
            let v5 = (*r576).clone();
            Ok(f168(f169(v5)?))
        }
        Adt14::C4(r577) => {
            let v6 = (*r577).clone();
            f170(f171(v6)?)
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

pub fn f126(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m578 = v0;
    match m578.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f172(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f112(f173(f114(List::cons(v1, f174(f36(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f126(v3, v1)?))
            }
        }
    }
}

pub fn f127(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h579, _) = v2.clone();
        h579
    };
    let v4: u64 = {
        let (h580, _) = {
            let (_, h581) = v2.clone();
            h581
        };
        h580
    };
    let v5: List<Bytes> = {
        let (_, h582) = {
            let (_, h583) = v2;
            h583
        };
        h582
    };
    let m584 = f175(v0.clone(), v3);
    match m584 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a585 = v6;
            let a586 = v1;
            if equal(a585, a586) {
                Ok(Some(({
                    let a587 = v3;
                    let a588 = 1u64;
                    nat_add(a587, a588)?
                }, ({
                    let a589 = v3;
                    let a590 = 1u64;
                    nat_add(a589, a590)?
                }, List::cons(f38(v0, v4, {
                    let a591 = v3;
                    let a592 = v4;
                    nat_sub(a591, a592)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a593 = v3;
                    let a594 = 1u64;
                    nat_add(a593, a594)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f128(v0: Fn2, p598: List<Bytes>, p599: List<Bytes>) -> List<Bytes> {
    let m597 = (p598, p599);
    {
        let mut m597 = m597;
        loop {
            let step = {
                let (v1, v2) = m597;
                let m595 = v2;
                match m595.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c596: Fn2 = v0.clone();
                            c596.apply(v1, v3)
                        }, v4))
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

pub fn f129(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f130(v0: Bytes) -> bool {
    f88(v0, f8(Str::lit(".lex.tex")))
}

pub fn f131(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m600 = v0;
    match m600.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f131(v3, {
                let a601 = v1;
                let a602 = 1u64;
                nat_add(a601, a602)?
            })?))
        }
    }
}

pub fn f132(v0: Adt15, v1: List<u64>, v2: Adt19) -> bool {
    let m603 = v2;
    match m603 {
        Adt19::C0(v3, v4) => {
            if f138(v3, match v0.clone() {
                Adt15::C0(h604, _, _, _) => {
                    h604
                }
            }) {
                f176(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f138(v5, match v0 {
                Adt15::C0(_, _, h605, _) => {
                    h605
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f134(v7, v1) {
                f138(v8, match v0 {
                    Adt15::C0(_, _, _, h606) => {
                        h606
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f133(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m607 = f71(v1.clone(), f8(Str::lit("std")));
    match m607 {
        None => {
            let m612 = f72(f71(v1.clone(), f8(Str::lit("req"))));
            match m612 {
                None => {
                    let m614 = f71(v1, f8(Str::lit("der")));
                    match m614 {
                        None => {
                            Ok(f177(v0))
                        }
                        Some(v5) => {
                            let m615 = f178(f69(v5.clone(), f8(Str::lit("rule"))));
                            match m615 {
                                None => {
                                    Ok(f177(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f97(f52(f102(f71(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h616) => {
                                            h616
                                        }
                                    })?, f136(f69(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h617, _) => {
                                            h617
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f136(v4, match v0 {
                        Adt12::C0(_, _, h613, _, _) => {
                            h613
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m608 = f72(f71(v2.clone(), f8(Str::lit("row"))));
            match m608 {
                None => {
                    Ok(Adt19::C0(f136(f69(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h609, _, _, _, _) => {
                            h609
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f136(f69(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h610, _, _, _, _) => {
                            h610
                        }
                    })?, Some(f136(v3, match v0 {
                        Adt12::C0(_, h611, _, _, _) => {
                            h611
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f134(p620: List<u64>, v1: List<u64>) -> bool {
    let m619 = p620;
    {
        let mut m619 = m619;
        loop {
            let step = {
                let v0 = m619;
                let m618 = v0;
                match m618.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f138(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m619 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(p625: List<u64>) -> bool {
    let m624 = p625;
    {
        let mut m624 = m624;
        loop {
            let step = {
                let v0 = m624;
                let m621 = v0;
                match m621.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a622 = v1;
                        let a623 = 7u64;
                        if nat_le(a622, a623) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m624 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m626 = f179(v0, v1.clone(), 0u64)?;
    match m626 {
        None => {
            let a627 = v1;
            Ok(length_list(a627))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f137(v0: Adt14) -> u64 {
    let m628 = f180(f71(v0, f8(Str::lit("stratum"))));
    match m628 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f138(v0: u64, p633: List<u64>) -> bool {
    let m632 = p633;
    {
        let mut m632 = m632;
        loop {
            let step = {
                let v1 = m632;
                let m629 = v1;
                match m629.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a630 = v2;
                        let a631 = v0;
                        if nat_eq(a630, a631) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m632 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f172(v0.clone(), v1) {
        let m634 = f36(v0, 34u8)?;
        match m634.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m635 = v3;
                match m635.uncons() {
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

pub fn f140(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f181(v0, v1)
}

pub fn f141(v0: Adt14) -> List<Adt14> {
    let m636 = v0;
    match m636 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r637) => {
            let v3 = (*r637).clone();
            f141(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r638, r639) => {
            let v1 = (*r638).clone();
            let v2 = (*r639).clone();
            List::cons(v1, f141(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f142(v0: Bytes, v1: u64) -> R<u64> {
    let m642 = {
        let a640 = v0;
        let a641 = v1;
        index_bytes(a640, a641)
    };
    match m642 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a687 = {
                let a663 = {
                    let a651 = {
                        let a645 = {
                            let a643 = v3;
                            let a644 = 128u8;
                            fixed_u8::bit_and(a643, a644)
                        };
                        let a646 = 128u8;
                        if equal(a645, a646) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a652 = {
                        let a649 = {
                            let a647 = v3;
                            let a648 = 64u8;
                            fixed_u8::bit_and(a647, a648)
                        };
                        let a650 = 64u8;
                        if equal(a649, a650) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a651, a652)?
                };
                let a664 = {
                    let a661 = {
                        let a655 = {
                            let a653 = v3;
                            let a654 = 32u8;
                            fixed_u8::bit_and(a653, a654)
                        };
                        let a656 = 32u8;
                        if equal(a655, a656) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a662 = {
                        let a659 = {
                            let a657 = v3;
                            let a658 = 16u8;
                            fixed_u8::bit_and(a657, a658)
                        };
                        let a660 = 16u8;
                        if equal(a659, a660) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a661, a662)?
                };
                nat_add(a663, a664)?
            };
            let a688 = {
                let a685 = {
                    let a673 = {
                        let a667 = {
                            let a665 = v3;
                            let a666 = 8u8;
                            fixed_u8::bit_and(a665, a666)
                        };
                        let a668 = 8u8;
                        if equal(a667, a668) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a674 = {
                        let a671 = {
                            let a669 = v3;
                            let a670 = 4u8;
                            fixed_u8::bit_and(a669, a670)
                        };
                        let a672 = 4u8;
                        if equal(a671, a672) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a673, a674)?
                };
                let a686 = {
                    let a683 = {
                        let a677 = {
                            let a675 = v3;
                            let a676 = 2u8;
                            fixed_u8::bit_and(a675, a676)
                        };
                        let a678 = 2u8;
                        if equal(a677, a678) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a684 = {
                        let a681 = {
                            let a679 = v3;
                            let a680 = 1u8;
                            fixed_u8::bit_and(a679, a680)
                        };
                        let a682 = 1u8;
                        if equal(a681, a682) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a683, a684)?
                };
                nat_add(a685, a686)?
            };
            nat_add(a687, a688)
        }
    }
}

pub fn f143(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m692 = {
        let a689 = v1;
        let a690 = v2;
        let a691 = 1u64;
        slice_bytes(a689, a690, a691)
    };
    match m692 {
        None => {
            v0
        }
        Some(v3) => {
            let a693 = v0;
            let a694 = v3;
            append_bytes(a693, a694)
        }
    }
}

pub fn f144(p697: List<Bytes>, p698: List<Bytes>) -> List<Bytes> {
    let m696 = (p697, p698);
    {
        let mut m696 = m696;
        loop {
            let step = {
                let (v0, v1) = m696;
                let m695 = v0;
                match m695.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m696 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f145(v0: Bytes, p770: u64, p771: u64, p772: Bytes) -> R<Bytes> {
    let m769 = (p770, (p771, p772));
    {
        let mut m769 = m769;
        loop {
            let step = {
                let (v1, (v2, v3)) = m769;
                let m699 = v2;
                if m699 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m699 - 1;
                    let m702 = {
                        let a700 = v0.clone();
                        let a701 = v1;
                        index_bytes(a700, a701)
                    };
                    match m702 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a747 = {
                                    let a723 = {
                                        let a711 = {
                                            let a705 = {
                                                let a703 = v6;
                                                let a704 = 128u8;
                                                fixed_u8::bit_and(a703, a704)
                                            };
                                            let a706 = 128u8;
                                            if equal(a705, a706) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a712 = {
                                            let a709 = {
                                                let a707 = v6;
                                                let a708 = 64u8;
                                                fixed_u8::bit_and(a707, a708)
                                            };
                                            let a710 = 64u8;
                                            if equal(a709, a710) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a711, a712)?
                                    };
                                    let a724 = {
                                        let a721 = {
                                            let a715 = {
                                                let a713 = v6;
                                                let a714 = 32u8;
                                                fixed_u8::bit_and(a713, a714)
                                            };
                                            let a716 = 32u8;
                                            if equal(a715, a716) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a722 = {
                                            let a719 = {
                                                let a717 = v6;
                                                let a718 = 16u8;
                                                fixed_u8::bit_and(a717, a718)
                                            };
                                            let a720 = 16u8;
                                            if equal(a719, a720) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a721, a722)?
                                    };
                                    nat_add(a723, a724)?
                                };
                                let a748 = {
                                    let a745 = {
                                        let a733 = {
                                            let a727 = {
                                                let a725 = v6;
                                                let a726 = 8u8;
                                                fixed_u8::bit_and(a725, a726)
                                            };
                                            let a728 = 8u8;
                                            if equal(a727, a728) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a734 = {
                                            let a731 = {
                                                let a729 = v6;
                                                let a730 = 4u8;
                                                fixed_u8::bit_and(a729, a730)
                                            };
                                            let a732 = 4u8;
                                            if equal(a731, a732) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a733, a734)?
                                    };
                                    let a746 = {
                                        let a743 = {
                                            let a737 = {
                                                let a735 = v6;
                                                let a736 = 2u8;
                                                fixed_u8::bit_and(a735, a736)
                                            };
                                            let a738 = 2u8;
                                            if equal(a737, a738) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a744 = {
                                            let a741 = {
                                                let a739 = v6;
                                                let a740 = 1u8;
                                                fixed_u8::bit_and(a739, a740)
                                            };
                                            let a742 = 1u8;
                                            if equal(a741, a742) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a743, a744)?
                                    };
                                    nat_add(a745, a746)?
                                };
                                nat_add(a747, a748)?
                            };
                            let a749 = v7;
                            let a750 = 34u64;
                            if if nat_eq(a749, a750) {
                                true
                            } else {
                                let a751 = v7;
                                let a752 = 92u64;
                                nat_eq(a751, a752)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a753 = v1;
                                    let a754 = 1u64;
                                    nat_add(a753, a754)?
                                }, (v4, {
                                    let a757 = v3;
                                    let a758 = {
                                        let a755 = Bytes::lit(&[0x5c]);
                                        let a756 = f38(v0.clone(), v1, 1u64);
                                        append_bytes(a755, a756)
                                    };
                                    append_bytes(a757, a758)
                                })))
                            } else {
                                let a759 = v7;
                                let a760 = 10u64;
                                if nat_eq(a759, a760) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a761 = v1;
                                        let a762 = 1u64;
                                        nat_add(a761, a762)?
                                    }, (v4, {
                                        let a763 = v3;
                                        let a764 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a763, a764)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a765 = v1;
                                        let a766 = 1u64;
                                        nat_add(a765, a766)?
                                    }, (v4, {
                                        let a767 = v3;
                                        let a768 = f38(v0.clone(), v1, 1u64);
                                        append_bytes(a767, a768)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: u32) -> Bytes {
    let a801 = f182({
        let a773 = f183(v0, 28u32);
        let a774 = 15u32;
        fixed_u32::bit_and(a773, a774)
    });
    let a802 = {
        let a799 = f182({
            let a775 = f183(v0, 24u32);
            let a776 = 15u32;
            fixed_u32::bit_and(a775, a776)
        });
        let a800 = {
            let a797 = f182({
                let a777 = f183(v0, 20u32);
                let a778 = 15u32;
                fixed_u32::bit_and(a777, a778)
            });
            let a798 = {
                let a795 = f182({
                    let a779 = f183(v0, 16u32);
                    let a780 = 15u32;
                    fixed_u32::bit_and(a779, a780)
                });
                let a796 = {
                    let a793 = f182({
                        let a781 = f183(v0, 12u32);
                        let a782 = 15u32;
                        fixed_u32::bit_and(a781, a782)
                    });
                    let a794 = {
                        let a791 = f182({
                            let a783 = f183(v0, 8u32);
                            let a784 = 15u32;
                            fixed_u32::bit_and(a783, a784)
                        });
                        let a792 = {
                            let a789 = f182({
                                let a785 = f183(v0, 4u32);
                                let a786 = 15u32;
                                fixed_u32::bit_and(a785, a786)
                            });
                            let a790 = f182({
                                let a787 = v0;
                                let a788 = 15u32;
                                fixed_u32::bit_and(a787, a788)
                            });
                            append_bytes(a789, a790)
                        };
                        append_bytes(a791, a792)
                    };
                    append_bytes(a793, a794)
                };
                append_bytes(a795, a796)
            };
            append_bytes(a797, a798)
        };
        append_bytes(a799, a800)
    };
    append_bytes(a801, a802)
}

pub fn f147(v0: Fn3, p806: u64, p807: (u64, Adt21)) -> R<(u64, Adt21)> {
    let m805 = (p806, p807);
    {
        let mut m805 = m805;
        loop {
            let step = {
                let (v1, v2) = m805;
                let m803 = v1;
                if m803 == 0 {
                    Err::<(u64, (u64, Adt21)), R<(u64, Adt21)>>(Ok(v2))
                } else {
                    let v3 = m803 - 1;
                    Ok::<(u64, (u64, Adt21)), R<(u64, Adt21)>>((v3, {
                        let c804: Fn3 = v0.clone();
                        c804.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m805 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt21)) -> R<(u64, Adt21)> {
    Ok(({
        let a809 = {
            let (h808, _) = v3.clone();
            h808
        };
        let a810 = 64u64;
        nat_add(a809, a810)?
    }, f184({
        let (_, h811) = v3;
        h811
    }, f185(v0, v1, v2, {
        let (h812, _) = v3;
        h812
    })?)?))
}

pub fn f149() -> Adt21 {
    Adt21::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f150(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f54(v1, f47(f112(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f151(v0: List<Bytes>) -> List<Bytes> {
    let m813 = v0;
    match m813.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f26(v1.clone(), f8(Str::lit("authority = []"))) {
                f151(v2)
            } else {
                List::cons(v1, f151(v2))
            }
        }
    }
}

pub fn f152(v0: Bytes) -> R<u64> {
    let (h814, _) = f186(Fn4::F187(v0.clone()), {
        let a815 = v0.clone();
        length_bytes(a815)
    }, {
        let a816 = v0;
        length_bytes(a816)
    })?;
    Ok(h814)
}

pub fn f153(v0: List<Bytes>) -> List<Bytes> {
    f128(Fn2::F188, List::<Bytes>::nil(), v0)
}

pub fn f154(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f189(v0, v1)
}

pub fn f155(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f190(f191(f192(v0))?))
}

pub fn f156(v0: Bytes) -> R<Bytes> {
    f193(v0.clone(), 0u64, {
        let a817 = v0;
        length_bytes(a817)
    }, false, false, Bytes::lit(&[]))
}

pub fn f157(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m818 = v2;
    if m818 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m818 - 1;
        let v5: u64 = f194(v0.clone(), v1)?;
        let a819 = v3;
        let a820 = 1u64;
        if nat_eq(a819, a820) {
            let m821 = f157(v0.clone(), v1, v4, 0u64)?;
            match m821 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f194(v0.clone(), {
                        let (_, h822) = v6.clone();
                        h822
                    })?;
                    let a823 = v7;
                    let a824 = 44u64;
                    if nat_eq(a823, a824) {
                        let m828 = f157(v0, {
                            let a826 = {
                                let (_, h825) = v6;
                                h825
                            };
                            let a827 = 1u64;
                            nat_add(a826, a827)?
                        }, v4, 1u64)?;
                        match m828 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h829, _) = v6;
                                    h829
                                }), std::rc::Rc::new({
                                    let (h830, _) = v8;
                                    h830
                                })), {
                                    let (_, h831) = v8;
                                    h831
                                })))
                            }
                        }
                    } else {
                        let a832 = v7;
                        let a833 = 93u64;
                        if nat_eq(a832, a833) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h834, _) = v6;
                                h834
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a836 = {
                                    let (_, h835) = v6;
                                    h835
                                };
                                let a837 = 1u64;
                                nat_add(a836, a837)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a838 = v3;
            let a839 = 2u64;
            if nat_eq(a838, a839) {
                let m842 = f195(v0.clone(), {
                    let a840 = v1;
                    let a841 = 1u64;
                    nat_add(a840, a841)?
                })?;
                match m842 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a843 = v5;
                        let a844 = 34u64;
                        if if nat_eq(a843, a844) {
                            let a846 = f194(v0.clone(), {
                                let (h845, _) = v9.clone();
                                h845
                            })?;
                            let a847 = 58u64;
                            nat_eq(a846, a847)
                        } else {
                            false
                        } {
                            let m851 = f157(v0.clone(), {
                                let a849 = {
                                    let (h848, _) = v9;
                                    h848
                                };
                                let a850 = 1u64;
                                nat_add(a849, a850)?
                            }, v4, 0u64)?;
                            match m851 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f194(v0.clone(), {
                                        let (_, h852) = v10.clone();
                                        h852
                                    })?;
                                    let a853 = v11;
                                    let a854 = 44u64;
                                    if nat_eq(a853, a854) {
                                        let m858 = f157(v0, {
                                            let a856 = {
                                                let (_, h855) = v10;
                                                h855
                                            };
                                            let a857 = 1u64;
                                            nat_add(a856, a857)?
                                        }, v4, 2u64)?;
                                        match m858 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h859) = v9;
                                                    h859
                                                }, std::rc::Rc::new({
                                                    let (h860, _) = v10;
                                                    h860
                                                }), std::rc::Rc::new({
                                                    let (h861, _) = v12;
                                                    h861
                                                })), {
                                                    let (_, h862) = v12;
                                                    h862
                                                })))
                                            }
                                        }
                                    } else {
                                        let a863 = v11;
                                        let a864 = 125u64;
                                        if nat_eq(a863, a864) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h865) = v9;
                                                h865
                                            }, std::rc::Rc::new({
                                                let (h866, _) = v10;
                                                h866
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a868 = {
                                                    let (_, h867) = v10;
                                                    h867
                                                };
                                                let a869 = 1u64;
                                                nat_add(a868, a869)?
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
                let a870 = v5;
                let a871 = 34u64;
                if nat_eq(a870, a871) {
                    let m874 = f195(v0, {
                        let a872 = v1;
                        let a873 = 1u64;
                        nat_add(a872, a873)?
                    })?;
                    match m874 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h875) = v13;
                                h875
                            }), {
                                let (h876, _) = v13;
                                h876
                            })))
                        }
                    }
                } else {
                    if f26(f38(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a877 = v1;
                            let a878 = 4u64;
                            nat_add(a877, a878)?
                        })))
                    } else {
                        if f26(f38(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a879 = v1;
                                let a880 = 5u64;
                                nat_add(a879, a880)?
                            })))
                        } else {
                            let a881 = v5;
                            let a882 = 91u64;
                            if nat_eq(a881, a882) {
                                let a885 = f194(v0.clone(), {
                                    let a883 = v1;
                                    let a884 = 1u64;
                                    nat_add(a883, a884)?
                                })?;
                                let a886 = 93u64;
                                if nat_eq(a885, a886) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a887 = v1;
                                        let a888 = 2u64;
                                        nat_add(a887, a888)?
                                    })))
                                } else {
                                    let m891 = f157(v0, {
                                        let a889 = v1;
                                        let a890 = 1u64;
                                        nat_add(a889, a890)?
                                    }, v4, 1u64)?;
                                    match m891 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h892, _) = v14;
                                                h892
                                            })), {
                                                let (_, h893) = v14;
                                                h893
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a894 = v5;
                                let a895 = 123u64;
                                if nat_eq(a894, a895) {
                                    let a898 = f194(v0.clone(), {
                                        let a896 = v1;
                                        let a897 = 1u64;
                                        nat_add(a896, a897)?
                                    })?;
                                    let a899 = 125u64;
                                    if nat_eq(a898, a899) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a900 = v1;
                                            let a901 = 2u64;
                                            nat_add(a900, a901)?
                                        })))
                                    } else {
                                        let m904 = f157(v0, {
                                            let a902 = v1;
                                            let a903 = 1u64;
                                            nat_add(a902, a903)?
                                        }, v4, 2u64)?;
                                        match m904 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h905, _) = v15;
                                                    h905
                                                })), {
                                                    let (_, h906) = v15;
                                                    h906
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m907 = f196(v0, v1)?;
                                    match m907 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h908) = v16;
                                                h908
                                            }, {
                                                let (h909, _) = v16;
                                                h909
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

pub fn f158(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m910 = v0;
    match m910 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f69(v2, v1)
        }
    }
}

pub fn f159(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f112(f173(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f160(v0: u64) -> bool {
    let a911 = v0;
    let a912 = 95u64;
    if nat_eq(a911, a912) {
        true
    } else {
        let a913 = v0;
        let a914 = 45u64;
        if nat_eq(a913, a914) {
            true
        } else {
            let a915 = v0;
            let a916 = 32u64;
            if nat_eq(a915, a916) {
                true
            } else {
                let a917 = v0;
                let a918 = 46u64;
                if nat_eq(a917, a918) {
                    true
                } else {
                    let a919 = v0;
                    let a920 = 47u64;
                    nat_eq(a919, a920)
                }
            }
        }
    }
}

pub fn f161(v0: u64) -> bool {
    let a921 = 97u64;
    let a922 = v0;
    if nat_le(a921, a922) {
        let a923 = v0;
        let a924 = 122u64;
        nat_le(a923, a924)
    } else {
        false
    }
}

pub fn f162(v0: u64) -> bool {
    let a925 = 65u64;
    let a926 = v0;
    if nat_le(a925, a926) {
        let a927 = v0;
        let a928 = 90u64;
        nat_le(a927, a928)
    } else {
        false
    }
}

pub fn f163(v0: u64) -> bool {
    let a929 = 48u64;
    let a930 = v0;
    if nat_le(a929, a930) {
        let a931 = v0;
        let a932 = 57u64;
        nat_le(a931, a932)
    } else {
        false
    }
}

pub fn f164(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m933 = f71(v1.clone(), f8(Str::lit("enum")));
    match m933 {
        None => {
            let m935 = f200(v0.clone(), f123(f71(v1.clone(), f8(Str::lit("properties")))), f52(f102(f71(v1, f8(Str::lit("required"))))))?;
            match m935 {
                Ok(v6) => {
                    Ok(f199(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m937 = {
                            let (h936, _) = v6.clone();
                            h936
                        };
                        match m937.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f112({
                                    let (h938, _) = v6;
                                    h938
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h939) = v6;
                        h939
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt17>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m934 = f197(f52(f141(v2)))?;
            match m934 {
                Ok(v4) => {
                    Ok(f199(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f112(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt17>::nil()))
                }
                Err(v3) => {
                    Ok(f198(v3, v0))
                }
            }
        }
    }
}

pub fn f165(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m940 = v0;
    match m940 {
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
        Adt14::C4(r941) => {
            let v4 = (*r941).clone();
            f165(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r942, r943) => {
            let v2 = (*r942).clone();
            let v3 = (*r943).clone();
            List::cons((v1, v2), f165(v3))
        }
    }
}

pub fn f166(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a944 = 0u64;
        let a945 = v1;
        nat_lt(a944, a945)
    } else {
        false
    } {
        let a946 = Bytes::lit(&[0x2d]);
        let a947 = f201(v1)?;
        Ok(append_bytes(a946, a947))
    } else {
        f201(v1)
    }
}

pub fn f167(v0: Bytes) -> R<Bytes> {
    let a953 = {
        let a951 = Bytes::lit(&[0x22]);
        let a952 = {
            let (_, h948) = {
                let (h949, _) = f202(Fn5::F203(v0.clone()), {
                    let a950 = v0;
                    length_bytes(a950)
                }, (0u64, Bytes::lit(&[])))?;
                h949
            };
            h948
        };
        append_bytes(a951, a952)
    };
    let a954 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a953, a954))
}

pub fn f168(v0: List<Bytes>) -> Bytes {
    let a957 = {
        let a955 = Bytes::lit(&[0x5b]);
        let a956 = f65(v0, Bytes::lit(&[0x2c]));
        append_bytes(a955, a956)
    };
    let a958 = Bytes::lit(&[0x5d]);
    append_bytes(a957, a958)
}

pub fn f169(v0: Adt14) -> R<List<Bytes>> {
    let m959 = v0;
    match m959 {
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
        Adt14::C6(r960, r961) => {
            let v7 = (*r960).clone();
            let v8 = (*r961).clone();
            Ok(List::cons(f125(v7)?, f169(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f170(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f204(Fn6::F205, List::<(Bytes, Bytes)>::nil(), f206(v0)?);
    let v2: List<Bytes> = f207(Fn7::F208, List::<Bytes>::nil(), v1)?;
    let a964 = {
        let a962 = Bytes::lit(&[0x7b]);
        let a963 = f65(v2, Bytes::lit(&[0x2c]));
        append_bytes(a962, a963)
    };
    let a965 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a964, a965))
}

pub fn f171(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m966 = v0;
    match m966 {
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
        Adt14::C7(v9, r967, r968) => {
            let v10 = (*r967).clone();
            let v11 = (*r968).clone();
            Ok(List::cons((v9, f125(v10)?), f171(v11)?))
        }
    }
}

pub fn f172(v0: Bytes, v1: Bytes) -> bool {
    let m973 = {
        let a970 = v0;
        let a971 = 0u64;
        let a972 = {
            let a969 = v1.clone();
            length_bytes(a969)
        };
        slice_bytes(a970, a971, a972)
    };
    match m973 {
        None => {
            false
        }
        Some(v2) => {
            let a974 = v2;
            let a975 = v1;
            equal(a974, a975)
        }
    }
}

pub fn f173(v0: List<Bytes>) -> R<List<Bytes>> {
    let m976 = v0;
    match m976.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f106(v1)?, f173(v2)?))
        }
    }
}

pub fn f174(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m977 = v0;
    match m977.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f174(v3, false))
            } else {
                f174(v3, true)
            }
        }
    }
}

pub fn f175(v0: Bytes, v1: u64) -> Option<u8> {
    let a978 = v0;
    let a979 = v1;
    index_bytes(a978, a979)
}

pub fn f176(v0: Adt15, v1: Option<u64>) -> bool {
    let m980 = v1;
    match m980 {
        None => {
            true
        }
        Some(v2) => {
            f138(v2, match v0 {
                Adt15::C0(_, h981, _, _) => {
                    h981
                }
            })
        }
    }
}

pub fn f177(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a983 = match v0 {
            Adt12::C0(_, _, h982, _, _) => {
                h982
            }
        };
        length_list(a983)
    })
}

pub fn f178(v0: Bytes) -> Option<Adt20> {
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

pub fn f179(v0: Bytes, p988: List<Bytes>, p989: u64) -> R<Option<u64>> {
    let m987 = (p988, p989);
    {
        let mut m987 = m987;
        loop {
            let step = {
                let (v1, v2) = m987;
                let m984 = v1;
                match m984.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f26(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a985 = v2;
                                let a986 = 1u64;
                                nat_add(a985, a986)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m987 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Option<Adt14>) -> Option<u64> {
    let m990 = v0;
    match m990 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m991 = v1;
            match m991 {
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

pub fn f181(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h992, _) = v1.clone();
        h992
    };
    let v3: u64 = {
        let (h993, _) = {
            let (_, h994) = v1.clone();
            h994
        };
        h993
    };
    let v4: List<Bytes> = {
        let (_, h995) = {
            let (_, h996) = v1;
            h996
        };
        h995
    };
    let m997 = f175(v0.clone(), v2);
    match m997 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a998 = v5;
            let a999 = 10u8;
            if equal(a998, a999) {
                Ok(Some(({
                    let a1000 = v2;
                    let a1001 = 1u64;
                    nat_add(a1000, a1001)?
                }, ({
                    let a1002 = v2;
                    let a1003 = 1u64;
                    nat_add(a1002, a1003)?
                }, List::cons(f209(f38(v0, v3, {
                    let a1004 = v2;
                    let a1005 = v3;
                    nat_sub(a1004, a1005)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1006 = v2;
                    let a1007 = 1u64;
                    nat_add(a1006, a1007)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f182(v0: u32) -> Bytes {
    let a1010 = {
        let a1008 = v0;
        let a1009 = 8u32;
        fixed_u32::bit_and(a1008, a1009)
    };
    let a1011 = 0u32;
    if equal(a1010, a1011) {
        let a1014 = {
            let a1012 = v0;
            let a1013 = 4u32;
            fixed_u32::bit_and(a1012, a1013)
        };
        let a1015 = 0u32;
        if equal(a1014, a1015) {
            let a1018 = {
                let a1016 = v0;
                let a1017 = 2u32;
                fixed_u32::bit_and(a1016, a1017)
            };
            let a1019 = 0u32;
            if equal(a1018, a1019) {
                let a1022 = {
                    let a1020 = v0;
                    let a1021 = 1u32;
                    fixed_u32::bit_and(a1020, a1021)
                };
                let a1023 = 0u32;
                if equal(a1022, a1023) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1026 = {
                    let a1024 = v0;
                    let a1025 = 1u32;
                    fixed_u32::bit_and(a1024, a1025)
                };
                let a1027 = 0u32;
                if equal(a1026, a1027) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1030 = {
                let a1028 = v0;
                let a1029 = 2u32;
                fixed_u32::bit_and(a1028, a1029)
            };
            let a1031 = 0u32;
            if equal(a1030, a1031) {
                let a1034 = {
                    let a1032 = v0;
                    let a1033 = 1u32;
                    fixed_u32::bit_and(a1032, a1033)
                };
                let a1035 = 0u32;
                if equal(a1034, a1035) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1038 = {
                    let a1036 = v0;
                    let a1037 = 1u32;
                    fixed_u32::bit_and(a1036, a1037)
                };
                let a1039 = 0u32;
                if equal(a1038, a1039) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1042 = {
            let a1040 = v0;
            let a1041 = 4u32;
            fixed_u32::bit_and(a1040, a1041)
        };
        let a1043 = 0u32;
        if equal(a1042, a1043) {
            let a1046 = {
                let a1044 = v0;
                let a1045 = 2u32;
                fixed_u32::bit_and(a1044, a1045)
            };
            let a1047 = 0u32;
            if equal(a1046, a1047) {
                let a1050 = {
                    let a1048 = v0;
                    let a1049 = 1u32;
                    fixed_u32::bit_and(a1048, a1049)
                };
                let a1051 = 0u32;
                if equal(a1050, a1051) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1054 = {
                    let a1052 = v0;
                    let a1053 = 1u32;
                    fixed_u32::bit_and(a1052, a1053)
                };
                let a1055 = 0u32;
                if equal(a1054, a1055) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1058 = {
                let a1056 = v0;
                let a1057 = 2u32;
                fixed_u32::bit_and(a1056, a1057)
            };
            let a1059 = 0u32;
            if equal(a1058, a1059) {
                let a1062 = {
                    let a1060 = v0;
                    let a1061 = 1u32;
                    fixed_u32::bit_and(a1060, a1061)
                };
                let a1063 = 0u32;
                if equal(a1062, a1063) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1066 = {
                    let a1064 = v0;
                    let a1065 = 1u32;
                    fixed_u32::bit_and(a1064, a1065)
                };
                let a1067 = 0u32;
                if equal(a1066, a1067) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f183(v0: u32, v1: u32) -> u32 {
    let m1070 = {
        let a1068 = v0;
        let a1069 = v1;
        fixed_u32::shift_right(a1068, a1069)
    };
    match m1070 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f184(v0: Adt21, v1: Adt22) -> R<Adt21> {
    let v2: Adt23 = f210(Fn8::F211, 64u64, Adt23::C0(match v0.clone() {
        Adt21::C0(h1071, _, _, _, _, _, _, _) => {
            h1071
        }
    }, match v0.clone() {
        Adt21::C0(_, h1072, _, _, _, _, _, _) => {
            h1072
        }
    }, match v0.clone() {
        Adt21::C0(_, _, h1073, _, _, _, _, _) => {
            h1073
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, h1074, _, _, _, _) => {
            h1074
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, h1075, _, _, _) => {
            h1075
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1076, _, _) => {
            h1076
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1077, _) => {
            h1077
        }
    }, match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, _, h1078) => {
            h1078
        }
    }, v1, 0u64))?;
    Ok(Adt21::C0(f212(match v0.clone() {
        Adt21::C0(h1079, _, _, _, _, _, _, _) => {
            h1079
        }
    }, match v2.clone() {
        Adt23::C0(h1080, _, _, _, _, _, _, _, _, _) => {
            h1080
        }
    }), f212(match v0.clone() {
        Adt21::C0(_, h1081, _, _, _, _, _, _) => {
            h1081
        }
    }, match v2.clone() {
        Adt23::C0(_, h1082, _, _, _, _, _, _, _, _) => {
            h1082
        }
    }), f212(match v0.clone() {
        Adt21::C0(_, _, h1083, _, _, _, _, _) => {
            h1083
        }
    }, match v2.clone() {
        Adt23::C0(_, _, h1084, _, _, _, _, _, _, _) => {
            h1084
        }
    }), f212(match v0.clone() {
        Adt21::C0(_, _, _, h1085, _, _, _, _) => {
            h1085
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, h1086, _, _, _, _, _, _) => {
            h1086
        }
    }), f212(match v0.clone() {
        Adt21::C0(_, _, _, _, h1087, _, _, _) => {
            h1087
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, h1088, _, _, _, _, _) => {
            h1088
        }
    }), f212(match v0.clone() {
        Adt21::C0(_, _, _, _, _, h1089, _, _) => {
            h1089
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, h1090, _, _, _, _) => {
            h1090
        }
    }), f212(match v0.clone() {
        Adt21::C0(_, _, _, _, _, _, h1091, _) => {
            h1091
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, _, h1092, _, _, _) => {
            h1092
        }
    }), f212(match v0 {
        Adt21::C0(_, _, _, _, _, _, _, h1093) => {
            h1093
        }
    }, match v2 {
        Adt23::C0(_, _, _, _, _, _, _, h1094, _, _) => {
            h1094
        }
    })))
}

pub fn f185(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    Ok(Adt22::C0(f213(v0.clone(), v1, v2, v3)?, f213(v0.clone(), v1, v2, {
        let a1095 = v3;
        let a1096 = 4u64;
        nat_add(a1095, a1096)?
    })?, f213(v0.clone(), v1, v2, {
        let a1097 = v3;
        let a1098 = 8u64;
        nat_add(a1097, a1098)?
    })?, f213(v0.clone(), v1, v2, {
        let a1099 = v3;
        let a1100 = 12u64;
        nat_add(a1099, a1100)?
    })?, f213(v0.clone(), v1, v2, {
        let a1101 = v3;
        let a1102 = 16u64;
        nat_add(a1101, a1102)?
    })?, f213(v0.clone(), v1, v2, {
        let a1103 = v3;
        let a1104 = 20u64;
        nat_add(a1103, a1104)?
    })?, f213(v0.clone(), v1, v2, {
        let a1105 = v3;
        let a1106 = 24u64;
        nat_add(a1105, a1106)?
    })?, f213(v0.clone(), v1, v2, {
        let a1107 = v3;
        let a1108 = 28u64;
        nat_add(a1107, a1108)?
    })?, f213(v0.clone(), v1, v2, {
        let a1109 = v3;
        let a1110 = 32u64;
        nat_add(a1109, a1110)?
    })?, f213(v0.clone(), v1, v2, {
        let a1111 = v3;
        let a1112 = 36u64;
        nat_add(a1111, a1112)?
    })?, f213(v0.clone(), v1, v2, {
        let a1113 = v3;
        let a1114 = 40u64;
        nat_add(a1113, a1114)?
    })?, f213(v0.clone(), v1, v2, {
        let a1115 = v3;
        let a1116 = 44u64;
        nat_add(a1115, a1116)?
    })?, f213(v0.clone(), v1, v2, {
        let a1117 = v3;
        let a1118 = 48u64;
        nat_add(a1117, a1118)?
    })?, f213(v0.clone(), v1, v2, {
        let a1119 = v3;
        let a1120 = 52u64;
        nat_add(a1119, a1120)?
    })?, f213(v0.clone(), v1, v2, {
        let a1121 = v3;
        let a1122 = 56u64;
        nat_add(a1121, a1122)?
    })?, f213(v0, v1, v2, {
        let a1123 = v3;
        let a1124 = 60u64;
        nat_add(a1123, a1124)?
    })?))
}

pub fn f186(v0: Fn4, p1129: u64, p1130: u64) -> R<(u64, bool)> {
    let m1128 = (p1129, p1130);
    {
        let mut m1128 = m1128;
        loop {
            let step = {
                let (v1, v2) = m1128;
                let m1125 = v1;
                if m1125 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1125 - 1;
                    let m1127 = {
                        let c1126: Fn4 = v0.clone();
                        c1126.apply(v2)?
                    };
                    match m1127 {
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
                Ok(next) => { m1128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Bytes, v1: u64) -> Option<u64> {
    let a1131 = 0u64;
    let a1132 = v1;
    if nat_lt(a1131, a1132) {
        if f214(v0, {
            let a1133 = v1;
            let a1134 = 1u64;
            nat_sub(a1133, a1134)
        }) {
            Some({
                let a1135 = v1;
                let a1136 = 1u64;
                nat_sub(a1135, a1136)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f188(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f189(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1137 = v0.clone();
    match m1137.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1138 = v2;
            let a1139 = v1.clone();
            if equal(a1138, a1139) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f190(v0: List<(Bytes, ())>) -> List<Bytes> {
    f153(f215(Fn9::F216, List::<Bytes>::nil(), v0))
}

pub fn f191(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f217(f218(Fn10::F219, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1144 = {
        let (h1140, _) = f220(Fn11::F221, {
            let a1142 = {
                let a1141 = v0;
                length_list(a1141)
            };
            let a1143 = 1u64;
            nat_add(a1142, a1143)?
        }, v1)?;
        h1140
    };
    match m1144.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f192(v0: List<Bytes>) -> List<(Bytes, ())> {
    f222(f223(Fn12::F224, List::<(Bytes, ())>::nil(), v0))
}

pub fn f193(v0: Bytes, p1224: u64, p1225: u64, p1226: bool, p1227: bool, p1228: Bytes) -> R<Bytes> {
    let m1223 = (p1224, (p1225, (p1226, (p1227, p1228))));
    {
        let mut m1223 = m1223;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1223;
                let m1145 = v2;
                if m1145 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1145 - 1;
                    let m1148 = {
                        let a1146 = v0.clone();
                        let a1147 = v1;
                        index_bytes(a1146, a1147)
                    };
                    match m1148 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1193 = {
                                    let a1169 = {
                                        let a1157 = {
                                            let a1151 = {
                                                let a1149 = v8;
                                                let a1150 = 128u8;
                                                fixed_u8::bit_and(a1149, a1150)
                                            };
                                            let a1152 = 128u8;
                                            if equal(a1151, a1152) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1158 = {
                                            let a1155 = {
                                                let a1153 = v8;
                                                let a1154 = 64u8;
                                                fixed_u8::bit_and(a1153, a1154)
                                            };
                                            let a1156 = 64u8;
                                            if equal(a1155, a1156) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1157, a1158)?
                                    };
                                    let a1170 = {
                                        let a1167 = {
                                            let a1161 = {
                                                let a1159 = v8;
                                                let a1160 = 32u8;
                                                fixed_u8::bit_and(a1159, a1160)
                                            };
                                            let a1162 = 32u8;
                                            if equal(a1161, a1162) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1168 = {
                                            let a1165 = {
                                                let a1163 = v8;
                                                let a1164 = 16u8;
                                                fixed_u8::bit_and(a1163, a1164)
                                            };
                                            let a1166 = 16u8;
                                            if equal(a1165, a1166) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1167, a1168)?
                                    };
                                    nat_add(a1169, a1170)?
                                };
                                let a1194 = {
                                    let a1191 = {
                                        let a1179 = {
                                            let a1173 = {
                                                let a1171 = v8;
                                                let a1172 = 8u8;
                                                fixed_u8::bit_and(a1171, a1172)
                                            };
                                            let a1174 = 8u8;
                                            if equal(a1173, a1174) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1180 = {
                                            let a1177 = {
                                                let a1175 = v8;
                                                let a1176 = 4u8;
                                                fixed_u8::bit_and(a1175, a1176)
                                            };
                                            let a1178 = 4u8;
                                            if equal(a1177, a1178) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1179, a1180)?
                                    };
                                    let a1192 = {
                                        let a1189 = {
                                            let a1183 = {
                                                let a1181 = v8;
                                                let a1182 = 2u8;
                                                fixed_u8::bit_and(a1181, a1182)
                                            };
                                            let a1184 = 2u8;
                                            if equal(a1183, a1184) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1190 = {
                                            let a1187 = {
                                                let a1185 = v8;
                                                let a1186 = 1u8;
                                                fixed_u8::bit_and(a1185, a1186)
                                            };
                                            let a1188 = 1u8;
                                            if equal(a1187, a1188) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1189, a1190)?
                                    };
                                    nat_add(a1191, a1192)?
                                };
                                nat_add(a1193, a1194)?
                            };
                            let v10: Bytes = f38(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1195 = v1;
                                    let a1196 = 1u64;
                                    nat_add(a1195, a1196)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1197 = v9;
                                    let a1198 = 34u64;
                                    !nat_eq(a1197, a1198)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1199 = v9;
                                    let a1200 = 92u64;
                                    nat_eq(a1199, a1200)
                                }, {
                                    let a1201 = v5;
                                    let a1202 = v10;
                                    append_bytes(a1201, a1202)
                                })))))
                            } else {
                                let a1203 = v9;
                                let a1204 = 32u64;
                                if if if if nat_eq(a1203, a1204) {
                                    true
                                } else {
                                    let a1205 = v9;
                                    let a1206 = 9u64;
                                    nat_eq(a1205, a1206)
                                } {
                                    true
                                } else {
                                    let a1207 = v9;
                                    let a1208 = 10u64;
                                    nat_eq(a1207, a1208)
                                } {
                                    true
                                } else {
                                    let a1209 = v9;
                                    let a1210 = 13u64;
                                    nat_eq(a1209, a1210)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1211 = v1;
                                        let a1212 = 1u64;
                                        nat_add(a1211, a1212)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f26(f38(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1213 = v1;
                                            let a1214 = 4u64;
                                            nat_add(a1213, a1214)?
                                        }, (v6, (false, (false, {
                                            let a1215 = v5;
                                            let a1216 = f8(Str::lit("[]"));
                                            append_bytes(a1215, a1216)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1217 = v1;
                                            let a1218 = 1u64;
                                            nat_add(a1217, a1218)?
                                        }, (v6, ({
                                            let a1219 = v9;
                                            let a1220 = 34u64;
                                            nat_eq(a1219, a1220)
                                        }, (false, {
                                            let a1221 = v5;
                                            let a1222 = v10;
                                            append_bytes(a1221, a1222)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1223 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: Bytes, v1: u64) -> R<u64> {
    let m1231 = {
        let a1229 = v0;
        let a1230 = v1;
        index_bytes(a1229, a1230)
    };
    match m1231 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1276 = {
                let a1252 = {
                    let a1240 = {
                        let a1234 = {
                            let a1232 = v3;
                            let a1233 = 128u8;
                            fixed_u8::bit_and(a1232, a1233)
                        };
                        let a1235 = 128u8;
                        if equal(a1234, a1235) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1241 = {
                        let a1238 = {
                            let a1236 = v3;
                            let a1237 = 64u8;
                            fixed_u8::bit_and(a1236, a1237)
                        };
                        let a1239 = 64u8;
                        if equal(a1238, a1239) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1240, a1241)?
                };
                let a1253 = {
                    let a1250 = {
                        let a1244 = {
                            let a1242 = v3;
                            let a1243 = 32u8;
                            fixed_u8::bit_and(a1242, a1243)
                        };
                        let a1245 = 32u8;
                        if equal(a1244, a1245) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1251 = {
                        let a1248 = {
                            let a1246 = v3;
                            let a1247 = 16u8;
                            fixed_u8::bit_and(a1246, a1247)
                        };
                        let a1249 = 16u8;
                        if equal(a1248, a1249) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1250, a1251)?
                };
                nat_add(a1252, a1253)?
            };
            let a1277 = {
                let a1274 = {
                    let a1262 = {
                        let a1256 = {
                            let a1254 = v3;
                            let a1255 = 8u8;
                            fixed_u8::bit_and(a1254, a1255)
                        };
                        let a1257 = 8u8;
                        if equal(a1256, a1257) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1263 = {
                        let a1260 = {
                            let a1258 = v3;
                            let a1259 = 4u8;
                            fixed_u8::bit_and(a1258, a1259)
                        };
                        let a1261 = 4u8;
                        if equal(a1260, a1261) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1262, a1263)?
                };
                let a1275 = {
                    let a1272 = {
                        let a1266 = {
                            let a1264 = v3;
                            let a1265 = 2u8;
                            fixed_u8::bit_and(a1264, a1265)
                        };
                        let a1267 = 2u8;
                        if equal(a1266, a1267) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1273 = {
                        let a1270 = {
                            let a1268 = v3;
                            let a1269 = 1u8;
                            fixed_u8::bit_and(a1268, a1269)
                        };
                        let a1271 = 1u8;
                        if equal(a1270, a1271) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1272, a1273)?
                };
                nat_add(a1274, a1275)?
            };
            nat_add(a1276, a1277)
        }
    }
}

pub fn f195(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f225(Fn13::F226(v0.clone()), {
        let a1279 = {
            let a1278 = v0;
            length_bytes(a1278)
        };
        let a1280 = 1u64;
        nat_add(a1279, a1280)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1284 = {
        let (_, h1281) = {
            let (_, h1282) = {
                let (h1283, _) = v2.clone();
                h1283
            };
            h1282
        };
        h1281
    };
    let a1285 = 1u64;
    if nat_eq(a1284, a1285) {
        Ok(Some(({
            let (h1286, _) = {
                let (h1287, _) = v2.clone();
                h1287
            };
            h1286
        }, {
            let (h1288, _) = {
                let (_, h1289) = {
                    let (h1290, _) = v2;
                    h1290
                };
                h1289
            };
            h1288
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f196(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1291 = f194(v0.clone(), v1)?;
        let a1292 = 45u64;
        nat_eq(a1291, a1292)
    };
    let v3: u64 = if v2 {
        let a1293 = v1;
        let a1294 = 1u64;
        nat_add(a1293, a1294)?
    } else {
        v1
    };
    let v4: u64 = f227(v0.clone(), v3)?;
    let a1297 = 19u64;
    let a1298 = {
        let a1295 = v4;
        let a1296 = v3;
        nat_sub(a1295, a1296)
    };
    if nat_lt(a1297, a1298) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1301 = f228(f38(v0, v3, {
            let a1299 = v4;
            let a1300 = v3;
            nat_sub(a1299, a1300)
        }))?;
        match m1301 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f229(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f197(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1302 = v0;
    match m1302.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f230(v1.clone())?;
            if f231(v3.clone())? {
                let m1303 = f197(v2)?;
                match m1303 {
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

pub fn f198(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt17>), Bytes> {
    Err::<(Bytes, List<Adt17>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f199(v0: Bytes, v1: List<Adt17>) -> Result<(Bytes, List<Adt17>), Bytes> {
    Ok::<(Bytes, List<Adt17>), Bytes>((v0, v1))
}

pub fn f200(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt17>), Bytes>> {
    let m1304 = v1;
    match m1304.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt17>), Bytes>((List::<Bytes>::nil(), List::<Adt17>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f230({
                let (h1305, _) = v3.clone();
                h1305
            })?;
            if f231(v5.clone())? {
                let m1310 = f232({
                    let (_, h1306) = v3.clone();
                    h1306
                }, {
                    let a1308 = v0.clone();
                    let a1309 = f77({
                        let (h1307, _) = v3.clone();
                        h1307
                    })?;
                    append_bytes(a1308, a1309)
                })?;
                match m1310 {
                    Ok(v7) => {
                        let v8: Bytes = if f54({
                            let (h1311, _) = v3;
                            h1311
                        }, v2.clone()) {
                            let (h1312, _) = v7;
                            h1312
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1313, _) = v7;
                                h1313
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1314 = f200(v0, v4, v2)?;
                        match m1314 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt17>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1315, _) = v10;
                                    h1315
                                }), f121({
                                    let (_, h1316) = v7;
                                    h1316
                                }, {
                                    let (_, h1317) = v10;
                                    h1317
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
                    let (h1318, _) = v3;
                    h1318
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f201(v0: u64) -> R<Bytes> {
    f233(f234(), 10u64, v0)
}

pub fn f202(v0: Fn5, p1323: u64, p1324: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1322 = (p1323, p1324);
    {
        let mut m1322 = m1322;
        loop {
            let step = {
                let (v1, v2) = m1322;
                let m1319 = v1;
                if m1319 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1319 - 1;
                    let m1321 = {
                        let c1320: Fn5 = v0.clone();
                        c1320.apply(v2.clone())?
                    };
                    match m1321 {
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
                Ok(next) => { m1322 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f235(v0, v1)
}

pub fn f204(v0: Fn6, p1328: List<(Bytes, Bytes)>, p1329: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1327 = (p1328, p1329);
    {
        let mut m1327 = m1327;
        loop {
            let step = {
                let (v1, v2) = m1327;
                let m1325 = v2;
                match m1325.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1326: Fn6 = v0.clone();
                            c1326.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1327 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f236(v0, v1)
}

pub fn f206(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f237(v0)
}

pub fn f207(v0: Fn7, p1333: List<Bytes>, p1334: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1332 = (p1333, p1334);
    {
        let mut m1332 = m1332;
        loop {
            let step = {
                let (v1, v2) = m1332;
                let m1330 = v2;
                match m1330.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1331: Fn7 = v0.clone();
                            c1331.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1332 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f238(v1)?, v0))
}

pub fn f209(v0: Bytes) -> Bytes {
    if f88(v0.clone(), Bytes::lit(&[0x0d])) {
        f38(v0.clone(), 0u64, {
            let a1336 = {
                let a1335 = v0;
                length_bytes(a1335)
            };
            let a1337 = 1u64;
            nat_sub(a1336, a1337)
        })
    } else {
        v0
    }
}

pub fn f210(v0: Fn8, p1341: u64, p1342: Adt23) -> R<Adt23> {
    let m1340 = (p1341, p1342);
    {
        let mut m1340 = m1340;
        loop {
            let step = {
                let (v1, v2) = m1340;
                let m1338 = v1;
                if m1338 == 0 {
                    Err::<(u64, Adt23), R<Adt23>>(Ok(v2))
                } else {
                    let v3 = m1338 - 1;
                    Ok::<(u64, Adt23), R<Adt23>>((v3, {
                        let c1339: Fn8 = v0.clone();
                        c1339.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: Adt23) -> R<Adt23> {
    f239(v0)
}

pub fn f212(v0: u32, v1: u32) -> u32 {
    let m1345 = {
        let a1343 = f240(v0);
        let a1344 = f240(v1);
        fixed_u64::checked_add(a1343, a1344)
    };
    match m1345 {
        None => {
            0u32
        }
        Some(v2) => {
            f241(v2)
        }
    }
}

pub fn f213(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1348 = {
        let a1346 = v3;
        let a1347 = 8u64;
        nat_add(a1346, a1347)?
    };
    let a1349 = v2;
    if nat_eq(a1348, a1349) {
        Ok(f242({
            let a1352 = {
                let a1350 = v1;
                let a1351 = 8u64;
                nat_mul(a1350, a1351)?
            };
            let a1353 = 4294967296u64;
            let a1354 = 0u64;
            nat_quot(a1352, a1353, a1354)
        }))
    } else {
        let a1357 = {
            let a1355 = v3;
            let a1356 = 4u64;
            nat_add(a1355, a1356)?
        };
        let a1358 = v2;
        if nat_eq(a1357, a1358) {
            Ok(f242({
                let a1361 = {
                    let a1359 = v1;
                    let a1360 = 8u64;
                    nat_mul(a1359, a1360)?
                };
                let a1362 = 4294967296u64;
                let a1363 = 0u64;
                nat_rem(a1361, a1362, a1363)
            }))
        } else {
            Ok(f243(f244(v0.clone(), v1, v3), f244(v0.clone(), v1, {
                let a1364 = v3;
                let a1365 = 1u64;
                nat_add(a1364, a1365)?
            }), f244(v0.clone(), v1, {
                let a1366 = v3;
                let a1367 = 2u64;
                nat_add(a1366, a1367)?
            }), f244(v0, v1, {
                let a1368 = v3;
                let a1369 = 3u64;
                nat_add(a1368, a1369)?
            })))
        }
    }
}

pub fn f214(v0: Bytes, v1: u64) -> bool {
    let m1370 = f175(v0, v1);
    match m1370 {
        None => {
            false
        }
        Some(v2) => {
            f245(v2)
        }
    }
}

pub fn f215(v0: Fn9, p1374: List<Bytes>, p1375: List<(Bytes, ())>) -> List<Bytes> {
    let m1373 = (p1374, p1375);
    {
        let mut m1373 = m1373;
        loop {
            let step = {
                let (v1, v2) = m1373;
                let m1371 = v2;
                match m1371.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1372: Fn9 = v0.clone();
                            c1372.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1376, _) = v1;
        h1376
    }, v0)
}

pub fn f217(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f246(Fn14::F247, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f218(v0: Fn10, p1380: List<List<(Bytes, ())>>, p1381: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1379 = (p1380, p1381);
    {
        let mut m1379 = m1379;
        loop {
            let step = {
                let (v1, v2) = m1379;
                let m1377 = v2;
                match m1377.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1378: Fn10 = v0.clone();
                            c1378.apply(v1, v3)
                        }, v4))
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

pub fn f219(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f220(v0: Fn11, p1386: u64, p1387: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1385 = (p1386, p1387);
    {
        let mut m1385 = m1385;
        loop {
            let step = {
                let (v1, v2) = m1385;
                let m1382 = v1;
                if m1382 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1382 - 1;
                    let m1384 = {
                        let c1383: Fn11 = v0.clone();
                        c1383.apply(v2.clone())?
                    };
                    match m1384 {
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
                Ok(next) => { m1385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f221(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f248(v0)
}

pub fn f222(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f249(Fn15::F250, List::<(Bytes, ())>::nil(), v0)
}

pub fn f223(v0: Fn12, p1391: List<(Bytes, ())>, p1392: List<Bytes>) -> List<(Bytes, ())> {
    let m1390 = (p1391, p1392);
    {
        let mut m1390 = m1390;
        loop {
            let step = {
                let (v1, v2) = m1390;
                let m1388 = v2;
                match m1388.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1389: Fn12 = v0.clone();
                            c1389.apply(v1, v3)
                        }, v4))
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

pub fn f224(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f225(v0: Fn13, p1397: u64, p1398: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1396 = (p1397, p1398);
    {
        let mut m1396 = m1396;
        loop {
            let step = {
                let (v1, v2) = m1396;
                let m1393 = v1;
                if m1393 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1393 - 1;
                    let m1395 = {
                        let c1394: Fn13 = v0.clone();
                        c1394.apply(v2.clone())?
                    };
                    match m1395 {
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
                Ok(next) => { m1396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f251(v0, v1)
}

pub fn f227(v0: Bytes, v1: u64) -> R<u64> {
    let (h1399, _) = f186(Fn4::F252(v0.clone()), {
        let a1401 = {
            let a1400 = v0;
            length_bytes(a1400)
        };
        let a1402 = 1u64;
        nat_add(a1401, a1402)?
    }, v1)?;
    Ok(h1399)
}

pub fn f228(v0: Bytes) -> R<Option<u64>> {
    if f253(v0.clone())? {
        Ok(Some({
            let (_, h1403) = {
                let (h1404, _) = f254(Fn16::F255(v0.clone()), {
                    let a1405 = v0;
                    length_bytes(a1405)
                }, (0u64, 0u64))?;
                h1404
            };
            h1403
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f229(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1406 = 0u64;
        let a1407 = v1;
        if nat_lt(a1406, a1407) {
            let a1408 = v1;
            let a1409 = 9223372036854775808u64;
            nat_le(a1408, a1409)
        } else {
            false
        }
    } else {
        let a1410 = v1;
        let a1411 = 9223372036854775807u64;
        nat_le(a1410, a1411)
    }
}

pub fn f230(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f119(v0.clone(), 0u64, {
        let a1412 = v0;
        length_bytes(a1412)
    }, false, Bytes::lit(&[]))?;
    let m1415 = {
        let a1413 = v1.clone();
        let a1414 = 0u64;
        index_bytes(a1413, a1414)
    };
    match m1415 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1460 = {
                    let a1436 = {
                        let a1424 = {
                            let a1418 = {
                                let a1416 = v3;
                                let a1417 = 128u8;
                                fixed_u8::bit_and(a1416, a1417)
                            };
                            let a1419 = 128u8;
                            if equal(a1418, a1419) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1425 = {
                            let a1422 = {
                                let a1420 = v3;
                                let a1421 = 64u8;
                                fixed_u8::bit_and(a1420, a1421)
                            };
                            let a1423 = 64u8;
                            if equal(a1422, a1423) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1424, a1425)?
                    };
                    let a1437 = {
                        let a1434 = {
                            let a1428 = {
                                let a1426 = v3;
                                let a1427 = 32u8;
                                fixed_u8::bit_and(a1426, a1427)
                            };
                            let a1429 = 32u8;
                            if equal(a1428, a1429) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1435 = {
                            let a1432 = {
                                let a1430 = v3;
                                let a1431 = 16u8;
                                fixed_u8::bit_and(a1430, a1431)
                            };
                            let a1433 = 16u8;
                            if equal(a1432, a1433) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1434, a1435)?
                    };
                    nat_add(a1436, a1437)?
                };
                let a1461 = {
                    let a1458 = {
                        let a1446 = {
                            let a1440 = {
                                let a1438 = v3;
                                let a1439 = 8u8;
                                fixed_u8::bit_and(a1438, a1439)
                            };
                            let a1441 = 8u8;
                            if equal(a1440, a1441) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1447 = {
                            let a1444 = {
                                let a1442 = v3;
                                let a1443 = 4u8;
                                fixed_u8::bit_and(a1442, a1443)
                            };
                            let a1445 = 4u8;
                            if equal(a1444, a1445) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1446, a1447)?
                    };
                    let a1459 = {
                        let a1456 = {
                            let a1450 = {
                                let a1448 = v3;
                                let a1449 = 2u8;
                                fixed_u8::bit_and(a1448, a1449)
                            };
                            let a1451 = 2u8;
                            if equal(a1450, a1451) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1457 = {
                            let a1454 = {
                                let a1452 = v3;
                                let a1453 = 1u8;
                                fixed_u8::bit_and(a1452, a1453)
                            };
                            let a1455 = 1u8;
                            if equal(a1454, a1455) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1456, a1457)?
                    };
                    nat_add(a1458, a1459)?
                };
                nat_add(a1460, a1461)?
            };
            if f162(v4) {
                let a1468 = f38({
                    let a1462 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a1462)
                }, {
                    let a1463 = v4;
                    let a1464 = 65u64;
                    nat_sub(a1463, a1464)
                }, 1u64);
                let a1469 = f38(v1.clone(), 1u64, {
                    let a1466 = {
                        let a1465 = v1;
                        length_bytes(a1465)
                    };
                    let a1467 = 1u64;
                    nat_sub(a1466, a1467)
                });
                Ok(append_bytes(a1468, a1469))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f231(v0: Bytes) -> R<bool> {
    let m1472 = {
        let a1470 = v0;
        let a1471 = 0u64;
        index_bytes(a1470, a1471)
    };
    match m1472 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f161({
                let v2: u8 = v1;
                let a1517 = {
                    let a1493 = {
                        let a1481 = {
                            let a1475 = {
                                let a1473 = v2;
                                let a1474 = 128u8;
                                fixed_u8::bit_and(a1473, a1474)
                            };
                            let a1476 = 128u8;
                            if equal(a1475, a1476) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1482 = {
                            let a1479 = {
                                let a1477 = v2;
                                let a1478 = 64u8;
                                fixed_u8::bit_and(a1477, a1478)
                            };
                            let a1480 = 64u8;
                            if equal(a1479, a1480) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1481, a1482)?
                    };
                    let a1494 = {
                        let a1491 = {
                            let a1485 = {
                                let a1483 = v2;
                                let a1484 = 32u8;
                                fixed_u8::bit_and(a1483, a1484)
                            };
                            let a1486 = 32u8;
                            if equal(a1485, a1486) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1492 = {
                            let a1489 = {
                                let a1487 = v2;
                                let a1488 = 16u8;
                                fixed_u8::bit_and(a1487, a1488)
                            };
                            let a1490 = 16u8;
                            if equal(a1489, a1490) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1491, a1492)?
                    };
                    nat_add(a1493, a1494)?
                };
                let a1518 = {
                    let a1515 = {
                        let a1503 = {
                            let a1497 = {
                                let a1495 = v2;
                                let a1496 = 8u8;
                                fixed_u8::bit_and(a1495, a1496)
                            };
                            let a1498 = 8u8;
                            if equal(a1497, a1498) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1504 = {
                            let a1501 = {
                                let a1499 = v2;
                                let a1500 = 4u8;
                                fixed_u8::bit_and(a1499, a1500)
                            };
                            let a1502 = 4u8;
                            if equal(a1501, a1502) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1503, a1504)?
                    };
                    let a1516 = {
                        let a1513 = {
                            let a1507 = {
                                let a1505 = v2;
                                let a1506 = 2u8;
                                fixed_u8::bit_and(a1505, a1506)
                            };
                            let a1508 = 2u8;
                            if equal(a1507, a1508) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1514 = {
                            let a1511 = {
                                let a1509 = v2;
                                let a1510 = 1u8;
                                fixed_u8::bit_and(a1509, a1510)
                            };
                            let a1512 = 1u8;
                            if equal(a1511, a1512) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1513, a1514)?
                    };
                    nat_add(a1515, a1516)?
                };
                nat_add(a1517, a1518)?
            }) {
                Ok(true)
            } else {
                Ok(f162({
                    let v3: u8 = v1;
                    let a1563 = {
                        let a1539 = {
                            let a1527 = {
                                let a1521 = {
                                    let a1519 = v3;
                                    let a1520 = 128u8;
                                    fixed_u8::bit_and(a1519, a1520)
                                };
                                let a1522 = 128u8;
                                if equal(a1521, a1522) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1528 = {
                                let a1525 = {
                                    let a1523 = v3;
                                    let a1524 = 64u8;
                                    fixed_u8::bit_and(a1523, a1524)
                                };
                                let a1526 = 64u8;
                                if equal(a1525, a1526) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1527, a1528)?
                        };
                        let a1540 = {
                            let a1537 = {
                                let a1531 = {
                                    let a1529 = v3;
                                    let a1530 = 32u8;
                                    fixed_u8::bit_and(a1529, a1530)
                                };
                                let a1532 = 32u8;
                                if equal(a1531, a1532) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1538 = {
                                let a1535 = {
                                    let a1533 = v3;
                                    let a1534 = 16u8;
                                    fixed_u8::bit_and(a1533, a1534)
                                };
                                let a1536 = 16u8;
                                if equal(a1535, a1536) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1537, a1538)?
                        };
                        nat_add(a1539, a1540)?
                    };
                    let a1564 = {
                        let a1561 = {
                            let a1549 = {
                                let a1543 = {
                                    let a1541 = v3;
                                    let a1542 = 8u8;
                                    fixed_u8::bit_and(a1541, a1542)
                                };
                                let a1544 = 8u8;
                                if equal(a1543, a1544) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1550 = {
                                let a1547 = {
                                    let a1545 = v3;
                                    let a1546 = 4u8;
                                    fixed_u8::bit_and(a1545, a1546)
                                };
                                let a1548 = 4u8;
                                if equal(a1547, a1548) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1549, a1550)?
                        };
                        let a1562 = {
                            let a1559 = {
                                let a1553 = {
                                    let a1551 = v3;
                                    let a1552 = 2u8;
                                    fixed_u8::bit_and(a1551, a1552)
                                };
                                let a1554 = 2u8;
                                if equal(a1553, a1554) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1560 = {
                                let a1557 = {
                                    let a1555 = v3;
                                    let a1556 = 1u8;
                                    fixed_u8::bit_and(a1555, a1556)
                                };
                                let a1558 = 1u8;
                                if equal(a1557, a1558) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1559, a1560)?
                        };
                        nat_add(a1561, a1562)?
                    };
                    nat_add(a1563, a1564)?
                }))
            }
        }
    }
}

pub fn f232(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1565 = f256(v0.clone());
    match m1565 {
        None => {
            if f26(f69(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1566 = f71(v0, f8(Str::lit("items")));
                match m1566 {
                    None => {
                        Ok(f198(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f257(f8(Str::lit("List")), f258(v3, {
                            let a1567 = v1;
                            let a1568 = f8(Str::lit("Item"));
                            append_bytes(a1567, a1568)
                        })?))
                    }
                }
            } else {
                f258(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f257(f8(Str::lit("Option")), f258(v2, v1)?))
        }
    }
}

pub fn f233(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1569 = v2;
    let a1570 = 0u64;
    if nat_eq(a1569, a1570) {
        Ok(f259(v0, 0u64))
    } else {
        let (_, h1571) = {
            let (h1572, _) = f202(Fn5::F260(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1572
        };
        Ok(h1571)
    }
}

pub fn f234() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f235(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1576 = {
        let a1574 = v0.clone();
        let a1575 = {
            let (h1573, _) = v1.clone();
            h1573
        };
        index_bytes(a1574, a1575)
    };
    match m1576 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1578 = {
                    let (h1577, _) = v1.clone();
                    h1577
                };
                let a1579 = 1u64;
                nat_add(a1578, a1579)?
            }, {
                let a1582 = {
                    let (_, h1580) = v1;
                    h1580
                };
                let a1583 = f261(v0, {
                    let (h1581, _) = v1;
                    h1581
                }, v2)?;
                append_bytes(a1582, a1583)
            })))
        }
    }
}

pub fn f236(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1584 = v0.clone();
    match m1584.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1587 = {
                let (h1585, _) = v2;
                h1585
            };
            let a1588 = {
                let (h1586, _) = v1.clone();
                h1586
            };
            if equal(a1587, a1588) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f237(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f262(f263(Fn17::F264, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1593 = {
        let (h1589, _) = f265(Fn18::F266, {
            let a1591 = {
                let a1590 = v0;
                length_list(a1590)
            };
            let a1592 = 1u64;
            nat_add(a1591, a1592)?
        }, v1)?;
        h1589
    };
    match m1593.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f238(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1598 = {
        let a1595 = f167({
            let (h1594, _) = v0;
            h1594
        })?;
        let a1596 = Bytes::lit(&[0x3a]);
        append_bytes(a1595, a1596)
    };
    let a1599 = {
        let (_, h1597) = v0;
        h1597
    };
    Ok(append_bytes(a1598, a1599))
}

pub fn f239(v0: Adt23) -> R<Adt23> {
    let v1: u32 = f267(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, _, h1600) => {
            h1600
        }
    });
    let v2: u32 = f212(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h1601, _, _) => {
            h1601
        }
    }, f212(f268(match v0.clone() {
        Adt23::C0(_, _, _, _, h1602, _, _, _, _, _) => {
            h1602
        }
    }), f212(f269(match v0.clone() {
        Adt23::C0(_, _, _, _, h1603, _, _, _, _, _) => {
            h1603
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1604, _, _, _, _) => {
            h1604
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1605, _, _, _) => {
            h1605
        }
    }), f212(v1, {
        let m1608 = match v0.clone() {
            Adt23::C0(_, _, _, _, _, _, _, _, h1607, _) => {
                h1607
            }
        };
        match m1608 {
            Adt22::C0(h1606, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1606
            }
        }
    }))));
    let v3: u32 = f212(f270(match v0.clone() {
        Adt23::C0(h1609, _, _, _, _, _, _, _, _, _) => {
            h1609
        }
    }), f271(match v0.clone() {
        Adt23::C0(h1610, _, _, _, _, _, _, _, _, _) => {
            h1610
        }
    }, match v0.clone() {
        Adt23::C0(_, h1611, _, _, _, _, _, _, _, _) => {
            h1611
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1612, _, _, _, _, _, _, _) => {
            h1612
        }
    }));
    Ok(Adt23::C0(f212(v2, v3), match v0.clone() {
        Adt23::C0(h1613, _, _, _, _, _, _, _, _, _) => {
            h1613
        }
    }, match v0.clone() {
        Adt23::C0(_, h1614, _, _, _, _, _, _, _, _) => {
            h1614
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1615, _, _, _, _, _, _, _) => {
            h1615
        }
    }, f212(match v0.clone() {
        Adt23::C0(_, _, _, h1616, _, _, _, _, _, _) => {
            h1616
        }
    }, v2), match v0.clone() {
        Adt23::C0(_, _, _, _, h1617, _, _, _, _, _) => {
            h1617
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1618, _, _, _, _) => {
            h1618
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1619, _, _, _) => {
            h1619
        }
    }, f272(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, h1620, _) => {
            h1620
        }
    }), {
        let a1622 = match v0 {
            Adt23::C0(_, _, _, _, _, _, _, _, _, h1621) => {
                h1621
            }
        };
        let a1623 = 1u64;
        nat_add(a1622, a1623)?
    }))
}

pub fn f240(v0: u32) -> u64 {
    let m1625 = {
        let a1624 = v0;
        fixed_u64::convert(i128::from(a1624))
    };
    match m1625 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f241(v0: u64) -> u32 {
    let m1629 = {
        let a1628 = {
            let a1626 = v0;
            let a1627 = 4294967295u64;
            fixed_u64::bit_and(a1626, a1627)
        };
        fixed_u32::convert(i128::from(a1628))
    };
    match m1629 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f242(v0: u64) -> u32 {
    let (_, h1630) = {
        let (_, h1631) = f273(Fn19::F274, 32u64, (v0, (1u32, 0u32)));
        h1631
    };
    h1630
}

pub fn f243(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1636 = f275(v0, 24u32);
    let a1637 = {
        let a1634 = f275(v1, 16u32);
        let a1635 = {
            let a1632 = f275(v2, 8u32);
            let a1633 = v3;
            fixed_u32::bit_or(a1632, a1633)
        };
        fixed_u32::bit_or(a1634, a1635)
    };
    fixed_u32::bit_or(a1636, a1637)
}

pub fn f244(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1638 = v2;
    let a1639 = v1;
    if nat_lt(a1638, a1639) {
        let m1642 = {
            let a1640 = v0;
            let a1641 = v2;
            index_bytes(a1640, a1641)
        };
        match m1642 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1644 = {
                    let a1643 = v3;
                    fixed_u32::convert(i128::from(a1643))
                };
                match m1644 {
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
        let a1645 = v2;
        let a1646 = v1;
        if nat_eq(a1645, a1646) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f245(v0: u8) -> bool {
    let a1647 = v0;
    let a1648 = 32u8;
    if if if equal(a1647, a1648) {
        true
    } else {
        let a1649 = v0;
        let a1650 = 9u8;
        equal(a1649, a1650)
    } {
        true
    } else {
        let a1651 = v0;
        let a1652 = 10u8;
        equal(a1651, a1652)
    } {
        true
    } else {
        let a1653 = v0;
        let a1654 = 12u8;
        if equal(a1653, a1654) {
            true
        } else {
            let a1655 = v0;
            let a1656 = 13u8;
            equal(a1655, a1656)
        }
    }
}

pub fn f246(v0: Fn14, p1660: List<List<(Bytes, ())>>, p1661: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1659 = (p1660, p1661);
    {
        let mut m1659 = m1659;
        loop {
            let step = {
                let (v1, v2) = m1659;
                let m1657 = v2;
                match m1657.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1658: Fn14 = v0.clone();
                            c1658.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1659 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f247(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f248(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1663 = {
        let a1662 = v0.clone();
        length_list(a1662)
    };
    let a1664 = 1u64;
    if nat_le(a1663, a1664) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f276(v0)?))
    }
}

pub fn f249(v0: Fn15, p1668: List<(Bytes, ())>, p1669: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1667 = (p1668, p1669);
    {
        let mut m1667 = m1667;
        loop {
            let step = {
                let (v1, v2) = m1667;
                let m1665 = v2;
                match m1665.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1666: Fn15 = v0.clone();
                            c1666.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1667 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f251(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1670, _) = v1.clone();
        h1670
    };
    let v3: Bytes = {
        let (h1671, _) = {
            let (_, h1672) = v1.clone();
            h1672
        };
        h1671
    };
    let a1675 = {
        let (_, h1673) = {
            let (_, h1674) = v1;
            h1674
        };
        h1673
    };
    let a1676 = 0u64;
    if nat_eq(a1675, a1676) {
        let v4: u64 = f194(v0.clone(), v2)?;
        let a1677 = v4;
        let a1678 = 34u64;
        if nat_eq(a1677, a1678) {
            Ok(Some(({
                let a1679 = v2;
                let a1680 = 1u64;
                nat_add(a1679, a1680)?
            }, (v3, 1u64))))
        } else {
            let a1681 = v4;
            let a1682 = 92u64;
            if nat_eq(a1681, a1682) {
                f277(v0, v2, v3)
            } else {
                let a1683 = v4;
                let a1684 = 32u64;
                if if nat_lt(a1683, a1684) {
                    true
                } else {
                    let a1685 = v4;
                    let a1686 = 256u64;
                    nat_eq(a1685, a1686)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1687 = v2;
                        let a1688 = 1u64;
                        nat_add(a1687, a1688)?
                    }, ({
                        let a1689 = v3;
                        let a1690 = f38(v0, v2, 1u64);
                        append_bytes(a1689, a1690)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f252(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f278(v0, v1)
}

pub fn f253(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1691 = v0.clone();
        length_bytes(a1691)
    };
    let a1692 = 0u64;
    let a1693 = v1;
    if nat_lt(a1692, a1693) {
        let a1694 = f279(v0.clone())?;
        let a1695 = v1;
        if nat_eq(a1694, a1695) {
            let a1696 = 1u64;
            let a1697 = v1;
            if nat_lt(a1696, a1697) {
                let m1700 = {
                    let a1698 = v0;
                    let a1699 = 0u64;
                    index_bytes(a1698, a1699)
                };
                match m1700 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1701 = v2;
                        let a1702 = 48u8;
                        Ok(!equal(a1701, a1702))
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

pub fn f254(v0: Fn16, p1707: u64, p1708: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1706 = (p1707, p1708);
    {
        let mut m1706 = m1706;
        loop {
            let step = {
                let (v1, v2) = m1706;
                let m1703 = v1;
                if m1703 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1703 - 1;
                    let m1705 = {
                        let c1704: Fn16 = v0.clone();
                        c1704.apply(v2)?
                    };
                    match m1705 {
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
                Ok(next) => { m1706 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f280(v0, v1)
}

pub fn f256(v0: Adt14) -> Option<Adt14> {
    let m1709 = f102(f71(v0, f8(Str::lit("oneOf"))));
    match m1709.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m1710 = v2;
            match m1710.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m1711 = v4;
                    match m1711.uncons() {
                        None => {
                            if f281(v3.clone()) {
                                Some(v1)
                            } else {
                                if f281(v1) {
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

pub fn f257(v0: Bytes, v1: Result<(Bytes, List<Adt17>), Bytes>) -> Result<(Bytes, List<Adt17>), Bytes> {
    let m1712 = v1;
    match m1712 {
        Ok(v3) => {
            f199(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h1713, _) = v3;
                h1713
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h1714) = v3;
                h1714
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt17>), Bytes>(v2)
        }
    }
}

pub fn f258(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m1715 = f256(v0.clone());
    match m1715 {
        None => {
            if f26(f69(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1716 = f71(v0, f8(Str::lit("items")));
                match m1716 {
                    None => {
                        Ok(f198(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f257(f8(Str::lit("List")), f282(v3, {
                            let a1717 = v1;
                            let a1718 = f8(Str::lit("Item"));
                            append_bytes(a1717, a1718)
                        })?))
                    }
                }
            } else {
                f282(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f257(f8(Str::lit("Option")), f282(v2, v1)?))
        }
    }
}

pub fn f259(v0: Bytes, v1: u64) -> Bytes {
    let m1722 = {
        let a1719 = v0;
        let a1720 = v1;
        let a1721 = 1u64;
        slice_bytes(a1719, a1720, a1721)
    };
    match m1722 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f260(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f283(v1, v0, v2)
}

pub fn f261(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a1767 = {
            let a1743 = {
                let a1731 = {
                    let a1725 = {
                        let a1723 = v3;
                        let a1724 = 128u8;
                        fixed_u8::bit_and(a1723, a1724)
                    };
                    let a1726 = 128u8;
                    if equal(a1725, a1726) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1732 = {
                    let a1729 = {
                        let a1727 = v3;
                        let a1728 = 64u8;
                        fixed_u8::bit_and(a1727, a1728)
                    };
                    let a1730 = 64u8;
                    if equal(a1729, a1730) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1731, a1732)?
            };
            let a1744 = {
                let a1741 = {
                    let a1735 = {
                        let a1733 = v3;
                        let a1734 = 32u8;
                        fixed_u8::bit_and(a1733, a1734)
                    };
                    let a1736 = 32u8;
                    if equal(a1735, a1736) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1742 = {
                    let a1739 = {
                        let a1737 = v3;
                        let a1738 = 16u8;
                        fixed_u8::bit_and(a1737, a1738)
                    };
                    let a1740 = 16u8;
                    if equal(a1739, a1740) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1741, a1742)?
            };
            nat_add(a1743, a1744)?
        };
        let a1768 = {
            let a1765 = {
                let a1753 = {
                    let a1747 = {
                        let a1745 = v3;
                        let a1746 = 8u8;
                        fixed_u8::bit_and(a1745, a1746)
                    };
                    let a1748 = 8u8;
                    if equal(a1747, a1748) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1754 = {
                    let a1751 = {
                        let a1749 = v3;
                        let a1750 = 4u8;
                        fixed_u8::bit_and(a1749, a1750)
                    };
                    let a1752 = 4u8;
                    if equal(a1751, a1752) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1753, a1754)?
            };
            let a1766 = {
                let a1763 = {
                    let a1757 = {
                        let a1755 = v3;
                        let a1756 = 2u8;
                        fixed_u8::bit_and(a1755, a1756)
                    };
                    let a1758 = 2u8;
                    if equal(a1757, a1758) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1764 = {
                    let a1761 = {
                        let a1759 = v3;
                        let a1760 = 1u8;
                        fixed_u8::bit_and(a1759, a1760)
                    };
                    let a1762 = 1u8;
                    if equal(a1761, a1762) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1763, a1764)?
            };
            nat_add(a1765, a1766)?
        };
        nat_add(a1767, a1768)?
    };
    let a1769 = v4;
    let a1770 = 34u64;
    if nat_eq(a1769, a1770) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a1771 = v4;
        let a1772 = 92u64;
        if nat_eq(a1771, a1772) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a1773 = v4;
            let a1774 = 8u64;
            if nat_eq(a1773, a1774) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a1775 = v4;
                let a1776 = 12u64;
                if nat_eq(a1775, a1776) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a1777 = v4;
                    let a1778 = 10u64;
                    if nat_eq(a1777, a1778) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a1779 = v4;
                        let a1780 = 13u64;
                        if nat_eq(a1779, a1780) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a1781 = v4;
                            let a1782 = 9u64;
                            if nat_eq(a1781, a1782) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a1783 = v4;
                                let a1784 = 32u64;
                                if nat_lt(a1783, a1784) {
                                    let a1785 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a1786 = f284(v2)?;
                                    Ok(append_bytes(a1785, a1786))
                                } else {
                                    Ok(f38(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f262(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f285(Fn20::F286, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f263(v0: Fn17, p1790: List<List<(Bytes, Bytes)>>, p1791: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m1789 = (p1790, p1791);
    {
        let mut m1789 = m1789;
        loop {
            let step = {
                let (v1, v2) = m1789;
                let m1787 = v2;
                match m1787.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c1788: Fn17 = v0.clone();
                            c1788.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1789 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f265(v0: Fn18, p1796: u64, p1797: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m1795 = (p1796, p1797);
    {
        let mut m1795 = m1795;
        loop {
            let step = {
                let (v1, v2) = m1795;
                let m1792 = v1;
                if m1792 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1792 - 1;
                    let m1794 = {
                        let c1793: Fn18 = v0.clone();
                        c1793.apply(v2.clone())?
                    };
                    match m1794 {
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
                Ok(next) => { m1795 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f266(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f287(v0)
}

pub fn f267(v0: u64) -> u32 {
    let a1798 = v0;
    let a1799 = 32u64;
    if nat_lt(a1798, a1799) {
        let a1800 = v0;
        let a1801 = 16u64;
        if nat_lt(a1800, a1801) {
            let a1802 = v0;
            let a1803 = 8u64;
            if nat_lt(a1802, a1803) {
                let a1804 = v0;
                let a1805 = 4u64;
                if nat_lt(a1804, a1805) {
                    let a1806 = v0;
                    let a1807 = 2u64;
                    if nat_lt(a1806, a1807) {
                        let a1808 = v0;
                        let a1809 = 1u64;
                        if nat_lt(a1808, a1809) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1810 = v0;
                        let a1811 = 3u64;
                        if nat_lt(a1810, a1811) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1812 = v0;
                    let a1813 = 6u64;
                    if nat_lt(a1812, a1813) {
                        let a1814 = v0;
                        let a1815 = 5u64;
                        if nat_lt(a1814, a1815) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1816 = v0;
                        let a1817 = 7u64;
                        if nat_lt(a1816, a1817) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1818 = v0;
                let a1819 = 12u64;
                if nat_lt(a1818, a1819) {
                    let a1820 = v0;
                    let a1821 = 10u64;
                    if nat_lt(a1820, a1821) {
                        let a1822 = v0;
                        let a1823 = 9u64;
                        if nat_lt(a1822, a1823) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1824 = v0;
                        let a1825 = 11u64;
                        if nat_lt(a1824, a1825) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1826 = v0;
                    let a1827 = 14u64;
                    if nat_lt(a1826, a1827) {
                        let a1828 = v0;
                        let a1829 = 13u64;
                        if nat_lt(a1828, a1829) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1830 = v0;
                        let a1831 = 15u64;
                        if nat_lt(a1830, a1831) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1832 = v0;
            let a1833 = 24u64;
            if nat_lt(a1832, a1833) {
                let a1834 = v0;
                let a1835 = 20u64;
                if nat_lt(a1834, a1835) {
                    let a1836 = v0;
                    let a1837 = 18u64;
                    if nat_lt(a1836, a1837) {
                        let a1838 = v0;
                        let a1839 = 17u64;
                        if nat_lt(a1838, a1839) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1840 = v0;
                        let a1841 = 19u64;
                        if nat_lt(a1840, a1841) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1842 = v0;
                    let a1843 = 22u64;
                    if nat_lt(a1842, a1843) {
                        let a1844 = v0;
                        let a1845 = 21u64;
                        if nat_lt(a1844, a1845) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1846 = v0;
                        let a1847 = 23u64;
                        if nat_lt(a1846, a1847) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1848 = v0;
                let a1849 = 28u64;
                if nat_lt(a1848, a1849) {
                    let a1850 = v0;
                    let a1851 = 26u64;
                    if nat_lt(a1850, a1851) {
                        let a1852 = v0;
                        let a1853 = 25u64;
                        if nat_lt(a1852, a1853) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1854 = v0;
                        let a1855 = 27u64;
                        if nat_lt(a1854, a1855) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1856 = v0;
                    let a1857 = 30u64;
                    if nat_lt(a1856, a1857) {
                        let a1858 = v0;
                        let a1859 = 29u64;
                        if nat_lt(a1858, a1859) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1860 = v0;
                        let a1861 = 31u64;
                        if nat_lt(a1860, a1861) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1862 = v0;
        let a1863 = 48u64;
        if nat_lt(a1862, a1863) {
            let a1864 = v0;
            let a1865 = 40u64;
            if nat_lt(a1864, a1865) {
                let a1866 = v0;
                let a1867 = 36u64;
                if nat_lt(a1866, a1867) {
                    let a1868 = v0;
                    let a1869 = 34u64;
                    if nat_lt(a1868, a1869) {
                        let a1870 = v0;
                        let a1871 = 33u64;
                        if nat_lt(a1870, a1871) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a1872 = v0;
                        let a1873 = 35u64;
                        if nat_lt(a1872, a1873) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a1874 = v0;
                    let a1875 = 38u64;
                    if nat_lt(a1874, a1875) {
                        let a1876 = v0;
                        let a1877 = 37u64;
                        if nat_lt(a1876, a1877) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a1878 = v0;
                        let a1879 = 39u64;
                        if nat_lt(a1878, a1879) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a1880 = v0;
                let a1881 = 44u64;
                if nat_lt(a1880, a1881) {
                    let a1882 = v0;
                    let a1883 = 42u64;
                    if nat_lt(a1882, a1883) {
                        let a1884 = v0;
                        let a1885 = 41u64;
                        if nat_lt(a1884, a1885) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a1886 = v0;
                        let a1887 = 43u64;
                        if nat_lt(a1886, a1887) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a1888 = v0;
                    let a1889 = 46u64;
                    if nat_lt(a1888, a1889) {
                        let a1890 = v0;
                        let a1891 = 45u64;
                        if nat_lt(a1890, a1891) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a1892 = v0;
                        let a1893 = 47u64;
                        if nat_lt(a1892, a1893) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a1894 = v0;
            let a1895 = 56u64;
            if nat_lt(a1894, a1895) {
                let a1896 = v0;
                let a1897 = 52u64;
                if nat_lt(a1896, a1897) {
                    let a1898 = v0;
                    let a1899 = 50u64;
                    if nat_lt(a1898, a1899) {
                        let a1900 = v0;
                        let a1901 = 49u64;
                        if nat_lt(a1900, a1901) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a1902 = v0;
                        let a1903 = 51u64;
                        if nat_lt(a1902, a1903) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a1904 = v0;
                    let a1905 = 54u64;
                    if nat_lt(a1904, a1905) {
                        let a1906 = v0;
                        let a1907 = 53u64;
                        if nat_lt(a1906, a1907) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a1908 = v0;
                        let a1909 = 55u64;
                        if nat_lt(a1908, a1909) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a1910 = v0;
                let a1911 = 60u64;
                if nat_lt(a1910, a1911) {
                    let a1912 = v0;
                    let a1913 = 58u64;
                    if nat_lt(a1912, a1913) {
                        let a1914 = v0;
                        let a1915 = 57u64;
                        if nat_lt(a1914, a1915) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a1916 = v0;
                        let a1917 = 59u64;
                        if nat_lt(a1916, a1917) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a1918 = v0;
                    let a1919 = 62u64;
                    if nat_lt(a1918, a1919) {
                        let a1920 = v0;
                        let a1921 = 61u64;
                        if nat_lt(a1920, a1921) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a1922 = v0;
                        let a1923 = 63u64;
                        if nat_lt(a1922, a1923) {
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

pub fn f268(v0: u32) -> u32 {
    let a1926 = {
        let a1924 = f288(v0, 6u32, 26u32);
        let a1925 = f288(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a1924, a1925)
    };
    let a1927 = f288(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a1926, a1927)
}

pub fn f269(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1933 = {
        let a1928 = v0;
        let a1929 = v1;
        fixed_u32::bit_and(a1928, a1929)
    };
    let a1934 = {
        let a1931 = {
            let a1930 = v0;
            fixed_u32::bit_not(a1930)
        };
        let a1932 = v2;
        fixed_u32::bit_and(a1931, a1932)
    };
    fixed_u32::bit_xor(a1933, a1934)
}

pub fn f270(v0: u32) -> u32 {
    let a1937 = {
        let a1935 = f288(v0, 2u32, 30u32);
        let a1936 = f288(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a1935, a1936)
    };
    let a1938 = f288(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a1937, a1938)
}

pub fn f271(v0: u32, v1: u32, v2: u32) -> u32 {
    let a1947 = {
        let a1943 = {
            let a1939 = v0;
            let a1940 = v1;
            fixed_u32::bit_and(a1939, a1940)
        };
        let a1944 = {
            let a1941 = v0;
            let a1942 = v2;
            fixed_u32::bit_and(a1941, a1942)
        };
        fixed_u32::bit_xor(a1943, a1944)
    };
    let a1948 = {
        let a1945 = v1;
        let a1946 = v2;
        fixed_u32::bit_and(a1945, a1946)
    };
    fixed_u32::bit_xor(a1947, a1948)
}

pub fn f272(v0: Adt22) -> Adt22 {
    Adt22::C0(match v0.clone() {
        Adt22::C0(_, h1949, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1949
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h1950, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1950
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h1951, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1951
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h1952, _, _, _, _, _, _, _, _, _, _, _) => {
            h1952
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1953, _, _, _, _, _, _, _, _, _, _) => {
            h1953
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1954, _, _, _, _, _, _, _, _, _) => {
            h1954
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h1955, _, _, _, _, _, _, _, _) => {
            h1955
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h1956, _, _, _, _, _, _, _) => {
            h1956
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h1957, _, _, _, _, _, _) => {
            h1957
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, h1958, _, _, _, _, _) => {
            h1958
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, h1959, _, _, _, _) => {
            h1959
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1960, _, _, _) => {
            h1960
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1961, _, _) => {
            h1961
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1962, _) => {
            h1962
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h1963) => {
            h1963
        }
    }, f212(f289(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1964, _) => {
            h1964
        }
    }), f212(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h1965, _, _, _, _, _, _) => {
            h1965
        }
    }, f212(f290(match v0.clone() {
        Adt22::C0(_, h1966, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1966
        }
    }), match v0 {
        Adt22::C0(h1967, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h1967
        }
    }))))
}

pub fn f273(v0: Fn19, p1971: u64, p1972: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m1970 = (p1971, p1972);
    {
        let mut m1970 = m1970;
        loop {
            let step = {
                let (v1, v2) = m1970;
                let m1968 = v1;
                if m1968 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m1968 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c1969: Fn19 = v0.clone();
                        c1969.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m1970 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f274(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h1973, _) = v0;
        h1973
    };
    let v2: u32 = {
        let (h1974, _) = {
            let (_, h1975) = v0;
            h1975
        };
        h1974
    };
    let v3: u32 = {
        let (_, h1976) = {
            let (_, h1977) = v0;
            h1977
        };
        h1976
    };
    ({
        let a1978 = v1;
        let a1979 = 2u64;
        let a1980 = 0u64;
        nat_quot(a1978, a1979, a1980)
    }, (f275(v2, 1u32), {
        let a1984 = {
            let a1981 = v1;
            let a1982 = 2u64;
            let a1983 = 0u64;
            nat_rem(a1981, a1982, a1983)
        };
        let a1985 = 1u64;
        if nat_eq(a1984, a1985) {
            let a1986 = v3;
            let a1987 = v2;
            fixed_u32::bit_or(a1986, a1987)
        } else {
            v3
        }
    }))
}

pub fn f275(v0: u32, v1: u32) -> u32 {
    let m1990 = {
        let a1988 = v0;
        let a1989 = v1;
        fixed_u32::shift_left(a1988, a1989)
    };
    match m1990 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f276(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f217({
        let (_, h1991) = {
            let (h1992, _) = f291(Fn21::F292, {
                let a1994 = {
                    let a1993 = v0.clone();
                    length_list(a1993)
                };
                let a1995 = 1u64;
                nat_add(a1994, a1995)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1992
        };
        h1991
    }))
}

pub fn f277(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f194(v0.clone(), {
        let a1996 = v1;
        let a1997 = 1u64;
        nat_add(a1996, a1997)?
    })?;
    let v4: Bytes = {
        let a1998 = v3;
        let a1999 = 117u64;
        if nat_eq(a1998, a1999) {
            f293(v0, v1)?
        } else {
            f294(v3)
        }
    };
    let a2000 = v4.clone();
    let a2001 = Bytes::lit(&[]);
    if equal(a2000, a2001) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2002 = v3;
            let a2003 = 117u64;
            if nat_eq(a2002, a2003) {
                let a2004 = v1;
                let a2005 = 6u64;
                nat_add(a2004, a2005)?
            } else {
                let a2006 = v1;
                let a2007 = 2u64;
                nat_add(a2006, a2007)?
            }
        }, ({
            let a2008 = v2;
            let a2009 = v4;
            append_bytes(a2008, a2009)
        }, 0u64))))
    }
}

pub fn f278(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f194(v0, v1)?;
    let a2010 = 47u64;
    let a2011 = v2;
    if if nat_lt(a2010, a2011) {
        let a2012 = v2;
        let a2013 = 58u64;
        nat_lt(a2012, a2013)
    } else {
        false
    } {
        Ok(Some({
            let a2014 = v1;
            let a2015 = 1u64;
            nat_add(a2014, a2015)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f279(v0: Bytes) -> R<u64> {
    let (h2016, _) = f186(Fn4::F295(v0.clone()), {
        let a2017 = v0;
        length_bytes(a2017)
    }, 0u64)?;
    Ok(h2016)
}

pub fn f280(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2021 = {
        let a2019 = v0;
        let a2020 = {
            let (h2018, _) = v1;
            h2018
        };
        index_bytes(a2019, a2020)
    };
    match m2021 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2023 = {
                    let (h2022, _) = v1;
                    h2022
                };
                let a2024 = 1u64;
                nat_add(a2023, a2024)?
            }, {
                let a2028 = {
                    let a2026 = {
                        let (_, h2025) = v1;
                        h2025
                    };
                    let a2027 = 10u64;
                    nat_mul(a2026, a2027)?
                };
                let a2029 = f296(v2)?;
                nat_add(a2028, a2029)?
            })))
        }
    }
}

pub fn f281(v0: Adt14) -> bool {
    f26(f69(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f282(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt17>), Bytes>> {
    let m2030 = f297(f165(v0.clone()));
    match m2030 {
        None => {
            let m2031 = f72(f71(v0.clone(), f8(Str::lit("$ref"))));
            match m2031 {
                None => {
                    let m2033 = f71(v0.clone(), f8(Str::lit("enum")));
                    match m2033 {
                        None => {
                            let m2034 = f71(v0.clone(), f8(Str::lit("const")));
                            match m2034 {
                                None => {
                                    let v8: Bytes = f69(v0.clone(), f8(Str::lit("type")));
                                    if f26(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f199(f8(Str::lit("Bytes")), List::<Adt17>::nil()))
                                    } else {
                                        if f26(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f199(f299(v0), List::<Adt17>::nil()))
                                        } else {
                                            if f26(v8.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f199(f8(Str::lit("Bool")), List::<Adt17>::nil()))
                                            } else {
                                                if f26(v8.clone(), f8(Str::lit("object"))) {
                                                    Ok(f199(v1.clone(), List::cons(Adt17::C0(v1, v0), List::<Adt17>::nil())))
                                                } else {
                                                    Ok(f198(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2035 = f72(Some(v6));
                                    match m2035 {
                                        None => {
                                            Ok(f198(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f199(f8(Str::lit("Bytes")), List::<Adt17>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f199(v1.clone(), List::cons(Adt17::C0(v1, v0), List::<Adt17>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2032 = f298(v3)?;
                    match m2032 {
                        None => {
                            Ok(f198(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f199(v4, List::<Adt17>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f198(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f283(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2036, _) = v2;
        h2036
    };
    let a2037 = v3;
    let a2038 = 0u64;
    if nat_eq(a2037, a2038) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2039 = v3;
            let a2040 = v1;
            let a2041 = 0u64;
            nat_quot(a2039, a2040, a2041)
        }, {
            let a2046 = f259(v0, {
                let a2042 = v3;
                let a2043 = v1;
                let a2044 = 0u64;
                nat_rem(a2042, a2043, a2044)
            });
            let a2047 = {
                let (_, h2045) = v2;
                h2045
            };
            append_bytes(a2046, a2047)
        }))
    }
}

pub fn f284(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2092 = {
            let a2068 = {
                let a2056 = {
                    let a2050 = {
                        let a2048 = v1;
                        let a2049 = 128u8;
                        fixed_u8::bit_and(a2048, a2049)
                    };
                    let a2051 = 128u8;
                    if equal(a2050, a2051) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2057 = {
                    let a2054 = {
                        let a2052 = v1;
                        let a2053 = 64u8;
                        fixed_u8::bit_and(a2052, a2053)
                    };
                    let a2055 = 64u8;
                    if equal(a2054, a2055) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2056, a2057)?
            };
            let a2069 = {
                let a2066 = {
                    let a2060 = {
                        let a2058 = v1;
                        let a2059 = 32u8;
                        fixed_u8::bit_and(a2058, a2059)
                    };
                    let a2061 = 32u8;
                    if equal(a2060, a2061) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2067 = {
                    let a2064 = {
                        let a2062 = v1;
                        let a2063 = 16u8;
                        fixed_u8::bit_and(a2062, a2063)
                    };
                    let a2065 = 16u8;
                    if equal(a2064, a2065) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2066, a2067)?
            };
            nat_add(a2068, a2069)?
        };
        let a2093 = {
            let a2090 = {
                let a2078 = {
                    let a2072 = {
                        let a2070 = v1;
                        let a2071 = 8u8;
                        fixed_u8::bit_and(a2070, a2071)
                    };
                    let a2073 = 8u8;
                    if equal(a2072, a2073) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2079 = {
                    let a2076 = {
                        let a2074 = v1;
                        let a2075 = 4u8;
                        fixed_u8::bit_and(a2074, a2075)
                    };
                    let a2077 = 4u8;
                    if equal(a2076, a2077) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2078, a2079)?
            };
            let a2091 = {
                let a2088 = {
                    let a2082 = {
                        let a2080 = v1;
                        let a2081 = 2u8;
                        fixed_u8::bit_and(a2080, a2081)
                    };
                    let a2083 = 2u8;
                    if equal(a2082, a2083) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2089 = {
                    let a2086 = {
                        let a2084 = v1;
                        let a2085 = 1u8;
                        fixed_u8::bit_and(a2084, a2085)
                    };
                    let a2087 = 1u8;
                    if equal(a2086, a2087) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2088, a2089)?
            };
            nat_add(a2090, a2091)?
        };
        nat_add(a2092, a2093)?
    };
    let a2100 = f259(f300(), {
        let a2094 = v2;
        let a2095 = 16u64;
        let a2096 = 0u64;
        nat_quot(a2094, a2095, a2096)
    });
    let a2101 = f259(f300(), {
        let a2097 = v2;
        let a2098 = 16u64;
        let a2099 = 0u64;
        nat_rem(a2097, a2098, a2099)
    });
    Ok(append_bytes(a2100, a2101))
}

pub fn f285(v0: Fn20, p2105: List<List<(Bytes, Bytes)>>, p2106: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2104 = (p2105, p2106);
    {
        let mut m2104 = m2104;
        loop {
            let step = {
                let (v1, v2) = m2104;
                let m2102 = v2;
                match m2102.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2103: Fn20 = v0.clone();
                            c2103.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f286(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f287(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2108 = {
        let a2107 = v0.clone();
        length_list(a2107)
    };
    let a2109 = 1u64;
    if nat_le(a2108, a2109) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f301(v0)?))
    }
}

pub fn f288(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2110 = f183(v0, v1);
    let a2111 = f275(v0, v2);
    fixed_u32::bit_or(a2110, a2111)
}

pub fn f289(v0: u32) -> u32 {
    let a2114 = {
        let a2112 = f288(v0, 17u32, 15u32);
        let a2113 = f288(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2112, a2113)
    };
    let a2115 = f183(v0, 10u32);
    fixed_u32::bit_xor(a2114, a2115)
}

pub fn f290(v0: u32) -> u32 {
    let a2118 = {
        let a2116 = f288(v0, 7u32, 25u32);
        let a2117 = f288(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2116, a2117)
    };
    let a2119 = f183(v0, 3u32);
    fixed_u32::bit_xor(a2118, a2119)
}

pub fn f291(v0: Fn21, p2124: u64, p2125: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2123 = (p2124, p2125);
    {
        let mut m2123 = m2123;
        loop {
            let step = {
                let (v1, v2) = m2123;
                let m2120 = v1;
                if m2120 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2120 - 1;
                    let m2122 = {
                        let c2121: Fn21 = v0.clone();
                        c2121.apply(v2.clone())?
                    };
                    match m2122 {
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
                Ok(next) => { m2123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f292(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f302(v0)
}

pub fn f293(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2128 = f38(v0.clone(), {
        let a2126 = v1;
        let a2127 = 2u64;
        nat_add(a2126, a2127)?
    }, 2u64);
    let a2129 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2128, a2129) {
        let m2132 = f303(f38(v0, {
            let a2130 = v1;
            let a2131 = 4u64;
            nat_add(a2130, a2131)?
        }, 2u64))?;
        match m2132 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f194(v2.clone(), 0u64)?;
                let a2133 = v3;
                let a2134 = 32u64;
                if nat_lt(a2133, a2134) {
                    if f304(v3) {
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

pub fn f294(v0: u64) -> Bytes {
    let a2135 = v0;
    let a2136 = 34u64;
    if nat_eq(a2135, a2136) {
        Bytes::lit(&[0x22])
    } else {
        let a2137 = v0;
        let a2138 = 92u64;
        if nat_eq(a2137, a2138) {
            Bytes::lit(&[0x5c])
        } else {
            let a2139 = v0;
            let a2140 = 98u64;
            if nat_eq(a2139, a2140) {
                Bytes::lit(&[0x08])
            } else {
                let a2141 = v0;
                let a2142 = 102u64;
                if nat_eq(a2141, a2142) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2143 = v0;
                    let a2144 = 110u64;
                    if nat_eq(a2143, a2144) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2145 = v0;
                        let a2146 = 114u64;
                        if nat_eq(a2145, a2146) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2147 = v0;
                            let a2148 = 116u64;
                            if nat_eq(a2147, a2148) {
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

pub fn f295(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2151 = {
        let a2149 = v0;
        let a2150 = v1;
        index_bytes(a2149, a2150)
    };
    match m2151 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f305(v2)? {
                Ok(Some({
                    let a2152 = v1;
                    let a2153 = 1u64;
                    nat_add(a2152, a2153)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f296(v0: u8) -> R<u64> {
    let a2200 = {
        let v1: u8 = v0;
        let a2198 = {
            let a2174 = {
                let a2162 = {
                    let a2156 = {
                        let a2154 = v1;
                        let a2155 = 128u8;
                        fixed_u8::bit_and(a2154, a2155)
                    };
                    let a2157 = 128u8;
                    if equal(a2156, a2157) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2163 = {
                    let a2160 = {
                        let a2158 = v1;
                        let a2159 = 64u8;
                        fixed_u8::bit_and(a2158, a2159)
                    };
                    let a2161 = 64u8;
                    if equal(a2160, a2161) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2162, a2163)?
            };
            let a2175 = {
                let a2172 = {
                    let a2166 = {
                        let a2164 = v1;
                        let a2165 = 32u8;
                        fixed_u8::bit_and(a2164, a2165)
                    };
                    let a2167 = 32u8;
                    if equal(a2166, a2167) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2173 = {
                    let a2170 = {
                        let a2168 = v1;
                        let a2169 = 16u8;
                        fixed_u8::bit_and(a2168, a2169)
                    };
                    let a2171 = 16u8;
                    if equal(a2170, a2171) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2172, a2173)?
            };
            nat_add(a2174, a2175)?
        };
        let a2199 = {
            let a2196 = {
                let a2184 = {
                    let a2178 = {
                        let a2176 = v1;
                        let a2177 = 8u8;
                        fixed_u8::bit_and(a2176, a2177)
                    };
                    let a2179 = 8u8;
                    if equal(a2178, a2179) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2185 = {
                    let a2182 = {
                        let a2180 = v1;
                        let a2181 = 4u8;
                        fixed_u8::bit_and(a2180, a2181)
                    };
                    let a2183 = 4u8;
                    if equal(a2182, a2183) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2184, a2185)?
            };
            let a2197 = {
                let a2194 = {
                    let a2188 = {
                        let a2186 = v1;
                        let a2187 = 2u8;
                        fixed_u8::bit_and(a2186, a2187)
                    };
                    let a2189 = 2u8;
                    if equal(a2188, a2189) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2195 = {
                    let a2192 = {
                        let a2190 = v1;
                        let a2191 = 1u8;
                        fixed_u8::bit_and(a2190, a2191)
                    };
                    let a2193 = 1u8;
                    if equal(a2192, a2193) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2194, a2195)?
            };
            nat_add(a2196, a2197)?
        };
        nat_add(a2198, a2199)?
    };
    let a2201 = 48u64;
    Ok(nat_sub(a2200, a2201))
}

pub fn f297(p2207: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2206 = p2207;
    {
        let mut m2206 = m2206;
        loop {
            let step = {
                let v0 = m2206;
                let m2202 = v0;
                match m2202.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f54({
                            let (h2203, _) = v1.clone();
                            h2203
                        }, f306()) {
                            true
                        } else {
                            f54({
                                let (h2204, _) = v1.clone();
                                h2204
                            }, f307())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2205, _) = v1;
                                h2205
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2206 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: Bytes) -> R<Option<Bytes>> {
    if f172(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f77(f38(v0.clone(), 8u64, {
            let a2209 = {
                let a2208 = v0;
                length_bytes(a2208)
            };
            let a2210 = 8u64;
            nat_sub(a2209, a2210)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f299(v0: Adt14) -> Bytes {
    let m2211 = f180(f71(v0, f8(Str::lit("minimum"))));
    match m2211 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f300() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f301(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f262({
        let (_, h2212) = {
            let (h2213, _) = f308(Fn22::F309, {
                let a2215 = {
                    let a2214 = v0.clone();
                    length_list(a2214)
                };
                let a2216 = 1u64;
                nat_add(a2215, a2216)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2213
        };
        h2212
    }))
}

pub fn f302(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2217, _) = v0;
        h2217
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2218) = v0;
        h2218
    };
    let m2219 = v1;
    match m2219.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2220 = v4;
            match m2220.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f310(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f303(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2221 = v0.clone();
        length_bytes(a2221)
    };
    let a2225 = {
        let a2222 = v1;
        let a2223 = 2u64;
        let a2224 = 0u64;
        nat_rem(a2222, a2223, a2224)
    };
    let a2226 = 0u64;
    if nat_eq(a2225, a2226) {
        let (_, h2227) = {
            let (h2228, _) = f311(Fn23::F312(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2228
        };
        Ok(h2227)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f304(v0: u64) -> bool {
    let a2229 = v0;
    let a2230 = 8u64;
    if nat_eq(a2229, a2230) {
        true
    } else {
        let a2231 = v0;
        let a2232 = 9u64;
        if nat_eq(a2231, a2232) {
            true
        } else {
            let a2233 = v0;
            let a2234 = 10u64;
            if nat_eq(a2233, a2234) {
                true
            } else {
                let a2235 = v0;
                let a2236 = 12u64;
                if nat_eq(a2235, a2236) {
                    true
                } else {
                    let a2237 = v0;
                    let a2238 = 13u64;
                    nat_eq(a2237, a2238)
                }
            }
        }
    }
}

pub fn f305(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2283 = {
            let a2259 = {
                let a2247 = {
                    let a2241 = {
                        let a2239 = v1;
                        let a2240 = 128u8;
                        fixed_u8::bit_and(a2239, a2240)
                    };
                    let a2242 = 128u8;
                    if equal(a2241, a2242) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2248 = {
                    let a2245 = {
                        let a2243 = v1;
                        let a2244 = 64u8;
                        fixed_u8::bit_and(a2243, a2244)
                    };
                    let a2246 = 64u8;
                    if equal(a2245, a2246) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2247, a2248)?
            };
            let a2260 = {
                let a2257 = {
                    let a2251 = {
                        let a2249 = v1;
                        let a2250 = 32u8;
                        fixed_u8::bit_and(a2249, a2250)
                    };
                    let a2252 = 32u8;
                    if equal(a2251, a2252) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2258 = {
                    let a2255 = {
                        let a2253 = v1;
                        let a2254 = 16u8;
                        fixed_u8::bit_and(a2253, a2254)
                    };
                    let a2256 = 16u8;
                    if equal(a2255, a2256) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2257, a2258)?
            };
            nat_add(a2259, a2260)?
        };
        let a2284 = {
            let a2281 = {
                let a2269 = {
                    let a2263 = {
                        let a2261 = v1;
                        let a2262 = 8u8;
                        fixed_u8::bit_and(a2261, a2262)
                    };
                    let a2264 = 8u8;
                    if equal(a2263, a2264) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2270 = {
                    let a2267 = {
                        let a2265 = v1;
                        let a2266 = 4u8;
                        fixed_u8::bit_and(a2265, a2266)
                    };
                    let a2268 = 4u8;
                    if equal(a2267, a2268) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2269, a2270)?
            };
            let a2282 = {
                let a2279 = {
                    let a2273 = {
                        let a2271 = v1;
                        let a2272 = 2u8;
                        fixed_u8::bit_and(a2271, a2272)
                    };
                    let a2274 = 2u8;
                    if equal(a2273, a2274) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2280 = {
                    let a2277 = {
                        let a2275 = v1;
                        let a2276 = 1u8;
                        fixed_u8::bit_and(a2275, a2276)
                    };
                    let a2278 = 1u8;
                    if equal(a2277, a2278) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2279, a2280)?
            };
            nat_add(a2281, a2282)?
        };
        nat_add(a2283, a2284)?
    };
    let a2285 = 48u64;
    let a2286 = v2;
    if nat_le(a2285, a2286) {
        let a2287 = v2;
        let a2288 = 57u64;
        Ok(nat_le(a2287, a2288))
    } else {
        Ok(false)
    }
}

pub fn f306() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f307() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f308(v0: Fn22, p2293: u64, p2294: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2292 = (p2293, p2294);
    {
        let mut m2292 = m2292;
        loop {
            let step = {
                let (v1, v2) = m2292;
                let m2289 = v1;
                if m2289 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2289 - 1;
                    let m2291 = {
                        let c2290: Fn22 = v0.clone();
                        c2290.apply(v2.clone())?
                    };
                    match m2291 {
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
                Ok(next) => { m2292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f309(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f313(v0)
}

pub fn f310(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2299 = {
            let a2297 = {
                let a2295 = v0.clone();
                length_list(a2295)
            };
            let a2298 = {
                let a2296 = v1.clone();
                length_list(a2296)
            };
            nat_add(a2297, a2298)?
        };
        let a2300 = 1u64;
        nat_add(a2299, a2300)?
    };
    Ok(f222({
        let (_, h2301) = {
            let (_, h2302) = {
                let (h2303, _) = f314(Fn24::F315, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2303
            };
            h2302
        };
        h2301
    }))
}

pub fn f311(v0: Fn23, p2308: u64, p2309: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2307 = (p2308, p2309);
    {
        let mut m2307 = m2307;
        loop {
            let step = {
                let (v1, v2) = m2307;
                let m2304 = v1;
                if m2304 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2304 - 1;
                    let m2306 = {
                        let c2305: Fn23 = v0.clone();
                        c2305.apply(v2.clone())?
                    };
                    match m2306 {
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
                Ok(next) => { m2307 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f316(v0, v1)
}

pub fn f313(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2310, _) = v0;
        h2310
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2311) = v0;
        h2311
    };
    let m2312 = v1;
    match m2312.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2313 = v4;
            match m2313.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f317(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f314(v0: Fn24, p2318: u64, p2319: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2317 = (p2318, p2319);
    {
        let mut m2317 = m2317;
        loop {
            let step = {
                let (v1, v2) = m2317;
                let m2314 = v1;
                if m2314 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2314 - 1;
                    let m2316 = {
                        let c2315: Fn24 = v0.clone();
                        c2315.apply(v2.clone())
                    };
                    match m2316 {
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
                Ok(next) => { m2317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f318(v0)
}

pub fn f316(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2320, _) = v1;
        h2320
    };
    let m2322 = {
        let (_, h2321) = v1;
        h2321
    };
    match m2322 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2325 = {
                let a2323 = v0.clone();
                let a2324 = v2;
                index_bytes(a2323, a2324)
            };
            match m2325 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2330 = {
                        let a2328 = v0;
                        let a2329 = {
                            let a2326 = v2;
                            let a2327 = 1u64;
                            nat_add(a2326, a2327)?
                        };
                        index_bytes(a2328, a2329)
                    };
                    match m2330 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f319(v4)? {
                                f319(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2331 = v2;
                                    let a2332 = 2u64;
                                    nat_add(a2331, a2332)?
                                }, Some({
                                    let a2337 = v3;
                                    let a2338 = f259(f320(), {
                                        let a2335 = {
                                            let a2333 = f321(v4)?;
                                            let a2334 = 16u64;
                                            nat_mul(a2333, a2334)?
                                        };
                                        let a2336 = f321(v5)?;
                                        nat_add(a2335, a2336)?
                                    });
                                    append_bytes(a2337, a2338)
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

pub fn f317(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a2343 = {
            let a2341 = {
                let a2339 = v0.clone();
                length_list(a2339)
            };
            let a2342 = {
                let a2340 = v1.clone();
                length_list(a2340)
            };
            nat_add(a2341, a2342)?
        };
        let a2344 = 1u64;
        nat_add(a2343, a2344)?
    };
    Ok(f322({
        let (_, h2345) = {
            let (_, h2346) = {
                let (h2347, _) = f323(Fn25::F324, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h2347
            };
            h2346
        };
        h2345
    }))
}

pub fn f318(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2348, _) = v0.clone();
        h2348
    };
    let v2: List<(Bytes, ())> = {
        let (h2349, _) = {
            let (_, h2350) = v0.clone();
            h2350
        };
        h2349
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2351) = {
            let (_, h2352) = v0;
            h2352
        };
        h2351
    };
    let m2353 = v1.clone();
    match m2353.uncons() {
        None => {
            let m2354 = v2;
            match m2354.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2355 = v2.clone();
            match m2355.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f325({
                        let (h2356, _) = v8.clone();
                        h2356
                    }, {
                        let (h2357, _) = v6.clone();
                        h2357
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

pub fn f319(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2402 = {
            let a2378 = {
                let a2366 = {
                    let a2360 = {
                        let a2358 = v1;
                        let a2359 = 128u8;
                        fixed_u8::bit_and(a2358, a2359)
                    };
                    let a2361 = 128u8;
                    if equal(a2360, a2361) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2367 = {
                    let a2364 = {
                        let a2362 = v1;
                        let a2363 = 64u8;
                        fixed_u8::bit_and(a2362, a2363)
                    };
                    let a2365 = 64u8;
                    if equal(a2364, a2365) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2366, a2367)?
            };
            let a2379 = {
                let a2376 = {
                    let a2370 = {
                        let a2368 = v1;
                        let a2369 = 32u8;
                        fixed_u8::bit_and(a2368, a2369)
                    };
                    let a2371 = 32u8;
                    if equal(a2370, a2371) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2377 = {
                    let a2374 = {
                        let a2372 = v1;
                        let a2373 = 16u8;
                        fixed_u8::bit_and(a2372, a2373)
                    };
                    let a2375 = 16u8;
                    if equal(a2374, a2375) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2376, a2377)?
            };
            nat_add(a2378, a2379)?
        };
        let a2403 = {
            let a2400 = {
                let a2388 = {
                    let a2382 = {
                        let a2380 = v1;
                        let a2381 = 8u8;
                        fixed_u8::bit_and(a2380, a2381)
                    };
                    let a2383 = 8u8;
                    if equal(a2382, a2383) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2389 = {
                    let a2386 = {
                        let a2384 = v1;
                        let a2385 = 4u8;
                        fixed_u8::bit_and(a2384, a2385)
                    };
                    let a2387 = 4u8;
                    if equal(a2386, a2387) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2388, a2389)?
            };
            let a2401 = {
                let a2398 = {
                    let a2392 = {
                        let a2390 = v1;
                        let a2391 = 2u8;
                        fixed_u8::bit_and(a2390, a2391)
                    };
                    let a2393 = 2u8;
                    if equal(a2392, a2393) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2399 = {
                    let a2396 = {
                        let a2394 = v1;
                        let a2395 = 1u8;
                        fixed_u8::bit_and(a2394, a2395)
                    };
                    let a2397 = 1u8;
                    if equal(a2396, a2397) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2398, a2399)?
            };
            nat_add(a2400, a2401)?
        };
        nat_add(a2402, a2403)?
    };
    let a2404 = 48u64;
    let a2405 = v2;
    if if nat_le(a2404, a2405) {
        let a2406 = v2;
        let a2407 = 57u64;
        nat_le(a2406, a2407)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2408 = 97u64;
        let a2409 = v2;
        if nat_le(a2408, a2409) {
            let a2410 = v2;
            let a2411 = 102u64;
            Ok(nat_le(a2410, a2411))
        } else {
            Ok(false)
        }
    }
}

pub fn f320() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f321(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2456 = {
            let a2432 = {
                let a2420 = {
                    let a2414 = {
                        let a2412 = v1;
                        let a2413 = 128u8;
                        fixed_u8::bit_and(a2412, a2413)
                    };
                    let a2415 = 128u8;
                    if equal(a2414, a2415) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2421 = {
                    let a2418 = {
                        let a2416 = v1;
                        let a2417 = 64u8;
                        fixed_u8::bit_and(a2416, a2417)
                    };
                    let a2419 = 64u8;
                    if equal(a2418, a2419) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2420, a2421)?
            };
            let a2433 = {
                let a2430 = {
                    let a2424 = {
                        let a2422 = v1;
                        let a2423 = 32u8;
                        fixed_u8::bit_and(a2422, a2423)
                    };
                    let a2425 = 32u8;
                    if equal(a2424, a2425) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2431 = {
                    let a2428 = {
                        let a2426 = v1;
                        let a2427 = 16u8;
                        fixed_u8::bit_and(a2426, a2427)
                    };
                    let a2429 = 16u8;
                    if equal(a2428, a2429) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2430, a2431)?
            };
            nat_add(a2432, a2433)?
        };
        let a2457 = {
            let a2454 = {
                let a2442 = {
                    let a2436 = {
                        let a2434 = v1;
                        let a2435 = 8u8;
                        fixed_u8::bit_and(a2434, a2435)
                    };
                    let a2437 = 8u8;
                    if equal(a2436, a2437) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2443 = {
                    let a2440 = {
                        let a2438 = v1;
                        let a2439 = 4u8;
                        fixed_u8::bit_and(a2438, a2439)
                    };
                    let a2441 = 4u8;
                    if equal(a2440, a2441) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2442, a2443)?
            };
            let a2455 = {
                let a2452 = {
                    let a2446 = {
                        let a2444 = v1;
                        let a2445 = 2u8;
                        fixed_u8::bit_and(a2444, a2445)
                    };
                    let a2447 = 2u8;
                    if equal(a2446, a2447) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2453 = {
                    let a2450 = {
                        let a2448 = v1;
                        let a2449 = 1u8;
                        fixed_u8::bit_and(a2448, a2449)
                    };
                    let a2451 = 1u8;
                    if equal(a2450, a2451) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2452, a2453)?
            };
            nat_add(a2454, a2455)?
        };
        nat_add(a2456, a2457)?
    };
    let a2458 = v2;
    let a2459 = 57u64;
    if nat_le(a2458, a2459) {
        let a2460 = v2;
        let a2461 = 48u64;
        Ok(nat_sub(a2460, a2461))
    } else {
        let a2462 = v2;
        let a2463 = 87u64;
        Ok(nat_sub(a2462, a2463))
    }
}

pub fn f322(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f204(Fn6::F326, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f323(v0: Fn25, p2468: u64, p2469: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m2467 = (p2468, p2469);
    {
        let mut m2467 = m2467;
        loop {
            let step = {
                let (v1, v2) = m2467;
                let m2464 = v1;
                if m2464 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m2464 - 1;
                    let m2466 = {
                        let c2465: Fn25 = v0.clone();
                        c2465.apply(v2.clone())
                    };
                    match m2466 {
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
                Ok(next) => { m2467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f327(v0)
}

pub fn f325(v0: Bytes, v1: Bytes) -> bool {
    let a2474 = {
        let a2470 = v0;
        let a2471 = v1;
        compare_bytes(a2470, a2471)
    };
    let a2475 = {
        let a2472 = Bytes::lit(&[]);
        let a2473 = Bytes::lit(&[0x01]);
        compare_bytes(a2472, a2473)
    };
    equal(a2474, a2475)
}

pub fn f326(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f327(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h2476, _) = v0.clone();
        h2476
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h2477, _) = {
            let (_, h2478) = v0.clone();
            h2478
        };
        h2477
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h2479) = {
            let (_, h2480) = v0;
            h2480
        };
        h2479
    };
    let m2481 = v1.clone();
    match m2481.uncons() {
        None => {
            let m2482 = v2;
            match m2482.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2483 = v2.clone();
            match m2483.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f325({
                        let (h2484, _) = v8.clone();
                        h2484
                    }, {
                        let (h2485, _) = v6.clone();
                        h2485
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
