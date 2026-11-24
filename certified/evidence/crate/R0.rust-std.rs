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
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Bytes, Adt14),
    C1(Bytes),
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
    F55(u8, Bytes),
    F170(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F55(k0, k1) => f55(*k0, k1.clone(), p0),
            Fn0::F170(k0) => f170(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F148,
    F166,
    F234,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F148 => f148(p0, p1),
            Fn1::F166 => f166(p0, p1),
            Fn1::F234 => f234(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F199(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F199(k0) => f199(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F223(Bytes, u64, Bytes, u64),
    F316(Bytes),
    F317(u64, Bytes),
    F379(Bytes),
    F426(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F223(k0, k1, k2, k3) => f223(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F316(k0) => Ok(f316(k0.clone(), p0)),
            Fn3::F317(k0, k1) => f317(*k0, k1.clone(), p0),
            Fn3::F379(k0) => f379(k0.clone(), p0),
            Fn3::F426(k0) => f426(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F265(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt22)) -> R<(u64, Adt22)> {
        match self {
            Fn4::F265(k0, k1, k2) => f265(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F297,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn5::F297 => f297(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F300,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn6::F300 => f300(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F302,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn7::F302 => f302(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F305,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn8::F305 => f305(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F332(Bytes),
    F389(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F332(k0) => f332(k0.clone(), p0),
            Fn9::F389(k0, k1) => Ok(f389(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F334,
    F467,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F334 => f334(p0, p1),
            Fn10::F467 => f467(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F337,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F337 => f337(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F339,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F339 => f339(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F342,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F342 => f342(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F347,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn14::F347 => f347(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F352(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F352(k0) => f352(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F382(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F382(k0) => f382(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F393,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F393 => f393(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F395,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F395 => f395(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F397,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F397 => f397(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F405,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F405 => f405(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F418,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F418 => f418(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F445,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F445 => f445(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F447,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F447 => f447(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F449(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F449(k0) => f449(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F466,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F466 => f466(p0),
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
                                            if f36(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                f43(v1, v4)
                                            } else {
                                                if f36(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                    f44(v1, v4, false)
                                                } else {
                                                    if f36(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                        f44(v1, v4, true)
                                                    } else {
                                                        if f36(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                            f45(v1, v4, false)
                                                        } else {
                                                            if f36(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                f45(v1, v4, true)
                                                            } else {
                                                                if f36(v2.clone(), f8(Str::lit("admit"))) {
                                                                    f46(v1, v4)
                                                                } else {
                                                                    if f36(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                        f47(v1, v4)
                                                                    } else {
                                                                        if f36(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                            f48(v1, v4)
                                                                        } else {
                                                                            if f36(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                f49(v1, v4)
                                                                            } else {
                                                                                if f36(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                    f50(v1, v4)
                                                                                } else {
                                                                                    if f36(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                        f51(v1, v4)
                                                                                    } else {
                                                                                        if f36(v2, f8(Str::lit("propose-encoding"))) {
                                                                                            let m179 = v5;
                                                                                            match m179.uncons() {
                                                                                                None => {
                                                                                                    Ok(Adt11::C4(f35()))
                                                                                                }
                                                                                                Some((v6, v7)) => {
                                                                                                    f52(v1, v4, v6, v7)
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

pub fn f29(v0: u8) -> R<Str> {
    let m228 = {
        let a226 = f53();
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
        let (h230, _) = f54(Fn0::F55(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h230
    };
    let v4: u64 = {
        let (h231, _) = {
            let (_, h232) = v3.clone();
            h232
        };
        h231
    };
    Ok(f56(List::cons(f32(v0, v4, {
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
                        if f57(v1.clone(), 0u64, {
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
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | stage-mappings JSONL | stage-matrix JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f36(v0: Bytes, v1: Bytes) -> bool {
    let a251 = v0;
    let a252 = v1;
    equal(a251, a252)
}

pub fn f37(v0: Adt1) -> R<Adt11> {
    let m253 = f58(v0.clone(), f59());
    match m253.uncons() {
        None => {
            let m254 = f60(v0.clone(), f59())?;
            match m254.uncons() {
                None => {
                    let m257 = f61(match v0.clone() {
                        Adt1::C0(_, h255, _) => {
                            h255
                        }
                    }, f62(f63(), f64(match v0.clone() {
                        Adt1::C0(_, h256, _) => {
                            h256
                        }
                    }, false)));
                    match m257.uncons() {
                        None => {
                            let m259 = f58(v0.clone(), f64(match v0.clone() {
                                Adt1::C0(_, h258, _) => {
                                    h258
                                }
                            }, true));
                            match m259.uncons() {
                                None => {
                                    let m261 = f65(v0.clone(), f64(match v0.clone() {
                                        Adt1::C0(_, h260, _) => {
                                            h260
                                        }
                                    }, true))?;
                                    match m261 {
                                        None => {
                                            let v10: (bool, Bytes) = f66(Adt13::C0(Adt12::C0(f67(f68(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f69(v0.clone())?, f70(f71(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f67(f68(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f70(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f71(v0.clone(), f8(Str::lit("model/strata.json")))?, f72(f71(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f64(match v0 {
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
    if f73(f74(match v0.clone() {
        Adt1::C0(_, h266, _) => {
            h266
        }
    }, f75())) {
        Ok(Adt11::C1(List::cons(f75(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f76(f77(f75(), f78(match v0.clone() {
            Adt1::C0(_, h267, _) => {
                h267
            }
        }, f75())))?;
        let m268 = f58(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m268.uncons() {
            None => {
                let m269 = f79(v0.clone(), f80(f81(f68(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f82(v2, v1.clone()))?;
                match m269 {
                    None => {
                        f83(v0, v1)
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
    if f73(f84(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h270) => {
            h270
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f85(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f73(f84(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h271) => {
                h271
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f85(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f40(v0: Adt1) -> R<Adt11> {
    let m272 = f86(v0.clone());
    match m272.uncons() {
        None => {
            let m273 = f62(f58(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f87(), List::<Bytes>::nil())))), f88(v0.clone())?);
            match m273.uncons() {
                None => {
                    Ok(Adt11::C3(f89(f90(v0.clone())?, List::cons((f91(), f92()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f93()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f94()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f95()), List::cons((f8(Str::lit("docs/catalog.md")), f96(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f97(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f98(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f99()), List::<(Bytes, Bytes)>::nil()))))))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m274 = f86(v0.clone());
    match m274.uncons() {
        None => {
            let m275 = f88(v0.clone())?;
            match m275.uncons() {
                None => {
                    let v5: Adt15 = f100(v0.clone())?;
                    let v6: List<Adt14> = f71(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f62(f101(match v5.clone() {
                        Adt15::C0(h276, _, _) => {
                            h276
                        }
                    }, f102(v6.clone())), f103(match v5.clone() {
                        Adt15::C0(h277, _, _) => {
                            h277
                        }
                    }, f104(v5.clone(), v6, f105(v5, f106(v0.clone(), f107(v0)?)?))?)))))
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
    let m278 = f58(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m278.uncons() {
        None => {
            let v4: Bytes = f68(v0.clone(), v1);
            let v5: List<List<Bytes>> = f108(v4.clone())?;
            if f73(f109(f110(v5.clone()), f111())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f112(Str::lit("id"), v6.clone())?, List::cons(f112(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f112(Str::lit("artifact"), f113())?, List::cons(f112(Str::lit("checksum"), f114(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f113(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f115(f68(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f116(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f117(f118(f119(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f43(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m279 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m279.uncons() {
        None => {
            let m280 = f120(f81(f68(v0, v1))?)?;
            match m280 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f117(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f44(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m281 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m281.uncons() {
        None => {
            let v5: Bytes = f121(if v2 {
                f122()
            } else {
                f123()
            }, f124(v1.clone())?);
            if f73(f125(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m282 = f126(f81(f68(v0.clone(), v1.clone()))?, v2)?;
                match m282 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a284 = {
                            let a283 = v6;
                            length_list(a283)
                        };
                        let a285 = 0u64;
                        if nat_eq(a284, a285) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f68(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f45(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m286 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m286.uncons() {
        None => {
            let v5: List<Bytes> = f81(f68(v0.clone(), v1.clone()))?;
            if f73(f125(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f127(v5)?
                } else {
                    f128(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f121(f129(), f124(v1.clone())?), f68(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f121(f129(), f124(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f130()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f46(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m287 = f58(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil())))));
    match m287.uncons() {
        None => {
            let v4: List<Bytes> = f81(f68(v0.clone(), v1))?;
            let m288 = f131(v4.clone(), f8(Str::lit("requirement")))?;
            match m288 {
                None => {
                    Ok(Adt11::C4(f130()))
                }
                Some(v5) => {
                    let m289 = f131(v4, f8(Str::lit("admission")))?;
                    match m289 {
                        None => {
                            Ok(Adt11::C4(f130()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f71(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f71(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f62(f70(v5.clone(), f8(Str::lit("id"))), f70(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f76(f62(f70(v7.clone(), f8(Str::lit("id"))), f70(v8.clone(), f8(Str::lit("construct")))))?;
                            let a291 = {
                                let a290 = v9.clone();
                                length_list(a290)
                            };
                            let a292 = 0u64;
                            if nat_eq(a291, a292) {
                                Ok(Adt11::C4(f130()))
                            } else {
                                if f73({
                                    let a295 = {
                                        let a293 = f76(f62(v9.clone(), v10.clone()))?;
                                        length_list(a293)
                                    };
                                    let a296 = {
                                        let a294 = f62(v9.clone(), v10);
                                        length_list(a294)
                                    };
                                    nat_eq(a295, a296)
                                }) {
                                    Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                } else {
                                    Ok(Adt11::C3(List::cons(f132(f8(Str::lit("model/requirements.json")), f133(v7, v5))?, List::cons(f132(f8(Str::lit("model/admissions.json")), f133(v8, v6))?, List::cons(f116(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f117(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f47(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m297 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m297.uncons() {
        None => {
            let m298 = f134(f81(f68(v0, v1))?)?;
            match m298 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f117(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f48(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m299 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m299.uncons() {
        None => {
            let m300 = f135(v0.clone(), v1.clone())?;
            match m300 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m301 = f136(v4.clone(), f8(Str::lit("schema")));
                    match m301 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f137(v0, f138(v5.clone())?, v5, f139(v4.clone(), f8(Str::lit("name"))), f72(f140(f136(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
    let m302 = f58(v0.clone(), List::cons(v1.clone(), List::cons(f87(), List::<Bytes>::nil())));
    match m302.uncons() {
        None => {
            let m303 = f141(f81(f68(v0.clone(), v1))?)?;
            match m303 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f71(v0, f87())?;
                    let v6: List<Bytes> = f70(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f76(f70(v5.clone(), f8(Str::lit("at"))))?;
                    let a305 = {
                        let a304 = v6.clone();
                        length_list(a304)
                    };
                    let a306 = 0u64;
                    if if nat_eq(a305, a306) {
                        true
                    } else {
                        f73({
                            let a309 = {
                                let a307 = f76(f62(v6.clone(), v7.clone()))?;
                                length_list(a307)
                            };
                            let a310 = {
                                let a308 = f62(v6.clone(), v7);
                                length_list(a308)
                            };
                            nat_eq(a309, a310)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f132(f87(), f133(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f117(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m311 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m311.uncons() {
        None => {
            let m312 = f135(v0, v1)?;
            match m312 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f142(f140(Some(v4))) {
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

pub fn f51(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m313 = f58(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m313.uncons() {
        None => {
            let m314 = f135(v0.clone(), v1.clone())?;
            match m314 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f73(f36(f139(v4.clone(), f8(Str::lit("schema"))), f143())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f139(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m315 = f136(v4.clone(), f8(Str::lit("source")));
                            match m315 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f144(f136(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f121(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f121(v9.clone(), f8(Str::lit("binding.json"))), f68(v0.clone(), v1));
                        let m316 = v8;
                        match m316 {
                            None => {
                                f145(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f73(f36(f139(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f145(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f121(v9, f124(f139(v7.clone(), f8(Str::lit("url"))))?);
                                    if f73(f84(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h317) => {
                                            h317
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f139(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m318 = f58(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m318.uncons() {
                                            None => {
                                                if f36(f114(f68(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f145(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f52(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m319 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m319.uncons() {
        None => {
            let m320 = f135(v0.clone(), v1.clone())?;
            match m320 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f137(v0.clone(), f68(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f53() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f54(v0: Fn0, p325: u64, p326: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m324 = (p325, p326);
    {
        let mut m324 = m324;
        loop {
            let step = {
                let (v1, v2) = m324;
                let m321 = v1;
                if m321 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m321 - 1;
                    let m323 = {
                        let c322: Fn0 = v0.clone();
                        c322.apply(v2.clone())?
                    };
                    match m323 {
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
                Ok(next) => { m324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f146(v1, v0, v2)
}

pub fn f56(v0: List<Bytes>) -> List<Bytes> {
    f147(Fn1::F148, List::<Bytes>::nil(), v0)
}

pub fn f57(v0: Bytes, p382: u64, p383: u64) -> R<bool> {
    let m381 = (p382, p383);
    {
        let mut m381 = m381;
        loop {
            let step = {
                let (v1, v2) = m381;
                let m327 = v2;
                if m327 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m327 - 1;
                    let m330 = {
                        let a328 = v0.clone();
                        let a329 = v1;
                        index_bytes(a328, a329)
                    };
                    match m330 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a377 = {
                                let v5: u8 = v4;
                                let a375 = {
                                    let a351 = {
                                        let a339 = {
                                            let a333 = {
                                                let a331 = v5;
                                                let a332 = 128u8;
                                                fixed_u8::bit_and(a331, a332)
                                            };
                                            let a334 = 128u8;
                                            if equal(a333, a334) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a340 = {
                                            let a337 = {
                                                let a335 = v5;
                                                let a336 = 64u8;
                                                fixed_u8::bit_and(a335, a336)
                                            };
                                            let a338 = 64u8;
                                            if equal(a337, a338) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a339, a340)?
                                    };
                                    let a352 = {
                                        let a349 = {
                                            let a343 = {
                                                let a341 = v5;
                                                let a342 = 32u8;
                                                fixed_u8::bit_and(a341, a342)
                                            };
                                            let a344 = 32u8;
                                            if equal(a343, a344) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a350 = {
                                            let a347 = {
                                                let a345 = v5;
                                                let a346 = 16u8;
                                                fixed_u8::bit_and(a345, a346)
                                            };
                                            let a348 = 16u8;
                                            if equal(a347, a348) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a349, a350)?
                                    };
                                    nat_add(a351, a352)?
                                };
                                let a376 = {
                                    let a373 = {
                                        let a361 = {
                                            let a355 = {
                                                let a353 = v5;
                                                let a354 = 8u8;
                                                fixed_u8::bit_and(a353, a354)
                                            };
                                            let a356 = 8u8;
                                            if equal(a355, a356) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a362 = {
                                            let a359 = {
                                                let a357 = v5;
                                                let a358 = 4u8;
                                                fixed_u8::bit_and(a357, a358)
                                            };
                                            let a360 = 4u8;
                                            if equal(a359, a360) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a361, a362)?
                                    };
                                    let a374 = {
                                        let a371 = {
                                            let a365 = {
                                                let a363 = v5;
                                                let a364 = 2u8;
                                                fixed_u8::bit_and(a363, a364)
                                            };
                                            let a366 = 2u8;
                                            if equal(a365, a366) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a372 = {
                                            let a369 = {
                                                let a367 = v5;
                                                let a368 = 1u8;
                                                fixed_u8::bit_and(a367, a368)
                                            };
                                            let a370 = 1u8;
                                            if equal(a369, a370) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a371, a372)?
                                    };
                                    nat_add(a373, a374)?
                                };
                                nat_add(a375, a376)?
                            };
                            let a378 = 127u64;
                            if nat_le(a377, a378) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a379 = v1;
                                    let a380 = 1u64;
                                    nat_add(a379, a380)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m384 = v1;
    match m384.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m386 = f149(match v0.clone() {
                Adt1::C0(h385, _, _) => {
                    h385
                }
            }, v2.clone());
            match m386 {
                None => {
                    List::cons(v2, f58(v0, v3))
                }
                Some(_) => {
                    f58(v0, v3)
                }
            }
        }
    }
}

pub fn f59() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f113(), List::<Bytes>::nil())))))))
}

pub fn f60(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m387 = v1;
    match m387.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f150(v0.clone(), v2.clone()) {
                f125(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m388 = f135(v0.clone(), v2.clone())?;
                match m388 {
                    None => {
                        Ok(List::cons(v2, f60(v0, v3)?))
                    }
                    Some(_) => {
                        f60(v0, v3)
                    }
                }
            } else {
                f60(v0, v3)
            }
        }
    }
}

pub fn f61(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m389 = v1;
    match m389.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f74(v0.clone(), v2.clone()) {
                f61(v0, v3)
            } else {
                List::cons(v2, f61(v0, v3))
            }
        }
    }
}

pub fn f62(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m390 = v0;
    match m390.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f62(v3, v1))
        }
    }
}

pub fn f63() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f64(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m391 = v0;
    match m391.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f62(f151({
                let (h392, _) = v2;
                h392
            }, {
                let (_, h393) = v2;
                h393
            }, v1), f64(v3, v1))
        }
    }
}

pub fn f65(v0: Adt1, p397: List<Bytes>) -> R<Option<Bytes>> {
    let m396 = p397;
    {
        let mut m396 = m396;
        loop {
            let step = {
                let v1 = m396;
                let m394 = v1;
                match m394.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m395 = f152(f68(v0.clone(), v2.clone()), f153())?;
                        match m395 {
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
                Ok(next) => { m396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h398, _, _, _, _) => {
            h398
        }
    };
    let v2: Adt16 = f154(v1.clone())?;
    let v3: List<Bytes> = f155(v1.clone());
    let v4: List<Bytes> = f70(match v0.clone() {
        Adt13::C0(_, _, h399, _, _) => {
            h399
        }
    }, f8(Str::lit("path")));
    let m401 = f156(v2, List::<u64>::nil(), f157(v1.clone(), match v0.clone() {
        Adt13::C0(_, h400, _, _, _) => {
            h400
        }
    }, 0u64)?);
    match m401 {
        None => {
            if f159(f160(match v0.clone() {
                Adt13::C0(_, _, _, _, h403) => {
                    h403
                }
            }, f62(match v0.clone() {
                Adt13::C0(_, _, _, _, h404) => {
                    h404
                }
            }, v4.clone()))?, f160(v4.clone(), f62(match v0.clone() {
                Adt13::C0(_, _, _, _, h405) => {
                    h405
                }
            }, v4.clone()))?, f161(match v0.clone() {
                Adt13::C0(_, _, h406, _, _) => {
                    h406
                }
            })) {
                let m410 = f162(f160(match v0.clone() {
                    Adt13::C0(_, _, _, h407, _) => {
                        h407
                    }
                }, f62(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h408, _) => {
                        h408
                    }
                }))?, f160(v3.clone(), f62(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h409, _) => {
                        h409
                    }
                }))?);
                match m410 {
                    None => {
                        let m413 = f163(f70(match v0 {
                            Adt13::C0(_, h412, _, _, _) => {
                                h412
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m413 {
                            None => {
                                let m414 = f163(v4, List::<Bytes>::nil());
                                match m414 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f117(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f158(f62(v3, match v0 {
                            Adt13::C0(_, _, _, h411, _) => {
                                h411
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f158(match v1 {
                Adt12::C0(_, _, _, _, h402) => {
                    h402
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f67(v0: Bytes) -> R<List<Bytes>> {
    f164(f81(v0)?, f8(Str::lit("id = \"")))
}

pub fn f68(v0: Adt1, v1: Bytes) -> Bytes {
    let m416 = f149(match v0 {
        Adt1::C0(h415, _, _) => {
            h415
        }
    }, v1);
    match m416 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m417 = v2;
            match m417 {
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

pub fn f69(v0: Adt1) -> R<List<Bytes>> {
    let m418 = f108(f68(v0, f113()))?;
    match m418.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f118(v2))
        }
    }
}

pub fn f70(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m419 = v0;
    match m419.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f139(v2, v1.clone()), f70(v3, v1))
        }
    }
}

pub fn f71(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f140(f135(v0, v1)?))
}

pub fn f72(v0: List<Adt14>) -> List<Bytes> {
    let m420 = v0;
    match m420.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m421 = f144(Some(v1));
            match m421 {
                None => {
                    f72(v2)
                }
                Some(v3) => {
                    List::cons(v3, f72(v2))
                }
            }
        }
    }
}

pub fn f73(v0: bool) -> bool {
    !v0
}

pub fn f74(p425: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m424 = p425;
    {
        let mut m424 = m424;
        loop {
            let step = {
                let v0 = m424;
                let m422 = v0;
                match m422.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h423, _) = v2;
                            h423
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m424 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f76(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f165(f147(Fn1::F166, List::<Bytes>::nil(), f167(v0)?)))
}

pub fn f77(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m426 = v1;
    match m426.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f125(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f121(v0.clone(), v2), f77(v0, v3))
            } else {
                f77(v0, v3)
            }
        }
    }
}

pub fn f78(p431: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m430 = p431;
    {
        let mut m430 = m430;
        loop {
            let step = {
                let v0 = m430;
                let m427 = v0;
                match m427.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h428, _) = v2;
                            h428
                        }, v1.clone()) {
                            let (_, h429) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h429)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m430 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: Adt1, v1: List<List<Bytes>>, p434: List<Bytes>) -> R<Option<Bytes>> {
    let m433 = p434;
    {
        let mut m433 = m433;
        loop {
            let step = {
                let v2 = m433;
                let m432 = v2;
                match m432.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f168(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
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

pub fn f80(p437: List<Bytes>, p438: List<Bytes>, p439: List<List<Bytes>>) -> List<List<Bytes>> {
    let m436 = (p437, (p438, p439));
    {
        let mut m436 = m436;
        loop {
            let step = {
                let (v0, (v1, v2)) = m436;
                let m435 = v0;
                match m435.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f169(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f36(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f169(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
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

pub fn f81(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a440 = v0.clone();
        length_bytes(a440)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h441, _) = f54(Fn0::F170(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h441
    };
    let v3: u64 = {
        let (h442, _) = {
            let (_, h443) = v2.clone();
            h443
        };
        h442
    };
    let v4: List<Bytes> = {
        let (_, h444) = {
            let (_, h445) = v2;
            h445
        };
        h444
    };
    let a446 = v3;
    let a447 = v1;
    if nat_lt(a446, a447) {
        Ok(f56(List::cons(f32(v0, v3, {
            let a448 = v1;
            let a449 = v3;
            nat_sub(a448, a449)
        }), v4)))
    } else {
        Ok(f56(v4))
    }
}

pub fn f82(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m450 = v0;
    match m450.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84(v2.clone(), v1.clone()) {
                f82(v3, v1)
            } else {
                List::cons(v2, f82(v3, v1))
            }
        }
    }
}

pub fn f83(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f73(f74(match v0.clone() {
        Adt1::C0(_, h451, _) => {
            h451
        }
    }, f171())) {
        Ok(Adt11::C1(List::cons(f171(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f82(f76(f77(f171(), f78(match v0.clone() {
            Adt1::C0(_, h452, _) => {
                h452
            }
        }, f171())))?, v1.clone());
        let m453 = f58(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m453.uncons() {
            None => {
                let m454 = f172(v0.clone(), v2, f70(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m454 {
                    None => {
                        f173(v0, v1)
                    }
                    Some(v5) => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("admit-encoding ")), List::cons(v5, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            }
            Some((v3, v4)) => {
                Ok(Adt11::C0(List::cons(v3, v4)))
            }
        }
    }
}

pub fn f84(v0: Bytes, p457: List<Bytes>) -> bool {
    let m456 = p457;
    {
        let mut m456 = m456;
        loop {
            let step = {
                let v1 = m456;
                let m455 = v1;
                match m455.uncons() {
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
                Ok(next) => { m456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f86(v0: Adt1) -> List<Bytes> {
    if f74(match v0.clone() {
        Adt1::C0(_, h458, _) => {
            h458
        }
    }, f123()) {
        if f74(match v0 {
            Adt1::C0(_, h459, _) => {
                h459
            }
        }, f122()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f122(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f123(), List::<Bytes>::nil())
    }
}

pub fn f87() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f88(v0: Adt1) -> R<List<Bytes>> {
    Ok(f58(v0.clone(), List::cons(f113(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f107(v0)?)))))
}

pub fn f89(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m460 = v0;
    match m460.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f89(v3, v1))
        }
    }
}

pub fn f90(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m461 = f71(v0.clone(), f87())?;
    match m461.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f174(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f91() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f92() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f93() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**', 'model/encodings/**', 'model/queue/**', 'model/authorities.toml']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | grep '^  name: ' | tail -n 1; done < target/hunks.txt\u{a}              git diff -U0 -- studio/src | grep '^[+]  name: ' | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f94() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src and\u{a}# README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one.\u{a}name: refresh\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  refresh:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply each change to the new main, export it again, and push it to its own pull request\u{a}        if: steps.stale.outputs.stale != ''\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ steps.stale.outputs.stale }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again either way.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /'\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}"))
}

pub fn f95() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f96(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f100(v0.clone())?;
    let v2: List<Adt14> = f71(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f105(v1.clone(), f106(v0.clone(), f107(v0.clone())?)?);
    let v4: List<List<Bytes>> = f119(f108(f68(v0, f113()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f175(v4.clone(), f118(v4), f176(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f104(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f97(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f177(f100(v0.clone())?, f106(v0.clone(), f107(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f98(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f178(f71(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f179(f71(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f99() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f100(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f69(v0.clone())?, f67(f68(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f70(f71(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f101(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m462 = v0;
    match m462.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84(v2.clone(), v1.clone()) {
                f101(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f101(v3, v1))
            }
        }
    }
}

pub fn f102(v0: List<Adt14>) -> List<Bytes> {
    let m463 = v0;
    match m463.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m464 = f136(v1, f8(Str::lit("std")));
            match m464 {
                None => {
                    f102(v2)
                }
                Some(v3) => {
                    List::cons(f139(v3, f8(Str::lit("row"))), f102(v2))
                }
            }
        }
    }
}

pub fn f103(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m465 = v0;
    match m465.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84(v2.clone(), v1.clone()) {
                f103(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f103(v3, v1))
            }
        }
    }
}

pub fn f104(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f89(f180(v2.clone()), f181(f176(v0, v1.clone(), v2)?));
    f182(f76(f183(v1))?, v3.clone(), {
        let a467 = {
            let a466 = v3;
            length_list(a466)
        };
        let a468 = 1u64;
        nat_add(a467, a468)?
    })
}

pub fn f105(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m469 = v1;
    match m469.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f184(v2.clone(), f8(Str::lit("subject"))) {
                f185(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f105(v0, v3))
            } else {
                f105(v0, v3)
            }
        }
    }
}

pub fn f106(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m470 = v1;
    match m470.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m471 = f126(f81(f68(v0.clone(), v2.clone()))?, f186(v2, f122()))?;
            match m471 {
                None => {
                    f106(v0, v3)
                }
                Some(v4) => {
                    Ok(f133(v4, f106(v0, v3)?))
                }
            }
        }
    }
}

pub fn f107(v0: Adt1) -> R<List<Bytes>> {
    Ok(f62(f76(f187(f123(), f78(match v0.clone() {
        Adt1::C0(_, h472, _) => {
            h472
        }
    }, f123())))?, f76(f187(f122(), f78(match v0 {
        Adt1::C0(_, h473, _) => {
            h473
        }
    }, f122())))?))
}

pub fn f108(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f188(v0.clone(), 0u64, {
        let a474 = v0;
        length_bytes(a474)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a477 = {
            let a476 = match v1.clone() {
                Adt17::C0(_, _, h475, _, _) => {
                    h475
                }
            };
            length_bytes(a476)
        };
        let a478 = 0u64;
        if if nat_eq(a477, a478) {
            let m480 = match v1.clone() {
                Adt17::C0(_, _, _, h479, _) => {
                    h479
                }
            };
            match m480.uncons() {
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
            f189(v1)
        }
    };
    Ok(f190(match v4 {
        Adt17::C0(_, _, _, _, h481) => {
            h481
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f109(p486: List<Bytes>, p487: List<Bytes>) -> bool {
    let m485 = (p486, p487);
    {
        let mut m485 = m485;
        loop {
            let step = {
                let (v0, v1) = m485;
                let m482 = v0;
                match m482.uncons() {
                    None => {
                        let m483 = v1;
                        match m483.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m484 = v1;
                        match m484.uncons() {
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
                Ok(next) => { m485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: List<List<Bytes>>) -> List<Bytes> {
    let m488 = v0;
    match m488.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f111() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f112(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f191(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f113() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f114(v0: Bytes) -> R<Bytes> {
    Ok(f192(f193(v0)?))
}

pub fn f115(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f194(f80(f81(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f195(f196(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f116(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f138(Adt14::C3(std::rc::Rc::new(f197(f76(f62(v1, f72(f71(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f117(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m489 = f198(Fn2::F199(v1), None::<Bytes>, v0);
    match m489 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f118(v0: List<List<Bytes>>) -> List<Bytes> {
    let m490 = v0;
    match m490.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m491 = v1;
            match m491.uncons() {
                None => {
                    f118(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f118(v2))
                }
            }
        }
    }
}

pub fn f119(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m492 = v0;
    match m492.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f120(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m493 = v0;
    match m493.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f200(v1)?;
            let a495 = {
                let a494 = v3.clone();
                length_bytes(a494)
            };
            let a496 = 0u64;
            if nat_eq(a495, a496) {
                f120(v2)
            } else {
                let m497 = f201(v3.clone())?;
                match m497 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f139(v4.clone(), f8(Str::lit("id")));
                        if if f36(f139(v4, f8(Str::lit("schema"))), f143()) {
                            f73({
                                let a499 = {
                                    let a498 = v5.clone();
                                    length_bytes(a498)
                                };
                                let a500 = 0u64;
                                nat_eq(a499, a500)
                            })
                        } else {
                            false
                        } {
                            let m501 = f120(v2)?;
                            match m501 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f202(v5), {
                                        let a502 = v3;
                                        let a503 = Bytes::lit(&[0x0a]);
                                        append_bytes(a502, a503)
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

pub fn f121(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f122() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f123() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f124(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f125(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a504 = v0.clone();
        length_bytes(a504)
    };
    let v3: u64 = {
        let a505 = v1.clone();
        length_bytes(a505)
    };
    let a506 = v3;
    let a507 = v2;
    if nat_le(a506, a507) {
        let m513 = {
            let a510 = v0;
            let a511 = {
                let a508 = v2;
                let a509 = v3;
                nat_sub(a508, a509)
            };
            let a512 = v3;
            slice_bytes(a510, a511, a512)
        };
        match m513 {
            None => {
                false
            }
            Some(v4) => {
                let a514 = v4;
                let a515 = v1;
                equal(a514, a515)
            }
        }
    } else {
        false
    }
}

pub fn f126(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m516 = v0;
    match m516.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f200(v2)?;
            let a518 = {
                let a517 = v4.clone();
                length_bytes(a517)
            };
            let a519 = 0u64;
            if nat_eq(a518, a519) {
                f126(v3, v1)
            } else {
                let m520 = f201(v4)?;
                match m520 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f203(v5.clone())
                        } else {
                            f204(v5.clone())
                        } {
                            let m521 = f126(v3, v1)?;
                            match m521 {
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

pub fn f127(v0: List<Bytes>) -> R<bool> {
    if f205(f141(v0.clone())?) {
        Ok(f73({
            let a523 = {
                let a522 = f206(f141(v0)?);
                length_list(a522)
            };
            let a524 = 0u64;
            nat_eq(a523, a524)
        }))
    } else {
        Ok(false)
    }
}

pub fn f128(v0: List<Bytes>) -> R<bool> {
    if f205(f131(v0.clone(), f8(Str::lit("requirement")))?) {
        if f205(f131(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f73({
                let a526 = {
                    let a525 = f133(f206(f131(v0.clone(), f8(Str::lit("requirement")))?), f206(f131(v0, f8(Str::lit("admission")))?));
                    length_list(a525)
                };
                let a527 = 0u64;
                nat_eq(a526, a527)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f129() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f130() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f131(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m528 = v0;
    match m528.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f200(v2)?;
            let a530 = {
                let a529 = v4.clone();
                length_bytes(a529)
            };
            let a531 = 0u64;
            if nat_eq(a530, a531) {
                f131(v3, v1)
            } else {
                let m532 = f201(v4)?;
                match m532 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f207(v5.clone());
                        let a534 = {
                            let a533 = v6.clone();
                            length_bytes(a533)
                        };
                        let a535 = 0u64;
                        if nat_eq(a534, a535) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m536 = f131(v3, v1.clone())?;
                            match m536 {
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

pub fn f132(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f138(Adt14::C3(std::rc::Rc::new(f208(v1))))?))
}

pub fn f133(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m537 = v0;
    match m537.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f133(v3, v1))
        }
    }
}

pub fn f134(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m538 = v0;
    match m538.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f200(v1)?;
            let a540 = {
                let a539 = v3.clone();
                length_bytes(a539)
            };
            let a541 = 0u64;
            if nat_eq(a540, a541) {
                f134(v2)
            } else {
                let m542 = f201(v3.clone())?;
                match m542 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f209(v4.clone()) {
                            let m543 = f134(v2)?;
                            match m543 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f171(), List::cons(f8(Str::lit("/")), List::cons(f210(f139(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a544 = v3;
                                        let a545 = Bytes::lit(&[0x0a]);
                                        append_bytes(a544, a545)
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

pub fn f135(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f201(f68(v0, v1))
}

pub fn f136(p551: Adt14, v1: Bytes) -> Option<Adt14> {
    let m550 = p551;
    {
        let mut m550 = m550;
        loop {
            let step = {
                let v0 = m550;
                let m546 = v0;
                match m546 {
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
                    Adt14::C4(r547) => {
                        let v5 = (*r547).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r548, r549) => {
                        let v3 = (*r548).clone();
                        let v4 = (*r549).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
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

pub fn f137(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m552 = f58(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m552.uncons() {
        None => {
            let m553 = f211(v4);
            match m553 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a554 = f8(Str::lit("Catalog."));
                        let a555 = f210(v3.clone())?;
                        append_bytes(a554, a555)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f210(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m556 = f212(v2, v9.clone(), f210(v3.clone())?)?;
                    match m556 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f210(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f138(Adt14::C3(std::rc::Rc::new(f208(f213(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f138(Adt14::C3(std::rc::Rc::new(f208(f213(f71(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a560 = f214(f68(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a558 = {
                                        let a557 = v10;
                                        length_bytes(a557)
                                    };
                                    let a559 = 4u64;
                                    nat_sub(a558, a559)
                                }))?;
                                let a561 = Bytes::lit(&[0x0a]);
                                append_bytes(a560, a561)
                            }), List::cons(f116(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f138(v0: Adt14) -> R<Bytes> {
    let a562 = f215(v0)?;
    let a563 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a562, a563))
}

pub fn f139(v0: Adt14, v1: Bytes) -> Bytes {
    let m564 = f144(f136(v0, v1));
    match m564 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f140(v0: Option<Adt14>) -> List<Adt14> {
    let m565 = v0;
    match m565 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f216(v1)
        }
    }
}

pub fn f141(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m566 = v0;
    match m566.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f200(v1)?;
            let a568 = {
                let a567 = v3.clone();
                length_bytes(a567)
            };
            let a569 = 0u64;
            if nat_eq(a568, a569) {
                f141(v2)
            } else {
                let m570 = f201(v3)?;
                match m570 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f184(v4.clone(), f8(Str::lit("agent"))) {
                            if f184(v4.clone(), f8(Str::lit("at"))) {
                                if f184(v4.clone(), f8(Str::lit("quote"))) {
                                    f217(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m571 = f141(v2)?;
                            match m571 {
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

pub fn f142(p574: List<Adt14>) -> bool {
    let m573 = p574;
    {
        let mut m573 = m573;
        loop {
            let step = {
                let v0 = m573;
                let m572 = v0;
                match m572.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f139(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f84(f8(Str::lit("studio")), f70(f218(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f144(v0: Option<Adt14>) -> Option<Bytes> {
    let m575 = v0;
    match m575 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m576 = v1;
            match m576 {
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

pub fn f145(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f115(f68(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f219(v1, v4)?)?), List::cons(f116(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f146(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h577, _) = v2.clone();
        h577
    };
    let v4: u64 = {
        let (h578, _) = {
            let (_, h579) = v2.clone();
            h579
        };
        h578
    };
    let v5: List<Bytes> = {
        let (_, h580) = {
            let (_, h581) = v2;
            h581
        };
        h580
    };
    let m582 = f220(v0.clone(), v3);
    match m582 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a583 = v6;
            let a584 = v1;
            if equal(a583, a584) {
                Ok(Some(({
                    let a585 = v3;
                    let a586 = 1u64;
                    nat_add(a585, a586)?
                }, ({
                    let a587 = v3;
                    let a588 = 1u64;
                    nat_add(a587, a588)?
                }, List::cons(f32(v0, v4, {
                    let a589 = v3;
                    let a590 = v4;
                    nat_sub(a589, a590)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a591 = v3;
                    let a592 = 1u64;
                    nat_add(a591, a592)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f147(v0: Fn1, p596: List<Bytes>, p597: List<Bytes>) -> List<Bytes> {
    let m595 = (p596, p597);
    {
        let mut m595 = m595;
        loop {
            let step = {
                let (v1, v2) = m595;
                let m593 = v2;
                match m593.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c594: Fn1 = v0.clone();
                            c594.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m595 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f149(p602: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m601 = p602;
    {
        let mut m601 = m601;
        loop {
            let step = {
                let v0 = m601;
                let m598 = v0;
                match m598.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h599, _) = v2;
                            h599
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h600) = v2;
                                h600
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f150(v0: Adt1, v1: Bytes) -> bool {
    let m604 = f149(match v0 {
        Adt1::C0(h603, _, _) => {
            h603
        }
    }, v1);
    match m604 {
        None => {
            false
        }
        Some(v2) => {
            let m605 = v2;
            match m605 {
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

pub fn f151(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m606 = v1;
    match m606.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f221(v3.clone())
            } else {
                f73(f221(v3.clone()))
            } {
                List::cons(f121(v0.clone(), v3), f151(v0, v4, v2))
            } else {
                f151(v0, v4, v2)
            }
        }
    }
}

pub fn f152(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a607 = v0.clone();
        length_bytes(a607)
    };
    let v3: u64 = {
        let a608 = v1.clone();
        length_bytes(a608)
    };
    let v4: u64 = {
        let (h609, _) = f222(Fn3::F223(v1.clone(), v3, v0.clone(), v2), {
            let a610 = v2;
            let a611 = 1u64;
            nat_add(a610, a611)?
        }, 0u64)?;
        h609
    };
    if f224(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f153() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f154(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f225(match v0.clone() {
        Adt12::C0(h612, _, _, _, _) => {
            h612
        }
    }, 0u64)?, f225(match v0.clone() {
        Adt12::C0(_, h613, _, _, _) => {
            h613
        }
    }, 0u64)?, f225(match v0.clone() {
        Adt12::C0(_, _, h614, _, _) => {
            h614
        }
    }, 0u64)?, f225(match v0 {
        Adt12::C0(_, _, _, h615, _) => {
            h615
        }
    }, 0u64)?))
}

pub fn f155(v0: Adt12) -> List<Bytes> {
    f62(match v0.clone() {
        Adt12::C0(h616, _, _, _, _) => {
            h616
        }
    }, f62(match v0.clone() {
        Adt12::C0(_, _, h617, _, _) => {
            h617
        }
    }, f62(match v0.clone() {
        Adt12::C0(_, _, _, h618, _) => {
            h618
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h619) => {
            h619
        }
    })))
}

pub fn f156(v0: Adt16, p625: List<u64>, p626: List<Adt18>) -> Option<u64> {
    let m624 = (p625, p626);
    {
        let mut m624 = m624;
        loop {
            let step = {
                let (v1, v2) = m624;
                let m620 = v2;
                match m620.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f226(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h621) => {
                                h621
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h622, _) => {
                                    h622
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h623, _) => {
                                    h623
                                }
                            }))
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

pub fn f157(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m627 = v1;
    match m627.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f227(v0.clone(), v3)?), f157(v0, v4, {
                let a628 = v2;
                let a629 = 1u64;
                nat_add(a628, a629)?
            })?))
        }
    }
}

pub fn f158(p633: List<Bytes>, p634: u64) -> Bytes {
    let m632 = (p633, p634);
    {
        let mut m632 = m632;
        loop {
            let step = {
                let (v0, v1) = m632;
                let m630 = v0;
                match m630.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m631 = v1;
                        if m631 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m631 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
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

pub fn f159(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f228(v0.clone(), v1.clone()) {
        if f228(v1, v0) {
            f229(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f160(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m635 = v0;
    match m635.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f230(v2, v1.clone())?, f160(v3, v1)?))
        }
    }
}

pub fn f161(v0: List<Adt14>) -> List<u64> {
    let m636 = v0;
    match m636.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f231(v1), f161(v2))
        }
    }
}

pub fn f162(p639: List<u64>, v1: List<u64>) -> Option<u64> {
    let m638 = p639;
    {
        let mut m638 = m638;
        loop {
            let step = {
                let v0 = m638;
                let m637 = v0;
                match m637.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f232(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m638 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(p642: List<Bytes>, p643: List<Bytes>) -> Option<Bytes> {
    let m641 = (p642, p643);
    {
        let mut m641 = m641;
        loop {
            let step = {
                let (v0, v1) = m641;
                let m640 = v0;
                match m640.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f84(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m644 = v0;
    match m644.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m645 = f233(v2, v1.clone())?;
            match m645 {
                None => {
                    f164(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f164(v3, v1)?))
                }
            }
        }
    }
}

pub fn f165(v0: List<Bytes>) -> List<Bytes> {
    f147(Fn1::F234, List::<Bytes>::nil(), v0)
}

pub fn f166(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f235(v0, v1)
}

pub fn f167(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f236(f237(f238(v0))?))
}

pub fn f168(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m646 = f135(v0, v2)?;
    match m646 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f239(v1, f139(v3.clone(), f8(Str::lit("id"))))?;
            let m647 = v4.clone();
            match m647.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m649 = f144(f136({
                        let m648 = f136(v3.clone(), f8(Str::lit("source")));
                        match m648 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m649 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f73(f84(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f191(f240(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f169(p652: List<Bytes>, p653: List<Bytes>) -> List<Bytes> {
    let m651 = (p652, p653);
    {
        let mut m651 = m651;
        loop {
            let step = {
                let (v0, v1) = m651;
                let m650 = v0;
                match m650.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f170(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f241(v0, v1)
}

pub fn f171() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f172(v0: Adt1, p657: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m656 = p657;
    {
        let mut m656 = m656;
        loop {
            let step = {
                let v1 = m656;
                let m654 = v1;
                match m654.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f84(f242(f139({
                            let m655 = f135(v0.clone(), v3.clone())?;
                            match m655 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v5) => {
                                    v5
                                }
                            }
                        }, f8(Str::lit("name"))))?, v2.clone()) {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f173(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f73(f74(match v0.clone() {
        Adt1::C0(_, h658, _) => {
            h658
        }
    }, f129())) {
        Ok(Adt11::C1(List::cons(f129(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f82(f76(f243(f78(match v0.clone() {
            Adt1::C0(_, h659, _) => {
                h659
            }
        }, f129())))?, v1);
        let m660 = f58(v0.clone(), List::cons(f87(), List::cons(f113(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m660.uncons() {
            None => {
                let v5: (bool, Bytes) = f244(v0.clone(), v2)?;
                let (h661, _) = v5;
                if h661 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h662) = v5;
                        h662
                    }))
                } else {
                    let m663 = f86(v0.clone());
                    match m663.uncons() {
                        None => {
                            let m664 = f88(v0.clone())?;
                            match m664.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h665) = v5;
                                        h665
                                    }, List::cons(f7(f76(f245(f100(v0.clone())?, f106(v0.clone(), f107(v0.clone())?)?))?), List::cons(f246(v0)?, List::<Bytes>::nil()))))))
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

pub fn f174(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f247(f71(v0, f87())?)?)))
}

pub fn f175(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt21>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m666 = v1;
    match m666.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f62(f248(v5.clone(), v2.clone()), f249(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f250(v0.clone(), v5.clone()), List::cons(if f84(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a668 = {
                    let a667 = v7.clone();
                    length_list(a667)
                };
                let a669 = 0u64;
                if nat_eq(a668, a669) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f117(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f175(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f176(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt21>> {
    let v3: List<(Bytes, Bytes)> = f251(v1);
    let v4: List<(Bytes, Bytes)> = f252(v0, v2);
    f253(v3.clone(), f89(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f177(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m670 = v1;
    match m670.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f184(v2.clone(), f8(Str::lit("domain"))) {
                f254(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f139(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f139(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m671 = f136(v2.clone(), f8(Str::lit("role")));
                    match m671 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f139(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f139(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f139(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f255(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m672 = f136(v2.clone(), f8(Str::lit("origin")));
                    match m672 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f139(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m673 = f136(v2, f8(Str::lit("evidence")));
                    match m673 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f139(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f177(v0, v3))
            } else {
                f177(v0, v3)
            }
        }
    }
}

pub fn f178(v0: List<Adt14>) -> List<Bytes> {
    let m674 = v0;
    match m674.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f139(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f139(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f139(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f178(v2))
        }
    }
}

pub fn f179(v0: List<Adt14>) -> List<Bytes> {
    let m675 = v0;
    match m675.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f139(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f256(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f179(v2))
        }
    }
}

pub fn f180(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m676 = v0;
    match m676.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f139(v1.clone(), f8(Str::lit("subject"))), f139(v1, f8(Str::lit("object")))), f180(v2))
        }
    }
}

pub fn f181(v0: List<Adt21>) -> List<(Bytes, Bytes)> {
    let m677 = v0;
    match m677.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt21::C0(h678, _, _) => {
                    h678
                }
            }, match v1 {
                Adt21::C0(_, _, h679) => {
                    h679
                }
            }), f181(v2))
        }
    }
}

pub fn f182(p686: List<Bytes>, v1: List<(Bytes, Bytes)>, p687: u64) -> R<List<Bytes>> {
    let m685 = (p686, p687);
    {
        let mut m685 = m685;
        loop {
            let step = {
                let (v0, v2) = m685;
                let m680 = v2;
                if m680 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m680 - 1;
                    let v4: List<Bytes> = f76(f62(v0.clone(), f257(v0.clone(), v1.clone())))?;
                    let a683 = {
                        let a681 = v4.clone();
                        length_list(a681)
                    };
                    let a684 = {
                        let a682 = v0.clone();
                        length_list(a682)
                    };
                    if nat_eq(a683, a684) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m685 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: List<Adt14>) -> List<Bytes> {
    let m688 = v0;
    match m688.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m689 = f136(v1, f8(Str::lit("std")));
            match m689 {
                None => {
                    f183(v2)
                }
                Some(v3) => {
                    List::cons(f139(v3.clone(), f8(Str::lit("authority"))), if f184(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f139(v3, f8(Str::lit("row"))), f183(v2))
                    } else {
                        f183(v2)
                    })
                }
            }
        }
    }
}

pub fn f184(v0: Adt14, v1: Bytes) -> bool {
    f73({
        let a691 = {
            let a690 = f139(v0, v1);
            length_bytes(a690)
        };
        let a692 = 0u64;
        nat_eq(a691, a692)
    })
}

pub fn f185(v0: Adt15, v1: Adt14) -> bool {
    if f258(v0.clone(), v1.clone(), f139(v1.clone(), f8(Str::lit("subject")))) {
        f258(v0, v1.clone(), f139(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f186(v0: Bytes, v1: Bytes) -> bool {
    let m697 = {
        let a694 = v0;
        let a695 = 0u64;
        let a696 = {
            let a693 = v1.clone();
            length_bytes(a693)
        };
        slice_bytes(a694, a695, a696)
    };
    match m697 {
        None => {
            false
        }
        Some(v2) => {
            let a698 = v2;
            let a699 = v1;
            equal(a698, a699)
        }
    }
}

pub fn f187(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m700 = v1;
    match m700.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f125(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f121(v0.clone(), v2), f187(v0, v3))
            } else {
                f187(v0, v3)
            }
        }
    }
}

pub fn f188(v0: Bytes, p758: u64, p759: u64, p760: Adt17) -> R<Adt17> {
    let m757 = (p758, (p759, p760));
    {
        let mut m757 = m757;
        loop {
            let step = {
                let (v1, (v2, v3)) = m757;
                let m701 = v2;
                if m701 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m701 - 1;
                    let v5: u64 = f259(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h702, _, _, _) => {
                            h702
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a703 = v1;
                            let a704 = 1u64;
                            nat_add(a703, a704)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h705, _, _) => {
                                h705
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h706, _) => {
                                h706
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h707) => {
                                h707
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h708, _, _, _, _) => {
                                h708
                            }
                        } {
                            let a709 = v5;
                            let a710 = 34u64;
                            if nat_eq(a709, a710) {
                                let a713 = f259(v0.clone(), {
                                    let a711 = v1;
                                    let a712 = 1u64;
                                    nat_add(a711, a712)?
                                })?;
                                let a714 = 34u64;
                                if nat_eq(a713, a714) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a715 = v1;
                                        let a716 = 1u64;
                                        nat_add(a715, a716)?
                                    }, (v4, Adt17::C0(true, true, f260(match v3.clone() {
                                        Adt17::C0(_, _, h717, _, _) => {
                                            h717
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h718, _) => {
                                            h718
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h719) => {
                                            h719
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a720 = v1;
                                        let a721 = 1u64;
                                        nat_add(a720, a721)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h722, _, _) => {
                                            h722
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h723, _) => {
                                            h723
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h724) => {
                                            h724
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a725 = v1;
                                    let a726 = 1u64;
                                    nat_add(a725, a726)?
                                }, (v4, Adt17::C0(true, false, f260(match v3.clone() {
                                    Adt17::C0(_, _, h727, _, _) => {
                                        h727
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h728, _) => {
                                        h728
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h729) => {
                                        h729
                                    }
                                }))))
                            }
                        } else {
                            let a730 = v5;
                            let a731 = 34u64;
                            if nat_eq(a730, a731) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a732 = v1;
                                    let a733 = 1u64;
                                    nat_add(a732, a733)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h734, _, _) => {
                                        h734
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h735, _) => {
                                        h735
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h736) => {
                                        h736
                                    }
                                }))))
                            } else {
                                let a737 = v5;
                                let a738 = 44u64;
                                if nat_eq(a737, a738) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a739 = v1;
                                        let a740 = 1u64;
                                        nat_add(a739, a740)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h741, _, _) => {
                                            h741
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h742, _) => {
                                            h742
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h743) => {
                                            h743
                                        }
                                    }))))
                                } else {
                                    let a744 = v5;
                                    let a745 = 13u64;
                                    if nat_eq(a744, a745) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a746 = v1;
                                            let a747 = 1u64;
                                            nat_add(a746, a747)?
                                        }, (v4, v3)))
                                    } else {
                                        let a748 = v5;
                                        let a749 = 10u64;
                                        if nat_eq(a748, a749) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a750 = v1;
                                                let a751 = 1u64;
                                                nat_add(a750, a751)?
                                            }, (v4, f189(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a752 = v1;
                                                let a753 = 1u64;
                                                nat_add(a752, a753)?
                                            }, (v4, Adt17::C0(false, false, f260(match v3.clone() {
                                                Adt17::C0(_, _, h754, _, _) => {
                                                    h754
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h755, _) => {
                                                    h755
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h756) => {
                                                    h756
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
                Ok(next) => { m757 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f261(List::cons(match v0.clone() {
        Adt17::C0(_, _, h761, _, _) => {
            h761
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h762, _) => {
            h762
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h763) => {
            h763
        }
    }))
}

pub fn f190(p766: List<List<Bytes>>, p767: List<List<Bytes>>) -> List<List<Bytes>> {
    let m765 = (p766, p767);
    {
        let mut m765 = m765;
        loop {
            let step = {
                let (v0, v1) = m765;
                let m764 = v0;
                match m764.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m765 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f262(v0.clone(), 0u64, {
        let a768 = v0;
        length_bytes(a768)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f192(v0: Adt22) -> Bytes {
    let a789 = f263(match v0.clone() {
        Adt22::C0(h769, _, _, _, _, _, _, _) => {
            h769
        }
    });
    let a790 = {
        let a787 = f263(match v0.clone() {
            Adt22::C0(_, h770, _, _, _, _, _, _) => {
                h770
            }
        });
        let a788 = {
            let a785 = f263(match v0.clone() {
                Adt22::C0(_, _, h771, _, _, _, _, _) => {
                    h771
                }
            });
            let a786 = {
                let a783 = f263(match v0.clone() {
                    Adt22::C0(_, _, _, h772, _, _, _, _) => {
                        h772
                    }
                });
                let a784 = {
                    let a781 = f263(match v0.clone() {
                        Adt22::C0(_, _, _, _, h773, _, _, _) => {
                            h773
                        }
                    });
                    let a782 = {
                        let a779 = f263(match v0.clone() {
                            Adt22::C0(_, _, _, _, _, h774, _, _) => {
                                h774
                            }
                        });
                        let a780 = {
                            let a777 = f263(match v0.clone() {
                                Adt22::C0(_, _, _, _, _, _, h775, _) => {
                                    h775
                                }
                            });
                            let a778 = f263(match v0 {
                                Adt22::C0(_, _, _, _, _, _, _, h776) => {
                                    h776
                                }
                            });
                            append_bytes(a777, a778)
                        };
                        append_bytes(a779, a780)
                    };
                    append_bytes(a781, a782)
                };
                append_bytes(a783, a784)
            };
            append_bytes(a785, a786)
        };
        append_bytes(a787, a788)
    };
    append_bytes(a789, a790)
}

pub fn f193(v0: Bytes) -> R<Adt22> {
    let v1: u64 = {
        let a791 = v0.clone();
        length_bytes(a791)
    };
    let v2: u64 = {
        let a797 = {
            let a794 = {
                let a792 = v1;
                let a793 = 72u64;
                nat_add(a792, a793)?
            };
            let a795 = 64u64;
            let a796 = 0u64;
            nat_quot(a794, a795, a796)
        };
        let a798 = 64u64;
        nat_mul(a797, a798)?
    };
    let (_, h799) = f264(Fn4::F265(v0, v1, v2), {
        let a800 = v2;
        let a801 = 64u64;
        let a802 = 0u64;
        nat_quot(a800, a801, a802)
    }, (0u64, f266()))?;
    Ok(h799)
}

pub fn f194(p805: List<List<Bytes>>, v1: Bytes, p806: List<Bytes>) -> R<List<Bytes>> {
    let m804 = (p805, p806);
    {
        let mut m804 = m804;
        loop {
            let step = {
                let (v0, v2) = m804;
                let m803 = v0;
                match m803.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f267(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f196(f268(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m804 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f269(v0)?))
}

pub fn f196(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m807 = v0;
    match m807.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m808 = v3.clone();
            match m808.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a811 = v2;
                    let a812 = {
                        let a809 = v1.clone();
                        let a810 = f196(v3, v1);
                        append_bytes(a809, a810)
                    };
                    append_bytes(a811, a812)
                }
            }
        }
    }
}

pub fn f197(v0: List<Bytes>) -> Adt14 {
    let m813 = v0;
    match m813.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f197(v2)))
        }
    }
}

pub fn f198(v0: Fn2, p817: Option<Bytes>, p818: List<Bytes>) -> Option<Bytes> {
    let m816 = (p817, p818);
    {
        let mut m816 = m816;
        loop {
            let step = {
                let (v1, v2) = m816;
                let m814 = v2;
                match m814.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c815: Fn2 = v0.clone();
                            c815.apply(v1, v3)
                        }, v4))
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

pub fn f199(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m819 = v1;
    match m819 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a822 = {
                    let a820 = v3;
                    let a821 = v0;
                    append_bytes(a820, a821)
                };
                let a823 = v2;
                append_bytes(a822, a823)
            })
        }
    }
}

pub fn f200(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f269(v0.clone())?;
    let v2: u64 = f270(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a824 = v1;
        let a825 = v2;
        nat_sub(a824, a825)
    }))
}

pub fn f201(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f271(v0)?;
    let m829 = f272(v1.clone(), 0u64, {
        let a827 = {
            let a826 = v1.clone();
            length_bytes(a826)
        };
        let a828 = 2u64;
        nat_add(a827, a828)?
    }, 0u64)?;
    match m829 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a832 = {
                let (_, h830) = v2;
                h830
            };
            let a833 = {
                let a831 = v1;
                length_bytes(a831)
            };
            if nat_eq(a832, a833) {
                Ok(Some({
                    let (h834, _) = v2;
                    h834
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f202(v0: Bytes) -> Bytes {
    f7(List::cons(f75(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f203(v0: Adt14) -> bool {
    if f184(v0.clone(), f8(Str::lit("domain"))) {
        if f184(v0.clone(), f8(Str::lit("row"))) {
            if f84(f139(v0.clone(), f8(Str::lit("relation"))), f273()) {
                if f274(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f274(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f274(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f274(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f204(v0: Adt14) -> bool {
    if f184(v0.clone(), f8(Str::lit("subject"))) {
        if f184(v0.clone(), f8(Str::lit("object"))) {
            if f184(v0.clone(), f8(Str::lit("evidence"))) {
                f84(f139(v0, f8(Str::lit("relation"))), f275())
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

pub fn f205(v0: Option<List<Adt14>>) -> bool {
    let m835 = v0;
    match m835 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f206(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m836 = v0;
    match m836 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f207(v0: Adt14) -> Bytes {
    if f184(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f184(v0.clone(), f8(Str::lit("id"))) {
            if f184(v0.clone(), f8(Str::lit("statement"))) {
                if f184(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f217(v0, f8(Str::lit("evidence")))
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

pub fn f208(v0: List<Adt14>) -> Adt14 {
    let m837 = v0;
    match m837.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f208(v2)))
        }
    }
}

pub fn f209(v0: Adt14) -> bool {
    if f184(v0.clone(), f8(Str::lit("name"))) {
        if f217(v0.clone(), f8(Str::lit("schema"))) {
            let m838 = f211(f72(f140(f136(v0, f8(Str::lit("citation"))))));
            match m838 {
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

pub fn f210(v0: Bytes) -> R<Bytes> {
    f276(v0.clone(), 0u64, {
        let a839 = v0;
        length_bytes(a839)
    }, true, Bytes::lit(&[]))
}

pub fn f211(v0: List<Bytes>) -> Option<Adt14> {
    let m840 = v0;
    match m840.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m841 = v2;
                match m841.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m842 = v2;
                    match m842.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m843 = v6;
                                match m843.uncons() {
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
                        let m844 = v2;
                        match m844.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m845 = v10;
                                match m845.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f197(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f212(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m846 = f277(4096u64, f278(f279(f280(f136(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt23::C0(v2, v0), List::<Adt23>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m846 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f196(f169(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f213(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m847 = v0;
    match m847.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f36(f139(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f281(v5, v1, v2))
            } else {
                List::cons(v4, f213(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f214(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f196(f282(f81(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f215(v0: Adt14) -> R<Bytes> {
    let m848 = v0;
    match m848 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f283(v2, v3)
        }
        Adt14::C2(v4) => {
            f284(v4)
        }
        Adt14::C3(r849) => {
            let v5 = (*r849).clone();
            Ok(f285(f286(v5)?))
        }
        Adt14::C4(r850) => {
            let v6 = (*r850).clone();
            f287(f288(v6)?)
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

pub fn f216(v0: Adt14) -> List<Adt14> {
    let m851 = v0;
    match m851 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r852) => {
            let v3 = (*r852).clone();
            f216(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r853, r854) => {
            let v1 = (*r853).clone();
            let v2 = (*r854).clone();
            List::cons(v1, f216(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f217(v0: Adt14, v1: Bytes) -> bool {
    let m855 = f136(v0, v1);
    match m855 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f218(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m856 = f136(v0, v1);
    match m856 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f140(f136(v3, v2))
        }
    }
}

pub fn f219(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f136(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f112(Str::lit("id"), f139(v0.clone(), f8(Str::lit("id"))))?, List::cons(f112(Str::lit("name"), f139(v0.clone(), f8(Str::lit("name"))))?, List::cons(f112(Str::lit("issuer"), f139(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f112(Str::lit("edition"), f139(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f112(Str::lit("canonical_id"), f139(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f112(Str::lit("source_role"), f139(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f112(Str::lit("retrieval_date"), f139(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f112(Str::lit("url"), f289(v2.clone(), f8(Str::lit("url"))))?, List::cons(f112(Str::lit("media_type"), f289(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f112(Str::lit("license"), f289(v2.clone(), f8(Str::lit("license"))))?, List::cons(f112(Str::lit("redistribution"), f289(v2, f8(Str::lit("redistribution"))))?, List::cons(f112(Str::lit("statement"), f139(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m857 = v1;
        match m857 {
            None => {
                f7(List::cons(f112(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f112(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f112(Str::lit("artifact"), {
                    let (h858, _) = v3;
                    h858
                })?, List::cons(f112(Str::lit("checksum"), {
                    let (_, h859) = v3;
                    h859
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f290(f72(f140(f136(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f220(v0: Bytes, v1: u64) -> Option<u8> {
    let a860 = v0;
    let a861 = v1;
    index_bytes(a860, a861)
}

pub fn f221(v0: Bytes) -> bool {
    f125(v0, f8(Str::lit(".lex.tex")))
}

pub fn f222(v0: Fn3, p866: u64, p867: u64) -> R<(u64, bool)> {
    let m865 = (p866, p867);
    {
        let mut m865 = m865;
        loop {
            let step = {
                let (v1, v2) = m865;
                let m862 = v1;
                if m862 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m862 - 1;
                    let m864 = {
                        let c863: Fn3 = v0.clone();
                        c863.apply(v2)?
                    };
                    match m864 {
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
                Ok(next) => { m865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f224(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a870 = {
            let a868 = v4;
            let a869 = v1;
            nat_add(a868, a869)?
        };
        let a871 = v3;
        if nat_lt(a870, a871) {
            Ok(Some({
                let a872 = v4;
                let a873 = 1u64;
                nat_add(a872, a873)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f224(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m878 = {
        let a875 = v0;
        let a876 = v2;
        let a877 = {
            let a874 = v1.clone();
            length_bytes(a874)
        };
        slice_bytes(a875, a876, a877)
    };
    match m878 {
        None => {
            false
        }
        Some(v3) => {
            let a879 = v3;
            let a880 = v1;
            equal(a879, a880)
        }
    }
}

pub fn f225(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m881 = v0;
    match m881.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f225(v3, {
                let a882 = v1;
                let a883 = 1u64;
                nat_add(a882, a883)?
            })?))
        }
    }
}

pub fn f226(v0: Adt16, v1: List<u64>, v2: Adt19) -> bool {
    let m884 = v2;
    match m884 {
        Adt19::C0(v3, v4) => {
            if f232(v3, match v0.clone() {
                Adt16::C0(h885, _, _, _) => {
                    h885
                }
            }) {
                f291(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f232(v5, match v0 {
                Adt16::C0(_, _, h886, _) => {
                    h886
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f228(v7, v1) {
                f232(v8, match v0 {
                    Adt16::C0(_, _, _, h887) => {
                        h887
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f227(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m888 = f136(v1.clone(), f8(Str::lit("std")));
    match m888 {
        None => {
            let m893 = f144(f136(v1.clone(), f8(Str::lit("req"))));
            match m893 {
                None => {
                    let m895 = f136(v1, f8(Str::lit("der")));
                    match m895 {
                        None => {
                            Ok(f292(v0))
                        }
                        Some(v5) => {
                            let m896 = f293(f139(v5.clone(), f8(Str::lit("rule"))));
                            match m896 {
                                None => {
                                    Ok(f292(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f160(f72(f140(f136(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h897) => {
                                            h897
                                        }
                                    })?, f230(f139(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h898, _) => {
                                            h898
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f230(v4, match v0 {
                        Adt12::C0(_, _, h894, _, _) => {
                            h894
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m889 = f144(f136(v2.clone(), f8(Str::lit("row"))));
            match m889 {
                None => {
                    Ok(Adt19::C0(f230(f139(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h890, _, _, _, _) => {
                            h890
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f230(f139(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h891, _, _, _, _) => {
                            h891
                        }
                    })?, Some(f230(v3, match v0 {
                        Adt12::C0(_, h892, _, _, _) => {
                            h892
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f228(p901: List<u64>, v1: List<u64>) -> bool {
    let m900 = p901;
    {
        let mut m900 = m900;
        loop {
            let step = {
                let v0 = m900;
                let m899 = v0;
                match m899.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f232(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m900 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(p906: List<u64>) -> bool {
    let m905 = p906;
    {
        let mut m905 = m905;
        loop {
            let step = {
                let v0 = m905;
                let m902 = v0;
                match m902.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a903 = v1;
                        let a904 = 7u64;
                        if nat_le(a903, a904) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m905 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m907 = f294(v0, v1.clone(), 0u64)?;
    match m907 {
        None => {
            let a908 = v1;
            Ok(length_list(a908))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f231(v0: Adt14) -> u64 {
    let m909 = f295(f136(v0, f8(Str::lit("stratum"))));
    match m909 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f232(v0: u64, p914: List<u64>) -> bool {
    let m913 = p914;
    {
        let mut m913 = m913;
        loop {
            let step = {
                let v1 = m913;
                let m910 = v1;
                match m910.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a911 = v2;
                        let a912 = v0;
                        if nat_eq(a911, a912) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m913 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f233(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f186(v0.clone(), v1) {
        let m915 = f30(v0, 34u8)?;
        match m915.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m916 = v3;
                match m916.uncons() {
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

pub fn f234(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f235(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m917 = v0.clone();
    match m917.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a918 = v2;
            let a919 = v1.clone();
            if equal(a918, a919) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f236(v0: List<(Bytes, ())>) -> List<Bytes> {
    f165(f296(Fn5::F297, List::<Bytes>::nil(), v0))
}

pub fn f237(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f298(f299(Fn6::F300, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m924 = {
        let (h920, _) = f301(Fn7::F302, {
            let a922 = {
                let a921 = v0;
                length_list(a921)
            };
            let a923 = 1u64;
            nat_add(a922, a923)?
        }, v1)?;
        h920
    };
    match m924.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f238(v0: List<Bytes>) -> List<(Bytes, ())> {
    f303(f304(Fn8::F305, List::<(Bytes, ())>::nil(), v0))
}

pub fn f239(p927: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m926 = p927;
    {
        let mut m926 = m926;
        loop {
            let step = {
                let v0 = m926;
                let m925 = v0;
                match m925.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f267(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m926 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f289(f136(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f241(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h928, _) = v1.clone();
        h928
    };
    let v3: u64 = {
        let (h929, _) = {
            let (_, h930) = v1.clone();
            h930
        };
        h929
    };
    let v4: List<Bytes> = {
        let (_, h931) = {
            let (_, h932) = v1;
            h932
        };
        h931
    };
    let m933 = f220(v0.clone(), v2);
    match m933 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a934 = v5;
            let a935 = 10u8;
            if equal(a934, a935) {
                Ok(Some(({
                    let a936 = v2;
                    let a937 = 1u64;
                    nat_add(a936, a937)?
                }, ({
                    let a938 = v2;
                    let a939 = 1u64;
                    nat_add(a938, a939)?
                }, List::cons(f306(f32(v0, v3, {
                    let a940 = v2;
                    let a941 = v3;
                    nat_sub(a940, a941)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a942 = v2;
                    let a943 = 1u64;
                    nat_add(a942, a943)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f242(v0: Bytes) -> R<Bytes> {
    let a944 = f8(Str::lit("Catalog."));
    let a945 = f210(v0)?;
    Ok(append_bytes(a944, a945))
}

pub fn f243(v0: List<Bytes>) -> List<Bytes> {
    let m946 = v0;
    match m946.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f125(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f121(f129(), v1), f243(v2))
            } else {
                f243(v2)
            }
        }
    }
}

pub fn f244(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m947 = v1;
    match m947.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f307(v0.clone(), v2)?;
            let (h948, _) = v4.clone();
            if h948 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f244(v0, v3)?;
                let (h949, _) = v5.clone();
                if h949 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a952 = {
                            let (_, h950) = v4;
                            h950
                        };
                        let a953 = {
                            let (_, h951) = v5;
                            h951
                        };
                        append_bytes(a952, a953)
                    }))
                }
            }
        }
    }
}

pub fn f245(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m954 = v1;
    match m954.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f184(v2.clone(), f8(Str::lit("subject"))) {
                if f185(v0.clone(), v2.clone()) {
                    f245(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f139(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f139(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f139(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f245(v0, v3))
                }
            } else {
                if f254(v0.clone(), v2.clone()) {
                    f245(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f139(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f255(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f245(v0, v3))
                }
            }
        }
    }
}

pub fn f246(v0: Adt1) -> R<Bytes> {
    let a956 = {
        let a955 = f101(f118(f119(f108(f68(v0.clone(), f113()))?)), f102(f71(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a955)
    };
    let a957 = 0u64;
    if nat_eq(a956, a957) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f247(v0: List<Adt14>) -> R<List<Bytes>> {
    let m958 = v0;
    match m958.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f117(f72(f140(f136(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f139(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f139(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f308(f81(f139(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f247(v2)?))
        }
    }
}

pub fn f248(v0: Bytes, v1: List<Adt21>) -> List<Bytes> {
    let m959 = v1;
    match m959.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(match v2.clone() {
                Adt21::C0(h960, _, _) => {
                    h960
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt21::C0(_, h961, _) => {
                        h961
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt21::C0(_, _, h962) => {
                        h962
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f248(v0, v3))
            } else {
                f248(v0, v3)
            }
        }
    }
}

pub fn f249(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m963 = v1;
    match m963.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(f139(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f139(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f139(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f249(v0, v3))
            } else {
                if f36(f139(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f139(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f139(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f249(v0, v3))
                } else {
                    f249(v0, v3)
                }
            }
        }
    }
}

pub fn f250(p968: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m967 = p968;
    {
        let mut m967 = m967;
        loop {
            let step = {
                let v0 = m967;
                let m964 = v0;
                match m964.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m965 = v2;
                        match m965.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f36(v4, v1.clone()) {
                                    let m966 = v5;
                                    match m966.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f309(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m967 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m969 = v0;
    match m969.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m970 = f136(v1, f8(Str::lit("std")));
            match m970 {
                None => {
                    f251(v2)
                }
                Some(v3) => {
                    if f184(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f139(v3.clone(), f8(Str::lit("row"))), f139(v3, f8(Str::lit("authority")))), f251(v2))
                    } else {
                        f251(v2)
                    }
                }
            }
        }
    }
}

pub fn f252(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m971 = v1;
    match m971.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f139(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f139(v2, f8(Str::lit("object")));
            if if f84(v4.clone(), match v0.clone() {
                Adt15::C0(h972, _, _) => {
                    h972
                }
            }) {
                f84(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h973, _) => {
                        h973
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f252(v0, v3))
            } else {
                if if f84(v5.clone(), match v0.clone() {
                    Adt15::C0(h974, _, _) => {
                        h974
                    }
                }) {
                    f84(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h975, _) => {
                            h975
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f252(v0, v3))
                } else {
                    f252(v0, v3)
                }
            }
        }
    }
}

pub fn f253(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt21>> {
    let m976 = v0;
    match m976.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v4, v5)) => {
            if if f310(v4.clone(), v2.clone()) {
                true
            } else {
                f310(v4.clone(), v3.clone())
            } {
                f253(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt21::C0({
                    let (h977, _) = v4.clone();
                    h977
                }, {
                    let a980 = {
                        let a979 = f76(f311({
                            let (h978, _) = v4.clone();
                            h978
                        }, v1.clone()))?;
                        length_list(a979)
                    };
                    let a981 = 1u64;
                    if nat_eq(a980, a981) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h982) = v4.clone();
                    h982
                }), f253(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f254(v0: Adt15, v1: Adt14) -> bool {
    if f84(f139(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h983, _, _) => {
            h983
        }
    }) {
        f84(f255(v1), match v0 {
            Adt15::C0(_, h984, _) => {
                h984
            }
        })
    } else {
        false
    }
}

pub fn f255(v0: Adt14) -> Bytes {
    let m985 = f136(v0, f8(Str::lit("evidence")));
    match m985 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f139(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f256(v0: Adt14) -> Bytes {
    if f184(v0.clone(), f8(Str::lit("req"))) {
        let a986 = f8(Str::lit("req "));
        let a987 = f139(v0, f8(Str::lit("req")));
        append_bytes(a986, a987)
    } else {
        let m988 = f136(v0.clone(), f8(Str::lit("std")));
        match m988 {
            None => {
                let m991 = f136(v0, f8(Str::lit("der")));
                match m991 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f139(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f117(f72(f140(f136(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f139(v1.clone(), f8(Str::lit("authority"))), List::cons(if f184(v1.clone(), f8(Str::lit("row"))) {
                    let a989 = f8(Str::lit(" row "));
                    let a990 = f139(v1, f8(Str::lit("row")));
                    append_bytes(a989, a990)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f257(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m992 = v1;
    match m992.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84({
                let (h993, _) = v2.clone();
                h993
            }, v0.clone()) {
                List::cons({
                    let (_, h994) = v2;
                    h994
                }, f257(v0, v3))
            } else {
                if f84({
                    let (_, h995) = v2;
                    h995
                }, v0.clone()) {
                    List::cons({
                        let (h996, _) = v2;
                        h996
                    }, f257(v0, v3))
                } else {
                    f257(v0, v3)
                }
            }
        }
    }
}

pub fn f258(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f84(v2.clone(), match v0.clone() {
        Adt15::C0(h997, _, _) => {
            h997
        }
    }) {
        true
    } else {
        if f84(v2.clone(), match v0.clone() {
            Adt15::C0(_, h998, _) => {
                h998
            }
        }) {
            true
        } else {
            if f84(v2, match v0.clone() {
                Adt15::C0(_, _, h999) => {
                    h999
                }
            }) {
                true
            } else {
                if f184(v1.clone(), f8(Str::lit("scheme"))) {
                    f84(f139(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1000, _) => {
                            h1000
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f259(v0: Bytes, v1: u64) -> R<u64> {
    let m1003 = {
        let a1001 = v0;
        let a1002 = v1;
        index_bytes(a1001, a1002)
    };
    match m1003 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1048 = {
                let a1024 = {
                    let a1012 = {
                        let a1006 = {
                            let a1004 = v3;
                            let a1005 = 128u8;
                            fixed_u8::bit_and(a1004, a1005)
                        };
                        let a1007 = 128u8;
                        if equal(a1006, a1007) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1013 = {
                        let a1010 = {
                            let a1008 = v3;
                            let a1009 = 64u8;
                            fixed_u8::bit_and(a1008, a1009)
                        };
                        let a1011 = 64u8;
                        if equal(a1010, a1011) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1012, a1013)?
                };
                let a1025 = {
                    let a1022 = {
                        let a1016 = {
                            let a1014 = v3;
                            let a1015 = 32u8;
                            fixed_u8::bit_and(a1014, a1015)
                        };
                        let a1017 = 32u8;
                        if equal(a1016, a1017) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1023 = {
                        let a1020 = {
                            let a1018 = v3;
                            let a1019 = 16u8;
                            fixed_u8::bit_and(a1018, a1019)
                        };
                        let a1021 = 16u8;
                        if equal(a1020, a1021) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1022, a1023)?
                };
                nat_add(a1024, a1025)?
            };
            let a1049 = {
                let a1046 = {
                    let a1034 = {
                        let a1028 = {
                            let a1026 = v3;
                            let a1027 = 8u8;
                            fixed_u8::bit_and(a1026, a1027)
                        };
                        let a1029 = 8u8;
                        if equal(a1028, a1029) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1035 = {
                        let a1032 = {
                            let a1030 = v3;
                            let a1031 = 4u8;
                            fixed_u8::bit_and(a1030, a1031)
                        };
                        let a1033 = 4u8;
                        if equal(a1032, a1033) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1034, a1035)?
                };
                let a1047 = {
                    let a1044 = {
                        let a1038 = {
                            let a1036 = v3;
                            let a1037 = 2u8;
                            fixed_u8::bit_and(a1036, a1037)
                        };
                        let a1039 = 2u8;
                        if equal(a1038, a1039) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1045 = {
                        let a1042 = {
                            let a1040 = v3;
                            let a1041 = 1u8;
                            fixed_u8::bit_and(a1040, a1041)
                        };
                        let a1043 = 1u8;
                        if equal(a1042, a1043) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1044, a1045)?
                };
                nat_add(a1046, a1047)?
            };
            nat_add(a1048, a1049)
        }
    }
}

pub fn f260(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1053 = {
        let a1050 = v1;
        let a1051 = v2;
        let a1052 = 1u64;
        slice_bytes(a1050, a1051, a1052)
    };
    match m1053 {
        None => {
            v0
        }
        Some(v3) => {
            let a1054 = v0;
            let a1055 = v3;
            append_bytes(a1054, a1055)
        }
    }
}

pub fn f261(p1058: List<Bytes>, p1059: List<Bytes>) -> List<Bytes> {
    let m1057 = (p1058, p1059);
    {
        let mut m1057 = m1057;
        loop {
            let step = {
                let (v0, v1) = m1057;
                let m1056 = v0;
                match m1056.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f262(v0: Bytes, p1131: u64, p1132: u64, p1133: Bytes) -> R<Bytes> {
    let m1130 = (p1131, (p1132, p1133));
    {
        let mut m1130 = m1130;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1130;
                let m1060 = v2;
                if m1060 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1060 - 1;
                    let m1063 = {
                        let a1061 = v0.clone();
                        let a1062 = v1;
                        index_bytes(a1061, a1062)
                    };
                    match m1063 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1108 = {
                                    let a1084 = {
                                        let a1072 = {
                                            let a1066 = {
                                                let a1064 = v6;
                                                let a1065 = 128u8;
                                                fixed_u8::bit_and(a1064, a1065)
                                            };
                                            let a1067 = 128u8;
                                            if equal(a1066, a1067) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1073 = {
                                            let a1070 = {
                                                let a1068 = v6;
                                                let a1069 = 64u8;
                                                fixed_u8::bit_and(a1068, a1069)
                                            };
                                            let a1071 = 64u8;
                                            if equal(a1070, a1071) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1072, a1073)?
                                    };
                                    let a1085 = {
                                        let a1082 = {
                                            let a1076 = {
                                                let a1074 = v6;
                                                let a1075 = 32u8;
                                                fixed_u8::bit_and(a1074, a1075)
                                            };
                                            let a1077 = 32u8;
                                            if equal(a1076, a1077) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1083 = {
                                            let a1080 = {
                                                let a1078 = v6;
                                                let a1079 = 16u8;
                                                fixed_u8::bit_and(a1078, a1079)
                                            };
                                            let a1081 = 16u8;
                                            if equal(a1080, a1081) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1082, a1083)?
                                    };
                                    nat_add(a1084, a1085)?
                                };
                                let a1109 = {
                                    let a1106 = {
                                        let a1094 = {
                                            let a1088 = {
                                                let a1086 = v6;
                                                let a1087 = 8u8;
                                                fixed_u8::bit_and(a1086, a1087)
                                            };
                                            let a1089 = 8u8;
                                            if equal(a1088, a1089) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1095 = {
                                            let a1092 = {
                                                let a1090 = v6;
                                                let a1091 = 4u8;
                                                fixed_u8::bit_and(a1090, a1091)
                                            };
                                            let a1093 = 4u8;
                                            if equal(a1092, a1093) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1094, a1095)?
                                    };
                                    let a1107 = {
                                        let a1104 = {
                                            let a1098 = {
                                                let a1096 = v6;
                                                let a1097 = 2u8;
                                                fixed_u8::bit_and(a1096, a1097)
                                            };
                                            let a1099 = 2u8;
                                            if equal(a1098, a1099) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1105 = {
                                            let a1102 = {
                                                let a1100 = v6;
                                                let a1101 = 1u8;
                                                fixed_u8::bit_and(a1100, a1101)
                                            };
                                            let a1103 = 1u8;
                                            if equal(a1102, a1103) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1104, a1105)?
                                    };
                                    nat_add(a1106, a1107)?
                                };
                                nat_add(a1108, a1109)?
                            };
                            let a1110 = v7;
                            let a1111 = 34u64;
                            if if nat_eq(a1110, a1111) {
                                true
                            } else {
                                let a1112 = v7;
                                let a1113 = 92u64;
                                nat_eq(a1112, a1113)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1114 = v1;
                                    let a1115 = 1u64;
                                    nat_add(a1114, a1115)?
                                }, (v4, {
                                    let a1118 = v3;
                                    let a1119 = {
                                        let a1116 = Bytes::lit(&[0x5c]);
                                        let a1117 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1116, a1117)
                                    };
                                    append_bytes(a1118, a1119)
                                })))
                            } else {
                                let a1120 = v7;
                                let a1121 = 10u64;
                                if nat_eq(a1120, a1121) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1122 = v1;
                                        let a1123 = 1u64;
                                        nat_add(a1122, a1123)?
                                    }, (v4, {
                                        let a1124 = v3;
                                        let a1125 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1124, a1125)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1126 = v1;
                                        let a1127 = 1u64;
                                        nat_add(a1126, a1127)?
                                    }, (v4, {
                                        let a1128 = v3;
                                        let a1129 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1128, a1129)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1130 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: u32) -> Bytes {
    let a1162 = f312({
        let a1134 = f313(v0, 28u32);
        let a1135 = 15u32;
        fixed_u32::bit_and(a1134, a1135)
    });
    let a1163 = {
        let a1160 = f312({
            let a1136 = f313(v0, 24u32);
            let a1137 = 15u32;
            fixed_u32::bit_and(a1136, a1137)
        });
        let a1161 = {
            let a1158 = f312({
                let a1138 = f313(v0, 20u32);
                let a1139 = 15u32;
                fixed_u32::bit_and(a1138, a1139)
            });
            let a1159 = {
                let a1156 = f312({
                    let a1140 = f313(v0, 16u32);
                    let a1141 = 15u32;
                    fixed_u32::bit_and(a1140, a1141)
                });
                let a1157 = {
                    let a1154 = f312({
                        let a1142 = f313(v0, 12u32);
                        let a1143 = 15u32;
                        fixed_u32::bit_and(a1142, a1143)
                    });
                    let a1155 = {
                        let a1152 = f312({
                            let a1144 = f313(v0, 8u32);
                            let a1145 = 15u32;
                            fixed_u32::bit_and(a1144, a1145)
                        });
                        let a1153 = {
                            let a1150 = f312({
                                let a1146 = f313(v0, 4u32);
                                let a1147 = 15u32;
                                fixed_u32::bit_and(a1146, a1147)
                            });
                            let a1151 = f312({
                                let a1148 = v0;
                                let a1149 = 15u32;
                                fixed_u32::bit_and(a1148, a1149)
                            });
                            append_bytes(a1150, a1151)
                        };
                        append_bytes(a1152, a1153)
                    };
                    append_bytes(a1154, a1155)
                };
                append_bytes(a1156, a1157)
            };
            append_bytes(a1158, a1159)
        };
        append_bytes(a1160, a1161)
    };
    append_bytes(a1162, a1163)
}

pub fn f264(v0: Fn4, p1167: u64, p1168: (u64, Adt22)) -> R<(u64, Adt22)> {
    let m1166 = (p1167, p1168);
    {
        let mut m1166 = m1166;
        loop {
            let step = {
                let (v1, v2) = m1166;
                let m1164 = v1;
                if m1164 == 0 {
                    Err::<(u64, (u64, Adt22)), R<(u64, Adt22)>>(Ok(v2))
                } else {
                    let v3 = m1164 - 1;
                    Ok::<(u64, (u64, Adt22)), R<(u64, Adt22)>>((v3, {
                        let c1165: Fn4 = v0.clone();
                        c1165.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f265(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt22)) -> R<(u64, Adt22)> {
    Ok(({
        let a1170 = {
            let (h1169, _) = v3.clone();
            h1169
        };
        let a1171 = 64u64;
        nat_add(a1170, a1171)?
    }, f314({
        let (_, h1172) = v3;
        h1172
    }, f315(v0, v1, v2, {
        let (h1173, _) = v3;
        h1173
    })?)?))
}

pub fn f266() -> Adt22 {
    Adt22::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f267(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f84(v1, f67(f196(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f268(v0: List<Bytes>) -> List<Bytes> {
    let m1174 = v0;
    match m1174.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f268(v2)
            } else {
                List::cons(v1, f268(v2))
            }
        }
    }
}

pub fn f269(v0: Bytes) -> R<u64> {
    let (h1175, _) = f222(Fn3::F316(v0.clone()), {
        let a1176 = v0.clone();
        length_bytes(a1176)
    }, {
        let a1177 = v0;
        length_bytes(a1177)
    })?;
    Ok(h1175)
}

pub fn f270(v0: Bytes, v1: u64) -> R<u64> {
    let (h1178, _) = f222(Fn3::F317(v1, v0), v1, 0u64)?;
    Ok(h1178)
}

pub fn f271(v0: Bytes) -> R<Bytes> {
    f318(v0.clone(), 0u64, {
        let a1179 = v0;
        length_bytes(a1179)
    }, false, false, Bytes::lit(&[]))
}

pub fn f272(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1180 = v2;
    if m1180 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1180 - 1;
        let v5: u64 = f319(v0.clone(), v1)?;
        let a1181 = v3;
        let a1182 = 1u64;
        if nat_eq(a1181, a1182) {
            let m1183 = f272(v0.clone(), v1, v4, 0u64)?;
            match m1183 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f319(v0.clone(), {
                        let (_, h1184) = v6.clone();
                        h1184
                    })?;
                    let a1185 = v7;
                    let a1186 = 44u64;
                    if nat_eq(a1185, a1186) {
                        let m1190 = f272(v0, {
                            let a1188 = {
                                let (_, h1187) = v6;
                                h1187
                            };
                            let a1189 = 1u64;
                            nat_add(a1188, a1189)?
                        }, v4, 1u64)?;
                        match m1190 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1191, _) = v6;
                                    h1191
                                }), std::rc::Rc::new({
                                    let (h1192, _) = v8;
                                    h1192
                                })), {
                                    let (_, h1193) = v8;
                                    h1193
                                })))
                            }
                        }
                    } else {
                        let a1194 = v7;
                        let a1195 = 93u64;
                        if nat_eq(a1194, a1195) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1196, _) = v6;
                                h1196
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1198 = {
                                    let (_, h1197) = v6;
                                    h1197
                                };
                                let a1199 = 1u64;
                                nat_add(a1198, a1199)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1200 = v3;
            let a1201 = 2u64;
            if nat_eq(a1200, a1201) {
                let m1204 = f320(v0.clone(), {
                    let a1202 = v1;
                    let a1203 = 1u64;
                    nat_add(a1202, a1203)?
                })?;
                match m1204 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1205 = v5;
                        let a1206 = 34u64;
                        if if nat_eq(a1205, a1206) {
                            let a1208 = f319(v0.clone(), {
                                let (h1207, _) = v9.clone();
                                h1207
                            })?;
                            let a1209 = 58u64;
                            nat_eq(a1208, a1209)
                        } else {
                            false
                        } {
                            let m1213 = f272(v0.clone(), {
                                let a1211 = {
                                    let (h1210, _) = v9;
                                    h1210
                                };
                                let a1212 = 1u64;
                                nat_add(a1211, a1212)?
                            }, v4, 0u64)?;
                            match m1213 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f319(v0.clone(), {
                                        let (_, h1214) = v10.clone();
                                        h1214
                                    })?;
                                    let a1215 = v11;
                                    let a1216 = 44u64;
                                    if nat_eq(a1215, a1216) {
                                        let m1220 = f272(v0, {
                                            let a1218 = {
                                                let (_, h1217) = v10;
                                                h1217
                                            };
                                            let a1219 = 1u64;
                                            nat_add(a1218, a1219)?
                                        }, v4, 2u64)?;
                                        match m1220 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1221) = v9;
                                                    h1221
                                                }, std::rc::Rc::new({
                                                    let (h1222, _) = v10;
                                                    h1222
                                                }), std::rc::Rc::new({
                                                    let (h1223, _) = v12;
                                                    h1223
                                                })), {
                                                    let (_, h1224) = v12;
                                                    h1224
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1225 = v11;
                                        let a1226 = 125u64;
                                        if nat_eq(a1225, a1226) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1227) = v9;
                                                h1227
                                            }, std::rc::Rc::new({
                                                let (h1228, _) = v10;
                                                h1228
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1230 = {
                                                    let (_, h1229) = v10;
                                                    h1229
                                                };
                                                let a1231 = 1u64;
                                                nat_add(a1230, a1231)?
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
                let a1232 = v5;
                let a1233 = 34u64;
                if nat_eq(a1232, a1233) {
                    let m1236 = f320(v0, {
                        let a1234 = v1;
                        let a1235 = 1u64;
                        nat_add(a1234, a1235)?
                    })?;
                    match m1236 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1237) = v13;
                                h1237
                            }), {
                                let (h1238, _) = v13;
                                h1238
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1239 = v1;
                            let a1240 = 4u64;
                            nat_add(a1239, a1240)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1241 = v1;
                                let a1242 = 5u64;
                                nat_add(a1241, a1242)?
                            })))
                        } else {
                            let a1243 = v5;
                            let a1244 = 91u64;
                            if nat_eq(a1243, a1244) {
                                let a1247 = f319(v0.clone(), {
                                    let a1245 = v1;
                                    let a1246 = 1u64;
                                    nat_add(a1245, a1246)?
                                })?;
                                let a1248 = 93u64;
                                if nat_eq(a1247, a1248) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1249 = v1;
                                        let a1250 = 2u64;
                                        nat_add(a1249, a1250)?
                                    })))
                                } else {
                                    let m1253 = f272(v0, {
                                        let a1251 = v1;
                                        let a1252 = 1u64;
                                        nat_add(a1251, a1252)?
                                    }, v4, 1u64)?;
                                    match m1253 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1254, _) = v14;
                                                h1254
                                            })), {
                                                let (_, h1255) = v14;
                                                h1255
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1256 = v5;
                                let a1257 = 123u64;
                                if nat_eq(a1256, a1257) {
                                    let a1260 = f319(v0.clone(), {
                                        let a1258 = v1;
                                        let a1259 = 1u64;
                                        nat_add(a1258, a1259)?
                                    })?;
                                    let a1261 = 125u64;
                                    if nat_eq(a1260, a1261) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1262 = v1;
                                            let a1263 = 2u64;
                                            nat_add(a1262, a1263)?
                                        })))
                                    } else {
                                        let m1266 = f272(v0, {
                                            let a1264 = v1;
                                            let a1265 = 1u64;
                                            nat_add(a1264, a1265)?
                                        }, v4, 2u64)?;
                                        match m1266 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1267, _) = v15;
                                                    h1267
                                                })), {
                                                    let (_, h1268) = v15;
                                                    h1268
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1269 = f321(v0, v1)?;
                                    match m1269 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1270) = v16;
                                                h1270
                                            }, {
                                                let (h1271, _) = v16;
                                                h1271
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

pub fn f273() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f274(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1272 = f136(v0, v1);
    match m1272 {
        None => {
            false
        }
        Some(v3) => {
            f184(v3, v2)
        }
    }
}

pub fn f275() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f276(v0: Bytes, p1339: u64, p1340: u64, p1341: bool, p1342: Bytes) -> R<Bytes> {
    let m1338 = (p1339, (p1340, (p1341, p1342)));
    {
        let mut m1338 = m1338;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1338;
                let m1273 = v2;
                if m1273 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1273 - 1;
                    let m1276 = {
                        let a1274 = v0.clone();
                        let a1275 = v1;
                        index_bytes(a1274, a1275)
                    };
                    match m1276 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1321 = {
                                    let a1297 = {
                                        let a1285 = {
                                            let a1279 = {
                                                let a1277 = v7;
                                                let a1278 = 128u8;
                                                fixed_u8::bit_and(a1277, a1278)
                                            };
                                            let a1280 = 128u8;
                                            if equal(a1279, a1280) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1286 = {
                                            let a1283 = {
                                                let a1281 = v7;
                                                let a1282 = 64u8;
                                                fixed_u8::bit_and(a1281, a1282)
                                            };
                                            let a1284 = 64u8;
                                            if equal(a1283, a1284) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1285, a1286)?
                                    };
                                    let a1298 = {
                                        let a1295 = {
                                            let a1289 = {
                                                let a1287 = v7;
                                                let a1288 = 32u8;
                                                fixed_u8::bit_and(a1287, a1288)
                                            };
                                            let a1290 = 32u8;
                                            if equal(a1289, a1290) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1296 = {
                                            let a1293 = {
                                                let a1291 = v7;
                                                let a1292 = 16u8;
                                                fixed_u8::bit_and(a1291, a1292)
                                            };
                                            let a1294 = 16u8;
                                            if equal(a1293, a1294) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1295, a1296)?
                                    };
                                    nat_add(a1297, a1298)?
                                };
                                let a1322 = {
                                    let a1319 = {
                                        let a1307 = {
                                            let a1301 = {
                                                let a1299 = v7;
                                                let a1300 = 8u8;
                                                fixed_u8::bit_and(a1299, a1300)
                                            };
                                            let a1302 = 8u8;
                                            if equal(a1301, a1302) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1308 = {
                                            let a1305 = {
                                                let a1303 = v7;
                                                let a1304 = 4u8;
                                                fixed_u8::bit_and(a1303, a1304)
                                            };
                                            let a1306 = 4u8;
                                            if equal(a1305, a1306) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1307, a1308)?
                                    };
                                    let a1320 = {
                                        let a1317 = {
                                            let a1311 = {
                                                let a1309 = v7;
                                                let a1310 = 2u8;
                                                fixed_u8::bit_and(a1309, a1310)
                                            };
                                            let a1312 = 2u8;
                                            if equal(a1311, a1312) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1318 = {
                                            let a1315 = {
                                                let a1313 = v7;
                                                let a1314 = 1u8;
                                                fixed_u8::bit_and(a1313, a1314)
                                            };
                                            let a1316 = 1u8;
                                            if equal(a1315, a1316) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1317, a1318)?
                                    };
                                    nat_add(a1319, a1320)?
                                };
                                nat_add(a1321, a1322)?
                            };
                            if f322(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1323 = v1;
                                    let a1324 = 1u64;
                                    nat_add(a1323, a1324)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f323(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1325 = v1;
                                        let a1326 = 1u64;
                                        nat_add(a1325, a1326)?
                                    }, (v5, (false, {
                                        let a1330 = v4;
                                        let a1331 = f32({
                                            let a1327 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1327)
                                        }, {
                                            let a1328 = v8;
                                            let a1329 = 97u64;
                                            nat_sub(a1328, a1329)
                                        }, 1u64);
                                        append_bytes(a1330, a1331)
                                    }))))
                                } else {
                                    if if f323(v8) {
                                        true
                                    } else {
                                        if f324(v8) {
                                            true
                                        } else {
                                            f325(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1332 = v1;
                                            let a1333 = 1u64;
                                            nat_add(a1332, a1333)?
                                        }, (v5, (false, {
                                            let a1334 = v4;
                                            let a1335 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1334, a1335)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1336 = v1;
                                            let a1337 = 1u64;
                                            nat_add(a1336, a1337)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(p1350: u64, p1351: List<Adt23>, p1352: List<Bytes>, p1353: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1349 = (p1350, (p1351, (p1352, p1353)));
    {
        let mut m1349 = m1349;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1349;
                let m1343 = v0;
                if m1343 == 0 {
                    Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1343 - 1;
                    let m1344 = v1;
                    match m1344.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1345 = v5;
                            match m1345 {
                                Adt23::C0(v8, v9) => {
                                    if f84(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m1346 = f326(v8.clone(), v9)?;
                                        match m1346 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f278({
                                                    let (_, h1347) = v11;
                                                    h1347
                                                }, List::cons(Adt23::C1({
                                                    let (h1348, _) = v11;
                                                    h1348
                                                }), v6)), (List::cons(v8, v2), v3))))
                                            }
                                            Err(v10) => {
                                                Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                            }
                                        }
                                    }
                                }
                                Adt23::C1(v7) => {
                                    Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: List<Adt23>, v1: List<Adt23>) -> List<Adt23> {
    let m1354 = v0;
    match m1354.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f278(v3, v1))
        }
    }
}

pub fn f279(v0: List<(Bytes, Adt14)>) -> R<List<Adt23>> {
    let m1355 = v0;
    match m1355.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt23::C0(f210({
                let (h1356, _) = v1;
                h1356
            })?, {
                let (_, h1357) = v1;
                h1357
            }), f279(v2)?))
        }
    }
}

pub fn f280(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1358 = v0;
    match m1358 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f327(v1)
        }
    }
}

pub fn f281(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1359 = v0;
    match m1359.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f36(f139(v3.clone(), v1.clone()), v2.clone()) {
                f281(v4, v1, v2)
            } else {
                List::cons(v3, f281(v4, v1, v2))
            }
        }
    }
}

pub fn f282(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1360 = v0;
    match m1360.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f186(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f196(f328(f76(List::cons(v1, f329(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f282(v3, v1)?))
            }
        }
    }
}

pub fn f283(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1361 = 0u64;
        let a1362 = v1;
        nat_lt(a1361, a1362)
    } else {
        false
    } {
        let a1363 = Bytes::lit(&[0x2d]);
        let a1364 = f330(v1)?;
        Ok(append_bytes(a1363, a1364))
    } else {
        f330(v1)
    }
}

pub fn f284(v0: Bytes) -> R<Bytes> {
    let a1370 = {
        let a1368 = Bytes::lit(&[0x22]);
        let a1369 = {
            let (_, h1365) = {
                let (h1366, _) = f331(Fn9::F332(v0.clone()), {
                    let a1367 = v0;
                    length_bytes(a1367)
                }, (0u64, Bytes::lit(&[])))?;
                h1366
            };
            h1365
        };
        append_bytes(a1368, a1369)
    };
    let a1371 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1370, a1371))
}

pub fn f285(v0: List<Bytes>) -> Bytes {
    let a1374 = {
        let a1372 = Bytes::lit(&[0x5b]);
        let a1373 = f117(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1372, a1373)
    };
    let a1375 = Bytes::lit(&[0x5d]);
    append_bytes(a1374, a1375)
}

pub fn f286(v0: Adt14) -> R<List<Bytes>> {
    let m1376 = v0;
    match m1376 {
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
        Adt14::C6(r1377, r1378) => {
            let v7 = (*r1377).clone();
            let v8 = (*r1378).clone();
            Ok(List::cons(f215(v7)?, f286(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f287(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f333(Fn10::F334, List::<(Bytes, Bytes)>::nil(), f335(v0)?);
    let v2: List<Bytes> = f336(Fn11::F337, List::<Bytes>::nil(), v1)?;
    let a1381 = {
        let a1379 = Bytes::lit(&[0x7b]);
        let a1380 = f117(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1379, a1380)
    };
    let a1382 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1381, a1382))
}

pub fn f288(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1383 = v0;
    match m1383 {
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
        Adt14::C7(v9, r1384, r1385) => {
            let v10 = (*r1384).clone();
            let v11 = (*r1385).clone();
            Ok(List::cons((v9, f215(v10)?), f288(v11)?))
        }
    }
}

pub fn f289(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1386 = v0;
    match m1386 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f139(v2, v1)
        }
    }
}

pub fn f290(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f196(f328(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f291(v0: Adt16, v1: Option<u64>) -> bool {
    let m1387 = v1;
    match m1387 {
        None => {
            true
        }
        Some(v2) => {
            f232(v2, match v0 {
                Adt16::C0(_, h1388, _, _) => {
                    h1388
                }
            })
        }
    }
}

pub fn f292(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a1390 = match v0 {
            Adt12::C0(_, _, h1389, _, _) => {
                h1389
            }
        };
        length_list(a1390)
    })
}

pub fn f293(v0: Bytes) -> Option<Adt20> {
    if f36(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt20::C0)
    } else {
        if f36(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt20::C1)
        } else {
            if f36(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt20::C2)
            } else {
                None::<Adt20>
            }
        }
    }
}

pub fn f294(v0: Bytes, p1395: List<Bytes>, p1396: u64) -> R<Option<u64>> {
    let m1394 = (p1395, p1396);
    {
        let mut m1394 = m1394;
        loop {
            let step = {
                let (v1, v2) = m1394;
                let m1391 = v1;
                match m1391.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1392 = v2;
                                let a1393 = 1u64;
                                nat_add(a1392, a1393)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: Option<Adt14>) -> Option<u64> {
    let m1397 = v0;
    match m1397 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1398 = v1;
            match m1398 {
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

pub fn f296(v0: Fn5, p1402: List<Bytes>, p1403: List<(Bytes, ())>) -> List<Bytes> {
    let m1401 = (p1402, p1403);
    {
        let mut m1401 = m1401;
        loop {
            let step = {
                let (v1, v2) = m1401;
                let m1399 = v2;
                match m1399.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1400: Fn5 = v0.clone();
                            c1400.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1401 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1404, _) = v1;
        h1404
    }, v0)
}

pub fn f298(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f338(Fn12::F339, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f299(v0: Fn6, p1408: List<List<(Bytes, ())>>, p1409: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1407 = (p1408, p1409);
    {
        let mut m1407 = m1407;
        loop {
            let step = {
                let (v1, v2) = m1407;
                let m1405 = v2;
                match m1405.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1406: Fn6 = v0.clone();
                            c1406.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1407 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f301(v0: Fn7, p1414: u64, p1415: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1413 = (p1414, p1415);
    {
        let mut m1413 = m1413;
        loop {
            let step = {
                let (v1, v2) = m1413;
                let m1410 = v1;
                if m1410 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1410 - 1;
                    let m1412 = {
                        let c1411: Fn7 = v0.clone();
                        c1411.apply(v2.clone())?
                    };
                    match m1412 {
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
                Ok(next) => { m1413 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f340(v0)
}

pub fn f303(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f341(Fn13::F342, List::<(Bytes, ())>::nil(), v0)
}

pub fn f304(v0: Fn8, p1419: List<(Bytes, ())>, p1420: List<Bytes>) -> List<(Bytes, ())> {
    let m1418 = (p1419, p1420);
    {
        let mut m1418 = m1418;
        loop {
            let step = {
                let (v1, v2) = m1418;
                let m1416 = v2;
                match m1416.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1417: Fn8 = v0.clone();
                            c1417.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1418 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f305(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f306(v0: Bytes) -> Bytes {
    if f125(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1422 = {
                let a1421 = v0;
                length_bytes(a1421)
            };
            let a1423 = 1u64;
            nat_sub(a1422, a1423)
        })
    } else {
        v0
    }
}

pub fn f307(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f81(f68(v0.clone(), v1.clone()))?;
    let m1424 = f141(v2.clone())?;
    match m1424 {
        None => {
            f344(v0, v1, v2)
        }
        Some(v3) => {
            f343(v0, v1, v3)
        }
    }
}

pub fn f308(v0: List<Bytes>) -> List<Bytes> {
    let m1425 = v0;
    match m1425.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1427 = {
                    let a1426 = v1.clone();
                    length_bytes(a1426)
                };
                let a1428 = 0u64;
                if nat_eq(a1427, a1428) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f308(v2))
        }
    }
}

pub fn f309(v0: List<Bytes>) -> Bytes {
    let m1429 = v0;
    match m1429.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f310(v0: (Bytes, Bytes), p1432: List<(Bytes, Bytes)>) -> bool {
    let m1431 = p1432;
    {
        let mut m1431 = m1431;
        loop {
            let step = {
                let v1 = m1431;
                let m1430 = v1;
                match m1430.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f345(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1433 = v1;
    match m1433.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1434, _) = v2;
                h1434
            }, v0.clone()) {
                List::cons({
                    let (_, h1435) = v2;
                    h1435
                }, f311(v0, v3))
            } else {
                f311(v0, v3)
            }
        }
    }
}

pub fn f312(v0: u32) -> Bytes {
    let a1438 = {
        let a1436 = v0;
        let a1437 = 8u32;
        fixed_u32::bit_and(a1436, a1437)
    };
    let a1439 = 0u32;
    if equal(a1438, a1439) {
        let a1442 = {
            let a1440 = v0;
            let a1441 = 4u32;
            fixed_u32::bit_and(a1440, a1441)
        };
        let a1443 = 0u32;
        if equal(a1442, a1443) {
            let a1446 = {
                let a1444 = v0;
                let a1445 = 2u32;
                fixed_u32::bit_and(a1444, a1445)
            };
            let a1447 = 0u32;
            if equal(a1446, a1447) {
                let a1450 = {
                    let a1448 = v0;
                    let a1449 = 1u32;
                    fixed_u32::bit_and(a1448, a1449)
                };
                let a1451 = 0u32;
                if equal(a1450, a1451) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1454 = {
                    let a1452 = v0;
                    let a1453 = 1u32;
                    fixed_u32::bit_and(a1452, a1453)
                };
                let a1455 = 0u32;
                if equal(a1454, a1455) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1458 = {
                let a1456 = v0;
                let a1457 = 2u32;
                fixed_u32::bit_and(a1456, a1457)
            };
            let a1459 = 0u32;
            if equal(a1458, a1459) {
                let a1462 = {
                    let a1460 = v0;
                    let a1461 = 1u32;
                    fixed_u32::bit_and(a1460, a1461)
                };
                let a1463 = 0u32;
                if equal(a1462, a1463) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1466 = {
                    let a1464 = v0;
                    let a1465 = 1u32;
                    fixed_u32::bit_and(a1464, a1465)
                };
                let a1467 = 0u32;
                if equal(a1466, a1467) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1470 = {
            let a1468 = v0;
            let a1469 = 4u32;
            fixed_u32::bit_and(a1468, a1469)
        };
        let a1471 = 0u32;
        if equal(a1470, a1471) {
            let a1474 = {
                let a1472 = v0;
                let a1473 = 2u32;
                fixed_u32::bit_and(a1472, a1473)
            };
            let a1475 = 0u32;
            if equal(a1474, a1475) {
                let a1478 = {
                    let a1476 = v0;
                    let a1477 = 1u32;
                    fixed_u32::bit_and(a1476, a1477)
                };
                let a1479 = 0u32;
                if equal(a1478, a1479) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1482 = {
                    let a1480 = v0;
                    let a1481 = 1u32;
                    fixed_u32::bit_and(a1480, a1481)
                };
                let a1483 = 0u32;
                if equal(a1482, a1483) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1486 = {
                let a1484 = v0;
                let a1485 = 2u32;
                fixed_u32::bit_and(a1484, a1485)
            };
            let a1487 = 0u32;
            if equal(a1486, a1487) {
                let a1490 = {
                    let a1488 = v0;
                    let a1489 = 1u32;
                    fixed_u32::bit_and(a1488, a1489)
                };
                let a1491 = 0u32;
                if equal(a1490, a1491) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1494 = {
                    let a1492 = v0;
                    let a1493 = 1u32;
                    fixed_u32::bit_and(a1492, a1493)
                };
                let a1495 = 0u32;
                if equal(a1494, a1495) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f313(v0: u32, v1: u32) -> u32 {
    let m1498 = {
        let a1496 = v0;
        let a1497 = v1;
        fixed_u32::shift_right(a1496, a1497)
    };
    match m1498 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f314(v0: Adt22, v1: Adt24) -> R<Adt22> {
    let v2: Adt25 = f346(Fn14::F347, 64u64, Adt25::C0(match v0.clone() {
        Adt22::C0(h1499, _, _, _, _, _, _, _) => {
            h1499
        }
    }, match v0.clone() {
        Adt22::C0(_, h1500, _, _, _, _, _, _) => {
            h1500
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h1501, _, _, _, _, _) => {
            h1501
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h1502, _, _, _, _) => {
            h1502
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h1503, _, _, _) => {
            h1503
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1504, _, _) => {
            h1504
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1505, _) => {
            h1505
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h1506) => {
            h1506
        }
    }, v1, 0u64))?;
    Ok(Adt22::C0(f348(match v0.clone() {
        Adt22::C0(h1507, _, _, _, _, _, _, _) => {
            h1507
        }
    }, match v2.clone() {
        Adt25::C0(h1508, _, _, _, _, _, _, _, _, _) => {
            h1508
        }
    }), f348(match v0.clone() {
        Adt22::C0(_, h1509, _, _, _, _, _, _) => {
            h1509
        }
    }, match v2.clone() {
        Adt25::C0(_, h1510, _, _, _, _, _, _, _, _) => {
            h1510
        }
    }), f348(match v0.clone() {
        Adt22::C0(_, _, h1511, _, _, _, _, _) => {
            h1511
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h1512, _, _, _, _, _, _, _) => {
            h1512
        }
    }), f348(match v0.clone() {
        Adt22::C0(_, _, _, h1513, _, _, _, _) => {
            h1513
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h1514, _, _, _, _, _, _) => {
            h1514
        }
    }), f348(match v0.clone() {
        Adt22::C0(_, _, _, _, h1515, _, _, _) => {
            h1515
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h1516, _, _, _, _, _) => {
            h1516
        }
    }), f348(match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1517, _, _) => {
            h1517
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h1518, _, _, _, _) => {
            h1518
        }
    }), f348(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1519, _) => {
            h1519
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h1520, _, _, _) => {
            h1520
        }
    }), f348(match v0 {
        Adt22::C0(_, _, _, _, _, _, _, h1521) => {
            h1521
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h1522, _, _) => {
            h1522
        }
    })))
}

pub fn f315(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f349(v0.clone(), v1, v2, v3)?, f349(v0.clone(), v1, v2, {
        let a1523 = v3;
        let a1524 = 4u64;
        nat_add(a1523, a1524)?
    })?, f349(v0.clone(), v1, v2, {
        let a1525 = v3;
        let a1526 = 8u64;
        nat_add(a1525, a1526)?
    })?, f349(v0.clone(), v1, v2, {
        let a1527 = v3;
        let a1528 = 12u64;
        nat_add(a1527, a1528)?
    })?, f349(v0.clone(), v1, v2, {
        let a1529 = v3;
        let a1530 = 16u64;
        nat_add(a1529, a1530)?
    })?, f349(v0.clone(), v1, v2, {
        let a1531 = v3;
        let a1532 = 20u64;
        nat_add(a1531, a1532)?
    })?, f349(v0.clone(), v1, v2, {
        let a1533 = v3;
        let a1534 = 24u64;
        nat_add(a1533, a1534)?
    })?, f349(v0.clone(), v1, v2, {
        let a1535 = v3;
        let a1536 = 28u64;
        nat_add(a1535, a1536)?
    })?, f349(v0.clone(), v1, v2, {
        let a1537 = v3;
        let a1538 = 32u64;
        nat_add(a1537, a1538)?
    })?, f349(v0.clone(), v1, v2, {
        let a1539 = v3;
        let a1540 = 36u64;
        nat_add(a1539, a1540)?
    })?, f349(v0.clone(), v1, v2, {
        let a1541 = v3;
        let a1542 = 40u64;
        nat_add(a1541, a1542)?
    })?, f349(v0.clone(), v1, v2, {
        let a1543 = v3;
        let a1544 = 44u64;
        nat_add(a1543, a1544)?
    })?, f349(v0.clone(), v1, v2, {
        let a1545 = v3;
        let a1546 = 48u64;
        nat_add(a1545, a1546)?
    })?, f349(v0.clone(), v1, v2, {
        let a1547 = v3;
        let a1548 = 52u64;
        nat_add(a1547, a1548)?
    })?, f349(v0.clone(), v1, v2, {
        let a1549 = v3;
        let a1550 = 56u64;
        nat_add(a1549, a1550)?
    })?, f349(v0, v1, v2, {
        let a1551 = v3;
        let a1552 = 60u64;
        nat_add(a1551, a1552)?
    })?))
}

pub fn f316(v0: Bytes, v1: u64) -> Option<u64> {
    let a1553 = 0u64;
    let a1554 = v1;
    if nat_lt(a1553, a1554) {
        if f350(v0, {
            let a1555 = v1;
            let a1556 = 1u64;
            nat_sub(a1555, a1556)
        }) {
            Some({
                let a1557 = v1;
                let a1558 = 1u64;
                nat_sub(a1557, a1558)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f317(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1559 = v2;
    let a1560 = v0;
    if nat_lt(a1559, a1560) {
        if f350(v1, v2) {
            Ok(Some({
                let a1561 = v2;
                let a1562 = 1u64;
                nat_add(a1561, a1562)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f318(v0: Bytes, p1642: u64, p1643: u64, p1644: bool, p1645: bool, p1646: Bytes) -> R<Bytes> {
    let m1641 = (p1642, (p1643, (p1644, (p1645, p1646))));
    {
        let mut m1641 = m1641;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1641;
                let m1563 = v2;
                if m1563 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1563 - 1;
                    let m1566 = {
                        let a1564 = v0.clone();
                        let a1565 = v1;
                        index_bytes(a1564, a1565)
                    };
                    match m1566 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1611 = {
                                    let a1587 = {
                                        let a1575 = {
                                            let a1569 = {
                                                let a1567 = v8;
                                                let a1568 = 128u8;
                                                fixed_u8::bit_and(a1567, a1568)
                                            };
                                            let a1570 = 128u8;
                                            if equal(a1569, a1570) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1576 = {
                                            let a1573 = {
                                                let a1571 = v8;
                                                let a1572 = 64u8;
                                                fixed_u8::bit_and(a1571, a1572)
                                            };
                                            let a1574 = 64u8;
                                            if equal(a1573, a1574) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1575, a1576)?
                                    };
                                    let a1588 = {
                                        let a1585 = {
                                            let a1579 = {
                                                let a1577 = v8;
                                                let a1578 = 32u8;
                                                fixed_u8::bit_and(a1577, a1578)
                                            };
                                            let a1580 = 32u8;
                                            if equal(a1579, a1580) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1586 = {
                                            let a1583 = {
                                                let a1581 = v8;
                                                let a1582 = 16u8;
                                                fixed_u8::bit_and(a1581, a1582)
                                            };
                                            let a1584 = 16u8;
                                            if equal(a1583, a1584) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1585, a1586)?
                                    };
                                    nat_add(a1587, a1588)?
                                };
                                let a1612 = {
                                    let a1609 = {
                                        let a1597 = {
                                            let a1591 = {
                                                let a1589 = v8;
                                                let a1590 = 8u8;
                                                fixed_u8::bit_and(a1589, a1590)
                                            };
                                            let a1592 = 8u8;
                                            if equal(a1591, a1592) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1598 = {
                                            let a1595 = {
                                                let a1593 = v8;
                                                let a1594 = 4u8;
                                                fixed_u8::bit_and(a1593, a1594)
                                            };
                                            let a1596 = 4u8;
                                            if equal(a1595, a1596) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1597, a1598)?
                                    };
                                    let a1610 = {
                                        let a1607 = {
                                            let a1601 = {
                                                let a1599 = v8;
                                                let a1600 = 2u8;
                                                fixed_u8::bit_and(a1599, a1600)
                                            };
                                            let a1602 = 2u8;
                                            if equal(a1601, a1602) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1608 = {
                                            let a1605 = {
                                                let a1603 = v8;
                                                let a1604 = 1u8;
                                                fixed_u8::bit_and(a1603, a1604)
                                            };
                                            let a1606 = 1u8;
                                            if equal(a1605, a1606) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1607, a1608)?
                                    };
                                    nat_add(a1609, a1610)?
                                };
                                nat_add(a1611, a1612)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1613 = v1;
                                    let a1614 = 1u64;
                                    nat_add(a1613, a1614)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1615 = v9;
                                    let a1616 = 34u64;
                                    !nat_eq(a1615, a1616)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1617 = v9;
                                    let a1618 = 92u64;
                                    nat_eq(a1617, a1618)
                                }, {
                                    let a1619 = v5;
                                    let a1620 = v10;
                                    append_bytes(a1619, a1620)
                                })))))
                            } else {
                                let a1621 = v9;
                                let a1622 = 32u64;
                                if if if if nat_eq(a1621, a1622) {
                                    true
                                } else {
                                    let a1623 = v9;
                                    let a1624 = 9u64;
                                    nat_eq(a1623, a1624)
                                } {
                                    true
                                } else {
                                    let a1625 = v9;
                                    let a1626 = 10u64;
                                    nat_eq(a1625, a1626)
                                } {
                                    true
                                } else {
                                    let a1627 = v9;
                                    let a1628 = 13u64;
                                    nat_eq(a1627, a1628)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1629 = v1;
                                        let a1630 = 1u64;
                                        nat_add(a1629, a1630)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1631 = v1;
                                            let a1632 = 4u64;
                                            nat_add(a1631, a1632)?
                                        }, (v6, (false, (false, {
                                            let a1633 = v5;
                                            let a1634 = f8(Str::lit("[]"));
                                            append_bytes(a1633, a1634)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1635 = v1;
                                            let a1636 = 1u64;
                                            nat_add(a1635, a1636)?
                                        }, (v6, ({
                                            let a1637 = v9;
                                            let a1638 = 34u64;
                                            nat_eq(a1637, a1638)
                                        }, (false, {
                                            let a1639 = v5;
                                            let a1640 = v10;
                                            append_bytes(a1639, a1640)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<u64> {
    let m1649 = {
        let a1647 = v0;
        let a1648 = v1;
        index_bytes(a1647, a1648)
    };
    match m1649 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1694 = {
                let a1670 = {
                    let a1658 = {
                        let a1652 = {
                            let a1650 = v3;
                            let a1651 = 128u8;
                            fixed_u8::bit_and(a1650, a1651)
                        };
                        let a1653 = 128u8;
                        if equal(a1652, a1653) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1659 = {
                        let a1656 = {
                            let a1654 = v3;
                            let a1655 = 64u8;
                            fixed_u8::bit_and(a1654, a1655)
                        };
                        let a1657 = 64u8;
                        if equal(a1656, a1657) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1658, a1659)?
                };
                let a1671 = {
                    let a1668 = {
                        let a1662 = {
                            let a1660 = v3;
                            let a1661 = 32u8;
                            fixed_u8::bit_and(a1660, a1661)
                        };
                        let a1663 = 32u8;
                        if equal(a1662, a1663) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1669 = {
                        let a1666 = {
                            let a1664 = v3;
                            let a1665 = 16u8;
                            fixed_u8::bit_and(a1664, a1665)
                        };
                        let a1667 = 16u8;
                        if equal(a1666, a1667) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1668, a1669)?
                };
                nat_add(a1670, a1671)?
            };
            let a1695 = {
                let a1692 = {
                    let a1680 = {
                        let a1674 = {
                            let a1672 = v3;
                            let a1673 = 8u8;
                            fixed_u8::bit_and(a1672, a1673)
                        };
                        let a1675 = 8u8;
                        if equal(a1674, a1675) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1681 = {
                        let a1678 = {
                            let a1676 = v3;
                            let a1677 = 4u8;
                            fixed_u8::bit_and(a1676, a1677)
                        };
                        let a1679 = 4u8;
                        if equal(a1678, a1679) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1680, a1681)?
                };
                let a1693 = {
                    let a1690 = {
                        let a1684 = {
                            let a1682 = v3;
                            let a1683 = 2u8;
                            fixed_u8::bit_and(a1682, a1683)
                        };
                        let a1685 = 2u8;
                        if equal(a1684, a1685) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1691 = {
                        let a1688 = {
                            let a1686 = v3;
                            let a1687 = 1u8;
                            fixed_u8::bit_and(a1686, a1687)
                        };
                        let a1689 = 1u8;
                        if equal(a1688, a1689) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1690, a1691)?
                };
                nat_add(a1692, a1693)?
            };
            nat_add(a1694, a1695)
        }
    }
}

pub fn f320(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f351(Fn15::F352(v0.clone()), {
        let a1697 = {
            let a1696 = v0;
            length_bytes(a1696)
        };
        let a1698 = 1u64;
        nat_add(a1697, a1698)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1702 = {
        let (_, h1699) = {
            let (_, h1700) = {
                let (h1701, _) = v2.clone();
                h1701
            };
            h1700
        };
        h1699
    };
    let a1703 = 1u64;
    if nat_eq(a1702, a1703) {
        Ok(Some(({
            let (h1704, _) = {
                let (h1705, _) = v2.clone();
                h1705
            };
            h1704
        }, {
            let (h1706, _) = {
                let (_, h1707) = {
                    let (h1708, _) = v2;
                    h1708
                };
                h1707
            };
            h1706
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f321(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1709 = f319(v0.clone(), v1)?;
        let a1710 = 45u64;
        nat_eq(a1709, a1710)
    };
    let v3: u64 = if v2 {
        let a1711 = v1;
        let a1712 = 1u64;
        nat_add(a1711, a1712)?
    } else {
        v1
    };
    let v4: u64 = f353(v0.clone(), v3)?;
    let a1715 = 19u64;
    let a1716 = {
        let a1713 = v4;
        let a1714 = v3;
        nat_sub(a1713, a1714)
    };
    if nat_lt(a1715, a1716) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1719 = f354(f32(v0, v3, {
            let a1717 = v4;
            let a1718 = v3;
            nat_sub(a1717, a1718)
        }))?;
        match m1719 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f355(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f322(v0: u64) -> bool {
    let a1720 = v0;
    let a1721 = 95u64;
    if nat_eq(a1720, a1721) {
        true
    } else {
        let a1722 = v0;
        let a1723 = 45u64;
        if nat_eq(a1722, a1723) {
            true
        } else {
            let a1724 = v0;
            let a1725 = 32u64;
            if nat_eq(a1724, a1725) {
                true
            } else {
                let a1726 = v0;
                let a1727 = 46u64;
                if nat_eq(a1726, a1727) {
                    true
                } else {
                    let a1728 = v0;
                    let a1729 = 47u64;
                    nat_eq(a1728, a1729)
                }
            }
        }
    }
}

pub fn f323(v0: u64) -> bool {
    let a1730 = 97u64;
    let a1731 = v0;
    if nat_le(a1730, a1731) {
        let a1732 = v0;
        let a1733 = 122u64;
        nat_le(a1732, a1733)
    } else {
        false
    }
}

pub fn f324(v0: u64) -> bool {
    let a1734 = 65u64;
    let a1735 = v0;
    if nat_le(a1734, a1735) {
        let a1736 = v0;
        let a1737 = 90u64;
        nat_le(a1736, a1737)
    } else {
        false
    }
}

pub fn f325(v0: u64) -> bool {
    let a1738 = 48u64;
    let a1739 = v0;
    if nat_le(a1738, a1739) {
        let a1740 = v0;
        let a1741 = 57u64;
        nat_le(a1740, a1741)
    } else {
        false
    }
}

pub fn f326(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m1742 = f136(v1.clone(), f8(Str::lit("enum")));
    match m1742 {
        None => {
            let m1744 = f359(v0.clone(), f280(f136(v1.clone(), f8(Str::lit("properties")))), f72(f140(f136(v1, f8(Str::lit("required"))))))?;
            match m1744 {
                Ok(v6) => {
                    Ok(f358(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m1746 = {
                            let (h1745, _) = v6.clone();
                            h1745
                        };
                        match m1746.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f196({
                                    let (h1747, _) = v6;
                                    h1747
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1748) = v6;
                        h1748
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt23>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m1743 = f356(f216(v2), List::<Bytes>::nil())?;
            match m1743 {
                Ok(v4) => {
                    Ok(f358(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f196(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt23>::nil()))
                }
                Err(v3) => {
                    Ok(f357(v3, v0))
                }
            }
        }
    }
}

pub fn f327(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1749 = v0;
    match m1749 {
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
        Adt14::C4(r1750) => {
            let v4 = (*r1750).clone();
            f327(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1751, r1752) => {
            let v2 = (*r1751).clone();
            let v3 = (*r1752).clone();
            List::cons((v1, v2), f327(v3))
        }
    }
}

pub fn f328(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1753 = v0;
    match m1753.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f191(v1)?, f328(v2)?))
        }
    }
}

pub fn f329(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1754 = v0;
    match m1754.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f329(v3, false))
            } else {
                f329(v3, true)
            }
        }
    }
}

pub fn f330(v0: u64) -> R<Bytes> {
    f360(f361(), 10u64, v0)
}

pub fn f331(v0: Fn9, p1759: u64, p1760: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1758 = (p1759, p1760);
    {
        let mut m1758 = m1758;
        loop {
            let step = {
                let (v1, v2) = m1758;
                let m1755 = v1;
                if m1755 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1755 - 1;
                    let m1757 = {
                        let c1756: Fn9 = v0.clone();
                        c1756.apply(v2.clone())?
                    };
                    match m1757 {
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
                Ok(next) => { m1758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f362(v0, v1)
}

pub fn f333(v0: Fn10, p1764: List<(Bytes, Bytes)>, p1765: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1763 = (p1764, p1765);
    {
        let mut m1763 = m1763;
        loop {
            let step = {
                let (v1, v2) = m1763;
                let m1761 = v2;
                match m1761.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1762: Fn10 = v0.clone();
                            c1762.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1763 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f334(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f363(v0, v1)
}

pub fn f335(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f364(v0)
}

pub fn f336(v0: Fn11, p1769: List<Bytes>, p1770: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1768 = (p1769, p1770);
    {
        let mut m1768 = m1768;
        loop {
            let step = {
                let (v1, v2) = m1768;
                let m1766 = v2;
                match m1766.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1767: Fn11 = v0.clone();
                            c1767.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1768 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f365(v1)?, v0))
}

pub fn f338(v0: Fn12, p1774: List<List<(Bytes, ())>>, p1775: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1773 = (p1774, p1775);
    {
        let mut m1773 = m1773;
        loop {
            let step = {
                let (v1, v2) = m1773;
                let m1771 = v2;
                match m1771.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1772: Fn12 = v0.clone();
                            c1772.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1773 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f340(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1777 = {
        let a1776 = v0.clone();
        length_list(a1776)
    };
    let a1778 = 1u64;
    if nat_le(a1777, a1778) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f366(v0)?))
    }
}

pub fn f341(v0: Fn13, p1782: List<(Bytes, ())>, p1783: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1781 = (p1782, p1783);
    {
        let mut m1781 = m1781;
        loop {
            let step = {
                let (v1, v2) = m1781;
                let m1779 = v2;
                match m1779.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1780: Fn13 = v0.clone();
                            c1780.apply(v1, v3)
                        }, v4))
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

pub fn f342(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f343(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f70(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f367(v3.clone(), f70(f71(v0, f87())?, f8(Str::lit("at"))));
    let a1785 = {
        let a1784 = v4.clone();
        length_list(a1784)
    };
    let a1786 = 0u64;
    if nat_eq(a1785, a1786) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1789 = {
            let a1787 = v4;
            length_list(a1787)
        };
        let a1790 = {
            let a1788 = v3;
            length_list(a1788)
        };
        if nat_eq(a1789, a1790) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f368(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f344(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f206(f131(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f206(f131(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f62(f70(v3.clone(), f8(Str::lit("id"))), f70(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f70(f71(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f367(v5.clone(), f62(v6.clone(), f70(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1792 = {
        let a1791 = v7.clone();
        length_list(a1791)
    };
    let a1793 = 0u64;
    if nat_eq(a1792, a1793) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f73({
            let a1796 = {
                let a1794 = v7;
                length_list(a1794)
            };
            let a1797 = {
                let a1795 = v5;
                length_list(a1795)
            };
            nat_eq(a1796, a1797)
        }) {
            Ok((false, f368(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f76(f62(f367(f369(v4.clone()), f67(f68(v0, f8(Str::lit("model/authorities.toml"))))?), f367(f370(v4), f62(v6, f70(v3, f8(Str::lit("id")))))))?;
            let a1799 = {
                let a1798 = v8.clone();
                length_list(a1798)
            };
            let a1800 = 0u64;
            if nat_eq(a1799, a1800) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f368(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f117(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f345(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h1801, _) = v0;
        h1801
    }, {
        let (h1802, _) = v1;
        h1802
    }) {
        f36({
            let (_, h1803) = v0;
            h1803
        }, {
            let (_, h1804) = v1;
            h1804
        })
    } else {
        false
    }
}

pub fn f346(v0: Fn14, p1808: u64, p1809: Adt25) -> R<Adt25> {
    let m1807 = (p1808, p1809);
    {
        let mut m1807 = m1807;
        loop {
            let step = {
                let (v1, v2) = m1807;
                let m1805 = v1;
                if m1805 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m1805 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c1806: Fn14 = v0.clone();
                        c1806.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1807 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: Adt25) -> R<Adt25> {
    f371(v0)
}

pub fn f348(v0: u32, v1: u32) -> u32 {
    let m1812 = {
        let a1810 = f372(v0);
        let a1811 = f372(v1);
        fixed_u64::checked_add(a1810, a1811)
    };
    match m1812 {
        None => {
            0u32
        }
        Some(v2) => {
            f373(v2)
        }
    }
}

pub fn f349(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1815 = {
        let a1813 = v3;
        let a1814 = 8u64;
        nat_add(a1813, a1814)?
    };
    let a1816 = v2;
    if nat_eq(a1815, a1816) {
        Ok(f374({
            let a1819 = {
                let a1817 = v1;
                let a1818 = 8u64;
                nat_mul(a1817, a1818)?
            };
            let a1820 = 4294967296u64;
            let a1821 = 0u64;
            nat_quot(a1819, a1820, a1821)
        }))
    } else {
        let a1824 = {
            let a1822 = v3;
            let a1823 = 4u64;
            nat_add(a1822, a1823)?
        };
        let a1825 = v2;
        if nat_eq(a1824, a1825) {
            Ok(f374({
                let a1828 = {
                    let a1826 = v1;
                    let a1827 = 8u64;
                    nat_mul(a1826, a1827)?
                };
                let a1829 = 4294967296u64;
                let a1830 = 0u64;
                nat_rem(a1828, a1829, a1830)
            }))
        } else {
            Ok(f375(f376(v0.clone(), v1, v3), f376(v0.clone(), v1, {
                let a1831 = v3;
                let a1832 = 1u64;
                nat_add(a1831, a1832)?
            }), f376(v0.clone(), v1, {
                let a1833 = v3;
                let a1834 = 2u64;
                nat_add(a1833, a1834)?
            }), f376(v0, v1, {
                let a1835 = v3;
                let a1836 = 3u64;
                nat_add(a1835, a1836)?
            })))
        }
    }
}

pub fn f350(v0: Bytes, v1: u64) -> bool {
    let m1837 = f220(v0, v1);
    match m1837 {
        None => {
            false
        }
        Some(v2) => {
            f377(v2)
        }
    }
}

pub fn f351(v0: Fn15, p1842: u64, p1843: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1841 = (p1842, p1843);
    {
        let mut m1841 = m1841;
        loop {
            let step = {
                let (v1, v2) = m1841;
                let m1838 = v1;
                if m1838 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1838 - 1;
                    let m1840 = {
                        let c1839: Fn15 = v0.clone();
                        c1839.apply(v2.clone())?
                    };
                    match m1840 {
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
                Ok(next) => { m1841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f378(v0, v1)
}

pub fn f353(v0: Bytes, v1: u64) -> R<u64> {
    let (h1844, _) = f222(Fn3::F379(v0.clone()), {
        let a1846 = {
            let a1845 = v0;
            length_bytes(a1845)
        };
        let a1847 = 1u64;
        nat_add(a1846, a1847)?
    }, v1)?;
    Ok(h1844)
}

pub fn f354(v0: Bytes) -> R<Option<u64>> {
    if f380(v0.clone())? {
        Ok(Some({
            let (_, h1848) = {
                let (h1849, _) = f381(Fn16::F382(v0.clone()), {
                    let a1850 = v0;
                    length_bytes(a1850)
                }, (0u64, 0u64))?;
                h1849
            };
            h1848
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f355(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1851 = 0u64;
        let a1852 = v1;
        if nat_lt(a1851, a1852) {
            let a1853 = v1;
            let a1854 = 9223372036854775808u64;
            nat_le(a1853, a1854)
        } else {
            false
        }
    } else {
        let a1855 = v1;
        let a1856 = 9223372036854775807u64;
        nat_le(a1855, a1856)
    }
}

pub fn f356(v0: List<Adt14>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1857 = v0;
    match m1857.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v2, v3)) => {
            let m1858 = f383(v2.clone())?;
            match m1858 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f384(v2)?, List::<Bytes>::nil())))))
                }
                Some(v4) => {
                    if f84(v4.clone(), v1.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v4, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1859 = f356(v3, List::cons(v4.clone(), v1))?;
                        match m1859 {
                            Ok(v6) => {
                                Ok(Ok::<List<Bytes>, Bytes>(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v4, List::cons(f8(Str::lit(", fields: []}")), List::<Bytes>::nil())))), v6)))
                            }
                            Err(v5) => {
                                Ok(Err::<List<Bytes>, Bytes>(v5))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f357(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt23>), Bytes> {
    Err::<(Bytes, List<Adt23>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f358(v0: Bytes, v1: List<Adt23>) -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((v0, v1))
}

pub fn f359(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m1860 = v1;
    match m1860.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f385({
                let (h1861, _) = v3.clone();
                h1861
            })?;
            if f386(v5.clone())? {
                let m1866 = f387({
                    let (_, h1862) = v3.clone();
                    h1862
                }, {
                    let a1864 = v0.clone();
                    let a1865 = f210({
                        let (h1863, _) = v3.clone();
                        h1863
                    })?;
                    append_bytes(a1864, a1865)
                })?;
                match m1866 {
                    Ok(v7) => {
                        let v8: Bytes = if f84({
                            let (h1867, _) = v3;
                            h1867
                        }, v2.clone()) {
                            let (h1868, _) = v7;
                            h1868
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1869, _) = v7;
                                h1869
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1870 = f359(v0, v4, v2)?;
                        match m1870 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1871, _) = v10;
                                    h1871
                                }), f278({
                                    let (_, h1872) = v7;
                                    h1872
                                }, {
                                    let (_, h1873) = v10;
                                    h1873
                                }))))
                            }
                            Err(v9) => {
                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v9))
                            }
                        }
                    }
                    Err(v6) => {
                        Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v6))
                    }
                }
            } else {
                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                    let (h1874, _) = v3;
                    h1874
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f360(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1875 = v2;
    let a1876 = 0u64;
    if nat_eq(a1875, a1876) {
        Ok(f388(v0, 0u64))
    } else {
        let (_, h1877) = {
            let (h1878, _) = f331(Fn9::F389(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1878
        };
        Ok(h1877)
    }
}

pub fn f361() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f362(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1882 = {
        let a1880 = v0.clone();
        let a1881 = {
            let (h1879, _) = v1.clone();
            h1879
        };
        index_bytes(a1880, a1881)
    };
    match m1882 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1884 = {
                    let (h1883, _) = v1.clone();
                    h1883
                };
                let a1885 = 1u64;
                nat_add(a1884, a1885)?
            }, {
                let a1888 = {
                    let (_, h1886) = v1;
                    h1886
                };
                let a1889 = f390(v0, {
                    let (h1887, _) = v1;
                    h1887
                }, v2)?;
                append_bytes(a1888, a1889)
            })))
        }
    }
}

pub fn f363(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1890 = v0.clone();
    match m1890.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1893 = {
                let (h1891, _) = v2;
                h1891
            };
            let a1894 = {
                let (h1892, _) = v1.clone();
                h1892
            };
            if equal(a1893, a1894) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f364(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f391(f392(Fn17::F393, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1899 = {
        let (h1895, _) = f394(Fn18::F395, {
            let a1897 = {
                let a1896 = v0;
                length_list(a1896)
            };
            let a1898 = 1u64;
            nat_add(a1897, a1898)?
        }, v1)?;
        h1895
    };
    match m1899.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f365(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1904 = {
        let a1901 = f284({
            let (h1900, _) = v0;
            h1900
        })?;
        let a1902 = Bytes::lit(&[0x3a]);
        append_bytes(a1901, a1902)
    };
    let a1905 = {
        let (_, h1903) = v0;
        h1903
    };
    Ok(append_bytes(a1904, a1905))
}

pub fn f366(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f298({
        let (_, h1906) = {
            let (h1907, _) = f396(Fn19::F397, {
                let a1909 = {
                    let a1908 = v0.clone();
                    length_list(a1908)
                };
                let a1910 = 1u64;
                nat_add(a1909, a1910)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1907
        };
        h1906
    }))
}

pub fn f367(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1911 = v0;
    match m1911.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84(v2.clone(), v1.clone()) {
                f367(v3, v1)
            } else {
                List::cons(v2, f367(v3, v1))
            }
        }
    }
}

pub fn f368(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f369(v0: List<Adt14>) -> List<Bytes> {
    let m1912 = v0;
    match m1912.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1913 = f136(v1, f8(Str::lit("std")));
            match m1913 {
                None => {
                    f369(v2)
                }
                Some(v3) => {
                    List::cons(f139(v3, f8(Str::lit("authority"))), f369(v2))
                }
            }
        }
    }
}

pub fn f370(v0: List<Adt14>) -> List<Bytes> {
    let m1914 = v0;
    match m1914.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1915 = f144(f136(v1, f8(Str::lit("req"))));
            match m1915 {
                None => {
                    f370(v2)
                }
                Some(v3) => {
                    List::cons(v3, f370(v2))
                }
            }
        }
    }
}

pub fn f371(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f398(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h1916) => {
            h1916
        }
    });
    let v2: u32 = f348(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h1917, _, _) => {
            h1917
        }
    }, f348(f399(match v0.clone() {
        Adt25::C0(_, _, _, _, h1918, _, _, _, _, _) => {
            h1918
        }
    }), f348(f400(match v0.clone() {
        Adt25::C0(_, _, _, _, h1919, _, _, _, _, _) => {
            h1919
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1920, _, _, _, _) => {
            h1920
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1921, _, _, _) => {
            h1921
        }
    }), f348(v1, {
        let m1924 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h1923, _) => {
                h1923
            }
        };
        match m1924 {
            Adt24::C0(h1922, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1922
            }
        }
    }))));
    let v3: u32 = f348(f401(match v0.clone() {
        Adt25::C0(h1925, _, _, _, _, _, _, _, _, _) => {
            h1925
        }
    }), f402(match v0.clone() {
        Adt25::C0(h1926, _, _, _, _, _, _, _, _, _) => {
            h1926
        }
    }, match v0.clone() {
        Adt25::C0(_, h1927, _, _, _, _, _, _, _, _) => {
            h1927
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1928, _, _, _, _, _, _, _) => {
            h1928
        }
    }));
    Ok(Adt25::C0(f348(v2, v3), match v0.clone() {
        Adt25::C0(h1929, _, _, _, _, _, _, _, _, _) => {
            h1929
        }
    }, match v0.clone() {
        Adt25::C0(_, h1930, _, _, _, _, _, _, _, _) => {
            h1930
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1931, _, _, _, _, _, _, _) => {
            h1931
        }
    }, f348(match v0.clone() {
        Adt25::C0(_, _, _, h1932, _, _, _, _, _, _) => {
            h1932
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h1933, _, _, _, _, _) => {
            h1933
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1934, _, _, _, _) => {
            h1934
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1935, _, _, _) => {
            h1935
        }
    }, f403(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h1936, _) => {
            h1936
        }
    }), {
        let a1938 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h1937) => {
                h1937
            }
        };
        let a1939 = 1u64;
        nat_add(a1938, a1939)?
    }))
}

pub fn f372(v0: u32) -> u64 {
    let m1941 = {
        let a1940 = v0;
        fixed_u64::convert(i128::from(a1940))
    };
    match m1941 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f373(v0: u64) -> u32 {
    let m1945 = {
        let a1944 = {
            let a1942 = v0;
            let a1943 = 4294967295u64;
            fixed_u64::bit_and(a1942, a1943)
        };
        fixed_u32::convert(i128::from(a1944))
    };
    match m1945 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f374(v0: u64) -> u32 {
    let (_, h1946) = {
        let (_, h1947) = f404(Fn20::F405, 32u64, (v0, (1u32, 0u32)));
        h1947
    };
    h1946
}

pub fn f375(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1952 = f406(v0, 24u32);
    let a1953 = {
        let a1950 = f406(v1, 16u32);
        let a1951 = {
            let a1948 = f406(v2, 8u32);
            let a1949 = v3;
            fixed_u32::bit_or(a1948, a1949)
        };
        fixed_u32::bit_or(a1950, a1951)
    };
    fixed_u32::bit_or(a1952, a1953)
}

pub fn f376(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1954 = v2;
    let a1955 = v1;
    if nat_lt(a1954, a1955) {
        let m1958 = {
            let a1956 = v0;
            let a1957 = v2;
            index_bytes(a1956, a1957)
        };
        match m1958 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1960 = {
                    let a1959 = v3;
                    fixed_u32::convert(i128::from(a1959))
                };
                match m1960 {
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
        let a1961 = v2;
        let a1962 = v1;
        if nat_eq(a1961, a1962) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f377(v0: u8) -> bool {
    let a1963 = v0;
    let a1964 = 32u8;
    if if if equal(a1963, a1964) {
        true
    } else {
        let a1965 = v0;
        let a1966 = 9u8;
        equal(a1965, a1966)
    } {
        true
    } else {
        let a1967 = v0;
        let a1968 = 10u8;
        equal(a1967, a1968)
    } {
        true
    } else {
        let a1969 = v0;
        let a1970 = 12u8;
        if equal(a1969, a1970) {
            true
        } else {
            let a1971 = v0;
            let a1972 = 13u8;
            equal(a1971, a1972)
        }
    }
}

pub fn f378(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1973, _) = v1.clone();
        h1973
    };
    let v3: Bytes = {
        let (h1974, _) = {
            let (_, h1975) = v1.clone();
            h1975
        };
        h1974
    };
    let a1978 = {
        let (_, h1976) = {
            let (_, h1977) = v1;
            h1977
        };
        h1976
    };
    let a1979 = 0u64;
    if nat_eq(a1978, a1979) {
        let v4: u64 = f319(v0.clone(), v2)?;
        let a1980 = v4;
        let a1981 = 34u64;
        if nat_eq(a1980, a1981) {
            Ok(Some(({
                let a1982 = v2;
                let a1983 = 1u64;
                nat_add(a1982, a1983)?
            }, (v3, 1u64))))
        } else {
            let a1984 = v4;
            let a1985 = 92u64;
            if nat_eq(a1984, a1985) {
                f407(v0, v2, v3)
            } else {
                let a1986 = v4;
                let a1987 = 32u64;
                if if nat_lt(a1986, a1987) {
                    true
                } else {
                    let a1988 = v4;
                    let a1989 = 256u64;
                    nat_eq(a1988, a1989)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1990 = v2;
                        let a1991 = 1u64;
                        nat_add(a1990, a1991)?
                    }, ({
                        let a1992 = v3;
                        let a1993 = f32(v0, v2, 1u64);
                        append_bytes(a1992, a1993)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f379(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f408(v0, v1)
}

pub fn f380(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1994 = v0.clone();
        length_bytes(a1994)
    };
    let a1995 = 0u64;
    let a1996 = v1;
    if nat_lt(a1995, a1996) {
        let a1997 = f409(v0.clone())?;
        let a1998 = v1;
        if nat_eq(a1997, a1998) {
            let a1999 = 1u64;
            let a2000 = v1;
            if nat_lt(a1999, a2000) {
                let m2003 = {
                    let a2001 = v0;
                    let a2002 = 0u64;
                    index_bytes(a2001, a2002)
                };
                match m2003 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2004 = v2;
                        let a2005 = 48u8;
                        Ok(!equal(a2004, a2005))
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

pub fn f381(v0: Fn16, p2010: u64, p2011: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2009 = (p2010, p2011);
    {
        let mut m2009 = m2009;
        loop {
            let step = {
                let (v1, v2) = m2009;
                let m2006 = v1;
                if m2006 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2006 - 1;
                    let m2008 = {
                        let c2007: Fn16 = v0.clone();
                        c2007.apply(v2)?
                    };
                    match m2008 {
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
                Ok(next) => { m2009 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f410(v0, v1)
}

pub fn f383(v0: Adt14) -> R<Option<Bytes>> {
    let m2012 = v0;
    match m2012 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v2, v3) => {
            Ok(Some({
                let a2015 = if if v2 {
                    let a2013 = 0u64;
                    let a2014 = v3;
                    nat_lt(a2013, a2014)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2016 = f283(false, v3)?;
                append_bytes(a2015, a2016)
            }))
        }
        Adt14::C2(v1) => {
            f411(v1)
        }
        Adt14::C3(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C5 => {
            Ok(None::<Bytes>)
        }
        Adt14::C6(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt14::C7(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f384(v0: Adt14) -> R<Bytes> {
    f215(v0)
}

pub fn f385(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f276(v0.clone(), 0u64, {
        let a2017 = v0;
        length_bytes(a2017)
    }, false, Bytes::lit(&[]))?;
    let m2020 = {
        let a2018 = v1.clone();
        let a2019 = 0u64;
        index_bytes(a2018, a2019)
    };
    match m2020 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2065 = {
                    let a2041 = {
                        let a2029 = {
                            let a2023 = {
                                let a2021 = v3;
                                let a2022 = 128u8;
                                fixed_u8::bit_and(a2021, a2022)
                            };
                            let a2024 = 128u8;
                            if equal(a2023, a2024) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2030 = {
                            let a2027 = {
                                let a2025 = v3;
                                let a2026 = 64u8;
                                fixed_u8::bit_and(a2025, a2026)
                            };
                            let a2028 = 64u8;
                            if equal(a2027, a2028) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2029, a2030)?
                    };
                    let a2042 = {
                        let a2039 = {
                            let a2033 = {
                                let a2031 = v3;
                                let a2032 = 32u8;
                                fixed_u8::bit_and(a2031, a2032)
                            };
                            let a2034 = 32u8;
                            if equal(a2033, a2034) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2040 = {
                            let a2037 = {
                                let a2035 = v3;
                                let a2036 = 16u8;
                                fixed_u8::bit_and(a2035, a2036)
                            };
                            let a2038 = 16u8;
                            if equal(a2037, a2038) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2039, a2040)?
                    };
                    nat_add(a2041, a2042)?
                };
                let a2066 = {
                    let a2063 = {
                        let a2051 = {
                            let a2045 = {
                                let a2043 = v3;
                                let a2044 = 8u8;
                                fixed_u8::bit_and(a2043, a2044)
                            };
                            let a2046 = 8u8;
                            if equal(a2045, a2046) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2052 = {
                            let a2049 = {
                                let a2047 = v3;
                                let a2048 = 4u8;
                                fixed_u8::bit_and(a2047, a2048)
                            };
                            let a2050 = 4u8;
                            if equal(a2049, a2050) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2051, a2052)?
                    };
                    let a2064 = {
                        let a2061 = {
                            let a2055 = {
                                let a2053 = v3;
                                let a2054 = 2u8;
                                fixed_u8::bit_and(a2053, a2054)
                            };
                            let a2056 = 2u8;
                            if equal(a2055, a2056) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2062 = {
                            let a2059 = {
                                let a2057 = v3;
                                let a2058 = 1u8;
                                fixed_u8::bit_and(a2057, a2058)
                            };
                            let a2060 = 1u8;
                            if equal(a2059, a2060) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2061, a2062)?
                    };
                    nat_add(a2063, a2064)?
                };
                nat_add(a2065, a2066)?
            };
            if f324(v4) {
                let a2073 = f32({
                    let a2067 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2067)
                }, {
                    let a2068 = v4;
                    let a2069 = 65u64;
                    nat_sub(a2068, a2069)
                }, 1u64);
                let a2074 = f32(v1.clone(), 1u64, {
                    let a2071 = {
                        let a2070 = v1;
                        length_bytes(a2070)
                    };
                    let a2072 = 1u64;
                    nat_sub(a2071, a2072)
                });
                Ok(append_bytes(a2073, a2074))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f386(v0: Bytes) -> R<bool> {
    let m2077 = {
        let a2075 = v0;
        let a2076 = 0u64;
        index_bytes(a2075, a2076)
    };
    match m2077 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f323({
                let v2: u8 = v1;
                let a2122 = {
                    let a2098 = {
                        let a2086 = {
                            let a2080 = {
                                let a2078 = v2;
                                let a2079 = 128u8;
                                fixed_u8::bit_and(a2078, a2079)
                            };
                            let a2081 = 128u8;
                            if equal(a2080, a2081) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2087 = {
                            let a2084 = {
                                let a2082 = v2;
                                let a2083 = 64u8;
                                fixed_u8::bit_and(a2082, a2083)
                            };
                            let a2085 = 64u8;
                            if equal(a2084, a2085) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2086, a2087)?
                    };
                    let a2099 = {
                        let a2096 = {
                            let a2090 = {
                                let a2088 = v2;
                                let a2089 = 32u8;
                                fixed_u8::bit_and(a2088, a2089)
                            };
                            let a2091 = 32u8;
                            if equal(a2090, a2091) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2097 = {
                            let a2094 = {
                                let a2092 = v2;
                                let a2093 = 16u8;
                                fixed_u8::bit_and(a2092, a2093)
                            };
                            let a2095 = 16u8;
                            if equal(a2094, a2095) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2096, a2097)?
                    };
                    nat_add(a2098, a2099)?
                };
                let a2123 = {
                    let a2120 = {
                        let a2108 = {
                            let a2102 = {
                                let a2100 = v2;
                                let a2101 = 8u8;
                                fixed_u8::bit_and(a2100, a2101)
                            };
                            let a2103 = 8u8;
                            if equal(a2102, a2103) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2109 = {
                            let a2106 = {
                                let a2104 = v2;
                                let a2105 = 4u8;
                                fixed_u8::bit_and(a2104, a2105)
                            };
                            let a2107 = 4u8;
                            if equal(a2106, a2107) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2108, a2109)?
                    };
                    let a2121 = {
                        let a2118 = {
                            let a2112 = {
                                let a2110 = v2;
                                let a2111 = 2u8;
                                fixed_u8::bit_and(a2110, a2111)
                            };
                            let a2113 = 2u8;
                            if equal(a2112, a2113) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2119 = {
                            let a2116 = {
                                let a2114 = v2;
                                let a2115 = 1u8;
                                fixed_u8::bit_and(a2114, a2115)
                            };
                            let a2117 = 1u8;
                            if equal(a2116, a2117) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2118, a2119)?
                    };
                    nat_add(a2120, a2121)?
                };
                nat_add(a2122, a2123)?
            }) {
                Ok(true)
            } else {
                Ok(f324({
                    let v3: u8 = v1;
                    let a2168 = {
                        let a2144 = {
                            let a2132 = {
                                let a2126 = {
                                    let a2124 = v3;
                                    let a2125 = 128u8;
                                    fixed_u8::bit_and(a2124, a2125)
                                };
                                let a2127 = 128u8;
                                if equal(a2126, a2127) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a2133 = {
                                let a2130 = {
                                    let a2128 = v3;
                                    let a2129 = 64u8;
                                    fixed_u8::bit_and(a2128, a2129)
                                };
                                let a2131 = 64u8;
                                if equal(a2130, a2131) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2132, a2133)?
                        };
                        let a2145 = {
                            let a2142 = {
                                let a2136 = {
                                    let a2134 = v3;
                                    let a2135 = 32u8;
                                    fixed_u8::bit_and(a2134, a2135)
                                };
                                let a2137 = 32u8;
                                if equal(a2136, a2137) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a2143 = {
                                let a2140 = {
                                    let a2138 = v3;
                                    let a2139 = 16u8;
                                    fixed_u8::bit_and(a2138, a2139)
                                };
                                let a2141 = 16u8;
                                if equal(a2140, a2141) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2142, a2143)?
                        };
                        nat_add(a2144, a2145)?
                    };
                    let a2169 = {
                        let a2166 = {
                            let a2154 = {
                                let a2148 = {
                                    let a2146 = v3;
                                    let a2147 = 8u8;
                                    fixed_u8::bit_and(a2146, a2147)
                                };
                                let a2149 = 8u8;
                                if equal(a2148, a2149) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a2155 = {
                                let a2152 = {
                                    let a2150 = v3;
                                    let a2151 = 4u8;
                                    fixed_u8::bit_and(a2150, a2151)
                                };
                                let a2153 = 4u8;
                                if equal(a2152, a2153) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2154, a2155)?
                        };
                        let a2167 = {
                            let a2164 = {
                                let a2158 = {
                                    let a2156 = v3;
                                    let a2157 = 2u8;
                                    fixed_u8::bit_and(a2156, a2157)
                                };
                                let a2159 = 2u8;
                                if equal(a2158, a2159) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2165 = {
                                let a2162 = {
                                    let a2160 = v3;
                                    let a2161 = 1u8;
                                    fixed_u8::bit_and(a2160, a2161)
                                };
                                let a2163 = 1u8;
                                if equal(a2162, a2163) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2164, a2165)?
                        };
                        nat_add(a2166, a2167)?
                    };
                    nat_add(a2168, a2169)?
                }))
            }
        }
    }
}

pub fn f387(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2170 = f412(v0.clone());
    match m2170 {
        None => {
            if f36(f139(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2171 = f136(v0, f8(Str::lit("items")));
                match m2171 {
                    None => {
                        Ok(f357(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f413(f8(Str::lit("List")), f414(v3, {
                            let a2172 = v1;
                            let a2173 = f8(Str::lit("Item"));
                            append_bytes(a2172, a2173)
                        })?))
                    }
                }
            } else {
                f414(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f413(f8(Str::lit("Option")), f414(v2, v1)?))
        }
    }
}

pub fn f388(v0: Bytes, v1: u64) -> Bytes {
    let m2177 = {
        let a2174 = v0;
        let a2175 = v1;
        let a2176 = 1u64;
        slice_bytes(a2174, a2175, a2176)
    };
    match m2177 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f389(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f415(v1, v0, v2)
}

pub fn f390(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2222 = {
            let a2198 = {
                let a2186 = {
                    let a2180 = {
                        let a2178 = v3;
                        let a2179 = 128u8;
                        fixed_u8::bit_and(a2178, a2179)
                    };
                    let a2181 = 128u8;
                    if equal(a2180, a2181) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2187 = {
                    let a2184 = {
                        let a2182 = v3;
                        let a2183 = 64u8;
                        fixed_u8::bit_and(a2182, a2183)
                    };
                    let a2185 = 64u8;
                    if equal(a2184, a2185) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2186, a2187)?
            };
            let a2199 = {
                let a2196 = {
                    let a2190 = {
                        let a2188 = v3;
                        let a2189 = 32u8;
                        fixed_u8::bit_and(a2188, a2189)
                    };
                    let a2191 = 32u8;
                    if equal(a2190, a2191) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2197 = {
                    let a2194 = {
                        let a2192 = v3;
                        let a2193 = 16u8;
                        fixed_u8::bit_and(a2192, a2193)
                    };
                    let a2195 = 16u8;
                    if equal(a2194, a2195) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2196, a2197)?
            };
            nat_add(a2198, a2199)?
        };
        let a2223 = {
            let a2220 = {
                let a2208 = {
                    let a2202 = {
                        let a2200 = v3;
                        let a2201 = 8u8;
                        fixed_u8::bit_and(a2200, a2201)
                    };
                    let a2203 = 8u8;
                    if equal(a2202, a2203) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2209 = {
                    let a2206 = {
                        let a2204 = v3;
                        let a2205 = 4u8;
                        fixed_u8::bit_and(a2204, a2205)
                    };
                    let a2207 = 4u8;
                    if equal(a2206, a2207) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2208, a2209)?
            };
            let a2221 = {
                let a2218 = {
                    let a2212 = {
                        let a2210 = v3;
                        let a2211 = 2u8;
                        fixed_u8::bit_and(a2210, a2211)
                    };
                    let a2213 = 2u8;
                    if equal(a2212, a2213) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2219 = {
                    let a2216 = {
                        let a2214 = v3;
                        let a2215 = 1u8;
                        fixed_u8::bit_and(a2214, a2215)
                    };
                    let a2217 = 1u8;
                    if equal(a2216, a2217) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2218, a2219)?
            };
            nat_add(a2220, a2221)?
        };
        nat_add(a2222, a2223)?
    };
    let a2224 = v4;
    let a2225 = 34u64;
    if nat_eq(a2224, a2225) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2226 = v4;
        let a2227 = 92u64;
        if nat_eq(a2226, a2227) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2228 = v4;
            let a2229 = 8u64;
            if nat_eq(a2228, a2229) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2230 = v4;
                let a2231 = 12u64;
                if nat_eq(a2230, a2231) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2232 = v4;
                    let a2233 = 10u64;
                    if nat_eq(a2232, a2233) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2234 = v4;
                        let a2235 = 13u64;
                        if nat_eq(a2234, a2235) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2236 = v4;
                            let a2237 = 9u64;
                            if nat_eq(a2236, a2237) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2238 = v4;
                                let a2239 = 32u64;
                                if nat_lt(a2238, a2239) {
                                    let a2240 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2241 = f416(v2)?;
                                    Ok(append_bytes(a2240, a2241))
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

pub fn f391(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f417(Fn21::F418, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f392(v0: Fn17, p2245: List<List<(Bytes, Bytes)>>, p2246: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2244 = (p2245, p2246);
    {
        let mut m2244 = m2244;
        loop {
            let step = {
                let (v1, v2) = m2244;
                let m2242 = v2;
                match m2242.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2243: Fn17 = v0.clone();
                            c2243.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f394(v0: Fn18, p2251: u64, p2252: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2250 = (p2251, p2252);
    {
        let mut m2250 = m2250;
        loop {
            let step = {
                let (v1, v2) = m2250;
                let m2247 = v1;
                if m2247 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2247 - 1;
                    let m2249 = {
                        let c2248: Fn18 = v0.clone();
                        c2248.apply(v2.clone())?
                    };
                    match m2249 {
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
                Ok(next) => { m2250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f419(v0)
}

pub fn f396(v0: Fn19, p2257: u64, p2258: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2256 = (p2257, p2258);
    {
        let mut m2256 = m2256;
        loop {
            let step = {
                let (v1, v2) = m2256;
                let m2253 = v1;
                if m2253 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2253 - 1;
                    let m2255 = {
                        let c2254: Fn19 = v0.clone();
                        c2254.apply(v2.clone())?
                    };
                    match m2255 {
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
                Ok(next) => { m2256 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f420(v0)
}

pub fn f398(v0: u64) -> u32 {
    let a2259 = v0;
    let a2260 = 32u64;
    if nat_lt(a2259, a2260) {
        let a2261 = v0;
        let a2262 = 16u64;
        if nat_lt(a2261, a2262) {
            let a2263 = v0;
            let a2264 = 8u64;
            if nat_lt(a2263, a2264) {
                let a2265 = v0;
                let a2266 = 4u64;
                if nat_lt(a2265, a2266) {
                    let a2267 = v0;
                    let a2268 = 2u64;
                    if nat_lt(a2267, a2268) {
                        let a2269 = v0;
                        let a2270 = 1u64;
                        if nat_lt(a2269, a2270) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2271 = v0;
                        let a2272 = 3u64;
                        if nat_lt(a2271, a2272) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2273 = v0;
                    let a2274 = 6u64;
                    if nat_lt(a2273, a2274) {
                        let a2275 = v0;
                        let a2276 = 5u64;
                        if nat_lt(a2275, a2276) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2277 = v0;
                        let a2278 = 7u64;
                        if nat_lt(a2277, a2278) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2279 = v0;
                let a2280 = 12u64;
                if nat_lt(a2279, a2280) {
                    let a2281 = v0;
                    let a2282 = 10u64;
                    if nat_lt(a2281, a2282) {
                        let a2283 = v0;
                        let a2284 = 9u64;
                        if nat_lt(a2283, a2284) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2285 = v0;
                        let a2286 = 11u64;
                        if nat_lt(a2285, a2286) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2287 = v0;
                    let a2288 = 14u64;
                    if nat_lt(a2287, a2288) {
                        let a2289 = v0;
                        let a2290 = 13u64;
                        if nat_lt(a2289, a2290) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2291 = v0;
                        let a2292 = 15u64;
                        if nat_lt(a2291, a2292) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2293 = v0;
            let a2294 = 24u64;
            if nat_lt(a2293, a2294) {
                let a2295 = v0;
                let a2296 = 20u64;
                if nat_lt(a2295, a2296) {
                    let a2297 = v0;
                    let a2298 = 18u64;
                    if nat_lt(a2297, a2298) {
                        let a2299 = v0;
                        let a2300 = 17u64;
                        if nat_lt(a2299, a2300) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2301 = v0;
                        let a2302 = 19u64;
                        if nat_lt(a2301, a2302) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2303 = v0;
                    let a2304 = 22u64;
                    if nat_lt(a2303, a2304) {
                        let a2305 = v0;
                        let a2306 = 21u64;
                        if nat_lt(a2305, a2306) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2307 = v0;
                        let a2308 = 23u64;
                        if nat_lt(a2307, a2308) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2309 = v0;
                let a2310 = 28u64;
                if nat_lt(a2309, a2310) {
                    let a2311 = v0;
                    let a2312 = 26u64;
                    if nat_lt(a2311, a2312) {
                        let a2313 = v0;
                        let a2314 = 25u64;
                        if nat_lt(a2313, a2314) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2315 = v0;
                        let a2316 = 27u64;
                        if nat_lt(a2315, a2316) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2317 = v0;
                    let a2318 = 30u64;
                    if nat_lt(a2317, a2318) {
                        let a2319 = v0;
                        let a2320 = 29u64;
                        if nat_lt(a2319, a2320) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2321 = v0;
                        let a2322 = 31u64;
                        if nat_lt(a2321, a2322) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2323 = v0;
        let a2324 = 48u64;
        if nat_lt(a2323, a2324) {
            let a2325 = v0;
            let a2326 = 40u64;
            if nat_lt(a2325, a2326) {
                let a2327 = v0;
                let a2328 = 36u64;
                if nat_lt(a2327, a2328) {
                    let a2329 = v0;
                    let a2330 = 34u64;
                    if nat_lt(a2329, a2330) {
                        let a2331 = v0;
                        let a2332 = 33u64;
                        if nat_lt(a2331, a2332) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2333 = v0;
                        let a2334 = 35u64;
                        if nat_lt(a2333, a2334) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2335 = v0;
                    let a2336 = 38u64;
                    if nat_lt(a2335, a2336) {
                        let a2337 = v0;
                        let a2338 = 37u64;
                        if nat_lt(a2337, a2338) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2339 = v0;
                        let a2340 = 39u64;
                        if nat_lt(a2339, a2340) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2341 = v0;
                let a2342 = 44u64;
                if nat_lt(a2341, a2342) {
                    let a2343 = v0;
                    let a2344 = 42u64;
                    if nat_lt(a2343, a2344) {
                        let a2345 = v0;
                        let a2346 = 41u64;
                        if nat_lt(a2345, a2346) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2347 = v0;
                        let a2348 = 43u64;
                        if nat_lt(a2347, a2348) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2349 = v0;
                    let a2350 = 46u64;
                    if nat_lt(a2349, a2350) {
                        let a2351 = v0;
                        let a2352 = 45u64;
                        if nat_lt(a2351, a2352) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2353 = v0;
                        let a2354 = 47u64;
                        if nat_lt(a2353, a2354) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2355 = v0;
            let a2356 = 56u64;
            if nat_lt(a2355, a2356) {
                let a2357 = v0;
                let a2358 = 52u64;
                if nat_lt(a2357, a2358) {
                    let a2359 = v0;
                    let a2360 = 50u64;
                    if nat_lt(a2359, a2360) {
                        let a2361 = v0;
                        let a2362 = 49u64;
                        if nat_lt(a2361, a2362) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2363 = v0;
                        let a2364 = 51u64;
                        if nat_lt(a2363, a2364) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2365 = v0;
                    let a2366 = 54u64;
                    if nat_lt(a2365, a2366) {
                        let a2367 = v0;
                        let a2368 = 53u64;
                        if nat_lt(a2367, a2368) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2369 = v0;
                        let a2370 = 55u64;
                        if nat_lt(a2369, a2370) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2371 = v0;
                let a2372 = 60u64;
                if nat_lt(a2371, a2372) {
                    let a2373 = v0;
                    let a2374 = 58u64;
                    if nat_lt(a2373, a2374) {
                        let a2375 = v0;
                        let a2376 = 57u64;
                        if nat_lt(a2375, a2376) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2377 = v0;
                        let a2378 = 59u64;
                        if nat_lt(a2377, a2378) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2379 = v0;
                    let a2380 = 62u64;
                    if nat_lt(a2379, a2380) {
                        let a2381 = v0;
                        let a2382 = 61u64;
                        if nat_lt(a2381, a2382) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2383 = v0;
                        let a2384 = 63u64;
                        if nat_lt(a2383, a2384) {
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

pub fn f399(v0: u32) -> u32 {
    let a2387 = {
        let a2385 = f421(v0, 6u32, 26u32);
        let a2386 = f421(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2385, a2386)
    };
    let a2388 = f421(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2387, a2388)
}

pub fn f400(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2394 = {
        let a2389 = v0;
        let a2390 = v1;
        fixed_u32::bit_and(a2389, a2390)
    };
    let a2395 = {
        let a2392 = {
            let a2391 = v0;
            fixed_u32::bit_not(a2391)
        };
        let a2393 = v2;
        fixed_u32::bit_and(a2392, a2393)
    };
    fixed_u32::bit_xor(a2394, a2395)
}

pub fn f401(v0: u32) -> u32 {
    let a2398 = {
        let a2396 = f421(v0, 2u32, 30u32);
        let a2397 = f421(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2396, a2397)
    };
    let a2399 = f421(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2398, a2399)
}

pub fn f402(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2408 = {
        let a2404 = {
            let a2400 = v0;
            let a2401 = v1;
            fixed_u32::bit_and(a2400, a2401)
        };
        let a2405 = {
            let a2402 = v0;
            let a2403 = v2;
            fixed_u32::bit_and(a2402, a2403)
        };
        fixed_u32::bit_xor(a2404, a2405)
    };
    let a2409 = {
        let a2406 = v1;
        let a2407 = v2;
        fixed_u32::bit_and(a2406, a2407)
    };
    fixed_u32::bit_xor(a2408, a2409)
}

pub fn f403(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h2410, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2410
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2411, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2411
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2412, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2412
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2413, _, _, _, _, _, _, _, _, _, _, _) => {
            h2413
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2414, _, _, _, _, _, _, _, _, _, _) => {
            h2414
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2415, _, _, _, _, _, _, _, _, _) => {
            h2415
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2416, _, _, _, _, _, _, _, _) => {
            h2416
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h2417, _, _, _, _, _, _, _) => {
            h2417
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2418, _, _, _, _, _, _) => {
            h2418
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h2419, _, _, _, _, _) => {
            h2419
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h2420, _, _, _, _) => {
            h2420
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2421, _, _, _) => {
            h2421
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2422, _, _) => {
            h2422
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2423, _) => {
            h2423
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2424) => {
            h2424
        }
    }, f348(f422(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2425, _) => {
            h2425
        }
    }), f348(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2426, _, _, _, _, _, _) => {
            h2426
        }
    }, f348(f423(match v0.clone() {
        Adt24::C0(_, h2427, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2427
        }
    }), match v0 {
        Adt24::C0(h2428, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2428
        }
    }))))
}

pub fn f404(v0: Fn20, p2432: u64, p2433: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2431 = (p2432, p2433);
    {
        let mut m2431 = m2431;
        loop {
            let step = {
                let (v1, v2) = m2431;
                let m2429 = v1;
                if m2429 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2429 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2430: Fn20 = v0.clone();
                        c2430.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2434, _) = v0;
        h2434
    };
    let v2: u32 = {
        let (h2435, _) = {
            let (_, h2436) = v0;
            h2436
        };
        h2435
    };
    let v3: u32 = {
        let (_, h2437) = {
            let (_, h2438) = v0;
            h2438
        };
        h2437
    };
    ({
        let a2439 = v1;
        let a2440 = 2u64;
        let a2441 = 0u64;
        nat_quot(a2439, a2440, a2441)
    }, (f406(v2, 1u32), {
        let a2445 = {
            let a2442 = v1;
            let a2443 = 2u64;
            let a2444 = 0u64;
            nat_rem(a2442, a2443, a2444)
        };
        let a2446 = 1u64;
        if nat_eq(a2445, a2446) {
            let a2447 = v3;
            let a2448 = v2;
            fixed_u32::bit_or(a2447, a2448)
        } else {
            v3
        }
    }))
}

pub fn f406(v0: u32, v1: u32) -> u32 {
    let m2451 = {
        let a2449 = v0;
        let a2450 = v1;
        fixed_u32::shift_left(a2449, a2450)
    };
    match m2451 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f407(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f319(v0.clone(), {
        let a2452 = v1;
        let a2453 = 1u64;
        nat_add(a2452, a2453)?
    })?;
    let v4: Bytes = {
        let a2454 = v3;
        let a2455 = 117u64;
        if nat_eq(a2454, a2455) {
            f424(v0, v1)?
        } else {
            f425(v3)
        }
    };
    let a2456 = v4.clone();
    let a2457 = Bytes::lit(&[]);
    if equal(a2456, a2457) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2458 = v3;
            let a2459 = 117u64;
            if nat_eq(a2458, a2459) {
                let a2460 = v1;
                let a2461 = 6u64;
                nat_add(a2460, a2461)?
            } else {
                let a2462 = v1;
                let a2463 = 2u64;
                nat_add(a2462, a2463)?
            }
        }, ({
            let a2464 = v2;
            let a2465 = v4;
            append_bytes(a2464, a2465)
        }, 0u64))))
    }
}

pub fn f408(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f319(v0, v1)?;
    let a2466 = 47u64;
    let a2467 = v2;
    if if nat_lt(a2466, a2467) {
        let a2468 = v2;
        let a2469 = 58u64;
        nat_lt(a2468, a2469)
    } else {
        false
    } {
        Ok(Some({
            let a2470 = v1;
            let a2471 = 1u64;
            nat_add(a2470, a2471)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f409(v0: Bytes) -> R<u64> {
    let (h2472, _) = f222(Fn3::F426(v0.clone()), {
        let a2473 = v0;
        length_bytes(a2473)
    }, 0u64)?;
    Ok(h2472)
}

pub fn f410(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2477 = {
        let a2475 = v0;
        let a2476 = {
            let (h2474, _) = v1;
            h2474
        };
        index_bytes(a2475, a2476)
    };
    match m2477 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2479 = {
                    let (h2478, _) = v1;
                    h2478
                };
                let a2480 = 1u64;
                nat_add(a2479, a2480)?
            }, {
                let a2484 = {
                    let a2482 = {
                        let (_, h2481) = v1;
                        h2481
                    };
                    let a2483 = 10u64;
                    nat_mul(a2482, a2483)?
                };
                let a2485 = f427(v2)?;
                nat_add(a2484, a2485)?
            })))
        }
    }
}

pub fn f411(v0: Bytes) -> R<Option<Bytes>> {
    if f428({
        let a2486 = v0.clone();
        length_bytes(a2486)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f385(v0)?;
        if f386(v1.clone())? {
            Ok(Some(v1))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        f429({
            let a2487 = v0.clone();
            length_bytes(a2487)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f412(v0: Adt14) -> Option<Adt14> {
    let m2488 = f140(f136(v0, f8(Str::lit("oneOf"))));
    match m2488.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2489 = v2;
            match m2489.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2490 = v4;
                    match m2490.uncons() {
                        None => {
                            if f430(v3.clone()) {
                                Some(v1)
                            } else {
                                if f430(v1) {
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

pub fn f413(v0: Bytes, v1: Result<(Bytes, List<Adt23>), Bytes>) -> Result<(Bytes, List<Adt23>), Bytes> {
    let m2491 = v1;
    match m2491 {
        Ok(v3) => {
            f358(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2492, _) = v3;
                h2492
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2493) = v3;
                h2493
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v2)
        }
    }
}

pub fn f414(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2494 = f412(v0.clone());
    match m2494 {
        None => {
            if f36(f139(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2495 = f136(v0, f8(Str::lit("items")));
                match m2495 {
                    None => {
                        Ok(f357(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f413(f8(Str::lit("List")), f431(v3, {
                            let a2496 = v1;
                            let a2497 = f8(Str::lit("Item"));
                            append_bytes(a2496, a2497)
                        })?))
                    }
                }
            } else {
                f431(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f413(f8(Str::lit("Option")), f431(v2, v1)?))
        }
    }
}

pub fn f415(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2498, _) = v2;
        h2498
    };
    let a2499 = v3;
    let a2500 = 0u64;
    if nat_eq(a2499, a2500) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2501 = v3;
            let a2502 = v1;
            let a2503 = 0u64;
            nat_quot(a2501, a2502, a2503)
        }, {
            let a2508 = f388(v0, {
                let a2504 = v3;
                let a2505 = v1;
                let a2506 = 0u64;
                nat_rem(a2504, a2505, a2506)
            });
            let a2509 = {
                let (_, h2507) = v2;
                h2507
            };
            append_bytes(a2508, a2509)
        }))
    }
}

pub fn f416(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2554 = {
            let a2530 = {
                let a2518 = {
                    let a2512 = {
                        let a2510 = v1;
                        let a2511 = 128u8;
                        fixed_u8::bit_and(a2510, a2511)
                    };
                    let a2513 = 128u8;
                    if equal(a2512, a2513) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2519 = {
                    let a2516 = {
                        let a2514 = v1;
                        let a2515 = 64u8;
                        fixed_u8::bit_and(a2514, a2515)
                    };
                    let a2517 = 64u8;
                    if equal(a2516, a2517) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2518, a2519)?
            };
            let a2531 = {
                let a2528 = {
                    let a2522 = {
                        let a2520 = v1;
                        let a2521 = 32u8;
                        fixed_u8::bit_and(a2520, a2521)
                    };
                    let a2523 = 32u8;
                    if equal(a2522, a2523) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2529 = {
                    let a2526 = {
                        let a2524 = v1;
                        let a2525 = 16u8;
                        fixed_u8::bit_and(a2524, a2525)
                    };
                    let a2527 = 16u8;
                    if equal(a2526, a2527) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2528, a2529)?
            };
            nat_add(a2530, a2531)?
        };
        let a2555 = {
            let a2552 = {
                let a2540 = {
                    let a2534 = {
                        let a2532 = v1;
                        let a2533 = 8u8;
                        fixed_u8::bit_and(a2532, a2533)
                    };
                    let a2535 = 8u8;
                    if equal(a2534, a2535) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2541 = {
                    let a2538 = {
                        let a2536 = v1;
                        let a2537 = 4u8;
                        fixed_u8::bit_and(a2536, a2537)
                    };
                    let a2539 = 4u8;
                    if equal(a2538, a2539) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2540, a2541)?
            };
            let a2553 = {
                let a2550 = {
                    let a2544 = {
                        let a2542 = v1;
                        let a2543 = 2u8;
                        fixed_u8::bit_and(a2542, a2543)
                    };
                    let a2545 = 2u8;
                    if equal(a2544, a2545) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2551 = {
                    let a2548 = {
                        let a2546 = v1;
                        let a2547 = 1u8;
                        fixed_u8::bit_and(a2546, a2547)
                    };
                    let a2549 = 1u8;
                    if equal(a2548, a2549) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2550, a2551)?
            };
            nat_add(a2552, a2553)?
        };
        nat_add(a2554, a2555)?
    };
    let a2562 = f388(f432(), {
        let a2556 = v2;
        let a2557 = 16u64;
        let a2558 = 0u64;
        nat_quot(a2556, a2557, a2558)
    });
    let a2563 = f388(f432(), {
        let a2559 = v2;
        let a2560 = 16u64;
        let a2561 = 0u64;
        nat_rem(a2559, a2560, a2561)
    });
    Ok(append_bytes(a2562, a2563))
}

pub fn f417(v0: Fn21, p2567: List<List<(Bytes, Bytes)>>, p2568: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2566 = (p2567, p2568);
    {
        let mut m2566 = m2566;
        loop {
            let step = {
                let (v1, v2) = m2566;
                let m2564 = v2;
                match m2564.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2565: Fn21 = v0.clone();
                            c2565.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2566 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f418(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f419(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2570 = {
        let a2569 = v0.clone();
        length_list(a2569)
    };
    let a2571 = 1u64;
    if nat_le(a2570, a2571) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f433(v0)?))
    }
}

pub fn f420(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2572, _) = v0;
        h2572
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2573) = v0;
        h2573
    };
    let m2574 = v1;
    match m2574.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2575 = v4;
            match m2575.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f434(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f421(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2576 = f313(v0, v1);
    let a2577 = f406(v0, v2);
    fixed_u32::bit_or(a2576, a2577)
}

pub fn f422(v0: u32) -> u32 {
    let a2580 = {
        let a2578 = f421(v0, 17u32, 15u32);
        let a2579 = f421(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2578, a2579)
    };
    let a2581 = f313(v0, 10u32);
    fixed_u32::bit_xor(a2580, a2581)
}

pub fn f423(v0: u32) -> u32 {
    let a2584 = {
        let a2582 = f421(v0, 7u32, 25u32);
        let a2583 = f421(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2582, a2583)
    };
    let a2585 = f313(v0, 3u32);
    fixed_u32::bit_xor(a2584, a2585)
}

pub fn f424(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2588 = f32(v0.clone(), {
        let a2586 = v1;
        let a2587 = 2u64;
        nat_add(a2586, a2587)?
    }, 2u64);
    let a2589 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2588, a2589) {
        let m2592 = f435(f32(v0, {
            let a2590 = v1;
            let a2591 = 4u64;
            nat_add(a2590, a2591)?
        }, 2u64))?;
        match m2592 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f319(v2.clone(), 0u64)?;
                let a2593 = v3;
                let a2594 = 32u64;
                if nat_lt(a2593, a2594) {
                    if f436(v3) {
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

pub fn f425(v0: u64) -> Bytes {
    let a2595 = v0;
    let a2596 = 34u64;
    if nat_eq(a2595, a2596) {
        Bytes::lit(&[0x22])
    } else {
        let a2597 = v0;
        let a2598 = 92u64;
        if nat_eq(a2597, a2598) {
            Bytes::lit(&[0x5c])
        } else {
            let a2599 = v0;
            let a2600 = 98u64;
            if nat_eq(a2599, a2600) {
                Bytes::lit(&[0x08])
            } else {
                let a2601 = v0;
                let a2602 = 102u64;
                if nat_eq(a2601, a2602) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2603 = v0;
                    let a2604 = 110u64;
                    if nat_eq(a2603, a2604) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2605 = v0;
                        let a2606 = 114u64;
                        if nat_eq(a2605, a2606) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2607 = v0;
                            let a2608 = 116u64;
                            if nat_eq(a2607, a2608) {
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

pub fn f426(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2611 = {
        let a2609 = v0;
        let a2610 = v1;
        index_bytes(a2609, a2610)
    };
    match m2611 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f437(v2)? {
                Ok(Some({
                    let a2612 = v1;
                    let a2613 = 1u64;
                    nat_add(a2612, a2613)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f427(v0: u8) -> R<u64> {
    let a2660 = {
        let v1: u8 = v0;
        let a2658 = {
            let a2634 = {
                let a2622 = {
                    let a2616 = {
                        let a2614 = v1;
                        let a2615 = 128u8;
                        fixed_u8::bit_and(a2614, a2615)
                    };
                    let a2617 = 128u8;
                    if equal(a2616, a2617) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2623 = {
                    let a2620 = {
                        let a2618 = v1;
                        let a2619 = 64u8;
                        fixed_u8::bit_and(a2618, a2619)
                    };
                    let a2621 = 64u8;
                    if equal(a2620, a2621) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2622, a2623)?
            };
            let a2635 = {
                let a2632 = {
                    let a2626 = {
                        let a2624 = v1;
                        let a2625 = 32u8;
                        fixed_u8::bit_and(a2624, a2625)
                    };
                    let a2627 = 32u8;
                    if equal(a2626, a2627) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2633 = {
                    let a2630 = {
                        let a2628 = v1;
                        let a2629 = 16u8;
                        fixed_u8::bit_and(a2628, a2629)
                    };
                    let a2631 = 16u8;
                    if equal(a2630, a2631) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2632, a2633)?
            };
            nat_add(a2634, a2635)?
        };
        let a2659 = {
            let a2656 = {
                let a2644 = {
                    let a2638 = {
                        let a2636 = v1;
                        let a2637 = 8u8;
                        fixed_u8::bit_and(a2636, a2637)
                    };
                    let a2639 = 8u8;
                    if equal(a2638, a2639) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2645 = {
                    let a2642 = {
                        let a2640 = v1;
                        let a2641 = 4u8;
                        fixed_u8::bit_and(a2640, a2641)
                    };
                    let a2643 = 4u8;
                    if equal(a2642, a2643) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2644, a2645)?
            };
            let a2657 = {
                let a2654 = {
                    let a2648 = {
                        let a2646 = v1;
                        let a2647 = 2u8;
                        fixed_u8::bit_and(a2646, a2647)
                    };
                    let a2649 = 2u8;
                    if equal(a2648, a2649) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2655 = {
                    let a2652 = {
                        let a2650 = v1;
                        let a2651 = 1u8;
                        fixed_u8::bit_and(a2650, a2651)
                    };
                    let a2653 = 1u8;
                    if equal(a2652, a2653) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2654, a2655)?
            };
            nat_add(a2656, a2657)?
        };
        nat_add(a2658, a2659)?
    };
    let a2661 = 48u64;
    Ok(nat_sub(a2660, a2661))
}

pub fn f428(p2717: u64, v1: Bytes, p2718: u64) -> R<bool> {
    let m2716 = (p2717, p2718);
    {
        let mut m2716 = m2716;
        loop {
            let step = {
                let (v0, v2) = m2716;
                let m2662 = v0;
                if m2662 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2662 - 1;
                    let m2665 = {
                        let a2663 = v1.clone();
                        let a2664 = v2;
                        index_bytes(a2663, a2664)
                    };
                    match m2665 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2712 = {
                                let v5: u8 = v4;
                                let a2710 = {
                                    let a2686 = {
                                        let a2674 = {
                                            let a2668 = {
                                                let a2666 = v5;
                                                let a2667 = 128u8;
                                                fixed_u8::bit_and(a2666, a2667)
                                            };
                                            let a2669 = 128u8;
                                            if equal(a2668, a2669) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2675 = {
                                            let a2672 = {
                                                let a2670 = v5;
                                                let a2671 = 64u8;
                                                fixed_u8::bit_and(a2670, a2671)
                                            };
                                            let a2673 = 64u8;
                                            if equal(a2672, a2673) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2674, a2675)?
                                    };
                                    let a2687 = {
                                        let a2684 = {
                                            let a2678 = {
                                                let a2676 = v5;
                                                let a2677 = 32u8;
                                                fixed_u8::bit_and(a2676, a2677)
                                            };
                                            let a2679 = 32u8;
                                            if equal(a2678, a2679) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2685 = {
                                            let a2682 = {
                                                let a2680 = v5;
                                                let a2681 = 16u8;
                                                fixed_u8::bit_and(a2680, a2681)
                                            };
                                            let a2683 = 16u8;
                                            if equal(a2682, a2683) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2684, a2685)?
                                    };
                                    nat_add(a2686, a2687)?
                                };
                                let a2711 = {
                                    let a2708 = {
                                        let a2696 = {
                                            let a2690 = {
                                                let a2688 = v5;
                                                let a2689 = 8u8;
                                                fixed_u8::bit_and(a2688, a2689)
                                            };
                                            let a2691 = 8u8;
                                            if equal(a2690, a2691) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2697 = {
                                            let a2694 = {
                                                let a2692 = v5;
                                                let a2693 = 4u8;
                                                fixed_u8::bit_and(a2692, a2693)
                                            };
                                            let a2695 = 4u8;
                                            if equal(a2694, a2695) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2696, a2697)?
                                    };
                                    let a2709 = {
                                        let a2706 = {
                                            let a2700 = {
                                                let a2698 = v5;
                                                let a2699 = 2u8;
                                                fixed_u8::bit_and(a2698, a2699)
                                            };
                                            let a2701 = 2u8;
                                            if equal(a2700, a2701) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2707 = {
                                            let a2704 = {
                                                let a2702 = v5;
                                                let a2703 = 1u8;
                                                fixed_u8::bit_and(a2702, a2703)
                                            };
                                            let a2705 = 1u8;
                                            if equal(a2704, a2705) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2706, a2707)?
                                    };
                                    nat_add(a2708, a2709)?
                                };
                                nat_add(a2710, a2711)?
                            };
                            let a2713 = 128u64;
                            if nat_lt(a2712, a2713) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2714 = v2;
                                    let a2715 = 1u64;
                                    nat_add(a2714, a2715)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f429(p2731: u64, v1: Bytes, p2732: u64, p2733: Bytes) -> R<Option<Bytes>> {
    let m2730 = (p2731, (p2732, p2733));
    {
        let mut m2730 = m2730;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2730;
                let m2719 = v0;
                if m2719 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2719 - 1;
                    let a2721 = {
                        let a2720 = v1.clone();
                        length_bytes(a2720)
                    };
                    let a2722 = v2;
                    if nat_le(a2721, a2722) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2723 = f438(v1.clone(), v2)?;
                        match m2723 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2725 = v2;
                                    let a2726 = {
                                        let (_, h2724) = v5;
                                        h2724
                                    };
                                    nat_add(a2725, a2726)?
                                }, {
                                    let a2728 = v3;
                                    let a2729 = f439({
                                        let (h2727, _) = v5;
                                        h2727
                                    });
                                    append_bytes(a2728, a2729)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2730 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f430(v0: Adt14) -> bool {
    f36(f139(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f431(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2734 = f440(f327(v0.clone()));
    match m2734 {
        None => {
            let m2735 = f144(f136(v0.clone(), f8(Str::lit("$ref"))));
            match m2735 {
                None => {
                    let m2737 = f136(v0.clone(), f8(Str::lit("enum")));
                    match m2737 {
                        None => {
                            let m2738 = f136(v0.clone(), f8(Str::lit("const")));
                            match m2738 {
                                None => {
                                    let v8: Bytes = f139(v0.clone(), f8(Str::lit("type")));
                                    if f36(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f358(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                    } else {
                                        if f36(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f358(f442(v0), List::<Adt23>::nil()))
                                        } else {
                                            if f36(v8.clone(), f8(Str::lit("number"))) {
                                                Ok(f358(f443(), List::<Adt23>::nil()))
                                            } else {
                                                if f36(v8.clone(), f8(Str::lit("boolean"))) {
                                                    Ok(f358(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                                } else {
                                                    if f36(v8.clone(), f8(Str::lit("object"))) {
                                                        Ok(f358(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                    } else {
                                                        Ok(f357(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2739 = f144(Some(v6));
                                    match m2739 {
                                        None => {
                                            Ok(f357(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f358(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f358(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2736 = f441(v3)?;
                    match m2736 {
                        None => {
                            Ok(f357(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f358(v4, List::<Adt23>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f357(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f432() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f433(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f391({
        let (_, h2740) = {
            let (h2741, _) = f444(Fn22::F445, {
                let a2743 = {
                    let a2742 = v0.clone();
                    length_list(a2742)
                };
                let a2744 = 1u64;
                nat_add(a2743, a2744)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2741
        };
        h2740
    }))
}

pub fn f434(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2749 = {
            let a2747 = {
                let a2745 = v0.clone();
                length_list(a2745)
            };
            let a2748 = {
                let a2746 = v1.clone();
                length_list(a2746)
            };
            nat_add(a2747, a2748)?
        };
        let a2750 = 1u64;
        nat_add(a2749, a2750)?
    };
    Ok(f303({
        let (_, h2751) = {
            let (_, h2752) = {
                let (h2753, _) = f446(Fn23::F447, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2753
            };
            h2752
        };
        h2751
    }))
}

pub fn f435(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2754 = v0.clone();
        length_bytes(a2754)
    };
    let a2758 = {
        let a2755 = v1;
        let a2756 = 2u64;
        let a2757 = 0u64;
        nat_rem(a2755, a2756, a2757)
    };
    let a2759 = 0u64;
    if nat_eq(a2758, a2759) {
        let (_, h2760) = {
            let (h2761, _) = f448(Fn24::F449(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2761
        };
        Ok(h2760)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f436(v0: u64) -> bool {
    let a2762 = v0;
    let a2763 = 8u64;
    if nat_eq(a2762, a2763) {
        true
    } else {
        let a2764 = v0;
        let a2765 = 9u64;
        if nat_eq(a2764, a2765) {
            true
        } else {
            let a2766 = v0;
            let a2767 = 10u64;
            if nat_eq(a2766, a2767) {
                true
            } else {
                let a2768 = v0;
                let a2769 = 12u64;
                if nat_eq(a2768, a2769) {
                    true
                } else {
                    let a2770 = v0;
                    let a2771 = 13u64;
                    nat_eq(a2770, a2771)
                }
            }
        }
    }
}

pub fn f437(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2816 = {
            let a2792 = {
                let a2780 = {
                    let a2774 = {
                        let a2772 = v1;
                        let a2773 = 128u8;
                        fixed_u8::bit_and(a2772, a2773)
                    };
                    let a2775 = 128u8;
                    if equal(a2774, a2775) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2781 = {
                    let a2778 = {
                        let a2776 = v1;
                        let a2777 = 64u8;
                        fixed_u8::bit_and(a2776, a2777)
                    };
                    let a2779 = 64u8;
                    if equal(a2778, a2779) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2780, a2781)?
            };
            let a2793 = {
                let a2790 = {
                    let a2784 = {
                        let a2782 = v1;
                        let a2783 = 32u8;
                        fixed_u8::bit_and(a2782, a2783)
                    };
                    let a2785 = 32u8;
                    if equal(a2784, a2785) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2791 = {
                    let a2788 = {
                        let a2786 = v1;
                        let a2787 = 16u8;
                        fixed_u8::bit_and(a2786, a2787)
                    };
                    let a2789 = 16u8;
                    if equal(a2788, a2789) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2790, a2791)?
            };
            nat_add(a2792, a2793)?
        };
        let a2817 = {
            let a2814 = {
                let a2802 = {
                    let a2796 = {
                        let a2794 = v1;
                        let a2795 = 8u8;
                        fixed_u8::bit_and(a2794, a2795)
                    };
                    let a2797 = 8u8;
                    if equal(a2796, a2797) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2803 = {
                    let a2800 = {
                        let a2798 = v1;
                        let a2799 = 4u8;
                        fixed_u8::bit_and(a2798, a2799)
                    };
                    let a2801 = 4u8;
                    if equal(a2800, a2801) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2802, a2803)?
            };
            let a2815 = {
                let a2812 = {
                    let a2806 = {
                        let a2804 = v1;
                        let a2805 = 2u8;
                        fixed_u8::bit_and(a2804, a2805)
                    };
                    let a2807 = 2u8;
                    if equal(a2806, a2807) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2813 = {
                    let a2810 = {
                        let a2808 = v1;
                        let a2809 = 1u8;
                        fixed_u8::bit_and(a2808, a2809)
                    };
                    let a2811 = 1u8;
                    if equal(a2810, a2811) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2812, a2813)?
            };
            nat_add(a2814, a2815)?
        };
        nat_add(a2816, a2817)?
    };
    let a2818 = 48u64;
    let a2819 = v2;
    if nat_le(a2818, a2819) {
        let a2820 = v2;
        let a2821 = 57u64;
        Ok(nat_le(a2820, a2821))
    } else {
        Ok(false)
    }
}

pub fn f438(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2824 = {
        let a2822 = v0.clone();
        let a2823 = v1;
        index_bytes(a2822, a2823)
    };
    match m2824 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2869 = {
                    let a2845 = {
                        let a2833 = {
                            let a2827 = {
                                let a2825 = v3;
                                let a2826 = 128u8;
                                fixed_u8::bit_and(a2825, a2826)
                            };
                            let a2828 = 128u8;
                            if equal(a2827, a2828) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2834 = {
                            let a2831 = {
                                let a2829 = v3;
                                let a2830 = 64u8;
                                fixed_u8::bit_and(a2829, a2830)
                            };
                            let a2832 = 64u8;
                            if equal(a2831, a2832) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2833, a2834)?
                    };
                    let a2846 = {
                        let a2843 = {
                            let a2837 = {
                                let a2835 = v3;
                                let a2836 = 32u8;
                                fixed_u8::bit_and(a2835, a2836)
                            };
                            let a2838 = 32u8;
                            if equal(a2837, a2838) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2844 = {
                            let a2841 = {
                                let a2839 = v3;
                                let a2840 = 16u8;
                                fixed_u8::bit_and(a2839, a2840)
                            };
                            let a2842 = 16u8;
                            if equal(a2841, a2842) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2843, a2844)?
                    };
                    nat_add(a2845, a2846)?
                };
                let a2870 = {
                    let a2867 = {
                        let a2855 = {
                            let a2849 = {
                                let a2847 = v3;
                                let a2848 = 8u8;
                                fixed_u8::bit_and(a2847, a2848)
                            };
                            let a2850 = 8u8;
                            if equal(a2849, a2850) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2856 = {
                            let a2853 = {
                                let a2851 = v3;
                                let a2852 = 4u8;
                                fixed_u8::bit_and(a2851, a2852)
                            };
                            let a2854 = 4u8;
                            if equal(a2853, a2854) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2855, a2856)?
                    };
                    let a2868 = {
                        let a2865 = {
                            let a2859 = {
                                let a2857 = v3;
                                let a2858 = 2u8;
                                fixed_u8::bit_and(a2857, a2858)
                            };
                            let a2860 = 2u8;
                            if equal(a2859, a2860) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2866 = {
                            let a2863 = {
                                let a2861 = v3;
                                let a2862 = 1u8;
                                fixed_u8::bit_and(a2861, a2862)
                            };
                            let a2864 = 1u8;
                            if equal(a2863, a2864) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2865, a2866)?
                    };
                    nat_add(a2867, a2868)?
                };
                nat_add(a2869, a2870)?
            };
            let a2871 = v4;
            let a2872 = 128u64;
            if nat_lt(a2871, a2872) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2873 = 194u64;
                let a2874 = v4;
                if if nat_le(a2873, a2874) {
                    let a2875 = v4;
                    let a2876 = 223u64;
                    nat_le(a2875, a2876)
                } else {
                    false
                } {
                    let m2879 = f450(v0, {
                        let a2877 = v1;
                        let a2878 = 1u64;
                        nat_add(a2877, a2878)?
                    })?;
                    match m2879 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f451({
                                let a2884 = {
                                    let a2882 = {
                                        let a2880 = v4;
                                        let a2881 = 192u64;
                                        nat_sub(a2880, a2881)
                                    };
                                    let a2883 = 64u64;
                                    nat_mul(a2882, a2883)?
                                };
                                let a2885 = v5;
                                nat_add(a2884, a2885)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2886 = 224u64;
                    let a2887 = v4;
                    if if nat_le(a2886, a2887) {
                        let a2888 = v4;
                        let a2889 = 239u64;
                        nat_le(a2888, a2889)
                    } else {
                        false
                    } {
                        let m2892 = f450(v0.clone(), {
                            let a2890 = v1;
                            let a2891 = 1u64;
                            nat_add(a2890, a2891)?
                        })?;
                        match m2892 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2895 = f450(v0, {
                                    let a2893 = v1;
                                    let a2894 = 2u64;
                                    nat_add(a2893, a2894)?
                                })?;
                                match m2895 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f451({
                                            let a2904 = {
                                                let a2898 = {
                                                    let a2896 = v4;
                                                    let a2897 = 224u64;
                                                    nat_sub(a2896, a2897)
                                                };
                                                let a2899 = 4096u64;
                                                nat_mul(a2898, a2899)?
                                            };
                                            let a2905 = {
                                                let a2902 = {
                                                    let a2900 = v6;
                                                    let a2901 = 64u64;
                                                    nat_mul(a2900, a2901)?
                                                };
                                                let a2903 = v7;
                                                nat_add(a2902, a2903)?
                                            };
                                            nat_add(a2904, a2905)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2906 = 240u64;
                        let a2907 = v4;
                        if if nat_le(a2906, a2907) {
                            let a2908 = v4;
                            let a2909 = 244u64;
                            nat_le(a2908, a2909)
                        } else {
                            false
                        } {
                            let m2912 = f450(v0.clone(), {
                                let a2910 = v1;
                                let a2911 = 1u64;
                                nat_add(a2910, a2911)?
                            })?;
                            match m2912 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2915 = f450(v0.clone(), {
                                        let a2913 = v1;
                                        let a2914 = 2u64;
                                        nat_add(a2913, a2914)?
                                    })?;
                                    match m2915 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m2918 = f450(v0, {
                                                let a2916 = v1;
                                                let a2917 = 3u64;
                                                nat_add(a2916, a2917)?
                                            })?;
                                            match m2918 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f451({
                                                        let a2931 = {
                                                            let a2921 = {
                                                                let a2919 = v4;
                                                                let a2920 = 240u64;
                                                                nat_sub(a2919, a2920)
                                                            };
                                                            let a2922 = 262144u64;
                                                            nat_mul(a2921, a2922)?
                                                        };
                                                        let a2932 = {
                                                            let a2929 = {
                                                                let a2923 = v8;
                                                                let a2924 = 4096u64;
                                                                nat_mul(a2923, a2924)?
                                                            };
                                                            let a2930 = {
                                                                let a2927 = {
                                                                    let a2925 = v9;
                                                                    let a2926 = 64u64;
                                                                    nat_mul(a2925, a2926)?
                                                                };
                                                                let a2928 = v10;
                                                                nat_add(a2927, a2928)?
                                                            };
                                                            nat_add(a2929, a2930)?
                                                        };
                                                        nat_add(a2931, a2932)?
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

pub fn f439(v0: u64) -> Bytes {
    let a2933 = f8(Str::lit("u"));
    let a2934 = f452(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a2933, a2934)
}

pub fn f440(p2940: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2939 = p2940;
    {
        let mut m2939 = m2939;
        loop {
            let step = {
                let v0 = m2939;
                let m2935 = v0;
                match m2935.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f84({
                            let (h2936, _) = v1.clone();
                            h2936
                        }, f453()) {
                            true
                        } else {
                            f84({
                                let (h2937, _) = v1.clone();
                                h2937
                            }, f454())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2938, _) = v1;
                                h2938
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2939 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: Bytes) -> R<Option<Bytes>> {
    if f186(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f210(f32(v0.clone(), 8u64, {
            let a2942 = {
                let a2941 = v0;
                length_bytes(a2941)
            };
            let a2943 = 8u64;
            nat_sub(a2942, a2943)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f442(v0: Adt14) -> Bytes {
    let m2944 = f295(f136(v0, f8(Str::lit("minimum"))));
    match m2944 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f443() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f444(v0: Fn22, p2949: u64, p2950: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2948 = (p2949, p2950);
    {
        let mut m2948 = m2948;
        loop {
            let step = {
                let (v1, v2) = m2948;
                let m2945 = v1;
                if m2945 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2945 - 1;
                    let m2947 = {
                        let c2946: Fn22 = v0.clone();
                        c2946.apply(v2.clone())?
                    };
                    match m2947 {
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
                Ok(next) => { m2948 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f455(v0)
}

pub fn f446(v0: Fn23, p2955: u64, p2956: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2954 = (p2955, p2956);
    {
        let mut m2954 = m2954;
        loop {
            let step = {
                let (v1, v2) = m2954;
                let m2951 = v1;
                if m2951 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2951 - 1;
                    let m2953 = {
                        let c2952: Fn23 = v0.clone();
                        c2952.apply(v2.clone())
                    };
                    match m2953 {
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
                Ok(next) => { m2954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f456(v0)
}

pub fn f448(v0: Fn24, p2961: u64, p2962: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2960 = (p2961, p2962);
    {
        let mut m2960 = m2960;
        loop {
            let step = {
                let (v1, v2) = m2960;
                let m2957 = v1;
                if m2957 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2957 - 1;
                    let m2959 = {
                        let c2958: Fn24 = v0.clone();
                        c2958.apply(v2.clone())?
                    };
                    match m2959 {
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
                Ok(next) => { m2960 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f457(v0, v1)
}

pub fn f450(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2965 = {
        let a2963 = v0;
        let a2964 = v1;
        index_bytes(a2963, a2964)
    };
    match m2965 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3010 = {
                    let a2986 = {
                        let a2974 = {
                            let a2968 = {
                                let a2966 = v3;
                                let a2967 = 128u8;
                                fixed_u8::bit_and(a2966, a2967)
                            };
                            let a2969 = 128u8;
                            if equal(a2968, a2969) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2975 = {
                            let a2972 = {
                                let a2970 = v3;
                                let a2971 = 64u8;
                                fixed_u8::bit_and(a2970, a2971)
                            };
                            let a2973 = 64u8;
                            if equal(a2972, a2973) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2974, a2975)?
                    };
                    let a2987 = {
                        let a2984 = {
                            let a2978 = {
                                let a2976 = v3;
                                let a2977 = 32u8;
                                fixed_u8::bit_and(a2976, a2977)
                            };
                            let a2979 = 32u8;
                            if equal(a2978, a2979) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2985 = {
                            let a2982 = {
                                let a2980 = v3;
                                let a2981 = 16u8;
                                fixed_u8::bit_and(a2980, a2981)
                            };
                            let a2983 = 16u8;
                            if equal(a2982, a2983) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2984, a2985)?
                    };
                    nat_add(a2986, a2987)?
                };
                let a3011 = {
                    let a3008 = {
                        let a2996 = {
                            let a2990 = {
                                let a2988 = v3;
                                let a2989 = 8u8;
                                fixed_u8::bit_and(a2988, a2989)
                            };
                            let a2991 = 8u8;
                            if equal(a2990, a2991) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2997 = {
                            let a2994 = {
                                let a2992 = v3;
                                let a2993 = 4u8;
                                fixed_u8::bit_and(a2992, a2993)
                            };
                            let a2995 = 4u8;
                            if equal(a2994, a2995) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2996, a2997)?
                    };
                    let a3009 = {
                        let a3006 = {
                            let a3000 = {
                                let a2998 = v3;
                                let a2999 = 2u8;
                                fixed_u8::bit_and(a2998, a2999)
                            };
                            let a3001 = 2u8;
                            if equal(a3000, a3001) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3007 = {
                            let a3004 = {
                                let a3002 = v3;
                                let a3003 = 1u8;
                                fixed_u8::bit_and(a3002, a3003)
                            };
                            let a3005 = 1u8;
                            if equal(a3004, a3005) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3006, a3007)?
                    };
                    nat_add(a3008, a3009)?
                };
                nat_add(a3010, a3011)?
            };
            let a3012 = 128u64;
            let a3013 = v4;
            if if nat_le(a3012, a3013) {
                let a3014 = v4;
                let a3015 = 191u64;
                nat_le(a3014, a3015)
            } else {
                false
            } {
                Ok(Some({
                    let a3016 = v4;
                    let a3017 = 128u64;
                    nat_sub(a3016, a3017)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f451(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3018 = v1;
    let a3019 = v0;
    if if nat_le(a3018, a3019) {
        let a3020 = v0;
        let a3021 = v2;
        if nat_le(a3020, a3021) {
            let a3022 = v0;
            let a3023 = 55296u64;
            if nat_lt(a3022, a3023) {
                true
            } else {
                let a3024 = 57343u64;
                let a3025 = v0;
                nat_lt(a3024, a3025)
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

pub fn f452(p3042: u64, p3043: u64, p3044: u64, p3045: Bytes) -> Bytes {
    let m3041 = (p3042, (p3043, (p3044, p3045)));
    {
        let mut m3041 = m3041;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3041;
                let m3026 = v0;
                if m3026 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3026 - 1;
                    let a3027 = v1;
                    let a3028 = 0u64;
                    if if nat_eq(a3027, a3028) {
                        let a3029 = v2;
                        let a3030 = 0u64;
                        nat_eq(a3029, a3030)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3031 = v1;
                            let a3032 = 16u64;
                            let a3033 = 0u64;
                            nat_quot(a3031, a3032, a3033)
                        }, ({
                            let a3034 = v2;
                            let a3035 = 1u64;
                            nat_sub(a3034, a3035)
                        }, {
                            let a3039 = f458({
                                let a3036 = v1;
                                let a3037 = 16u64;
                                let a3038 = 0u64;
                                nat_rem(a3036, a3037, a3038)
                            });
                            let a3040 = v3;
                            append_bytes(a3039, a3040)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f454() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f455(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3046, _) = v0;
        h3046
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3047) = v0;
        h3047
    };
    let m3048 = v1;
    match m3048.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3049 = v4;
            match m3049.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f459(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f456(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3050, _) = v0.clone();
        h3050
    };
    let v2: List<(Bytes, ())> = {
        let (h3051, _) = {
            let (_, h3052) = v0.clone();
            h3052
        };
        h3051
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3053) = {
            let (_, h3054) = v0;
            h3054
        };
        h3053
    };
    let m3055 = v1.clone();
    match m3055.uncons() {
        None => {
            let m3056 = v2;
            match m3056.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3057 = v2.clone();
            match m3057.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f460({
                        let (h3058, _) = v8.clone();
                        h3058
                    }, {
                        let (h3059, _) = v6.clone();
                        h3059
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

pub fn f457(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3060, _) = v1;
        h3060
    };
    let m3062 = {
        let (_, h3061) = v1;
        h3061
    };
    match m3062 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3065 = {
                let a3063 = v0.clone();
                let a3064 = v2;
                index_bytes(a3063, a3064)
            };
            match m3065 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3070 = {
                        let a3068 = v0;
                        let a3069 = {
                            let a3066 = v2;
                            let a3067 = 1u64;
                            nat_add(a3066, a3067)?
                        };
                        index_bytes(a3068, a3069)
                    };
                    match m3070 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f461(v4)? {
                                f461(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3071 = v2;
                                    let a3072 = 2u64;
                                    nat_add(a3071, a3072)?
                                }, Some({
                                    let a3077 = v3;
                                    let a3078 = f388(f462(), {
                                        let a3075 = {
                                            let a3073 = f463(v4)?;
                                            let a3074 = 16u64;
                                            nat_mul(a3073, a3074)?
                                        };
                                        let a3076 = f463(v5)?;
                                        nat_add(a3075, a3076)?
                                    });
                                    append_bytes(a3077, a3078)
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

pub fn f458(v0: u64) -> Bytes {
    f32({
        let a3079 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3079)
    }, v0, 1u64)
}

pub fn f459(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3084 = {
            let a3082 = {
                let a3080 = v0.clone();
                length_list(a3080)
            };
            let a3083 = {
                let a3081 = v1.clone();
                length_list(a3081)
            };
            nat_add(a3082, a3083)?
        };
        let a3085 = 1u64;
        nat_add(a3084, a3085)?
    };
    Ok(f464({
        let (_, h3086) = {
            let (_, h3087) = {
                let (h3088, _) = f465(Fn25::F466, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3088
            };
            h3087
        };
        h3086
    }))
}

pub fn f460(v0: Bytes, v1: Bytes) -> bool {
    let a3093 = {
        let a3089 = v0;
        let a3090 = v1;
        compare_bytes(a3089, a3090)
    };
    let a3094 = {
        let a3091 = Bytes::lit(&[]);
        let a3092 = Bytes::lit(&[0x01]);
        compare_bytes(a3091, a3092)
    };
    equal(a3093, a3094)
}

pub fn f461(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3139 = {
            let a3115 = {
                let a3103 = {
                    let a3097 = {
                        let a3095 = v1;
                        let a3096 = 128u8;
                        fixed_u8::bit_and(a3095, a3096)
                    };
                    let a3098 = 128u8;
                    if equal(a3097, a3098) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3104 = {
                    let a3101 = {
                        let a3099 = v1;
                        let a3100 = 64u8;
                        fixed_u8::bit_and(a3099, a3100)
                    };
                    let a3102 = 64u8;
                    if equal(a3101, a3102) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3103, a3104)?
            };
            let a3116 = {
                let a3113 = {
                    let a3107 = {
                        let a3105 = v1;
                        let a3106 = 32u8;
                        fixed_u8::bit_and(a3105, a3106)
                    };
                    let a3108 = 32u8;
                    if equal(a3107, a3108) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3114 = {
                    let a3111 = {
                        let a3109 = v1;
                        let a3110 = 16u8;
                        fixed_u8::bit_and(a3109, a3110)
                    };
                    let a3112 = 16u8;
                    if equal(a3111, a3112) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3113, a3114)?
            };
            nat_add(a3115, a3116)?
        };
        let a3140 = {
            let a3137 = {
                let a3125 = {
                    let a3119 = {
                        let a3117 = v1;
                        let a3118 = 8u8;
                        fixed_u8::bit_and(a3117, a3118)
                    };
                    let a3120 = 8u8;
                    if equal(a3119, a3120) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3126 = {
                    let a3123 = {
                        let a3121 = v1;
                        let a3122 = 4u8;
                        fixed_u8::bit_and(a3121, a3122)
                    };
                    let a3124 = 4u8;
                    if equal(a3123, a3124) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3125, a3126)?
            };
            let a3138 = {
                let a3135 = {
                    let a3129 = {
                        let a3127 = v1;
                        let a3128 = 2u8;
                        fixed_u8::bit_and(a3127, a3128)
                    };
                    let a3130 = 2u8;
                    if equal(a3129, a3130) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3136 = {
                    let a3133 = {
                        let a3131 = v1;
                        let a3132 = 1u8;
                        fixed_u8::bit_and(a3131, a3132)
                    };
                    let a3134 = 1u8;
                    if equal(a3133, a3134) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3135, a3136)?
            };
            nat_add(a3137, a3138)?
        };
        nat_add(a3139, a3140)?
    };
    let a3141 = 48u64;
    let a3142 = v2;
    if if nat_le(a3141, a3142) {
        let a3143 = v2;
        let a3144 = 57u64;
        nat_le(a3143, a3144)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3145 = 97u64;
        let a3146 = v2;
        if nat_le(a3145, a3146) {
            let a3147 = v2;
            let a3148 = 102u64;
            Ok(nat_le(a3147, a3148))
        } else {
            Ok(false)
        }
    }
}

pub fn f462() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f463(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3193 = {
            let a3169 = {
                let a3157 = {
                    let a3151 = {
                        let a3149 = v1;
                        let a3150 = 128u8;
                        fixed_u8::bit_and(a3149, a3150)
                    };
                    let a3152 = 128u8;
                    if equal(a3151, a3152) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3158 = {
                    let a3155 = {
                        let a3153 = v1;
                        let a3154 = 64u8;
                        fixed_u8::bit_and(a3153, a3154)
                    };
                    let a3156 = 64u8;
                    if equal(a3155, a3156) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3157, a3158)?
            };
            let a3170 = {
                let a3167 = {
                    let a3161 = {
                        let a3159 = v1;
                        let a3160 = 32u8;
                        fixed_u8::bit_and(a3159, a3160)
                    };
                    let a3162 = 32u8;
                    if equal(a3161, a3162) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3168 = {
                    let a3165 = {
                        let a3163 = v1;
                        let a3164 = 16u8;
                        fixed_u8::bit_and(a3163, a3164)
                    };
                    let a3166 = 16u8;
                    if equal(a3165, a3166) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3167, a3168)?
            };
            nat_add(a3169, a3170)?
        };
        let a3194 = {
            let a3191 = {
                let a3179 = {
                    let a3173 = {
                        let a3171 = v1;
                        let a3172 = 8u8;
                        fixed_u8::bit_and(a3171, a3172)
                    };
                    let a3174 = 8u8;
                    if equal(a3173, a3174) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3180 = {
                    let a3177 = {
                        let a3175 = v1;
                        let a3176 = 4u8;
                        fixed_u8::bit_and(a3175, a3176)
                    };
                    let a3178 = 4u8;
                    if equal(a3177, a3178) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3179, a3180)?
            };
            let a3192 = {
                let a3189 = {
                    let a3183 = {
                        let a3181 = v1;
                        let a3182 = 2u8;
                        fixed_u8::bit_and(a3181, a3182)
                    };
                    let a3184 = 2u8;
                    if equal(a3183, a3184) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3190 = {
                    let a3187 = {
                        let a3185 = v1;
                        let a3186 = 1u8;
                        fixed_u8::bit_and(a3185, a3186)
                    };
                    let a3188 = 1u8;
                    if equal(a3187, a3188) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3189, a3190)?
            };
            nat_add(a3191, a3192)?
        };
        nat_add(a3193, a3194)?
    };
    let a3195 = v2;
    let a3196 = 57u64;
    if nat_le(a3195, a3196) {
        let a3197 = v2;
        let a3198 = 48u64;
        Ok(nat_sub(a3197, a3198))
    } else {
        let a3199 = v2;
        let a3200 = 87u64;
        Ok(nat_sub(a3199, a3200))
    }
}

pub fn f464(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f333(Fn10::F467, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f465(v0: Fn25, p3205: u64, p3206: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3204 = (p3205, p3206);
    {
        let mut m3204 = m3204;
        loop {
            let step = {
                let (v1, v2) = m3204;
                let m3201 = v1;
                if m3201 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3201 - 1;
                    let m3203 = {
                        let c3202: Fn25 = v0.clone();
                        c3202.apply(v2.clone())
                    };
                    match m3203 {
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
                Ok(next) => { m3204 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f468(v0)
}

pub fn f467(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f468(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3207, _) = v0.clone();
        h3207
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3208, _) = {
            let (_, h3209) = v0.clone();
            h3209
        };
        h3208
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3210) = {
            let (_, h3211) = v0;
            h3211
        };
        h3210
    };
    let m3212 = v1.clone();
    match m3212.uncons() {
        None => {
            let m3213 = v2;
            match m3213.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3214 = v2.clone();
            match m3214.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f460({
                        let (h3215, _) = v8.clone();
                        h3215
                    }, {
                        let (h3216, _) = v6.clone();
                        h3216
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
