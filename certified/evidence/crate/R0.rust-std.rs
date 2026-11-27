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
    F183(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F77(k0, k1) => f77(*k0, k1.clone(), p0),
            Fn0::F183(k0) => f183(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F121,
    F236,
    F343,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F121 => f121(p0, p1),
            Fn1::F236 => f236(p0, p1),
            Fn1::F343 => f343(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F206(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F206(k0) => f206(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F312(Bytes),
    F313(Bytes),
    F410(Bytes),
    F411(u64, Bytes),
    F418(Bytes),
    F426(Bytes, u64, Bytes, u64),
    F486(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F312(k0) => f312(k0.clone(), p0),
            Fn3::F313(k0) => f313(k0.clone(), p0),
            Fn3::F410(k0) => Ok(f410(k0.clone(), p0)),
            Fn3::F411(k0, k1) => f411(*k0, k1.clone(), p0),
            Fn3::F418(k0) => f418(k0.clone(), p0),
            Fn3::F426(k0, k1, k2, k3) => f426(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F486(k0) => f486(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F316(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F316(k0) => f316(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F402,
    F590,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F402 => f402(p0, p1),
            Fn5::F590 => f590(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F405,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F405 => f405(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F437,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F437 => f437(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F440,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F440 => f440(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F442,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F442 => f442(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F445,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F445 => f445(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F466(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt23)) -> R<(u64, Adt23)> {
        match self {
            Fn11::F466(k0, k1, k2) => f466(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F497,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F497 => f497(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F500,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F500 => f500(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F519(u64, Bytes),
    F530(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn14::F519(k0, k1) => Ok(f519(*k0, k1.clone(), p0)),
            Fn14::F530(k0) => f530(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F527,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F527 => f527(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F529,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F529 => f529(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F533(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F533(k0) => f533(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F542,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn18::F542 => f542(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F549,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F549 => f549(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F554,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F554 => f554(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F573,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F573 => f573(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F576,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F576 => f576(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F583,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F583 => f583(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F588,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F588 => f588(p0),
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
                    Ok(Adt12::C3(f154(f155(v0.clone())?, List::cons((f156(), f157()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f158()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f159()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f160()), List::cons((f8(Str::lit(".github/scripts/readmit.sh")), f161()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f162()), List::cons((f8(Str::lit(".github/lexlean.pin")), f163()), List::cons((f8(Str::lit("docs/catalog.md")), f164(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f165(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f166(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f167()), List::<(Bytes, Bytes)>::nil())))))))))))), f8(Str::lit("published the workflows, the export and readmit scripts, CODEOWNERS, the lexlean pin, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
        f168(v3.clone(), v1.clone(), f169(v3, v1, false)?)
    } else {
        if f32(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f168(v3.clone(), v1.clone(), f170(v3, v1)?)
        } else {
            if f32(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f168(v3.clone(), v1.clone(), f169(v3, v1, true)?)
            } else {
                if f32(v0.clone(), f8(Str::lit("stage-registries"))) {
                    f168(v3.clone(), v1.clone(), f171(v3, v1)?)
                } else {
                    if f32(v0.clone(), f8(Str::lit("stage-encodings"))) {
                        f168(v3.clone(), v1.clone(), f172(v3, v1)?)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("stage-mappings"))) {
                            f168(v3.clone(), v1.clone(), f173(v3, v1, false)?)
                        } else {
                            if f32(v0, f8(Str::lit("stage-matrix"))) {
                                f168(v3.clone(), v1.clone(), f173(v3, v1, true)?)
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
            f174(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("ingest-index"))) {
                f175(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("merge-gate"))) {
                    f176(v3, v1)
                } else {
                    if f32(v0.clone(), f8(Str::lit("record-decision"))) {
                        f177(v3, v1)
                    } else {
                        if f32(v0, f8(Str::lit("record-error"))) {
                            f178(v3, v1, v2)
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
    f179(v0.clone(), 0u64, {
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
    let m490 = f180(f181(v0.clone(), v1.clone()), v1.clone());
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
            let m500 = f182(v2, v1.clone())?;
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
        let (h502, _) = f76(Fn0::F183(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
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
    Ok(f184(f57(v0, v1)?))
}

pub fn f97(v0: List<Adt2>, v1: Bytes) -> List<Adt2> {
    let m511 = v0;
    match m511.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f185(v2, v1.clone()), f97(v3, v1))
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
    let a520 = f186(v0)?;
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
        let m527 = f187(v0.clone(), f60())?;
        match m527 {
            None => {
                if f51(f52(f190(), match v0.clone() {
                    Adt1::C0(_, _, h528, _) => {
                        h528
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f190()))
                } else {
                    let m529 = f56(v0.clone(), List::cons(f191(), f192(f60())?));
                    match m529.uncons() {
                        None => {
                            Ok(Adt12::C3(f193(v0.clone(), f60(), v1.clone(), v2.clone(), f194(f64(v0, f191()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f188(v3.clone())?, f189(v3, v2)))
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
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f195(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m530 - 1;
                    let m533 = {
                        let a531 = v0.clone();
                        let a532 = v1;
                        index_bytes(a531, a532)
                    };
                    match m533 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f195(v0.clone(), v5, v1, v6)))
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
                                    }, f195(v0.clone(), v5, v1, v6)))))))
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
    let v2: u64 = f196(v0.clone(), v1, {
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
        f197(v0.clone(), v1, {
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
    let v4: u64 = f198(v0.clone(), v3)?;
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
        f198(v0.clone(), v7)?
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
    let v12: bool = f117(v10, f199({
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
        f198(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a648 = v14;
        let a649 = v13;
        nat_sub(a648, a649)
    });
    let v16: u64 = f200(v15.clone())?;
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
                f199(if v10 {
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
                f201(v17)?
            }
        };
        match m669 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m674 = f202(f46(v0.clone(), v3, v5), f46(v0, v7, {
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
                            let m680 = f201({
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
                                    if f203(v21, v20) {
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
    let m692 = f204(v0.clone(), v3);
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
    let m708 = f205(Fn2::F206(v1), None::<Bytes>, v0);
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
    }, f207()) {
        if f138(match v0 {
            Adt1::C0(_, h710, _, _) => {
                h710
            }
        }, f208()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f208(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f207(), List::<Bytes>::nil())
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
    Ok(f56(v0.clone(), List::cons(f209(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f136(v0)?)))))
}

pub fn f127(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f210(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f142(f96(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
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
            f125(f211(f184(f93(v1, f8(Str::lit("defers"))))), f130(v2))
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
    let v3: List<(Bytes, Bytes)> = f154(f212(v2.clone()), f213(f214(v0, v1.clone(), v2)?));
    f215(f145(f216(v1))?, v3.clone(), {
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
            if if f217(v2.clone(), f8(Str::lit("subject"))) {
                f218(v0.clone(), v2.clone())
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
            let m723 = f219(f95(f64(v0.clone(), v2.clone()))?, f85(v2, f208()))?;
            match m723 {
                None => {
                    f135(v0, v3)
                }
                Some(v4) => {
                    Ok(f220(v4, f135(v0, v3)?))
                }
            }
        }
    }
}

pub fn f136(v0: Adt1) -> R<List<Bytes>> {
    Ok(f125(f145(f221(f207(), f147(match v0.clone() {
        Adt1::C0(_, h724, _, _) => {
            h724
        }
    }, f207())))?, f145(f221(f208(), f147(match v0 {
        Adt1::C0(_, h725, _, _) => {
            h725
        }
    }, f208())))?))
}

pub fn f137(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("-c")), List::cons(f222(), List::cons(f8(Str::lit("prism-code")), List::cons(f8(Str::lit("UOR-Foundation/prismpm")), List::cons(f223(), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil())))))))))
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
    f145(f221(f139(), f147(match v0 {
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
                f224(v4.clone(), f95(f64(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f225(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
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
    let m733 = f56(v0.clone(), f226());
    match m733.uncons() {
        None => {
            let m734 = f227(v0.clone(), f226())?;
            match m734.uncons() {
                None => {
                    let m737 = f228(match v0.clone() {
                        Adt1::C0(_, h735, _, _) => {
                            h735
                        }
                    }, f125(f229(), f230(match v0.clone() {
                        Adt1::C0(_, h736, _, _) => {
                            h736
                        }
                    }, false)));
                    match m737.uncons() {
                        None => {
                            let m739 = f56(v0.clone(), f230(match v0.clone() {
                                Adt1::C0(_, h738, _, _) => {
                                    h738
                                }
                            }, true));
                            match m739.uncons() {
                                None => {
                                    let m740 = f56(v0.clone(), f231());
                                    match m740.uncons() {
                                        None => {
                                            let m741 = f232(v0.clone())?;
                                            match m741 {
                                                None => {
                                                    let m743 = f233(v0.clone(), f230(match v0.clone() {
                                                        Adt1::C0(_, h742, _, _) => {
                                                            h742
                                                        }
                                                    }, true))?;
                                                    match m743 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f234(Adt16::C0(Adt15::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f210(v0.clone())?, f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f96(v0.clone(), f8(Str::lit("model/strata.json")))?, f211(f96(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f230(match v0 {
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
    Ok(f235(f120(Fn1::F236, List::<Bytes>::nil(), f237(v0)?)))
}

pub fn f146(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m748 = v1;
    match m748.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f238(v0.clone(), v2), f146(v0, v3))
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
                        if f239(v0.clone(), v1.clone(), v3.clone())? {
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
            Ok(List::cons((f240(v1.clone())?, v1), f149(v2)?))
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
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f241(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f241(v1, List::<Bytes>::nil()), v2))))
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
    }, f242())) {
        Ok(Adt12::C1(List::cons(f242(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f146(f242(), f147(match v0.clone() {
            Adt1::C0(_, h765, _, _) => {
                h765
            }
        }, f242())))?, v1.clone());
        let m766 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m766.uncons() {
            None => {
                let v5: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m767 = f56(v0.clone(), f243(v0.clone(), v2.clone(), v5.clone())?);
                match m767.uncons() {
                    None => {
                        let m768 = f244(v0.clone(), v2, v5)?;
                        match m768 {
                            None => {
                                f245(v0, v1)
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
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f246(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f156() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f157() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}            target/release/verdant_xml\u{a}            target/release/verdant_admit\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      # Restored on every run, not only when certified/ changed: the end-to-end propose-command step below\u{a}      # exports with the attested lexlean binary, and a fresh cargo install does not reproduce its hash.\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers certified-xml certified-admit)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/, certified-readers/, certified-xml/ or certified-admit/:\"\u{a}            git status --short -- certified certified-readers certified-xml certified-admit | head -20\u{a}            exit 1\u{a}          fi\u{a}      # verify-artifact replays a bundle but does not compare it with the sources. So each LexLean project's\u{a}      # semantic id must equal the one its bundle records. Lock-level ids move with every admission to app/,\u{a}      # which readers depends on without using it, so they are not compared.\u{a}      - name: the certified bundles were exported from these sources (semantic ids, no Lean)\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The lexlean cache is keyed by .github/lexlean.pin, so after the pin moves the replay above may have been skipped with no binary restored.\u{a}          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          for pair in studio:certified readers:certified-readers readers/xml:certified-xml; do\u{a}            project=$(cut -d : -f 1 <<< \"$pair\")\u{a}            bundle=$(cut -d : -f 2 <<< \"$pair\")\u{a}            if [ ! -f \"$project/lexlean.toml\" ]; then continue; fi\u{a}            if [ ! -f \"$bundle/result.json\" ]; then echo \"::warning::$project has no committed bundle $bundle, so nothing records what it was exported from\"; continue; fi\u{a}            now=$(cd \"$project\" && \"$ll\" lock >/dev/null && \"$ll\" check | sed -n 's/.*semantic \\([0-9a-f]*\\)).*/\\1/p')\u{a}            git checkout -q -- \"$project/lexlean.lock\"\u{a}            was=$(jq -r .source.semantic_id \"$bundle/result.json\")\u{a}            if [ \"$now\" != \"$was\" ]; then echo \"::error::$project now has semantic id $now, but $bundle was exported from $was: export it again through propose-command\"; exit 1; fi\u{a}            echo \"$project matches $bundle (semantic $now)\"\u{a}          done\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          # Each encoding admitted through admit-translated recorded the reader command that printed it: run it again and fail if the output differs.\u{a}          target/release/verdant_readers verify-translations model/admissions.json\u{a}          if [ -x target/release/verdant_xml ]; then target/release/verdant_xml verify-translations model/admissions.json; fi\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          # certified-xml/ and certified-admit/ join the comparison once main holds them: the export that first writes it is the one that adds it.\u{a}          bundles=\"certified certified-readers\"\u{a}          if [ -n \"$(git ls-files certified-xml | head -n 1)\" ]; then bundles=\"$bundles certified-xml\"; fi\u{a}          if [ -n \"$(git ls-files certified-admit | head -n 1)\" ]; then bundles=\"$bundles certified-admit\"; fi\u{a}          if [ -n \"$(git status --porcelain -- $bundles)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- $bundles | head -20\u{a}            attested=$(git show HEAD:certified/evidence/attestation.json | jq -c .lexlean); used=$(jq -c .lexlean certified/evidence/attestation.json)\u{a}            [ \"$attested\" = \"$used\" ] || echo \"::error::the export ran with lexlean $used, but certified/ attests lexlean $attested. lexlean is built from .github/lexlean.pin with rust 1.97.0 and its cargo home remapped, so a different executable means the toolchain or the runner image changed\"\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f158() -> Bytes {
    f7(List::cons(f247(), List::cons(f248(), List::cons(f249(), List::cons(f250(), List::cons(f251(), List::cons(f252(), List::<Bytes>::nil())))))))
}

pub fn f159() -> Bytes {
    f7(List::cons(f253(), List::cons(f254(), List::cons(f255(), List::cons(f256(), List::<Bytes>::nil())))))
}

pub fn f160() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports under its own limits: studio, readers, admit and readers/xml when they exist, largest first.\u{a}# Lean's proof of a project grows with the square of its function count, so the readers are split by family and admission is its own project.\u{a}# Each export runs two Lean phases at once, so at most half as many exports as cores run together: more slows every phase.\u{a}slots=$(( $(nproc) / 2 ))\u{a}if [ \"$slots\" -lt 1 ]; then slots=1; fi\u{a}running=0\u{a}for project in studio readers admit readers/xml; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    if [ \"$running\" -ge \"$slots\" ]; then wait -n || status=$?; running=$((running - 1)); fi\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.${project//\\//-}\" 2>&1 &\u{a}    running=$((running + 1))\u{a}  fi\u{a}done\u{a}while [ \"$running\" -gt 0 ]; do wait -n || status=$?; running=$((running - 1)); done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}if [ -f \"$log.readers-xml\" ]; then cat \"$log.readers-xml\" >> \"$log\"; fi\u{a}if [ -f \"$log.admit\" ]; then cat \"$log.admit\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}if [ -f readers/xml/lexlean.toml ]; then\u{a}  rm -rf certified-xml\u{a}  cp -r readers/xml/.lexlean/production/*/ certified-xml\u{a}  rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-xml\u{a}fi\u{a}if [ -f admit/lexlean.toml ]; then\u{a}  rm -rf certified-admit\u{a}  cp -r admit/.lexlean/production/*/ certified-admit\u{a}  rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-admit\u{a}fi\u{a}"))
}

pub fn f161() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env bash\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# A change to a reader can change what it prints for an encoding already admitted, and the studio check\u{a}")), List::cons(f8(Str::lit("# replays every admitted reader command (verify-translations). So the change re-derives those encodings\u{a}")), List::cons(f8(Str::lit("# itself: each Catalog encoding whose replay now differs is admitted again through read-admit, with the\u{a}")), List::cons(f8(Str::lit("# command and citation its admission row records, and the pull request carries both (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("# It prints each construct it admitted again, one per line.\u{a}")), List::cons(f8(Str::lit("set -euo pipefail\u{a}")), List::cons(f8(Str::lit("readers=$1\u{a}")), List::cons(f8(Str::lit("mkdir -p target\u{a}")), List::cons(f8(Str::lit("rows=$(jq length model/admissions.json)\u{a}")), List::cons(f8(Str::lit("for try in $(seq 1 \"$rows\"); do\u{a}")), List::cons(f8(Str::lit("  if \"$readers\" verify-translations model/admissions.json > target/replay.txt 2>&1; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("  construct=$(sed -n 's/^\\([^:]*\\): its reader command now prints .*/\\1/p' target/replay.txt | head -n 1)\u{a}")), List::cons(f8(Str::lit("  case \"$construct\" in\u{a}")), List::cons(f8(Str::lit("    Catalog.*) ;;\u{a}")), List::cons(f8(Str::lit("    *) cat target/replay.txt >&2; echo \"readmit: only a Catalog encoding whose reader now prints other output is admitted again\" >&2; exit 1;;\u{a}")), List::cons(f8(Str::lit("  esac\u{a}")), List::cons(f8(Str::lit("  row=$(jq -c --arg construct \"$construct\" '[.[] | select(.construct == $construct and .translation != null)][0]' model/admissions.json)\u{a}")), List::cons(f8(Str::lit("  authority=$(jq -r '.std.authority // \"\"' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  cited=$(jq -r '.std.row // \"\"' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  if [ -z \"$authority\" ]; then echo \"readmit: $construct cites no std authority, so read-admit cannot cite it again\" >&2; exit 1; fi\u{a}")), List::cons(f8(Str::lit("  mapfile -t command < <(jq -r '.translation.command[]' <<< \"$row\")\u{a}")), List::cons(f8(Str::lit("  \"$readers\" read-admit \"${construct#Catalog.}\" \"${command[@]}\" std \"$authority\" $cited >/dev/null\u{a}")), List::cons(f8(Str::lit("  echo \"$construct\"\u{a}")), List::cons(f8(Str::lit("done\u{a}")), List::cons(f8(Str::lit("echo \"readmit: the replay did not settle after $rows admissions\" >&2\u{a}")), List::cons(f8(Str::lit("exit 1\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))
}

pub fn f162() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f163() -> Bytes {
    f8(Str::lit("--git https://github.com/UOR-Foundation/lexlean --rev 82a32815c45762ea8931c84cb9465a5aac902866\u{a}"))
}

pub fn f164(v0: Adt1) -> R<Bytes> {
    let v1: Adt14 = f127(v0.clone())?;
    let v2: List<Adt2> = f96(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt2> = f134(v1.clone(), f135(v0.clone(), f136(v0.clone())?)?);
    let v4: List<List<Bytes>> = f257(f258(f64(v0, f209()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f259(v4.clone(), f260(v4), f214(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f133(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f165(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f261(f127(v0.clone())?, f135(v0.clone(), f136(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f166(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f262(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f263(f96(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f167() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f168(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m771 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m771.uncons() {
        None => {
            if f264(f95(f64(v0, v1))?)? {
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

pub fn f169(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m772 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m772.uncons() {
        None => {
            let v5: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f265(v5)?
                } else {
                    f266(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f238(f267(), f268(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f238(f267(), f268(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f269()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f170(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m773 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m773.uncons() {
        None => {
            let m774 = f270(f95(f64(v0.clone(), v1.clone()))?)?;
            match m774 {
                None => {
                    Ok(Adt12::C4(f271()))
                }
                Some(v4) => {
                    let m775 = f272(f95(f64(v0, v1))?)?;
                    match m775 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f273(v5)))
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

pub fn f171(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let v2: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
        Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
    } else {
        if f117(f274(f275(v2.clone())?), f51({
            let a777 = {
                let a776 = f276(f275(v2)?);
                length_list(a776)
            };
            let a778 = 0u64;
            nat_eq(a777, a778)
        })) {
            Ok(Adt12::C3(List::cons((f238(f267(), f268(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f238(f267(), f268(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt12::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f172(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m779 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m779.uncons() {
        None => {
            let m780 = f277(f95(f64(v0, v1))?)?;
            match m780 {
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

pub fn f173(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m781 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m781.uncons() {
        None => {
            let v5: Bytes = f238(if v2 {
                f208()
            } else {
                f207()
            }, f268(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m782 = f219(f95(f64(v0.clone(), v1.clone()))?, v2)?;
                match m782 {
                    None => {
                        Ok(Adt12::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a784 = {
                            let a783 = v6;
                            length_list(a783)
                        };
                        let a785 = 0u64;
                        if nat_eq(a784, a785) {
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

pub fn f174(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m786 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m786.uncons() {
        None => {
            let m787 = f57(v0.clone(), v1.clone())?;
            match m787 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f278())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt2 = {
                            let m788 = f93(v4.clone(), f8(Str::lit("source")));
                            match m788 {
                                None => {
                                    Adt2::C9
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f92(f93(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f238(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f238(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m789 = v8;
                        match m789 {
                            None => {
                                f279(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f279(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f238(v9, f268(f58(v7.clone(), f8(Str::lit("url"))))?);
                                    if f51(f52(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h790, _) => {
                                            h790
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f58(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m791 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m791.uncons() {
                                            None => {
                                                if f32(f280(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f279(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f175(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m792 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m792.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f258(v4.clone())?;
            if f51(f98(f281(v5.clone()), f282())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f283(Str::lit("id"), v6.clone())?, List::cons(f283(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f283(Str::lit("artifact"), f209())?, List::cons(f283(Str::lit("checksum"), f280(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f209(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f284(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f285(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f122(f260(f257(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f176(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m793 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m793.uncons() {
        None => {
            let m794 = f57(v0, v1)?;
            match m794 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f286(f184(Some(v4))) {
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

pub fn f177(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m795 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f153(), List::<Bytes>::nil())));
    match m795.uncons() {
        None => {
            let m796 = f287(f95(f64(v0.clone(), v1))?)?;
            match m796 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt2> = f96(v0, f153())?;
                    let v6: List<Bytes> = f142(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f145(f142(v5.clone(), f8(Str::lit("at"))))?;
                    let a798 = {
                        let a797 = v6.clone();
                        length_list(a797)
                    };
                    let a799 = 0u64;
                    if if nat_eq(a798, a799) {
                        true
                    } else {
                        f51({
                            let a802 = {
                                let a800 = f145(f125(v6.clone(), v7.clone()))?;
                                length_list(a800)
                            };
                            let a803 = {
                                let a801 = f125(v6.clone(), v7);
                                length_list(a801)
                            };
                            nat_eq(a802, a803)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f288(f153(), f220(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f122(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f178(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m804 = v2;
    match m804.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m805 = v4;
            match m805.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m806 = v6.clone();
                    match m806.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((_, _)) => {
                            let m807 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f289(), List::<Bytes>::nil())));
                            match m807.uncons() {
                                None => {
                                    let v11: Bytes = f122(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt2> = f96(v0, f289())?;
                                    if f290(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f288(f289(), f220(v13, List::cons(f291(v11.clone(), v12, v5, v3), List::<Adt2>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f179(v0: Bytes, p861: u64, p862: u64) -> R<bool> {
    let m860 = (p861, p862);
    {
        let mut m860 = m860;
        loop {
            let step = {
                let (v1, v2) = m860;
                let m808 = v2;
                if m808 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m808 - 1;
                    let m811 = {
                        let a809 = v0.clone();
                        let a810 = v1;
                        index_bytes(a809, a810)
                    };
                    match m811 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f292({
                                let v5: u8 = v4;
                                let a856 = {
                                    let a832 = {
                                        let a820 = {
                                            let a814 = {
                                                let a812 = v5;
                                                let a813 = 128u8;
                                                fixed_u8::bit_and(a812, a813)
                                            };
                                            let a815 = 128u8;
                                            if equal(a814, a815) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a821 = {
                                            let a818 = {
                                                let a816 = v5;
                                                let a817 = 64u8;
                                                fixed_u8::bit_and(a816, a817)
                                            };
                                            let a819 = 64u8;
                                            if equal(a818, a819) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a820, a821)?
                                    };
                                    let a833 = {
                                        let a830 = {
                                            let a824 = {
                                                let a822 = v5;
                                                let a823 = 32u8;
                                                fixed_u8::bit_and(a822, a823)
                                            };
                                            let a825 = 32u8;
                                            if equal(a824, a825) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a831 = {
                                            let a828 = {
                                                let a826 = v5;
                                                let a827 = 16u8;
                                                fixed_u8::bit_and(a826, a827)
                                            };
                                            let a829 = 16u8;
                                            if equal(a828, a829) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a830, a831)?
                                    };
                                    nat_add(a832, a833)?
                                };
                                let a857 = {
                                    let a854 = {
                                        let a842 = {
                                            let a836 = {
                                                let a834 = v5;
                                                let a835 = 8u8;
                                                fixed_u8::bit_and(a834, a835)
                                            };
                                            let a837 = 8u8;
                                            if equal(a836, a837) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a843 = {
                                            let a840 = {
                                                let a838 = v5;
                                                let a839 = 4u8;
                                                fixed_u8::bit_and(a838, a839)
                                            };
                                            let a841 = 4u8;
                                            if equal(a840, a841) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a842, a843)?
                                    };
                                    let a855 = {
                                        let a852 = {
                                            let a846 = {
                                                let a844 = v5;
                                                let a845 = 2u8;
                                                fixed_u8::bit_and(a844, a845)
                                            };
                                            let a847 = 2u8;
                                            if equal(a846, a847) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a853 = {
                                            let a850 = {
                                                let a848 = v5;
                                                let a849 = 1u8;
                                                fixed_u8::bit_and(a848, a849)
                                            };
                                            let a851 = 1u8;
                                            if equal(a850, a851) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a852, a853)?
                                    };
                                    nat_add(a854, a855)?
                                };
                                nat_add(a856, a857)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a858 = v1;
                                    let a859 = 1u64;
                                    nat_add(a858, a859)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m860 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(p867: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m866 = p867;
    {
        let mut m866 = m866;
        loop {
            let step = {
                let v0 = m866;
                let m863 = v0;
                match m863.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h864, _) = v2;
                            h864
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h865) = v2;
                                h865
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m866 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m871 = {
        let a869 = match v0 {
            Adt1::C0(_, _, _, h868) => {
                h868
            }
        };
        let a870 = f40(v1);
        index_list(a869, a870)
    };
    match m871 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f182(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), v1) {
        let m872 = f44(v0, 34u8)?;
        match m872.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m873 = v3;
                match m873.uncons() {
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

pub fn f183(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f293(v0, v1)
}

pub fn f184(v0: Option<Adt2>) -> List<Adt2> {
    let m874 = v0;
    match m874 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f294(v1)
        }
    }
}

pub fn f185(v0: Adt2, v1: Bytes) -> Adt2 {
    let m875 = v0;
    match m875 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r876) => {
            let v12 = (*r876).clone();
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
        Adt2::C7(r877) => {
            let v10 = (*r877).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r878) => {
            let v5 = (*r878).clone();
            Adt2::C8(std::rc::Rc::new(f185(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r879, r880) => {
            let v18 = (*r879).clone();
            let v19 = (*r880).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r881, r882) => {
            let v3 = (*r881).clone();
            let v4 = (*r882).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt2::C11(v2, std::rc::Rc::new(f295(v3, v1.clone())), std::rc::Rc::new(f185(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f185(v4, v1)))
            }
        }
    }
}

pub fn f186(v0: Adt2) -> R<Bytes> {
    let m883 = v0;
    match m883 {
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
            f296(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f297(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f298(v9, v10)
        }
        Adt2::C6(v11) => {
            f299(v11)
        }
        Adt2::C7(r884) => {
            let v12 = (*r884).clone();
            Ok(f300(f301(v12)?))
        }
        Adt2::C8(r885) => {
            let v13 = (*r885).clone();
            f302(f303(v13)?)
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

pub fn f187(v0: Adt1, p889: List<Adt13>) -> R<Option<Adt13>> {
    let m888 = p889;
    {
        let mut m888 = m888;
        loop {
            let step = {
                let v1 = m888;
                let m886 = v1;
                match m886.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f188(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h887, _) => {
                                h887
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
                Ok(next) => { m888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Adt13) -> R<Bytes> {
    Ok(f238(f304(), f268(f305(v0))?))
}

pub fn f189(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f305(v0), List::<Bytes>::nil())))))
}

pub fn f190() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f191() -> Bytes {
    f306()
}

pub fn f192(v0: List<Adt13>) -> R<List<Bytes>> {
    let m890 = v0;
    match m890.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f188(v1)?, f192(v2)?))
        }
    }
}

pub fn f193(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m891 = v1;
    match m891.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f307(v5.clone(), v2.clone(), v3.clone(), f280(f64(v0.clone(), f188(v5)?))?, v4.clone())?), f193(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f194(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f308(v0.clone())?;
    let v2: u64 = f309(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a892 = v1;
        let a893 = v2;
        nat_sub(a892, a893)
    }))
}

pub fn f195(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a896 = v3;
    let a897 = f46(v0, v1, {
        let a894 = v2;
        let a895 = v1;
        nat_sub(a894, a895)
    });
    append_bytes(a896, a897)
}

pub fn f196(v0: Bytes, p910: u64, p911: u64) -> R<u64> {
    let m909 = (p910, p911);
    {
        let mut m909 = m909;
        loop {
            let step = {
                let (v1, v2) = m909;
                let m898 = v2;
                if m898 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m898 - 1;
                    let v4: u64 = f115(v0.clone(), v1)?;
                    let a899 = v4;
                    let a900 = 34u64;
                    if nat_eq(a899, a900) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a901 = v4;
                        let a902 = 92u64;
                        if nat_eq(a901, a902) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a903 = v4;
                            let a904 = 32u64;
                            if nat_lt(a903, a904) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a905 = v4;
                                let a906 = 256u64;
                                if nat_eq(a905, a906) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a907 = v1;
                                        let a908 = 1u64;
                                        nat_add(a907, a908)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Bytes, p937: u64, p938: u64, p939: Bytes) -> R<Option<(u64, Bytes)>> {
    let m936 = (p937, (p938, p939));
    {
        let mut m936 = m936;
        loop {
            let step = {
                let (v1, (v2, v3)) = m936;
                let m912 = v2;
                if m912 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m912 - 1;
                    let v5: u64 = f196(v0.clone(), v1, {
                        let a916 = {
                            let a914 = {
                                let a913 = v0.clone();
                                length_bytes(a913)
                            };
                            let a915 = v1;
                            nat_sub(a914, a915)
                        };
                        let a917 = 1u64;
                        nat_add(a916, a917)?
                    })?;
                    let v6: Bytes = {
                        let a920 = v3;
                        let a921 = f46(v0.clone(), v1, {
                            let a918 = v5;
                            let a919 = v1;
                            nat_sub(a918, a919)
                        });
                        append_bytes(a920, a921)
                    };
                    let v7: u64 = f115(v0.clone(), v5)?;
                    let a922 = v7;
                    let a923 = 34u64;
                    if nat_eq(a922, a923) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a924 = v5;
                            let a925 = 1u64;
                            nat_add(a924, a925)?
                        }, v6))))
                    } else {
                        let a926 = v7;
                        let a927 = 92u64;
                        if nat_eq(a926, a927) {
                            let m928 = f310(v0.clone(), v5, v6)?;
                            match m928 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a931 = {
                                        let (_, h929) = {
                                            let (_, h930) = v8.clone();
                                            h930
                                        };
                                        h929
                                    };
                                    let a932 = 0u64;
                                    if nat_eq(a931, a932) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h933, _) = v8;
                                            h933
                                        }, (v4, {
                                            let (h934, _) = {
                                                let (_, h935) = v8;
                                                h935
                                            };
                                            h934
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
                Ok(next) => { m936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: Bytes, v1: u64) -> R<u64> {
    let (h940, _) = f311(Fn3::F312(v0.clone()), {
        let a942 = {
            let a941 = v0;
            length_bytes(a941)
        };
        let a943 = 1u64;
        nat_add(a942, a943)?
    }, v1)?;
    Ok(h940)
}

pub fn f199(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f200(v0: Bytes) -> R<u64> {
    let (h944, _) = f311(Fn3::F313(v0.clone()), {
        let a946 = {
            let a945 = v0;
            length_bytes(a945)
        };
        let a947 = 1u64;
        nat_add(a946, a947)?
    }, 0u64)?;
    Ok(h944)
}

pub fn f201(v0: Bytes) -> R<Option<u64>> {
    if f314(v0.clone())? {
        Ok(Some({
            let (_, h948) = {
                let (h949, _) = f315(Fn4::F316(v0.clone()), {
                    let a950 = v0;
                    length_bytes(a950)
                }, (0u64, 0u64))?;
                h949
            };
            h948
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f202(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a952 = {
            let a951 = v1.clone();
            length_bytes(a951)
        };
        let a953 = f317(v1)?;
        nat_sub(a952, a953)
    });
    let v5: Bytes = {
        let a954 = v0;
        let a955 = v4.clone();
        append_bytes(a954, a955)
    };
    let v6: u64 = f200(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a957 = {
            let a956 = v5;
            length_bytes(a956)
        };
        let a958 = v6;
        nat_sub(a957, a958)
    });
    let v8: u64 = {
        let a959 = v4;
        length_bytes(a959)
    };
    let v9: u64 = {
        let a960 = v7.clone();
        length_bytes(a960)
    };
    let a961 = v9;
    let a962 = 0u64;
    if nat_eq(a961, a962) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f199(v2, {
            let a963 = v3;
            let a964 = v8;
            nat_lt(a963, a964)
        }) {
            let v10: u64 = if v2 {
                let a965 = v3;
                let a966 = v8;
                nat_add(a965, a966)?
            } else {
                let a967 = v8;
                let a968 = v3;
                nat_sub(a967, a968)
            };
            let v11: u64 = f317(v7.clone())?;
            let a969 = v10;
            let a970 = v11;
            if nat_le(a969, a970) {
                Ok(Some((f46(v7, 0u64, {
                    let a971 = v9;
                    let a972 = v10;
                    nat_sub(a971, a972)
                }), 0u64)))
            } else {
                let a975 = {
                    let a973 = v10;
                    let a974 = v11;
                    nat_sub(a973, a974)
                };
                let a976 = 19u64;
                if nat_le(a975, a976) {
                    Ok(Some((f46(v7, 0u64, {
                        let a977 = v9;
                        let a978 = v11;
                        nat_sub(a977, a978)
                    }), {
                        let a979 = v10;
                        let a980 = v11;
                        nat_sub(a979, a980)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a981 = v3;
                let a982 = v8;
                nat_sub(a981, a982)
            };
            let a985 = {
                let a983 = v9;
                let a984 = v12;
                nat_add(a983, a984)?
            };
            let a986 = 19u64;
            if nat_le(a985, a986) {
                Ok(Some(({
                    let a987 = v7;
                    let a988 = f318(v12);
                    append_bytes(a987, a988)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f203(v0: bool, v1: u64) -> bool {
    if v0 {
        let a989 = 0u64;
        let a990 = v1;
        if nat_lt(a989, a990) {
            let a991 = v1;
            let a992 = 9223372036854775808u64;
            nat_le(a991, a992)
        } else {
            false
        }
    } else {
        let a993 = v1;
        let a994 = 9223372036854775807u64;
        nat_le(a993, a994)
    }
}

pub fn f204(v0: Bytes, v1: u64) -> Option<u8> {
    let a995 = v0;
    let a996 = v1;
    index_bytes(a995, a996)
}

pub fn f205(v0: Fn2, p1000: Option<Bytes>, p1001: List<Bytes>) -> Option<Bytes> {
    let m999 = (p1000, p1001);
    {
        let mut m999 = m999;
        loop {
            let step = {
                let (v1, v2) = m999;
                let m997 = v2;
                match m997.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c998: Fn2 = v0.clone();
                            c998.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m999 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1002 = v1;
    match m1002 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1005 = {
                    let a1003 = v3;
                    let a1004 = v0;
                    append_bytes(a1003, a1004)
                };
                let a1006 = v2;
                append_bytes(a1005, a1006)
            })
        }
    }
}

pub fn f207() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f208() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f209() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f210(v0: Adt1) -> R<List<Bytes>> {
    let m1007 = f258(f64(v0, f209()))?;
    match m1007.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f260(v2))
        }
    }
}

pub fn f211(v0: List<Adt2>) -> List<Bytes> {
    let m1008 = v0;
    match m1008.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1009 = f92(Some(v1));
            match m1009 {
                None => {
                    f211(v2)
                }
                Some(v3) => {
                    List::cons(v3, f211(v2))
                }
            }
        }
    }
}

pub fn f212(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1010 = v0;
    match m1010.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f212(v2))
        }
    }
}

pub fn f213(v0: List<Adt17>) -> List<(Bytes, Bytes)> {
    let m1011 = v0;
    match m1011.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt17::C0(h1012, _, _) => {
                    h1012
                }
            }, match v1 {
                Adt17::C0(_, _, h1013) => {
                    h1013
                }
            }), f213(v2))
        }
    }
}

pub fn f214(v0: Adt14, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Adt17>> {
    let v3: List<(Bytes, Bytes)> = f319(v1);
    let v4: List<(Bytes, Bytes)> = f320(v0, v2);
    f321(v3.clone(), f154(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f215(p1020: List<Bytes>, v1: List<(Bytes, Bytes)>, p1021: u64) -> R<List<Bytes>> {
    let m1019 = (p1020, p1021);
    {
        let mut m1019 = m1019;
        loop {
            let step = {
                let (v0, v2) = m1019;
                let m1014 = v2;
                if m1014 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1014 - 1;
                    let v4: List<Bytes> = f145(f125(v0.clone(), f322(v0.clone(), v1.clone())))?;
                    let a1017 = {
                        let a1015 = v4.clone();
                        length_list(a1015)
                    };
                    let a1018 = {
                        let a1016 = v0.clone();
                        length_list(a1016)
                    };
                    if nat_eq(a1017, a1018) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1019 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(v0: List<Adt2>) -> List<Bytes> {
    let m1022 = v0;
    match m1022.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1023 = f93(v1, f8(Str::lit("std")));
            match m1023 {
                None => {
                    f216(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f217(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f216(v2))
                    } else {
                        f216(v2)
                    })
                }
            }
        }
    }
}

pub fn f217(v0: Adt2, v1: Bytes) -> bool {
    f51({
        let a1025 = {
            let a1024 = f58(v0, v1);
            length_bytes(a1024)
        };
        let a1026 = 0u64;
        nat_eq(a1025, a1026)
    })
}

pub fn f218(v0: Adt14, v1: Adt2) -> bool {
    if f323(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f323(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f219(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt2>>> {
    let m1027 = v0;
    match m1027.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f194(v2)?;
            let a1029 = {
                let a1028 = v4.clone();
                length_bytes(a1028)
            };
            let a1030 = 0u64;
            if nat_eq(a1029, a1030) {
                f219(v3, v1)
            } else {
                let m1031 = f43(v4)?;
                match m1031 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f324(v5.clone())
                        } else {
                            f325(v5.clone())
                        } {
                            let m1032 = f219(v3, v1)?;
                            match m1032 {
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

pub fn f220(v0: List<Adt2>, v1: List<Adt2>) -> List<Adt2> {
    let m1033 = v0;
    match m1033.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f220(v3, v1))
        }
    }
}

pub fn f221(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1034 = v1;
    match m1034.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f238(v0.clone(), v2), f221(v0, v3))
            } else {
                f221(v0, v3)
            }
        }
    }
}

pub fn f222() -> Bytes {
    f8(Str::lit("set -euo pipefail; repo=$1; rev=$2; shift 2; d=target/prismpm; if [ \"$(git -C $d rev-parse HEAD 2>/dev/null)\" != \"$rev\" ]; then rm -rf $d; git init -q $d; git -C $d remote add origin \"https://github.com/$repo\"; git -C $d sparse-checkout set --no-cone sdk/npm/prism-code; git -C $d fetch -q --depth=1 --filter=blob:none origin \"$rev\"; git -C $d checkout -q FETCH_HEAD; fi; exec node $d/sdk/npm/prism-code/bin/prism-code.mjs \"$@\""))
}

pub fn f223() -> Bytes {
    f8(Str::lit("029e9bcfe0c56deb652dd2d77ad3b3fe46cf2dcd"))
}

pub fn f224(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1035 = v1;
    match m1035.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f125({
                let a1037 = {
                    let a1036 = f194(v4.clone())?;
                    length_bytes(a1036)
                };
                let a1038 = 0u64;
                if nat_eq(a1037, a1038) {
                    List::<Bytes>::nil()
                } else {
                    f326(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f224(v0, v5, v2, v3)?))
        }
    }
}

pub fn f225(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f226() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f209(), List::<Bytes>::nil())))))))
}

pub fn f227(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1039 = v1;
    match m1039.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m1040 = f57(v0.clone(), v2.clone())?;
                match m1040 {
                    None => {
                        Ok(List::cons(v2, f227(v0, v3)?))
                    }
                    Some(_) => {
                        f227(v0, v3)
                    }
                }
            } else {
                f227(v0, v3)
            }
        }
    }
}

pub fn f228(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m1041 = v1;
    match m1041.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f138(v0.clone(), v2.clone()) {
                f228(v0, v3)
            } else {
                List::cons(v2, f228(v0, v3))
            }
        }
    }
}

pub fn f229() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f230(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m1042 = v0;
    match m1042.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f125(f327({
                let (h1043, _) = v2;
                h1043
            }, {
                let (_, h1044) = v2;
                h1044
            }, v1), f230(v3, v1))
        }
    }
}

pub fn f231() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::cons(f8(Str::lit(".github/scripts/readmit.sh")), List::<Bytes>::nil())))))
}

pub fn f232(v0: Adt1) -> R<Option<Bytes>> {
    f328(f329(v0, f231())?)
}

pub fn f233(v0: Adt1, p1048: List<Bytes>) -> R<Option<Bytes>> {
    let m1047 = p1048;
    {
        let mut m1047 = m1047;
        loop {
            let step = {
                let v1 = m1047;
                let m1045 = v1;
                match m1045.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1046 = f330(f64(v0.clone(), v2.clone()), f331())?;
                        match m1046 {
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
                Ok(next) => { m1047 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h1049, _, _, _, _, _) => {
            h1049
        }
    };
    let v2: Adt18 = f332(v1.clone())?;
    let v3: List<Bytes> = f333(v1.clone());
    let v4: List<Bytes> = f142(match v0.clone() {
        Adt16::C0(_, _, _, h1050, _, _) => {
            h1050
        }
    }, f8(Str::lit("path")));
    let m1052 = f334(v2, List::<u64>::nil(), f335(v1.clone(), match v0.clone() {
        Adt16::C0(_, h1051, _, _, _, _) => {
            h1051
        }
    }, 0u64)?);
    match m1052 {
        None => {
            if f337(f338(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1054) => {
                    h1054
                }
            }, f125(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1055) => {
                    h1055
                }
            }, v4.clone()))?, f338(v4.clone(), f125(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1056) => {
                    h1056
                }
            }, v4.clone()))?, f339(match v0.clone() {
                Adt16::C0(_, _, _, h1057, _, _) => {
                    h1057
                }
            })) {
                let m1061 = f340(f338(match v0.clone() {
                    Adt16::C0(_, _, _, _, h1058, _) => {
                        h1058
                    }
                }, f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, _, h1059, _) => {
                        h1059
                    }
                }))?, f338(v3.clone(), f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, _, h1060, _) => {
                        h1060
                    }
                }))?);
                match m1061 {
                    None => {
                        let m1064 = f341(f142(match v0.clone() {
                            Adt16::C0(_, h1063, _, _, _, _) => {
                                h1063
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m1064 {
                            None => {
                                let m1065 = f341(v4, List::<Bytes>::nil());
                                match m1065 {
                                    None => {
                                        let m1067 = f342(v1, match v0 {
                                            Adt16::C0(_, _, h1066, _, _, _) => {
                                                h1066
                                            }
                                        });
                                        match m1067 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f336(f125(v3, match v0 {
                            Adt16::C0(_, _, _, _, h1062, _) => {
                                h1062
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f336(match v1 {
                Adt15::C0(_, _, _, _, h1053) => {
                    h1053
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f235(v0: List<Bytes>) -> List<Bytes> {
    f120(Fn1::F343, List::<Bytes>::nil(), v0)
}

pub fn f236(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f344(v0, v1)
}

pub fn f237(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f345(f346(f347(v0))?))
}

pub fn f238(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f239(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1068 = f57(v0, v2)?;
    match m1068 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f348(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1069 = v4.clone();
            match m1069.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1071 = f92(f93({
                        let m1070 = f93(v3.clone(), f8(Str::lit("source")));
                        match m1070 {
                            None => {
                                Adt2::C9
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1071 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f349(f350(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f240(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1072 = v0;
    match m1072.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f351(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1076 = f44(f46(v1.clone(), 6u64, {
                    let a1074 = {
                        let a1073 = v1;
                        length_bytes(a1073)
                    };
                    let a1075 = 6u64;
                    nat_sub(a1074, a1075)
                }), 34u8)?;
                match m1076.uncons() {
                    None => {
                        f240(v2)
                    }
                    Some((v3, v4)) => {
                        let m1077 = v4;
                        match m1077.uncons() {
                            None => {
                                f240(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f240(v2)?))
                            }
                        }
                    }
                }
            } else {
                f240(v2)
            }
        }
    }
}

pub fn f241(p1080: List<Bytes>, p1081: List<Bytes>) -> List<Bytes> {
    let m1079 = (p1080, p1081);
    {
        let mut m1079 = m1079;
        loop {
            let step = {
                let (v0, v1) = m1079;
                let m1078 = v0;
                match m1078.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1079 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f243(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1082 = v1;
    match m1082.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f352(v0.clone(), v3)?;
            if f52(f353(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f354(v5)?, f243(v0, v4, v2)?))
            } else {
                f243(v0, v4, v2)
            }
        }
    }
}

pub fn f244(v0: Adt1, p1086: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1085 = p1086;
    {
        let mut m1085 = m1085;
        loop {
            let step = {
                let v1 = m1085;
                let m1083 = v1;
                match m1083.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f352(v0.clone(), v3.clone())?;
                        if if f52(f353(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f100({
                                let m1084 = f93(v5.clone(), f8(Str::lit("schema")));
                                match m1084 {
                                    None => {
                                        Adt2::C9
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f354(v5)?))
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
                Ok(next) => { m1085 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h1087, _, _) => {
            h1087
        }
    }, f267())) {
        Ok(Adt12::C1(List::cons(f267(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f355(f147(match v0.clone() {
            Adt1::C0(_, h1088, _, _) => {
                h1088
            }
        }, f267())))?, v1.clone());
        let m1089 = f56(v0.clone(), List::cons(f153(), List::cons(f209(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1089.uncons() {
            None => {
                let m1090 = f56(v0.clone(), f356(v0.clone(), v2.clone())?);
                match m1090.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f357(v0.clone(), v2, v1)?;
                        let (h1091, _) = v7;
                        if h1091 {
                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                let (_, h1092) = v7;
                                h1092
                            }))
                        } else {
                            let m1093 = f124(v0.clone());
                            match m1093.uncons() {
                                None => {
                                    let m1094 = f126(v0.clone())?;
                                    match m1094.uncons() {
                                        None => {
                                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                                let (_, h1095) = v7;
                                                h1095
                                            }, List::cons(f7(f145(f358(f127(v0.clone())?, f135(v0.clone(), f136(v0.clone())?)?))?), List::cons(f359(v0)?, List::<Bytes>::nil()))))))
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

pub fn f246(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f360(f96(v0, f153())?)?)))
}

pub fn f247() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# The one way an agent changes this repository: dispatch this workflow with an\u{a}")), List::cons(f8(Str::lit("# operation of the API. It runs the certified command, and opens a pull request\u{a}")), List::cons(f8(Str::lit("# as the Verdant Studio GitHub App, which merges it once the studio check passes.\u{a}")), List::cons(f8(Str::lit("name: operate\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}")), List::cons(f8(Str::lit("# that defers to an open advance pull request always has a successor when that merges.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("    inputs:\u{a}")), List::cons(f8(Str::lit("      args:\u{a}")), List::cons(f8(Str::lit("        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}")), List::cons(f8(Str::lit("        required: true\u{a}")), List::cons(f8(Str::lit("        default: 'advance'\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input:\u{a}")), List::cons(f8(Str::lit("        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input_name:\u{a}")), List::cons(f8(Str::lit("        description: 'File name for input'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("        default: 'input'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Only operations that write the registers share one group. GitHub keeps one\u{a}")), List::cons(f8(Str::lit("# pending run per group, so a staging or propose-command run gets its own.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  operate:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 30\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      # Main moves about every 10 minutes, and refresh's regeneration of an API change plus its check takes\u{a}")), List::cons(f8(Str::lit("      # about 12, so an operation merged in that window pushes the change behind main again (#309, #315).\u{a}")), List::cons(f8(Str::lit("      # refresh keeps a regenerated change ready with its auto-merge set, so this run waits for every such\u{a}")), List::cons(f8(Str::lit("      # change whose studio check has not failed to merge first, then starts from the new main. It waits at\u{a}")), List::cons(f8(Str::lit("      # most 10 minutes, so neither queue stalls. A new propose-command waits too, so it never merges ahead of an\u{a}")), List::cons(f8(Str::lit("      # older change that is being regenerated; revert-merge, the repair of a broken path, never waits.\u{a}")), List::cons(f8(Str::lit("      - name: wait for the API changes that are next to merge (every operation but revert-merge)\u{a}")), List::cons(f8(Str::lit("        if: ${{ !startsWith(inputs.args, 'revert-merge ') }}\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          for try in $(seq 1 20); do\u{a}")), List::cons(f8(Str::lit("            next=''\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio propose-command\")) and .auto_merge != null and (.draft | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then next=\"$next #${pr}\"; fi\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f248() -> Bytes {
    f7(List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -z \"$next\" ]; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("            echo \"waiting for${next} to merge (check $try of 20)\"\u{a}")), List::cons(f8(Str::lit("            sleep 30\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          echo \"::warning::${next# } still open after 10 minutes, so this run goes on\"\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}")), List::cons(f8(Str::lit("        if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          mkdir -p target/input\u{a}")), List::cons(f8(Str::lit("          cat > target/input/change.patch <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}")), List::cons(f8(Str::lit("          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}")), List::cons(f8(Str::lit("          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^admit/' -e '^lexlean-workspace.toml$' -e '^studio/lexlean.toml$' -e '^studio/production-request.json$' -e '^app/lexlean.toml$' -e '^stdlib/lexlean.toml$' -e '^.github/lexlean.pin$' || true)\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers, admit, lexlean-workspace.toml, the studio, app and stdlib lexlean.toml, studio/production-request.json and .github/lexlean.pin, not: $outside\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          # studio/lexlean.lock, readers/lexlean.lock, readers/xml/lexlean.lock and admit/lexlean.lock are generated, and the export below writes them again.\u{a}")), List::cons(f8(Str::lit("          git apply --exclude=studio/lexlean.lock --exclude=readers/lexlean.lock --exclude=readers/xml/lexlean.lock --exclude=admit/lexlean.lock target/input/change.patch\u{a}")), List::cons(f8(Str::lit("          # The owner reads this pull request, so its body leads with the proposer's own account (the prose\u{a}")), List::cons(f8(Str::lit("          # before the first diff), then the definitions it changes. Checked before the export, which takes minutes.\u{a}")), List::cons(f8(Str::lit("          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}")), List::cons(f8(Str::lit("          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}")), List::cons(f8(Str::lit("            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The export refuses Lean output holding an absolute path (LLV7006) only at its end, so refuse an added\u{a}")), List::cons(f8(Str::lit("          # string literal with a slash at a token boundary before a name character here, in seconds.\u{a}")), List::cons(f8(Str::lit("          if grep '^+[^+]' target/input/change.patch | grep '\"' | grep -E \"(^|[[:space:]\\\"'\\`([{<,;=])/[[:alnum:]_.]\" > target/absolute.txt; then\u{a}")), List::cons(f8(Str::lit("            echo \"::error::LLV7006: these added lines put a slash at a token boundary, which the export refuses as an absolute path (write >/dev/null with no space):\"\u{a}")), List::cons(f8(Str::lit("            cat target/absolute.txt\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          # The one export, shared with refresh and run end to end by the studio check.\u{a}")), List::cons(f8(Str::lit("          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f361(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("            tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}")), List::cons(f8(Str::lit("          {\u{a}")), List::cons(f8(Str::lit("            cat target/summary.md\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"### Definitions this adds or changes\"\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f249() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            {\u{a}")), List::cons(f8(Str::lit("              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}")), List::cons(f8(Str::lit("              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}")), List::cons(f8(Str::lit("            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            slash=/\u{a}")), List::cons(f8(Str::lit("            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            head -n 8 target/export.log.phases || true\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"<${slash}details>\"\u{a}")), List::cons(f8(Str::lit("          } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("      - id: command\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: |\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_readers\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_xml\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_admit\u{a}")), List::cons(f8(Str::lit("          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}")), List::cons(f8(Str::lit("        if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          target/lexlean/bin/lexlean production verify-artifact certified\u{a}")), List::cons(f8(Str::lit("          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}")), List::cons(f8(Str::lit("          if [ -d certified-readers ]; then\u{a}")), List::cons(f8(Str::lit("            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}")), List::cons(f8(Str::lit("            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}")), List::cons(f8(Str::lit("      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("          INPUT_NAME: ${{ inputs.input_name }}\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail -f\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          # read-* operations run in their reader project's command when it is built (the XML ones in verdant_xml), the admission operations in verdant_admit, every other in the studio command.\u{a}")), List::cons(f8(Str::lit("          vs() { case \"$1\" in admit|admit-encoding|admit-registry|admit-translated|propose-encoding) if [ -x target/release/verdant_admit ]; then target/release/verdant_admit \"$@\"; return; fi;; read-xml|read-xsd|read-xsd-components) if [ -x target/release/verdant_xml ]; then target/release/verdant_xml \"$@\"; return; fi;; read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f250() -> Bytes {
    f7(List::cons(f8(Str::lit("          # An App pull request whose files already hold on main has nothing left to merge,\u{a}")), List::cons(f8(Str::lit("          # so it is closed; the propose-command ones change the API and stay open until they merge.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}")), List::cons(f8(Str::lit("              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}")), List::cons(f8(Str::lit("              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}")), List::cons(f8(Str::lit("              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}")), List::cons(f8(Str::lit("                exit 0\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            # A change to a reader re-derives the encodings it printed before the studio check replays them.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then\u{a}")), List::cons(f8(Str::lit("              readmitted=$(bash .github/scripts/readmit.sh target/release/verdant_readers; if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi)\u{a}")), List::cons(f8(Str::lit("              if [ -n \"$readmitted\" ]; then\u{a}")), List::cons(f8(Str::lit("                { echo; echo \"### Encodings admitted again\"; echo; echo \"This change alters what a reader prints for these admitted encodings, so each was admitted again through read-admit with the command and citation its row records:\"; echo; sed 's/^/- /' <<< \"$readmitted\"; } >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            cat target/operation.txt\u{a}")), List::cons(f8(Str::lit("          elif [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            : > target/operation.txt\u{a}")), List::cons(f8(Str::lit("            last=''\u{a}")), List::cons(f8(Str::lit("            skipped=''\u{a}")), List::cons(f8(Str::lit("            # next prints one operation it can dispatch, or else the lines that say why the\u{a}")), List::cons(f8(Str::lit("            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}")), List::cons(f8(Str::lit("            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}")), List::cons(f8(Str::lit("              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("              if vs $op > target/attempt.txt 2>&1; then\u{a}")), List::cons(f8(Str::lit("                tee -a target/operation.txt < target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                last=$op\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                cat target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                binding=${op#* }\u{a}")), List::cons(f8(Str::lit("                echo \"::warning::$op was refused; the other bindings continue\"\u{a}")), List::cons(f8(Str::lit("                echo \"refused: $op\" >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}")), List::cons(f8(Str::lit("                skipped=\"$skipped $binding\"\u{a}")), List::cons(f8(Str::lit("                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            t0=$EPOCHREALTIME\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}")), List::cons(f8(Str::lit("            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}")), List::cons(f8(Str::lit("            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}")), List::cons(f8(Str::lit("              echo \"waiting merge: #${pr} changes the API, and merges once the studio check passes\" >> target/waiting.txt\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}")), List::cons(f8(Str::lit("          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f251() -> Bytes {
    f7(List::cons(f8(Str::lit("            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}")), List::cons(f8(Str::lit("            # good main commit, it reverts the merge on main as the App, and merges once the studio check passes.\u{a}")), List::cons(f8(Str::lit("            merge=${ARGS#revert-merge }\u{a}")), List::cons(f8(Str::lit("            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B revert origin/main\u{a}")), List::cons(f8(Str::lit("            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("            git revert -m 1 --no-commit \"$merge\"\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::cons(f8(Str::lit("            vs $ARGS | tee target/operation.txt\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("      - name: open a pull request as the Verdant Studio app\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          git add -A\u{a}")), List::cons(f8(Str::lit("          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}")), List::cons(f8(Str::lit("          verdant_studio $ARGS\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("          $(cat target/operation.txt)\u{a}")), List::cons(f8(Str::lit("          MSG\u{a}")), List::cons(f8(Str::lit("          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}")), List::cons(f8(Str::lit("          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}")), List::cons(f8(Str::lit("            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          title=\"verdant_studio $ARGS\"\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}")), List::cons(f8(Str::lit("          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}")), List::cons(f8(Str::lit("          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}")), List::cons(f8(Str::lit("          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}")), List::cons(f8(Str::lit("          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}")), List::cons(f8(Str::lit("          pr=\"\"\u{a}")), List::cons(f8(Str::lit("          for try in 1 2 3 4; do\u{a}")), List::cons(f8(Str::lit("            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            pr=\"\"\u{a}")), List::cons(f8(Str::lit("            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}")), List::cons(f8(Str::lit("            sleep $((2 ** try))\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          echo \"opened #${pr}\"\u{a}")), List::cons(f8(Str::lit("          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("          # A change to the API itself (propose-command, revert-merge) merges as every App pull request does:\u{a}")), List::cons(f8(Str::lit("          # the owner automated these approvals (2026-11-26), so the studio check, which runs the API's own\u{a}")), List::cons(f8(Str::lit("          # end-to-end propose run, is its gate. refresh keeps its auto-merge while it regenerates it.\u{a}")), List::cons(f8(Str::lit("          if [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}")), List::cons(f8(Str::lit("            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f252() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::<Bytes>::nil())))
}

pub fn f253() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# When main moves, every open propose-command pull request that no longer merges\u{a}")), List::cons(f8(Str::lit("# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}")), List::cons(f8(Str::lit("# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}")), List::cons(f8(Str::lit("# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}")), List::cons(f8(Str::lit("# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}")), List::cons(f8(Str::lit("# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}")), List::cons(f8(Str::lit("# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("# The exception is one whose auto-merge is set, the next change to merge: it stays ready and keeps its\u{a}")), List::cons(f8(Str::lit("# auto-merge, so it merges as soon as its regeneration passes the studio check. It is\u{a}")), List::cons(f8(Str::lit("# regenerated first, and only the oldest such one at a time, since two changes to the\u{a}")), List::cons(f8(Str::lit("# API conflict on their export (#309 and #315 were pushed behind main this way).\u{a}")), List::cons(f8(Str::lit("name: refresh\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}")), List::cons(f8(Str::lit("# operate run completes, which needs no token that may dispatch workflows, so every\u{a}")), List::cons(f8(Str::lit("# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_run:\u{a}")), List::cons(f8(Str::lit("    workflows: [operate]\u{a}")), List::cons(f8(Str::lit("    types: [completed]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A running refresh always finishes, because an export takes minutes and main moves more\u{a}")), List::cons(f8(Str::lit("# often than that; GitHub keeps only the newest pending run behind it.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: refresh\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  find:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 15\u{a}")), List::cons(f8(Str::lit("    outputs:\u{a}")), List::cons(f8(Str::lit("      stale: ${{ steps.stale.outputs.stale }}\u{a}")), List::cons(f8(Str::lit("      matrix: ${{ steps.stale.outputs.matrix }}\u{a}")), List::cons(f8(Str::lit("      auto: ${{ steps.stale.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - id: stale\u{a}")), List::cons(f8(Str::lit("        name: find the propose-command pull requests that conflict with main\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          stale=''\u{a}")), List::cons(f8(Str::lit("          auto=''\u{a}")), List::cons(f8(Str::lit("          # Oldest first, so the oldest one whose auto-merge is set is the one regenerated now.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}")), List::cons(f8(Str::lit("            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}")), List::cons(f8(Str::lit("            # which a cancelled run may have left behind.\u{a}")), List::cons(f8(Str::lit("            mergeable=null\u{a}")), List::cons(f8(Str::lit("            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}")), List::cons(f8(Str::lit("              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f254() -> Bytes {
    f7(List::cons(f8(Str::lit("              if [ \"$mergeable\" != null ]; then break; fi\u{a}")), List::cons(f8(Str::lit("              sleep 10\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            # One that changes .github is exported again by the studio check, byte for byte, so it also fails\u{a}")), List::cons(f8(Str::lit("            # when main changed its export inputs since it was exported, even without a conflict: readers\u{a}")), List::cons(f8(Str::lit("            # depends on app, so admissions merged during its export (#357 and #361 under #362) turn it red.\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = true ] && [ \"$(gh api --paginate \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files\" --jq '.[].filename' | grep -c '^\\.github/' || true)\" != 0 ]; then\u{a}")), List::cons(f8(Str::lit("              base=$(gh api \"repos/${GITHUB_REPOSITORY}/compare/main...$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\" --jq .merge_base_commit.sha)\u{a}")), List::cons(f8(Str::lit("              moved=$(gh api \"repos/${GITHUB_REPOSITORY}/compare/${base}...main\" --jq '[.files[]?.filename | select(test(\"^(app|stdlib|studio|readers)/\"))] | length')\u{a}")), List::cons(f8(Str::lit("              if [ \"$moved\" != 0 ]; then echo \"#${pr}: main changed ${moved} of its export inputs since it was exported\"; mergeable=false; fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = false ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              # Drafting would drop its auto-merge, so it stays ready. A later one waits: it is\u{a}")), List::cons(f8(Str::lit("              # regenerated once this one merges, when it would conflict again anyway.\u{a}")), List::cons(f8(Str::lit("              if [ -z \"$auto\" ]; then auto=$pr; else echo \"#${pr} waits for #${auto} to merge\"; fi\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = false ]; then\u{a}")), List::cons(f8(Str::lit("              stale=\"$stale $pr\"\u{a}")), List::cons(f8(Str::lit("              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}")), List::cons(f8(Str::lit("              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$auto\" ]; then stale=\" $auto$stale\"; fi\u{a}")), List::cons(f8(Str::lit("          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"auto=$auto\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("  regenerate:\u{a}")), List::cons(f8(Str::lit("    needs: find\u{a}")), List::cons(f8(Str::lit("    if: needs.find.outputs.stale != ''\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 60\u{a}")), List::cons(f8(Str::lit("    strategy:\u{a}")), List::cons(f8(Str::lit("      fail-fast: false\u{a}")), List::cons(f8(Str::lit("      matrix:\u{a}")), List::cons(f8(Str::lit("        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          fetch-depth: 0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          STALE: ${{ matrix.pr }}\u{a}")), List::cons(f8(Str::lit("          AUTO: ${{ needs.find.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A regeneration of the one whose auto-merge is set that fails leaves it a draft, which drops\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f255() -> Bytes {
    f7(List::cons(f8(Str::lit("          # its auto-merge, so it is no longer the next change to merge and nothing is handed over red.\u{a}")), List::cons(f8(Str::lit("          trap 'rc=$?; if [ \"$rc\" != 0 ] && [ \"$STALE\" = \"$AUTO\" ]; then gh pr ready --undo \"$STALE\" || true; fi' EXIT\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            mkdir -p target\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          failed=''\u{a}")), List::cons(f8(Str::lit("          for pr in $STALE; do\u{a}")), List::cons(f8(Str::lit("            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}")), List::cons(f8(Str::lit("            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}")), List::cons(f8(Str::lit("            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("            # The one whose auto-merge is set stays ready, so that it keeps its auto-merge.\u{a}")), List::cons(f8(Str::lit("            if [ \"$pr\" != \"$AUTO\" ]; then gh pr ready --undo \"$pr\" || true; fi\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin \"$branch\" main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B refresh origin/main\u{a}")), List::cons(f8(Str::lit("            git clean -fdq\u{a}")), List::cons(f8(Str::lit("            # readers/lexlean.lock and readers/xml/lexlean.lock are generated: the export below writes them again, so they are left out of the\u{a}")), List::cons(f8(Str::lit("            # change, and two reader pull requests never conflict on it.\u{a}")), List::cons(f8(Str::lit("            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml ':!readers/lexlean.lock' ':!readers/xml/lexlean.lock' ':!admit/lexlean.lock' > target/change.patch\u{a}")), List::cons(f8(Str::lit("            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}")), List::cons(f8(Str::lit("            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}")), List::cons(f8(Str::lit("            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}")), List::cons(f8(Str::lit("            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}")), List::cons(f8(Str::lit("              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}")), List::cons(f8(Str::lit("                base=$(git merge-base origin/main \"origin/$branch\")\u{a}")), List::cons(f8(Str::lit("                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}")), List::cons(f8(Str::lit("                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}")), List::cons(f8(Str::lit("                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}")), List::cons(f8(Str::lit("                git add model/strata.json\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}")), List::cons(f8(Str::lit("                git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("                failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}")), List::cons(f8(Str::lit("                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("                continue\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f361(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("              tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("              git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("              failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("              continue\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            rm -rf studio/.lexlean/production readers/.lexlean/production readers/xml/.lexlean/production admit/.lexlean/production\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            # The regenerated change re-derives the encodings whose reader output it changes, as propose-command does.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then bash .github/scripts/readmit.sh target/release/verdant_readers; fi\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("            git add -A\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f256() -> Bytes {
    f7(List::cons(f8(Str::lit("            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml; } > target/message.txt\u{a}")), List::cons(f8(Str::lit("            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}")), List::cons(f8(Str::lit("            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}")), List::cons(f8(Str::lit("            gh pr ready \"$pr\" || true\u{a}")), List::cons(f8(Str::lit("            echo \"::notice::regenerated #${pr} on the new main\"\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))))))
}

pub fn f257(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1096 = v0;
    match m1096.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f258(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt19 = f362(v0.clone(), 0u64, {
        let a1097 = v0;
        length_bytes(a1097)
    }, Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt19 = {
        let a1100 = {
            let a1099 = match v1.clone() {
                Adt19::C0(_, _, h1098, _, _) => {
                    h1098
                }
            };
            length_bytes(a1099)
        };
        let a1101 = 0u64;
        if if nat_eq(a1100, a1101) {
            let m1103 = match v1.clone() {
                Adt19::C0(_, _, _, h1102, _) => {
                    h1102
                }
            };
            match m1103.uncons() {
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
            f363(v1)
        }
    };
    Ok(f364(match v4 {
        Adt19::C0(_, _, _, _, h1104) => {
            h1104
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f259(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt17>, v3: List<Adt2>, v4: List<Bytes>) -> List<Bytes> {
    let m1105 = v1;
    match m1105.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f125(f365(v5.clone(), v2.clone()), f366(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f367(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1107 = {
                    let a1106 = v7.clone();
                    length_list(a1106)
                };
                let a1108 = 0u64;
                if nat_eq(a1107, a1108) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f122(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f259(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f260(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1109 = v0;
    match m1109.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1110 = v1;
            match m1110.uncons() {
                None => {
                    f260(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f260(v2))
                }
            }
        }
    }
}

pub fn f261(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1111 = v1;
    match m1111.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f217(v2.clone(), f8(Str::lit("domain"))) {
                f368(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1112 = f93(v2.clone(), f8(Str::lit("role")));
                    match m1112 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f369(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1113 = f93(v2.clone(), f8(Str::lit("origin")));
                    match m1113 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1114 = f93(v2, f8(Str::lit("evidence")));
                    match m1114 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f261(v0, v3))
            } else {
                f261(v0, v3)
            }
        }
    }
}

pub fn f262(v0: List<Adt2>) -> List<Bytes> {
    let m1115 = v0;
    match m1115.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f262(v2))
        }
    }
}

pub fn f263(v0: List<Adt2>) -> List<Bytes> {
    let m1116 = v0;
    match m1116.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f370(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f263(v2))
        }
    }
}

pub fn f264(p1122: List<Bytes>) -> R<bool> {
    let m1121 = p1122;
    {
        let mut m1121 = m1121;
        loop {
            let step = {
                let v0 = m1121;
                let m1117 = v0;
                match m1117.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1119 = {
                            let a1118 = f194(v1)?;
                            length_bytes(a1118)
                        };
                        let a1120 = 0u64;
                        if nat_eq(a1119, a1120) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f265(v0: List<Bytes>) -> R<bool> {
    if f274(f287(v0.clone())?) {
        Ok(f51({
            let a1124 = {
                let a1123 = f276(f287(v0)?);
                length_list(a1123)
            };
            let a1125 = 0u64;
            nat_eq(a1124, a1125)
        }))
    } else {
        Ok(false)
    }
}

pub fn f266(v0: List<Bytes>) -> R<bool> {
    if f274(f371(v0.clone(), f8(Str::lit("requirement")))?) {
        if f274(f371(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1127 = {
                    let a1126 = f220(f276(f371(v0.clone(), f8(Str::lit("requirement")))?), f276(f371(v0, f8(Str::lit("admission")))?));
                    length_list(a1126)
                };
                let a1128 = 0u64;
                nat_eq(a1127, a1128)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f267() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f268(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f269() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f270(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1129 = v0;
    match m1129.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f194(v1)?;
            let a1131 = {
                let a1130 = v3.clone();
                length_bytes(a1130)
            };
            let a1132 = 0u64;
            if nat_eq(a1131, a1132) {
                f270(v2)
            } else {
                let m1133 = f43(v3.clone())?;
                match m1133 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f372(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1134 = f270(v2)?;
                            match m1134 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f373(v5), {
                                        let a1135 = v3;
                                        let a1136 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1135, a1136)
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

pub fn f271() -> Bytes {
    let a1137 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1137)
}

pub fn f272(p1142: List<Bytes>) -> R<Option<Bytes>> {
    let m1141 = p1142;
    {
        let mut m1141 = m1141;
        loop {
            let step = {
                let v0 = m1141;
                let m1138 = v0;
                match m1138.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1139 = f43(f194(v1)?)?;
                        match m1139 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1140 = f93(v3.clone(), f8(Str::lit("source")));
                                match m1140 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f374(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m1141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: Bytes) -> Bytes {
    let a1147 = {
        let a1143 = Str::lit("binding ");
        utf8_encode(a1143)
    };
    let a1148 = {
        let a1145 = v0;
        let a1146 = {
            let a1144 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1144)
        };
        append_bytes(a1145, a1146)
    };
    append_bytes(a1147, a1148)
}

pub fn f274(v0: Option<List<Adt2>>) -> bool {
    let m1149 = v0;
    match m1149 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f275(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1150 = v0;
    match m1150.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f194(v1)?;
            let a1152 = {
                let a1151 = v3.clone();
                length_bytes(a1151)
            };
            let a1153 = 0u64;
            if nat_eq(a1152, a1153) {
                f275(v2)
            } else {
                let m1154 = f43(v3)?;
                match m1154 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if f117(f117(f217(v4.clone(), f8(Str::lit("authority"))), f217(v4.clone(), f8(Str::lit("row")))), f117(f117(f217(v4.clone(), f8(Str::lit("construct"))), f217(v4.clone(), f8(Str::lit("artifact")))), f375(f58(v4.clone(), f8(Str::lit("reader"))), f58(v4.clone(), f8(Str::lit("construct")))))) {
                            let m1155 = f275(v2)?;
                            match m1155 {
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

pub fn f276(v0: Option<List<Adt2>>) -> List<Adt2> {
    let m1156 = v0;
    match m1156 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f277(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1157 = v0;
    match m1157.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f194(v1)?;
            let a1159 = {
                let a1158 = v3.clone();
                length_bytes(a1158)
            };
            let a1160 = 0u64;
            if nat_eq(a1159, a1160) {
                f277(v2)
            } else {
                let m1161 = f43(v3.clone())?;
                match m1161 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f376(v4.clone()) {
                            let m1162 = f277(v2)?;
                            match m1162 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f242(), List::cons(f8(Str::lit("/")), List::cons(f377(f58(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1163 = v3;
                                        let a1164 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1163, a1164)
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

pub fn f278() -> Bytes {
    f378()
}

pub fn f279(v0: Adt1, v1: Adt2, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f284(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f379(v1, v4)?)?), List::cons(f285(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f280(v0: Bytes) -> R<Bytes> {
    Ok(f380(f381(v0)?))
}

pub fn f281(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1165 = v0;
    match m1165.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f282() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f283(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f349(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f284(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f382(f150(f95(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f383(f384(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f285(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f100(Adt2::C7(std::rc::Rc::new(f385(f145(f125(v1, f211(f96(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f286(p1168: List<Adt2>) -> bool {
    let m1167 = p1168;
    {
        let mut m1167 = m1167;
        loop {
            let step = {
                let v0 = m1167;
                let m1166 = v0;
                match m1166.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f142(f386(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1169 = v0;
    match m1169.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f194(v1)?;
            let a1171 = {
                let a1170 = v3.clone();
                length_bytes(a1170)
            };
            let a1172 = 0u64;
            if nat_eq(a1171, a1172) {
                f287(v2)
            } else {
                let m1173 = f43(v3)?;
                match m1173 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if if f217(v4.clone(), f8(Str::lit("agent"))) {
                            if f217(v4.clone(), f8(Str::lit("at"))) {
                                if f217(v4.clone(), f8(Str::lit("quote"))) {
                                    f387(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1174 = f287(v2)?;
                            match m1174 {
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

pub fn f288(v0: Bytes, v1: List<Adt2>) -> R<(Bytes, Bytes)> {
    Ok((v0, f100(Adt2::C7(std::rc::Rc::new(f101(v1))))?))
}

pub fn f289() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f290(p1177: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m1176 = p1177;
    {
        let mut m1176 = m1176;
        loop {
            let step = {
                let v0 = m1176;
                let m1175 = v0;
                match m1175.uncons() {
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
                Ok(next) => { m1176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt2 {
    Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt2::C6(v2)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt2::C6(v0)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9))))))))))))
}

pub fn f292(v0: u64) -> bool {
    let a1178 = v0;
    let a1179 = 127u64;
    nat_le(a1178, a1179)
}

pub fn f293(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1180, _) = v1.clone();
        h1180
    };
    let v3: u64 = {
        let (h1181, _) = {
            let (_, h1182) = v1.clone();
            h1182
        };
        h1181
    };
    let v4: List<Bytes> = {
        let (_, h1183) = {
            let (_, h1184) = v1;
            h1184
        };
        h1183
    };
    let m1185 = f204(v0.clone(), v2);
    match m1185 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1186 = v5;
            let a1187 = 10u8;
            if equal(a1186, a1187) {
                Ok(Some(({
                    let a1188 = v2;
                    let a1189 = 1u64;
                    nat_add(a1188, a1189)?
                }, ({
                    let a1190 = v2;
                    let a1191 = 1u64;
                    nat_add(a1190, a1191)?
                }, List::cons(f388(f46(v0, v3, {
                    let a1192 = v2;
                    let a1193 = v3;
                    nat_sub(a1192, a1193)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1194 = v2;
                    let a1195 = 1u64;
                    nat_add(a1194, a1195)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f294(v0: Adt2) -> List<Adt2> {
    let m1196 = v0;
    match m1196 {
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
        Adt2::C7(r1197) => {
            let v3 = (*r1197).clone();
            f294(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r1198, r1199) => {
            let v1 = (*r1198).clone();
            let v2 = (*r1199).clone();
            List::cons(v1, f294(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f295(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1200 = v0;
    match m1200 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r1201) => {
            let v12 = (*r1201).clone();
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
        Adt2::C7(r1202) => {
            let v10 = (*r1202).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r1203) => {
            let v5 = (*r1203).clone();
            Adt2::C8(std::rc::Rc::new(f295(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1204, r1205) => {
            let v18 = (*r1204).clone();
            let v19 = (*r1205).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r1206, r1207) => {
            let v3 = (*r1206).clone();
            let v4 = (*r1207).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt2::C11(v2, std::rc::Rc::new(f389(v3, v1.clone())), std::rc::Rc::new(f295(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f295(v4, v1)))
            }
        }
    }
}

pub fn f296(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1208 = 0u64;
        let a1209 = v1;
        nat_lt(a1208, a1209)
    } else {
        false
    } {
        let a1210 = Bytes::lit(&[0x2d]);
        let a1211 = f390(v1)?;
        Ok(append_bytes(a1210, a1211))
    } else {
        f390(v1)
    }
}

pub fn f297(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f390(v1)?;
    let v4: u64 = {
        let a1212 = v3.clone();
        length_bytes(a1212)
    };
    let v5: Bytes = {
        let a1213 = v2;
        let a1214 = v4;
        if nat_lt(a1213, a1214) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1215 = v4;
                let a1216 = v2;
                nat_sub(a1215, a1216)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1217 = v4;
                let a1218 = v2;
                nat_sub(a1217, a1218)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f318({
                let a1219 = v2;
                let a1220 = v4;
                nat_sub(a1219, a1220)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1221 = f8(Str::lit("-"));
        let a1222 = v5;
        Ok(append_bytes(a1221, a1222))
    } else {
        Ok(v5)
    }
}

pub fn f298(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f391(v0)?;
    let v3: bool = {
        let a1223 = f392(v0)?;
        let a1224 = v1;
        nat_le(a1223, a1224)
    };
    let v4: u64 = {
        let a1228 = {
            let a1225 = v1;
            let a1226 = v2;
            let a1227 = 0u64;
            nat_quot(a1225, a1226, a1227)
        };
        let a1229 = f393(f394(v0))?;
        let a1230 = 0u64;
        nat_rem(a1228, a1229, a1230)
    };
    let v5: u64 = {
        let a1231 = v1;
        let a1232 = v2;
        let a1233 = 0u64;
        nat_rem(a1231, a1232, a1233)
    };
    let a1234 = v4;
    let a1235 = f395(v0)?;
    if nat_eq(a1234, a1235) {
        let a1236 = v5;
        let a1237 = 0u64;
        if nat_eq(a1236, a1237) {
            Ok(f396(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f396(v3, {
                let a1241 = {
                    let a1238 = v2;
                    let a1239 = 2u64;
                    let a1240 = 0u64;
                    nat_quot(a1238, a1239, a1240)
                };
                let a1242 = v5;
                if nat_le(a1241, a1242) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f117({
            let a1243 = v4;
            let a1244 = 0u64;
            nat_eq(a1243, a1244)
        }, {
            let a1245 = v5;
            let a1246 = 0u64;
            nat_eq(a1245, a1246)
        }) {
            Ok(f396(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1247 = v4;
                let a1248 = 0u64;
                if nat_eq(a1247, a1248) {
                    v5
                } else {
                    let a1249 = v2;
                    let a1250 = v5;
                    nat_add(a1249, a1250)?
                }
            };
            let v7: u64 = {
                let a1251 = v4;
                let a1252 = 0u64;
                if nat_eq(a1251, a1252) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1259 = {
                    let a1255 = {
                        let a1253 = v7;
                        let a1254 = 2000u64;
                        nat_add(a1253, a1254)?
                    };
                    let a1256 = f397(v0);
                    nat_sub(a1255, a1256)
                };
                let a1260 = {
                    let a1257 = f398(v0);
                    let a1258 = 1u64;
                    nat_sub(a1257, a1258)
                };
                nat_sub(a1259, a1260)
            };
            Ok(f396(v3, {
                let a1261 = 2000u64;
                let a1262 = v8;
                if nat_le(a1261, a1262) {
                    f399(v6, true, {
                        let a1263 = v8;
                        let a1264 = 2000u64;
                        nat_sub(a1263, a1264)
                    })?
                } else {
                    f399(v6, false, {
                        let a1265 = 2000u64;
                        let a1266 = v8;
                        nat_sub(a1265, a1266)
                    })?
                }
            }))
        }
    }
}

pub fn f299(v0: Bytes) -> R<Bytes> {
    let a1271 = f196(v0.clone(), 0u64, {
        let a1268 = {
            let a1267 = v0.clone();
            length_bytes(a1267)
        };
        let a1269 = 1u64;
        nat_add(a1268, a1269)?
    })?;
    let a1272 = {
        let a1270 = v0.clone();
        length_bytes(a1270)
    };
    if nat_eq(a1271, a1272) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1278 = {
            let a1276 = Bytes::lit(&[0x22]);
            let a1277 = f400(v0.clone(), 0u64, {
                let a1274 = {
                    let a1273 = v0;
                    length_bytes(a1273)
                };
                let a1275 = 1u64;
                nat_add(a1274, a1275)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1276, a1277)
        };
        let a1279 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1278, a1279))
    }
}

pub fn f300(v0: List<Bytes>) -> Bytes {
    let a1282 = {
        let a1280 = Bytes::lit(&[0x5b]);
        let a1281 = f122(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1280, a1281)
    };
    let a1283 = Bytes::lit(&[0x5d]);
    append_bytes(a1282, a1283)
}

pub fn f301(v0: Adt2) -> R<List<Bytes>> {
    let m1284 = v0;
    match m1284 {
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
        Adt2::C10(r1285, r1286) => {
            let v14 = (*r1285).clone();
            let v15 = (*r1286).clone();
            Ok(List::cons(f186(v14)?, f301(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f302(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f401(Fn5::F402, List::<(Bytes, Bytes)>::nil(), f403(v0)?);
    let v2: List<Bytes> = f404(Fn6::F405, List::<Bytes>::nil(), v1)?;
    let a1289 = {
        let a1287 = Bytes::lit(&[0x7b]);
        let a1288 = f122(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1287, a1288)
    };
    let a1290 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1289, a1290))
}

pub fn f303(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1291 = v0;
    match m1291 {
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
        Adt2::C11(v16, r1292, r1293) => {
            let v17 = (*r1292).clone();
            let v18 = (*r1293).clone();
            Ok(List::cons((v16, f186(v17)?), f303(v18)?))
        }
    }
}

pub fn f304() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f305(v0: Adt13) -> Bytes {
    let m1294 = v0;
    match m1294 {
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

pub fn f306() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f307(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f406(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f406(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f406(f8(Str::lit("name")), f407(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f406(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f406(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f406(f8(Str::lit("statement")), f408(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f406(f8(Str::lit("media_type")), f409(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f406(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f406(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f406(f8(Str::lit("url")), f189(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f308(v0: Bytes) -> R<u64> {
    let (h1295, _) = f311(Fn3::F410(v0.clone()), {
        let a1296 = v0.clone();
        length_bytes(a1296)
    }, {
        let a1297 = v0;
        length_bytes(a1297)
    })?;
    Ok(h1295)
}

pub fn f309(v0: Bytes, v1: u64) -> R<u64> {
    let (h1298, _) = f311(Fn3::F411(v1, v0), v1, 0u64)?;
    Ok(h1298)
}

pub fn f310(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f115(v0.clone(), {
        let a1299 = v1;
        let a1300 = 1u64;
        nat_add(a1299, a1300)?
    })?;
    let v4: Bytes = {
        let a1301 = v3;
        let a1302 = 117u64;
        if nat_eq(a1301, a1302) {
            f412(v0, v1)?
        } else {
            f413(v3)
        }
    };
    let a1303 = v4.clone();
    let a1304 = Bytes::lit(&[]);
    if equal(a1303, a1304) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1305 = v3;
            let a1306 = 117u64;
            if nat_eq(a1305, a1306) {
                let a1307 = v1;
                let a1308 = 6u64;
                nat_add(a1307, a1308)?
            } else {
                let a1309 = v1;
                let a1310 = 2u64;
                nat_add(a1309, a1310)?
            }
        }, ({
            let a1311 = v2;
            let a1312 = v4;
            append_bytes(a1311, a1312)
        }, 0u64))))
    }
}

pub fn f311(v0: Fn3, p1317: u64, p1318: u64) -> R<(u64, bool)> {
    let m1316 = (p1317, p1318);
    {
        let mut m1316 = m1316;
        loop {
            let step = {
                let (v1, v2) = m1316;
                let m1313 = v1;
                if m1313 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1313 - 1;
                    let m1315 = {
                        let c1314: Fn3 = v0.clone();
                        c1314.apply(v2)?
                    };
                    match m1315 {
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
                Ok(next) => { m1316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f414(v0, v1)
}

pub fn f313(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f415(v0, v1)
}

pub fn f314(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1319 = v0.clone();
        length_bytes(a1319)
    };
    let a1320 = 0u64;
    let a1321 = v1;
    if nat_lt(a1320, a1321) {
        let a1322 = f416(v0.clone())?;
        let a1323 = v1;
        if nat_eq(a1322, a1323) {
            let a1324 = 1u64;
            let a1325 = v1;
            if nat_lt(a1324, a1325) {
                let m1328 = {
                    let a1326 = v0;
                    let a1327 = 0u64;
                    index_bytes(a1326, a1327)
                };
                match m1328 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1329 = v2;
                        let a1330 = 48u8;
                        Ok(!equal(a1329, a1330))
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

pub fn f315(v0: Fn4, p1335: u64, p1336: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1334 = (p1335, p1336);
    {
        let mut m1334 = m1334;
        loop {
            let step = {
                let (v1, v2) = m1334;
                let m1331 = v1;
                if m1331 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1331 - 1;
                    let m1333 = {
                        let c1332: Fn4 = v0.clone();
                        c1332.apply(v2)?
                    };
                    match m1333 {
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
                Ok(next) => { m1334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f417(v0, v1)
}

pub fn f317(v0: Bytes) -> R<u64> {
    let (h1337, _) = f311(Fn3::F418(v0.clone()), {
        let a1339 = {
            let a1338 = v0;
            length_bytes(a1338)
        };
        let a1340 = 1u64;
        nat_add(a1339, a1340)?
    }, 0u64)?;
    Ok(h1337)
}

pub fn f318(v0: u64) -> Bytes {
    let m1341 = v0;
    if m1341 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1341 - 1;
        let a1342 = f8(Str::lit("0"));
        let a1343 = f318(v1);
        append_bytes(a1342, a1343)
    }
}

pub fn f319(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1344 = v0;
    match m1344.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1345 = f93(v1, f8(Str::lit("std")));
            match m1345 {
                None => {
                    f319(v2)
                }
                Some(v3) => {
                    if f217(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f319(v2))
                    } else {
                        f319(v2)
                    }
                }
            }
        }
    }
}

pub fn f320(v0: Adt14, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1346 = v1;
    match m1346.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt14::C0(h1347, _, _) => {
                    h1347
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h1348, _) => {
                        h1348
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f320(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt14::C0(h1349, _, _) => {
                        h1349
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1350, _) => {
                            h1350
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f320(v0, v3))
                } else {
                    f320(v0, v3)
                }
            }
        }
    }
}

pub fn f321(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt17>> {
    let m1351 = v0;
    match m1351.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v4, v5)) => {
            if if f419(v4.clone(), v2.clone()) {
                true
            } else {
                f419(v4.clone(), v3.clone())
            } {
                f321(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt17::C0({
                    let (h1352, _) = v4.clone();
                    h1352
                }, {
                    let a1355 = {
                        let a1354 = f145(f420({
                            let (h1353, _) = v4.clone();
                            h1353
                        }, v1.clone()))?;
                        length_list(a1354)
                    };
                    let a1356 = 1u64;
                    if nat_eq(a1355, a1356) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1357) = v4.clone();
                    h1357
                }), f321(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f322(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1358 = v1;
    match m1358.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1359, _) = v2.clone();
                h1359
            }, v0.clone()) {
                List::cons({
                    let (_, h1360) = v2;
                    h1360
                }, f322(v0, v3))
            } else {
                if f52({
                    let (_, h1361) = v2;
                    h1361
                }, v0.clone()) {
                    List::cons({
                        let (h1362, _) = v2;
                        h1362
                    }, f322(v0, v3))
                } else {
                    f322(v0, v3)
                }
            }
        }
    }
}

pub fn f323(v0: Adt14, v1: Adt2, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt14::C0(h1363, _, _) => {
            h1363
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1364, _) => {
                h1364
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt14::C0(_, _, h1365) => {
                    h1365
                }
            }) {
                true
            } else {
                if f217(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1366, _) => {
                            h1366
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f324(v0: Adt2) -> bool {
    if f217(v0.clone(), f8(Str::lit("domain"))) {
        if f217(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f421()) {
                if f422(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f422(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f422(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f422(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f325(v0: Adt2) -> bool {
    if f217(v0.clone(), f8(Str::lit("subject"))) {
        if f217(v0.clone(), f8(Str::lit("object"))) {
            if f217(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f423())
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

pub fn f326(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1367 = f43(v1)?;
    match m1367 {
        None => {
            Ok(f225(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f211(f184(f93(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1368 = f92(f93(v4.clone(), f8(Str::lit("construct"))));
                match m1368 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1369 = v5;
            match m1369.uncons() {
                None => {
                    Ok(f225(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1370 = v9;
                    match m1370.uncons() {
                        None => {
                            Ok(f225(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f51(f32(v8, f8(Str::lit("std")))) {
                                Ok(f225(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f51(f52(v10.clone(), v2)) {
                                    Ok(f225(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f117(f51(f32(v7.clone(), f8(Str::lit("none")))), f51(f52(v7.clone(), v3))) {
                                        Ok(f225(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1371 = f92(f93(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1371 {
                                            None => {
                                                Ok(f225(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1372 = f93(v4.clone(), f8(Str::lit("reader")));
                                                match m1372 {
                                                    None => {
                                                        let m1373 = f93(v4, f8(Str::lit("value")));
                                                        match m1373 {
                                                            None => {
                                                                Ok(f225(v0, f8(Str::lit("a row names neither the reader of its artifact nor a value"))))
                                                            }
                                                            Some(_) => {
                                                                Ok(List::<Bytes>::nil())
                                                            }
                                                        }
                                                    }
                                                    Some(_) => {
                                                        if f217(v4, f8(Str::lit("artifact"))) {
                                                            Ok(List::<Bytes>::nil())
                                                        } else {
                                                            Ok(f225(v0, f8(Str::lit("a row names its reader but not its artifact"))))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f327(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1374 = v1;
    match m1374.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f424(v3.clone())
            } else {
                f51(f424(v3.clone()))
            } {
                List::cons(f238(v0.clone(), v3), f327(v0, v4, v2))
            } else {
                f327(v0, v4, v2)
            }
        }
    }
}

pub fn f328(p1377: List<Bytes>) -> R<Option<Bytes>> {
    let m1376 = p1377;
    {
        let mut m1376 = m1376;
        loop {
            let step = {
                let v0 = m1376;
                let m1375 = v0;
                match m1375.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f425(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f194(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f329(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1378 = v1;
    match m1378.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f95(f64(v0.clone(), v2))?, f329(v0, v3)?))
        }
    }
}

pub fn f330(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1379 = v0.clone();
        length_bytes(a1379)
    };
    let v3: u64 = {
        let a1380 = v1.clone();
        length_bytes(a1380)
    };
    let v4: u64 = {
        let (h1381, _) = f311(Fn3::F426(v1.clone(), v3, v0.clone(), v2), {
            let a1382 = v2;
            let a1383 = 1u64;
            nat_add(a1382, a1383)?
        }, 0u64)?;
        h1381
    };
    if f427(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f331() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f332(v0: Adt15) -> R<Adt18> {
    Ok(Adt18::C0(f428(match v0.clone() {
        Adt15::C0(h1384, _, _, _, _) => {
            h1384
        }
    }, 0u64)?, f428(match v0.clone() {
        Adt15::C0(_, h1385, _, _, _) => {
            h1385
        }
    }, 0u64)?, f428(match v0.clone() {
        Adt15::C0(_, _, h1386, _, _) => {
            h1386
        }
    }, 0u64)?, f428(match v0 {
        Adt15::C0(_, _, _, h1387, _) => {
            h1387
        }
    }, 0u64)?))
}

pub fn f333(v0: Adt15) -> List<Bytes> {
    f125(match v0.clone() {
        Adt15::C0(h1388, _, _, _, _) => {
            h1388
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, h1389, _, _) => {
            h1389
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, _, h1390, _) => {
            h1390
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1391) => {
            h1391
        }
    })))
}

pub fn f334(v0: Adt18, p1397: List<u64>, p1398: List<Adt20>) -> Option<u64> {
    let m1396 = (p1397, p1398);
    {
        let mut m1396 = m1396;
        loop {
            let step = {
                let (v1, v2) = m1396;
                let m1392 = v2;
                match m1392.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f429(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1393) => {
                                h1393
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1394, _) => {
                                    h1394
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1395, _) => {
                                    h1395
                                }
                            }))
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

pub fn f335(v0: Adt15, v1: List<Adt2>, v2: u64) -> R<List<Adt20>> {
    let m1399 = v1;
    match m1399.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f430(v0.clone(), v3)?), f335(v0, v4, {
                let a1400 = v2;
                let a1401 = 1u64;
                nat_add(a1400, a1401)?
            })?))
        }
    }
}

pub fn f336(p1405: List<Bytes>, p1406: u64) -> Bytes {
    let m1404 = (p1405, p1406);
    {
        let mut m1404 = m1404;
        loop {
            let step = {
                let (v0, v1) = m1404;
                let m1402 = v0;
                match m1402.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1403 = v1;
                        if m1403 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1403 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
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

pub fn f337(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f431(v0.clone(), v1.clone()) {
        if f431(v1, v0) {
            f432(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f338(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1407 = v0;
    match m1407.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f433(v2, v1.clone())?, f338(v3, v1)?))
        }
    }
}

pub fn f339(v0: List<Adt2>) -> List<u64> {
    let m1408 = v0;
    match m1408.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f434(v1), f339(v2))
        }
    }
}

pub fn f340(p1411: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1410 = p1411;
    {
        let mut m1410 = m1410;
        loop {
            let step = {
                let v0 = m1410;
                let m1409 = v0;
                match m1409.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f435(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1410 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(p1414: List<Bytes>, p1415: List<Bytes>) -> Option<Bytes> {
    let m1413 = (p1414, p1415);
    {
        let mut m1413 = m1413;
        loop {
            let step = {
                let (v0, v1) = m1413;
                let m1412 = v0;
                match m1412.uncons() {
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
                Ok(next) => { m1413 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f342(v0: Adt15, p1423: List<Adt2>) -> Option<Bytes> {
    let m1422 = p1423;
    {
        let mut m1422 = m1422;
        loop {
            let step = {
                let v1 = m1422;
                let m1416 = v1;
                match m1416.uncons() {
                    None => {
                        Err::<List<Adt2>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1417 = f93(v2.clone(), f8(Str::lit("evidence")));
                        match m1417 {
                            None => {
                                Ok::<List<Adt2>, Option<Bytes>>(v3)
                            }
                            Some(v4) => {
                                let m1418 = f93(v4, f8(Str::lit("constraint")));
                                match m1418 {
                                    None => {
                                        Ok::<List<Adt2>, Option<Bytes>>(v3)
                                    }
                                    Some(v5) => {
                                        if f52(f58(v5.clone(), f8(Str::lit("authority"))), match v0.clone() {
                                            Adt15::C0(h1419, _, _, _, _) => {
                                                h1419
                                            }
                                        }) {
                                            let m1420 = f92(f93(v5, f8(Str::lit("row"))));
                                            match m1420 {
                                                None => {
                                                    Ok::<List<Adt2>, Option<Bytes>>(v3)
                                                }
                                                Some(v6) => {
                                                    if f52(v6, match v0.clone() {
                                                        Adt15::C0(_, h1421, _, _, _) => {
                                                            h1421
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
                Ok(next) => { m1422 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f344(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1424 = v0.clone();
    match m1424.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1425 = v2;
            let a1426 = v1.clone();
            if equal(a1425, a1426) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f345(v0: List<(Bytes, ())>) -> List<Bytes> {
    f235(f436(Fn7::F437, List::<Bytes>::nil(), v0))
}

pub fn f346(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f438(f439(Fn8::F440, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1431 = {
        let (h1427, _) = f441(Fn9::F442, {
            let a1429 = {
                let a1428 = v0;
                length_list(a1428)
            };
            let a1430 = 1u64;
            nat_add(a1429, a1430)?
        }, v1)?;
        h1427
    };
    match m1431.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f347(v0: List<Bytes>) -> List<(Bytes, ())> {
    f443(f444(Fn10::F445, List::<(Bytes, ())>::nil(), v0))
}

pub fn f348(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1432 = f44(v1, 34u8)?;
    match m1432.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1433 = v3;
            match m1433.uncons() {
                None => {
                    Ok(f446(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f349(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f447(v0.clone(), 0u64, {
        let a1434 = v0;
        length_bytes(a1434)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f350(v0: Adt2, v1: Bytes) -> Bytes {
    if f32(f448(f93(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f351(v0: Bytes, v1: Bytes, p1447: u64, p1448: u64) -> R<bool> {
    let m1446 = (p1447, p1448);
    {
        let mut m1446 = m1446;
        loop {
            let step = {
                let (v2, v3) = m1446;
                let m1435 = v3;
                if m1435 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1435 - 1;
                    let m1438 = {
                        let a1436 = v1.clone();
                        let a1437 = v2;
                        index_bytes(a1436, a1437)
                    };
                    match m1438 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1441 = {
                                let a1439 = v0.clone();
                                let a1440 = v2;
                                index_bytes(a1439, a1440)
                            };
                            match m1441 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1442 = v5;
                                    let a1443 = v6;
                                    if equal(a1442, a1443) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1444 = v2;
                                            let a1445 = 1u64;
                                            nat_add(a1444, a1445)?
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
                Ok(next) => { m1446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1449 = f91(v0, v1)?;
    match m1449 {
        None => {
            Ok(Adt2::C9)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f353(v0: Bytes) -> R<Bytes> {
    let a1450 = f8(Str::lit("Catalog."));
    let a1451 = f377(v0)?;
    Ok(append_bytes(a1450, a1451))
}

pub fn f354(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f377(f58(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f355(v0: List<Bytes>) -> List<Bytes> {
    let m1452 = v0;
    match m1452.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f238(f267(), v1), f355(v2))
            } else {
                f355(v2)
            }
        }
    }
}

pub fn f356(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1453 = v1;
    match m1453.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f449(f450(f95(f64(v0.clone(), v2))?)?), f356(v0, v3)?))
        }
    }
}

pub fn f357(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1454 = v1;
    match m1454.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f451(v0.clone(), v3, v2.clone())?;
            let (h1455, _) = v5.clone();
            if h1455 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f357(v0, v4, v2)?;
                let (h1456, _) = v6.clone();
                if h1456 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1459 = {
                            let (_, h1457) = v5;
                            h1457
                        };
                        let a1460 = {
                            let (_, h1458) = v6;
                            h1458
                        };
                        append_bytes(a1459, a1460)
                    }))
                }
            }
        }
    }
}

pub fn f358(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1461 = v1;
    match m1461.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f217(v2.clone(), f8(Str::lit("subject"))) {
                if f218(v0.clone(), v2.clone()) {
                    f358(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f358(v0, v3))
                }
            } else {
                if f368(v0.clone(), v2.clone()) {
                    f358(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f369(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f358(v0, v3))
                }
            }
        }
    }
}

pub fn f359(v0: Adt1) -> R<Bytes> {
    let a1463 = {
        let a1462 = f128(f129(f260(f257(f258(f64(v0.clone(), f209()))?)), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f131(f96(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1462)
    };
    let a1464 = 0u64;
    if nat_eq(a1463, a1464) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f360(v0: List<Adt2>) -> R<List<Bytes>> {
    let m1465 = v0;
    match m1465.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f122(f211(f184(f93(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f452(f95(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f360(v2)?))
        }
    }
}

pub fn f361() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f362(v0: Bytes, p1523: u64, p1524: u64, p1525: Adt19) -> R<Adt19> {
    let m1522 = (p1523, (p1524, p1525));
    {
        let mut m1522 = m1522;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1522;
                let m1466 = v2;
                if m1466 == 0 {
                    Err::<(u64, (u64, Adt19)), R<Adt19>>(Ok(v3))
                } else {
                    let v4 = m1466 - 1;
                    let v5: u64 = f453(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt19::C0(_, h1467, _, _, _) => {
                            h1467
                        }
                    } {
                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                            let a1468 = v1;
                            let a1469 = 1u64;
                            nat_add(a1468, a1469)?
                        }, (v4, Adt19::C0(true, false, match v3.clone() {
                            Adt19::C0(_, _, h1470, _, _) => {
                                h1470
                            }
                        }, match v3.clone() {
                            Adt19::C0(_, _, _, h1471, _) => {
                                h1471
                            }
                        }, match v3 {
                            Adt19::C0(_, _, _, _, h1472) => {
                                h1472
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt19::C0(h1473, _, _, _, _) => {
                                h1473
                            }
                        } {
                            let a1474 = v5;
                            let a1475 = 34u64;
                            if nat_eq(a1474, a1475) {
                                let a1478 = f453(v0.clone(), {
                                    let a1476 = v1;
                                    let a1477 = 1u64;
                                    nat_add(a1476, a1477)?
                                })?;
                                let a1479 = 34u64;
                                if nat_eq(a1478, a1479) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1480 = v1;
                                        let a1481 = 1u64;
                                        nat_add(a1480, a1481)?
                                    }, (v4, Adt19::C0(true, true, f454(match v3.clone() {
                                        Adt19::C0(_, _, h1482, _, _) => {
                                            h1482
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt19::C0(_, _, _, h1483, _) => {
                                            h1483
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1484) => {
                                            h1484
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1485 = v1;
                                        let a1486 = 1u64;
                                        nat_add(a1485, a1486)?
                                    }, (v4, Adt19::C0(false, false, match v3.clone() {
                                        Adt19::C0(_, _, h1487, _, _) => {
                                            h1487
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1488, _) => {
                                            h1488
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1489) => {
                                            h1489
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1490 = v1;
                                    let a1491 = 1u64;
                                    nat_add(a1490, a1491)?
                                }, (v4, Adt19::C0(true, false, f454(match v3.clone() {
                                    Adt19::C0(_, _, h1492, _, _) => {
                                        h1492
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt19::C0(_, _, _, h1493, _) => {
                                        h1493
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1494) => {
                                        h1494
                                    }
                                }))))
                            }
                        } else {
                            let a1495 = v5;
                            let a1496 = 34u64;
                            if nat_eq(a1495, a1496) {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1497 = v1;
                                    let a1498 = 1u64;
                                    nat_add(a1497, a1498)?
                                }, (v4, Adt19::C0(true, false, match v3.clone() {
                                    Adt19::C0(_, _, h1499, _, _) => {
                                        h1499
                                    }
                                }, match v3.clone() {
                                    Adt19::C0(_, _, _, h1500, _) => {
                                        h1500
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1501) => {
                                        h1501
                                    }
                                }))))
                            } else {
                                let a1502 = v5;
                                let a1503 = 44u64;
                                if nat_eq(a1502, a1503) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1504 = v1;
                                        let a1505 = 1u64;
                                        nat_add(a1504, a1505)?
                                    }, (v4, Adt19::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt19::C0(_, _, h1506, _, _) => {
                                            h1506
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1507, _) => {
                                            h1507
                                        }
                                    }), match v3 {
                                        Adt19::C0(_, _, _, _, h1508) => {
                                            h1508
                                        }
                                    }))))
                                } else {
                                    let a1509 = v5;
                                    let a1510 = 13u64;
                                    if nat_eq(a1509, a1510) {
                                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                            let a1511 = v1;
                                            let a1512 = 1u64;
                                            nat_add(a1511, a1512)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1513 = v5;
                                        let a1514 = 10u64;
                                        if nat_eq(a1513, a1514) {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1515 = v1;
                                                let a1516 = 1u64;
                                                nat_add(a1515, a1516)?
                                            }, (v4, f363(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1517 = v1;
                                                let a1518 = 1u64;
                                                nat_add(a1517, a1518)?
                                            }, (v4, Adt19::C0(false, false, f454(match v3.clone() {
                                                Adt19::C0(_, _, h1519, _, _) => {
                                                    h1519
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt19::C0(_, _, _, h1520, _) => {
                                                    h1520
                                                }
                                            }, match v3 {
                                                Adt19::C0(_, _, _, _, h1521) => {
                                                    h1521
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
                Ok(next) => { m1522 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363(v0: Adt19) -> Adt19 {
    Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f455(List::cons(match v0.clone() {
        Adt19::C0(_, _, h1526, _, _) => {
            h1526
        }
    }, match v0.clone() {
        Adt19::C0(_, _, _, h1527, _) => {
            h1527
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt19::C0(_, _, _, _, h1528) => {
            h1528
        }
    }))
}

pub fn f364(p1531: List<List<Bytes>>, p1532: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1530 = (p1531, p1532);
    {
        let mut m1530 = m1530;
        loop {
            let step = {
                let (v0, v1) = m1530;
                let m1529 = v0;
                match m1529.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f365(v0: Bytes, v1: List<Adt17>) -> List<Bytes> {
    let m1533 = v1;
    match m1533.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt17::C0(h1534, _, _) => {
                    h1534
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt17::C0(_, h1535, _) => {
                        h1535
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt17::C0(_, _, h1536) => {
                        h1536
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f365(v0, v3))
            } else {
                f365(v0, v3)
            }
        }
    }
}

pub fn f366(v0: Bytes, v1: List<Adt2>) -> List<Bytes> {
    let m1537 = v1;
    match m1537.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f366(v0, v3))
            } else {
                if f32(f58(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f58(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f366(v0, v3))
                } else {
                    f366(v0, v3)
                }
            }
        }
    }
}

pub fn f367(p1542: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1541 = p1542;
    {
        let mut m1541 = m1541;
        loop {
            let step = {
                let v0 = m1541;
                let m1538 = v0;
                match m1538.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1539 = v2;
                        match m1539.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1540 = v5;
                                    match m1540.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f456(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1541 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(v0: Adt14, v1: Adt2) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1543, _, _) => {
            h1543
        }
    }) {
        f52(f369(v1), match v0 {
            Adt14::C0(_, h1544, _) => {
                h1544
            }
        })
    } else {
        false
    }
}

pub fn f369(v0: Adt2) -> Bytes {
    let m1545 = f93(v0, f8(Str::lit("evidence")));
    match m1545 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f370(v0: Adt2) -> Bytes {
    if f217(v0.clone(), f8(Str::lit("req"))) {
        let a1546 = f8(Str::lit("req "));
        let a1547 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1546, a1547)
    } else {
        let m1548 = f93(v0.clone(), f8(Str::lit("std")));
        match m1548 {
            None => {
                let m1551 = f93(v0, f8(Str::lit("der")));
                match m1551 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f122(f211(f184(f93(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f217(v1.clone(), f8(Str::lit("row"))) {
                    let a1549 = f8(Str::lit(" row "));
                    let a1550 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1549, a1550)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f371(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt2>>> {
    let m1552 = v0;
    match m1552.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f194(v2)?;
            let a1554 = {
                let a1553 = v4.clone();
                length_bytes(a1553)
            };
            let a1555 = 0u64;
            if nat_eq(a1554, a1555) {
                f371(v3, v1)
            } else {
                let m1556 = f43(v4)?;
                match m1556 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f457(v5.clone());
                        let a1558 = {
                            let a1557 = v6.clone();
                            length_bytes(a1557)
                        };
                        let a1559 = 0u64;
                        if nat_eq(a1558, a1559) {
                            Ok(None::<List<Adt2>>)
                        } else {
                            let m1560 = f371(v3, v1.clone())?;
                            match m1560 {
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

pub fn f372(v0: Bytes, v1: Bytes) -> bool {
    let a1561 = v0;
    let a1562 = f378();
    if equal(a1561, a1562) {
        let a1566 = {
            let a1564 = {
                let a1563 = v1;
                length_bytes(a1563)
            };
            let a1565 = 0u64;
            nat_eq(a1564, a1565)
        };
        bool_not(a1566)
    } else {
        false
    }
}

pub fn f373(v0: Bytes) -> Bytes {
    f7(List::cons(f144(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f374(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1568 = v0;
    let a1569 = {
        let a1567 = Str::lit("redistributable");
        utf8_encode(a1567)
    };
    if equal(a1568, a1569) {
        f458(v1)
    } else {
        Ok(false)
    }
}

pub fn f375(v0: Bytes, v1: Bytes) -> bool {
    f199(f85(v0.clone(), f8(Str::lit("read-"))), f117(f32(v0, f8(Str::lit("check-yaml"))), f32(v1, f8(Str::lit("none")))))
}

pub fn f376(v0: Adt2) -> bool {
    if f459(v0.clone(), f8(Str::lit("name"))) {
        if f460(v0.clone(), f8(Str::lit("schema"))) {
            let m1570 = f461(f211(f184(f93(v0, f8(Str::lit("citation"))))));
            match m1570 {
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

pub fn f377(v0: Bytes) -> R<Bytes> {
    f462(v0.clone(), 0u64, {
        let a1571 = v0;
        length_bytes(a1571)
    }, true, Bytes::lit(&[]))
}

pub fn f378() -> Bytes {
    let a1572 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1572)
}

pub fn f379(v0: Adt2, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt2> = f93(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f283(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f283(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f283(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f283(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f283(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f283(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f283(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f283(Str::lit("url"), f448(v2.clone(), f8(Str::lit("url"))))?, List::cons(f283(Str::lit("media_type"), f448(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f283(Str::lit("license"), f448(v2.clone(), f8(Str::lit("license"))))?, List::cons(f283(Str::lit("redistribution"), f448(v2, f8(Str::lit("redistribution"))))?, List::cons(f283(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1573 = v1;
        match m1573 {
            None => {
                f7(List::cons(f283(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f283(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f283(Str::lit("artifact"), {
                    let (h1574, _) = v3;
                    h1574
                })?, List::cons(f283(Str::lit("checksum"), {
                    let (_, h1575) = v3;
                    h1575
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f463(f211(f184(f93(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f380(v0: Adt23) -> Bytes {
    let a1596 = f464(match v0.clone() {
        Adt23::C0(h1576, _, _, _, _, _, _, _) => {
            h1576
        }
    });
    let a1597 = {
        let a1594 = f464(match v0.clone() {
            Adt23::C0(_, h1577, _, _, _, _, _, _) => {
                h1577
            }
        });
        let a1595 = {
            let a1592 = f464(match v0.clone() {
                Adt23::C0(_, _, h1578, _, _, _, _, _) => {
                    h1578
                }
            });
            let a1593 = {
                let a1590 = f464(match v0.clone() {
                    Adt23::C0(_, _, _, h1579, _, _, _, _) => {
                        h1579
                    }
                });
                let a1591 = {
                    let a1588 = f464(match v0.clone() {
                        Adt23::C0(_, _, _, _, h1580, _, _, _) => {
                            h1580
                        }
                    });
                    let a1589 = {
                        let a1586 = f464(match v0.clone() {
                            Adt23::C0(_, _, _, _, _, h1581, _, _) => {
                                h1581
                            }
                        });
                        let a1587 = {
                            let a1584 = f464(match v0.clone() {
                                Adt23::C0(_, _, _, _, _, _, h1582, _) => {
                                    h1582
                                }
                            });
                            let a1585 = f464(match v0 {
                                Adt23::C0(_, _, _, _, _, _, _, h1583) => {
                                    h1583
                                }
                            });
                            append_bytes(a1584, a1585)
                        };
                        append_bytes(a1586, a1587)
                    };
                    append_bytes(a1588, a1589)
                };
                append_bytes(a1590, a1591)
            };
            append_bytes(a1592, a1593)
        };
        append_bytes(a1594, a1595)
    };
    append_bytes(a1596, a1597)
}

pub fn f381(v0: Bytes) -> R<Adt23> {
    let v1: u64 = {
        let a1598 = v0.clone();
        length_bytes(a1598)
    };
    let v2: u64 = {
        let a1604 = {
            let a1601 = {
                let a1599 = v1;
                let a1600 = 72u64;
                nat_add(a1599, a1600)?
            };
            let a1602 = 64u64;
            let a1603 = 0u64;
            nat_quot(a1601, a1602, a1603)
        };
        let a1605 = 64u64;
        nat_mul(a1604, a1605)?
    };
    let (_, h1606) = f465(Fn11::F466(v0, v1, v2), {
        let a1607 = v2;
        let a1608 = 64u64;
        let a1609 = 0u64;
        nat_quot(a1607, a1608, a1609)
    }, (0u64, f467()))?;
    Ok(h1606)
}

pub fn f382(p1612: List<List<Bytes>>, v1: Bytes, p1613: List<Bytes>) -> R<List<Bytes>> {
    let m1611 = (p1612, p1613);
    {
        let mut m1611 = m1611;
        loop {
            let step = {
                let (v0, v2) = m1611;
                let m1610 = v0;
                match m1610.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f468(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f384(f469(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1611 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f308(v0)?))
}

pub fn f384(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1614 = v0;
    match m1614.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1615 = v3.clone();
            match m1615.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1618 = v2;
                    let a1619 = {
                        let a1616 = v1.clone();
                        let a1617 = f384(v3, v1);
                        append_bytes(a1616, a1617)
                    };
                    append_bytes(a1618, a1619)
                }
            }
        }
    }
}

pub fn f385(v0: List<Bytes>) -> Adt2 {
    let m1620 = v0;
    match m1620.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f385(v2)))
        }
    }
}

pub fn f386(v0: Adt2, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1621 = f93(v0, v1);
    match m1621 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v3) => {
            f184(f93(v3, v2))
        }
    }
}

pub fn f387(v0: Adt2, v1: Bytes) -> bool {
    let m1622 = f93(v0, v1);
    match m1622 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f388(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1624 = {
                let a1623 = v0;
                length_bytes(a1623)
            };
            let a1625 = 1u64;
            nat_sub(a1624, a1625)
        })
    } else {
        v0
    }
}

pub fn f389(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1626 = v0;
    match m1626 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v8, r1627) => {
            let v9 = (*r1627).clone();
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
            Adt2::C6(f470(f60(), v2, v1))
        }
        Adt2::C7(r1628) => {
            let v6 = (*r1628).clone();
            Adt2::C7(std::rc::Rc::new(v6))
        }
        Adt2::C8(r1629) => {
            let v7 = (*r1629).clone();
            Adt2::C8(std::rc::Rc::new(v7))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1630, r1631) => {
            let v15 = (*r1630).clone();
            let v16 = (*r1631).clone();
            Adt2::C10(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt2::C11(v17, r1632, r1633) => {
            let v18 = (*r1632).clone();
            let v19 = (*r1633).clone();
            Adt2::C11(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
    }
}

pub fn f390(v0: u64) -> R<Bytes> {
    f471(f472(), 10u64, v0)
}

pub fn f391(v0: bool) -> R<u64> {
    f393({
        let a1634 = f398(v0);
        let a1635 = 1u64;
        nat_sub(a1634, a1635)
    })
}

pub fn f392(v0: bool) -> R<u64> {
    f393(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f393(v0: u64) -> R<u64> {
    let m1636 = v0;
    if m1636 == 0 {
        Ok(1u64)
    } else {
        let v1 = m1636 - 1;
        let a1637 = 2u64;
        let a1638 = f393(v1)?;
        nat_mul(a1637, a1638)
    }
}

pub fn f394(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f395(v0: bool) -> R<u64> {
    let a1639 = f393(f394(v0))?;
    let a1640 = 1u64;
    Ok(nat_sub(a1639, a1640))
}

pub fn f396(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a1641 = Bytes::lit(&[0x2d]);
        let a1642 = v1;
        append_bytes(a1641, a1642)
    } else {
        v1
    }
}

pub fn f397(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f398(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f399(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        f473(f474(v2, f475(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f473(f474(v2, f475(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a1643 = v3.clone();
            length_bytes(a1643)
        };
        let v5: Bytes = {
            let a1644 = v2;
            let a1645 = v4;
            if nat_lt(a1644, a1645) {
                f46(v3.clone(), 0u64, {
                    let a1646 = v4;
                    let a1647 = v2;
                    nat_sub(a1646, a1647)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f476({
            let a1648 = v2;
            let a1649 = v4;
            if nat_lt(a1648, a1649) {
                f46(v3, {
                    let a1650 = v4;
                    let a1651 = v2;
                    nat_sub(a1650, a1651)
                }, v2)
            } else {
                let a1654 = f477({
                    let a1652 = v2;
                    let a1653 = v4;
                    nat_sub(a1652, a1653)
                });
                let a1655 = v3;
                append_bytes(a1654, a1655)
            }
        }, {
            let a1656 = v2;
            let a1657 = 1u64;
            nat_add(a1656, a1657)?
        })?;
        let a1659 = {
            let a1658 = v6.clone();
            length_bytes(a1658)
        };
        let a1660 = 0u64;
        if nat_eq(a1659, a1660) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f400(v0: Bytes, p1679: u64, p1680: u64, p1681: Bytes) -> R<Bytes> {
    let m1678 = (p1679, (p1680, p1681));
    {
        let mut m1678 = m1678;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1678;
                let m1661 = v2;
                if m1661 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1661 - 1;
                    let v5: u64 = f196(v0.clone(), v1, {
                        let a1665 = {
                            let a1663 = {
                                let a1662 = v0.clone();
                                length_bytes(a1662)
                            };
                            let a1664 = v1;
                            nat_sub(a1663, a1664)
                        };
                        let a1666 = 1u64;
                        nat_add(a1665, a1666)?
                    })?;
                    let v6: Bytes = {
                        let a1669 = v3;
                        let a1670 = f46(v0.clone(), v1, {
                            let a1667 = v5;
                            let a1668 = v1;
                            nat_sub(a1667, a1668)
                        });
                        append_bytes(a1669, a1670)
                    };
                    let m1673 = {
                        let a1671 = v0.clone();
                        let a1672 = v5;
                        index_bytes(a1671, a1672)
                    };
                    match m1673 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a1674 = v5;
                                let a1675 = 1u64;
                                nat_add(a1674, a1675)?
                            }, (v4, {
                                let a1676 = v6;
                                let a1677 = f478(v0.clone(), v5, v7)?;
                                append_bytes(a1676, a1677)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f401(v0: Fn5, p1685: List<(Bytes, Bytes)>, p1686: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1684 = (p1685, p1686);
    {
        let mut m1684 = m1684;
        loop {
            let step = {
                let (v1, v2) = m1684;
                let m1682 = v2;
                match m1682.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1683: Fn5 = v0.clone();
                            c1683.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1684 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f479(v0, v1)
}

pub fn f403(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f480(v0)
}

pub fn f404(v0: Fn6, p1690: List<Bytes>, p1691: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1689 = (p1690, p1691);
    {
        let mut m1689 = m1689;
        loop {
            let step = {
                let (v1, v2) = m1689;
                let m1687 = v2;
                match m1687.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1688: Fn6 = v0.clone();
                            c1688.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1689 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f481(v1)?, v0))
}

pub fn f406(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f482(v0)?, List::cons(f8(Str::lit(":")), List::cons(f482(v1)?, List::<Bytes>::nil())))))
}

pub fn f407(v0: Adt13) -> Bytes {
    let m1692 = v0;
    match m1692 {
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

pub fn f408(v0: Adt13) -> Bytes {
    let m1693 = v0;
    match m1693 {
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

pub fn f409(v0: Adt13) -> Bytes {
    let m1694 = v0;
    match m1694 {
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

pub fn f410(v0: Bytes, v1: u64) -> Option<u64> {
    let a1695 = 0u64;
    let a1696 = v1;
    if nat_lt(a1695, a1696) {
        if f483(v0, {
            let a1697 = v1;
            let a1698 = 1u64;
            nat_sub(a1697, a1698)
        }) {
            Some({
                let a1699 = v1;
                let a1700 = 1u64;
                nat_sub(a1699, a1700)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f411(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1701 = v2;
    let a1702 = v0;
    if nat_lt(a1701, a1702) {
        if f483(v1, v2) {
            Ok(Some({
                let a1703 = v2;
                let a1704 = 1u64;
                nat_add(a1703, a1704)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f412(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1707 = f46(v0.clone(), {
        let a1705 = v1;
        let a1706 = 2u64;
        nat_add(a1705, a1706)?
    }, 2u64);
    let a1708 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1707, a1708) {
        let m1711 = f484(f46(v0, {
            let a1709 = v1;
            let a1710 = 4u64;
            nat_add(a1709, a1710)?
        }, 2u64))?;
        match m1711 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f115(v2.clone(), 0u64)?;
                let a1712 = v3;
                let a1713 = 32u64;
                if nat_lt(a1712, a1713) {
                    if f485(v3) {
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

pub fn f413(v0: u64) -> Bytes {
    let a1714 = v0;
    let a1715 = 34u64;
    if nat_eq(a1714, a1715) {
        Bytes::lit(&[0x22])
    } else {
        let a1716 = v0;
        let a1717 = 92u64;
        if nat_eq(a1716, a1717) {
            Bytes::lit(&[0x5c])
        } else {
            let a1718 = v0;
            let a1719 = 98u64;
            if nat_eq(a1718, a1719) {
                Bytes::lit(&[0x08])
            } else {
                let a1720 = v0;
                let a1721 = 102u64;
                if nat_eq(a1720, a1721) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1722 = v0;
                    let a1723 = 110u64;
                    if nat_eq(a1722, a1723) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1724 = v0;
                        let a1725 = 114u64;
                        if nat_eq(a1724, a1725) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1726 = v0;
                            let a1727 = 116u64;
                            if nat_eq(a1726, a1727) {
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

pub fn f414(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f115(v0, v1)?;
    let a1728 = 47u64;
    let a1729 = v2;
    if if nat_lt(a1728, a1729) {
        let a1730 = v2;
        let a1731 = 58u64;
        nat_lt(a1730, a1731)
    } else {
        false
    } {
        Ok(Some({
            let a1732 = v1;
            let a1733 = 1u64;
            nat_add(a1732, a1733)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f415(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1734 = f115(v0, v1)?;
    let a1735 = 48u64;
    if nat_eq(a1734, a1735) {
        Ok(Some({
            let a1736 = v1;
            let a1737 = 1u64;
            nat_add(a1736, a1737)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f416(v0: Bytes) -> R<u64> {
    let (h1738, _) = f311(Fn3::F486(v0.clone()), {
        let a1739 = v0;
        length_bytes(a1739)
    }, 0u64)?;
    Ok(h1738)
}

pub fn f417(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1743 = {
        let a1741 = v0;
        let a1742 = {
            let (h1740, _) = v1;
            h1740
        };
        index_bytes(a1741, a1742)
    };
    match m1743 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1745 = {
                    let (h1744, _) = v1;
                    h1744
                };
                let a1746 = 1u64;
                nat_add(a1745, a1746)?
            }, {
                let a1750 = {
                    let a1748 = {
                        let (_, h1747) = v1;
                        h1747
                    };
                    let a1749 = 10u64;
                    nat_mul(a1748, a1749)?
                };
                let a1751 = f487(v2)?;
                nat_add(a1750, a1751)?
            })))
        }
    }
}

pub fn f418(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f488(v0, v1)
}

pub fn f419(v0: (Bytes, Bytes), p1754: List<(Bytes, Bytes)>) -> bool {
    let m1753 = p1754;
    {
        let mut m1753 = m1753;
        loop {
            let step = {
                let v1 = m1753;
                let m1752 = v1;
                match m1752.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f489(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
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

pub fn f420(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1755 = v1;
    match m1755.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1756, _) = v2;
                h1756
            }, v0.clone()) {
                List::cons({
                    let (_, h1757) = v2;
                    h1757
                }, f420(v0, v3))
            } else {
                f420(v0, v3)
            }
        }
    }
}

pub fn f421() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f422(v0: Adt2, v1: Bytes, v2: Bytes) -> bool {
    let m1758 = f93(v0, v1);
    match m1758 {
        None => {
            false
        }
        Some(v3) => {
            f217(v3, v2)
        }
    }
}

pub fn f423() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f424(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f425(v0: Bytes) -> R<bool> {
    let v1: Bytes = f194(v0)?;
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
        let a1760 = {
            let a1759 = v1;
            length_bytes(a1759)
        };
        let a1761 = v2;
        nat_sub(a1760, a1761)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1762 = f330(v3.clone(), f8(Str::lit(")")))?;
        match m1762 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1763 = v4;
                    let a1764 = 1u64;
                    nat_add(a1763, a1764)?
                }, {
                    let a1768 = {
                        let a1765 = v3;
                        length_bytes(a1765)
                    };
                    let a1769 = {
                        let a1766 = v4;
                        let a1767 = 1u64;
                        nat_add(a1766, a1767)?
                    };
                    nat_sub(a1768, a1769)
                })
            }
        }
    } else {
        v3
    };
    let a1770 = 0u64;
    let a1771 = v2;
    if nat_lt(a1770, a1771) {
        f490(v5)
    } else {
        Ok(false)
    }
}

pub fn f426(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f427(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1774 = {
            let a1772 = v4;
            let a1773 = v1;
            nat_add(a1772, a1773)?
        };
        let a1775 = v3;
        if nat_lt(a1774, a1775) {
            Ok(Some({
                let a1776 = v4;
                let a1777 = 1u64;
                nat_add(a1776, a1777)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f427(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1782 = {
        let a1779 = v0;
        let a1780 = v2;
        let a1781 = {
            let a1778 = v1.clone();
            length_bytes(a1778)
        };
        slice_bytes(a1779, a1780, a1781)
    };
    match m1782 {
        None => {
            false
        }
        Some(v3) => {
            let a1783 = v3;
            let a1784 = v1;
            equal(a1783, a1784)
        }
    }
}

pub fn f428(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1785 = v0;
    match m1785.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f428(v3, {
                let a1786 = v1;
                let a1787 = 1u64;
                nat_add(a1786, a1787)?
            })?))
        }
    }
}

pub fn f429(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1788 = v2;
    match m1788 {
        Adt21::C0(v3, v4) => {
            if f435(v3, match v0.clone() {
                Adt18::C0(h1789, _, _, _) => {
                    h1789
                }
            }) {
                f491(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f435(v5, match v0 {
                Adt18::C0(_, _, h1790, _) => {
                    h1790
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f431(v7, v1) {
                f435(v8, match v0 {
                    Adt18::C0(_, _, _, h1791) => {
                        h1791
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f430(v0: Adt15, v1: Adt2) -> R<Adt21> {
    let m1792 = f93(v1.clone(), f8(Str::lit("std")));
    match m1792 {
        None => {
            let m1797 = f92(f93(v1.clone(), f8(Str::lit("req"))));
            match m1797 {
                None => {
                    let m1799 = f93(v1, f8(Str::lit("der")));
                    match m1799 {
                        None => {
                            Ok(f492(v0))
                        }
                        Some(v5) => {
                            let m1800 = f493(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1800 {
                                None => {
                                    Ok(f492(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f338(f211(f184(f93(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1801) => {
                                            h1801
                                        }
                                    })?, f433(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1802, _) => {
                                            h1802
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f433(v4, match v0 {
                        Adt15::C0(_, _, h1798, _, _) => {
                            h1798
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1793 = f92(f93(v2.clone(), f8(Str::lit("row"))));
            match m1793 {
                None => {
                    Ok(Adt21::C0(f433(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1794, _, _, _, _) => {
                            h1794
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f433(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1795, _, _, _, _) => {
                            h1795
                        }
                    })?, Some(f433(v3, match v0 {
                        Adt15::C0(_, h1796, _, _, _) => {
                            h1796
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f431(p1805: List<u64>, v1: List<u64>) -> bool {
    let m1804 = p1805;
    {
        let mut m1804 = m1804;
        loop {
            let step = {
                let v0 = m1804;
                let m1803 = v0;
                match m1803.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f435(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1804 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(p1810: List<u64>) -> bool {
    let m1809 = p1810;
    {
        let mut m1809 = m1809;
        loop {
            let step = {
                let v0 = m1809;
                let m1806 = v0;
                match m1806.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1807 = v1;
                        let a1808 = 7u64;
                        if nat_le(a1807, a1808) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1811 = f494(v0, v1.clone(), 0u64)?;
    match m1811 {
        None => {
            let a1812 = v1;
            Ok(length_list(a1812))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f434(v0: Adt2) -> u64 {
    let m1813 = f495(f93(v0, f8(Str::lit("stratum"))));
    match m1813 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f435(v0: u64, p1818: List<u64>) -> bool {
    let m1817 = p1818;
    {
        let mut m1817 = m1817;
        loop {
            let step = {
                let v1 = m1817;
                let m1814 = v1;
                match m1814.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1815 = v2;
                        let a1816 = v0;
                        if nat_eq(a1815, a1816) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: Fn7, p1822: List<Bytes>, p1823: List<(Bytes, ())>) -> List<Bytes> {
    let m1821 = (p1822, p1823);
    {
        let mut m1821 = m1821;
        loop {
            let step = {
                let (v1, v2) = m1821;
                let m1819 = v2;
                match m1819.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1820: Fn7 = v0.clone();
                            c1820.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1821 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1824, _) = v1;
        h1824
    }, v0)
}

pub fn f438(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f496(Fn12::F497, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f439(v0: Fn8, p1828: List<List<(Bytes, ())>>, p1829: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1827 = (p1828, p1829);
    {
        let mut m1827 = m1827;
        loop {
            let step = {
                let (v1, v2) = m1827;
                let m1825 = v2;
                match m1825.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1826: Fn8 = v0.clone();
                            c1826.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f441(v0: Fn9, p1834: u64, p1835: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1833 = (p1834, p1835);
    {
        let mut m1833 = m1833;
        loop {
            let step = {
                let (v1, v2) = m1833;
                let m1830 = v1;
                if m1830 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1830 - 1;
                    let m1832 = {
                        let c1831: Fn9 = v0.clone();
                        c1831.apply(v2.clone())?
                    };
                    match m1832 {
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
                Ok(next) => { m1833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f498(v0)
}

pub fn f443(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f499(Fn13::F500, List::<(Bytes, ())>::nil(), v0)
}

pub fn f444(v0: Fn10, p1839: List<(Bytes, ())>, p1840: List<Bytes>) -> List<(Bytes, ())> {
    let m1838 = (p1839, p1840);
    {
        let mut m1838 = m1838;
        loop {
            let step = {
                let (v1, v2) = m1838;
                let m1836 = v2;
                match m1836.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1837: Fn10 = v0.clone();
                            c1837.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1838 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f446(p1845: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1844 = p1845;
    {
        let mut m1844 = m1844;
        loop {
            let step = {
                let v0 = m1844;
                let m1841 = v0;
                match m1841.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1842, _) = v2;
                            h1842
                        }) {
                            let (_, h1843) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1843)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1844 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: Bytes, p1871: u64, p1872: u64, p1873: Bytes) -> R<Bytes> {
    let m1870 = (p1871, (p1872, p1873));
    {
        let mut m1870 = m1870;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1870;
                let m1846 = v2;
                if m1846 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1846 - 1;
                    let m1849 = {
                        let a1847 = v0.clone();
                        let a1848 = v1;
                        index_bytes(a1847, a1848)
                    };
                    match m1849 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f199({
                                let a1850 = v6;
                                let a1851 = 34u64;
                                nat_eq(a1850, a1851)
                            }, {
                                let a1852 = v6;
                                let a1853 = 92u64;
                                nat_eq(a1852, a1853)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1854 = v1;
                                    let a1855 = 1u64;
                                    nat_add(a1854, a1855)?
                                }, (v4, {
                                    let a1858 = v3;
                                    let a1859 = {
                                        let a1856 = f8(Str::lit("\\"));
                                        let a1857 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1856, a1857)
                                    };
                                    append_bytes(a1858, a1859)
                                })))
                            } else {
                                let a1860 = v6;
                                let a1861 = 10u64;
                                if nat_eq(a1860, a1861) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1862 = v1;
                                        let a1863 = 1u64;
                                        nat_add(a1862, a1863)?
                                    }, (v4, {
                                        let a1864 = v3;
                                        let a1865 = f8(Str::lit("\\n"));
                                        append_bytes(a1864, a1865)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1866 = v1;
                                        let a1867 = 1u64;
                                        nat_add(a1866, a1867)?
                                    }, (v4, {
                                        let a1868 = v3;
                                        let a1869 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1868, a1869)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1870 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f448(v0: Option<Adt2>, v1: Bytes) -> Bytes {
    let m1874 = v0;
    match m1874 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f449(v0: List<Adt2>) -> List<Bytes> {
    let m1875 = v0;
    match m1875.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f501(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f502(f58(v1, f8(Str::lit("authority")))), f449(v2)))
        }
    }
}

pub fn f450(v0: List<Bytes>) -> R<List<Adt2>> {
    let m1876 = f287(v0.clone())?;
    match m1876 {
        None => {
            Ok(f276(f275(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt2>::nil())
        }
    }
}

pub fn f451(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    let m1877 = f287(v3.clone())?;
    match m1877 {
        None => {
            let m1878 = f275(v3.clone())?;
            match m1878 {
                None => {
                    f505(v0, v1, v3)
                }
                Some(v5) => {
                    f504(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f503(v0, v1, v4)
        }
    }
}

pub fn f452(v0: List<Bytes>) -> List<Bytes> {
    let m1879 = v0;
    match m1879.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1881 = {
                    let a1880 = v1.clone();
                    length_bytes(a1880)
                };
                let a1882 = 0u64;
                if nat_eq(a1881, a1882) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f452(v2))
        }
    }
}

pub fn f453(v0: Bytes, v1: u64) -> R<u64> {
    let m1885 = {
        let a1883 = v0;
        let a1884 = v1;
        index_bytes(a1883, a1884)
    };
    match m1885 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f454(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1889 = {
        let a1886 = v1;
        let a1887 = v2;
        let a1888 = 1u64;
        slice_bytes(a1886, a1887, a1888)
    };
    match m1889 {
        None => {
            v0
        }
        Some(v3) => {
            let a1890 = v0;
            let a1891 = v3;
            append_bytes(a1890, a1891)
        }
    }
}

pub fn f455(p1894: List<Bytes>, p1895: List<Bytes>) -> List<Bytes> {
    let m1893 = (p1894, p1895);
    {
        let mut m1893 = m1893;
        loop {
            let step = {
                let (v0, v1) = m1893;
                let m1892 = v0;
                match m1892.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f456(v0: List<Bytes>) -> Bytes {
    let m1896 = v0;
    match m1896.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f457(v0: Adt2) -> Bytes {
    if f217(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f217(v0.clone(), f8(Str::lit("id"))) {
            if f217(v0.clone(), f8(Str::lit("statement"))) {
                if f217(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f387(v0, f8(Str::lit("evidence")))
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

pub fn f458(v0: Bytes) -> R<bool> {
    let m1898 = {
        let a1897 = v0.clone();
        length_bytes(a1897)
    };
    if m1898 == 0 {
        Ok(true)
    } else {
        let v1 = m1898 - 1;
        let m1901 = {
            let a1899 = v0;
            let a1900 = v1;
            index_bytes(a1899, a1900)
        };
        match m1901 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f506({
                    let v3: u8 = v2;
                    let a1946 = {
                        let a1922 = {
                            let a1910 = {
                                let a1904 = {
                                    let a1902 = v3;
                                    let a1903 = 128u8;
                                    fixed_u8::bit_and(a1902, a1903)
                                };
                                let a1905 = 128u8;
                                if equal(a1904, a1905) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1911 = {
                                let a1908 = {
                                    let a1906 = v3;
                                    let a1907 = 64u8;
                                    fixed_u8::bit_and(a1906, a1907)
                                };
                                let a1909 = 64u8;
                                if equal(a1908, a1909) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1910, a1911)?
                        };
                        let a1923 = {
                            let a1920 = {
                                let a1914 = {
                                    let a1912 = v3;
                                    let a1913 = 32u8;
                                    fixed_u8::bit_and(a1912, a1913)
                                };
                                let a1915 = 32u8;
                                if equal(a1914, a1915) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1921 = {
                                let a1918 = {
                                    let a1916 = v3;
                                    let a1917 = 16u8;
                                    fixed_u8::bit_and(a1916, a1917)
                                };
                                let a1919 = 16u8;
                                if equal(a1918, a1919) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1920, a1921)?
                        };
                        nat_add(a1922, a1923)?
                    };
                    let a1947 = {
                        let a1944 = {
                            let a1932 = {
                                let a1926 = {
                                    let a1924 = v3;
                                    let a1925 = 8u8;
                                    fixed_u8::bit_and(a1924, a1925)
                                };
                                let a1927 = 8u8;
                                if equal(a1926, a1927) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1933 = {
                                let a1930 = {
                                    let a1928 = v3;
                                    let a1929 = 4u8;
                                    fixed_u8::bit_and(a1928, a1929)
                                };
                                let a1931 = 4u8;
                                if equal(a1930, a1931) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1932, a1933)?
                        };
                        let a1945 = {
                            let a1942 = {
                                let a1936 = {
                                    let a1934 = v3;
                                    let a1935 = 2u8;
                                    fixed_u8::bit_and(a1934, a1935)
                                };
                                let a1937 = 2u8;
                                if equal(a1936, a1937) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1943 = {
                                let a1940 = {
                                    let a1938 = v3;
                                    let a1939 = 1u8;
                                    fixed_u8::bit_and(a1938, a1939)
                                };
                                let a1941 = 1u8;
                                if equal(a1940, a1941) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1942, a1943)?
                        };
                        nat_add(a1944, a1945)?
                    };
                    nat_add(a1946, a1947)?
                }))
            }
        }
    }
}

pub fn f459(v0: Adt2, v1: Bytes) -> bool {
    let a1949 = {
        let a1948 = f58(v0, v1);
        length_bytes(a1948)
    };
    let a1950 = 0u64;
    !nat_eq(a1949, a1950)
}

pub fn f460(v0: Adt2, v1: Bytes) -> bool {
    let m1951 = f93(v0, v1);
    match m1951 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f461(v0: List<Bytes>) -> Option<Adt2> {
    let m1952 = v0;
    match m1952.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1953 = v2;
                match m1953.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1954 = v2;
                    match m1954.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m1955 = v6;
                                match m1955.uncons() {
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
                        let m1956 = v2;
                        match m1956.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m1957 = v10;
                                match m1957.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C11(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C6(v11)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f385(v12)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C6(v9)), std::rc::Rc::new(Adt2::C9))))))))), std::rc::Rc::new(Adt2::C9)))
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

pub fn f462(v0: Bytes, p1978: u64, p1979: u64, p1980: bool, p1981: Bytes) -> R<Bytes> {
    let m1977 = (p1978, (p1979, (p1980, p1981)));
    {
        let mut m1977 = m1977;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1977;
                let m1958 = v2;
                if m1958 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1958 - 1;
                    let m1961 = {
                        let a1959 = v0.clone();
                        let a1960 = v1;
                        index_bytes(a1959, a1960)
                    };
                    match m1961 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f507(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1962 = v1;
                                    let a1963 = 1u64;
                                    nat_add(a1962, a1963)?
                                }, (v5, (true, v4))))
                            } else {
                                if f117(f508(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1964 = v1;
                                        let a1965 = 1u64;
                                        nat_add(a1964, a1965)?
                                    }, (v5, (false, {
                                        let a1969 = v4;
                                        let a1970 = f46({
                                            let a1966 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1966)
                                        }, {
                                            let a1967 = v7;
                                            let a1968 = 97u64;
                                            nat_sub(a1967, a1968)
                                        }, 1u64);
                                        append_bytes(a1969, a1970)
                                    }))))
                                } else {
                                    if f199(f508(v7), f199(f509(v7), f510(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1971 = v1;
                                            let a1972 = 1u64;
                                            nat_add(a1971, a1972)?
                                        }, (v5, (false, {
                                            let a1973 = v4;
                                            let a1974 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1973, a1974)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1975 = v1;
                                            let a1976 = 1u64;
                                            nat_add(a1975, a1976)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1977 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f384(f511(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f464(v0: u32) -> Bytes {
    let a2010 = f512({
        let a1982 = f513(v0, 28u32);
        let a1983 = 15u32;
        fixed_u32::bit_and(a1982, a1983)
    });
    let a2011 = {
        let a2008 = f512({
            let a1984 = f513(v0, 24u32);
            let a1985 = 15u32;
            fixed_u32::bit_and(a1984, a1985)
        });
        let a2009 = {
            let a2006 = f512({
                let a1986 = f513(v0, 20u32);
                let a1987 = 15u32;
                fixed_u32::bit_and(a1986, a1987)
            });
            let a2007 = {
                let a2004 = f512({
                    let a1988 = f513(v0, 16u32);
                    let a1989 = 15u32;
                    fixed_u32::bit_and(a1988, a1989)
                });
                let a2005 = {
                    let a2002 = f512({
                        let a1990 = f513(v0, 12u32);
                        let a1991 = 15u32;
                        fixed_u32::bit_and(a1990, a1991)
                    });
                    let a2003 = {
                        let a2000 = f512({
                            let a1992 = f513(v0, 8u32);
                            let a1993 = 15u32;
                            fixed_u32::bit_and(a1992, a1993)
                        });
                        let a2001 = {
                            let a1998 = f512({
                                let a1994 = f513(v0, 4u32);
                                let a1995 = 15u32;
                                fixed_u32::bit_and(a1994, a1995)
                            });
                            let a1999 = f512({
                                let a1996 = v0;
                                let a1997 = 15u32;
                                fixed_u32::bit_and(a1996, a1997)
                            });
                            append_bytes(a1998, a1999)
                        };
                        append_bytes(a2000, a2001)
                    };
                    append_bytes(a2002, a2003)
                };
                append_bytes(a2004, a2005)
            };
            append_bytes(a2006, a2007)
        };
        append_bytes(a2008, a2009)
    };
    append_bytes(a2010, a2011)
}

pub fn f465(v0: Fn11, p2015: u64, p2016: (u64, Adt23)) -> R<(u64, Adt23)> {
    let m2014 = (p2015, p2016);
    {
        let mut m2014 = m2014;
        loop {
            let step = {
                let (v1, v2) = m2014;
                let m2012 = v1;
                if m2012 == 0 {
                    Err::<(u64, (u64, Adt23)), R<(u64, Adt23)>>(Ok(v2))
                } else {
                    let v3 = m2012 - 1;
                    Ok::<(u64, (u64, Adt23)), R<(u64, Adt23)>>((v3, {
                        let c2013: Fn11 = v0.clone();
                        c2013.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2014 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt23)) -> R<(u64, Adt23)> {
    Ok(({
        let a2018 = {
            let (h2017, _) = v3.clone();
            h2017
        };
        let a2019 = 64u64;
        nat_add(a2018, a2019)?
    }, f514({
        let (_, h2020) = v3;
        h2020
    }, f515(v0, v1, v2, {
        let (h2021, _) = v3;
        h2021
    })?)?))
}

pub fn f467() -> Adt23 {
    Adt23::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f468(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2022 = f44(v1, 34u8)?;
    match m2022.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2023 = v3;
            match m2023.uncons() {
                None => {
                    Ok(f516(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f469(v0: List<Bytes>) -> List<Bytes> {
    let m2024 = v0;
    match m2024.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f469(v2)
            } else {
                List::cons(v1, f469(v2))
            }
        }
    }
}

pub fn f470(p2027: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2026 = p2027;
    {
        let mut m2026 = m2026;
        loop {
            let step = {
                let v0 = m2026;
                let m2025 = v0;
                match m2025.uncons() {
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
                Ok(next) => { m2026 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2028 = v2;
    let a2029 = 0u64;
    if nat_eq(a2028, a2029) {
        Ok(f517(v0, 0u64))
    } else {
        let (_, h2030) = {
            let (h2031, _) = f518(Fn14::F519(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2031
        };
        Ok(h2030)
    }
}

pub fn f472() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f473(v0: List<u64>) -> R<Bytes> {
    let m2032 = f520(v0, List::<u64>::nil());
    match m2032.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a2033 = f390(v1)?;
            let a2034 = f521(v2)?;
            Ok(append_bytes(a2033, a2034))
        }
    }
}

pub fn f474(p2037: u64, p2038: List<u64>, v2: u64) -> R<List<u64>> {
    let m2036 = (p2037, p2038);
    {
        let mut m2036 = m2036;
        loop {
            let step = {
                let (v0, v1) = m2036;
                let m2035 = v0;
                if m2035 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m2035 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f522(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m2036 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475(v0: u64, v1: u64) -> List<u64> {
    let m2039 = v0;
    if m2039 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m2039 - 1;
        let a2040 = v1;
        let a2041 = 0u64;
        if nat_eq(a2040, a2041) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a2042 = v1;
                let a2043 = f523();
                let a2044 = 0u64;
                nat_rem(a2042, a2043, a2044)
            }, f475(v2, {
                let a2045 = v1;
                let a2046 = f523();
                let a2047 = 0u64;
                nat_quot(a2045, a2046, a2047)
            }))
        }
    }
}

pub fn f476(p2058: Bytes, p2059: u64) -> R<Bytes> {
    let m2057 = (p2058, p2059);
    {
        let mut m2057 = m2057;
        loop {
            let step = {
                let (v0, v1) = m2057;
                let m2048 = v1;
                if m2048 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m2048 - 1;
                    let a2052 = f115(v0.clone(), {
                        let a2050 = {
                            let a2049 = v0.clone();
                            length_bytes(a2049)
                        };
                        let a2051 = 1u64;
                        nat_sub(a2050, a2051)
                    })?;
                    let a2053 = 48u64;
                    if nat_eq(a2052, a2053) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a2055 = {
                                let a2054 = v0;
                                length_bytes(a2054)
                            };
                            let a2056 = 1u64;
                            nat_sub(a2055, a2056)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m2057 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477(v0: u64) -> Bytes {
    let m2060 = v0;
    if m2060 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2060 - 1;
        let a2061 = Bytes::lit(&[0x30]);
        let a2062 = f477(v1);
        append_bytes(a2061, a2062)
    }
}

pub fn f478(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2107 = {
            let a2083 = {
                let a2071 = {
                    let a2065 = {
                        let a2063 = v3;
                        let a2064 = 128u8;
                        fixed_u8::bit_and(a2063, a2064)
                    };
                    let a2066 = 128u8;
                    if equal(a2065, a2066) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2072 = {
                    let a2069 = {
                        let a2067 = v3;
                        let a2068 = 64u8;
                        fixed_u8::bit_and(a2067, a2068)
                    };
                    let a2070 = 64u8;
                    if equal(a2069, a2070) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2071, a2072)?
            };
            let a2084 = {
                let a2081 = {
                    let a2075 = {
                        let a2073 = v3;
                        let a2074 = 32u8;
                        fixed_u8::bit_and(a2073, a2074)
                    };
                    let a2076 = 32u8;
                    if equal(a2075, a2076) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2082 = {
                    let a2079 = {
                        let a2077 = v3;
                        let a2078 = 16u8;
                        fixed_u8::bit_and(a2077, a2078)
                    };
                    let a2080 = 16u8;
                    if equal(a2079, a2080) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2081, a2082)?
            };
            nat_add(a2083, a2084)?
        };
        let a2108 = {
            let a2105 = {
                let a2093 = {
                    let a2087 = {
                        let a2085 = v3;
                        let a2086 = 8u8;
                        fixed_u8::bit_and(a2085, a2086)
                    };
                    let a2088 = 8u8;
                    if equal(a2087, a2088) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2094 = {
                    let a2091 = {
                        let a2089 = v3;
                        let a2090 = 4u8;
                        fixed_u8::bit_and(a2089, a2090)
                    };
                    let a2092 = 4u8;
                    if equal(a2091, a2092) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2093, a2094)?
            };
            let a2106 = {
                let a2103 = {
                    let a2097 = {
                        let a2095 = v3;
                        let a2096 = 2u8;
                        fixed_u8::bit_and(a2095, a2096)
                    };
                    let a2098 = 2u8;
                    if equal(a2097, a2098) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2104 = {
                    let a2101 = {
                        let a2099 = v3;
                        let a2100 = 1u8;
                        fixed_u8::bit_and(a2099, a2100)
                    };
                    let a2102 = 1u8;
                    if equal(a2101, a2102) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2103, a2104)?
            };
            nat_add(a2105, a2106)?
        };
        nat_add(a2107, a2108)?
    };
    let a2109 = v4;
    let a2110 = 34u64;
    if nat_eq(a2109, a2110) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2111 = v4;
        let a2112 = 92u64;
        if nat_eq(a2111, a2112) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2113 = v4;
            let a2114 = 8u64;
            if nat_eq(a2113, a2114) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2115 = v4;
                let a2116 = 12u64;
                if nat_eq(a2115, a2116) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2117 = v4;
                    let a2118 = 10u64;
                    if nat_eq(a2117, a2118) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2119 = v4;
                        let a2120 = 13u64;
                        if nat_eq(a2119, a2120) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2121 = v4;
                            let a2122 = 9u64;
                            if nat_eq(a2121, a2122) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2123 = v4;
                                let a2124 = 32u64;
                                if nat_lt(a2123, a2124) {
                                    let a2125 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2126 = f524(v2)?;
                                    Ok(append_bytes(a2125, a2126))
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

pub fn f479(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2127 = v0.clone();
    match m2127.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2130 = {
                let (h2128, _) = v2;
                h2128
            };
            let a2131 = {
                let (h2129, _) = v1.clone();
                h2129
            };
            if equal(a2130, a2131) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f480(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f525(f526(Fn15::F527, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2136 = {
        let (h2132, _) = f528(Fn16::F529, {
            let a2134 = {
                let a2133 = v0;
                length_list(a2133)
            };
            let a2135 = 1u64;
            nat_add(a2134, a2135)?
        }, v1)?;
        h2132
    };
    match m2136.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f481(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2141 = {
        let a2138 = f299({
            let (h2137, _) = v0;
            h2137
        })?;
        let a2139 = Bytes::lit(&[0x3a]);
        append_bytes(a2138, a2139)
    };
    let a2142 = {
        let (_, h2140) = v0;
        h2140
    };
    Ok(append_bytes(a2141, a2142))
}

pub fn f482(v0: Bytes) -> R<Bytes> {
    let a2148 = {
        let a2146 = Bytes::lit(&[0x22]);
        let a2147 = {
            let (_, h2143) = {
                let (h2144, _) = f518(Fn14::F530(v0.clone()), {
                    let a2145 = v0;
                    length_bytes(a2145)
                }, (0u64, Bytes::lit(&[])))?;
                h2144
            };
            h2143
        };
        append_bytes(a2146, a2147)
    };
    let a2149 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2148, a2149))
}

pub fn f483(v0: Bytes, v1: u64) -> bool {
    let m2150 = f204(v0, v1);
    match m2150 {
        None => {
            false
        }
        Some(v2) => {
            f531(v2)
        }
    }
}

pub fn f484(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2151 = v0.clone();
        length_bytes(a2151)
    };
    let a2155 = {
        let a2152 = v1;
        let a2153 = 2u64;
        let a2154 = 0u64;
        nat_rem(a2152, a2153, a2154)
    };
    let a2156 = 0u64;
    if nat_eq(a2155, a2156) {
        let (_, h2157) = {
            let (h2158, _) = f532(Fn17::F533(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2158
        };
        Ok(h2157)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f485(v0: u64) -> bool {
    let a2159 = v0;
    let a2160 = 8u64;
    if nat_eq(a2159, a2160) {
        true
    } else {
        let a2161 = v0;
        let a2162 = 9u64;
        if nat_eq(a2161, a2162) {
            true
        } else {
            let a2163 = v0;
            let a2164 = 10u64;
            if nat_eq(a2163, a2164) {
                true
            } else {
                let a2165 = v0;
                let a2166 = 12u64;
                if nat_eq(a2165, a2166) {
                    true
                } else {
                    let a2167 = v0;
                    let a2168 = 13u64;
                    nat_eq(a2167, a2168)
                }
            }
        }
    }
}

pub fn f486(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2171 = {
        let a2169 = v0;
        let a2170 = v1;
        index_bytes(a2169, a2170)
    };
    match m2171 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f534(v2)? {
                Ok(Some({
                    let a2172 = v1;
                    let a2173 = 1u64;
                    nat_add(a2172, a2173)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f487(v0: u8) -> R<u64> {
    let a2220 = {
        let v1: u8 = v0;
        let a2218 = {
            let a2194 = {
                let a2182 = {
                    let a2176 = {
                        let a2174 = v1;
                        let a2175 = 128u8;
                        fixed_u8::bit_and(a2174, a2175)
                    };
                    let a2177 = 128u8;
                    if equal(a2176, a2177) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2183 = {
                    let a2180 = {
                        let a2178 = v1;
                        let a2179 = 64u8;
                        fixed_u8::bit_and(a2178, a2179)
                    };
                    let a2181 = 64u8;
                    if equal(a2180, a2181) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2182, a2183)?
            };
            let a2195 = {
                let a2192 = {
                    let a2186 = {
                        let a2184 = v1;
                        let a2185 = 32u8;
                        fixed_u8::bit_and(a2184, a2185)
                    };
                    let a2187 = 32u8;
                    if equal(a2186, a2187) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2193 = {
                    let a2190 = {
                        let a2188 = v1;
                        let a2189 = 16u8;
                        fixed_u8::bit_and(a2188, a2189)
                    };
                    let a2191 = 16u8;
                    if equal(a2190, a2191) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2192, a2193)?
            };
            nat_add(a2194, a2195)?
        };
        let a2219 = {
            let a2216 = {
                let a2204 = {
                    let a2198 = {
                        let a2196 = v1;
                        let a2197 = 8u8;
                        fixed_u8::bit_and(a2196, a2197)
                    };
                    let a2199 = 8u8;
                    if equal(a2198, a2199) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2205 = {
                    let a2202 = {
                        let a2200 = v1;
                        let a2201 = 4u8;
                        fixed_u8::bit_and(a2200, a2201)
                    };
                    let a2203 = 4u8;
                    if equal(a2202, a2203) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2204, a2205)?
            };
            let a2217 = {
                let a2214 = {
                    let a2208 = {
                        let a2206 = v1;
                        let a2207 = 2u8;
                        fixed_u8::bit_and(a2206, a2207)
                    };
                    let a2209 = 2u8;
                    if equal(a2208, a2209) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2215 = {
                    let a2212 = {
                        let a2210 = v1;
                        let a2211 = 1u8;
                        fixed_u8::bit_and(a2210, a2211)
                    };
                    let a2213 = 1u8;
                    if equal(a2212, a2213) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2214, a2215)?
            };
            nat_add(a2216, a2217)?
        };
        nat_add(a2218, a2219)?
    };
    let a2221 = 48u64;
    Ok(nat_sub(a2220, a2221))
}

pub fn f488(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2222 = v0.clone();
        length_bytes(a2222)
    };
    let a2223 = v1;
    let a2224 = v2;
    if if nat_lt(a2223, a2224) {
        let a2229 = f115(v0, {
            let a2227 = v2;
            let a2228 = {
                let a2225 = v1;
                let a2226 = 1u64;
                nat_add(a2225, a2226)?
            };
            nat_sub(a2227, a2228)
        })?;
        let a2230 = 48u64;
        nat_eq(a2229, a2230)
    } else {
        false
    } {
        Ok(Some({
            let a2231 = v1;
            let a2232 = 1u64;
            nat_add(a2231, a2232)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f489(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f117(f32({
        let (h2233, _) = v0;
        h2233
    }, {
        let (h2234, _) = v1;
        h2234
    }), f32({
        let (_, h2235) = v0;
        h2235
    }, {
        let (_, h2236) = v1;
        h2236
    }))
}

pub fn f490(v0: Bytes) -> R<bool> {
    let m2237 = f330(v0.clone(), f8(Str::lit("&&")))?;
    if match m2237 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2238 = f330(v0, f8(Str::lit("||")))?;
        match m2238 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f491(v0: Adt18, v1: Option<u64>) -> bool {
    let m2239 = v1;
    match m2239 {
        None => {
            true
        }
        Some(v2) => {
            f435(v2, match v0 {
                Adt18::C0(_, h2240, _, _) => {
                    h2240
                }
            })
        }
    }
}

pub fn f492(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a2242 = match v0 {
            Adt15::C0(_, _, h2241, _, _) => {
                h2241
            }
        };
        length_list(a2242)
    })
}

pub fn f493(v0: Bytes) -> Option<Adt22> {
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

pub fn f494(v0: Bytes, p2247: List<Bytes>, p2248: u64) -> R<Option<u64>> {
    let m2246 = (p2247, p2248);
    {
        let mut m2246 = m2246;
        loop {
            let step = {
                let (v1, v2) = m2246;
                let m2243 = v1;
                match m2243.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2244 = v2;
                                let a2245 = 1u64;
                                nat_add(a2244, a2245)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(v0: Option<Adt2>) -> Option<u64> {
    let m2249 = v0;
    match m2249 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2250 = v1;
            match m2250 {
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

pub fn f496(v0: Fn12, p2254: List<List<(Bytes, ())>>, p2255: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2253 = (p2254, p2255);
    {
        let mut m2253 = m2253;
        loop {
            let step = {
                let (v1, v2) = m2253;
                let m2251 = v2;
                match m2251.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2252: Fn12 = v0.clone();
                            c2252.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2253 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f498(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2257 = {
        let a2256 = v0.clone();
        length_list(a2256)
    };
    let a2258 = 1u64;
    if nat_le(a2257, a2258) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f535(v0)?))
    }
}

pub fn f499(v0: Fn13, p2262: List<(Bytes, ())>, p2263: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2261 = (p2262, p2263);
    {
        let mut m2261 = m2261;
        loop {
            let step = {
                let (v1, v2) = m2261;
                let m2259 = v2;
                match m2259.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2260: Fn13 = v0.clone();
                            c2260.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2261 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f500(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f501(v0: Bytes) -> Bytes {
    f7(List::cons(f139(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f502(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f503(v0: Adt1, v1: Bytes, v2: List<Adt2>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f142(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f142(f96(v0, f153())?, f8(Str::lit("at"))));
    let a2265 = {
        let a2264 = v4.clone();
        length_list(a2264)
    };
    let a2266 = 0u64;
    if nat_eq(a2265, a2266) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2269 = {
            let a2267 = v4;
            length_list(a2267)
        };
        let a2270 = {
            let a2268 = v3;
            length_list(a2268)
        };
        if nat_eq(a2269, a2270) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f536(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f504(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt2>) -> R<(bool, Bytes)> {
    let m2271 = v3;
    match m2271.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f199(f537(v0.clone(), v6.clone())?, f52(v6.clone(), v2.clone())) {
                f504(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f502(v6.clone()))) {
                    let v9: (bool, Bytes) = f504(v0, v1.clone(), v2, v5)?;
                    let (h2272, _) = v9.clone();
                    if h2272 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2274 = f536(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2275 = {
                                let (_, h2273) = v9;
                                h2273
                            };
                            append_bytes(a2274, a2275)
                        }))
                    }
                } else {
                    if f117(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f538(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2276 = f8(Str::lit("Catalog."));
                        let a2277 = v8.clone();
                        append_bytes(a2276, a2277)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f504(v0, v1.clone(), v2, v5)?;
                        let (h2278, _) = v10.clone();
                        if h2278 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2280 = f536(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2281 = {
                                    let (_, h2279) = v10;
                                    h2279
                                };
                                append_bytes(a2280, a2281)
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

pub fn f505(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt2> = f276(f371(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt2> = f276(f371(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f125(f142(v3.clone(), f8(Str::lit("id"))), f142(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f125(v6.clone(), f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2283 = {
        let a2282 = v7.clone();
        length_list(a2282)
    };
    let a2284 = 0u64;
    if nat_eq(a2283, a2284) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2287 = {
                let a2285 = v7;
                length_list(a2285)
            };
            let a2288 = {
                let a2286 = v5;
                length_list(a2286)
            };
            nat_eq(a2287, a2288)
        }) {
            Ok((false, f536(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f145(f125(f62(f539(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f540(v4), f125(v6, f142(v3, f8(Str::lit("id")))))))?;
            let a2290 = {
                let a2289 = v8.clone();
                length_list(a2289)
            };
            let a2291 = 0u64;
            if nat_eq(a2290, a2291) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f536(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f122(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f506(v0: u64) -> bool {
    let a2292 = v0;
    let a2293 = 47u64;
    nat_eq(a2292, a2293)
}

pub fn f507(v0: u64) -> bool {
    f199({
        let a2294 = v0;
        let a2295 = 95u64;
        nat_eq(a2294, a2295)
    }, f199({
        let a2296 = v0;
        let a2297 = 45u64;
        nat_eq(a2296, a2297)
    }, f199({
        let a2298 = v0;
        let a2299 = 32u64;
        nat_eq(a2298, a2299)
    }, f199({
        let a2300 = v0;
        let a2301 = 46u64;
        nat_eq(a2300, a2301)
    }, {
        let a2302 = v0;
        let a2303 = 47u64;
        nat_eq(a2302, a2303)
    }))))
}

pub fn f508(v0: u64) -> bool {
    f117({
        let a2304 = 97u64;
        let a2305 = v0;
        nat_le(a2304, a2305)
    }, {
        let a2306 = v0;
        let a2307 = 122u64;
        nat_le(a2306, a2307)
    })
}

pub fn f509(v0: u64) -> bool {
    f117({
        let a2308 = 65u64;
        let a2309 = v0;
        nat_le(a2308, a2309)
    }, {
        let a2310 = v0;
        let a2311 = 90u64;
        nat_le(a2310, a2311)
    })
}

pub fn f510(v0: u64) -> bool {
    f117({
        let a2312 = 48u64;
        let a2313 = v0;
        nat_le(a2312, a2313)
    }, {
        let a2314 = v0;
        let a2315 = 57u64;
        nat_le(a2314, a2315)
    })
}

pub fn f511(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2316 = v0;
    match m2316.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f349(v1)?, f511(v2)?))
        }
    }
}

pub fn f512(v0: u32) -> Bytes {
    let a2319 = {
        let a2317 = v0;
        let a2318 = 8u32;
        fixed_u32::bit_and(a2317, a2318)
    };
    let a2320 = 0u32;
    if equal(a2319, a2320) {
        let a2323 = {
            let a2321 = v0;
            let a2322 = 4u32;
            fixed_u32::bit_and(a2321, a2322)
        };
        let a2324 = 0u32;
        if equal(a2323, a2324) {
            let a2327 = {
                let a2325 = v0;
                let a2326 = 2u32;
                fixed_u32::bit_and(a2325, a2326)
            };
            let a2328 = 0u32;
            if equal(a2327, a2328) {
                let a2331 = {
                    let a2329 = v0;
                    let a2330 = 1u32;
                    fixed_u32::bit_and(a2329, a2330)
                };
                let a2332 = 0u32;
                if equal(a2331, a2332) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2335 = {
                    let a2333 = v0;
                    let a2334 = 1u32;
                    fixed_u32::bit_and(a2333, a2334)
                };
                let a2336 = 0u32;
                if equal(a2335, a2336) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2339 = {
                let a2337 = v0;
                let a2338 = 2u32;
                fixed_u32::bit_and(a2337, a2338)
            };
            let a2340 = 0u32;
            if equal(a2339, a2340) {
                let a2343 = {
                    let a2341 = v0;
                    let a2342 = 1u32;
                    fixed_u32::bit_and(a2341, a2342)
                };
                let a2344 = 0u32;
                if equal(a2343, a2344) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2347 = {
                    let a2345 = v0;
                    let a2346 = 1u32;
                    fixed_u32::bit_and(a2345, a2346)
                };
                let a2348 = 0u32;
                if equal(a2347, a2348) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2351 = {
            let a2349 = v0;
            let a2350 = 4u32;
            fixed_u32::bit_and(a2349, a2350)
        };
        let a2352 = 0u32;
        if equal(a2351, a2352) {
            let a2355 = {
                let a2353 = v0;
                let a2354 = 2u32;
                fixed_u32::bit_and(a2353, a2354)
            };
            let a2356 = 0u32;
            if equal(a2355, a2356) {
                let a2359 = {
                    let a2357 = v0;
                    let a2358 = 1u32;
                    fixed_u32::bit_and(a2357, a2358)
                };
                let a2360 = 0u32;
                if equal(a2359, a2360) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2363 = {
                    let a2361 = v0;
                    let a2362 = 1u32;
                    fixed_u32::bit_and(a2361, a2362)
                };
                let a2364 = 0u32;
                if equal(a2363, a2364) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2367 = {
                let a2365 = v0;
                let a2366 = 2u32;
                fixed_u32::bit_and(a2365, a2366)
            };
            let a2368 = 0u32;
            if equal(a2367, a2368) {
                let a2371 = {
                    let a2369 = v0;
                    let a2370 = 1u32;
                    fixed_u32::bit_and(a2369, a2370)
                };
                let a2372 = 0u32;
                if equal(a2371, a2372) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2375 = {
                    let a2373 = v0;
                    let a2374 = 1u32;
                    fixed_u32::bit_and(a2373, a2374)
                };
                let a2376 = 0u32;
                if equal(a2375, a2376) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f513(v0: u32, v1: u32) -> u32 {
    let m2379 = {
        let a2377 = v0;
        let a2378 = v1;
        fixed_u32::shift_right(a2377, a2378)
    };
    match m2379 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f514(v0: Adt23, v1: Adt24) -> R<Adt23> {
    let v2: Adt25 = f541(Fn18::F542, 64u64, Adt25::C0(match v0.clone() {
        Adt23::C0(h2380, _, _, _, _, _, _, _) => {
            h2380
        }
    }, match v0.clone() {
        Adt23::C0(_, h2381, _, _, _, _, _, _) => {
            h2381
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h2382, _, _, _, _, _) => {
            h2382
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h2383, _, _, _, _) => {
            h2383
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, h2384, _, _, _) => {
            h2384
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2385, _, _) => {
            h2385
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2386, _) => {
            h2386
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h2387) => {
            h2387
        }
    }, v1, 0u64))?;
    Ok(Adt23::C0(f543(match v0.clone() {
        Adt23::C0(h2388, _, _, _, _, _, _, _) => {
            h2388
        }
    }, match v2.clone() {
        Adt25::C0(h2389, _, _, _, _, _, _, _, _, _) => {
            h2389
        }
    }), f543(match v0.clone() {
        Adt23::C0(_, h2390, _, _, _, _, _, _) => {
            h2390
        }
    }, match v2.clone() {
        Adt25::C0(_, h2391, _, _, _, _, _, _, _, _) => {
            h2391
        }
    }), f543(match v0.clone() {
        Adt23::C0(_, _, h2392, _, _, _, _, _) => {
            h2392
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h2393, _, _, _, _, _, _, _) => {
            h2393
        }
    }), f543(match v0.clone() {
        Adt23::C0(_, _, _, h2394, _, _, _, _) => {
            h2394
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h2395, _, _, _, _, _, _) => {
            h2395
        }
    }), f543(match v0.clone() {
        Adt23::C0(_, _, _, _, h2396, _, _, _) => {
            h2396
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h2397, _, _, _, _, _) => {
            h2397
        }
    }), f543(match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2398, _, _) => {
            h2398
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h2399, _, _, _, _) => {
            h2399
        }
    }), f543(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2400, _) => {
            h2400
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h2401, _, _, _) => {
            h2401
        }
    }), f543(match v0 {
        Adt23::C0(_, _, _, _, _, _, _, h2402) => {
            h2402
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h2403, _, _) => {
            h2403
        }
    })))
}

pub fn f515(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f544(v0.clone(), v1, v2, v3)?, f544(v0.clone(), v1, v2, {
        let a2404 = v3;
        let a2405 = 4u64;
        nat_add(a2404, a2405)?
    })?, f544(v0.clone(), v1, v2, {
        let a2406 = v3;
        let a2407 = 8u64;
        nat_add(a2406, a2407)?
    })?, f544(v0.clone(), v1, v2, {
        let a2408 = v3;
        let a2409 = 12u64;
        nat_add(a2408, a2409)?
    })?, f544(v0.clone(), v1, v2, {
        let a2410 = v3;
        let a2411 = 16u64;
        nat_add(a2410, a2411)?
    })?, f544(v0.clone(), v1, v2, {
        let a2412 = v3;
        let a2413 = 20u64;
        nat_add(a2412, a2413)?
    })?, f544(v0.clone(), v1, v2, {
        let a2414 = v3;
        let a2415 = 24u64;
        nat_add(a2414, a2415)?
    })?, f544(v0.clone(), v1, v2, {
        let a2416 = v3;
        let a2417 = 28u64;
        nat_add(a2416, a2417)?
    })?, f544(v0.clone(), v1, v2, {
        let a2418 = v3;
        let a2419 = 32u64;
        nat_add(a2418, a2419)?
    })?, f544(v0.clone(), v1, v2, {
        let a2420 = v3;
        let a2421 = 36u64;
        nat_add(a2420, a2421)?
    })?, f544(v0.clone(), v1, v2, {
        let a2422 = v3;
        let a2423 = 40u64;
        nat_add(a2422, a2423)?
    })?, f544(v0.clone(), v1, v2, {
        let a2424 = v3;
        let a2425 = 44u64;
        nat_add(a2424, a2425)?
    })?, f544(v0.clone(), v1, v2, {
        let a2426 = v3;
        let a2427 = 48u64;
        nat_add(a2426, a2427)?
    })?, f544(v0.clone(), v1, v2, {
        let a2428 = v3;
        let a2429 = 52u64;
        nat_add(a2428, a2429)?
    })?, f544(v0.clone(), v1, v2, {
        let a2430 = v3;
        let a2431 = 56u64;
        nat_add(a2430, a2431)?
    })?, f544(v0, v1, v2, {
        let a2432 = v3;
        let a2433 = 60u64;
        nat_add(a2432, a2433)?
    })?))
}

pub fn f516(p2436: List<Bytes>, v1: Bytes) -> bool {
    let m2435 = p2436;
    {
        let mut m2435 = m2435;
        loop {
            let step = {
                let v0 = m2435;
                let m2434 = v0;
                match m2434.uncons() {
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
                Ok(next) => { m2435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: Bytes, v1: u64) -> Bytes {
    let m2440 = {
        let a2437 = v0;
        let a2438 = v1;
        let a2439 = 1u64;
        slice_bytes(a2437, a2438, a2439)
    };
    match m2440 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f518(v0: Fn14, p2445: u64, p2446: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m2444 = (p2445, p2446);
    {
        let mut m2444 = m2444;
        loop {
            let step = {
                let (v1, v2) = m2444;
                let m2441 = v1;
                if m2441 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2441 - 1;
                    let m2443 = {
                        let c2442: Fn14 = v0.clone();
                        c2442.apply(v2.clone())?
                    };
                    match m2443 {
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
                Ok(next) => { m2444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f519(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f545(v1, v0, v2)
}

pub fn f520(p2449: List<u64>, p2450: List<u64>) -> List<u64> {
    let m2448 = (p2449, p2450);
    {
        let mut m2448 = m2448;
        loop {
            let step = {
                let (v0, v1) = m2448;
                let m2447 = v0;
                match m2447.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: List<u64>) -> R<Bytes> {
    let m2451 = v0;
    match m2451.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a2452 = f546(v1)?;
            let a2453 = f521(v2)?;
            Ok(append_bytes(a2452, a2453))
        }
    }
}

pub fn f522(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m2454 = v0;
    match m2454.uncons() {
        None => {
            let a2455 = v2;
            let a2456 = 0u64;
            if nat_eq(a2455, a2456) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a2459 = {
                    let a2457 = v3;
                    let a2458 = v1;
                    nat_mul(a2457, a2458)?
                };
                let a2460 = v2;
                nat_add(a2459, a2460)?
            };
            Ok(List::cons({
                let a2461 = v5;
                let a2462 = f523();
                let a2463 = 0u64;
                nat_rem(a2461, a2462, a2463)
            }, f522(v4, v1, {
                let a2464 = v5;
                let a2465 = f523();
                let a2466 = 0u64;
                nat_quot(a2464, a2465, a2466)
            })?))
        }
    }
}

pub fn f523() -> u64 {
    1000000000u64
}

pub fn f524(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2511 = {
            let a2487 = {
                let a2475 = {
                    let a2469 = {
                        let a2467 = v1;
                        let a2468 = 128u8;
                        fixed_u8::bit_and(a2467, a2468)
                    };
                    let a2470 = 128u8;
                    if equal(a2469, a2470) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2476 = {
                    let a2473 = {
                        let a2471 = v1;
                        let a2472 = 64u8;
                        fixed_u8::bit_and(a2471, a2472)
                    };
                    let a2474 = 64u8;
                    if equal(a2473, a2474) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2475, a2476)?
            };
            let a2488 = {
                let a2485 = {
                    let a2479 = {
                        let a2477 = v1;
                        let a2478 = 32u8;
                        fixed_u8::bit_and(a2477, a2478)
                    };
                    let a2480 = 32u8;
                    if equal(a2479, a2480) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2486 = {
                    let a2483 = {
                        let a2481 = v1;
                        let a2482 = 16u8;
                        fixed_u8::bit_and(a2481, a2482)
                    };
                    let a2484 = 16u8;
                    if equal(a2483, a2484) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2485, a2486)?
            };
            nat_add(a2487, a2488)?
        };
        let a2512 = {
            let a2509 = {
                let a2497 = {
                    let a2491 = {
                        let a2489 = v1;
                        let a2490 = 8u8;
                        fixed_u8::bit_and(a2489, a2490)
                    };
                    let a2492 = 8u8;
                    if equal(a2491, a2492) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2498 = {
                    let a2495 = {
                        let a2493 = v1;
                        let a2494 = 4u8;
                        fixed_u8::bit_and(a2493, a2494)
                    };
                    let a2496 = 4u8;
                    if equal(a2495, a2496) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2497, a2498)?
            };
            let a2510 = {
                let a2507 = {
                    let a2501 = {
                        let a2499 = v1;
                        let a2500 = 2u8;
                        fixed_u8::bit_and(a2499, a2500)
                    };
                    let a2502 = 2u8;
                    if equal(a2501, a2502) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2508 = {
                    let a2505 = {
                        let a2503 = v1;
                        let a2504 = 1u8;
                        fixed_u8::bit_and(a2503, a2504)
                    };
                    let a2506 = 1u8;
                    if equal(a2505, a2506) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2507, a2508)?
            };
            nat_add(a2509, a2510)?
        };
        nat_add(a2511, a2512)?
    };
    let a2519 = f517(f547(), {
        let a2513 = v2;
        let a2514 = 16u64;
        let a2515 = 0u64;
        nat_quot(a2513, a2514, a2515)
    });
    let a2520 = f517(f547(), {
        let a2516 = v2;
        let a2517 = 16u64;
        let a2518 = 0u64;
        nat_rem(a2516, a2517, a2518)
    });
    Ok(append_bytes(a2519, a2520))
}

pub fn f525(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f548(Fn19::F549, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f526(v0: Fn15, p2524: List<List<(Bytes, Bytes)>>, p2525: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2523 = (p2524, p2525);
    {
        let mut m2523 = m2523;
        loop {
            let step = {
                let (v1, v2) = m2523;
                let m2521 = v2;
                match m2521.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2522: Fn15 = v0.clone();
                            c2522.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f528(v0: Fn16, p2530: u64, p2531: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2529 = (p2530, p2531);
    {
        let mut m2529 = m2529;
        loop {
            let step = {
                let (v1, v2) = m2529;
                let m2526 = v1;
                if m2526 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2526 - 1;
                    let m2528 = {
                        let c2527: Fn16 = v0.clone();
                        c2527.apply(v2.clone())?
                    };
                    match m2528 {
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
                Ok(next) => { m2529 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f550(v0)
}

pub fn f530(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f551(v0, v1)
}

pub fn f531(v0: u8) -> bool {
    let a2532 = v0;
    let a2533 = 32u8;
    if if if equal(a2532, a2533) {
        true
    } else {
        let a2534 = v0;
        let a2535 = 9u8;
        equal(a2534, a2535)
    } {
        true
    } else {
        let a2536 = v0;
        let a2537 = 10u8;
        equal(a2536, a2537)
    } {
        true
    } else {
        let a2538 = v0;
        let a2539 = 12u8;
        if equal(a2538, a2539) {
            true
        } else {
            let a2540 = v0;
            let a2541 = 13u8;
            equal(a2540, a2541)
        }
    }
}

pub fn f532(v0: Fn17, p2546: u64, p2547: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2545 = (p2546, p2547);
    {
        let mut m2545 = m2545;
        loop {
            let step = {
                let (v1, v2) = m2545;
                let m2542 = v1;
                if m2542 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2542 - 1;
                    let m2544 = {
                        let c2543: Fn17 = v0.clone();
                        c2543.apply(v2.clone())?
                    };
                    match m2544 {
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
                Ok(next) => { m2545 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f533(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f552(v0, v1)
}

pub fn f534(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2592 = {
            let a2568 = {
                let a2556 = {
                    let a2550 = {
                        let a2548 = v1;
                        let a2549 = 128u8;
                        fixed_u8::bit_and(a2548, a2549)
                    };
                    let a2551 = 128u8;
                    if equal(a2550, a2551) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2557 = {
                    let a2554 = {
                        let a2552 = v1;
                        let a2553 = 64u8;
                        fixed_u8::bit_and(a2552, a2553)
                    };
                    let a2555 = 64u8;
                    if equal(a2554, a2555) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2556, a2557)?
            };
            let a2569 = {
                let a2566 = {
                    let a2560 = {
                        let a2558 = v1;
                        let a2559 = 32u8;
                        fixed_u8::bit_and(a2558, a2559)
                    };
                    let a2561 = 32u8;
                    if equal(a2560, a2561) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2567 = {
                    let a2564 = {
                        let a2562 = v1;
                        let a2563 = 16u8;
                        fixed_u8::bit_and(a2562, a2563)
                    };
                    let a2565 = 16u8;
                    if equal(a2564, a2565) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2566, a2567)?
            };
            nat_add(a2568, a2569)?
        };
        let a2593 = {
            let a2590 = {
                let a2578 = {
                    let a2572 = {
                        let a2570 = v1;
                        let a2571 = 8u8;
                        fixed_u8::bit_and(a2570, a2571)
                    };
                    let a2573 = 8u8;
                    if equal(a2572, a2573) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2579 = {
                    let a2576 = {
                        let a2574 = v1;
                        let a2575 = 4u8;
                        fixed_u8::bit_and(a2574, a2575)
                    };
                    let a2577 = 4u8;
                    if equal(a2576, a2577) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2578, a2579)?
            };
            let a2591 = {
                let a2588 = {
                    let a2582 = {
                        let a2580 = v1;
                        let a2581 = 2u8;
                        fixed_u8::bit_and(a2580, a2581)
                    };
                    let a2583 = 2u8;
                    if equal(a2582, a2583) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2589 = {
                    let a2586 = {
                        let a2584 = v1;
                        let a2585 = 1u8;
                        fixed_u8::bit_and(a2584, a2585)
                    };
                    let a2587 = 1u8;
                    if equal(a2586, a2587) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2588, a2589)?
            };
            nat_add(a2590, a2591)?
        };
        nat_add(a2592, a2593)?
    };
    let a2594 = 48u64;
    let a2595 = v2;
    if nat_le(a2594, a2595) {
        let a2596 = v2;
        let a2597 = 57u64;
        Ok(nat_le(a2596, a2597))
    } else {
        Ok(false)
    }
}

pub fn f535(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f438({
        let (_, h2598) = {
            let (h2599, _) = f553(Fn20::F554, {
                let a2601 = {
                    let a2600 = v0.clone();
                    length_list(a2600)
                };
                let a2602 = 1u64;
                nat_add(a2601, a2602)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2599
        };
        h2598
    }))
}

pub fn f536(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f537(v0: Adt1, v1: Bytes) -> R<bool> {
    if f67(v0.clone(), f501(v1.clone())) {
        let m2603 = f95(f64(v0, f501(v1)))?;
        match m2603.uncons() {
            None => {
                Ok(false)
            }
            Some((v2, _)) => {
                let m2604 = f43(v2)?;
                match m2604 {
                    None => {
                        Ok(false)
                    }
                    Some(v4) => {
                        let m2605 = f93(v4, f8(Str::lit("reader")));
                        match m2605 {
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

pub fn f538(p2609: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m2608 = p2609;
    {
        let mut m2608 = m2608;
        loop {
            let step = {
                let v0 = m2608;
                let m2606 = v0;
                match m2606.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f117(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2607 = f93(v3, f8(Str::lit("std")));
                            match m2607 {
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
                Ok(next) => { m2608 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f539(v0: List<Adt2>) -> List<Bytes> {
    let m2610 = v0;
    match m2610.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2611 = f93(v1, f8(Str::lit("std")));
            match m2611 {
                None => {
                    f539(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f539(v2))
                }
            }
        }
    }
}

pub fn f540(v0: List<Adt2>) -> List<Bytes> {
    let m2612 = v0;
    match m2612.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2613 = f92(f93(v1, f8(Str::lit("req"))));
            match m2613 {
                None => {
                    f540(v2)
                }
                Some(v3) => {
                    List::cons(v3, f540(v2))
                }
            }
        }
    }
}

pub fn f541(v0: Fn18, p2617: u64, p2618: Adt25) -> R<Adt25> {
    let m2616 = (p2617, p2618);
    {
        let mut m2616 = m2616;
        loop {
            let step = {
                let (v1, v2) = m2616;
                let m2614 = v1;
                if m2614 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m2614 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c2615: Fn18 = v0.clone();
                        c2615.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f542(v0: Adt25) -> R<Adt25> {
    f555(v0)
}

pub fn f543(v0: u32, v1: u32) -> u32 {
    let m2621 = {
        let a2619 = f556(v0);
        let a2620 = f556(v1);
        fixed_u64::checked_add(a2619, a2620)
    };
    match m2621 {
        None => {
            0u32
        }
        Some(v2) => {
            f557(v2)
        }
    }
}

pub fn f544(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2624 = {
        let a2622 = v3;
        let a2623 = 8u64;
        nat_add(a2622, a2623)?
    };
    let a2625 = v2;
    if nat_eq(a2624, a2625) {
        Ok(f558({
            let a2628 = {
                let a2626 = v1;
                let a2627 = 8u64;
                nat_mul(a2626, a2627)?
            };
            let a2629 = 4294967296u64;
            let a2630 = 0u64;
            nat_quot(a2628, a2629, a2630)
        }))
    } else {
        let a2633 = {
            let a2631 = v3;
            let a2632 = 4u64;
            nat_add(a2631, a2632)?
        };
        let a2634 = v2;
        if nat_eq(a2633, a2634) {
            Ok(f558({
                let a2637 = {
                    let a2635 = v1;
                    let a2636 = 8u64;
                    nat_mul(a2635, a2636)?
                };
                let a2638 = 4294967296u64;
                let a2639 = 0u64;
                nat_rem(a2637, a2638, a2639)
            }))
        } else {
            Ok(f559(f560(v0.clone(), v1, v3), f560(v0.clone(), v1, {
                let a2640 = v3;
                let a2641 = 1u64;
                nat_add(a2640, a2641)?
            }), f560(v0.clone(), v1, {
                let a2642 = v3;
                let a2643 = 2u64;
                nat_add(a2642, a2643)?
            }), f560(v0, v1, {
                let a2644 = v3;
                let a2645 = 3u64;
                nat_add(a2644, a2645)?
            })))
        }
    }
}

pub fn f545(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2646, _) = v2;
        h2646
    };
    let a2647 = v3;
    let a2648 = 0u64;
    if nat_eq(a2647, a2648) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2649 = v3;
            let a2650 = v1;
            let a2651 = 0u64;
            nat_quot(a2649, a2650, a2651)
        }, {
            let a2656 = f517(v0, {
                let a2652 = v3;
                let a2653 = v1;
                let a2654 = 0u64;
                nat_rem(a2652, a2653, a2654)
            });
            let a2657 = {
                let (_, h2655) = v2;
                h2655
            };
            append_bytes(a2656, a2657)
        }))
    }
}

pub fn f546(v0: u64) -> R<Bytes> {
    let v1: Bytes = f390(v0)?;
    let a2661 = f477({
        let a2659 = 9u64;
        let a2660 = {
            let a2658 = v1.clone();
            length_bytes(a2658)
        };
        nat_sub(a2659, a2660)
    });
    let a2662 = v1;
    Ok(append_bytes(a2661, a2662))
}

pub fn f547() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f548(v0: Fn19, p2666: List<List<(Bytes, Bytes)>>, p2667: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2665 = (p2666, p2667);
    {
        let mut m2665 = m2665;
        loop {
            let step = {
                let (v1, v2) = m2665;
                let m2663 = v2;
                match m2663.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2664: Fn19 = v0.clone();
                            c2664.apply(v1, v3)
                        }, v4))
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

pub fn f549(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f550(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2669 = {
        let a2668 = v0.clone();
        length_list(a2668)
    };
    let a2670 = 1u64;
    if nat_le(a2669, a2670) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f561(v0)?))
    }
}

pub fn f551(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2674 = {
        let a2672 = v0.clone();
        let a2673 = {
            let (h2671, _) = v1.clone();
            h2671
        };
        index_bytes(a2672, a2673)
    };
    match m2674 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2676 = {
                    let (h2675, _) = v1.clone();
                    h2675
                };
                let a2677 = 1u64;
                nat_add(a2676, a2677)?
            }, {
                let a2680 = {
                    let (_, h2678) = v1;
                    h2678
                };
                let a2681 = f478(v0, {
                    let (h2679, _) = v1;
                    h2679
                }, v2)?;
                append_bytes(a2680, a2681)
            })))
        }
    }
}

pub fn f552(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2682, _) = v1;
        h2682
    };
    let m2684 = {
        let (_, h2683) = v1;
        h2683
    };
    match m2684 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2687 = {
                let a2685 = v0.clone();
                let a2686 = v2;
                index_bytes(a2685, a2686)
            };
            match m2687 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2692 = {
                        let a2690 = v0;
                        let a2691 = {
                            let a2688 = v2;
                            let a2689 = 1u64;
                            nat_add(a2688, a2689)?
                        };
                        index_bytes(a2690, a2691)
                    };
                    match m2692 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f562(v4)? {
                                f562(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2693 = v2;
                                    let a2694 = 2u64;
                                    nat_add(a2693, a2694)?
                                }, Some({
                                    let a2699 = v3;
                                    let a2700 = f517(f563(), {
                                        let a2697 = {
                                            let a2695 = f564(v4)?;
                                            let a2696 = 16u64;
                                            nat_mul(a2695, a2696)?
                                        };
                                        let a2698 = f564(v5)?;
                                        nat_add(a2697, a2698)?
                                    });
                                    append_bytes(a2699, a2700)
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

pub fn f553(v0: Fn20, p2705: u64, p2706: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2704 = (p2705, p2706);
    {
        let mut m2704 = m2704;
        loop {
            let step = {
                let (v1, v2) = m2704;
                let m2701 = v1;
                if m2701 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2701 - 1;
                    let m2703 = {
                        let c2702: Fn20 = v0.clone();
                        c2702.apply(v2.clone())?
                    };
                    match m2703 {
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
                Ok(next) => { m2704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f554(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f565(v0)
}

pub fn f555(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f566(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h2707) => {
            h2707
        }
    });
    let v2: u32 = f543(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h2708, _, _) => {
            h2708
        }
    }, f543(f567(match v0.clone() {
        Adt25::C0(_, _, _, _, h2709, _, _, _, _, _) => {
            h2709
        }
    }), f543(f568(match v0.clone() {
        Adt25::C0(_, _, _, _, h2710, _, _, _, _, _) => {
            h2710
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2711, _, _, _, _) => {
            h2711
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2712, _, _, _) => {
            h2712
        }
    }), f543(v1, {
        let m2715 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h2714, _) => {
                h2714
            }
        };
        match m2715 {
            Adt24::C0(h2713, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2713
            }
        }
    }))));
    let v3: u32 = f543(f569(match v0.clone() {
        Adt25::C0(h2716, _, _, _, _, _, _, _, _, _) => {
            h2716
        }
    }), f570(match v0.clone() {
        Adt25::C0(h2717, _, _, _, _, _, _, _, _, _) => {
            h2717
        }
    }, match v0.clone() {
        Adt25::C0(_, h2718, _, _, _, _, _, _, _, _) => {
            h2718
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2719, _, _, _, _, _, _, _) => {
            h2719
        }
    }));
    Ok(Adt25::C0(f543(v2, v3), match v0.clone() {
        Adt25::C0(h2720, _, _, _, _, _, _, _, _, _) => {
            h2720
        }
    }, match v0.clone() {
        Adt25::C0(_, h2721, _, _, _, _, _, _, _, _) => {
            h2721
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2722, _, _, _, _, _, _, _) => {
            h2722
        }
    }, f543(match v0.clone() {
        Adt25::C0(_, _, _, h2723, _, _, _, _, _, _) => {
            h2723
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h2724, _, _, _, _, _) => {
            h2724
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2725, _, _, _, _) => {
            h2725
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2726, _, _, _) => {
            h2726
        }
    }, f571(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h2727, _) => {
            h2727
        }
    }), {
        let a2729 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h2728) => {
                h2728
            }
        };
        let a2730 = 1u64;
        nat_add(a2729, a2730)?
    }))
}

pub fn f556(v0: u32) -> u64 {
    let m2732 = {
        let a2731 = v0;
        fixed_u64::convert(i128::from(a2731))
    };
    match m2732 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f557(v0: u64) -> u32 {
    let m2736 = {
        let a2735 = {
            let a2733 = v0;
            let a2734 = 4294967295u64;
            fixed_u64::bit_and(a2733, a2734)
        };
        fixed_u32::convert(i128::from(a2735))
    };
    match m2736 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f558(v0: u64) -> u32 {
    let (_, h2737) = {
        let (_, h2738) = f572(Fn21::F573, 32u64, (v0, (1u32, 0u32)));
        h2738
    };
    h2737
}

pub fn f559(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2743 = f574(v0, 24u32);
    let a2744 = {
        let a2741 = f574(v1, 16u32);
        let a2742 = {
            let a2739 = f574(v2, 8u32);
            let a2740 = v3;
            fixed_u32::bit_or(a2739, a2740)
        };
        fixed_u32::bit_or(a2741, a2742)
    };
    fixed_u32::bit_or(a2743, a2744)
}

pub fn f560(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2745 = v2;
    let a2746 = v1;
    if nat_lt(a2745, a2746) {
        let m2749 = {
            let a2747 = v0;
            let a2748 = v2;
            index_bytes(a2747, a2748)
        };
        match m2749 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2751 = {
                    let a2750 = v3;
                    fixed_u32::convert(i128::from(a2750))
                };
                match m2751 {
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
        let a2752 = v2;
        let a2753 = v1;
        if nat_eq(a2752, a2753) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f561(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f525({
        let (_, h2754) = {
            let (h2755, _) = f575(Fn22::F576, {
                let a2757 = {
                    let a2756 = v0.clone();
                    length_list(a2756)
                };
                let a2758 = 1u64;
                nat_add(a2757, a2758)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2755
        };
        h2754
    }))
}

pub fn f562(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2803 = {
            let a2779 = {
                let a2767 = {
                    let a2761 = {
                        let a2759 = v1;
                        let a2760 = 128u8;
                        fixed_u8::bit_and(a2759, a2760)
                    };
                    let a2762 = 128u8;
                    if equal(a2761, a2762) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2768 = {
                    let a2765 = {
                        let a2763 = v1;
                        let a2764 = 64u8;
                        fixed_u8::bit_and(a2763, a2764)
                    };
                    let a2766 = 64u8;
                    if equal(a2765, a2766) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2767, a2768)?
            };
            let a2780 = {
                let a2777 = {
                    let a2771 = {
                        let a2769 = v1;
                        let a2770 = 32u8;
                        fixed_u8::bit_and(a2769, a2770)
                    };
                    let a2772 = 32u8;
                    if equal(a2771, a2772) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2778 = {
                    let a2775 = {
                        let a2773 = v1;
                        let a2774 = 16u8;
                        fixed_u8::bit_and(a2773, a2774)
                    };
                    let a2776 = 16u8;
                    if equal(a2775, a2776) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2777, a2778)?
            };
            nat_add(a2779, a2780)?
        };
        let a2804 = {
            let a2801 = {
                let a2789 = {
                    let a2783 = {
                        let a2781 = v1;
                        let a2782 = 8u8;
                        fixed_u8::bit_and(a2781, a2782)
                    };
                    let a2784 = 8u8;
                    if equal(a2783, a2784) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2790 = {
                    let a2787 = {
                        let a2785 = v1;
                        let a2786 = 4u8;
                        fixed_u8::bit_and(a2785, a2786)
                    };
                    let a2788 = 4u8;
                    if equal(a2787, a2788) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2789, a2790)?
            };
            let a2802 = {
                let a2799 = {
                    let a2793 = {
                        let a2791 = v1;
                        let a2792 = 2u8;
                        fixed_u8::bit_and(a2791, a2792)
                    };
                    let a2794 = 2u8;
                    if equal(a2793, a2794) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2800 = {
                    let a2797 = {
                        let a2795 = v1;
                        let a2796 = 1u8;
                        fixed_u8::bit_and(a2795, a2796)
                    };
                    let a2798 = 1u8;
                    if equal(a2797, a2798) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2799, a2800)?
            };
            nat_add(a2801, a2802)?
        };
        nat_add(a2803, a2804)?
    };
    let a2805 = 48u64;
    let a2806 = v2;
    if if nat_le(a2805, a2806) {
        let a2807 = v2;
        let a2808 = 57u64;
        nat_le(a2807, a2808)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2809 = 97u64;
        let a2810 = v2;
        if nat_le(a2809, a2810) {
            let a2811 = v2;
            let a2812 = 102u64;
            Ok(nat_le(a2811, a2812))
        } else {
            Ok(false)
        }
    }
}

pub fn f563() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f564(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2857 = {
            let a2833 = {
                let a2821 = {
                    let a2815 = {
                        let a2813 = v1;
                        let a2814 = 128u8;
                        fixed_u8::bit_and(a2813, a2814)
                    };
                    let a2816 = 128u8;
                    if equal(a2815, a2816) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2822 = {
                    let a2819 = {
                        let a2817 = v1;
                        let a2818 = 64u8;
                        fixed_u8::bit_and(a2817, a2818)
                    };
                    let a2820 = 64u8;
                    if equal(a2819, a2820) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2821, a2822)?
            };
            let a2834 = {
                let a2831 = {
                    let a2825 = {
                        let a2823 = v1;
                        let a2824 = 32u8;
                        fixed_u8::bit_and(a2823, a2824)
                    };
                    let a2826 = 32u8;
                    if equal(a2825, a2826) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2832 = {
                    let a2829 = {
                        let a2827 = v1;
                        let a2828 = 16u8;
                        fixed_u8::bit_and(a2827, a2828)
                    };
                    let a2830 = 16u8;
                    if equal(a2829, a2830) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2831, a2832)?
            };
            nat_add(a2833, a2834)?
        };
        let a2858 = {
            let a2855 = {
                let a2843 = {
                    let a2837 = {
                        let a2835 = v1;
                        let a2836 = 8u8;
                        fixed_u8::bit_and(a2835, a2836)
                    };
                    let a2838 = 8u8;
                    if equal(a2837, a2838) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2844 = {
                    let a2841 = {
                        let a2839 = v1;
                        let a2840 = 4u8;
                        fixed_u8::bit_and(a2839, a2840)
                    };
                    let a2842 = 4u8;
                    if equal(a2841, a2842) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2843, a2844)?
            };
            let a2856 = {
                let a2853 = {
                    let a2847 = {
                        let a2845 = v1;
                        let a2846 = 2u8;
                        fixed_u8::bit_and(a2845, a2846)
                    };
                    let a2848 = 2u8;
                    if equal(a2847, a2848) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2854 = {
                    let a2851 = {
                        let a2849 = v1;
                        let a2850 = 1u8;
                        fixed_u8::bit_and(a2849, a2850)
                    };
                    let a2852 = 1u8;
                    if equal(a2851, a2852) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2853, a2854)?
            };
            nat_add(a2855, a2856)?
        };
        nat_add(a2857, a2858)?
    };
    let a2859 = v2;
    let a2860 = 57u64;
    if nat_le(a2859, a2860) {
        let a2861 = v2;
        let a2862 = 48u64;
        Ok(nat_sub(a2861, a2862))
    } else {
        let a2863 = v2;
        let a2864 = 87u64;
        Ok(nat_sub(a2863, a2864))
    }
}

pub fn f565(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2865, _) = v0;
        h2865
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2866) = v0;
        h2866
    };
    let m2867 = v1;
    match m2867.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2868 = v4;
            match m2868.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f577(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f566(v0: u64) -> u32 {
    let a2869 = v0;
    let a2870 = 32u64;
    if nat_lt(a2869, a2870) {
        let a2871 = v0;
        let a2872 = 16u64;
        if nat_lt(a2871, a2872) {
            let a2873 = v0;
            let a2874 = 8u64;
            if nat_lt(a2873, a2874) {
                let a2875 = v0;
                let a2876 = 4u64;
                if nat_lt(a2875, a2876) {
                    let a2877 = v0;
                    let a2878 = 2u64;
                    if nat_lt(a2877, a2878) {
                        let a2879 = v0;
                        let a2880 = 1u64;
                        if nat_lt(a2879, a2880) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2881 = v0;
                        let a2882 = 3u64;
                        if nat_lt(a2881, a2882) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2883 = v0;
                    let a2884 = 6u64;
                    if nat_lt(a2883, a2884) {
                        let a2885 = v0;
                        let a2886 = 5u64;
                        if nat_lt(a2885, a2886) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2887 = v0;
                        let a2888 = 7u64;
                        if nat_lt(a2887, a2888) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2889 = v0;
                let a2890 = 12u64;
                if nat_lt(a2889, a2890) {
                    let a2891 = v0;
                    let a2892 = 10u64;
                    if nat_lt(a2891, a2892) {
                        let a2893 = v0;
                        let a2894 = 9u64;
                        if nat_lt(a2893, a2894) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2895 = v0;
                        let a2896 = 11u64;
                        if nat_lt(a2895, a2896) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2897 = v0;
                    let a2898 = 14u64;
                    if nat_lt(a2897, a2898) {
                        let a2899 = v0;
                        let a2900 = 13u64;
                        if nat_lt(a2899, a2900) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2901 = v0;
                        let a2902 = 15u64;
                        if nat_lt(a2901, a2902) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2903 = v0;
            let a2904 = 24u64;
            if nat_lt(a2903, a2904) {
                let a2905 = v0;
                let a2906 = 20u64;
                if nat_lt(a2905, a2906) {
                    let a2907 = v0;
                    let a2908 = 18u64;
                    if nat_lt(a2907, a2908) {
                        let a2909 = v0;
                        let a2910 = 17u64;
                        if nat_lt(a2909, a2910) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2911 = v0;
                        let a2912 = 19u64;
                        if nat_lt(a2911, a2912) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2913 = v0;
                    let a2914 = 22u64;
                    if nat_lt(a2913, a2914) {
                        let a2915 = v0;
                        let a2916 = 21u64;
                        if nat_lt(a2915, a2916) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2917 = v0;
                        let a2918 = 23u64;
                        if nat_lt(a2917, a2918) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2919 = v0;
                let a2920 = 28u64;
                if nat_lt(a2919, a2920) {
                    let a2921 = v0;
                    let a2922 = 26u64;
                    if nat_lt(a2921, a2922) {
                        let a2923 = v0;
                        let a2924 = 25u64;
                        if nat_lt(a2923, a2924) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2925 = v0;
                        let a2926 = 27u64;
                        if nat_lt(a2925, a2926) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2927 = v0;
                    let a2928 = 30u64;
                    if nat_lt(a2927, a2928) {
                        let a2929 = v0;
                        let a2930 = 29u64;
                        if nat_lt(a2929, a2930) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2931 = v0;
                        let a2932 = 31u64;
                        if nat_lt(a2931, a2932) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2933 = v0;
        let a2934 = 48u64;
        if nat_lt(a2933, a2934) {
            let a2935 = v0;
            let a2936 = 40u64;
            if nat_lt(a2935, a2936) {
                let a2937 = v0;
                let a2938 = 36u64;
                if nat_lt(a2937, a2938) {
                    let a2939 = v0;
                    let a2940 = 34u64;
                    if nat_lt(a2939, a2940) {
                        let a2941 = v0;
                        let a2942 = 33u64;
                        if nat_lt(a2941, a2942) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2943 = v0;
                        let a2944 = 35u64;
                        if nat_lt(a2943, a2944) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2945 = v0;
                    let a2946 = 38u64;
                    if nat_lt(a2945, a2946) {
                        let a2947 = v0;
                        let a2948 = 37u64;
                        if nat_lt(a2947, a2948) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2949 = v0;
                        let a2950 = 39u64;
                        if nat_lt(a2949, a2950) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2951 = v0;
                let a2952 = 44u64;
                if nat_lt(a2951, a2952) {
                    let a2953 = v0;
                    let a2954 = 42u64;
                    if nat_lt(a2953, a2954) {
                        let a2955 = v0;
                        let a2956 = 41u64;
                        if nat_lt(a2955, a2956) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2957 = v0;
                        let a2958 = 43u64;
                        if nat_lt(a2957, a2958) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2959 = v0;
                    let a2960 = 46u64;
                    if nat_lt(a2959, a2960) {
                        let a2961 = v0;
                        let a2962 = 45u64;
                        if nat_lt(a2961, a2962) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2963 = v0;
                        let a2964 = 47u64;
                        if nat_lt(a2963, a2964) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2965 = v0;
            let a2966 = 56u64;
            if nat_lt(a2965, a2966) {
                let a2967 = v0;
                let a2968 = 52u64;
                if nat_lt(a2967, a2968) {
                    let a2969 = v0;
                    let a2970 = 50u64;
                    if nat_lt(a2969, a2970) {
                        let a2971 = v0;
                        let a2972 = 49u64;
                        if nat_lt(a2971, a2972) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2973 = v0;
                        let a2974 = 51u64;
                        if nat_lt(a2973, a2974) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2975 = v0;
                    let a2976 = 54u64;
                    if nat_lt(a2975, a2976) {
                        let a2977 = v0;
                        let a2978 = 53u64;
                        if nat_lt(a2977, a2978) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2979 = v0;
                        let a2980 = 55u64;
                        if nat_lt(a2979, a2980) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2981 = v0;
                let a2982 = 60u64;
                if nat_lt(a2981, a2982) {
                    let a2983 = v0;
                    let a2984 = 58u64;
                    if nat_lt(a2983, a2984) {
                        let a2985 = v0;
                        let a2986 = 57u64;
                        if nat_lt(a2985, a2986) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2987 = v0;
                        let a2988 = 59u64;
                        if nat_lt(a2987, a2988) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2989 = v0;
                    let a2990 = 62u64;
                    if nat_lt(a2989, a2990) {
                        let a2991 = v0;
                        let a2992 = 61u64;
                        if nat_lt(a2991, a2992) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2993 = v0;
                        let a2994 = 63u64;
                        if nat_lt(a2993, a2994) {
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

pub fn f567(v0: u32) -> u32 {
    let a2997 = {
        let a2995 = f578(v0, 6u32, 26u32);
        let a2996 = f578(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2995, a2996)
    };
    let a2998 = f578(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2997, a2998)
}

pub fn f568(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3004 = {
        let a2999 = v0;
        let a3000 = v1;
        fixed_u32::bit_and(a2999, a3000)
    };
    let a3005 = {
        let a3002 = {
            let a3001 = v0;
            fixed_u32::bit_not(a3001)
        };
        let a3003 = v2;
        fixed_u32::bit_and(a3002, a3003)
    };
    fixed_u32::bit_xor(a3004, a3005)
}

pub fn f569(v0: u32) -> u32 {
    let a3008 = {
        let a3006 = f578(v0, 2u32, 30u32);
        let a3007 = f578(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3006, a3007)
    };
    let a3009 = f578(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3008, a3009)
}

pub fn f570(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3018 = {
        let a3014 = {
            let a3010 = v0;
            let a3011 = v1;
            fixed_u32::bit_and(a3010, a3011)
        };
        let a3015 = {
            let a3012 = v0;
            let a3013 = v2;
            fixed_u32::bit_and(a3012, a3013)
        };
        fixed_u32::bit_xor(a3014, a3015)
    };
    let a3019 = {
        let a3016 = v1;
        let a3017 = v2;
        fixed_u32::bit_and(a3016, a3017)
    };
    fixed_u32::bit_xor(a3018, a3019)
}

pub fn f571(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h3020, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3020
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h3021, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3021
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h3022, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3022
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h3023, _, _, _, _, _, _, _, _, _, _, _) => {
            h3023
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h3024, _, _, _, _, _, _, _, _, _, _) => {
            h3024
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h3025, _, _, _, _, _, _, _, _, _) => {
            h3025
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h3026, _, _, _, _, _, _, _, _) => {
            h3026
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h3027, _, _, _, _, _, _, _) => {
            h3027
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h3028, _, _, _, _, _, _) => {
            h3028
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h3029, _, _, _, _, _) => {
            h3029
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h3030, _, _, _, _) => {
            h3030
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3031, _, _, _) => {
            h3031
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3032, _, _) => {
            h3032
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3033, _) => {
            h3033
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3034) => {
            h3034
        }
    }, f543(f579(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3035, _) => {
            h3035
        }
    }), f543(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h3036, _, _, _, _, _, _) => {
            h3036
        }
    }, f543(f580(match v0.clone() {
        Adt24::C0(_, h3037, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3037
        }
    }), match v0 {
        Adt24::C0(h3038, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3038
        }
    }))))
}

pub fn f572(v0: Fn21, p3042: u64, p3043: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3041 = (p3042, p3043);
    {
        let mut m3041 = m3041;
        loop {
            let step = {
                let (v1, v2) = m3041;
                let m3039 = v1;
                if m3039 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3039 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3040: Fn21 = v0.clone();
                        c3040.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3044, _) = v0;
        h3044
    };
    let v2: u32 = {
        let (h3045, _) = {
            let (_, h3046) = v0;
            h3046
        };
        h3045
    };
    let v3: u32 = {
        let (_, h3047) = {
            let (_, h3048) = v0;
            h3048
        };
        h3047
    };
    ({
        let a3049 = v1;
        let a3050 = 2u64;
        let a3051 = 0u64;
        nat_quot(a3049, a3050, a3051)
    }, (f574(v2, 1u32), {
        let a3055 = {
            let a3052 = v1;
            let a3053 = 2u64;
            let a3054 = 0u64;
            nat_rem(a3052, a3053, a3054)
        };
        let a3056 = 1u64;
        if nat_eq(a3055, a3056) {
            let a3057 = v3;
            let a3058 = v2;
            fixed_u32::bit_or(a3057, a3058)
        } else {
            v3
        }
    }))
}

pub fn f574(v0: u32, v1: u32) -> u32 {
    let m3061 = {
        let a3059 = v0;
        let a3060 = v1;
        fixed_u32::shift_left(a3059, a3060)
    };
    match m3061 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f575(v0: Fn22, p3066: u64, p3067: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3065 = (p3066, p3067);
    {
        let mut m3065 = m3065;
        loop {
            let step = {
                let (v1, v2) = m3065;
                let m3062 = v1;
                if m3062 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3062 - 1;
                    let m3064 = {
                        let c3063: Fn22 = v0.clone();
                        c3063.apply(v2.clone())?
                    };
                    match m3064 {
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
                Ok(next) => { m3065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f576(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f581(v0)
}

pub fn f577(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3072 = {
            let a3070 = {
                let a3068 = v0.clone();
                length_list(a3068)
            };
            let a3071 = {
                let a3069 = v1.clone();
                length_list(a3069)
            };
            nat_add(a3070, a3071)?
        };
        let a3073 = 1u64;
        nat_add(a3072, a3073)?
    };
    Ok(f443({
        let (_, h3074) = {
            let (_, h3075) = {
                let (h3076, _) = f582(Fn23::F583, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3076
            };
            h3075
        };
        h3074
    }))
}

pub fn f578(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3077 = f513(v0, v1);
    let a3078 = f574(v0, v2);
    fixed_u32::bit_or(a3077, a3078)
}

pub fn f579(v0: u32) -> u32 {
    let a3081 = {
        let a3079 = f578(v0, 17u32, 15u32);
        let a3080 = f578(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3079, a3080)
    };
    let a3082 = f513(v0, 10u32);
    fixed_u32::bit_xor(a3081, a3082)
}

pub fn f580(v0: u32) -> u32 {
    let a3085 = {
        let a3083 = f578(v0, 7u32, 25u32);
        let a3084 = f578(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3083, a3084)
    };
    let a3086 = f513(v0, 3u32);
    fixed_u32::bit_xor(a3085, a3086)
}

pub fn f581(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3087, _) = v0;
        h3087
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3088) = v0;
        h3088
    };
    let m3089 = v1;
    match m3089.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3090 = v4;
            match m3090.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f584(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f582(v0: Fn23, p3095: u64, p3096: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3094 = (p3095, p3096);
    {
        let mut m3094 = m3094;
        loop {
            let step = {
                let (v1, v2) = m3094;
                let m3091 = v1;
                if m3091 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3091 - 1;
                    let m3093 = {
                        let c3092: Fn23 = v0.clone();
                        c3092.apply(v2.clone())
                    };
                    match m3093 {
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
                Ok(next) => { m3094 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f585(v0)
}

pub fn f584(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3101 = {
            let a3099 = {
                let a3097 = v0.clone();
                length_list(a3097)
            };
            let a3100 = {
                let a3098 = v1.clone();
                length_list(a3098)
            };
            nat_add(a3099, a3100)?
        };
        let a3102 = 1u64;
        nat_add(a3101, a3102)?
    };
    Ok(f586({
        let (_, h3103) = {
            let (_, h3104) = {
                let (h3105, _) = f587(Fn24::F588, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3105
            };
            h3104
        };
        h3103
    }))
}

pub fn f585(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3106, _) = v0.clone();
        h3106
    };
    let v2: List<(Bytes, ())> = {
        let (h3107, _) = {
            let (_, h3108) = v0.clone();
            h3108
        };
        h3107
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3109) = {
            let (_, h3110) = v0;
            h3110
        };
        h3109
    };
    let m3111 = v1.clone();
    match m3111.uncons() {
        None => {
            let m3112 = v2;
            match m3112.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3113 = v2.clone();
            match m3113.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f589({
                        let (h3114, _) = v8.clone();
                        h3114
                    }, {
                        let (h3115, _) = v6.clone();
                        h3115
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

pub fn f586(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f401(Fn5::F590, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f587(v0: Fn24, p3120: u64, p3121: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3119 = (p3120, p3121);
    {
        let mut m3119 = m3119;
        loop {
            let step = {
                let (v1, v2) = m3119;
                let m3116 = v1;
                if m3116 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3116 - 1;
                    let m3118 = {
                        let c3117: Fn24 = v0.clone();
                        c3117.apply(v2.clone())
                    };
                    match m3118 {
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
                Ok(next) => { m3119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f591(v0)
}

pub fn f589(v0: Bytes, v1: Bytes) -> bool {
    let a3126 = {
        let a3122 = v0;
        let a3123 = v1;
        compare_bytes(a3122, a3123)
    };
    let a3127 = {
        let a3124 = Bytes::lit(&[]);
        let a3125 = Bytes::lit(&[0x01]);
        compare_bytes(a3124, a3125)
    };
    equal(a3126, a3127)
}

pub fn f590(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f591(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3128, _) = v0.clone();
        h3128
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3129, _) = {
            let (_, h3130) = v0.clone();
            h3130
        };
        h3129
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3131) = {
            let (_, h3132) = v0;
            h3132
        };
        h3131
    };
    let m3133 = v1.clone();
    match m3133.uncons() {
        None => {
            let m3134 = v2;
            match m3134.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3135 = v2.clone();
            match m3135.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f589({
                        let (h3136, _) = v8.clone();
                        h3136
                    }, {
                        let (h3137, _) = v6.clone();
                        h3137
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
