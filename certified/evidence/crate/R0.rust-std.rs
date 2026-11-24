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
    F174(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F55(k0, k1) => f55(*k0, k1.clone(), p0),
            Fn0::F174(k0) => f174(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F152,
    F170,
    F236,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F152 => f152(p0, p1),
            Fn1::F170 => f170(p0, p1),
            Fn1::F236 => f236(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F203(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F203(k0) => f203(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F225(Bytes, u64, Bytes, u64),
    F315(Bytes),
    F316(u64, Bytes),
    F380(Bytes),
    F427(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F225(k0, k1, k2, k3) => f225(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F315(k0) => Ok(f315(k0.clone(), p0)),
            Fn3::F316(k0, k1) => f316(*k0, k1.clone(), p0),
            Fn3::F380(k0) => f380(k0.clone(), p0),
            Fn3::F427(k0) => f427(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F267(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt22)) -> R<(u64, Adt22)> {
        match self {
            Fn4::F267(k0, k1, k2) => f267(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F296,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn5::F296 => f296(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F299,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn6::F299 => f299(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F301,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn7::F301 => f301(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F304,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn8::F304 => f304(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F333(Bytes),
    F389(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F333(k0) => f333(k0.clone(), p0),
            Fn9::F389(k0, k1) => Ok(f389(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F335,
    F472,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F335 => f335(p0, p1),
            Fn10::F472 => f472(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F338,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F338 => f338(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F341,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F341 => f341(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F344,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F344 => f344(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F349,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn14::F349 => f349(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F354(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F354(k0) => f354(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F383(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F383(k0) => f383(k0.clone(), p0),
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
    F419,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F419 => f419(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F444,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F444 => f444(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F446,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F446 => f446(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F448(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F448(k0) => f448(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F471,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F471 => f471(p0),
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
            let m275 = f62(f58(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f88(v0.clone())?);
            match m275.uncons() {
                None => {
                    let v5: Adt15 = f100(v0.clone())?;
                    let v6: List<Adt14> = f71(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f62(f101(f102(match v5.clone() {
                        Adt15::C0(h276, _, _) => {
                            h276
                        }
                    }, f103(f71(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(v6.clone())), f105(f102(match v5.clone() {
                        Adt15::C0(h277, _, _) => {
                            h277
                        }
                    }, f103(f71(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f106(v5.clone(), v6, f107(v5, f108(v0.clone(), f109(v0)?)?))?)))))
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
            let v5: List<List<Bytes>> = f110(v4.clone())?;
            if f73(f111(f112(v5.clone()), f113())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f114(Str::lit("id"), v6.clone())?, List::cons(f114(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f114(Str::lit("artifact"), f115())?, List::cons(f114(Str::lit("checksum"), f116(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f115(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f117(f68(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f118(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f119(f120(f121(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
            let m280 = f122(f81(f68(v0, v1))?)?;
            match m280 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f119(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
            let v5: Bytes = f123(if v2 {
                f124()
            } else {
                f125()
            }, f126(v1.clone())?);
            if f73(f127(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m282 = f128(f81(f68(v0.clone(), v1.clone()))?, v2)?;
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
            if f73(f127(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f129(v5)?
                } else {
                    f130(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f123(f131(), f126(v1.clone())?), f68(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f123(f131(), f126(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f132()
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
    let m287 = f58(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m287.uncons() {
        None => {
            let v4: List<Bytes> = f81(f68(v0.clone(), v1))?;
            let m288 = f133(v4.clone(), f8(Str::lit("requirement")))?;
            match m288 {
                None => {
                    Ok(Adt11::C4(f132()))
                }
                Some(v5) => {
                    let m289 = f133(v4, f8(Str::lit("admission")))?;
                    match m289 {
                        None => {
                            Ok(Adt11::C4(f132()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f71(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f71(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f62(f70(v5.clone(), f8(Str::lit("id"))), f70(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f76(f62(f70(v7.clone(), f8(Str::lit("id"))), f70(v8.clone(), f8(Str::lit("construct")))))?;
                            let m290 = f134(f104(v6.clone()), f103(f71(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m290 {
                                None => {
                                    let a292 = {
                                        let a291 = v9.clone();
                                        length_list(a291)
                                    };
                                    let a293 = 0u64;
                                    if nat_eq(a292, a293) {
                                        Ok(Adt11::C4(f132()))
                                    } else {
                                        if f73({
                                            let a296 = {
                                                let a294 = f76(f62(v9.clone(), v10.clone()))?;
                                                length_list(a294)
                                            };
                                            let a297 = {
                                                let a295 = f62(v9.clone(), v10);
                                                length_list(a295)
                                            };
                                            nat_eq(a296, a297)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f136(f8(Str::lit("model/requirements.json")), f137(v7, v5))?, List::cons(f136(f8(Str::lit("model/admissions.json")), f137(v8, v6))?, List::cons(f118(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f119(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f135(v11))
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
    let m298 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m298.uncons() {
        None => {
            let m299 = f138(f81(f68(v0, v1))?)?;
            match m299 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f119(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f48(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m300 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m300.uncons() {
        None => {
            let m301 = f139(v0.clone(), v1.clone())?;
            match m301 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m302 = f140(v4.clone(), f8(Str::lit("schema")));
                    match m302 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f141(v0, f142(v5.clone())?, v5, f143(v4.clone(), f8(Str::lit("name"))), f72(f144(f140(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
    let m303 = f58(v0.clone(), List::cons(v1.clone(), List::cons(f87(), List::<Bytes>::nil())));
    match m303.uncons() {
        None => {
            let m304 = f145(f81(f68(v0.clone(), v1))?)?;
            match m304 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f71(v0, f87())?;
                    let v6: List<Bytes> = f70(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f76(f70(v5.clone(), f8(Str::lit("at"))))?;
                    let a306 = {
                        let a305 = v6.clone();
                        length_list(a305)
                    };
                    let a307 = 0u64;
                    if if nat_eq(a306, a307) {
                        true
                    } else {
                        f73({
                            let a310 = {
                                let a308 = f76(f62(v6.clone(), v7.clone()))?;
                                length_list(a308)
                            };
                            let a311 = {
                                let a309 = f62(v6.clone(), v7);
                                length_list(a309)
                            };
                            nat_eq(a310, a311)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f136(f87(), f137(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f119(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m312 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m312.uncons() {
        None => {
            let m313 = f139(v0, v1)?;
            match m313 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f146(f144(Some(v4))) {
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
    let m314 = f58(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m314.uncons() {
        None => {
            let m315 = f139(v0.clone(), v1.clone())?;
            match m315 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f73(f36(f143(v4.clone(), f8(Str::lit("schema"))), f147())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f143(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m316 = f140(v4.clone(), f8(Str::lit("source")));
                            match m316 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f148(f140(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f123(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f123(v9.clone(), f8(Str::lit("binding.json"))), f68(v0.clone(), v1));
                        let m317 = v8;
                        match m317 {
                            None => {
                                f149(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f73(f36(f143(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f149(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f123(v9, f126(f143(v7.clone(), f8(Str::lit("url"))))?);
                                    if f73(f84(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h318) => {
                                            h318
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f143(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m319 = f58(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m319.uncons() {
                                            None => {
                                                if f36(f116(f68(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f149(v0, v4, v5, v10, Some((v12, v11)))
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
    let m320 = f58(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m320.uncons() {
        None => {
            let m321 = f139(v0.clone(), v1.clone())?;
            match m321 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f141(v0.clone(), f68(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
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

pub fn f54(v0: Fn0, p326: u64, p327: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m325 = (p326, p327);
    {
        let mut m325 = m325;
        loop {
            let step = {
                let (v1, v2) = m325;
                let m322 = v1;
                if m322 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m322 - 1;
                    let m324 = {
                        let c323: Fn0 = v0.clone();
                        c323.apply(v2.clone())?
                    };
                    match m324 {
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
                Ok(next) => { m325 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f55(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f150(v1, v0, v2)
}

pub fn f56(v0: List<Bytes>) -> List<Bytes> {
    f151(Fn1::F152, List::<Bytes>::nil(), v0)
}

pub fn f57(v0: Bytes, p383: u64, p384: u64) -> R<bool> {
    let m382 = (p383, p384);
    {
        let mut m382 = m382;
        loop {
            let step = {
                let (v1, v2) = m382;
                let m328 = v2;
                if m328 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m328 - 1;
                    let m331 = {
                        let a329 = v0.clone();
                        let a330 = v1;
                        index_bytes(a329, a330)
                    };
                    match m331 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a378 = {
                                let v5: u8 = v4;
                                let a376 = {
                                    let a352 = {
                                        let a340 = {
                                            let a334 = {
                                                let a332 = v5;
                                                let a333 = 128u8;
                                                fixed_u8::bit_and(a332, a333)
                                            };
                                            let a335 = 128u8;
                                            if equal(a334, a335) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a341 = {
                                            let a338 = {
                                                let a336 = v5;
                                                let a337 = 64u8;
                                                fixed_u8::bit_and(a336, a337)
                                            };
                                            let a339 = 64u8;
                                            if equal(a338, a339) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a340, a341)?
                                    };
                                    let a353 = {
                                        let a350 = {
                                            let a344 = {
                                                let a342 = v5;
                                                let a343 = 32u8;
                                                fixed_u8::bit_and(a342, a343)
                                            };
                                            let a345 = 32u8;
                                            if equal(a344, a345) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a351 = {
                                            let a348 = {
                                                let a346 = v5;
                                                let a347 = 16u8;
                                                fixed_u8::bit_and(a346, a347)
                                            };
                                            let a349 = 16u8;
                                            if equal(a348, a349) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a350, a351)?
                                    };
                                    nat_add(a352, a353)?
                                };
                                let a377 = {
                                    let a374 = {
                                        let a362 = {
                                            let a356 = {
                                                let a354 = v5;
                                                let a355 = 8u8;
                                                fixed_u8::bit_and(a354, a355)
                                            };
                                            let a357 = 8u8;
                                            if equal(a356, a357) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a363 = {
                                            let a360 = {
                                                let a358 = v5;
                                                let a359 = 4u8;
                                                fixed_u8::bit_and(a358, a359)
                                            };
                                            let a361 = 4u8;
                                            if equal(a360, a361) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a362, a363)?
                                    };
                                    let a375 = {
                                        let a372 = {
                                            let a366 = {
                                                let a364 = v5;
                                                let a365 = 2u8;
                                                fixed_u8::bit_and(a364, a365)
                                            };
                                            let a367 = 2u8;
                                            if equal(a366, a367) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a373 = {
                                            let a370 = {
                                                let a368 = v5;
                                                let a369 = 1u8;
                                                fixed_u8::bit_and(a368, a369)
                                            };
                                            let a371 = 1u8;
                                            if equal(a370, a371) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a372, a373)?
                                    };
                                    nat_add(a374, a375)?
                                };
                                nat_add(a376, a377)?
                            };
                            let a379 = 127u64;
                            if nat_le(a378, a379) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a380 = v1;
                                    let a381 = 1u64;
                                    nat_add(a380, a381)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m382 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m385 = v1;
    match m385.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m387 = f153(match v0.clone() {
                Adt1::C0(h386, _, _) => {
                    h386
                }
            }, v2.clone());
            match m387 {
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
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f115(), List::<Bytes>::nil())))))))
}

pub fn f60(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m388 = v1;
    match m388.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f154(v0.clone(), v2.clone()) {
                f127(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m389 = f139(v0.clone(), v2.clone())?;
                match m389 {
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
    let m390 = v1;
    match m390.uncons() {
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
    let m391 = v0;
    match m391.uncons() {
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
    let m392 = v0;
    match m392.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f62(f155({
                let (h393, _) = v2;
                h393
            }, {
                let (_, h394) = v2;
                h394
            }, v1), f64(v3, v1))
        }
    }
}

pub fn f65(v0: Adt1, p398: List<Bytes>) -> R<Option<Bytes>> {
    let m397 = p398;
    {
        let mut m397 = m397;
        loop {
            let step = {
                let v1 = m397;
                let m395 = v1;
                match m395.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m396 = f156(f68(v0.clone(), v2.clone()), f157())?;
                        match m396 {
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
                Ok(next) => { m397 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h399, _, _, _, _) => {
            h399
        }
    };
    let v2: Adt16 = f158(v1.clone())?;
    let v3: List<Bytes> = f159(v1.clone());
    let v4: List<Bytes> = f70(match v0.clone() {
        Adt13::C0(_, _, h400, _, _) => {
            h400
        }
    }, f8(Str::lit("path")));
    let m402 = f160(v2, List::<u64>::nil(), f161(v1.clone(), match v0.clone() {
        Adt13::C0(_, h401, _, _, _) => {
            h401
        }
    }, 0u64)?);
    match m402 {
        None => {
            if f163(f164(match v0.clone() {
                Adt13::C0(_, _, _, _, h404) => {
                    h404
                }
            }, f62(match v0.clone() {
                Adt13::C0(_, _, _, _, h405) => {
                    h405
                }
            }, v4.clone()))?, f164(v4.clone(), f62(match v0.clone() {
                Adt13::C0(_, _, _, _, h406) => {
                    h406
                }
            }, v4.clone()))?, f165(match v0.clone() {
                Adt13::C0(_, _, h407, _, _) => {
                    h407
                }
            })) {
                let m411 = f166(f164(match v0.clone() {
                    Adt13::C0(_, _, _, h408, _) => {
                        h408
                    }
                }, f62(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h409, _) => {
                        h409
                    }
                }))?, f164(v3.clone(), f62(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h410, _) => {
                        h410
                    }
                }))?);
                match m411 {
                    None => {
                        let m414 = f167(f70(match v0 {
                            Adt13::C0(_, h413, _, _, _) => {
                                h413
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m414 {
                            None => {
                                let m415 = f167(v4, List::<Bytes>::nil());
                                match m415 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f119(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f162(f62(v3, match v0 {
                            Adt13::C0(_, _, _, h412, _) => {
                                h412
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f162(match v1 {
                Adt12::C0(_, _, _, _, h403) => {
                    h403
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f67(v0: Bytes) -> R<List<Bytes>> {
    f168(f81(v0)?, f8(Str::lit("id = \"")))
}

pub fn f68(v0: Adt1, v1: Bytes) -> Bytes {
    let m417 = f153(match v0 {
        Adt1::C0(h416, _, _) => {
            h416
        }
    }, v1);
    match m417 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m418 = v2;
            match m418 {
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
    let m419 = f110(f68(v0, f115()))?;
    match m419.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f120(v2))
        }
    }
}

pub fn f70(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m420 = v0;
    match m420.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f143(v2, v1.clone()), f70(v3, v1))
        }
    }
}

pub fn f71(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f144(f139(v0, v1)?))
}

pub fn f72(v0: List<Adt14>) -> List<Bytes> {
    let m421 = v0;
    match m421.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m422 = f148(Some(v1));
            match m422 {
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

pub fn f74(p426: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m425 = p426;
    {
        let mut m425 = m425;
        loop {
            let step = {
                let v0 = m425;
                let m423 = v0;
                match m423.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h424, _) = v2;
                            h424
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m425 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f76(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f169(f151(Fn1::F170, List::<Bytes>::nil(), f171(v0)?)))
}

pub fn f77(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m427 = v1;
    match m427.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f127(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f123(v0.clone(), v2), f77(v0, v3))
            } else {
                f77(v0, v3)
            }
        }
    }
}

pub fn f78(p432: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m431 = p432;
    {
        let mut m431 = m431;
        loop {
            let step = {
                let v0 = m431;
                let m428 = v0;
                match m428.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h429, _) = v2;
                            h429
                        }, v1.clone()) {
                            let (_, h430) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h430)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: Adt1, v1: List<List<Bytes>>, p435: List<Bytes>) -> R<Option<Bytes>> {
    let m434 = p435;
    {
        let mut m434 = m434;
        loop {
            let step = {
                let v2 = m434;
                let m433 = v2;
                match m433.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f172(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
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

pub fn f80(p438: List<Bytes>, p439: List<Bytes>, p440: List<List<Bytes>>) -> List<List<Bytes>> {
    let m437 = (p438, (p439, p440));
    {
        let mut m437 = m437;
        loop {
            let step = {
                let (v0, (v1, v2)) = m437;
                let m436 = v0;
                match m436.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f173(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f36(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f173(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
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

pub fn f81(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a441 = v0.clone();
        length_bytes(a441)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h442, _) = f54(Fn0::F174(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h442
    };
    let v3: u64 = {
        let (h443, _) = {
            let (_, h444) = v2.clone();
            h444
        };
        h443
    };
    let v4: List<Bytes> = {
        let (_, h445) = {
            let (_, h446) = v2;
            h446
        };
        h445
    };
    let a447 = v3;
    let a448 = v1;
    if nat_lt(a447, a448) {
        Ok(f56(List::cons(f32(v0, v3, {
            let a449 = v1;
            let a450 = v3;
            nat_sub(a449, a450)
        }), v4)))
    } else {
        Ok(f56(v4))
    }
}

pub fn f82(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m451 = v0;
    match m451.uncons() {
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
        Adt1::C0(_, h452, _) => {
            h452
        }
    }, f175())) {
        Ok(Adt11::C1(List::cons(f175(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f82(f76(f77(f175(), f78(match v0.clone() {
            Adt1::C0(_, h453, _) => {
                h453
            }
        }, f175())))?, v1.clone());
        let m454 = f58(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m454.uncons() {
            None => {
                let m455 = f176(v0.clone(), v2, f70(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m455 {
                    None => {
                        f177(v0, v1)
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

pub fn f84(v0: Bytes, p458: List<Bytes>) -> bool {
    let m457 = p458;
    {
        let mut m457 = m457;
        loop {
            let step = {
                let v1 = m457;
                let m456 = v1;
                match m456.uncons() {
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
                Ok(next) => { m457 = next; }
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
        Adt1::C0(_, h459, _) => {
            h459
        }
    }, f125()) {
        if f74(match v0 {
            Adt1::C0(_, h460, _) => {
                h460
            }
        }, f124()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f124(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f125(), List::<Bytes>::nil())
    }
}

pub fn f87() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f88(v0: Adt1) -> R<List<Bytes>> {
    Ok(f58(v0.clone(), List::cons(f115(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f109(v0)?)))))
}

pub fn f89(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m461 = v0;
    match m461.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f89(v3, v1))
        }
    }
}

pub fn f90(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m462 = f71(v0.clone(), f87())?;
    match m462.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f178(v0)?), List::<(Bytes, Bytes)>::nil()))
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
    let v3: List<Adt14> = f107(v1.clone(), f108(v0.clone(), f109(v0.clone())?)?);
    let v4: List<List<Bytes>> = f121(f110(f68(v0, f115()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f179(v4.clone(), f120(v4), f180(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f106(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f97(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f181(f100(v0.clone())?, f108(v0.clone(), f109(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f98(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f182(f71(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f183(f71(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f99() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f100(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f69(v0.clone())?, f67(f68(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f70(f71(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f101(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m463 = v0;
    match m463.uncons() {
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

pub fn f102(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m464 = v0;
    match m464.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84(v2.clone(), v1.clone()) {
                f102(v3, v1)
            } else {
                List::cons(v2, f102(v3, v1))
            }
        }
    }
}

pub fn f103(v0: List<Adt14>) -> List<Bytes> {
    let m465 = v0;
    match m465.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f62(f72(f144(f140(v1, f8(Str::lit("defers"))))), f103(v2))
        }
    }
}

pub fn f104(v0: List<Adt14>) -> List<Bytes> {
    let m466 = v0;
    match m466.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m467 = f140(v1, f8(Str::lit("std")));
            match m467 {
                None => {
                    f104(v2)
                }
                Some(v3) => {
                    List::cons(f143(v3, f8(Str::lit("row"))), f104(v2))
                }
            }
        }
    }
}

pub fn f105(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m468 = v0;
    match m468.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84(v2.clone(), v1.clone()) {
                f105(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f105(v3, v1))
            }
        }
    }
}

pub fn f106(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f89(f184(v2.clone()), f185(f180(v0, v1.clone(), v2)?));
    f186(f76(f187(v1))?, v3.clone(), {
        let a470 = {
            let a469 = v3;
            length_list(a469)
        };
        let a471 = 1u64;
        nat_add(a470, a471)?
    })
}

pub fn f107(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m472 = v1;
    match m472.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f188(v2.clone(), f8(Str::lit("subject"))) {
                f189(v0.clone(), v2.clone())
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

pub fn f108(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m473 = v1;
    match m473.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m474 = f128(f81(f68(v0.clone(), v2.clone()))?, f190(v2, f124()))?;
            match m474 {
                None => {
                    f108(v0, v3)
                }
                Some(v4) => {
                    Ok(f137(v4, f108(v0, v3)?))
                }
            }
        }
    }
}

pub fn f109(v0: Adt1) -> R<List<Bytes>> {
    Ok(f62(f76(f191(f125(), f78(match v0.clone() {
        Adt1::C0(_, h475, _) => {
            h475
        }
    }, f125())))?, f76(f191(f124(), f78(match v0 {
        Adt1::C0(_, h476, _) => {
            h476
        }
    }, f124())))?))
}

pub fn f110(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f192(v0.clone(), 0u64, {
        let a477 = v0;
        length_bytes(a477)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a480 = {
            let a479 = match v1.clone() {
                Adt17::C0(_, _, h478, _, _) => {
                    h478
                }
            };
            length_bytes(a479)
        };
        let a481 = 0u64;
        if if nat_eq(a480, a481) {
            let m483 = match v1.clone() {
                Adt17::C0(_, _, _, h482, _) => {
                    h482
                }
            };
            match m483.uncons() {
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
            f193(v1)
        }
    };
    Ok(f194(match v4 {
        Adt17::C0(_, _, _, _, h484) => {
            h484
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f111(p489: List<Bytes>, p490: List<Bytes>) -> bool {
    let m488 = (p489, p490);
    {
        let mut m488 = m488;
        loop {
            let step = {
                let (v0, v1) = m488;
                let m485 = v0;
                match m485.uncons() {
                    None => {
                        let m486 = v1;
                        match m486.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m487 = v1;
                        match m487.uncons() {
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
                Ok(next) => { m488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: List<List<Bytes>>) -> List<Bytes> {
    let m491 = v0;
    match m491.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f113() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f114(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f195(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f115() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f116(v0: Bytes) -> R<Bytes> {
    Ok(f196(f197(v0)?))
}

pub fn f117(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f198(f80(f81(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f199(f200(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f118(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f142(Adt14::C3(std::rc::Rc::new(f201(f76(f62(v1, f72(f71(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f119(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m492 = f202(Fn2::F203(v1), None::<Bytes>, v0);
    match m492 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f120(v0: List<List<Bytes>>) -> List<Bytes> {
    let m493 = v0;
    match m493.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m494 = v1;
            match m494.uncons() {
                None => {
                    f120(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f120(v2))
                }
            }
        }
    }
}

pub fn f121(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m495 = v0;
    match m495.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f122(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m496 = v0;
    match m496.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f204(v1)?;
            let a498 = {
                let a497 = v3.clone();
                length_bytes(a497)
            };
            let a499 = 0u64;
            if nat_eq(a498, a499) {
                f122(v2)
            } else {
                let m500 = f205(v3.clone())?;
                match m500 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f143(v4.clone(), f8(Str::lit("id")));
                        if if f36(f143(v4, f8(Str::lit("schema"))), f147()) {
                            f73({
                                let a502 = {
                                    let a501 = v5.clone();
                                    length_bytes(a501)
                                };
                                let a503 = 0u64;
                                nat_eq(a502, a503)
                            })
                        } else {
                            false
                        } {
                            let m504 = f122(v2)?;
                            match m504 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f206(v5), {
                                        let a505 = v3;
                                        let a506 = Bytes::lit(&[0x0a]);
                                        append_bytes(a505, a506)
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

pub fn f123(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f124() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f125() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f126(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f127(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a507 = v0.clone();
        length_bytes(a507)
    };
    let v3: u64 = {
        let a508 = v1.clone();
        length_bytes(a508)
    };
    let a509 = v3;
    let a510 = v2;
    if nat_le(a509, a510) {
        let m516 = {
            let a513 = v0;
            let a514 = {
                let a511 = v2;
                let a512 = v3;
                nat_sub(a511, a512)
            };
            let a515 = v3;
            slice_bytes(a513, a514, a515)
        };
        match m516 {
            None => {
                false
            }
            Some(v4) => {
                let a517 = v4;
                let a518 = v1;
                equal(a517, a518)
            }
        }
    } else {
        false
    }
}

pub fn f128(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m519 = v0;
    match m519.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f204(v2)?;
            let a521 = {
                let a520 = v4.clone();
                length_bytes(a520)
            };
            let a522 = 0u64;
            if nat_eq(a521, a522) {
                f128(v3, v1)
            } else {
                let m523 = f205(v4)?;
                match m523 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f207(v5.clone())
                        } else {
                            f208(v5.clone())
                        } {
                            let m524 = f128(v3, v1)?;
                            match m524 {
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

pub fn f129(v0: List<Bytes>) -> R<bool> {
    if f209(f145(v0.clone())?) {
        Ok(f73({
            let a526 = {
                let a525 = f210(f145(v0)?);
                length_list(a525)
            };
            let a527 = 0u64;
            nat_eq(a526, a527)
        }))
    } else {
        Ok(false)
    }
}

pub fn f130(v0: List<Bytes>) -> R<bool> {
    if f209(f133(v0.clone(), f8(Str::lit("requirement")))?) {
        if f209(f133(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f73({
                let a529 = {
                    let a528 = f137(f210(f133(v0.clone(), f8(Str::lit("requirement")))?), f210(f133(v0, f8(Str::lit("admission")))?));
                    length_list(a528)
                };
                let a530 = 0u64;
                nat_eq(a529, a530)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f131() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f132() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f133(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m531 = v0;
    match m531.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f204(v2)?;
            let a533 = {
                let a532 = v4.clone();
                length_bytes(a532)
            };
            let a534 = 0u64;
            if nat_eq(a533, a534) {
                f133(v3, v1)
            } else {
                let m535 = f205(v4)?;
                match m535 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f211(v5.clone());
                        let a537 = {
                            let a536 = v6.clone();
                            length_bytes(a536)
                        };
                        let a538 = 0u64;
                        if nat_eq(a537, a538) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m539 = f133(v3, v1.clone())?;
                            match m539 {
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

pub fn f134(p542: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m541 = p542;
    {
        let mut m541 = m541;
        loop {
            let step = {
                let v0 = m541;
                let m540 = v0;
                match m540.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f84(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m541 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f136(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f142(Adt14::C3(std::rc::Rc::new(f212(v1))))?))
}

pub fn f137(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m543 = v0;
    match m543.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f137(v3, v1))
        }
    }
}

pub fn f138(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m544 = v0;
    match m544.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f204(v1)?;
            let a546 = {
                let a545 = v3.clone();
                length_bytes(a545)
            };
            let a547 = 0u64;
            if nat_eq(a546, a547) {
                f138(v2)
            } else {
                let m548 = f205(v3.clone())?;
                match m548 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f213(v4.clone()) {
                            let m549 = f138(v2)?;
                            match m549 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f175(), List::cons(f8(Str::lit("/")), List::cons(f214(f143(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a550 = v3;
                                        let a551 = Bytes::lit(&[0x0a]);
                                        append_bytes(a550, a551)
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

pub fn f139(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f205(f68(v0, v1))
}

pub fn f140(p557: Adt14, v1: Bytes) -> Option<Adt14> {
    let m556 = p557;
    {
        let mut m556 = m556;
        loop {
            let step = {
                let v0 = m556;
                let m552 = v0;
                match m552 {
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
                    Adt14::C4(r553) => {
                        let v5 = (*r553).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r554, r555) => {
                        let v3 = (*r554).clone();
                        let v4 = (*r555).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
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

pub fn f141(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m558 = f58(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m558.uncons() {
        None => {
            let m560 = f134(f104(List::cons(Adt14::C4(std::rc::Rc::new({
                let m559 = f215(v4.clone());
                match m559 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f103(f71(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m560 {
                None => {
                    f216(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f135(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f142(v0: Adt14) -> R<Bytes> {
    let a561 = f217(v0)?;
    let a562 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a561, a562))
}

pub fn f143(v0: Adt14, v1: Bytes) -> Bytes {
    let m563 = f148(f140(v0, v1));
    match m563 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f144(v0: Option<Adt14>) -> List<Adt14> {
    let m564 = v0;
    match m564 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f218(v1)
        }
    }
}

pub fn f145(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m565 = v0;
    match m565.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f204(v1)?;
            let a567 = {
                let a566 = v3.clone();
                length_bytes(a566)
            };
            let a568 = 0u64;
            if nat_eq(a567, a568) {
                f145(v2)
            } else {
                let m569 = f205(v3)?;
                match m569 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f188(v4.clone(), f8(Str::lit("agent"))) {
                            if f188(v4.clone(), f8(Str::lit("at"))) {
                                if f188(v4.clone(), f8(Str::lit("quote"))) {
                                    f219(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m570 = f145(v2)?;
                            match m570 {
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

pub fn f146(p573: List<Adt14>) -> bool {
    let m572 = p573;
    {
        let mut m572 = m572;
        loop {
            let step = {
                let v0 = m572;
                let m571 = v0;
                match m571.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f143(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f84(f8(Str::lit("studio")), f70(f220(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m572 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f148(v0: Option<Adt14>) -> Option<Bytes> {
    let m574 = v0;
    match m574 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m575 = v1;
            match m575 {
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

pub fn f149(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f117(f68(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f221(v1, v4)?)?), List::cons(f118(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f150(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h576, _) = v2.clone();
        h576
    };
    let v4: u64 = {
        let (h577, _) = {
            let (_, h578) = v2.clone();
            h578
        };
        h577
    };
    let v5: List<Bytes> = {
        let (_, h579) = {
            let (_, h580) = v2;
            h580
        };
        h579
    };
    let m581 = f222(v0.clone(), v3);
    match m581 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a582 = v6;
            let a583 = v1;
            if equal(a582, a583) {
                Ok(Some(({
                    let a584 = v3;
                    let a585 = 1u64;
                    nat_add(a584, a585)?
                }, ({
                    let a586 = v3;
                    let a587 = 1u64;
                    nat_add(a586, a587)?
                }, List::cons(f32(v0, v4, {
                    let a588 = v3;
                    let a589 = v4;
                    nat_sub(a588, a589)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a590 = v3;
                    let a591 = 1u64;
                    nat_add(a590, a591)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f151(v0: Fn1, p595: List<Bytes>, p596: List<Bytes>) -> List<Bytes> {
    let m594 = (p595, p596);
    {
        let mut m594 = m594;
        loop {
            let step = {
                let (v1, v2) = m594;
                let m592 = v2;
                match m592.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c593: Fn1 = v0.clone();
                            c593.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m594 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f153(p601: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m600 = p601;
    {
        let mut m600 = m600;
        loop {
            let step = {
                let v0 = m600;
                let m597 = v0;
                match m597.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h598, _) = v2;
                            h598
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h599) = v2;
                                h599
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: Adt1, v1: Bytes) -> bool {
    let m603 = f153(match v0 {
        Adt1::C0(h602, _, _) => {
            h602
        }
    }, v1);
    match m603 {
        None => {
            false
        }
        Some(v2) => {
            let m604 = v2;
            match m604 {
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

pub fn f155(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m605 = v1;
    match m605.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f223(v3.clone())
            } else {
                f73(f223(v3.clone()))
            } {
                List::cons(f123(v0.clone(), v3), f155(v0, v4, v2))
            } else {
                f155(v0, v4, v2)
            }
        }
    }
}

pub fn f156(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a606 = v0.clone();
        length_bytes(a606)
    };
    let v3: u64 = {
        let a607 = v1.clone();
        length_bytes(a607)
    };
    let v4: u64 = {
        let (h608, _) = f224(Fn3::F225(v1.clone(), v3, v0.clone(), v2), {
            let a609 = v2;
            let a610 = 1u64;
            nat_add(a609, a610)?
        }, 0u64)?;
        h608
    };
    if f226(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f157() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f158(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f227(match v0.clone() {
        Adt12::C0(h611, _, _, _, _) => {
            h611
        }
    }, 0u64)?, f227(match v0.clone() {
        Adt12::C0(_, h612, _, _, _) => {
            h612
        }
    }, 0u64)?, f227(match v0.clone() {
        Adt12::C0(_, _, h613, _, _) => {
            h613
        }
    }, 0u64)?, f227(match v0 {
        Adt12::C0(_, _, _, h614, _) => {
            h614
        }
    }, 0u64)?))
}

pub fn f159(v0: Adt12) -> List<Bytes> {
    f62(match v0.clone() {
        Adt12::C0(h615, _, _, _, _) => {
            h615
        }
    }, f62(match v0.clone() {
        Adt12::C0(_, _, h616, _, _) => {
            h616
        }
    }, f62(match v0.clone() {
        Adt12::C0(_, _, _, h617, _) => {
            h617
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h618) => {
            h618
        }
    })))
}

pub fn f160(v0: Adt16, p624: List<u64>, p625: List<Adt18>) -> Option<u64> {
    let m623 = (p624, p625);
    {
        let mut m623 = m623;
        loop {
            let step = {
                let (v1, v2) = m623;
                let m619 = v2;
                match m619.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f228(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h620) => {
                                h620
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h621, _) => {
                                    h621
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h622, _) => {
                                    h622
                                }
                            }))
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

pub fn f161(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m626 = v1;
    match m626.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f229(v0.clone(), v3)?), f161(v0, v4, {
                let a627 = v2;
                let a628 = 1u64;
                nat_add(a627, a628)?
            })?))
        }
    }
}

pub fn f162(p632: List<Bytes>, p633: u64) -> Bytes {
    let m631 = (p632, p633);
    {
        let mut m631 = m631;
        loop {
            let step = {
                let (v0, v1) = m631;
                let m629 = v0;
                match m629.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m630 = v1;
                        if m630 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m630 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f230(v0.clone(), v1.clone()) {
        if f230(v1, v0) {
            f231(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f164(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m634 = v0;
    match m634.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f232(v2, v1.clone())?, f164(v3, v1)?))
        }
    }
}

pub fn f165(v0: List<Adt14>) -> List<u64> {
    let m635 = v0;
    match m635.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f233(v1), f165(v2))
        }
    }
}

pub fn f166(p638: List<u64>, v1: List<u64>) -> Option<u64> {
    let m637 = p638;
    {
        let mut m637 = m637;
        loop {
            let step = {
                let v0 = m637;
                let m636 = v0;
                match m636.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f234(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f167(p641: List<Bytes>, p642: List<Bytes>) -> Option<Bytes> {
    let m640 = (p641, p642);
    {
        let mut m640 = m640;
        loop {
            let step = {
                let (v0, v1) = m640;
                let m639 = v0;
                match m639.uncons() {
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
                Ok(next) => { m640 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m643 = v0;
    match m643.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m644 = f235(v2, v1.clone())?;
            match m644 {
                None => {
                    f168(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f168(v3, v1)?))
                }
            }
        }
    }
}

pub fn f169(v0: List<Bytes>) -> List<Bytes> {
    f151(Fn1::F236, List::<Bytes>::nil(), v0)
}

pub fn f170(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f237(v0, v1)
}

pub fn f171(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f238(f239(f240(v0))?))
}

pub fn f172(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m645 = f139(v0, v2)?;
    match m645 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f241(v1, f143(v3.clone(), f8(Str::lit("id"))))?;
            let m646 = v4.clone();
            match m646.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m648 = f148(f140({
                        let m647 = f140(v3.clone(), f8(Str::lit("source")));
                        match m647 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m648 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f73(f84(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f195(f242(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f173(p651: List<Bytes>, p652: List<Bytes>) -> List<Bytes> {
    let m650 = (p651, p652);
    {
        let mut m650 = m650;
        loop {
            let step = {
                let (v0, v1) = m650;
                let m649 = v0;
                match m649.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f243(v0, v1)
}

pub fn f175() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f176(v0: Adt1, p656: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m655 = p656;
    {
        let mut m655 = m655;
        loop {
            let step = {
                let v1 = m655;
                let m653 = v1;
                match m653.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f84(f244(f143({
                            let m654 = f139(v0.clone(), v3.clone())?;
                            match m654 {
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
                Ok(next) => { m655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f73(f74(match v0.clone() {
        Adt1::C0(_, h657, _) => {
            h657
        }
    }, f131())) {
        Ok(Adt11::C1(List::cons(f131(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f82(f76(f245(f78(match v0.clone() {
            Adt1::C0(_, h658, _) => {
                h658
            }
        }, f131())))?, v1);
        let m659 = f58(v0.clone(), List::cons(f87(), List::cons(f115(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m659.uncons() {
            None => {
                let v5: (bool, Bytes) = f246(v0.clone(), v2)?;
                let (h660, _) = v5;
                if h660 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h661) = v5;
                        h661
                    }))
                } else {
                    let m662 = f86(v0.clone());
                    match m662.uncons() {
                        None => {
                            let m663 = f88(v0.clone())?;
                            match m663.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h664) = v5;
                                        h664
                                    }, List::cons(f7(f76(f247(f100(v0.clone())?, f108(v0.clone(), f109(v0.clone())?)?))?), List::cons(f248(v0)?, List::<Bytes>::nil()))))))
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

pub fn f178(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f249(f71(v0, f87())?)?)))
}

pub fn f179(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt21>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m665 = v1;
    match m665.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f62(f250(v5.clone(), v2.clone()), f251(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f252(v0.clone(), v5.clone()), List::cons(if f84(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a667 = {
                    let a666 = v7.clone();
                    length_list(a666)
                };
                let a668 = 0u64;
                if nat_eq(a667, a668) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f119(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f179(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f180(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt21>> {
    let v3: List<(Bytes, Bytes)> = f253(v1);
    let v4: List<(Bytes, Bytes)> = f254(v0, v2);
    f255(v3.clone(), f89(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f181(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m669 = v1;
    match m669.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f188(v2.clone(), f8(Str::lit("domain"))) {
                f256(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f143(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f143(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m670 = f140(v2.clone(), f8(Str::lit("role")));
                    match m670 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f143(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f143(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f143(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f257(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m671 = f140(v2.clone(), f8(Str::lit("origin")));
                    match m671 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f143(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m672 = f140(v2, f8(Str::lit("evidence")));
                    match m672 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f143(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f181(v0, v3))
            } else {
                f181(v0, v3)
            }
        }
    }
}

pub fn f182(v0: List<Adt14>) -> List<Bytes> {
    let m673 = v0;
    match m673.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f143(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f143(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f143(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f182(v2))
        }
    }
}

pub fn f183(v0: List<Adt14>) -> List<Bytes> {
    let m674 = v0;
    match m674.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f143(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f258(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f183(v2))
        }
    }
}

pub fn f184(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m675 = v0;
    match m675.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f143(v1.clone(), f8(Str::lit("subject"))), f143(v1, f8(Str::lit("object")))), f184(v2))
        }
    }
}

pub fn f185(v0: List<Adt21>) -> List<(Bytes, Bytes)> {
    let m676 = v0;
    match m676.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt21::C0(h677, _, _) => {
                    h677
                }
            }, match v1 {
                Adt21::C0(_, _, h678) => {
                    h678
                }
            }), f185(v2))
        }
    }
}

pub fn f186(p685: List<Bytes>, v1: List<(Bytes, Bytes)>, p686: u64) -> R<List<Bytes>> {
    let m684 = (p685, p686);
    {
        let mut m684 = m684;
        loop {
            let step = {
                let (v0, v2) = m684;
                let m679 = v2;
                if m679 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m679 - 1;
                    let v4: List<Bytes> = f76(f62(v0.clone(), f259(v0.clone(), v1.clone())))?;
                    let a682 = {
                        let a680 = v4.clone();
                        length_list(a680)
                    };
                    let a683 = {
                        let a681 = v0.clone();
                        length_list(a681)
                    };
                    if nat_eq(a682, a683) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m684 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: List<Adt14>) -> List<Bytes> {
    let m687 = v0;
    match m687.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m688 = f140(v1, f8(Str::lit("std")));
            match m688 {
                None => {
                    f187(v2)
                }
                Some(v3) => {
                    List::cons(f143(v3.clone(), f8(Str::lit("authority"))), if f188(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f143(v3, f8(Str::lit("row"))), f187(v2))
                    } else {
                        f187(v2)
                    })
                }
            }
        }
    }
}

pub fn f188(v0: Adt14, v1: Bytes) -> bool {
    f73({
        let a690 = {
            let a689 = f143(v0, v1);
            length_bytes(a689)
        };
        let a691 = 0u64;
        nat_eq(a690, a691)
    })
}

pub fn f189(v0: Adt15, v1: Adt14) -> bool {
    if f260(v0.clone(), v1.clone(), f143(v1.clone(), f8(Str::lit("subject")))) {
        f260(v0, v1.clone(), f143(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f190(v0: Bytes, v1: Bytes) -> bool {
    let m696 = {
        let a693 = v0;
        let a694 = 0u64;
        let a695 = {
            let a692 = v1.clone();
            length_bytes(a692)
        };
        slice_bytes(a693, a694, a695)
    };
    match m696 {
        None => {
            false
        }
        Some(v2) => {
            let a697 = v2;
            let a698 = v1;
            equal(a697, a698)
        }
    }
}

pub fn f191(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m699 = v1;
    match m699.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f127(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f123(v0.clone(), v2), f191(v0, v3))
            } else {
                f191(v0, v3)
            }
        }
    }
}

pub fn f192(v0: Bytes, p757: u64, p758: u64, p759: Adt17) -> R<Adt17> {
    let m756 = (p757, (p758, p759));
    {
        let mut m756 = m756;
        loop {
            let step = {
                let (v1, (v2, v3)) = m756;
                let m700 = v2;
                if m700 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m700 - 1;
                    let v5: u64 = f261(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h701, _, _, _) => {
                            h701
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a702 = v1;
                            let a703 = 1u64;
                            nat_add(a702, a703)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h704, _, _) => {
                                h704
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h705, _) => {
                                h705
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h706) => {
                                h706
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h707, _, _, _, _) => {
                                h707
                            }
                        } {
                            let a708 = v5;
                            let a709 = 34u64;
                            if nat_eq(a708, a709) {
                                let a712 = f261(v0.clone(), {
                                    let a710 = v1;
                                    let a711 = 1u64;
                                    nat_add(a710, a711)?
                                })?;
                                let a713 = 34u64;
                                if nat_eq(a712, a713) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a714 = v1;
                                        let a715 = 1u64;
                                        nat_add(a714, a715)?
                                    }, (v4, Adt17::C0(true, true, f262(match v3.clone() {
                                        Adt17::C0(_, _, h716, _, _) => {
                                            h716
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h717, _) => {
                                            h717
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h718) => {
                                            h718
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a719 = v1;
                                        let a720 = 1u64;
                                        nat_add(a719, a720)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h721, _, _) => {
                                            h721
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h722, _) => {
                                            h722
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h723) => {
                                            h723
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a724 = v1;
                                    let a725 = 1u64;
                                    nat_add(a724, a725)?
                                }, (v4, Adt17::C0(true, false, f262(match v3.clone() {
                                    Adt17::C0(_, _, h726, _, _) => {
                                        h726
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h727, _) => {
                                        h727
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h728) => {
                                        h728
                                    }
                                }))))
                            }
                        } else {
                            let a729 = v5;
                            let a730 = 34u64;
                            if nat_eq(a729, a730) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a731 = v1;
                                    let a732 = 1u64;
                                    nat_add(a731, a732)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h733, _, _) => {
                                        h733
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h734, _) => {
                                        h734
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h735) => {
                                        h735
                                    }
                                }))))
                            } else {
                                let a736 = v5;
                                let a737 = 44u64;
                                if nat_eq(a736, a737) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a738 = v1;
                                        let a739 = 1u64;
                                        nat_add(a738, a739)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h740, _, _) => {
                                            h740
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h741, _) => {
                                            h741
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h742) => {
                                            h742
                                        }
                                    }))))
                                } else {
                                    let a743 = v5;
                                    let a744 = 13u64;
                                    if nat_eq(a743, a744) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a745 = v1;
                                            let a746 = 1u64;
                                            nat_add(a745, a746)?
                                        }, (v4, v3)))
                                    } else {
                                        let a747 = v5;
                                        let a748 = 10u64;
                                        if nat_eq(a747, a748) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a749 = v1;
                                                let a750 = 1u64;
                                                nat_add(a749, a750)?
                                            }, (v4, f193(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a751 = v1;
                                                let a752 = 1u64;
                                                nat_add(a751, a752)?
                                            }, (v4, Adt17::C0(false, false, f262(match v3.clone() {
                                                Adt17::C0(_, _, h753, _, _) => {
                                                    h753
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h754, _) => {
                                                    h754
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h755) => {
                                                    h755
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
                Ok(next) => { m756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f263(List::cons(match v0.clone() {
        Adt17::C0(_, _, h760, _, _) => {
            h760
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h761, _) => {
            h761
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h762) => {
            h762
        }
    }))
}

pub fn f194(p765: List<List<Bytes>>, p766: List<List<Bytes>>) -> List<List<Bytes>> {
    let m764 = (p765, p766);
    {
        let mut m764 = m764;
        loop {
            let step = {
                let (v0, v1) = m764;
                let m763 = v0;
                match m763.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
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

pub fn f195(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f264(v0.clone(), 0u64, {
        let a767 = v0;
        length_bytes(a767)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f196(v0: Adt22) -> Bytes {
    let a788 = f265(match v0.clone() {
        Adt22::C0(h768, _, _, _, _, _, _, _) => {
            h768
        }
    });
    let a789 = {
        let a786 = f265(match v0.clone() {
            Adt22::C0(_, h769, _, _, _, _, _, _) => {
                h769
            }
        });
        let a787 = {
            let a784 = f265(match v0.clone() {
                Adt22::C0(_, _, h770, _, _, _, _, _) => {
                    h770
                }
            });
            let a785 = {
                let a782 = f265(match v0.clone() {
                    Adt22::C0(_, _, _, h771, _, _, _, _) => {
                        h771
                    }
                });
                let a783 = {
                    let a780 = f265(match v0.clone() {
                        Adt22::C0(_, _, _, _, h772, _, _, _) => {
                            h772
                        }
                    });
                    let a781 = {
                        let a778 = f265(match v0.clone() {
                            Adt22::C0(_, _, _, _, _, h773, _, _) => {
                                h773
                            }
                        });
                        let a779 = {
                            let a776 = f265(match v0.clone() {
                                Adt22::C0(_, _, _, _, _, _, h774, _) => {
                                    h774
                                }
                            });
                            let a777 = f265(match v0 {
                                Adt22::C0(_, _, _, _, _, _, _, h775) => {
                                    h775
                                }
                            });
                            append_bytes(a776, a777)
                        };
                        append_bytes(a778, a779)
                    };
                    append_bytes(a780, a781)
                };
                append_bytes(a782, a783)
            };
            append_bytes(a784, a785)
        };
        append_bytes(a786, a787)
    };
    append_bytes(a788, a789)
}

pub fn f197(v0: Bytes) -> R<Adt22> {
    let v1: u64 = {
        let a790 = v0.clone();
        length_bytes(a790)
    };
    let v2: u64 = {
        let a796 = {
            let a793 = {
                let a791 = v1;
                let a792 = 72u64;
                nat_add(a791, a792)?
            };
            let a794 = 64u64;
            let a795 = 0u64;
            nat_quot(a793, a794, a795)
        };
        let a797 = 64u64;
        nat_mul(a796, a797)?
    };
    let (_, h798) = f266(Fn4::F267(v0, v1, v2), {
        let a799 = v2;
        let a800 = 64u64;
        let a801 = 0u64;
        nat_quot(a799, a800, a801)
    }, (0u64, f268()))?;
    Ok(h798)
}

pub fn f198(p804: List<List<Bytes>>, v1: Bytes, p805: List<Bytes>) -> R<List<Bytes>> {
    let m803 = (p804, p805);
    {
        let mut m803 = m803;
        loop {
            let step = {
                let (v0, v2) = m803;
                let m802 = v0;
                match m802.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f269(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f200(f270(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m803 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f271(v0)?))
}

pub fn f200(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m806 = v0;
    match m806.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m807 = v3.clone();
            match m807.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a810 = v2;
                    let a811 = {
                        let a808 = v1.clone();
                        let a809 = f200(v3, v1);
                        append_bytes(a808, a809)
                    };
                    append_bytes(a810, a811)
                }
            }
        }
    }
}

pub fn f201(v0: List<Bytes>) -> Adt14 {
    let m812 = v0;
    match m812.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f201(v2)))
        }
    }
}

pub fn f202(v0: Fn2, p816: Option<Bytes>, p817: List<Bytes>) -> Option<Bytes> {
    let m815 = (p816, p817);
    {
        let mut m815 = m815;
        loop {
            let step = {
                let (v1, v2) = m815;
                let m813 = v2;
                match m813.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c814: Fn2 = v0.clone();
                            c814.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m818 = v1;
    match m818 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a821 = {
                    let a819 = v3;
                    let a820 = v0;
                    append_bytes(a819, a820)
                };
                let a822 = v2;
                append_bytes(a821, a822)
            })
        }
    }
}

pub fn f204(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f271(v0.clone())?;
    let v2: u64 = f272(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a823 = v1;
        let a824 = v2;
        nat_sub(a823, a824)
    }))
}

pub fn f205(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f273(v0)?;
    let m828 = f274(v1.clone(), 0u64, {
        let a826 = {
            let a825 = v1.clone();
            length_bytes(a825)
        };
        let a827 = 2u64;
        nat_add(a826, a827)?
    }, 0u64)?;
    match m828 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a831 = {
                let (_, h829) = v2;
                h829
            };
            let a832 = {
                let a830 = v1;
                length_bytes(a830)
            };
            if nat_eq(a831, a832) {
                Ok(Some({
                    let (h833, _) = v2;
                    h833
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f206(v0: Bytes) -> Bytes {
    f7(List::cons(f75(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f207(v0: Adt14) -> bool {
    if f188(v0.clone(), f8(Str::lit("domain"))) {
        if f188(v0.clone(), f8(Str::lit("row"))) {
            if f84(f143(v0.clone(), f8(Str::lit("relation"))), f275()) {
                if f276(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f276(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f276(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f276(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f208(v0: Adt14) -> bool {
    if f188(v0.clone(), f8(Str::lit("subject"))) {
        if f188(v0.clone(), f8(Str::lit("object"))) {
            if f188(v0.clone(), f8(Str::lit("evidence"))) {
                f84(f143(v0, f8(Str::lit("relation"))), f277())
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

pub fn f209(v0: Option<List<Adt14>>) -> bool {
    let m834 = v0;
    match m834 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f210(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m835 = v0;
    match m835 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f211(v0: Adt14) -> Bytes {
    if f188(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f188(v0.clone(), f8(Str::lit("id"))) {
            if f188(v0.clone(), f8(Str::lit("statement"))) {
                if f188(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f219(v0, f8(Str::lit("evidence")))
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

pub fn f212(v0: List<Adt14>) -> Adt14 {
    let m836 = v0;
    match m836.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f212(v2)))
        }
    }
}

pub fn f213(v0: Adt14) -> bool {
    if f188(v0.clone(), f8(Str::lit("name"))) {
        if f219(v0.clone(), f8(Str::lit("schema"))) {
            let m837 = f215(f72(f144(f140(v0, f8(Str::lit("citation"))))));
            match m837 {
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

pub fn f214(v0: Bytes) -> R<Bytes> {
    f278(v0.clone(), 0u64, {
        let a838 = v0;
        length_bytes(a838)
    }, true, Bytes::lit(&[]))
}

pub fn f215(v0: List<Bytes>) -> Option<Adt14> {
    let m839 = v0;
    match m839.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m840 = v2;
                match m840.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m841 = v2;
                    match m841.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m842 = v6;
                                match m842.uncons() {
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
                        let m843 = v2;
                        match m843.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m844 = v10;
                                match m844.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f201(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f216(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m845 = f58(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m845.uncons() {
        None => {
            let m846 = f215(v4);
            match m846 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a847 = f8(Str::lit("Catalog."));
                        let a848 = f214(v3.clone())?;
                        append_bytes(a847, a848)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f214(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m849 = f279(v2, v9.clone(), f214(v3.clone())?)?;
                    match m849 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f214(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f142(Adt14::C3(std::rc::Rc::new(f212(f280(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f142(Adt14::C3(std::rc::Rc::new(f212(f280(f71(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a853 = f281(f68(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a851 = {
                                        let a850 = v10;
                                        length_bytes(a850)
                                    };
                                    let a852 = 4u64;
                                    nat_sub(a851, a852)
                                }))?;
                                let a854 = Bytes::lit(&[0x0a]);
                                append_bytes(a853, a854)
                            }), List::cons(f118(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f217(v0: Adt14) -> R<Bytes> {
    let m855 = v0;
    match m855 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f282(v2, v3)
        }
        Adt14::C2(v4) => {
            f283(v4)
        }
        Adt14::C3(r856) => {
            let v5 = (*r856).clone();
            Ok(f284(f285(v5)?))
        }
        Adt14::C4(r857) => {
            let v6 = (*r857).clone();
            f286(f287(v6)?)
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

pub fn f218(v0: Adt14) -> List<Adt14> {
    let m858 = v0;
    match m858 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r859) => {
            let v3 = (*r859).clone();
            f218(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r860, r861) => {
            let v1 = (*r860).clone();
            let v2 = (*r861).clone();
            List::cons(v1, f218(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f219(v0: Adt14, v1: Bytes) -> bool {
    let m862 = f140(v0, v1);
    match m862 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f220(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m863 = f140(v0, v1);
    match m863 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f144(f140(v3, v2))
        }
    }
}

pub fn f221(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f140(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f114(Str::lit("id"), f143(v0.clone(), f8(Str::lit("id"))))?, List::cons(f114(Str::lit("name"), f143(v0.clone(), f8(Str::lit("name"))))?, List::cons(f114(Str::lit("issuer"), f143(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f114(Str::lit("edition"), f143(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f114(Str::lit("canonical_id"), f143(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f114(Str::lit("source_role"), f143(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f114(Str::lit("retrieval_date"), f143(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f114(Str::lit("url"), f288(v2.clone(), f8(Str::lit("url"))))?, List::cons(f114(Str::lit("media_type"), f288(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f114(Str::lit("license"), f288(v2.clone(), f8(Str::lit("license"))))?, List::cons(f114(Str::lit("redistribution"), f288(v2, f8(Str::lit("redistribution"))))?, List::cons(f114(Str::lit("statement"), f143(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m864 = v1;
        match m864 {
            None => {
                f7(List::cons(f114(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f114(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f114(Str::lit("artifact"), {
                    let (h865, _) = v3;
                    h865
                })?, List::cons(f114(Str::lit("checksum"), {
                    let (_, h866) = v3;
                    h866
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f289(f72(f144(f140(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f222(v0: Bytes, v1: u64) -> Option<u8> {
    let a867 = v0;
    let a868 = v1;
    index_bytes(a867, a868)
}

pub fn f223(v0: Bytes) -> bool {
    f127(v0, f8(Str::lit(".lex.tex")))
}

pub fn f224(v0: Fn3, p873: u64, p874: u64) -> R<(u64, bool)> {
    let m872 = (p873, p874);
    {
        let mut m872 = m872;
        loop {
            let step = {
                let (v1, v2) = m872;
                let m869 = v1;
                if m869 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m869 - 1;
                    let m871 = {
                        let c870: Fn3 = v0.clone();
                        c870.apply(v2)?
                    };
                    match m871 {
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
                Ok(next) => { m872 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f226(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a877 = {
            let a875 = v4;
            let a876 = v1;
            nat_add(a875, a876)?
        };
        let a878 = v3;
        if nat_lt(a877, a878) {
            Ok(Some({
                let a879 = v4;
                let a880 = 1u64;
                nat_add(a879, a880)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f226(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m885 = {
        let a882 = v0;
        let a883 = v2;
        let a884 = {
            let a881 = v1.clone();
            length_bytes(a881)
        };
        slice_bytes(a882, a883, a884)
    };
    match m885 {
        None => {
            false
        }
        Some(v3) => {
            let a886 = v3;
            let a887 = v1;
            equal(a886, a887)
        }
    }
}

pub fn f227(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m888 = v0;
    match m888.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f227(v3, {
                let a889 = v1;
                let a890 = 1u64;
                nat_add(a889, a890)?
            })?))
        }
    }
}

pub fn f228(v0: Adt16, v1: List<u64>, v2: Adt19) -> bool {
    let m891 = v2;
    match m891 {
        Adt19::C0(v3, v4) => {
            if f234(v3, match v0.clone() {
                Adt16::C0(h892, _, _, _) => {
                    h892
                }
            }) {
                f290(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f234(v5, match v0 {
                Adt16::C0(_, _, h893, _) => {
                    h893
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f230(v7, v1) {
                f234(v8, match v0 {
                    Adt16::C0(_, _, _, h894) => {
                        h894
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f229(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m895 = f140(v1.clone(), f8(Str::lit("std")));
    match m895 {
        None => {
            let m900 = f148(f140(v1.clone(), f8(Str::lit("req"))));
            match m900 {
                None => {
                    let m902 = f140(v1, f8(Str::lit("der")));
                    match m902 {
                        None => {
                            Ok(f291(v0))
                        }
                        Some(v5) => {
                            let m903 = f292(f143(v5.clone(), f8(Str::lit("rule"))));
                            match m903 {
                                None => {
                                    Ok(f291(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f164(f72(f144(f140(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h904) => {
                                            h904
                                        }
                                    })?, f232(f143(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h905, _) => {
                                            h905
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f232(v4, match v0 {
                        Adt12::C0(_, _, h901, _, _) => {
                            h901
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m896 = f148(f140(v2.clone(), f8(Str::lit("row"))));
            match m896 {
                None => {
                    Ok(Adt19::C0(f232(f143(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h897, _, _, _, _) => {
                            h897
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f232(f143(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h898, _, _, _, _) => {
                            h898
                        }
                    })?, Some(f232(v3, match v0 {
                        Adt12::C0(_, h899, _, _, _) => {
                            h899
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f230(p908: List<u64>, v1: List<u64>) -> bool {
    let m907 = p908;
    {
        let mut m907 = m907;
        loop {
            let step = {
                let v0 = m907;
                let m906 = v0;
                match m906.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f234(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m907 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(p913: List<u64>) -> bool {
    let m912 = p913;
    {
        let mut m912 = m912;
        loop {
            let step = {
                let v0 = m912;
                let m909 = v0;
                match m909.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a910 = v1;
                        let a911 = 7u64;
                        if nat_le(a910, a911) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f232(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m914 = f293(v0, v1.clone(), 0u64)?;
    match m914 {
        None => {
            let a915 = v1;
            Ok(length_list(a915))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f233(v0: Adt14) -> u64 {
    let m916 = f294(f140(v0, f8(Str::lit("stratum"))));
    match m916 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f234(v0: u64, p921: List<u64>) -> bool {
    let m920 = p921;
    {
        let mut m920 = m920;
        loop {
            let step = {
                let v1 = m920;
                let m917 = v1;
                match m917.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a918 = v2;
                        let a919 = v0;
                        if nat_eq(a918, a919) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m920 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f190(v0.clone(), v1) {
        let m922 = f30(v0, 34u8)?;
        match m922.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m923 = v3;
                match m923.uncons() {
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

pub fn f236(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f237(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m924 = v0.clone();
    match m924.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a925 = v2;
            let a926 = v1.clone();
            if equal(a925, a926) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f238(v0: List<(Bytes, ())>) -> List<Bytes> {
    f169(f295(Fn5::F296, List::<Bytes>::nil(), v0))
}

pub fn f239(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f297(f298(Fn6::F299, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m931 = {
        let (h927, _) = f300(Fn7::F301, {
            let a929 = {
                let a928 = v0;
                length_list(a928)
            };
            let a930 = 1u64;
            nat_add(a929, a930)?
        }, v1)?;
        h927
    };
    match m931.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f240(v0: List<Bytes>) -> List<(Bytes, ())> {
    f302(f303(Fn8::F304, List::<(Bytes, ())>::nil(), v0))
}

pub fn f241(p934: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m933 = p934;
    {
        let mut m933 = m933;
        loop {
            let step = {
                let v0 = m933;
                let m932 = v0;
                match m932.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f269(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m933 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f288(f140(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f243(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h935, _) = v1.clone();
        h935
    };
    let v3: u64 = {
        let (h936, _) = {
            let (_, h937) = v1.clone();
            h937
        };
        h936
    };
    let v4: List<Bytes> = {
        let (_, h938) = {
            let (_, h939) = v1;
            h939
        };
        h938
    };
    let m940 = f222(v0.clone(), v2);
    match m940 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a941 = v5;
            let a942 = 10u8;
            if equal(a941, a942) {
                Ok(Some(({
                    let a943 = v2;
                    let a944 = 1u64;
                    nat_add(a943, a944)?
                }, ({
                    let a945 = v2;
                    let a946 = 1u64;
                    nat_add(a945, a946)?
                }, List::cons(f305(f32(v0, v3, {
                    let a947 = v2;
                    let a948 = v3;
                    nat_sub(a947, a948)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a949 = v2;
                    let a950 = 1u64;
                    nat_add(a949, a950)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f244(v0: Bytes) -> R<Bytes> {
    let a951 = f8(Str::lit("Catalog."));
    let a952 = f214(v0)?;
    Ok(append_bytes(a951, a952))
}

pub fn f245(v0: List<Bytes>) -> List<Bytes> {
    let m953 = v0;
    match m953.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f127(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f123(f131(), v1), f245(v2))
            } else {
                f245(v2)
            }
        }
    }
}

pub fn f246(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m954 = v1;
    match m954.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f306(v0.clone(), v2)?;
            let (h955, _) = v4.clone();
            if h955 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f246(v0, v3)?;
                let (h956, _) = v5.clone();
                if h956 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a959 = {
                            let (_, h957) = v4;
                            h957
                        };
                        let a960 = {
                            let (_, h958) = v5;
                            h958
                        };
                        append_bytes(a959, a960)
                    }))
                }
            }
        }
    }
}

pub fn f247(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m961 = v1;
    match m961.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f188(v2.clone(), f8(Str::lit("subject"))) {
                if f189(v0.clone(), v2.clone()) {
                    f247(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f143(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f143(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f143(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f247(v0, v3))
                }
            } else {
                if f256(v0.clone(), v2.clone()) {
                    f247(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f143(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f257(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f247(v0, v3))
                }
            }
        }
    }
}

pub fn f248(v0: Adt1) -> R<Bytes> {
    let a963 = {
        let a962 = f101(f102(f120(f121(f110(f68(v0.clone(), f115()))?)), f103(f71(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f104(f71(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a962)
    };
    let a964 = 0u64;
    if nat_eq(a963, a964) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f249(v0: List<Adt14>) -> R<List<Bytes>> {
    let m965 = v0;
    match m965.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f119(f72(f144(f140(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f143(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f143(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f307(f81(f143(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f249(v2)?))
        }
    }
}

pub fn f250(v0: Bytes, v1: List<Adt21>) -> List<Bytes> {
    let m966 = v1;
    match m966.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(match v2.clone() {
                Adt21::C0(h967, _, _) => {
                    h967
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt21::C0(_, h968, _) => {
                        h968
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt21::C0(_, _, h969) => {
                        h969
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f250(v0, v3))
            } else {
                f250(v0, v3)
            }
        }
    }
}

pub fn f251(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m970 = v1;
    match m970.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(f143(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f143(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f143(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f251(v0, v3))
            } else {
                if f36(f143(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f143(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f143(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f251(v0, v3))
                } else {
                    f251(v0, v3)
                }
            }
        }
    }
}

pub fn f252(p975: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m974 = p975;
    {
        let mut m974 = m974;
        loop {
            let step = {
                let v0 = m974;
                let m971 = v0;
                match m971.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m972 = v2;
                        match m972.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f36(v4, v1.clone()) {
                                    let m973 = v5;
                                    match m973.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f308(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m974 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f253(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m976 = v0;
    match m976.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m977 = f140(v1, f8(Str::lit("std")));
            match m977 {
                None => {
                    f253(v2)
                }
                Some(v3) => {
                    if f188(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f143(v3.clone(), f8(Str::lit("row"))), f143(v3, f8(Str::lit("authority")))), f253(v2))
                    } else {
                        f253(v2)
                    }
                }
            }
        }
    }
}

pub fn f254(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m978 = v1;
    match m978.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f143(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f143(v2, f8(Str::lit("object")));
            if if f84(v4.clone(), match v0.clone() {
                Adt15::C0(h979, _, _) => {
                    h979
                }
            }) {
                f84(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h980, _) => {
                        h980
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f254(v0, v3))
            } else {
                if if f84(v5.clone(), match v0.clone() {
                    Adt15::C0(h981, _, _) => {
                        h981
                    }
                }) {
                    f84(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h982, _) => {
                            h982
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f254(v0, v3))
                } else {
                    f254(v0, v3)
                }
            }
        }
    }
}

pub fn f255(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt21>> {
    let m983 = v0;
    match m983.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v4, v5)) => {
            if if f309(v4.clone(), v2.clone()) {
                true
            } else {
                f309(v4.clone(), v3.clone())
            } {
                f255(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt21::C0({
                    let (h984, _) = v4.clone();
                    h984
                }, {
                    let a987 = {
                        let a986 = f76(f310({
                            let (h985, _) = v4.clone();
                            h985
                        }, v1.clone()))?;
                        length_list(a986)
                    };
                    let a988 = 1u64;
                    if nat_eq(a987, a988) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h989) = v4.clone();
                    h989
                }), f255(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f256(v0: Adt15, v1: Adt14) -> bool {
    if f84(f143(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h990, _, _) => {
            h990
        }
    }) {
        f84(f257(v1), match v0 {
            Adt15::C0(_, h991, _) => {
                h991
            }
        })
    } else {
        false
    }
}

pub fn f257(v0: Adt14) -> Bytes {
    let m992 = f140(v0, f8(Str::lit("evidence")));
    match m992 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f143(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f258(v0: Adt14) -> Bytes {
    if f188(v0.clone(), f8(Str::lit("req"))) {
        let a993 = f8(Str::lit("req "));
        let a994 = f143(v0, f8(Str::lit("req")));
        append_bytes(a993, a994)
    } else {
        let m995 = f140(v0.clone(), f8(Str::lit("std")));
        match m995 {
            None => {
                let m998 = f140(v0, f8(Str::lit("der")));
                match m998 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f143(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f119(f72(f144(f140(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f143(v1.clone(), f8(Str::lit("authority"))), List::cons(if f188(v1.clone(), f8(Str::lit("row"))) {
                    let a996 = f8(Str::lit(" row "));
                    let a997 = f143(v1, f8(Str::lit("row")));
                    append_bytes(a996, a997)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f259(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m999 = v1;
    match m999.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84({
                let (h1000, _) = v2.clone();
                h1000
            }, v0.clone()) {
                List::cons({
                    let (_, h1001) = v2;
                    h1001
                }, f259(v0, v3))
            } else {
                if f84({
                    let (_, h1002) = v2;
                    h1002
                }, v0.clone()) {
                    List::cons({
                        let (h1003, _) = v2;
                        h1003
                    }, f259(v0, v3))
                } else {
                    f259(v0, v3)
                }
            }
        }
    }
}

pub fn f260(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f84(v2.clone(), match v0.clone() {
        Adt15::C0(h1004, _, _) => {
            h1004
        }
    }) {
        true
    } else {
        if f84(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1005, _) => {
                h1005
            }
        }) {
            true
        } else {
            if f84(v2, match v0.clone() {
                Adt15::C0(_, _, h1006) => {
                    h1006
                }
            }) {
                true
            } else {
                if f188(v1.clone(), f8(Str::lit("scheme"))) {
                    f84(f143(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1007, _) => {
                            h1007
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f261(v0: Bytes, v1: u64) -> R<u64> {
    let m1010 = {
        let a1008 = v0;
        let a1009 = v1;
        index_bytes(a1008, a1009)
    };
    match m1010 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1055 = {
                let a1031 = {
                    let a1019 = {
                        let a1013 = {
                            let a1011 = v3;
                            let a1012 = 128u8;
                            fixed_u8::bit_and(a1011, a1012)
                        };
                        let a1014 = 128u8;
                        if equal(a1013, a1014) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1020 = {
                        let a1017 = {
                            let a1015 = v3;
                            let a1016 = 64u8;
                            fixed_u8::bit_and(a1015, a1016)
                        };
                        let a1018 = 64u8;
                        if equal(a1017, a1018) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1019, a1020)?
                };
                let a1032 = {
                    let a1029 = {
                        let a1023 = {
                            let a1021 = v3;
                            let a1022 = 32u8;
                            fixed_u8::bit_and(a1021, a1022)
                        };
                        let a1024 = 32u8;
                        if equal(a1023, a1024) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1030 = {
                        let a1027 = {
                            let a1025 = v3;
                            let a1026 = 16u8;
                            fixed_u8::bit_and(a1025, a1026)
                        };
                        let a1028 = 16u8;
                        if equal(a1027, a1028) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1029, a1030)?
                };
                nat_add(a1031, a1032)?
            };
            let a1056 = {
                let a1053 = {
                    let a1041 = {
                        let a1035 = {
                            let a1033 = v3;
                            let a1034 = 8u8;
                            fixed_u8::bit_and(a1033, a1034)
                        };
                        let a1036 = 8u8;
                        if equal(a1035, a1036) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1042 = {
                        let a1039 = {
                            let a1037 = v3;
                            let a1038 = 4u8;
                            fixed_u8::bit_and(a1037, a1038)
                        };
                        let a1040 = 4u8;
                        if equal(a1039, a1040) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1041, a1042)?
                };
                let a1054 = {
                    let a1051 = {
                        let a1045 = {
                            let a1043 = v3;
                            let a1044 = 2u8;
                            fixed_u8::bit_and(a1043, a1044)
                        };
                        let a1046 = 2u8;
                        if equal(a1045, a1046) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1052 = {
                        let a1049 = {
                            let a1047 = v3;
                            let a1048 = 1u8;
                            fixed_u8::bit_and(a1047, a1048)
                        };
                        let a1050 = 1u8;
                        if equal(a1049, a1050) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1051, a1052)?
                };
                nat_add(a1053, a1054)?
            };
            nat_add(a1055, a1056)
        }
    }
}

pub fn f262(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1060 = {
        let a1057 = v1;
        let a1058 = v2;
        let a1059 = 1u64;
        slice_bytes(a1057, a1058, a1059)
    };
    match m1060 {
        None => {
            v0
        }
        Some(v3) => {
            let a1061 = v0;
            let a1062 = v3;
            append_bytes(a1061, a1062)
        }
    }
}

pub fn f263(p1065: List<Bytes>, p1066: List<Bytes>) -> List<Bytes> {
    let m1064 = (p1065, p1066);
    {
        let mut m1064 = m1064;
        loop {
            let step = {
                let (v0, v1) = m1064;
                let m1063 = v0;
                match m1063.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1064 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: Bytes, p1138: u64, p1139: u64, p1140: Bytes) -> R<Bytes> {
    let m1137 = (p1138, (p1139, p1140));
    {
        let mut m1137 = m1137;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1137;
                let m1067 = v2;
                if m1067 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1067 - 1;
                    let m1070 = {
                        let a1068 = v0.clone();
                        let a1069 = v1;
                        index_bytes(a1068, a1069)
                    };
                    match m1070 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1115 = {
                                    let a1091 = {
                                        let a1079 = {
                                            let a1073 = {
                                                let a1071 = v6;
                                                let a1072 = 128u8;
                                                fixed_u8::bit_and(a1071, a1072)
                                            };
                                            let a1074 = 128u8;
                                            if equal(a1073, a1074) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1080 = {
                                            let a1077 = {
                                                let a1075 = v6;
                                                let a1076 = 64u8;
                                                fixed_u8::bit_and(a1075, a1076)
                                            };
                                            let a1078 = 64u8;
                                            if equal(a1077, a1078) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1079, a1080)?
                                    };
                                    let a1092 = {
                                        let a1089 = {
                                            let a1083 = {
                                                let a1081 = v6;
                                                let a1082 = 32u8;
                                                fixed_u8::bit_and(a1081, a1082)
                                            };
                                            let a1084 = 32u8;
                                            if equal(a1083, a1084) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1090 = {
                                            let a1087 = {
                                                let a1085 = v6;
                                                let a1086 = 16u8;
                                                fixed_u8::bit_and(a1085, a1086)
                                            };
                                            let a1088 = 16u8;
                                            if equal(a1087, a1088) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1089, a1090)?
                                    };
                                    nat_add(a1091, a1092)?
                                };
                                let a1116 = {
                                    let a1113 = {
                                        let a1101 = {
                                            let a1095 = {
                                                let a1093 = v6;
                                                let a1094 = 8u8;
                                                fixed_u8::bit_and(a1093, a1094)
                                            };
                                            let a1096 = 8u8;
                                            if equal(a1095, a1096) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1102 = {
                                            let a1099 = {
                                                let a1097 = v6;
                                                let a1098 = 4u8;
                                                fixed_u8::bit_and(a1097, a1098)
                                            };
                                            let a1100 = 4u8;
                                            if equal(a1099, a1100) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1101, a1102)?
                                    };
                                    let a1114 = {
                                        let a1111 = {
                                            let a1105 = {
                                                let a1103 = v6;
                                                let a1104 = 2u8;
                                                fixed_u8::bit_and(a1103, a1104)
                                            };
                                            let a1106 = 2u8;
                                            if equal(a1105, a1106) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1112 = {
                                            let a1109 = {
                                                let a1107 = v6;
                                                let a1108 = 1u8;
                                                fixed_u8::bit_and(a1107, a1108)
                                            };
                                            let a1110 = 1u8;
                                            if equal(a1109, a1110) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1111, a1112)?
                                    };
                                    nat_add(a1113, a1114)?
                                };
                                nat_add(a1115, a1116)?
                            };
                            let a1117 = v7;
                            let a1118 = 34u64;
                            if if nat_eq(a1117, a1118) {
                                true
                            } else {
                                let a1119 = v7;
                                let a1120 = 92u64;
                                nat_eq(a1119, a1120)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1121 = v1;
                                    let a1122 = 1u64;
                                    nat_add(a1121, a1122)?
                                }, (v4, {
                                    let a1125 = v3;
                                    let a1126 = {
                                        let a1123 = Bytes::lit(&[0x5c]);
                                        let a1124 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1123, a1124)
                                    };
                                    append_bytes(a1125, a1126)
                                })))
                            } else {
                                let a1127 = v7;
                                let a1128 = 10u64;
                                if nat_eq(a1127, a1128) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1129 = v1;
                                        let a1130 = 1u64;
                                        nat_add(a1129, a1130)?
                                    }, (v4, {
                                        let a1131 = v3;
                                        let a1132 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1131, a1132)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1133 = v1;
                                        let a1134 = 1u64;
                                        nat_add(a1133, a1134)?
                                    }, (v4, {
                                        let a1135 = v3;
                                        let a1136 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1135, a1136)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f265(v0: u32) -> Bytes {
    let a1169 = f311({
        let a1141 = f312(v0, 28u32);
        let a1142 = 15u32;
        fixed_u32::bit_and(a1141, a1142)
    });
    let a1170 = {
        let a1167 = f311({
            let a1143 = f312(v0, 24u32);
            let a1144 = 15u32;
            fixed_u32::bit_and(a1143, a1144)
        });
        let a1168 = {
            let a1165 = f311({
                let a1145 = f312(v0, 20u32);
                let a1146 = 15u32;
                fixed_u32::bit_and(a1145, a1146)
            });
            let a1166 = {
                let a1163 = f311({
                    let a1147 = f312(v0, 16u32);
                    let a1148 = 15u32;
                    fixed_u32::bit_and(a1147, a1148)
                });
                let a1164 = {
                    let a1161 = f311({
                        let a1149 = f312(v0, 12u32);
                        let a1150 = 15u32;
                        fixed_u32::bit_and(a1149, a1150)
                    });
                    let a1162 = {
                        let a1159 = f311({
                            let a1151 = f312(v0, 8u32);
                            let a1152 = 15u32;
                            fixed_u32::bit_and(a1151, a1152)
                        });
                        let a1160 = {
                            let a1157 = f311({
                                let a1153 = f312(v0, 4u32);
                                let a1154 = 15u32;
                                fixed_u32::bit_and(a1153, a1154)
                            });
                            let a1158 = f311({
                                let a1155 = v0;
                                let a1156 = 15u32;
                                fixed_u32::bit_and(a1155, a1156)
                            });
                            append_bytes(a1157, a1158)
                        };
                        append_bytes(a1159, a1160)
                    };
                    append_bytes(a1161, a1162)
                };
                append_bytes(a1163, a1164)
            };
            append_bytes(a1165, a1166)
        };
        append_bytes(a1167, a1168)
    };
    append_bytes(a1169, a1170)
}

pub fn f266(v0: Fn4, p1174: u64, p1175: (u64, Adt22)) -> R<(u64, Adt22)> {
    let m1173 = (p1174, p1175);
    {
        let mut m1173 = m1173;
        loop {
            let step = {
                let (v1, v2) = m1173;
                let m1171 = v1;
                if m1171 == 0 {
                    Err::<(u64, (u64, Adt22)), R<(u64, Adt22)>>(Ok(v2))
                } else {
                    let v3 = m1171 - 1;
                    Ok::<(u64, (u64, Adt22)), R<(u64, Adt22)>>((v3, {
                        let c1172: Fn4 = v0.clone();
                        c1172.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1173 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt22)) -> R<(u64, Adt22)> {
    Ok(({
        let a1177 = {
            let (h1176, _) = v3.clone();
            h1176
        };
        let a1178 = 64u64;
        nat_add(a1177, a1178)?
    }, f313({
        let (_, h1179) = v3;
        h1179
    }, f314(v0, v1, v2, {
        let (h1180, _) = v3;
        h1180
    })?)?))
}

pub fn f268() -> Adt22 {
    Adt22::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f269(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f84(v1, f67(f200(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f270(v0: List<Bytes>) -> List<Bytes> {
    let m1181 = v0;
    match m1181.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f270(v2)
            } else {
                List::cons(v1, f270(v2))
            }
        }
    }
}

pub fn f271(v0: Bytes) -> R<u64> {
    let (h1182, _) = f224(Fn3::F315(v0.clone()), {
        let a1183 = v0.clone();
        length_bytes(a1183)
    }, {
        let a1184 = v0;
        length_bytes(a1184)
    })?;
    Ok(h1182)
}

pub fn f272(v0: Bytes, v1: u64) -> R<u64> {
    let (h1185, _) = f224(Fn3::F316(v1, v0), v1, 0u64)?;
    Ok(h1185)
}

pub fn f273(v0: Bytes) -> R<Bytes> {
    f317(v0.clone(), 0u64, {
        let a1186 = v0;
        length_bytes(a1186)
    }, false, false, Bytes::lit(&[]))
}

pub fn f274(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1187 = v2;
    if m1187 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1187 - 1;
        let v5: u64 = f318(v0.clone(), v1)?;
        let a1188 = v3;
        let a1189 = 1u64;
        if nat_eq(a1188, a1189) {
            let m1190 = f274(v0.clone(), v1, v4, 0u64)?;
            match m1190 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f318(v0.clone(), {
                        let (_, h1191) = v6.clone();
                        h1191
                    })?;
                    let a1192 = v7;
                    let a1193 = 44u64;
                    if nat_eq(a1192, a1193) {
                        let m1197 = f274(v0, {
                            let a1195 = {
                                let (_, h1194) = v6;
                                h1194
                            };
                            let a1196 = 1u64;
                            nat_add(a1195, a1196)?
                        }, v4, 1u64)?;
                        match m1197 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1198, _) = v6;
                                    h1198
                                }), std::rc::Rc::new({
                                    let (h1199, _) = v8;
                                    h1199
                                })), {
                                    let (_, h1200) = v8;
                                    h1200
                                })))
                            }
                        }
                    } else {
                        let a1201 = v7;
                        let a1202 = 93u64;
                        if nat_eq(a1201, a1202) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1203, _) = v6;
                                h1203
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1205 = {
                                    let (_, h1204) = v6;
                                    h1204
                                };
                                let a1206 = 1u64;
                                nat_add(a1205, a1206)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1207 = v3;
            let a1208 = 2u64;
            if nat_eq(a1207, a1208) {
                let m1211 = f319(v0.clone(), {
                    let a1209 = v1;
                    let a1210 = 1u64;
                    nat_add(a1209, a1210)?
                })?;
                match m1211 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1212 = v5;
                        let a1213 = 34u64;
                        if if nat_eq(a1212, a1213) {
                            let a1215 = f318(v0.clone(), {
                                let (h1214, _) = v9.clone();
                                h1214
                            })?;
                            let a1216 = 58u64;
                            nat_eq(a1215, a1216)
                        } else {
                            false
                        } {
                            let m1220 = f274(v0.clone(), {
                                let a1218 = {
                                    let (h1217, _) = v9;
                                    h1217
                                };
                                let a1219 = 1u64;
                                nat_add(a1218, a1219)?
                            }, v4, 0u64)?;
                            match m1220 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f318(v0.clone(), {
                                        let (_, h1221) = v10.clone();
                                        h1221
                                    })?;
                                    let a1222 = v11;
                                    let a1223 = 44u64;
                                    if nat_eq(a1222, a1223) {
                                        let m1227 = f274(v0, {
                                            let a1225 = {
                                                let (_, h1224) = v10;
                                                h1224
                                            };
                                            let a1226 = 1u64;
                                            nat_add(a1225, a1226)?
                                        }, v4, 2u64)?;
                                        match m1227 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1228) = v9;
                                                    h1228
                                                }, std::rc::Rc::new({
                                                    let (h1229, _) = v10;
                                                    h1229
                                                }), std::rc::Rc::new({
                                                    let (h1230, _) = v12;
                                                    h1230
                                                })), {
                                                    let (_, h1231) = v12;
                                                    h1231
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1232 = v11;
                                        let a1233 = 125u64;
                                        if nat_eq(a1232, a1233) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1234) = v9;
                                                h1234
                                            }, std::rc::Rc::new({
                                                let (h1235, _) = v10;
                                                h1235
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1237 = {
                                                    let (_, h1236) = v10;
                                                    h1236
                                                };
                                                let a1238 = 1u64;
                                                nat_add(a1237, a1238)?
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
                let a1239 = v5;
                let a1240 = 34u64;
                if nat_eq(a1239, a1240) {
                    let m1243 = f319(v0, {
                        let a1241 = v1;
                        let a1242 = 1u64;
                        nat_add(a1241, a1242)?
                    })?;
                    match m1243 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1244) = v13;
                                h1244
                            }), {
                                let (h1245, _) = v13;
                                h1245
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1246 = v1;
                            let a1247 = 4u64;
                            nat_add(a1246, a1247)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1248 = v1;
                                let a1249 = 5u64;
                                nat_add(a1248, a1249)?
                            })))
                        } else {
                            let a1250 = v5;
                            let a1251 = 91u64;
                            if nat_eq(a1250, a1251) {
                                let a1254 = f318(v0.clone(), {
                                    let a1252 = v1;
                                    let a1253 = 1u64;
                                    nat_add(a1252, a1253)?
                                })?;
                                let a1255 = 93u64;
                                if nat_eq(a1254, a1255) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1256 = v1;
                                        let a1257 = 2u64;
                                        nat_add(a1256, a1257)?
                                    })))
                                } else {
                                    let m1260 = f274(v0, {
                                        let a1258 = v1;
                                        let a1259 = 1u64;
                                        nat_add(a1258, a1259)?
                                    }, v4, 1u64)?;
                                    match m1260 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1261, _) = v14;
                                                h1261
                                            })), {
                                                let (_, h1262) = v14;
                                                h1262
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1263 = v5;
                                let a1264 = 123u64;
                                if nat_eq(a1263, a1264) {
                                    let a1267 = f318(v0.clone(), {
                                        let a1265 = v1;
                                        let a1266 = 1u64;
                                        nat_add(a1265, a1266)?
                                    })?;
                                    let a1268 = 125u64;
                                    if nat_eq(a1267, a1268) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1269 = v1;
                                            let a1270 = 2u64;
                                            nat_add(a1269, a1270)?
                                        })))
                                    } else {
                                        let m1273 = f274(v0, {
                                            let a1271 = v1;
                                            let a1272 = 1u64;
                                            nat_add(a1271, a1272)?
                                        }, v4, 2u64)?;
                                        match m1273 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1274, _) = v15;
                                                    h1274
                                                })), {
                                                    let (_, h1275) = v15;
                                                    h1275
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1276 = f320(v0, v1)?;
                                    match m1276 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1277) = v16;
                                                h1277
                                            }, {
                                                let (h1278, _) = v16;
                                                h1278
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

pub fn f275() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f276(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1279 = f140(v0, v1);
    match m1279 {
        None => {
            false
        }
        Some(v3) => {
            f188(v3, v2)
        }
    }
}

pub fn f277() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f278(v0: Bytes, p1346: u64, p1347: u64, p1348: bool, p1349: Bytes) -> R<Bytes> {
    let m1345 = (p1346, (p1347, (p1348, p1349)));
    {
        let mut m1345 = m1345;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1345;
                let m1280 = v2;
                if m1280 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1280 - 1;
                    let m1283 = {
                        let a1281 = v0.clone();
                        let a1282 = v1;
                        index_bytes(a1281, a1282)
                    };
                    match m1283 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1328 = {
                                    let a1304 = {
                                        let a1292 = {
                                            let a1286 = {
                                                let a1284 = v7;
                                                let a1285 = 128u8;
                                                fixed_u8::bit_and(a1284, a1285)
                                            };
                                            let a1287 = 128u8;
                                            if equal(a1286, a1287) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1293 = {
                                            let a1290 = {
                                                let a1288 = v7;
                                                let a1289 = 64u8;
                                                fixed_u8::bit_and(a1288, a1289)
                                            };
                                            let a1291 = 64u8;
                                            if equal(a1290, a1291) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1292, a1293)?
                                    };
                                    let a1305 = {
                                        let a1302 = {
                                            let a1296 = {
                                                let a1294 = v7;
                                                let a1295 = 32u8;
                                                fixed_u8::bit_and(a1294, a1295)
                                            };
                                            let a1297 = 32u8;
                                            if equal(a1296, a1297) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1303 = {
                                            let a1300 = {
                                                let a1298 = v7;
                                                let a1299 = 16u8;
                                                fixed_u8::bit_and(a1298, a1299)
                                            };
                                            let a1301 = 16u8;
                                            if equal(a1300, a1301) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1302, a1303)?
                                    };
                                    nat_add(a1304, a1305)?
                                };
                                let a1329 = {
                                    let a1326 = {
                                        let a1314 = {
                                            let a1308 = {
                                                let a1306 = v7;
                                                let a1307 = 8u8;
                                                fixed_u8::bit_and(a1306, a1307)
                                            };
                                            let a1309 = 8u8;
                                            if equal(a1308, a1309) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1315 = {
                                            let a1312 = {
                                                let a1310 = v7;
                                                let a1311 = 4u8;
                                                fixed_u8::bit_and(a1310, a1311)
                                            };
                                            let a1313 = 4u8;
                                            if equal(a1312, a1313) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1314, a1315)?
                                    };
                                    let a1327 = {
                                        let a1324 = {
                                            let a1318 = {
                                                let a1316 = v7;
                                                let a1317 = 2u8;
                                                fixed_u8::bit_and(a1316, a1317)
                                            };
                                            let a1319 = 2u8;
                                            if equal(a1318, a1319) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1325 = {
                                            let a1322 = {
                                                let a1320 = v7;
                                                let a1321 = 1u8;
                                                fixed_u8::bit_and(a1320, a1321)
                                            };
                                            let a1323 = 1u8;
                                            if equal(a1322, a1323) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1324, a1325)?
                                    };
                                    nat_add(a1326, a1327)?
                                };
                                nat_add(a1328, a1329)?
                            };
                            if f321(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1330 = v1;
                                    let a1331 = 1u64;
                                    nat_add(a1330, a1331)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f322(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1332 = v1;
                                        let a1333 = 1u64;
                                        nat_add(a1332, a1333)?
                                    }, (v5, (false, {
                                        let a1337 = v4;
                                        let a1338 = f32({
                                            let a1334 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1334)
                                        }, {
                                            let a1335 = v8;
                                            let a1336 = 97u64;
                                            nat_sub(a1335, a1336)
                                        }, 1u64);
                                        append_bytes(a1337, a1338)
                                    }))))
                                } else {
                                    if if f322(v8) {
                                        true
                                    } else {
                                        if f323(v8) {
                                            true
                                        } else {
                                            f324(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1339 = v1;
                                            let a1340 = 1u64;
                                            nat_add(a1339, a1340)?
                                        }, (v5, (false, {
                                            let a1341 = v4;
                                            let a1342 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1341, a1342)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1343 = v1;
                                            let a1344 = 1u64;
                                            nat_add(a1343, a1344)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1350 = f325(4096u64, f326(f327(f328(f140(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt23::C0(v2, v0), List::<Adt23>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m1350 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f200(f173(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f280(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1351 = v0;
    match m1351.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f36(f143(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f329(v5, v1, v2))
            } else {
                List::cons(v4, f280(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f281(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f200(f330(f81(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f282(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1352 = 0u64;
        let a1353 = v1;
        nat_lt(a1352, a1353)
    } else {
        false
    } {
        let a1354 = Bytes::lit(&[0x2d]);
        let a1355 = f331(v1)?;
        Ok(append_bytes(a1354, a1355))
    } else {
        f331(v1)
    }
}

pub fn f283(v0: Bytes) -> R<Bytes> {
    let a1361 = {
        let a1359 = Bytes::lit(&[0x22]);
        let a1360 = {
            let (_, h1356) = {
                let (h1357, _) = f332(Fn9::F333(v0.clone()), {
                    let a1358 = v0;
                    length_bytes(a1358)
                }, (0u64, Bytes::lit(&[])))?;
                h1357
            };
            h1356
        };
        append_bytes(a1359, a1360)
    };
    let a1362 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1361, a1362))
}

pub fn f284(v0: List<Bytes>) -> Bytes {
    let a1365 = {
        let a1363 = Bytes::lit(&[0x5b]);
        let a1364 = f119(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1363, a1364)
    };
    let a1366 = Bytes::lit(&[0x5d]);
    append_bytes(a1365, a1366)
}

pub fn f285(v0: Adt14) -> R<List<Bytes>> {
    let m1367 = v0;
    match m1367 {
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
        Adt14::C6(r1368, r1369) => {
            let v7 = (*r1368).clone();
            let v8 = (*r1369).clone();
            Ok(List::cons(f217(v7)?, f285(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f286(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f334(Fn10::F335, List::<(Bytes, Bytes)>::nil(), f336(v0)?);
    let v2: List<Bytes> = f337(Fn11::F338, List::<Bytes>::nil(), v1)?;
    let a1372 = {
        let a1370 = Bytes::lit(&[0x7b]);
        let a1371 = f119(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1370, a1371)
    };
    let a1373 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1372, a1373))
}

pub fn f287(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1374 = v0;
    match m1374 {
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
        Adt14::C7(v9, r1375, r1376) => {
            let v10 = (*r1375).clone();
            let v11 = (*r1376).clone();
            Ok(List::cons((v9, f217(v10)?), f287(v11)?))
        }
    }
}

pub fn f288(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1377 = v0;
    match m1377 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f143(v2, v1)
        }
    }
}

pub fn f289(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f200(f339(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f290(v0: Adt16, v1: Option<u64>) -> bool {
    let m1378 = v1;
    match m1378 {
        None => {
            true
        }
        Some(v2) => {
            f234(v2, match v0 {
                Adt16::C0(_, h1379, _, _) => {
                    h1379
                }
            })
        }
    }
}

pub fn f291(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a1381 = match v0 {
            Adt12::C0(_, _, h1380, _, _) => {
                h1380
            }
        };
        length_list(a1381)
    })
}

pub fn f292(v0: Bytes) -> Option<Adt20> {
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

pub fn f293(v0: Bytes, p1386: List<Bytes>, p1387: u64) -> R<Option<u64>> {
    let m1385 = (p1386, p1387);
    {
        let mut m1385 = m1385;
        loop {
            let step = {
                let (v1, v2) = m1385;
                let m1382 = v1;
                match m1382.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1383 = v2;
                                let a1384 = 1u64;
                                nat_add(a1383, a1384)?
                            }))
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

pub fn f294(v0: Option<Adt14>) -> Option<u64> {
    let m1388 = v0;
    match m1388 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1389 = v1;
            match m1389 {
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

pub fn f295(v0: Fn5, p1393: List<Bytes>, p1394: List<(Bytes, ())>) -> List<Bytes> {
    let m1392 = (p1393, p1394);
    {
        let mut m1392 = m1392;
        loop {
            let step = {
                let (v1, v2) = m1392;
                let m1390 = v2;
                match m1390.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1391: Fn5 = v0.clone();
                            c1391.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1395, _) = v1;
        h1395
    }, v0)
}

pub fn f297(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f340(Fn12::F341, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f298(v0: Fn6, p1399: List<List<(Bytes, ())>>, p1400: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1398 = (p1399, p1400);
    {
        let mut m1398 = m1398;
        loop {
            let step = {
                let (v1, v2) = m1398;
                let m1396 = v2;
                match m1396.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1397: Fn6 = v0.clone();
                            c1397.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f299(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f300(v0: Fn7, p1405: u64, p1406: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1404 = (p1405, p1406);
    {
        let mut m1404 = m1404;
        loop {
            let step = {
                let (v1, v2) = m1404;
                let m1401 = v1;
                if m1401 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1401 - 1;
                    let m1403 = {
                        let c1402: Fn7 = v0.clone();
                        c1402.apply(v2.clone())?
                    };
                    match m1403 {
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
                Ok(next) => { m1404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f301(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f342(v0)
}

pub fn f302(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f343(Fn13::F344, List::<(Bytes, ())>::nil(), v0)
}

pub fn f303(v0: Fn8, p1410: List<(Bytes, ())>, p1411: List<Bytes>) -> List<(Bytes, ())> {
    let m1409 = (p1410, p1411);
    {
        let mut m1409 = m1409;
        loop {
            let step = {
                let (v1, v2) = m1409;
                let m1407 = v2;
                match m1407.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1408: Fn8 = v0.clone();
                            c1408.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f305(v0: Bytes) -> Bytes {
    if f127(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1413 = {
                let a1412 = v0;
                length_bytes(a1412)
            };
            let a1414 = 1u64;
            nat_sub(a1413, a1414)
        })
    } else {
        v0
    }
}

pub fn f306(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f81(f68(v0.clone(), v1.clone()))?;
    let m1415 = f145(v2.clone())?;
    match m1415 {
        None => {
            f346(v0, v1, v2)
        }
        Some(v3) => {
            f345(v0, v1, v3)
        }
    }
}

pub fn f307(v0: List<Bytes>) -> List<Bytes> {
    let m1416 = v0;
    match m1416.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1418 = {
                    let a1417 = v1.clone();
                    length_bytes(a1417)
                };
                let a1419 = 0u64;
                if nat_eq(a1418, a1419) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f307(v2))
        }
    }
}

pub fn f308(v0: List<Bytes>) -> Bytes {
    let m1420 = v0;
    match m1420.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f309(v0: (Bytes, Bytes), p1423: List<(Bytes, Bytes)>) -> bool {
    let m1422 = p1423;
    {
        let mut m1422 = m1422;
        loop {
            let step = {
                let v1 = m1422;
                let m1421 = v1;
                match m1421.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f347(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
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

pub fn f310(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1424 = v1;
    match m1424.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1425, _) = v2;
                h1425
            }, v0.clone()) {
                List::cons({
                    let (_, h1426) = v2;
                    h1426
                }, f310(v0, v3))
            } else {
                f310(v0, v3)
            }
        }
    }
}

pub fn f311(v0: u32) -> Bytes {
    let a1429 = {
        let a1427 = v0;
        let a1428 = 8u32;
        fixed_u32::bit_and(a1427, a1428)
    };
    let a1430 = 0u32;
    if equal(a1429, a1430) {
        let a1433 = {
            let a1431 = v0;
            let a1432 = 4u32;
            fixed_u32::bit_and(a1431, a1432)
        };
        let a1434 = 0u32;
        if equal(a1433, a1434) {
            let a1437 = {
                let a1435 = v0;
                let a1436 = 2u32;
                fixed_u32::bit_and(a1435, a1436)
            };
            let a1438 = 0u32;
            if equal(a1437, a1438) {
                let a1441 = {
                    let a1439 = v0;
                    let a1440 = 1u32;
                    fixed_u32::bit_and(a1439, a1440)
                };
                let a1442 = 0u32;
                if equal(a1441, a1442) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1445 = {
                    let a1443 = v0;
                    let a1444 = 1u32;
                    fixed_u32::bit_and(a1443, a1444)
                };
                let a1446 = 0u32;
                if equal(a1445, a1446) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1449 = {
                let a1447 = v0;
                let a1448 = 2u32;
                fixed_u32::bit_and(a1447, a1448)
            };
            let a1450 = 0u32;
            if equal(a1449, a1450) {
                let a1453 = {
                    let a1451 = v0;
                    let a1452 = 1u32;
                    fixed_u32::bit_and(a1451, a1452)
                };
                let a1454 = 0u32;
                if equal(a1453, a1454) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1457 = {
                    let a1455 = v0;
                    let a1456 = 1u32;
                    fixed_u32::bit_and(a1455, a1456)
                };
                let a1458 = 0u32;
                if equal(a1457, a1458) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1461 = {
            let a1459 = v0;
            let a1460 = 4u32;
            fixed_u32::bit_and(a1459, a1460)
        };
        let a1462 = 0u32;
        if equal(a1461, a1462) {
            let a1465 = {
                let a1463 = v0;
                let a1464 = 2u32;
                fixed_u32::bit_and(a1463, a1464)
            };
            let a1466 = 0u32;
            if equal(a1465, a1466) {
                let a1469 = {
                    let a1467 = v0;
                    let a1468 = 1u32;
                    fixed_u32::bit_and(a1467, a1468)
                };
                let a1470 = 0u32;
                if equal(a1469, a1470) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1473 = {
                    let a1471 = v0;
                    let a1472 = 1u32;
                    fixed_u32::bit_and(a1471, a1472)
                };
                let a1474 = 0u32;
                if equal(a1473, a1474) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1477 = {
                let a1475 = v0;
                let a1476 = 2u32;
                fixed_u32::bit_and(a1475, a1476)
            };
            let a1478 = 0u32;
            if equal(a1477, a1478) {
                let a1481 = {
                    let a1479 = v0;
                    let a1480 = 1u32;
                    fixed_u32::bit_and(a1479, a1480)
                };
                let a1482 = 0u32;
                if equal(a1481, a1482) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1485 = {
                    let a1483 = v0;
                    let a1484 = 1u32;
                    fixed_u32::bit_and(a1483, a1484)
                };
                let a1486 = 0u32;
                if equal(a1485, a1486) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f312(v0: u32, v1: u32) -> u32 {
    let m1489 = {
        let a1487 = v0;
        let a1488 = v1;
        fixed_u32::shift_right(a1487, a1488)
    };
    match m1489 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f313(v0: Adt22, v1: Adt24) -> R<Adt22> {
    let v2: Adt25 = f348(Fn14::F349, 64u64, Adt25::C0(match v0.clone() {
        Adt22::C0(h1490, _, _, _, _, _, _, _) => {
            h1490
        }
    }, match v0.clone() {
        Adt22::C0(_, h1491, _, _, _, _, _, _) => {
            h1491
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h1492, _, _, _, _, _) => {
            h1492
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h1493, _, _, _, _) => {
            h1493
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h1494, _, _, _) => {
            h1494
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1495, _, _) => {
            h1495
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1496, _) => {
            h1496
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h1497) => {
            h1497
        }
    }, v1, 0u64))?;
    Ok(Adt22::C0(f350(match v0.clone() {
        Adt22::C0(h1498, _, _, _, _, _, _, _) => {
            h1498
        }
    }, match v2.clone() {
        Adt25::C0(h1499, _, _, _, _, _, _, _, _, _) => {
            h1499
        }
    }), f350(match v0.clone() {
        Adt22::C0(_, h1500, _, _, _, _, _, _) => {
            h1500
        }
    }, match v2.clone() {
        Adt25::C0(_, h1501, _, _, _, _, _, _, _, _) => {
            h1501
        }
    }), f350(match v0.clone() {
        Adt22::C0(_, _, h1502, _, _, _, _, _) => {
            h1502
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h1503, _, _, _, _, _, _, _) => {
            h1503
        }
    }), f350(match v0.clone() {
        Adt22::C0(_, _, _, h1504, _, _, _, _) => {
            h1504
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h1505, _, _, _, _, _, _) => {
            h1505
        }
    }), f350(match v0.clone() {
        Adt22::C0(_, _, _, _, h1506, _, _, _) => {
            h1506
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h1507, _, _, _, _, _) => {
            h1507
        }
    }), f350(match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1508, _, _) => {
            h1508
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h1509, _, _, _, _) => {
            h1509
        }
    }), f350(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1510, _) => {
            h1510
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h1511, _, _, _) => {
            h1511
        }
    }), f350(match v0 {
        Adt22::C0(_, _, _, _, _, _, _, h1512) => {
            h1512
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h1513, _, _) => {
            h1513
        }
    })))
}

pub fn f314(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f351(v0.clone(), v1, v2, v3)?, f351(v0.clone(), v1, v2, {
        let a1514 = v3;
        let a1515 = 4u64;
        nat_add(a1514, a1515)?
    })?, f351(v0.clone(), v1, v2, {
        let a1516 = v3;
        let a1517 = 8u64;
        nat_add(a1516, a1517)?
    })?, f351(v0.clone(), v1, v2, {
        let a1518 = v3;
        let a1519 = 12u64;
        nat_add(a1518, a1519)?
    })?, f351(v0.clone(), v1, v2, {
        let a1520 = v3;
        let a1521 = 16u64;
        nat_add(a1520, a1521)?
    })?, f351(v0.clone(), v1, v2, {
        let a1522 = v3;
        let a1523 = 20u64;
        nat_add(a1522, a1523)?
    })?, f351(v0.clone(), v1, v2, {
        let a1524 = v3;
        let a1525 = 24u64;
        nat_add(a1524, a1525)?
    })?, f351(v0.clone(), v1, v2, {
        let a1526 = v3;
        let a1527 = 28u64;
        nat_add(a1526, a1527)?
    })?, f351(v0.clone(), v1, v2, {
        let a1528 = v3;
        let a1529 = 32u64;
        nat_add(a1528, a1529)?
    })?, f351(v0.clone(), v1, v2, {
        let a1530 = v3;
        let a1531 = 36u64;
        nat_add(a1530, a1531)?
    })?, f351(v0.clone(), v1, v2, {
        let a1532 = v3;
        let a1533 = 40u64;
        nat_add(a1532, a1533)?
    })?, f351(v0.clone(), v1, v2, {
        let a1534 = v3;
        let a1535 = 44u64;
        nat_add(a1534, a1535)?
    })?, f351(v0.clone(), v1, v2, {
        let a1536 = v3;
        let a1537 = 48u64;
        nat_add(a1536, a1537)?
    })?, f351(v0.clone(), v1, v2, {
        let a1538 = v3;
        let a1539 = 52u64;
        nat_add(a1538, a1539)?
    })?, f351(v0.clone(), v1, v2, {
        let a1540 = v3;
        let a1541 = 56u64;
        nat_add(a1540, a1541)?
    })?, f351(v0, v1, v2, {
        let a1542 = v3;
        let a1543 = 60u64;
        nat_add(a1542, a1543)?
    })?))
}

pub fn f315(v0: Bytes, v1: u64) -> Option<u64> {
    let a1544 = 0u64;
    let a1545 = v1;
    if nat_lt(a1544, a1545) {
        if f352(v0, {
            let a1546 = v1;
            let a1547 = 1u64;
            nat_sub(a1546, a1547)
        }) {
            Some({
                let a1548 = v1;
                let a1549 = 1u64;
                nat_sub(a1548, a1549)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f316(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1550 = v2;
    let a1551 = v0;
    if nat_lt(a1550, a1551) {
        if f352(v1, v2) {
            Ok(Some({
                let a1552 = v2;
                let a1553 = 1u64;
                nat_add(a1552, a1553)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f317(v0: Bytes, p1633: u64, p1634: u64, p1635: bool, p1636: bool, p1637: Bytes) -> R<Bytes> {
    let m1632 = (p1633, (p1634, (p1635, (p1636, p1637))));
    {
        let mut m1632 = m1632;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1632;
                let m1554 = v2;
                if m1554 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1554 - 1;
                    let m1557 = {
                        let a1555 = v0.clone();
                        let a1556 = v1;
                        index_bytes(a1555, a1556)
                    };
                    match m1557 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1602 = {
                                    let a1578 = {
                                        let a1566 = {
                                            let a1560 = {
                                                let a1558 = v8;
                                                let a1559 = 128u8;
                                                fixed_u8::bit_and(a1558, a1559)
                                            };
                                            let a1561 = 128u8;
                                            if equal(a1560, a1561) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1567 = {
                                            let a1564 = {
                                                let a1562 = v8;
                                                let a1563 = 64u8;
                                                fixed_u8::bit_and(a1562, a1563)
                                            };
                                            let a1565 = 64u8;
                                            if equal(a1564, a1565) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1566, a1567)?
                                    };
                                    let a1579 = {
                                        let a1576 = {
                                            let a1570 = {
                                                let a1568 = v8;
                                                let a1569 = 32u8;
                                                fixed_u8::bit_and(a1568, a1569)
                                            };
                                            let a1571 = 32u8;
                                            if equal(a1570, a1571) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1577 = {
                                            let a1574 = {
                                                let a1572 = v8;
                                                let a1573 = 16u8;
                                                fixed_u8::bit_and(a1572, a1573)
                                            };
                                            let a1575 = 16u8;
                                            if equal(a1574, a1575) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1576, a1577)?
                                    };
                                    nat_add(a1578, a1579)?
                                };
                                let a1603 = {
                                    let a1600 = {
                                        let a1588 = {
                                            let a1582 = {
                                                let a1580 = v8;
                                                let a1581 = 8u8;
                                                fixed_u8::bit_and(a1580, a1581)
                                            };
                                            let a1583 = 8u8;
                                            if equal(a1582, a1583) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1589 = {
                                            let a1586 = {
                                                let a1584 = v8;
                                                let a1585 = 4u8;
                                                fixed_u8::bit_and(a1584, a1585)
                                            };
                                            let a1587 = 4u8;
                                            if equal(a1586, a1587) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1588, a1589)?
                                    };
                                    let a1601 = {
                                        let a1598 = {
                                            let a1592 = {
                                                let a1590 = v8;
                                                let a1591 = 2u8;
                                                fixed_u8::bit_and(a1590, a1591)
                                            };
                                            let a1593 = 2u8;
                                            if equal(a1592, a1593) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1599 = {
                                            let a1596 = {
                                                let a1594 = v8;
                                                let a1595 = 1u8;
                                                fixed_u8::bit_and(a1594, a1595)
                                            };
                                            let a1597 = 1u8;
                                            if equal(a1596, a1597) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1598, a1599)?
                                    };
                                    nat_add(a1600, a1601)?
                                };
                                nat_add(a1602, a1603)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1604 = v1;
                                    let a1605 = 1u64;
                                    nat_add(a1604, a1605)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1606 = v9;
                                    let a1607 = 34u64;
                                    !nat_eq(a1606, a1607)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1608 = v9;
                                    let a1609 = 92u64;
                                    nat_eq(a1608, a1609)
                                }, {
                                    let a1610 = v5;
                                    let a1611 = v10;
                                    append_bytes(a1610, a1611)
                                })))))
                            } else {
                                let a1612 = v9;
                                let a1613 = 32u64;
                                if if if if nat_eq(a1612, a1613) {
                                    true
                                } else {
                                    let a1614 = v9;
                                    let a1615 = 9u64;
                                    nat_eq(a1614, a1615)
                                } {
                                    true
                                } else {
                                    let a1616 = v9;
                                    let a1617 = 10u64;
                                    nat_eq(a1616, a1617)
                                } {
                                    true
                                } else {
                                    let a1618 = v9;
                                    let a1619 = 13u64;
                                    nat_eq(a1618, a1619)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1620 = v1;
                                        let a1621 = 1u64;
                                        nat_add(a1620, a1621)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1622 = v1;
                                            let a1623 = 4u64;
                                            nat_add(a1622, a1623)?
                                        }, (v6, (false, (false, {
                                            let a1624 = v5;
                                            let a1625 = f8(Str::lit("[]"));
                                            append_bytes(a1624, a1625)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1626 = v1;
                                            let a1627 = 1u64;
                                            nat_add(a1626, a1627)?
                                        }, (v6, ({
                                            let a1628 = v9;
                                            let a1629 = 34u64;
                                            nat_eq(a1628, a1629)
                                        }, (false, {
                                            let a1630 = v5;
                                            let a1631 = v10;
                                            append_bytes(a1630, a1631)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1632 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f318(v0: Bytes, v1: u64) -> R<u64> {
    let m1640 = {
        let a1638 = v0;
        let a1639 = v1;
        index_bytes(a1638, a1639)
    };
    match m1640 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1685 = {
                let a1661 = {
                    let a1649 = {
                        let a1643 = {
                            let a1641 = v3;
                            let a1642 = 128u8;
                            fixed_u8::bit_and(a1641, a1642)
                        };
                        let a1644 = 128u8;
                        if equal(a1643, a1644) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1650 = {
                        let a1647 = {
                            let a1645 = v3;
                            let a1646 = 64u8;
                            fixed_u8::bit_and(a1645, a1646)
                        };
                        let a1648 = 64u8;
                        if equal(a1647, a1648) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1649, a1650)?
                };
                let a1662 = {
                    let a1659 = {
                        let a1653 = {
                            let a1651 = v3;
                            let a1652 = 32u8;
                            fixed_u8::bit_and(a1651, a1652)
                        };
                        let a1654 = 32u8;
                        if equal(a1653, a1654) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1660 = {
                        let a1657 = {
                            let a1655 = v3;
                            let a1656 = 16u8;
                            fixed_u8::bit_and(a1655, a1656)
                        };
                        let a1658 = 16u8;
                        if equal(a1657, a1658) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1659, a1660)?
                };
                nat_add(a1661, a1662)?
            };
            let a1686 = {
                let a1683 = {
                    let a1671 = {
                        let a1665 = {
                            let a1663 = v3;
                            let a1664 = 8u8;
                            fixed_u8::bit_and(a1663, a1664)
                        };
                        let a1666 = 8u8;
                        if equal(a1665, a1666) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1672 = {
                        let a1669 = {
                            let a1667 = v3;
                            let a1668 = 4u8;
                            fixed_u8::bit_and(a1667, a1668)
                        };
                        let a1670 = 4u8;
                        if equal(a1669, a1670) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1671, a1672)?
                };
                let a1684 = {
                    let a1681 = {
                        let a1675 = {
                            let a1673 = v3;
                            let a1674 = 2u8;
                            fixed_u8::bit_and(a1673, a1674)
                        };
                        let a1676 = 2u8;
                        if equal(a1675, a1676) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1682 = {
                        let a1679 = {
                            let a1677 = v3;
                            let a1678 = 1u8;
                            fixed_u8::bit_and(a1677, a1678)
                        };
                        let a1680 = 1u8;
                        if equal(a1679, a1680) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1681, a1682)?
                };
                nat_add(a1683, a1684)?
            };
            nat_add(a1685, a1686)
        }
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f353(Fn15::F354(v0.clone()), {
        let a1688 = {
            let a1687 = v0;
            length_bytes(a1687)
        };
        let a1689 = 1u64;
        nat_add(a1688, a1689)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1693 = {
        let (_, h1690) = {
            let (_, h1691) = {
                let (h1692, _) = v2.clone();
                h1692
            };
            h1691
        };
        h1690
    };
    let a1694 = 1u64;
    if nat_eq(a1693, a1694) {
        Ok(Some(({
            let (h1695, _) = {
                let (h1696, _) = v2.clone();
                h1696
            };
            h1695
        }, {
            let (h1697, _) = {
                let (_, h1698) = {
                    let (h1699, _) = v2;
                    h1699
                };
                h1698
            };
            h1697
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f320(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1700 = f318(v0.clone(), v1)?;
        let a1701 = 45u64;
        nat_eq(a1700, a1701)
    };
    let v3: u64 = if v2 {
        let a1702 = v1;
        let a1703 = 1u64;
        nat_add(a1702, a1703)?
    } else {
        v1
    };
    let v4: u64 = f355(v0.clone(), v3)?;
    let a1706 = 19u64;
    let a1707 = {
        let a1704 = v4;
        let a1705 = v3;
        nat_sub(a1704, a1705)
    };
    if nat_lt(a1706, a1707) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1710 = f356(f32(v0, v3, {
            let a1708 = v4;
            let a1709 = v3;
            nat_sub(a1708, a1709)
        }))?;
        match m1710 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f357(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f321(v0: u64) -> bool {
    let a1711 = v0;
    let a1712 = 95u64;
    if nat_eq(a1711, a1712) {
        true
    } else {
        let a1713 = v0;
        let a1714 = 45u64;
        if nat_eq(a1713, a1714) {
            true
        } else {
            let a1715 = v0;
            let a1716 = 32u64;
            if nat_eq(a1715, a1716) {
                true
            } else {
                let a1717 = v0;
                let a1718 = 46u64;
                if nat_eq(a1717, a1718) {
                    true
                } else {
                    let a1719 = v0;
                    let a1720 = 47u64;
                    nat_eq(a1719, a1720)
                }
            }
        }
    }
}

pub fn f322(v0: u64) -> bool {
    let a1721 = 97u64;
    let a1722 = v0;
    if nat_le(a1721, a1722) {
        let a1723 = v0;
        let a1724 = 122u64;
        nat_le(a1723, a1724)
    } else {
        false
    }
}

pub fn f323(v0: u64) -> bool {
    let a1725 = 65u64;
    let a1726 = v0;
    if nat_le(a1725, a1726) {
        let a1727 = v0;
        let a1728 = 90u64;
        nat_le(a1727, a1728)
    } else {
        false
    }
}

pub fn f324(v0: u64) -> bool {
    let a1729 = 48u64;
    let a1730 = v0;
    if nat_le(a1729, a1730) {
        let a1731 = v0;
        let a1732 = 57u64;
        nat_le(a1731, a1732)
    } else {
        false
    }
}

pub fn f325(p1740: u64, p1741: List<Adt23>, p1742: List<Bytes>, p1743: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1739 = (p1740, (p1741, (p1742, p1743)));
    {
        let mut m1739 = m1739;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1739;
                let m1733 = v0;
                if m1733 == 0 {
                    Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1733 - 1;
                    let m1734 = v1;
                    match m1734.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1735 = v5;
                            match m1735 {
                                Adt23::C0(v8, v9) => {
                                    if f84(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m1736 = f358(v8.clone(), v9)?;
                                        match m1736 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f326({
                                                    let (_, h1737) = v11;
                                                    h1737
                                                }, List::cons(Adt23::C1({
                                                    let (h1738, _) = v11;
                                                    h1738
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
                Ok(next) => { m1739 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(v0: List<Adt23>, v1: List<Adt23>) -> List<Adt23> {
    let m1744 = v0;
    match m1744.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f326(v3, v1))
        }
    }
}

pub fn f327(v0: List<(Bytes, Adt14)>) -> R<List<Adt23>> {
    let m1745 = v0;
    match m1745.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt23::C0(f214({
                let (h1746, _) = v1;
                h1746
            })?, {
                let (_, h1747) = v1;
                h1747
            }), f327(v2)?))
        }
    }
}

pub fn f328(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1748 = v0;
    match m1748 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f359(v1)
        }
    }
}

pub fn f329(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1749 = v0;
    match m1749.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f36(f143(v3.clone(), v1.clone()), v2.clone()) {
                f329(v4, v1, v2)
            } else {
                List::cons(v3, f329(v4, v1, v2))
            }
        }
    }
}

pub fn f330(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1750 = v0;
    match m1750.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f190(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f200(f339(f76(List::cons(v1, f360(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f330(v3, v1)?))
            }
        }
    }
}

pub fn f331(v0: u64) -> R<Bytes> {
    f361(f362(), 10u64, v0)
}

pub fn f332(v0: Fn9, p1755: u64, p1756: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1754 = (p1755, p1756);
    {
        let mut m1754 = m1754;
        loop {
            let step = {
                let (v1, v2) = m1754;
                let m1751 = v1;
                if m1751 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1751 - 1;
                    let m1753 = {
                        let c1752: Fn9 = v0.clone();
                        c1752.apply(v2.clone())?
                    };
                    match m1753 {
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
                Ok(next) => { m1754 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f333(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f363(v0, v1)
}

pub fn f334(v0: Fn10, p1760: List<(Bytes, Bytes)>, p1761: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1759 = (p1760, p1761);
    {
        let mut m1759 = m1759;
        loop {
            let step = {
                let (v1, v2) = m1759;
                let m1757 = v2;
                match m1757.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1758: Fn10 = v0.clone();
                            c1758.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1759 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f364(v0, v1)
}

pub fn f336(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f365(v0)
}

pub fn f337(v0: Fn11, p1765: List<Bytes>, p1766: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1764 = (p1765, p1766);
    {
        let mut m1764 = m1764;
        loop {
            let step = {
                let (v1, v2) = m1764;
                let m1762 = v2;
                match m1762.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1763: Fn11 = v0.clone();
                            c1763.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1764 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f366(v1)?, v0))
}

pub fn f339(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1767 = v0;
    match m1767.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f195(v1)?, f339(v2)?))
        }
    }
}

pub fn f340(v0: Fn12, p1771: List<List<(Bytes, ())>>, p1772: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1770 = (p1771, p1772);
    {
        let mut m1770 = m1770;
        loop {
            let step = {
                let (v1, v2) = m1770;
                let m1768 = v2;
                match m1768.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1769: Fn12 = v0.clone();
                            c1769.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f342(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1774 = {
        let a1773 = v0.clone();
        length_list(a1773)
    };
    let a1775 = 1u64;
    if nat_le(a1774, a1775) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f367(v0)?))
    }
}

pub fn f343(v0: Fn13, p1779: List<(Bytes, ())>, p1780: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1778 = (p1779, p1780);
    {
        let mut m1778 = m1778;
        loop {
            let step = {
                let (v1, v2) = m1778;
                let m1776 = v2;
                match m1776.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1777: Fn13 = v0.clone();
                            c1777.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1778 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f345(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f70(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f368(v3.clone(), f70(f71(v0, f87())?, f8(Str::lit("at"))));
    let a1782 = {
        let a1781 = v4.clone();
        length_list(a1781)
    };
    let a1783 = 0u64;
    if nat_eq(a1782, a1783) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1786 = {
            let a1784 = v4;
            length_list(a1784)
        };
        let a1787 = {
            let a1785 = v3;
            length_list(a1785)
        };
        if nat_eq(a1786, a1787) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f369(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f346(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f210(f133(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f210(f133(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f62(f70(v3.clone(), f8(Str::lit("id"))), f70(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f70(f71(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f368(v5.clone(), f62(v6.clone(), f70(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1789 = {
        let a1788 = v7.clone();
        length_list(a1788)
    };
    let a1790 = 0u64;
    if nat_eq(a1789, a1790) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f73({
            let a1793 = {
                let a1791 = v7;
                length_list(a1791)
            };
            let a1794 = {
                let a1792 = v5;
                length_list(a1792)
            };
            nat_eq(a1793, a1794)
        }) {
            Ok((false, f369(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f76(f62(f368(f370(v4.clone()), f67(f68(v0, f8(Str::lit("model/authorities.toml"))))?), f368(f371(v4), f62(v6, f70(v3, f8(Str::lit("id")))))))?;
            let a1796 = {
                let a1795 = v8.clone();
                length_list(a1795)
            };
            let a1797 = 0u64;
            if nat_eq(a1796, a1797) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f369(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f119(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f347(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h1798, _) = v0;
        h1798
    }, {
        let (h1799, _) = v1;
        h1799
    }) {
        f36({
            let (_, h1800) = v0;
            h1800
        }, {
            let (_, h1801) = v1;
            h1801
        })
    } else {
        false
    }
}

pub fn f348(v0: Fn14, p1805: u64, p1806: Adt25) -> R<Adt25> {
    let m1804 = (p1805, p1806);
    {
        let mut m1804 = m1804;
        loop {
            let step = {
                let (v1, v2) = m1804;
                let m1802 = v1;
                if m1802 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m1802 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c1803: Fn14 = v0.clone();
                        c1803.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1804 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(v0: Adt25) -> R<Adt25> {
    f372(v0)
}

pub fn f350(v0: u32, v1: u32) -> u32 {
    let m1809 = {
        let a1807 = f373(v0);
        let a1808 = f373(v1);
        fixed_u64::checked_add(a1807, a1808)
    };
    match m1809 {
        None => {
            0u32
        }
        Some(v2) => {
            f374(v2)
        }
    }
}

pub fn f351(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1812 = {
        let a1810 = v3;
        let a1811 = 8u64;
        nat_add(a1810, a1811)?
    };
    let a1813 = v2;
    if nat_eq(a1812, a1813) {
        Ok(f375({
            let a1816 = {
                let a1814 = v1;
                let a1815 = 8u64;
                nat_mul(a1814, a1815)?
            };
            let a1817 = 4294967296u64;
            let a1818 = 0u64;
            nat_quot(a1816, a1817, a1818)
        }))
    } else {
        let a1821 = {
            let a1819 = v3;
            let a1820 = 4u64;
            nat_add(a1819, a1820)?
        };
        let a1822 = v2;
        if nat_eq(a1821, a1822) {
            Ok(f375({
                let a1825 = {
                    let a1823 = v1;
                    let a1824 = 8u64;
                    nat_mul(a1823, a1824)?
                };
                let a1826 = 4294967296u64;
                let a1827 = 0u64;
                nat_rem(a1825, a1826, a1827)
            }))
        } else {
            Ok(f376(f377(v0.clone(), v1, v3), f377(v0.clone(), v1, {
                let a1828 = v3;
                let a1829 = 1u64;
                nat_add(a1828, a1829)?
            }), f377(v0.clone(), v1, {
                let a1830 = v3;
                let a1831 = 2u64;
                nat_add(a1830, a1831)?
            }), f377(v0, v1, {
                let a1832 = v3;
                let a1833 = 3u64;
                nat_add(a1832, a1833)?
            })))
        }
    }
}

pub fn f352(v0: Bytes, v1: u64) -> bool {
    let m1834 = f222(v0, v1);
    match m1834 {
        None => {
            false
        }
        Some(v2) => {
            f378(v2)
        }
    }
}

pub fn f353(v0: Fn15, p1839: u64, p1840: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1838 = (p1839, p1840);
    {
        let mut m1838 = m1838;
        loop {
            let step = {
                let (v1, v2) = m1838;
                let m1835 = v1;
                if m1835 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1835 - 1;
                    let m1837 = {
                        let c1836: Fn15 = v0.clone();
                        c1836.apply(v2.clone())?
                    };
                    match m1837 {
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
                Ok(next) => { m1838 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f379(v0, v1)
}

pub fn f355(v0: Bytes, v1: u64) -> R<u64> {
    let (h1841, _) = f224(Fn3::F380(v0.clone()), {
        let a1843 = {
            let a1842 = v0;
            length_bytes(a1842)
        };
        let a1844 = 1u64;
        nat_add(a1843, a1844)?
    }, v1)?;
    Ok(h1841)
}

pub fn f356(v0: Bytes) -> R<Option<u64>> {
    if f381(v0.clone())? {
        Ok(Some({
            let (_, h1845) = {
                let (h1846, _) = f382(Fn16::F383(v0.clone()), {
                    let a1847 = v0;
                    length_bytes(a1847)
                }, (0u64, 0u64))?;
                h1846
            };
            h1845
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f357(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1848 = 0u64;
        let a1849 = v1;
        if nat_lt(a1848, a1849) {
            let a1850 = v1;
            let a1851 = 9223372036854775808u64;
            nat_le(a1850, a1851)
        } else {
            false
        }
    } else {
        let a1852 = v1;
        let a1853 = 9223372036854775807u64;
        nat_le(a1852, a1853)
    }
}

pub fn f358(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m1854 = f140(v1.clone(), f8(Str::lit("enum")));
    match m1854 {
        None => {
            let m1856 = f387(v0.clone(), f328(f140(v1.clone(), f8(Str::lit("properties")))), f72(f144(f140(v1, f8(Str::lit("required"))))))?;
            match m1856 {
                Ok(v6) => {
                    Ok(f386(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m1858 = {
                            let (h1857, _) = v6.clone();
                            h1857
                        };
                        match m1858.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f200({
                                    let (h1859, _) = v6;
                                    h1859
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1860) = v6;
                        h1860
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt23>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m1855 = f384(f218(v2), List::<Bytes>::nil())?;
            match m1855 {
                Ok(v4) => {
                    Ok(f386(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f200(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt23>::nil()))
                }
                Err(v3) => {
                    Ok(f385(v3, v0))
                }
            }
        }
    }
}

pub fn f359(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1861 = v0;
    match m1861 {
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
        Adt14::C4(r1862) => {
            let v4 = (*r1862).clone();
            f359(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1863, r1864) => {
            let v2 = (*r1863).clone();
            let v3 = (*r1864).clone();
            List::cons((v1, v2), f359(v3))
        }
    }
}

pub fn f360(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1865 = v0;
    match m1865.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f360(v3, false))
            } else {
                f360(v3, true)
            }
        }
    }
}

pub fn f361(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1866 = v2;
    let a1867 = 0u64;
    if nat_eq(a1866, a1867) {
        Ok(f388(v0, 0u64))
    } else {
        let (_, h1868) = {
            let (h1869, _) = f332(Fn9::F389(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1869
        };
        Ok(h1868)
    }
}

pub fn f362() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f363(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1873 = {
        let a1871 = v0.clone();
        let a1872 = {
            let (h1870, _) = v1.clone();
            h1870
        };
        index_bytes(a1871, a1872)
    };
    match m1873 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1875 = {
                    let (h1874, _) = v1.clone();
                    h1874
                };
                let a1876 = 1u64;
                nat_add(a1875, a1876)?
            }, {
                let a1879 = {
                    let (_, h1877) = v1;
                    h1877
                };
                let a1880 = f390(v0, {
                    let (h1878, _) = v1;
                    h1878
                }, v2)?;
                append_bytes(a1879, a1880)
            })))
        }
    }
}

pub fn f364(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1881 = v0.clone();
    match m1881.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1884 = {
                let (h1882, _) = v2;
                h1882
            };
            let a1885 = {
                let (h1883, _) = v1.clone();
                h1883
            };
            if equal(a1884, a1885) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f365(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f391(f392(Fn17::F393, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1890 = {
        let (h1886, _) = f394(Fn18::F395, {
            let a1888 = {
                let a1887 = v0;
                length_list(a1887)
            };
            let a1889 = 1u64;
            nat_add(a1888, a1889)?
        }, v1)?;
        h1886
    };
    match m1890.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f366(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1895 = {
        let a1892 = f283({
            let (h1891, _) = v0;
            h1891
        })?;
        let a1893 = Bytes::lit(&[0x3a]);
        append_bytes(a1892, a1893)
    };
    let a1896 = {
        let (_, h1894) = v0;
        h1894
    };
    Ok(append_bytes(a1895, a1896))
}

pub fn f367(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f297({
        let (_, h1897) = {
            let (h1898, _) = f396(Fn19::F397, {
                let a1900 = {
                    let a1899 = v0.clone();
                    length_list(a1899)
                };
                let a1901 = 1u64;
                nat_add(a1900, a1901)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1898
        };
        h1897
    }))
}

pub fn f368(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1902 = v0;
    match m1902.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84(v2.clone(), v1.clone()) {
                f368(v3, v1)
            } else {
                List::cons(v2, f368(v3, v1))
            }
        }
    }
}

pub fn f369(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f370(v0: List<Adt14>) -> List<Bytes> {
    let m1903 = v0;
    match m1903.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1904 = f140(v1, f8(Str::lit("std")));
            match m1904 {
                None => {
                    f370(v2)
                }
                Some(v3) => {
                    List::cons(f143(v3, f8(Str::lit("authority"))), f370(v2))
                }
            }
        }
    }
}

pub fn f371(v0: List<Adt14>) -> List<Bytes> {
    let m1905 = v0;
    match m1905.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1906 = f148(f140(v1, f8(Str::lit("req"))));
            match m1906 {
                None => {
                    f371(v2)
                }
                Some(v3) => {
                    List::cons(v3, f371(v2))
                }
            }
        }
    }
}

pub fn f372(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f398(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h1907) => {
            h1907
        }
    });
    let v2: u32 = f350(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h1908, _, _) => {
            h1908
        }
    }, f350(f399(match v0.clone() {
        Adt25::C0(_, _, _, _, h1909, _, _, _, _, _) => {
            h1909
        }
    }), f350(f400(match v0.clone() {
        Adt25::C0(_, _, _, _, h1910, _, _, _, _, _) => {
            h1910
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1911, _, _, _, _) => {
            h1911
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1912, _, _, _) => {
            h1912
        }
    }), f350(v1, {
        let m1915 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h1914, _) => {
                h1914
            }
        };
        match m1915 {
            Adt24::C0(h1913, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1913
            }
        }
    }))));
    let v3: u32 = f350(f401(match v0.clone() {
        Adt25::C0(h1916, _, _, _, _, _, _, _, _, _) => {
            h1916
        }
    }), f402(match v0.clone() {
        Adt25::C0(h1917, _, _, _, _, _, _, _, _, _) => {
            h1917
        }
    }, match v0.clone() {
        Adt25::C0(_, h1918, _, _, _, _, _, _, _, _) => {
            h1918
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1919, _, _, _, _, _, _, _) => {
            h1919
        }
    }));
    Ok(Adt25::C0(f350(v2, v3), match v0.clone() {
        Adt25::C0(h1920, _, _, _, _, _, _, _, _, _) => {
            h1920
        }
    }, match v0.clone() {
        Adt25::C0(_, h1921, _, _, _, _, _, _, _, _) => {
            h1921
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1922, _, _, _, _, _, _, _) => {
            h1922
        }
    }, f350(match v0.clone() {
        Adt25::C0(_, _, _, h1923, _, _, _, _, _, _) => {
            h1923
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h1924, _, _, _, _, _) => {
            h1924
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1925, _, _, _, _) => {
            h1925
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1926, _, _, _) => {
            h1926
        }
    }, f403(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h1927, _) => {
            h1927
        }
    }), {
        let a1929 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h1928) => {
                h1928
            }
        };
        let a1930 = 1u64;
        nat_add(a1929, a1930)?
    }))
}

pub fn f373(v0: u32) -> u64 {
    let m1932 = {
        let a1931 = v0;
        fixed_u64::convert(i128::from(a1931))
    };
    match m1932 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f374(v0: u64) -> u32 {
    let m1936 = {
        let a1935 = {
            let a1933 = v0;
            let a1934 = 4294967295u64;
            fixed_u64::bit_and(a1933, a1934)
        };
        fixed_u32::convert(i128::from(a1935))
    };
    match m1936 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f375(v0: u64) -> u32 {
    let (_, h1937) = {
        let (_, h1938) = f404(Fn20::F405, 32u64, (v0, (1u32, 0u32)));
        h1938
    };
    h1937
}

pub fn f376(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1943 = f406(v0, 24u32);
    let a1944 = {
        let a1941 = f406(v1, 16u32);
        let a1942 = {
            let a1939 = f406(v2, 8u32);
            let a1940 = v3;
            fixed_u32::bit_or(a1939, a1940)
        };
        fixed_u32::bit_or(a1941, a1942)
    };
    fixed_u32::bit_or(a1943, a1944)
}

pub fn f377(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1945 = v2;
    let a1946 = v1;
    if nat_lt(a1945, a1946) {
        let m1949 = {
            let a1947 = v0;
            let a1948 = v2;
            index_bytes(a1947, a1948)
        };
        match m1949 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1951 = {
                    let a1950 = v3;
                    fixed_u32::convert(i128::from(a1950))
                };
                match m1951 {
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
        let a1952 = v2;
        let a1953 = v1;
        if nat_eq(a1952, a1953) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f378(v0: u8) -> bool {
    let a1954 = v0;
    let a1955 = 32u8;
    if if if equal(a1954, a1955) {
        true
    } else {
        let a1956 = v0;
        let a1957 = 9u8;
        equal(a1956, a1957)
    } {
        true
    } else {
        let a1958 = v0;
        let a1959 = 10u8;
        equal(a1958, a1959)
    } {
        true
    } else {
        let a1960 = v0;
        let a1961 = 12u8;
        if equal(a1960, a1961) {
            true
        } else {
            let a1962 = v0;
            let a1963 = 13u8;
            equal(a1962, a1963)
        }
    }
}

pub fn f379(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1964, _) = v1.clone();
        h1964
    };
    let v3: Bytes = {
        let (h1965, _) = {
            let (_, h1966) = v1.clone();
            h1966
        };
        h1965
    };
    let a1969 = {
        let (_, h1967) = {
            let (_, h1968) = v1;
            h1968
        };
        h1967
    };
    let a1970 = 0u64;
    if nat_eq(a1969, a1970) {
        let v4: u64 = f318(v0.clone(), v2)?;
        let a1971 = v4;
        let a1972 = 34u64;
        if nat_eq(a1971, a1972) {
            Ok(Some(({
                let a1973 = v2;
                let a1974 = 1u64;
                nat_add(a1973, a1974)?
            }, (v3, 1u64))))
        } else {
            let a1975 = v4;
            let a1976 = 92u64;
            if nat_eq(a1975, a1976) {
                f407(v0, v2, v3)
            } else {
                let a1977 = v4;
                let a1978 = 32u64;
                if if nat_lt(a1977, a1978) {
                    true
                } else {
                    let a1979 = v4;
                    let a1980 = 256u64;
                    nat_eq(a1979, a1980)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1981 = v2;
                        let a1982 = 1u64;
                        nat_add(a1981, a1982)?
                    }, ({
                        let a1983 = v3;
                        let a1984 = f32(v0, v2, 1u64);
                        append_bytes(a1983, a1984)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f380(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f408(v0, v1)
}

pub fn f381(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1985 = v0.clone();
        length_bytes(a1985)
    };
    let a1986 = 0u64;
    let a1987 = v1;
    if nat_lt(a1986, a1987) {
        let a1988 = f409(v0.clone())?;
        let a1989 = v1;
        if nat_eq(a1988, a1989) {
            let a1990 = 1u64;
            let a1991 = v1;
            if nat_lt(a1990, a1991) {
                let m1994 = {
                    let a1992 = v0;
                    let a1993 = 0u64;
                    index_bytes(a1992, a1993)
                };
                match m1994 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1995 = v2;
                        let a1996 = 48u8;
                        Ok(!equal(a1995, a1996))
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

pub fn f382(v0: Fn16, p2001: u64, p2002: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2000 = (p2001, p2002);
    {
        let mut m2000 = m2000;
        loop {
            let step = {
                let (v1, v2) = m2000;
                let m1997 = v1;
                if m1997 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1997 - 1;
                    let m1999 = {
                        let c1998: Fn16 = v0.clone();
                        c1998.apply(v2)?
                    };
                    match m1999 {
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
                Ok(next) => { m2000 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f410(v0, v1)
}

pub fn f384(v0: List<Adt14>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2003 = v0;
    match m2003.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v2, v3)) => {
            let m2004 = f411(v2.clone())?;
            match m2004 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f412(v2)?, List::<Bytes>::nil())))))
                }
                Some(v4) => {
                    if f84(v4.clone(), v1.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v4, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2005 = f384(v3, List::cons(v4.clone(), v1))?;
                        match m2005 {
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

pub fn f385(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt23>), Bytes> {
    Err::<(Bytes, List<Adt23>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f386(v0: Bytes, v1: List<Adt23>) -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((v0, v1))
}

pub fn f387(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2006 = v1;
    match m2006.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f413({
                let (h2007, _) = v3.clone();
                h2007
            })?;
            if f414(v5.clone())? {
                let m2012 = f415({
                    let (_, h2008) = v3.clone();
                    h2008
                }, {
                    let a2010 = v0.clone();
                    let a2011 = f214({
                        let (h2009, _) = v3.clone();
                        h2009
                    })?;
                    append_bytes(a2010, a2011)
                })?;
                match m2012 {
                    Ok(v7) => {
                        let v8: Bytes = if f84({
                            let (h2013, _) = v3;
                            h2013
                        }, v2.clone()) {
                            let (h2014, _) = v7;
                            h2014
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h2015, _) = v7;
                                h2015
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m2016 = f387(v0, v4, v2)?;
                        match m2016 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h2017, _) = v10;
                                    h2017
                                }), f326({
                                    let (_, h2018) = v7;
                                    h2018
                                }, {
                                    let (_, h2019) = v10;
                                    h2019
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
                    let (h2020, _) = v3;
                    h2020
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f388(v0: Bytes, v1: u64) -> Bytes {
    let m2024 = {
        let a2021 = v0;
        let a2022 = v1;
        let a2023 = 1u64;
        slice_bytes(a2021, a2022, a2023)
    };
    match m2024 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f389(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f416(v1, v0, v2)
}

pub fn f390(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2069 = {
            let a2045 = {
                let a2033 = {
                    let a2027 = {
                        let a2025 = v3;
                        let a2026 = 128u8;
                        fixed_u8::bit_and(a2025, a2026)
                    };
                    let a2028 = 128u8;
                    if equal(a2027, a2028) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2034 = {
                    let a2031 = {
                        let a2029 = v3;
                        let a2030 = 64u8;
                        fixed_u8::bit_and(a2029, a2030)
                    };
                    let a2032 = 64u8;
                    if equal(a2031, a2032) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2033, a2034)?
            };
            let a2046 = {
                let a2043 = {
                    let a2037 = {
                        let a2035 = v3;
                        let a2036 = 32u8;
                        fixed_u8::bit_and(a2035, a2036)
                    };
                    let a2038 = 32u8;
                    if equal(a2037, a2038) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2044 = {
                    let a2041 = {
                        let a2039 = v3;
                        let a2040 = 16u8;
                        fixed_u8::bit_and(a2039, a2040)
                    };
                    let a2042 = 16u8;
                    if equal(a2041, a2042) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2043, a2044)?
            };
            nat_add(a2045, a2046)?
        };
        let a2070 = {
            let a2067 = {
                let a2055 = {
                    let a2049 = {
                        let a2047 = v3;
                        let a2048 = 8u8;
                        fixed_u8::bit_and(a2047, a2048)
                    };
                    let a2050 = 8u8;
                    if equal(a2049, a2050) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2056 = {
                    let a2053 = {
                        let a2051 = v3;
                        let a2052 = 4u8;
                        fixed_u8::bit_and(a2051, a2052)
                    };
                    let a2054 = 4u8;
                    if equal(a2053, a2054) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2055, a2056)?
            };
            let a2068 = {
                let a2065 = {
                    let a2059 = {
                        let a2057 = v3;
                        let a2058 = 2u8;
                        fixed_u8::bit_and(a2057, a2058)
                    };
                    let a2060 = 2u8;
                    if equal(a2059, a2060) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2066 = {
                    let a2063 = {
                        let a2061 = v3;
                        let a2062 = 1u8;
                        fixed_u8::bit_and(a2061, a2062)
                    };
                    let a2064 = 1u8;
                    if equal(a2063, a2064) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2065, a2066)?
            };
            nat_add(a2067, a2068)?
        };
        nat_add(a2069, a2070)?
    };
    let a2071 = v4;
    let a2072 = 34u64;
    if nat_eq(a2071, a2072) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2073 = v4;
        let a2074 = 92u64;
        if nat_eq(a2073, a2074) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2075 = v4;
            let a2076 = 8u64;
            if nat_eq(a2075, a2076) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2077 = v4;
                let a2078 = 12u64;
                if nat_eq(a2077, a2078) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2079 = v4;
                    let a2080 = 10u64;
                    if nat_eq(a2079, a2080) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2081 = v4;
                        let a2082 = 13u64;
                        if nat_eq(a2081, a2082) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2083 = v4;
                            let a2084 = 9u64;
                            if nat_eq(a2083, a2084) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2085 = v4;
                                let a2086 = 32u64;
                                if nat_lt(a2085, a2086) {
                                    let a2087 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2088 = f417(v2)?;
                                    Ok(append_bytes(a2087, a2088))
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
    f418(Fn21::F419, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f392(v0: Fn17, p2092: List<List<(Bytes, Bytes)>>, p2093: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2091 = (p2092, p2093);
    {
        let mut m2091 = m2091;
        loop {
            let step = {
                let (v1, v2) = m2091;
                let m2089 = v2;
                match m2089.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2090: Fn17 = v0.clone();
                            c2090.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2091 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f394(v0: Fn18, p2098: u64, p2099: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2097 = (p2098, p2099);
    {
        let mut m2097 = m2097;
        loop {
            let step = {
                let (v1, v2) = m2097;
                let m2094 = v1;
                if m2094 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2094 - 1;
                    let m2096 = {
                        let c2095: Fn18 = v0.clone();
                        c2095.apply(v2.clone())?
                    };
                    match m2096 {
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
                Ok(next) => { m2097 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f420(v0)
}

pub fn f396(v0: Fn19, p2104: u64, p2105: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2103 = (p2104, p2105);
    {
        let mut m2103 = m2103;
        loop {
            let step = {
                let (v1, v2) = m2103;
                let m2100 = v1;
                if m2100 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2100 - 1;
                    let m2102 = {
                        let c2101: Fn19 = v0.clone();
                        c2101.apply(v2.clone())?
                    };
                    match m2102 {
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
                Ok(next) => { m2103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f421(v0)
}

pub fn f398(v0: u64) -> u32 {
    let a2106 = v0;
    let a2107 = 32u64;
    if nat_lt(a2106, a2107) {
        let a2108 = v0;
        let a2109 = 16u64;
        if nat_lt(a2108, a2109) {
            let a2110 = v0;
            let a2111 = 8u64;
            if nat_lt(a2110, a2111) {
                let a2112 = v0;
                let a2113 = 4u64;
                if nat_lt(a2112, a2113) {
                    let a2114 = v0;
                    let a2115 = 2u64;
                    if nat_lt(a2114, a2115) {
                        let a2116 = v0;
                        let a2117 = 1u64;
                        if nat_lt(a2116, a2117) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2118 = v0;
                        let a2119 = 3u64;
                        if nat_lt(a2118, a2119) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2120 = v0;
                    let a2121 = 6u64;
                    if nat_lt(a2120, a2121) {
                        let a2122 = v0;
                        let a2123 = 5u64;
                        if nat_lt(a2122, a2123) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2124 = v0;
                        let a2125 = 7u64;
                        if nat_lt(a2124, a2125) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2126 = v0;
                let a2127 = 12u64;
                if nat_lt(a2126, a2127) {
                    let a2128 = v0;
                    let a2129 = 10u64;
                    if nat_lt(a2128, a2129) {
                        let a2130 = v0;
                        let a2131 = 9u64;
                        if nat_lt(a2130, a2131) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2132 = v0;
                        let a2133 = 11u64;
                        if nat_lt(a2132, a2133) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2134 = v0;
                    let a2135 = 14u64;
                    if nat_lt(a2134, a2135) {
                        let a2136 = v0;
                        let a2137 = 13u64;
                        if nat_lt(a2136, a2137) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2138 = v0;
                        let a2139 = 15u64;
                        if nat_lt(a2138, a2139) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2140 = v0;
            let a2141 = 24u64;
            if nat_lt(a2140, a2141) {
                let a2142 = v0;
                let a2143 = 20u64;
                if nat_lt(a2142, a2143) {
                    let a2144 = v0;
                    let a2145 = 18u64;
                    if nat_lt(a2144, a2145) {
                        let a2146 = v0;
                        let a2147 = 17u64;
                        if nat_lt(a2146, a2147) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2148 = v0;
                        let a2149 = 19u64;
                        if nat_lt(a2148, a2149) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2150 = v0;
                    let a2151 = 22u64;
                    if nat_lt(a2150, a2151) {
                        let a2152 = v0;
                        let a2153 = 21u64;
                        if nat_lt(a2152, a2153) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2154 = v0;
                        let a2155 = 23u64;
                        if nat_lt(a2154, a2155) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2156 = v0;
                let a2157 = 28u64;
                if nat_lt(a2156, a2157) {
                    let a2158 = v0;
                    let a2159 = 26u64;
                    if nat_lt(a2158, a2159) {
                        let a2160 = v0;
                        let a2161 = 25u64;
                        if nat_lt(a2160, a2161) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2162 = v0;
                        let a2163 = 27u64;
                        if nat_lt(a2162, a2163) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2164 = v0;
                    let a2165 = 30u64;
                    if nat_lt(a2164, a2165) {
                        let a2166 = v0;
                        let a2167 = 29u64;
                        if nat_lt(a2166, a2167) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2168 = v0;
                        let a2169 = 31u64;
                        if nat_lt(a2168, a2169) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2170 = v0;
        let a2171 = 48u64;
        if nat_lt(a2170, a2171) {
            let a2172 = v0;
            let a2173 = 40u64;
            if nat_lt(a2172, a2173) {
                let a2174 = v0;
                let a2175 = 36u64;
                if nat_lt(a2174, a2175) {
                    let a2176 = v0;
                    let a2177 = 34u64;
                    if nat_lt(a2176, a2177) {
                        let a2178 = v0;
                        let a2179 = 33u64;
                        if nat_lt(a2178, a2179) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2180 = v0;
                        let a2181 = 35u64;
                        if nat_lt(a2180, a2181) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2182 = v0;
                    let a2183 = 38u64;
                    if nat_lt(a2182, a2183) {
                        let a2184 = v0;
                        let a2185 = 37u64;
                        if nat_lt(a2184, a2185) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2186 = v0;
                        let a2187 = 39u64;
                        if nat_lt(a2186, a2187) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2188 = v0;
                let a2189 = 44u64;
                if nat_lt(a2188, a2189) {
                    let a2190 = v0;
                    let a2191 = 42u64;
                    if nat_lt(a2190, a2191) {
                        let a2192 = v0;
                        let a2193 = 41u64;
                        if nat_lt(a2192, a2193) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2194 = v0;
                        let a2195 = 43u64;
                        if nat_lt(a2194, a2195) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2196 = v0;
                    let a2197 = 46u64;
                    if nat_lt(a2196, a2197) {
                        let a2198 = v0;
                        let a2199 = 45u64;
                        if nat_lt(a2198, a2199) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2200 = v0;
                        let a2201 = 47u64;
                        if nat_lt(a2200, a2201) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2202 = v0;
            let a2203 = 56u64;
            if nat_lt(a2202, a2203) {
                let a2204 = v0;
                let a2205 = 52u64;
                if nat_lt(a2204, a2205) {
                    let a2206 = v0;
                    let a2207 = 50u64;
                    if nat_lt(a2206, a2207) {
                        let a2208 = v0;
                        let a2209 = 49u64;
                        if nat_lt(a2208, a2209) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2210 = v0;
                        let a2211 = 51u64;
                        if nat_lt(a2210, a2211) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2212 = v0;
                    let a2213 = 54u64;
                    if nat_lt(a2212, a2213) {
                        let a2214 = v0;
                        let a2215 = 53u64;
                        if nat_lt(a2214, a2215) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2216 = v0;
                        let a2217 = 55u64;
                        if nat_lt(a2216, a2217) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2218 = v0;
                let a2219 = 60u64;
                if nat_lt(a2218, a2219) {
                    let a2220 = v0;
                    let a2221 = 58u64;
                    if nat_lt(a2220, a2221) {
                        let a2222 = v0;
                        let a2223 = 57u64;
                        if nat_lt(a2222, a2223) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2224 = v0;
                        let a2225 = 59u64;
                        if nat_lt(a2224, a2225) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2226 = v0;
                    let a2227 = 62u64;
                    if nat_lt(a2226, a2227) {
                        let a2228 = v0;
                        let a2229 = 61u64;
                        if nat_lt(a2228, a2229) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2230 = v0;
                        let a2231 = 63u64;
                        if nat_lt(a2230, a2231) {
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
    let a2234 = {
        let a2232 = f422(v0, 6u32, 26u32);
        let a2233 = f422(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2232, a2233)
    };
    let a2235 = f422(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2234, a2235)
}

pub fn f400(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2241 = {
        let a2236 = v0;
        let a2237 = v1;
        fixed_u32::bit_and(a2236, a2237)
    };
    let a2242 = {
        let a2239 = {
            let a2238 = v0;
            fixed_u32::bit_not(a2238)
        };
        let a2240 = v2;
        fixed_u32::bit_and(a2239, a2240)
    };
    fixed_u32::bit_xor(a2241, a2242)
}

pub fn f401(v0: u32) -> u32 {
    let a2245 = {
        let a2243 = f422(v0, 2u32, 30u32);
        let a2244 = f422(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2243, a2244)
    };
    let a2246 = f422(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2245, a2246)
}

pub fn f402(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2255 = {
        let a2251 = {
            let a2247 = v0;
            let a2248 = v1;
            fixed_u32::bit_and(a2247, a2248)
        };
        let a2252 = {
            let a2249 = v0;
            let a2250 = v2;
            fixed_u32::bit_and(a2249, a2250)
        };
        fixed_u32::bit_xor(a2251, a2252)
    };
    let a2256 = {
        let a2253 = v1;
        let a2254 = v2;
        fixed_u32::bit_and(a2253, a2254)
    };
    fixed_u32::bit_xor(a2255, a2256)
}

pub fn f403(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h2257, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2257
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2258, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2258
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2259, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2259
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2260, _, _, _, _, _, _, _, _, _, _, _) => {
            h2260
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2261, _, _, _, _, _, _, _, _, _, _) => {
            h2261
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2262, _, _, _, _, _, _, _, _, _) => {
            h2262
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2263, _, _, _, _, _, _, _, _) => {
            h2263
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h2264, _, _, _, _, _, _, _) => {
            h2264
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2265, _, _, _, _, _, _) => {
            h2265
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h2266, _, _, _, _, _) => {
            h2266
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h2267, _, _, _, _) => {
            h2267
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2268, _, _, _) => {
            h2268
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2269, _, _) => {
            h2269
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2270, _) => {
            h2270
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2271) => {
            h2271
        }
    }, f350(f423(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2272, _) => {
            h2272
        }
    }), f350(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2273, _, _, _, _, _, _) => {
            h2273
        }
    }, f350(f424(match v0.clone() {
        Adt24::C0(_, h2274, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2274
        }
    }), match v0 {
        Adt24::C0(h2275, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2275
        }
    }))))
}

pub fn f404(v0: Fn20, p2279: u64, p2280: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2278 = (p2279, p2280);
    {
        let mut m2278 = m2278;
        loop {
            let step = {
                let (v1, v2) = m2278;
                let m2276 = v1;
                if m2276 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2276 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2277: Fn20 = v0.clone();
                        c2277.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2281, _) = v0;
        h2281
    };
    let v2: u32 = {
        let (h2282, _) = {
            let (_, h2283) = v0;
            h2283
        };
        h2282
    };
    let v3: u32 = {
        let (_, h2284) = {
            let (_, h2285) = v0;
            h2285
        };
        h2284
    };
    ({
        let a2286 = v1;
        let a2287 = 2u64;
        let a2288 = 0u64;
        nat_quot(a2286, a2287, a2288)
    }, (f406(v2, 1u32), {
        let a2292 = {
            let a2289 = v1;
            let a2290 = 2u64;
            let a2291 = 0u64;
            nat_rem(a2289, a2290, a2291)
        };
        let a2293 = 1u64;
        if nat_eq(a2292, a2293) {
            let a2294 = v3;
            let a2295 = v2;
            fixed_u32::bit_or(a2294, a2295)
        } else {
            v3
        }
    }))
}

pub fn f406(v0: u32, v1: u32) -> u32 {
    let m2298 = {
        let a2296 = v0;
        let a2297 = v1;
        fixed_u32::shift_left(a2296, a2297)
    };
    match m2298 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f407(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f318(v0.clone(), {
        let a2299 = v1;
        let a2300 = 1u64;
        nat_add(a2299, a2300)?
    })?;
    let v4: Bytes = {
        let a2301 = v3;
        let a2302 = 117u64;
        if nat_eq(a2301, a2302) {
            f425(v0, v1)?
        } else {
            f426(v3)
        }
    };
    let a2303 = v4.clone();
    let a2304 = Bytes::lit(&[]);
    if equal(a2303, a2304) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2305 = v3;
            let a2306 = 117u64;
            if nat_eq(a2305, a2306) {
                let a2307 = v1;
                let a2308 = 6u64;
                nat_add(a2307, a2308)?
            } else {
                let a2309 = v1;
                let a2310 = 2u64;
                nat_add(a2309, a2310)?
            }
        }, ({
            let a2311 = v2;
            let a2312 = v4;
            append_bytes(a2311, a2312)
        }, 0u64))))
    }
}

pub fn f408(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f318(v0, v1)?;
    let a2313 = 47u64;
    let a2314 = v2;
    if if nat_lt(a2313, a2314) {
        let a2315 = v2;
        let a2316 = 58u64;
        nat_lt(a2315, a2316)
    } else {
        false
    } {
        Ok(Some({
            let a2317 = v1;
            let a2318 = 1u64;
            nat_add(a2317, a2318)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f409(v0: Bytes) -> R<u64> {
    let (h2319, _) = f224(Fn3::F427(v0.clone()), {
        let a2320 = v0;
        length_bytes(a2320)
    }, 0u64)?;
    Ok(h2319)
}

pub fn f410(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2324 = {
        let a2322 = v0;
        let a2323 = {
            let (h2321, _) = v1;
            h2321
        };
        index_bytes(a2322, a2323)
    };
    match m2324 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2326 = {
                    let (h2325, _) = v1;
                    h2325
                };
                let a2327 = 1u64;
                nat_add(a2326, a2327)?
            }, {
                let a2331 = {
                    let a2329 = {
                        let (_, h2328) = v1;
                        h2328
                    };
                    let a2330 = 10u64;
                    nat_mul(a2329, a2330)?
                };
                let a2332 = f428(v2)?;
                nat_add(a2331, a2332)?
            })))
        }
    }
}

pub fn f411(v0: Adt14) -> R<Option<Bytes>> {
    let m2333 = v0;
    match m2333 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v2, v3) => {
            Ok(Some({
                let a2336 = if if v2 {
                    let a2334 = 0u64;
                    let a2335 = v3;
                    nat_lt(a2334, a2335)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2337 = f282(false, v3)?;
                append_bytes(a2336, a2337)
            }))
        }
        Adt14::C2(v1) => {
            f429(v1)
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

pub fn f412(v0: Adt14) -> R<Bytes> {
    f217(v0)
}

pub fn f413(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f278(v0.clone(), 0u64, {
        let a2338 = v0;
        length_bytes(a2338)
    }, false, Bytes::lit(&[]))?;
    let m2341 = {
        let a2339 = v1.clone();
        let a2340 = 0u64;
        index_bytes(a2339, a2340)
    };
    match m2341 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2386 = {
                    let a2362 = {
                        let a2350 = {
                            let a2344 = {
                                let a2342 = v3;
                                let a2343 = 128u8;
                                fixed_u8::bit_and(a2342, a2343)
                            };
                            let a2345 = 128u8;
                            if equal(a2344, a2345) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2351 = {
                            let a2348 = {
                                let a2346 = v3;
                                let a2347 = 64u8;
                                fixed_u8::bit_and(a2346, a2347)
                            };
                            let a2349 = 64u8;
                            if equal(a2348, a2349) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2350, a2351)?
                    };
                    let a2363 = {
                        let a2360 = {
                            let a2354 = {
                                let a2352 = v3;
                                let a2353 = 32u8;
                                fixed_u8::bit_and(a2352, a2353)
                            };
                            let a2355 = 32u8;
                            if equal(a2354, a2355) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2361 = {
                            let a2358 = {
                                let a2356 = v3;
                                let a2357 = 16u8;
                                fixed_u8::bit_and(a2356, a2357)
                            };
                            let a2359 = 16u8;
                            if equal(a2358, a2359) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2360, a2361)?
                    };
                    nat_add(a2362, a2363)?
                };
                let a2387 = {
                    let a2384 = {
                        let a2372 = {
                            let a2366 = {
                                let a2364 = v3;
                                let a2365 = 8u8;
                                fixed_u8::bit_and(a2364, a2365)
                            };
                            let a2367 = 8u8;
                            if equal(a2366, a2367) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2373 = {
                            let a2370 = {
                                let a2368 = v3;
                                let a2369 = 4u8;
                                fixed_u8::bit_and(a2368, a2369)
                            };
                            let a2371 = 4u8;
                            if equal(a2370, a2371) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2372, a2373)?
                    };
                    let a2385 = {
                        let a2382 = {
                            let a2376 = {
                                let a2374 = v3;
                                let a2375 = 2u8;
                                fixed_u8::bit_and(a2374, a2375)
                            };
                            let a2377 = 2u8;
                            if equal(a2376, a2377) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2383 = {
                            let a2380 = {
                                let a2378 = v3;
                                let a2379 = 1u8;
                                fixed_u8::bit_and(a2378, a2379)
                            };
                            let a2381 = 1u8;
                            if equal(a2380, a2381) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2382, a2383)?
                    };
                    nat_add(a2384, a2385)?
                };
                nat_add(a2386, a2387)?
            };
            if f323(v4) {
                let a2394 = f32({
                    let a2388 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2388)
                }, {
                    let a2389 = v4;
                    let a2390 = 65u64;
                    nat_sub(a2389, a2390)
                }, 1u64);
                let a2395 = f32(v1.clone(), 1u64, {
                    let a2392 = {
                        let a2391 = v1;
                        length_bytes(a2391)
                    };
                    let a2393 = 1u64;
                    nat_sub(a2392, a2393)
                });
                Ok(append_bytes(a2394, a2395))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f414(v0: Bytes) -> R<bool> {
    let m2398 = {
        let a2396 = v0;
        let a2397 = 0u64;
        index_bytes(a2396, a2397)
    };
    match m2398 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f322({
                let v2: u8 = v1;
                let a2443 = {
                    let a2419 = {
                        let a2407 = {
                            let a2401 = {
                                let a2399 = v2;
                                let a2400 = 128u8;
                                fixed_u8::bit_and(a2399, a2400)
                            };
                            let a2402 = 128u8;
                            if equal(a2401, a2402) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2408 = {
                            let a2405 = {
                                let a2403 = v2;
                                let a2404 = 64u8;
                                fixed_u8::bit_and(a2403, a2404)
                            };
                            let a2406 = 64u8;
                            if equal(a2405, a2406) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2407, a2408)?
                    };
                    let a2420 = {
                        let a2417 = {
                            let a2411 = {
                                let a2409 = v2;
                                let a2410 = 32u8;
                                fixed_u8::bit_and(a2409, a2410)
                            };
                            let a2412 = 32u8;
                            if equal(a2411, a2412) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2418 = {
                            let a2415 = {
                                let a2413 = v2;
                                let a2414 = 16u8;
                                fixed_u8::bit_and(a2413, a2414)
                            };
                            let a2416 = 16u8;
                            if equal(a2415, a2416) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2417, a2418)?
                    };
                    nat_add(a2419, a2420)?
                };
                let a2444 = {
                    let a2441 = {
                        let a2429 = {
                            let a2423 = {
                                let a2421 = v2;
                                let a2422 = 8u8;
                                fixed_u8::bit_and(a2421, a2422)
                            };
                            let a2424 = 8u8;
                            if equal(a2423, a2424) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2430 = {
                            let a2427 = {
                                let a2425 = v2;
                                let a2426 = 4u8;
                                fixed_u8::bit_and(a2425, a2426)
                            };
                            let a2428 = 4u8;
                            if equal(a2427, a2428) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2429, a2430)?
                    };
                    let a2442 = {
                        let a2439 = {
                            let a2433 = {
                                let a2431 = v2;
                                let a2432 = 2u8;
                                fixed_u8::bit_and(a2431, a2432)
                            };
                            let a2434 = 2u8;
                            if equal(a2433, a2434) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2440 = {
                            let a2437 = {
                                let a2435 = v2;
                                let a2436 = 1u8;
                                fixed_u8::bit_and(a2435, a2436)
                            };
                            let a2438 = 1u8;
                            if equal(a2437, a2438) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2439, a2440)?
                    };
                    nat_add(a2441, a2442)?
                };
                nat_add(a2443, a2444)?
            }) {
                Ok(true)
            } else {
                Ok(f323({
                    let v3: u8 = v1;
                    let a2489 = {
                        let a2465 = {
                            let a2453 = {
                                let a2447 = {
                                    let a2445 = v3;
                                    let a2446 = 128u8;
                                    fixed_u8::bit_and(a2445, a2446)
                                };
                                let a2448 = 128u8;
                                if equal(a2447, a2448) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a2454 = {
                                let a2451 = {
                                    let a2449 = v3;
                                    let a2450 = 64u8;
                                    fixed_u8::bit_and(a2449, a2450)
                                };
                                let a2452 = 64u8;
                                if equal(a2451, a2452) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2453, a2454)?
                        };
                        let a2466 = {
                            let a2463 = {
                                let a2457 = {
                                    let a2455 = v3;
                                    let a2456 = 32u8;
                                    fixed_u8::bit_and(a2455, a2456)
                                };
                                let a2458 = 32u8;
                                if equal(a2457, a2458) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a2464 = {
                                let a2461 = {
                                    let a2459 = v3;
                                    let a2460 = 16u8;
                                    fixed_u8::bit_and(a2459, a2460)
                                };
                                let a2462 = 16u8;
                                if equal(a2461, a2462) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2463, a2464)?
                        };
                        nat_add(a2465, a2466)?
                    };
                    let a2490 = {
                        let a2487 = {
                            let a2475 = {
                                let a2469 = {
                                    let a2467 = v3;
                                    let a2468 = 8u8;
                                    fixed_u8::bit_and(a2467, a2468)
                                };
                                let a2470 = 8u8;
                                if equal(a2469, a2470) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a2476 = {
                                let a2473 = {
                                    let a2471 = v3;
                                    let a2472 = 4u8;
                                    fixed_u8::bit_and(a2471, a2472)
                                };
                                let a2474 = 4u8;
                                if equal(a2473, a2474) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2475, a2476)?
                        };
                        let a2488 = {
                            let a2485 = {
                                let a2479 = {
                                    let a2477 = v3;
                                    let a2478 = 2u8;
                                    fixed_u8::bit_and(a2477, a2478)
                                };
                                let a2480 = 2u8;
                                if equal(a2479, a2480) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2486 = {
                                let a2483 = {
                                    let a2481 = v3;
                                    let a2482 = 1u8;
                                    fixed_u8::bit_and(a2481, a2482)
                                };
                                let a2484 = 1u8;
                                if equal(a2483, a2484) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2485, a2486)?
                        };
                        nat_add(a2487, a2488)?
                    };
                    nat_add(a2489, a2490)?
                }))
            }
        }
    }
}

pub fn f415(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2491 = f430(v0.clone());
    match m2491 {
        None => {
            if f36(f143(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2492 = f140(v0, f8(Str::lit("items")));
                match m2492 {
                    None => {
                        Ok(f385(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f431(f8(Str::lit("List")), f432(v3, {
                            let a2493 = v1;
                            let a2494 = f8(Str::lit("Item"));
                            append_bytes(a2493, a2494)
                        })?))
                    }
                }
            } else {
                f432(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f431(f8(Str::lit("Option")), f432(v2, v1)?))
        }
    }
}

pub fn f416(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2495, _) = v2;
        h2495
    };
    let a2496 = v3;
    let a2497 = 0u64;
    if nat_eq(a2496, a2497) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2498 = v3;
            let a2499 = v1;
            let a2500 = 0u64;
            nat_quot(a2498, a2499, a2500)
        }, {
            let a2505 = f388(v0, {
                let a2501 = v3;
                let a2502 = v1;
                let a2503 = 0u64;
                nat_rem(a2501, a2502, a2503)
            });
            let a2506 = {
                let (_, h2504) = v2;
                h2504
            };
            append_bytes(a2505, a2506)
        }))
    }
}

pub fn f417(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2551 = {
            let a2527 = {
                let a2515 = {
                    let a2509 = {
                        let a2507 = v1;
                        let a2508 = 128u8;
                        fixed_u8::bit_and(a2507, a2508)
                    };
                    let a2510 = 128u8;
                    if equal(a2509, a2510) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2516 = {
                    let a2513 = {
                        let a2511 = v1;
                        let a2512 = 64u8;
                        fixed_u8::bit_and(a2511, a2512)
                    };
                    let a2514 = 64u8;
                    if equal(a2513, a2514) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2515, a2516)?
            };
            let a2528 = {
                let a2525 = {
                    let a2519 = {
                        let a2517 = v1;
                        let a2518 = 32u8;
                        fixed_u8::bit_and(a2517, a2518)
                    };
                    let a2520 = 32u8;
                    if equal(a2519, a2520) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2526 = {
                    let a2523 = {
                        let a2521 = v1;
                        let a2522 = 16u8;
                        fixed_u8::bit_and(a2521, a2522)
                    };
                    let a2524 = 16u8;
                    if equal(a2523, a2524) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2525, a2526)?
            };
            nat_add(a2527, a2528)?
        };
        let a2552 = {
            let a2549 = {
                let a2537 = {
                    let a2531 = {
                        let a2529 = v1;
                        let a2530 = 8u8;
                        fixed_u8::bit_and(a2529, a2530)
                    };
                    let a2532 = 8u8;
                    if equal(a2531, a2532) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2538 = {
                    let a2535 = {
                        let a2533 = v1;
                        let a2534 = 4u8;
                        fixed_u8::bit_and(a2533, a2534)
                    };
                    let a2536 = 4u8;
                    if equal(a2535, a2536) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2537, a2538)?
            };
            let a2550 = {
                let a2547 = {
                    let a2541 = {
                        let a2539 = v1;
                        let a2540 = 2u8;
                        fixed_u8::bit_and(a2539, a2540)
                    };
                    let a2542 = 2u8;
                    if equal(a2541, a2542) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2548 = {
                    let a2545 = {
                        let a2543 = v1;
                        let a2544 = 1u8;
                        fixed_u8::bit_and(a2543, a2544)
                    };
                    let a2546 = 1u8;
                    if equal(a2545, a2546) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2547, a2548)?
            };
            nat_add(a2549, a2550)?
        };
        nat_add(a2551, a2552)?
    };
    let a2559 = f388(f433(), {
        let a2553 = v2;
        let a2554 = 16u64;
        let a2555 = 0u64;
        nat_quot(a2553, a2554, a2555)
    });
    let a2560 = f388(f433(), {
        let a2556 = v2;
        let a2557 = 16u64;
        let a2558 = 0u64;
        nat_rem(a2556, a2557, a2558)
    });
    Ok(append_bytes(a2559, a2560))
}

pub fn f418(v0: Fn21, p2564: List<List<(Bytes, Bytes)>>, p2565: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2563 = (p2564, p2565);
    {
        let mut m2563 = m2563;
        loop {
            let step = {
                let (v1, v2) = m2563;
                let m2561 = v2;
                match m2561.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2562: Fn21 = v0.clone();
                            c2562.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2563 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f419(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f420(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2567 = {
        let a2566 = v0.clone();
        length_list(a2566)
    };
    let a2568 = 1u64;
    if nat_le(a2567, a2568) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f434(v0)?))
    }
}

pub fn f421(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2569, _) = v0;
        h2569
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2570) = v0;
        h2570
    };
    let m2571 = v1;
    match m2571.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2572 = v4;
            match m2572.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f435(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f422(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2573 = f312(v0, v1);
    let a2574 = f406(v0, v2);
    fixed_u32::bit_or(a2573, a2574)
}

pub fn f423(v0: u32) -> u32 {
    let a2577 = {
        let a2575 = f422(v0, 17u32, 15u32);
        let a2576 = f422(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2575, a2576)
    };
    let a2578 = f312(v0, 10u32);
    fixed_u32::bit_xor(a2577, a2578)
}

pub fn f424(v0: u32) -> u32 {
    let a2581 = {
        let a2579 = f422(v0, 7u32, 25u32);
        let a2580 = f422(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2579, a2580)
    };
    let a2582 = f312(v0, 3u32);
    fixed_u32::bit_xor(a2581, a2582)
}

pub fn f425(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2585 = f32(v0.clone(), {
        let a2583 = v1;
        let a2584 = 2u64;
        nat_add(a2583, a2584)?
    }, 2u64);
    let a2586 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2585, a2586) {
        let m2589 = f436(f32(v0, {
            let a2587 = v1;
            let a2588 = 4u64;
            nat_add(a2587, a2588)?
        }, 2u64))?;
        match m2589 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f318(v2.clone(), 0u64)?;
                let a2590 = v3;
                let a2591 = 32u64;
                if nat_lt(a2590, a2591) {
                    if f437(v3) {
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

pub fn f426(v0: u64) -> Bytes {
    let a2592 = v0;
    let a2593 = 34u64;
    if nat_eq(a2592, a2593) {
        Bytes::lit(&[0x22])
    } else {
        let a2594 = v0;
        let a2595 = 92u64;
        if nat_eq(a2594, a2595) {
            Bytes::lit(&[0x5c])
        } else {
            let a2596 = v0;
            let a2597 = 98u64;
            if nat_eq(a2596, a2597) {
                Bytes::lit(&[0x08])
            } else {
                let a2598 = v0;
                let a2599 = 102u64;
                if nat_eq(a2598, a2599) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2600 = v0;
                    let a2601 = 110u64;
                    if nat_eq(a2600, a2601) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2602 = v0;
                        let a2603 = 114u64;
                        if nat_eq(a2602, a2603) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2604 = v0;
                            let a2605 = 116u64;
                            if nat_eq(a2604, a2605) {
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

pub fn f427(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2608 = {
        let a2606 = v0;
        let a2607 = v1;
        index_bytes(a2606, a2607)
    };
    match m2608 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f438(v2)? {
                Ok(Some({
                    let a2609 = v1;
                    let a2610 = 1u64;
                    nat_add(a2609, a2610)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f428(v0: u8) -> R<u64> {
    let a2657 = {
        let v1: u8 = v0;
        let a2655 = {
            let a2631 = {
                let a2619 = {
                    let a2613 = {
                        let a2611 = v1;
                        let a2612 = 128u8;
                        fixed_u8::bit_and(a2611, a2612)
                    };
                    let a2614 = 128u8;
                    if equal(a2613, a2614) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2620 = {
                    let a2617 = {
                        let a2615 = v1;
                        let a2616 = 64u8;
                        fixed_u8::bit_and(a2615, a2616)
                    };
                    let a2618 = 64u8;
                    if equal(a2617, a2618) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2619, a2620)?
            };
            let a2632 = {
                let a2629 = {
                    let a2623 = {
                        let a2621 = v1;
                        let a2622 = 32u8;
                        fixed_u8::bit_and(a2621, a2622)
                    };
                    let a2624 = 32u8;
                    if equal(a2623, a2624) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2630 = {
                    let a2627 = {
                        let a2625 = v1;
                        let a2626 = 16u8;
                        fixed_u8::bit_and(a2625, a2626)
                    };
                    let a2628 = 16u8;
                    if equal(a2627, a2628) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2629, a2630)?
            };
            nat_add(a2631, a2632)?
        };
        let a2656 = {
            let a2653 = {
                let a2641 = {
                    let a2635 = {
                        let a2633 = v1;
                        let a2634 = 8u8;
                        fixed_u8::bit_and(a2633, a2634)
                    };
                    let a2636 = 8u8;
                    if equal(a2635, a2636) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2642 = {
                    let a2639 = {
                        let a2637 = v1;
                        let a2638 = 4u8;
                        fixed_u8::bit_and(a2637, a2638)
                    };
                    let a2640 = 4u8;
                    if equal(a2639, a2640) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2641, a2642)?
            };
            let a2654 = {
                let a2651 = {
                    let a2645 = {
                        let a2643 = v1;
                        let a2644 = 2u8;
                        fixed_u8::bit_and(a2643, a2644)
                    };
                    let a2646 = 2u8;
                    if equal(a2645, a2646) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2652 = {
                    let a2649 = {
                        let a2647 = v1;
                        let a2648 = 1u8;
                        fixed_u8::bit_and(a2647, a2648)
                    };
                    let a2650 = 1u8;
                    if equal(a2649, a2650) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2651, a2652)?
            };
            nat_add(a2653, a2654)?
        };
        nat_add(a2655, a2656)?
    };
    let a2658 = 48u64;
    Ok(nat_sub(a2657, a2658))
}

pub fn f429(v0: Bytes) -> R<Option<Bytes>> {
    if f439({
        let a2659 = v0.clone();
        length_bytes(a2659)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f413(v0)?;
        if f414(v1.clone())? {
            Ok(Some(v1))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        f440({
            let a2660 = v0.clone();
            length_bytes(a2660)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f430(v0: Adt14) -> Option<Adt14> {
    let m2661 = f144(f140(v0, f8(Str::lit("oneOf"))));
    match m2661.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2662 = v2;
            match m2662.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2663 = v4;
                    match m2663.uncons() {
                        None => {
                            if f441(v3.clone()) {
                                Some(v1)
                            } else {
                                if f441(v1) {
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

pub fn f431(v0: Bytes, v1: Result<(Bytes, List<Adt23>), Bytes>) -> Result<(Bytes, List<Adt23>), Bytes> {
    let m2664 = v1;
    match m2664 {
        Ok(v3) => {
            f386(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2665, _) = v3;
                h2665
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2666) = v3;
                h2666
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v2)
        }
    }
}

pub fn f432(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2667 = f430(v0.clone());
    match m2667 {
        None => {
            if f36(f143(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2668 = f140(v0, f8(Str::lit("items")));
                match m2668 {
                    None => {
                        Ok(f385(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f431(f8(Str::lit("List")), f442(v3, {
                            let a2669 = v1;
                            let a2670 = f8(Str::lit("Item"));
                            append_bytes(a2669, a2670)
                        })?))
                    }
                }
            } else {
                f442(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f431(f8(Str::lit("Option")), f442(v2, v1)?))
        }
    }
}

pub fn f433() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f434(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f391({
        let (_, h2671) = {
            let (h2672, _) = f443(Fn22::F444, {
                let a2674 = {
                    let a2673 = v0.clone();
                    length_list(a2673)
                };
                let a2675 = 1u64;
                nat_add(a2674, a2675)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2672
        };
        h2671
    }))
}

pub fn f435(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2680 = {
            let a2678 = {
                let a2676 = v0.clone();
                length_list(a2676)
            };
            let a2679 = {
                let a2677 = v1.clone();
                length_list(a2677)
            };
            nat_add(a2678, a2679)?
        };
        let a2681 = 1u64;
        nat_add(a2680, a2681)?
    };
    Ok(f302({
        let (_, h2682) = {
            let (_, h2683) = {
                let (h2684, _) = f445(Fn23::F446, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2684
            };
            h2683
        };
        h2682
    }))
}

pub fn f436(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2685 = v0.clone();
        length_bytes(a2685)
    };
    let a2689 = {
        let a2686 = v1;
        let a2687 = 2u64;
        let a2688 = 0u64;
        nat_rem(a2686, a2687, a2688)
    };
    let a2690 = 0u64;
    if nat_eq(a2689, a2690) {
        let (_, h2691) = {
            let (h2692, _) = f447(Fn24::F448(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2692
        };
        Ok(h2691)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f437(v0: u64) -> bool {
    let a2693 = v0;
    let a2694 = 8u64;
    if nat_eq(a2693, a2694) {
        true
    } else {
        let a2695 = v0;
        let a2696 = 9u64;
        if nat_eq(a2695, a2696) {
            true
        } else {
            let a2697 = v0;
            let a2698 = 10u64;
            if nat_eq(a2697, a2698) {
                true
            } else {
                let a2699 = v0;
                let a2700 = 12u64;
                if nat_eq(a2699, a2700) {
                    true
                } else {
                    let a2701 = v0;
                    let a2702 = 13u64;
                    nat_eq(a2701, a2702)
                }
            }
        }
    }
}

pub fn f438(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2747 = {
            let a2723 = {
                let a2711 = {
                    let a2705 = {
                        let a2703 = v1;
                        let a2704 = 128u8;
                        fixed_u8::bit_and(a2703, a2704)
                    };
                    let a2706 = 128u8;
                    if equal(a2705, a2706) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2712 = {
                    let a2709 = {
                        let a2707 = v1;
                        let a2708 = 64u8;
                        fixed_u8::bit_and(a2707, a2708)
                    };
                    let a2710 = 64u8;
                    if equal(a2709, a2710) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2711, a2712)?
            };
            let a2724 = {
                let a2721 = {
                    let a2715 = {
                        let a2713 = v1;
                        let a2714 = 32u8;
                        fixed_u8::bit_and(a2713, a2714)
                    };
                    let a2716 = 32u8;
                    if equal(a2715, a2716) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2722 = {
                    let a2719 = {
                        let a2717 = v1;
                        let a2718 = 16u8;
                        fixed_u8::bit_and(a2717, a2718)
                    };
                    let a2720 = 16u8;
                    if equal(a2719, a2720) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2721, a2722)?
            };
            nat_add(a2723, a2724)?
        };
        let a2748 = {
            let a2745 = {
                let a2733 = {
                    let a2727 = {
                        let a2725 = v1;
                        let a2726 = 8u8;
                        fixed_u8::bit_and(a2725, a2726)
                    };
                    let a2728 = 8u8;
                    if equal(a2727, a2728) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2734 = {
                    let a2731 = {
                        let a2729 = v1;
                        let a2730 = 4u8;
                        fixed_u8::bit_and(a2729, a2730)
                    };
                    let a2732 = 4u8;
                    if equal(a2731, a2732) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2733, a2734)?
            };
            let a2746 = {
                let a2743 = {
                    let a2737 = {
                        let a2735 = v1;
                        let a2736 = 2u8;
                        fixed_u8::bit_and(a2735, a2736)
                    };
                    let a2738 = 2u8;
                    if equal(a2737, a2738) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2744 = {
                    let a2741 = {
                        let a2739 = v1;
                        let a2740 = 1u8;
                        fixed_u8::bit_and(a2739, a2740)
                    };
                    let a2742 = 1u8;
                    if equal(a2741, a2742) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2743, a2744)?
            };
            nat_add(a2745, a2746)?
        };
        nat_add(a2747, a2748)?
    };
    let a2749 = 48u64;
    let a2750 = v2;
    if nat_le(a2749, a2750) {
        let a2751 = v2;
        let a2752 = 57u64;
        Ok(nat_le(a2751, a2752))
    } else {
        Ok(false)
    }
}

pub fn f439(p2808: u64, v1: Bytes, p2809: u64) -> R<bool> {
    let m2807 = (p2808, p2809);
    {
        let mut m2807 = m2807;
        loop {
            let step = {
                let (v0, v2) = m2807;
                let m2753 = v0;
                if m2753 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2753 - 1;
                    let m2756 = {
                        let a2754 = v1.clone();
                        let a2755 = v2;
                        index_bytes(a2754, a2755)
                    };
                    match m2756 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2803 = {
                                let v5: u8 = v4;
                                let a2801 = {
                                    let a2777 = {
                                        let a2765 = {
                                            let a2759 = {
                                                let a2757 = v5;
                                                let a2758 = 128u8;
                                                fixed_u8::bit_and(a2757, a2758)
                                            };
                                            let a2760 = 128u8;
                                            if equal(a2759, a2760) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2766 = {
                                            let a2763 = {
                                                let a2761 = v5;
                                                let a2762 = 64u8;
                                                fixed_u8::bit_and(a2761, a2762)
                                            };
                                            let a2764 = 64u8;
                                            if equal(a2763, a2764) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2765, a2766)?
                                    };
                                    let a2778 = {
                                        let a2775 = {
                                            let a2769 = {
                                                let a2767 = v5;
                                                let a2768 = 32u8;
                                                fixed_u8::bit_and(a2767, a2768)
                                            };
                                            let a2770 = 32u8;
                                            if equal(a2769, a2770) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2776 = {
                                            let a2773 = {
                                                let a2771 = v5;
                                                let a2772 = 16u8;
                                                fixed_u8::bit_and(a2771, a2772)
                                            };
                                            let a2774 = 16u8;
                                            if equal(a2773, a2774) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2775, a2776)?
                                    };
                                    nat_add(a2777, a2778)?
                                };
                                let a2802 = {
                                    let a2799 = {
                                        let a2787 = {
                                            let a2781 = {
                                                let a2779 = v5;
                                                let a2780 = 8u8;
                                                fixed_u8::bit_and(a2779, a2780)
                                            };
                                            let a2782 = 8u8;
                                            if equal(a2781, a2782) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2788 = {
                                            let a2785 = {
                                                let a2783 = v5;
                                                let a2784 = 4u8;
                                                fixed_u8::bit_and(a2783, a2784)
                                            };
                                            let a2786 = 4u8;
                                            if equal(a2785, a2786) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2787, a2788)?
                                    };
                                    let a2800 = {
                                        let a2797 = {
                                            let a2791 = {
                                                let a2789 = v5;
                                                let a2790 = 2u8;
                                                fixed_u8::bit_and(a2789, a2790)
                                            };
                                            let a2792 = 2u8;
                                            if equal(a2791, a2792) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2798 = {
                                            let a2795 = {
                                                let a2793 = v5;
                                                let a2794 = 1u8;
                                                fixed_u8::bit_and(a2793, a2794)
                                            };
                                            let a2796 = 1u8;
                                            if equal(a2795, a2796) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2797, a2798)?
                                    };
                                    nat_add(a2799, a2800)?
                                };
                                nat_add(a2801, a2802)?
                            };
                            let a2804 = 128u64;
                            if nat_lt(a2803, a2804) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2805 = v2;
                                    let a2806 = 1u64;
                                    nat_add(a2805, a2806)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
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

pub fn f440(p2822: u64, v1: Bytes, p2823: u64, p2824: Bytes) -> R<Option<Bytes>> {
    let m2821 = (p2822, (p2823, p2824));
    {
        let mut m2821 = m2821;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2821;
                let m2810 = v0;
                if m2810 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2810 - 1;
                    let a2812 = {
                        let a2811 = v1.clone();
                        length_bytes(a2811)
                    };
                    let a2813 = v2;
                    if nat_le(a2812, a2813) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2814 = f449(v1.clone(), v2)?;
                        match m2814 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2816 = v2;
                                    let a2817 = {
                                        let (_, h2815) = v5;
                                        h2815
                                    };
                                    nat_add(a2816, a2817)?
                                }, {
                                    let a2819 = v3;
                                    let a2820 = f450({
                                        let (h2818, _) = v5;
                                        h2818
                                    });
                                    append_bytes(a2819, a2820)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2821 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: Adt14) -> bool {
    f36(f143(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f442(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2825 = f451(f359(v0.clone()));
    match m2825 {
        None => {
            let m2826 = f148(f140(v0.clone(), f8(Str::lit("$ref"))));
            match m2826 {
                None => {
                    let m2828 = f140(v0.clone(), f8(Str::lit("enum")));
                    match m2828 {
                        None => {
                            let m2829 = f140(v0.clone(), f8(Str::lit("const")));
                            match m2829 {
                                None => {
                                    let v8: Bytes = f143(v0.clone(), f8(Str::lit("type")));
                                    if f36(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f386(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                    } else {
                                        if f36(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f386(f453(v0), List::<Adt23>::nil()))
                                        } else {
                                            if f36(v8.clone(), f8(Str::lit("number"))) {
                                                Ok(f386(f454(), List::<Adt23>::nil()))
                                            } else {
                                                if f36(v8.clone(), f8(Str::lit("boolean"))) {
                                                    Ok(f386(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                                } else {
                                                    if f36(v8.clone(), f8(Str::lit("object"))) {
                                                        Ok(f386(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                    } else {
                                                        Ok(f385(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2830 = f148(Some(v6));
                                    match m2830 {
                                        None => {
                                            Ok(f385(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f386(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f386(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2827 = f452(v3)?;
                    match m2827 {
                        None => {
                            Ok(f385(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f386(v4, List::<Adt23>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f385(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f443(v0: Fn22, p2835: u64, p2836: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2834 = (p2835, p2836);
    {
        let mut m2834 = m2834;
        loop {
            let step = {
                let (v1, v2) = m2834;
                let m2831 = v1;
                if m2831 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2831 - 1;
                    let m2833 = {
                        let c2832: Fn22 = v0.clone();
                        c2832.apply(v2.clone())?
                    };
                    match m2833 {
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
                Ok(next) => { m2834 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f444(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f455(v0)
}

pub fn f445(v0: Fn23, p2841: u64, p2842: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2840 = (p2841, p2842);
    {
        let mut m2840 = m2840;
        loop {
            let step = {
                let (v1, v2) = m2840;
                let m2837 = v1;
                if m2837 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2837 - 1;
                    let m2839 = {
                        let c2838: Fn23 = v0.clone();
                        c2838.apply(v2.clone())
                    };
                    match m2839 {
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
                Ok(next) => { m2840 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f456(v0)
}

pub fn f447(v0: Fn24, p2847: u64, p2848: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2846 = (p2847, p2848);
    {
        let mut m2846 = m2846;
        loop {
            let step = {
                let (v1, v2) = m2846;
                let m2843 = v1;
                if m2843 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2843 - 1;
                    let m2845 = {
                        let c2844: Fn24 = v0.clone();
                        c2844.apply(v2.clone())?
                    };
                    match m2845 {
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
                Ok(next) => { m2846 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f448(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f457(v0, v1)
}

pub fn f449(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2851 = {
        let a2849 = v0.clone();
        let a2850 = v1;
        index_bytes(a2849, a2850)
    };
    match m2851 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2896 = {
                    let a2872 = {
                        let a2860 = {
                            let a2854 = {
                                let a2852 = v3;
                                let a2853 = 128u8;
                                fixed_u8::bit_and(a2852, a2853)
                            };
                            let a2855 = 128u8;
                            if equal(a2854, a2855) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2861 = {
                            let a2858 = {
                                let a2856 = v3;
                                let a2857 = 64u8;
                                fixed_u8::bit_and(a2856, a2857)
                            };
                            let a2859 = 64u8;
                            if equal(a2858, a2859) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2860, a2861)?
                    };
                    let a2873 = {
                        let a2870 = {
                            let a2864 = {
                                let a2862 = v3;
                                let a2863 = 32u8;
                                fixed_u8::bit_and(a2862, a2863)
                            };
                            let a2865 = 32u8;
                            if equal(a2864, a2865) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2871 = {
                            let a2868 = {
                                let a2866 = v3;
                                let a2867 = 16u8;
                                fixed_u8::bit_and(a2866, a2867)
                            };
                            let a2869 = 16u8;
                            if equal(a2868, a2869) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2870, a2871)?
                    };
                    nat_add(a2872, a2873)?
                };
                let a2897 = {
                    let a2894 = {
                        let a2882 = {
                            let a2876 = {
                                let a2874 = v3;
                                let a2875 = 8u8;
                                fixed_u8::bit_and(a2874, a2875)
                            };
                            let a2877 = 8u8;
                            if equal(a2876, a2877) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2883 = {
                            let a2880 = {
                                let a2878 = v3;
                                let a2879 = 4u8;
                                fixed_u8::bit_and(a2878, a2879)
                            };
                            let a2881 = 4u8;
                            if equal(a2880, a2881) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2882, a2883)?
                    };
                    let a2895 = {
                        let a2892 = {
                            let a2886 = {
                                let a2884 = v3;
                                let a2885 = 2u8;
                                fixed_u8::bit_and(a2884, a2885)
                            };
                            let a2887 = 2u8;
                            if equal(a2886, a2887) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2893 = {
                            let a2890 = {
                                let a2888 = v3;
                                let a2889 = 1u8;
                                fixed_u8::bit_and(a2888, a2889)
                            };
                            let a2891 = 1u8;
                            if equal(a2890, a2891) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2892, a2893)?
                    };
                    nat_add(a2894, a2895)?
                };
                nat_add(a2896, a2897)?
            };
            let a2898 = v4;
            let a2899 = 128u64;
            if nat_lt(a2898, a2899) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2900 = 194u64;
                let a2901 = v4;
                if if nat_le(a2900, a2901) {
                    let a2902 = v4;
                    let a2903 = 223u64;
                    nat_le(a2902, a2903)
                } else {
                    false
                } {
                    let m2906 = f458(v0, {
                        let a2904 = v1;
                        let a2905 = 1u64;
                        nat_add(a2904, a2905)?
                    })?;
                    match m2906 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f459({
                                let a2911 = {
                                    let a2909 = {
                                        let a2907 = v4;
                                        let a2908 = 192u64;
                                        nat_sub(a2907, a2908)
                                    };
                                    let a2910 = 64u64;
                                    nat_mul(a2909, a2910)?
                                };
                                let a2912 = v5;
                                nat_add(a2911, a2912)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2913 = 224u64;
                    let a2914 = v4;
                    if if nat_le(a2913, a2914) {
                        let a2915 = v4;
                        let a2916 = 239u64;
                        nat_le(a2915, a2916)
                    } else {
                        false
                    } {
                        let m2919 = f458(v0.clone(), {
                            let a2917 = v1;
                            let a2918 = 1u64;
                            nat_add(a2917, a2918)?
                        })?;
                        match m2919 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2922 = f458(v0, {
                                    let a2920 = v1;
                                    let a2921 = 2u64;
                                    nat_add(a2920, a2921)?
                                })?;
                                match m2922 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f459({
                                            let a2931 = {
                                                let a2925 = {
                                                    let a2923 = v4;
                                                    let a2924 = 224u64;
                                                    nat_sub(a2923, a2924)
                                                };
                                                let a2926 = 4096u64;
                                                nat_mul(a2925, a2926)?
                                            };
                                            let a2932 = {
                                                let a2929 = {
                                                    let a2927 = v6;
                                                    let a2928 = 64u64;
                                                    nat_mul(a2927, a2928)?
                                                };
                                                let a2930 = v7;
                                                nat_add(a2929, a2930)?
                                            };
                                            nat_add(a2931, a2932)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2933 = 240u64;
                        let a2934 = v4;
                        if if nat_le(a2933, a2934) {
                            let a2935 = v4;
                            let a2936 = 244u64;
                            nat_le(a2935, a2936)
                        } else {
                            false
                        } {
                            let m2939 = f458(v0.clone(), {
                                let a2937 = v1;
                                let a2938 = 1u64;
                                nat_add(a2937, a2938)?
                            })?;
                            match m2939 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2942 = f458(v0.clone(), {
                                        let a2940 = v1;
                                        let a2941 = 2u64;
                                        nat_add(a2940, a2941)?
                                    })?;
                                    match m2942 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m2945 = f458(v0, {
                                                let a2943 = v1;
                                                let a2944 = 3u64;
                                                nat_add(a2943, a2944)?
                                            })?;
                                            match m2945 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f459({
                                                        let a2958 = {
                                                            let a2948 = {
                                                                let a2946 = v4;
                                                                let a2947 = 240u64;
                                                                nat_sub(a2946, a2947)
                                                            };
                                                            let a2949 = 262144u64;
                                                            nat_mul(a2948, a2949)?
                                                        };
                                                        let a2959 = {
                                                            let a2956 = {
                                                                let a2950 = v8;
                                                                let a2951 = 4096u64;
                                                                nat_mul(a2950, a2951)?
                                                            };
                                                            let a2957 = {
                                                                let a2954 = {
                                                                    let a2952 = v9;
                                                                    let a2953 = 64u64;
                                                                    nat_mul(a2952, a2953)?
                                                                };
                                                                let a2955 = v10;
                                                                nat_add(a2954, a2955)?
                                                            };
                                                            nat_add(a2956, a2957)?
                                                        };
                                                        nat_add(a2958, a2959)?
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

pub fn f450(v0: u64) -> Bytes {
    let a2960 = f8(Str::lit("u"));
    let a2961 = f460(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a2960, a2961)
}

pub fn f451(p2967: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2966 = p2967;
    {
        let mut m2966 = m2966;
        loop {
            let step = {
                let v0 = m2966;
                let m2962 = v0;
                match m2962.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f84({
                            let (h2963, _) = v1.clone();
                            h2963
                        }, f461()) {
                            true
                        } else {
                            f84({
                                let (h2964, _) = v1.clone();
                                h2964
                            }, f462())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2965, _) = v1;
                                h2965
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2966 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(v0: Bytes) -> R<Option<Bytes>> {
    if f190(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f214(f32(v0.clone(), 8u64, {
            let a2969 = {
                let a2968 = v0;
                length_bytes(a2968)
            };
            let a2970 = 8u64;
            nat_sub(a2969, a2970)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f453(v0: Adt14) -> Bytes {
    let m2971 = f294(f140(v0, f8(Str::lit("minimum"))));
    match m2971 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f454() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f455(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2972, _) = v0;
        h2972
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2973) = v0;
        h2973
    };
    let m2974 = v1;
    match m2974.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2975 = v4;
            match m2975.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f463(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f456(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2976, _) = v0.clone();
        h2976
    };
    let v2: List<(Bytes, ())> = {
        let (h2977, _) = {
            let (_, h2978) = v0.clone();
            h2978
        };
        h2977
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2979) = {
            let (_, h2980) = v0;
            h2980
        };
        h2979
    };
    let m2981 = v1.clone();
    match m2981.uncons() {
        None => {
            let m2982 = v2;
            match m2982.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2983 = v2.clone();
            match m2983.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f464({
                        let (h2984, _) = v8.clone();
                        h2984
                    }, {
                        let (h2985, _) = v6.clone();
                        h2985
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
        let (h2986, _) = v1;
        h2986
    };
    let m2988 = {
        let (_, h2987) = v1;
        h2987
    };
    match m2988 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2991 = {
                let a2989 = v0.clone();
                let a2990 = v2;
                index_bytes(a2989, a2990)
            };
            match m2991 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2996 = {
                        let a2994 = v0;
                        let a2995 = {
                            let a2992 = v2;
                            let a2993 = 1u64;
                            nat_add(a2992, a2993)?
                        };
                        index_bytes(a2994, a2995)
                    };
                    match m2996 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f465(v4)? {
                                f465(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2997 = v2;
                                    let a2998 = 2u64;
                                    nat_add(a2997, a2998)?
                                }, Some({
                                    let a3003 = v3;
                                    let a3004 = f388(f466(), {
                                        let a3001 = {
                                            let a2999 = f467(v4)?;
                                            let a3000 = 16u64;
                                            nat_mul(a2999, a3000)?
                                        };
                                        let a3002 = f467(v5)?;
                                        nat_add(a3001, a3002)?
                                    });
                                    append_bytes(a3003, a3004)
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

pub fn f458(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3007 = {
        let a3005 = v0;
        let a3006 = v1;
        index_bytes(a3005, a3006)
    };
    match m3007 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3052 = {
                    let a3028 = {
                        let a3016 = {
                            let a3010 = {
                                let a3008 = v3;
                                let a3009 = 128u8;
                                fixed_u8::bit_and(a3008, a3009)
                            };
                            let a3011 = 128u8;
                            if equal(a3010, a3011) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3017 = {
                            let a3014 = {
                                let a3012 = v3;
                                let a3013 = 64u8;
                                fixed_u8::bit_and(a3012, a3013)
                            };
                            let a3015 = 64u8;
                            if equal(a3014, a3015) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3016, a3017)?
                    };
                    let a3029 = {
                        let a3026 = {
                            let a3020 = {
                                let a3018 = v3;
                                let a3019 = 32u8;
                                fixed_u8::bit_and(a3018, a3019)
                            };
                            let a3021 = 32u8;
                            if equal(a3020, a3021) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3027 = {
                            let a3024 = {
                                let a3022 = v3;
                                let a3023 = 16u8;
                                fixed_u8::bit_and(a3022, a3023)
                            };
                            let a3025 = 16u8;
                            if equal(a3024, a3025) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3026, a3027)?
                    };
                    nat_add(a3028, a3029)?
                };
                let a3053 = {
                    let a3050 = {
                        let a3038 = {
                            let a3032 = {
                                let a3030 = v3;
                                let a3031 = 8u8;
                                fixed_u8::bit_and(a3030, a3031)
                            };
                            let a3033 = 8u8;
                            if equal(a3032, a3033) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3039 = {
                            let a3036 = {
                                let a3034 = v3;
                                let a3035 = 4u8;
                                fixed_u8::bit_and(a3034, a3035)
                            };
                            let a3037 = 4u8;
                            if equal(a3036, a3037) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3038, a3039)?
                    };
                    let a3051 = {
                        let a3048 = {
                            let a3042 = {
                                let a3040 = v3;
                                let a3041 = 2u8;
                                fixed_u8::bit_and(a3040, a3041)
                            };
                            let a3043 = 2u8;
                            if equal(a3042, a3043) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3049 = {
                            let a3046 = {
                                let a3044 = v3;
                                let a3045 = 1u8;
                                fixed_u8::bit_and(a3044, a3045)
                            };
                            let a3047 = 1u8;
                            if equal(a3046, a3047) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3048, a3049)?
                    };
                    nat_add(a3050, a3051)?
                };
                nat_add(a3052, a3053)?
            };
            let a3054 = 128u64;
            let a3055 = v4;
            if if nat_le(a3054, a3055) {
                let a3056 = v4;
                let a3057 = 191u64;
                nat_le(a3056, a3057)
            } else {
                false
            } {
                Ok(Some({
                    let a3058 = v4;
                    let a3059 = 128u64;
                    nat_sub(a3058, a3059)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f459(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3060 = v1;
    let a3061 = v0;
    if if nat_le(a3060, a3061) {
        let a3062 = v0;
        let a3063 = v2;
        if nat_le(a3062, a3063) {
            let a3064 = v0;
            let a3065 = 55296u64;
            if nat_lt(a3064, a3065) {
                true
            } else {
                let a3066 = 57343u64;
                let a3067 = v0;
                nat_lt(a3066, a3067)
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

pub fn f460(p3084: u64, p3085: u64, p3086: u64, p3087: Bytes) -> Bytes {
    let m3083 = (p3084, (p3085, (p3086, p3087)));
    {
        let mut m3083 = m3083;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3083;
                let m3068 = v0;
                if m3068 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3068 - 1;
                    let a3069 = v1;
                    let a3070 = 0u64;
                    if if nat_eq(a3069, a3070) {
                        let a3071 = v2;
                        let a3072 = 0u64;
                        nat_eq(a3071, a3072)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3073 = v1;
                            let a3074 = 16u64;
                            let a3075 = 0u64;
                            nat_quot(a3073, a3074, a3075)
                        }, ({
                            let a3076 = v2;
                            let a3077 = 1u64;
                            nat_sub(a3076, a3077)
                        }, {
                            let a3081 = f468({
                                let a3078 = v1;
                                let a3079 = 16u64;
                                let a3080 = 0u64;
                                nat_rem(a3078, a3079, a3080)
                            });
                            let a3082 = v3;
                            append_bytes(a3081, a3082)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3083 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f462() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f463(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3092 = {
            let a3090 = {
                let a3088 = v0.clone();
                length_list(a3088)
            };
            let a3091 = {
                let a3089 = v1.clone();
                length_list(a3089)
            };
            nat_add(a3090, a3091)?
        };
        let a3093 = 1u64;
        nat_add(a3092, a3093)?
    };
    Ok(f469({
        let (_, h3094) = {
            let (_, h3095) = {
                let (h3096, _) = f470(Fn25::F471, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3096
            };
            h3095
        };
        h3094
    }))
}

pub fn f464(v0: Bytes, v1: Bytes) -> bool {
    let a3101 = {
        let a3097 = v0;
        let a3098 = v1;
        compare_bytes(a3097, a3098)
    };
    let a3102 = {
        let a3099 = Bytes::lit(&[]);
        let a3100 = Bytes::lit(&[0x01]);
        compare_bytes(a3099, a3100)
    };
    equal(a3101, a3102)
}

pub fn f465(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3147 = {
            let a3123 = {
                let a3111 = {
                    let a3105 = {
                        let a3103 = v1;
                        let a3104 = 128u8;
                        fixed_u8::bit_and(a3103, a3104)
                    };
                    let a3106 = 128u8;
                    if equal(a3105, a3106) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3112 = {
                    let a3109 = {
                        let a3107 = v1;
                        let a3108 = 64u8;
                        fixed_u8::bit_and(a3107, a3108)
                    };
                    let a3110 = 64u8;
                    if equal(a3109, a3110) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3111, a3112)?
            };
            let a3124 = {
                let a3121 = {
                    let a3115 = {
                        let a3113 = v1;
                        let a3114 = 32u8;
                        fixed_u8::bit_and(a3113, a3114)
                    };
                    let a3116 = 32u8;
                    if equal(a3115, a3116) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3122 = {
                    let a3119 = {
                        let a3117 = v1;
                        let a3118 = 16u8;
                        fixed_u8::bit_and(a3117, a3118)
                    };
                    let a3120 = 16u8;
                    if equal(a3119, a3120) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3121, a3122)?
            };
            nat_add(a3123, a3124)?
        };
        let a3148 = {
            let a3145 = {
                let a3133 = {
                    let a3127 = {
                        let a3125 = v1;
                        let a3126 = 8u8;
                        fixed_u8::bit_and(a3125, a3126)
                    };
                    let a3128 = 8u8;
                    if equal(a3127, a3128) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3134 = {
                    let a3131 = {
                        let a3129 = v1;
                        let a3130 = 4u8;
                        fixed_u8::bit_and(a3129, a3130)
                    };
                    let a3132 = 4u8;
                    if equal(a3131, a3132) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3133, a3134)?
            };
            let a3146 = {
                let a3143 = {
                    let a3137 = {
                        let a3135 = v1;
                        let a3136 = 2u8;
                        fixed_u8::bit_and(a3135, a3136)
                    };
                    let a3138 = 2u8;
                    if equal(a3137, a3138) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3144 = {
                    let a3141 = {
                        let a3139 = v1;
                        let a3140 = 1u8;
                        fixed_u8::bit_and(a3139, a3140)
                    };
                    let a3142 = 1u8;
                    if equal(a3141, a3142) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3143, a3144)?
            };
            nat_add(a3145, a3146)?
        };
        nat_add(a3147, a3148)?
    };
    let a3149 = 48u64;
    let a3150 = v2;
    if if nat_le(a3149, a3150) {
        let a3151 = v2;
        let a3152 = 57u64;
        nat_le(a3151, a3152)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3153 = 97u64;
        let a3154 = v2;
        if nat_le(a3153, a3154) {
            let a3155 = v2;
            let a3156 = 102u64;
            Ok(nat_le(a3155, a3156))
        } else {
            Ok(false)
        }
    }
}

pub fn f466() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f467(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3201 = {
            let a3177 = {
                let a3165 = {
                    let a3159 = {
                        let a3157 = v1;
                        let a3158 = 128u8;
                        fixed_u8::bit_and(a3157, a3158)
                    };
                    let a3160 = 128u8;
                    if equal(a3159, a3160) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3166 = {
                    let a3163 = {
                        let a3161 = v1;
                        let a3162 = 64u8;
                        fixed_u8::bit_and(a3161, a3162)
                    };
                    let a3164 = 64u8;
                    if equal(a3163, a3164) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3165, a3166)?
            };
            let a3178 = {
                let a3175 = {
                    let a3169 = {
                        let a3167 = v1;
                        let a3168 = 32u8;
                        fixed_u8::bit_and(a3167, a3168)
                    };
                    let a3170 = 32u8;
                    if equal(a3169, a3170) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3176 = {
                    let a3173 = {
                        let a3171 = v1;
                        let a3172 = 16u8;
                        fixed_u8::bit_and(a3171, a3172)
                    };
                    let a3174 = 16u8;
                    if equal(a3173, a3174) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3175, a3176)?
            };
            nat_add(a3177, a3178)?
        };
        let a3202 = {
            let a3199 = {
                let a3187 = {
                    let a3181 = {
                        let a3179 = v1;
                        let a3180 = 8u8;
                        fixed_u8::bit_and(a3179, a3180)
                    };
                    let a3182 = 8u8;
                    if equal(a3181, a3182) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3188 = {
                    let a3185 = {
                        let a3183 = v1;
                        let a3184 = 4u8;
                        fixed_u8::bit_and(a3183, a3184)
                    };
                    let a3186 = 4u8;
                    if equal(a3185, a3186) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3187, a3188)?
            };
            let a3200 = {
                let a3197 = {
                    let a3191 = {
                        let a3189 = v1;
                        let a3190 = 2u8;
                        fixed_u8::bit_and(a3189, a3190)
                    };
                    let a3192 = 2u8;
                    if equal(a3191, a3192) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3198 = {
                    let a3195 = {
                        let a3193 = v1;
                        let a3194 = 1u8;
                        fixed_u8::bit_and(a3193, a3194)
                    };
                    let a3196 = 1u8;
                    if equal(a3195, a3196) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3197, a3198)?
            };
            nat_add(a3199, a3200)?
        };
        nat_add(a3201, a3202)?
    };
    let a3203 = v2;
    let a3204 = 57u64;
    if nat_le(a3203, a3204) {
        let a3205 = v2;
        let a3206 = 48u64;
        Ok(nat_sub(a3205, a3206))
    } else {
        let a3207 = v2;
        let a3208 = 87u64;
        Ok(nat_sub(a3207, a3208))
    }
}

pub fn f468(v0: u64) -> Bytes {
    f32({
        let a3209 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3209)
    }, v0, 1u64)
}

pub fn f469(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f334(Fn10::F472, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f470(v0: Fn25, p3214: u64, p3215: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3213 = (p3214, p3215);
    {
        let mut m3213 = m3213;
        loop {
            let step = {
                let (v1, v2) = m3213;
                let m3210 = v1;
                if m3210 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3210 - 1;
                    let m3212 = {
                        let c3211: Fn25 = v0.clone();
                        c3211.apply(v2.clone())
                    };
                    match m3212 {
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
                Ok(next) => { m3213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f473(v0)
}

pub fn f472(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f473(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3216, _) = v0.clone();
        h3216
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3217, _) = {
            let (_, h3218) = v0.clone();
            h3218
        };
        h3217
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3219) = {
            let (_, h3220) = v0;
            h3220
        };
        h3219
    };
    let m3221 = v1.clone();
    match m3221.uncons() {
        None => {
            let m3222 = v2;
            match m3222.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3223 = v2.clone();
            match m3223.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f464({
                        let (h3224, _) = v8.clone();
                        h3224
                    }, {
                        let (h3225, _) = v6.clone();
                        h3225
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
