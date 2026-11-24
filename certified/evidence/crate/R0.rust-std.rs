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
    F169(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F55(k0, k1) => f55(*k0, k1.clone(), p0),
            Fn0::F169(k0) => f169(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F148,
    F165,
    F233,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F148 => f148(p0, p1),
            Fn1::F165 => f165(p0, p1),
            Fn1::F233 => f233(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F198(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F198(k0) => f198(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F222(Bytes, u64, Bytes, u64),
    F314(Bytes),
    F315(u64, Bytes),
    F377(Bytes),
    F423(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F222(k0, k1, k2, k3) => f222(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F314(k0) => Ok(f314(k0.clone(), p0)),
            Fn3::F315(k0, k1) => f315(*k0, k1.clone(), p0),
            Fn3::F377(k0) => f377(k0.clone(), p0),
            Fn3::F423(k0) => f423(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F264(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt22)) -> R<(u64, Adt22)> {
        match self {
            Fn4::F264(k0, k1, k2) => f264(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F295,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn5::F295 => f295(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F298,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn6::F298 => f298(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F300,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn7::F300 => f300(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F303,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn8::F303 => f303(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F330(Bytes),
    F387(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F330(k0) => f330(k0.clone(), p0),
            Fn9::F387(k0, k1) => Ok(f387(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F332,
    F456,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F332 => f332(p0, p1),
            Fn10::F456 => f456(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F335,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F335 => f335(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F337,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F337 => f337(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F340,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F340 => f340(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F345,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn14::F345 => f345(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F350(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F350(k0) => f350(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F380(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F380(k0) => f380(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F391,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F391 => f391(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F393,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F393 => f393(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F395,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F395 => f395(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F403,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F403 => f403(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F415,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F415 => f415(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F438,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F438 => f438(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F440,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F440 => f440(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F442(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F442(k0) => f442(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F455,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F455 => f455(p0),
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
            }, v4))?, f161(match v0.clone() {
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
                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f117(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    f163(f81(v0)?, f8(Str::lit("id = \"")))
}

pub fn f68(v0: Adt1, v1: Bytes) -> Bytes {
    let m413 = f149(match v0 {
        Adt1::C0(h412, _, _) => {
            h412
        }
    }, v1);
    match m413 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m414 = v2;
            match m414 {
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
    let m415 = f108(f68(v0, f113()))?;
    match m415.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f118(v2))
        }
    }
}

pub fn f70(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m416 = v0;
    match m416.uncons() {
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
    let m417 = v0;
    match m417.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m418 = f144(Some(v1));
            match m418 {
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

pub fn f74(p422: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m421 = p422;
    {
        let mut m421 = m421;
        loop {
            let step = {
                let v0 = m421;
                let m419 = v0;
                match m419.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h420, _) = v2;
                            h420
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m421 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f76(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f164(f147(Fn1::F165, List::<Bytes>::nil(), f166(v0)?)))
}

pub fn f77(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m423 = v1;
    match m423.uncons() {
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

pub fn f78(p428: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m427 = p428;
    {
        let mut m427 = m427;
        loop {
            let step = {
                let v0 = m427;
                let m424 = v0;
                match m424.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h425, _) = v2;
                            h425
                        }, v1.clone()) {
                            let (_, h426) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h426)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: Adt1, v1: List<List<Bytes>>, p431: List<Bytes>) -> R<Option<Bytes>> {
    let m430 = p431;
    {
        let mut m430 = m430;
        loop {
            let step = {
                let v2 = m430;
                let m429 = v2;
                match m429.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f167(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
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

pub fn f80(p434: List<Bytes>, p435: List<Bytes>, p436: List<List<Bytes>>) -> List<List<Bytes>> {
    let m433 = (p434, (p435, p436));
    {
        let mut m433 = m433;
        loop {
            let step = {
                let (v0, (v1, v2)) = m433;
                let m432 = v0;
                match m432.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f168(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f36(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f168(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
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

pub fn f81(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a437 = v0.clone();
        length_bytes(a437)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h438, _) = f54(Fn0::F169(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h438
    };
    let v3: u64 = {
        let (h439, _) = {
            let (_, h440) = v2.clone();
            h440
        };
        h439
    };
    let v4: List<Bytes> = {
        let (_, h441) = {
            let (_, h442) = v2;
            h442
        };
        h441
    };
    let a443 = v3;
    let a444 = v1;
    if nat_lt(a443, a444) {
        Ok(f56(List::cons(f32(v0, v3, {
            let a445 = v1;
            let a446 = v3;
            nat_sub(a445, a446)
        }), v4)))
    } else {
        Ok(f56(v4))
    }
}

pub fn f82(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m447 = v0;
    match m447.uncons() {
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
        Adt1::C0(_, h448, _) => {
            h448
        }
    }, f170())) {
        Ok(Adt11::C1(List::cons(f170(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f82(f76(f77(f170(), f78(match v0.clone() {
            Adt1::C0(_, h449, _) => {
                h449
            }
        }, f170())))?, v1.clone());
        let m450 = f58(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m450.uncons() {
            None => {
                let m451 = f171(v0.clone(), v2, f70(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m451 {
                    None => {
                        f172(v0, v1)
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

pub fn f84(v0: Bytes, p454: List<Bytes>) -> bool {
    let m453 = p454;
    {
        let mut m453 = m453;
        loop {
            let step = {
                let v1 = m453;
                let m452 = v1;
                match m452.uncons() {
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
                Ok(next) => { m453 = next; }
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
        Adt1::C0(_, h455, _) => {
            h455
        }
    }, f123()) {
        if f74(match v0 {
            Adt1::C0(_, h456, _) => {
                h456
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
    let m457 = v0;
    match m457.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f89(v3, v1))
        }
    }
}

pub fn f90(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m458 = f71(v0.clone(), f87())?;
    match m458.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f173(v0)?), List::<(Bytes, Bytes)>::nil()))
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
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**', 'model/encodings/**', 'model/queue/**', 'model/authorities.toml']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            while read -r file line; do head -n \"$line\" \"$file\" | grep '^  name: ' | tail -n 1 | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }'; done < target/hunks.txt | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f94() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src and\u{a}# README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one.\u{a}name: refresh\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  refresh:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply each change to the new main, export it again, and push it to its own pull request\u{a}        if: steps.stale.outputs.stale != ''\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ steps.stale.outputs.stale }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again either way.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /'\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}"))
}

pub fn f95() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f96(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f100(v0.clone())?;
    let v2: List<Adt14> = f71(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f105(v1.clone(), f106(v0.clone(), f107(v0.clone())?)?);
    let v4: List<List<Bytes>> = f119(f108(f68(v0, f113()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f174(v4.clone(), f118(v4), f175(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f104(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f97(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f176(f100(v0.clone())?, f106(v0.clone(), f107(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f98(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f177(f71(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f178(f71(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f99() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f100(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f69(v0.clone())?, f67(f68(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f70(f71(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f101(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m459 = v0;
    match m459.uncons() {
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
    let m460 = v0;
    match m460.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m461 = f136(v1, f8(Str::lit("std")));
            match m461 {
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
    let m462 = v0;
    match m462.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f89(f179(v2.clone()), f180(f175(v0, v1.clone(), v2)?));
    f181(f76(f182(v1))?, v3.clone(), {
        let a464 = {
            let a463 = v3;
            length_list(a463)
        };
        let a465 = 1u64;
        nat_add(a464, a465)?
    })
}

pub fn f105(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m466 = v1;
    match m466.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f183(v2.clone(), f8(Str::lit("subject"))) {
                f184(v0.clone(), v2.clone())
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
    let m467 = v1;
    match m467.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m468 = f126(f81(f68(v0.clone(), v2.clone()))?, f185(v2, f122()))?;
            match m468 {
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
    Ok(f62(f76(f186(f123(), f78(match v0.clone() {
        Adt1::C0(_, h469, _) => {
            h469
        }
    }, f123())))?, f76(f186(f122(), f78(match v0 {
        Adt1::C0(_, h470, _) => {
            h470
        }
    }, f122())))?))
}

pub fn f108(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f187(v0.clone(), 0u64, {
        let a471 = v0;
        length_bytes(a471)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a474 = {
            let a473 = match v1.clone() {
                Adt17::C0(_, _, h472, _, _) => {
                    h472
                }
            };
            length_bytes(a473)
        };
        let a475 = 0u64;
        if if nat_eq(a474, a475) {
            let m477 = match v1.clone() {
                Adt17::C0(_, _, _, h476, _) => {
                    h476
                }
            };
            match m477.uncons() {
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
            f188(v1)
        }
    };
    Ok(f189(match v4 {
        Adt17::C0(_, _, _, _, h478) => {
            h478
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f109(p483: List<Bytes>, p484: List<Bytes>) -> bool {
    let m482 = (p483, p484);
    {
        let mut m482 = m482;
        loop {
            let step = {
                let (v0, v1) = m482;
                let m479 = v0;
                match m479.uncons() {
                    None => {
                        let m480 = v1;
                        match m480.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m481 = v1;
                        match m481.uncons() {
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
                Ok(next) => { m482 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: List<List<Bytes>>) -> List<Bytes> {
    let m485 = v0;
    match m485.uncons() {
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
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f190(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f113() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f114(v0: Bytes) -> R<Bytes> {
    Ok(f191(f192(v0)?))
}

pub fn f115(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f193(f80(f81(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f194(f195(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f116(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f138(Adt14::C3(std::rc::Rc::new(f196(f76(f62(v1, f72(f71(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f117(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m486 = f197(Fn2::F198(v1), None::<Bytes>, v0);
    match m486 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f118(v0: List<List<Bytes>>) -> List<Bytes> {
    let m487 = v0;
    match m487.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m488 = v1;
            match m488.uncons() {
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
    let m489 = v0;
    match m489.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f120(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m490 = v0;
    match m490.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f199(v1)?;
            let a492 = {
                let a491 = v3.clone();
                length_bytes(a491)
            };
            let a493 = 0u64;
            if nat_eq(a492, a493) {
                f120(v2)
            } else {
                let m494 = f200(v3.clone())?;
                match m494 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f139(v4.clone(), f8(Str::lit("id")));
                        if if f36(f139(v4, f8(Str::lit("schema"))), f143()) {
                            f73({
                                let a496 = {
                                    let a495 = v5.clone();
                                    length_bytes(a495)
                                };
                                let a497 = 0u64;
                                nat_eq(a496, a497)
                            })
                        } else {
                            false
                        } {
                            let m498 = f120(v2)?;
                            match m498 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f201(v5), {
                                        let a499 = v3;
                                        let a500 = Bytes::lit(&[0x0a]);
                                        append_bytes(a499, a500)
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
        let a501 = v0.clone();
        length_bytes(a501)
    };
    let v3: u64 = {
        let a502 = v1.clone();
        length_bytes(a502)
    };
    let a503 = v3;
    let a504 = v2;
    if nat_le(a503, a504) {
        let m510 = {
            let a507 = v0;
            let a508 = {
                let a505 = v2;
                let a506 = v3;
                nat_sub(a505, a506)
            };
            let a509 = v3;
            slice_bytes(a507, a508, a509)
        };
        match m510 {
            None => {
                false
            }
            Some(v4) => {
                let a511 = v4;
                let a512 = v1;
                equal(a511, a512)
            }
        }
    } else {
        false
    }
}

pub fn f126(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m513 = v0;
    match m513.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f199(v2)?;
            let a515 = {
                let a514 = v4.clone();
                length_bytes(a514)
            };
            let a516 = 0u64;
            if nat_eq(a515, a516) {
                f126(v3, v1)
            } else {
                let m517 = f200(v4)?;
                match m517 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f202(v5.clone())
                        } else {
                            f203(v5.clone())
                        } {
                            let m518 = f126(v3, v1)?;
                            match m518 {
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
    if f204(f141(v0.clone())?) {
        Ok(f73({
            let a520 = {
                let a519 = f205(f141(v0)?);
                length_list(a519)
            };
            let a521 = 0u64;
            nat_eq(a520, a521)
        }))
    } else {
        Ok(false)
    }
}

pub fn f128(v0: List<Bytes>) -> R<bool> {
    if f204(f131(v0.clone(), f8(Str::lit("requirement")))?) {
        if f204(f131(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f73({
                let a523 = {
                    let a522 = f133(f205(f131(v0.clone(), f8(Str::lit("requirement")))?), f205(f131(v0, f8(Str::lit("admission")))?));
                    length_list(a522)
                };
                let a524 = 0u64;
                nat_eq(a523, a524)
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
    let m525 = v0;
    match m525.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f199(v2)?;
            let a527 = {
                let a526 = v4.clone();
                length_bytes(a526)
            };
            let a528 = 0u64;
            if nat_eq(a527, a528) {
                f131(v3, v1)
            } else {
                let m529 = f200(v4)?;
                match m529 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f206(v5.clone());
                        let a531 = {
                            let a530 = v6.clone();
                            length_bytes(a530)
                        };
                        let a532 = 0u64;
                        if nat_eq(a531, a532) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m533 = f131(v3, v1.clone())?;
                            match m533 {
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
    Ok((v0, f138(Adt14::C3(std::rc::Rc::new(f207(v1))))?))
}

pub fn f133(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m534 = v0;
    match m534.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f133(v3, v1))
        }
    }
}

pub fn f134(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m535 = v0;
    match m535.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f199(v1)?;
            let a537 = {
                let a536 = v3.clone();
                length_bytes(a536)
            };
            let a538 = 0u64;
            if nat_eq(a537, a538) {
                f134(v2)
            } else {
                let m539 = f200(v3.clone())?;
                match m539 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f208(v4.clone()) {
                            let m540 = f134(v2)?;
                            match m540 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f170(), List::cons(f8(Str::lit("/")), List::cons(f209(f139(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a541 = v3;
                                        let a542 = Bytes::lit(&[0x0a]);
                                        append_bytes(a541, a542)
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
    f200(f68(v0, v1))
}

pub fn f136(p548: Adt14, v1: Bytes) -> Option<Adt14> {
    let m547 = p548;
    {
        let mut m547 = m547;
        loop {
            let step = {
                let v0 = m547;
                let m543 = v0;
                match m543 {
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
                    Adt14::C4(r544) => {
                        let v5 = (*r544).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r545, r546) => {
                        let v3 = (*r545).clone();
                        let v4 = (*r546).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m547 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m549 = f58(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m549.uncons() {
        None => {
            let m550 = f210(v4);
            match m550 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a551 = f8(Str::lit("Catalog."));
                        let a552 = f209(v3.clone())?;
                        append_bytes(a551, a552)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f209(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m553 = f211(v2, v9.clone(), f209(v3.clone())?)?;
                    match m553 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f209(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f138(Adt14::C3(std::rc::Rc::new(f207(f212(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f138(Adt14::C3(std::rc::Rc::new(f207(f212(f71(v0.clone(), f8(Str::lit("model/strata.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a557 = f213(f68(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a555 = {
                                        let a554 = v10;
                                        length_bytes(a554)
                                    };
                                    let a556 = 4u64;
                                    nat_sub(a555, a556)
                                }))?;
                                let a558 = Bytes::lit(&[0x0a]);
                                append_bytes(a557, a558)
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
    let a559 = f214(v0)?;
    let a560 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a559, a560))
}

pub fn f139(v0: Adt14, v1: Bytes) -> Bytes {
    let m561 = f144(f136(v0, v1));
    match m561 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f140(v0: Option<Adt14>) -> List<Adt14> {
    let m562 = v0;
    match m562 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f215(v1)
        }
    }
}

pub fn f141(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m563 = v0;
    match m563.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f199(v1)?;
            let a565 = {
                let a564 = v3.clone();
                length_bytes(a564)
            };
            let a566 = 0u64;
            if nat_eq(a565, a566) {
                f141(v2)
            } else {
                let m567 = f200(v3)?;
                match m567 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f183(v4.clone(), f8(Str::lit("agent"))) {
                            if f183(v4.clone(), f8(Str::lit("at"))) {
                                if f183(v4.clone(), f8(Str::lit("quote"))) {
                                    f216(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m568 = f141(v2)?;
                            match m568 {
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

pub fn f142(p571: List<Adt14>) -> bool {
    let m570 = p571;
    {
        let mut m570 = m570;
        loop {
            let step = {
                let v0 = m570;
                let m569 = v0;
                match m569.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f139(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f84(f8(Str::lit("studio")), f70(f217(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m570 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f144(v0: Option<Adt14>) -> Option<Bytes> {
    let m572 = v0;
    match m572 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m573 = v1;
            match m573 {
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
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f115(f68(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f218(v1, v4)?)?), List::cons(f116(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f146(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h574, _) = v2.clone();
        h574
    };
    let v4: u64 = {
        let (h575, _) = {
            let (_, h576) = v2.clone();
            h576
        };
        h575
    };
    let v5: List<Bytes> = {
        let (_, h577) = {
            let (_, h578) = v2;
            h578
        };
        h577
    };
    let m579 = f219(v0.clone(), v3);
    match m579 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a580 = v6;
            let a581 = v1;
            if equal(a580, a581) {
                Ok(Some(({
                    let a582 = v3;
                    let a583 = 1u64;
                    nat_add(a582, a583)?
                }, ({
                    let a584 = v3;
                    let a585 = 1u64;
                    nat_add(a584, a585)?
                }, List::cons(f32(v0, v4, {
                    let a586 = v3;
                    let a587 = v4;
                    nat_sub(a586, a587)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a588 = v3;
                    let a589 = 1u64;
                    nat_add(a588, a589)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f147(v0: Fn1, p593: List<Bytes>, p594: List<Bytes>) -> List<Bytes> {
    let m592 = (p593, p594);
    {
        let mut m592 = m592;
        loop {
            let step = {
                let (v1, v2) = m592;
                let m590 = v2;
                match m590.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c591: Fn1 = v0.clone();
                            c591.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m592 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f149(p599: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m598 = p599;
    {
        let mut m598 = m598;
        loop {
            let step = {
                let v0 = m598;
                let m595 = v0;
                match m595.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h596, _) = v2;
                            h596
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h597) = v2;
                                h597
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m598 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Adt1, v1: Bytes) -> bool {
    let m601 = f149(match v0 {
        Adt1::C0(h600, _, _) => {
            h600
        }
    }, v1);
    match m601 {
        None => {
            false
        }
        Some(v2) => {
            let m602 = v2;
            match m602 {
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
    let m603 = v1;
    match m603.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f220(v3.clone())
            } else {
                f73(f220(v3.clone()))
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
        let a604 = v0.clone();
        length_bytes(a604)
    };
    let v3: u64 = {
        let a605 = v1.clone();
        length_bytes(a605)
    };
    let v4: u64 = {
        let (h606, _) = f221(Fn3::F222(v1.clone(), v3, v0.clone(), v2), {
            let a607 = v2;
            let a608 = 1u64;
            nat_add(a607, a608)?
        }, 0u64)?;
        h606
    };
    if f223(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f153() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f154(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f224(match v0.clone() {
        Adt12::C0(h609, _, _, _, _) => {
            h609
        }
    }, 0u64)?, f224(match v0.clone() {
        Adt12::C0(_, h610, _, _, _) => {
            h610
        }
    }, 0u64)?, f224(match v0.clone() {
        Adt12::C0(_, _, h611, _, _) => {
            h611
        }
    }, 0u64)?, f224(match v0 {
        Adt12::C0(_, _, _, h612, _) => {
            h612
        }
    }, 0u64)?))
}

pub fn f155(v0: Adt12) -> List<Bytes> {
    f62(match v0.clone() {
        Adt12::C0(h613, _, _, _, _) => {
            h613
        }
    }, f62(match v0.clone() {
        Adt12::C0(_, _, h614, _, _) => {
            h614
        }
    }, f62(match v0.clone() {
        Adt12::C0(_, _, _, h615, _) => {
            h615
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h616) => {
            h616
        }
    })))
}

pub fn f156(v0: Adt16, p622: List<u64>, p623: List<Adt18>) -> Option<u64> {
    let m621 = (p622, p623);
    {
        let mut m621 = m621;
        loop {
            let step = {
                let (v1, v2) = m621;
                let m617 = v2;
                match m617.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f225(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h618) => {
                                h618
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h619, _) => {
                                    h619
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h620, _) => {
                                    h620
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m624 = v1;
    match m624.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f226(v0.clone(), v3)?), f157(v0, v4, {
                let a625 = v2;
                let a626 = 1u64;
                nat_add(a625, a626)?
            })?))
        }
    }
}

pub fn f158(p630: List<Bytes>, p631: u64) -> Bytes {
    let m629 = (p630, p631);
    {
        let mut m629 = m629;
        loop {
            let step = {
                let (v0, v1) = m629;
                let m627 = v0;
                match m627.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m628 = v1;
                        if m628 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m628 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m629 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f227(v0.clone(), v1.clone()) {
        if f227(v1, v0) {
            f228(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f160(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m632 = v0;
    match m632.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f229(v2, v1.clone())?, f160(v3, v1)?))
        }
    }
}

pub fn f161(v0: List<Adt14>) -> List<u64> {
    let m633 = v0;
    match m633.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f230(v1), f161(v2))
        }
    }
}

pub fn f162(p636: List<u64>, v1: List<u64>) -> Option<u64> {
    let m635 = p636;
    {
        let mut m635 = m635;
        loop {
            let step = {
                let v0 = m635;
                let m634 = v0;
                match m634.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f231(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m637 = v0;
    match m637.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m638 = f232(v2, v1.clone())?;
            match m638 {
                None => {
                    f163(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f163(v3, v1)?))
                }
            }
        }
    }
}

pub fn f164(v0: List<Bytes>) -> List<Bytes> {
    f147(Fn1::F233, List::<Bytes>::nil(), v0)
}

pub fn f165(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f234(v0, v1)
}

pub fn f166(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f235(f236(f237(v0))?))
}

pub fn f167(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m639 = f135(v0, v2)?;
    match m639 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f238(v1, f139(v3.clone(), f8(Str::lit("id"))))?;
            let m640 = v4.clone();
            match m640.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m642 = f144(f136({
                        let m641 = f136(v3.clone(), f8(Str::lit("source")));
                        match m641 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m642 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f73(f84(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f190(f239(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f168(p645: List<Bytes>, p646: List<Bytes>) -> List<Bytes> {
    let m644 = (p645, p646);
    {
        let mut m644 = m644;
        loop {
            let step = {
                let (v0, v1) = m644;
                let m643 = v0;
                match m643.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m644 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f240(v0, v1)
}

pub fn f170() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f171(v0: Adt1, p650: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m649 = p650;
    {
        let mut m649 = m649;
        loop {
            let step = {
                let v1 = m649;
                let m647 = v1;
                match m647.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f84(f241(f139({
                            let m648 = f135(v0.clone(), v3.clone())?;
                            match m648 {
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
                Ok(next) => { m649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f73(f74(match v0.clone() {
        Adt1::C0(_, h651, _) => {
            h651
        }
    }, f129())) {
        Ok(Adt11::C1(List::cons(f129(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f82(f76(f242(f78(match v0.clone() {
            Adt1::C0(_, h652, _) => {
                h652
            }
        }, f129())))?, v1);
        let m653 = f58(v0.clone(), List::cons(f87(), List::cons(f113(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m653.uncons() {
            None => {
                let v5: (bool, Bytes) = f243(v0.clone(), v2)?;
                let (h654, _) = v5;
                if h654 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h655) = v5;
                        h655
                    }))
                } else {
                    let m656 = f86(v0.clone());
                    match m656.uncons() {
                        None => {
                            let m657 = f88(v0.clone())?;
                            match m657.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h658) = v5;
                                        h658
                                    }, List::cons(f7(f76(f244(f100(v0.clone())?, f106(v0.clone(), f107(v0.clone())?)?))?), List::cons(f245(v0)?, List::<Bytes>::nil()))))))
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

pub fn f173(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f246(f71(v0, f87())?)?)))
}

pub fn f174(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt21>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m659 = v1;
    match m659.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f62(f247(v5.clone(), v2.clone()), f248(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f249(v0.clone(), v5.clone()), List::cons(if f84(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a661 = {
                    let a660 = v7.clone();
                    length_list(a660)
                };
                let a662 = 0u64;
                if nat_eq(a661, a662) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f117(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f174(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f175(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt21>> {
    let v3: List<(Bytes, Bytes)> = f250(v1);
    let v4: List<(Bytes, Bytes)> = f251(v0, v2);
    f252(v3.clone(), f89(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f176(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m663 = v1;
    match m663.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f183(v2.clone(), f8(Str::lit("domain"))) {
                f253(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f139(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f139(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m664 = f136(v2.clone(), f8(Str::lit("role")));
                    match m664 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f139(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f139(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f139(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f254(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m665 = f136(v2.clone(), f8(Str::lit("origin")));
                    match m665 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f139(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m666 = f136(v2, f8(Str::lit("evidence")));
                    match m666 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f139(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f176(v0, v3))
            } else {
                f176(v0, v3)
            }
        }
    }
}

pub fn f177(v0: List<Adt14>) -> List<Bytes> {
    let m667 = v0;
    match m667.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f139(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f139(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f139(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f177(v2))
        }
    }
}

pub fn f178(v0: List<Adt14>) -> List<Bytes> {
    let m668 = v0;
    match m668.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f139(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f255(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f178(v2))
        }
    }
}

pub fn f179(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m669 = v0;
    match m669.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f139(v1.clone(), f8(Str::lit("subject"))), f139(v1, f8(Str::lit("object")))), f179(v2))
        }
    }
}

pub fn f180(v0: List<Adt21>) -> List<(Bytes, Bytes)> {
    let m670 = v0;
    match m670.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt21::C0(h671, _, _) => {
                    h671
                }
            }, match v1 {
                Adt21::C0(_, _, h672) => {
                    h672
                }
            }), f180(v2))
        }
    }
}

pub fn f181(p679: List<Bytes>, v1: List<(Bytes, Bytes)>, p680: u64) -> R<List<Bytes>> {
    let m678 = (p679, p680);
    {
        let mut m678 = m678;
        loop {
            let step = {
                let (v0, v2) = m678;
                let m673 = v2;
                if m673 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m673 - 1;
                    let v4: List<Bytes> = f76(f62(v0.clone(), f256(v0.clone(), v1.clone())))?;
                    let a676 = {
                        let a674 = v4.clone();
                        length_list(a674)
                    };
                    let a677 = {
                        let a675 = v0.clone();
                        length_list(a675)
                    };
                    if nat_eq(a676, a677) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: List<Adt14>) -> List<Bytes> {
    let m681 = v0;
    match m681.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m682 = f136(v1, f8(Str::lit("std")));
            match m682 {
                None => {
                    f182(v2)
                }
                Some(v3) => {
                    List::cons(f139(v3.clone(), f8(Str::lit("authority"))), if f183(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f139(v3, f8(Str::lit("row"))), f182(v2))
                    } else {
                        f182(v2)
                    })
                }
            }
        }
    }
}

pub fn f183(v0: Adt14, v1: Bytes) -> bool {
    f73({
        let a684 = {
            let a683 = f139(v0, v1);
            length_bytes(a683)
        };
        let a685 = 0u64;
        nat_eq(a684, a685)
    })
}

pub fn f184(v0: Adt15, v1: Adt14) -> bool {
    if f257(v0.clone(), v1.clone(), f139(v1.clone(), f8(Str::lit("subject")))) {
        f257(v0, v1.clone(), f139(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f185(v0: Bytes, v1: Bytes) -> bool {
    let m690 = {
        let a687 = v0;
        let a688 = 0u64;
        let a689 = {
            let a686 = v1.clone();
            length_bytes(a686)
        };
        slice_bytes(a687, a688, a689)
    };
    match m690 {
        None => {
            false
        }
        Some(v2) => {
            let a691 = v2;
            let a692 = v1;
            equal(a691, a692)
        }
    }
}

pub fn f186(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m693 = v1;
    match m693.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f125(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f121(v0.clone(), v2), f186(v0, v3))
            } else {
                f186(v0, v3)
            }
        }
    }
}

pub fn f187(v0: Bytes, p751: u64, p752: u64, p753: Adt17) -> R<Adt17> {
    let m750 = (p751, (p752, p753));
    {
        let mut m750 = m750;
        loop {
            let step = {
                let (v1, (v2, v3)) = m750;
                let m694 = v2;
                if m694 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m694 - 1;
                    let v5: u64 = f258(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h695, _, _, _) => {
                            h695
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a696 = v1;
                            let a697 = 1u64;
                            nat_add(a696, a697)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h698, _, _) => {
                                h698
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h699, _) => {
                                h699
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h700) => {
                                h700
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h701, _, _, _, _) => {
                                h701
                            }
                        } {
                            let a702 = v5;
                            let a703 = 34u64;
                            if nat_eq(a702, a703) {
                                let a706 = f258(v0.clone(), {
                                    let a704 = v1;
                                    let a705 = 1u64;
                                    nat_add(a704, a705)?
                                })?;
                                let a707 = 34u64;
                                if nat_eq(a706, a707) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a708 = v1;
                                        let a709 = 1u64;
                                        nat_add(a708, a709)?
                                    }, (v4, Adt17::C0(true, true, f259(match v3.clone() {
                                        Adt17::C0(_, _, h710, _, _) => {
                                            h710
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h711, _) => {
                                            h711
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h712) => {
                                            h712
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a713 = v1;
                                        let a714 = 1u64;
                                        nat_add(a713, a714)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h715, _, _) => {
                                            h715
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h716, _) => {
                                            h716
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h717) => {
                                            h717
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a718 = v1;
                                    let a719 = 1u64;
                                    nat_add(a718, a719)?
                                }, (v4, Adt17::C0(true, false, f259(match v3.clone() {
                                    Adt17::C0(_, _, h720, _, _) => {
                                        h720
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h721, _) => {
                                        h721
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h722) => {
                                        h722
                                    }
                                }))))
                            }
                        } else {
                            let a723 = v5;
                            let a724 = 34u64;
                            if nat_eq(a723, a724) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a725 = v1;
                                    let a726 = 1u64;
                                    nat_add(a725, a726)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h727, _, _) => {
                                        h727
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h728, _) => {
                                        h728
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h729) => {
                                        h729
                                    }
                                }))))
                            } else {
                                let a730 = v5;
                                let a731 = 44u64;
                                if nat_eq(a730, a731) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a732 = v1;
                                        let a733 = 1u64;
                                        nat_add(a732, a733)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h734, _, _) => {
                                            h734
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h735, _) => {
                                            h735
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h736) => {
                                            h736
                                        }
                                    }))))
                                } else {
                                    let a737 = v5;
                                    let a738 = 13u64;
                                    if nat_eq(a737, a738) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a739 = v1;
                                            let a740 = 1u64;
                                            nat_add(a739, a740)?
                                        }, (v4, v3)))
                                    } else {
                                        let a741 = v5;
                                        let a742 = 10u64;
                                        if nat_eq(a741, a742) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a743 = v1;
                                                let a744 = 1u64;
                                                nat_add(a743, a744)?
                                            }, (v4, f188(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a745 = v1;
                                                let a746 = 1u64;
                                                nat_add(a745, a746)?
                                            }, (v4, Adt17::C0(false, false, f259(match v3.clone() {
                                                Adt17::C0(_, _, h747, _, _) => {
                                                    h747
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h748, _) => {
                                                    h748
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h749) => {
                                                    h749
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
                Ok(next) => { m750 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f260(List::cons(match v0.clone() {
        Adt17::C0(_, _, h754, _, _) => {
            h754
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h755, _) => {
            h755
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h756) => {
            h756
        }
    }))
}

pub fn f189(p759: List<List<Bytes>>, p760: List<List<Bytes>>) -> List<List<Bytes>> {
    let m758 = (p759, p760);
    {
        let mut m758 = m758;
        loop {
            let step = {
                let (v0, v1) = m758;
                let m757 = v0;
                match m757.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f261(v0.clone(), 0u64, {
        let a761 = v0;
        length_bytes(a761)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f191(v0: Adt22) -> Bytes {
    let a782 = f262(match v0.clone() {
        Adt22::C0(h762, _, _, _, _, _, _, _) => {
            h762
        }
    });
    let a783 = {
        let a780 = f262(match v0.clone() {
            Adt22::C0(_, h763, _, _, _, _, _, _) => {
                h763
            }
        });
        let a781 = {
            let a778 = f262(match v0.clone() {
                Adt22::C0(_, _, h764, _, _, _, _, _) => {
                    h764
                }
            });
            let a779 = {
                let a776 = f262(match v0.clone() {
                    Adt22::C0(_, _, _, h765, _, _, _, _) => {
                        h765
                    }
                });
                let a777 = {
                    let a774 = f262(match v0.clone() {
                        Adt22::C0(_, _, _, _, h766, _, _, _) => {
                            h766
                        }
                    });
                    let a775 = {
                        let a772 = f262(match v0.clone() {
                            Adt22::C0(_, _, _, _, _, h767, _, _) => {
                                h767
                            }
                        });
                        let a773 = {
                            let a770 = f262(match v0.clone() {
                                Adt22::C0(_, _, _, _, _, _, h768, _) => {
                                    h768
                                }
                            });
                            let a771 = f262(match v0 {
                                Adt22::C0(_, _, _, _, _, _, _, h769) => {
                                    h769
                                }
                            });
                            append_bytes(a770, a771)
                        };
                        append_bytes(a772, a773)
                    };
                    append_bytes(a774, a775)
                };
                append_bytes(a776, a777)
            };
            append_bytes(a778, a779)
        };
        append_bytes(a780, a781)
    };
    append_bytes(a782, a783)
}

pub fn f192(v0: Bytes) -> R<Adt22> {
    let v1: u64 = {
        let a784 = v0.clone();
        length_bytes(a784)
    };
    let v2: u64 = {
        let a790 = {
            let a787 = {
                let a785 = v1;
                let a786 = 72u64;
                nat_add(a785, a786)?
            };
            let a788 = 64u64;
            let a789 = 0u64;
            nat_quot(a787, a788, a789)
        };
        let a791 = 64u64;
        nat_mul(a790, a791)?
    };
    let (_, h792) = f263(Fn4::F264(v0, v1, v2), {
        let a793 = v2;
        let a794 = 64u64;
        let a795 = 0u64;
        nat_quot(a793, a794, a795)
    }, (0u64, f265()))?;
    Ok(h792)
}

pub fn f193(p798: List<List<Bytes>>, v1: Bytes, p799: List<Bytes>) -> R<List<Bytes>> {
    let m797 = (p798, p799);
    {
        let mut m797 = m797;
        loop {
            let step = {
                let (v0, v2) = m797;
                let m796 = v0;
                match m796.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f266(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f195(f267(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m797 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f268(v0)?))
}

pub fn f195(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m800 = v0;
    match m800.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m801 = v3.clone();
            match m801.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a804 = v2;
                    let a805 = {
                        let a802 = v1.clone();
                        let a803 = f195(v3, v1);
                        append_bytes(a802, a803)
                    };
                    append_bytes(a804, a805)
                }
            }
        }
    }
}

pub fn f196(v0: List<Bytes>) -> Adt14 {
    let m806 = v0;
    match m806.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f196(v2)))
        }
    }
}

pub fn f197(v0: Fn2, p810: Option<Bytes>, p811: List<Bytes>) -> Option<Bytes> {
    let m809 = (p810, p811);
    {
        let mut m809 = m809;
        loop {
            let step = {
                let (v1, v2) = m809;
                let m807 = v2;
                match m807.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c808: Fn2 = v0.clone();
                            c808.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m812 = v1;
    match m812 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a815 = {
                    let a813 = v3;
                    let a814 = v0;
                    append_bytes(a813, a814)
                };
                let a816 = v2;
                append_bytes(a815, a816)
            })
        }
    }
}

pub fn f199(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f268(v0.clone())?;
    let v2: u64 = f269(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a817 = v1;
        let a818 = v2;
        nat_sub(a817, a818)
    }))
}

pub fn f200(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f270(v0)?;
    let m822 = f271(v1.clone(), 0u64, {
        let a820 = {
            let a819 = v1.clone();
            length_bytes(a819)
        };
        let a821 = 2u64;
        nat_add(a820, a821)?
    }, 0u64)?;
    match m822 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a825 = {
                let (_, h823) = v2;
                h823
            };
            let a826 = {
                let a824 = v1;
                length_bytes(a824)
            };
            if nat_eq(a825, a826) {
                Ok(Some({
                    let (h827, _) = v2;
                    h827
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f201(v0: Bytes) -> Bytes {
    f7(List::cons(f75(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f202(v0: Adt14) -> bool {
    if f183(v0.clone(), f8(Str::lit("domain"))) {
        if f183(v0.clone(), f8(Str::lit("row"))) {
            if f84(f139(v0.clone(), f8(Str::lit("relation"))), f272()) {
                if f273(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f273(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f273(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f273(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f203(v0: Adt14) -> bool {
    if f183(v0.clone(), f8(Str::lit("subject"))) {
        if f183(v0.clone(), f8(Str::lit("object"))) {
            if f183(v0.clone(), f8(Str::lit("evidence"))) {
                f84(f139(v0, f8(Str::lit("relation"))), f274())
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

pub fn f204(v0: Option<List<Adt14>>) -> bool {
    let m828 = v0;
    match m828 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f205(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m829 = v0;
    match m829 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f206(v0: Adt14) -> Bytes {
    if f183(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f183(v0.clone(), f8(Str::lit("id"))) {
            if f183(v0.clone(), f8(Str::lit("statement"))) {
                if f183(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f216(v0, f8(Str::lit("evidence")))
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

pub fn f207(v0: List<Adt14>) -> Adt14 {
    let m830 = v0;
    match m830.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f207(v2)))
        }
    }
}

pub fn f208(v0: Adt14) -> bool {
    if f183(v0.clone(), f8(Str::lit("name"))) {
        if f216(v0.clone(), f8(Str::lit("schema"))) {
            let m831 = f210(f72(f140(f136(v0, f8(Str::lit("citation"))))));
            match m831 {
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

pub fn f209(v0: Bytes) -> R<Bytes> {
    f275(v0.clone(), 0u64, {
        let a832 = v0;
        length_bytes(a832)
    }, true, Bytes::lit(&[]))
}

pub fn f210(v0: List<Bytes>) -> Option<Adt14> {
    let m833 = v0;
    match m833.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m834 = v2;
                match m834.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m835 = v2;
                    match m835.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m836 = v6;
                                match m836.uncons() {
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
                        let m837 = v2;
                        match m837.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m838 = v10;
                                match m838.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f196(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f211(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m839 = f276(4096u64, f277(f278(f279(f136(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt23::C0(v2, v0), List::<Adt23>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m839 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f195(f168(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f212(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    let m840 = v0;
    match m840.uncons() {
        None => {
            List::cons(v1, List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            List::cons(v2, f212(v3, v1))
        }
    }
}

pub fn f213(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f195(f280(f81(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f214(v0: Adt14) -> R<Bytes> {
    let m841 = v0;
    match m841 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f281(v2, v3)
        }
        Adt14::C2(v4) => {
            f282(v4)
        }
        Adt14::C3(r842) => {
            let v5 = (*r842).clone();
            Ok(f283(f284(v5)?))
        }
        Adt14::C4(r843) => {
            let v6 = (*r843).clone();
            f285(f286(v6)?)
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

pub fn f215(v0: Adt14) -> List<Adt14> {
    let m844 = v0;
    match m844 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r845) => {
            let v3 = (*r845).clone();
            f215(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r846, r847) => {
            let v1 = (*r846).clone();
            let v2 = (*r847).clone();
            List::cons(v1, f215(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f216(v0: Adt14, v1: Bytes) -> bool {
    let m848 = f136(v0, v1);
    match m848 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f217(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m849 = f136(v0, v1);
    match m849 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f140(f136(v3, v2))
        }
    }
}

pub fn f218(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f136(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f112(Str::lit("id"), f139(v0.clone(), f8(Str::lit("id"))))?, List::cons(f112(Str::lit("name"), f139(v0.clone(), f8(Str::lit("name"))))?, List::cons(f112(Str::lit("issuer"), f139(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f112(Str::lit("edition"), f139(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f112(Str::lit("canonical_id"), f139(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f112(Str::lit("source_role"), f139(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f112(Str::lit("retrieval_date"), f139(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f112(Str::lit("url"), f287(v2.clone(), f8(Str::lit("url"))))?, List::cons(f112(Str::lit("media_type"), f287(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f112(Str::lit("license"), f287(v2.clone(), f8(Str::lit("license"))))?, List::cons(f112(Str::lit("redistribution"), f287(v2, f8(Str::lit("redistribution"))))?, List::cons(f112(Str::lit("statement"), f139(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m850 = v1;
        match m850 {
            None => {
                f7(List::cons(f112(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f112(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f112(Str::lit("artifact"), {
                    let (h851, _) = v3;
                    h851
                })?, List::cons(f112(Str::lit("checksum"), {
                    let (_, h852) = v3;
                    h852
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f288(f72(f140(f136(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f219(v0: Bytes, v1: u64) -> Option<u8> {
    let a853 = v0;
    let a854 = v1;
    index_bytes(a853, a854)
}

pub fn f220(v0: Bytes) -> bool {
    f125(v0, f8(Str::lit(".lex.tex")))
}

pub fn f221(v0: Fn3, p859: u64, p860: u64) -> R<(u64, bool)> {
    let m858 = (p859, p860);
    {
        let mut m858 = m858;
        loop {
            let step = {
                let (v1, v2) = m858;
                let m855 = v1;
                if m855 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m855 - 1;
                    let m857 = {
                        let c856: Fn3 = v0.clone();
                        c856.apply(v2)?
                    };
                    match m857 {
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
                Ok(next) => { m858 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f223(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a863 = {
            let a861 = v4;
            let a862 = v1;
            nat_add(a861, a862)?
        };
        let a864 = v3;
        if nat_lt(a863, a864) {
            Ok(Some({
                let a865 = v4;
                let a866 = 1u64;
                nat_add(a865, a866)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f223(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m871 = {
        let a868 = v0;
        let a869 = v2;
        let a870 = {
            let a867 = v1.clone();
            length_bytes(a867)
        };
        slice_bytes(a868, a869, a870)
    };
    match m871 {
        None => {
            false
        }
        Some(v3) => {
            let a872 = v3;
            let a873 = v1;
            equal(a872, a873)
        }
    }
}

pub fn f224(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m874 = v0;
    match m874.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f224(v3, {
                let a875 = v1;
                let a876 = 1u64;
                nat_add(a875, a876)?
            })?))
        }
    }
}

pub fn f225(v0: Adt16, v1: List<u64>, v2: Adt19) -> bool {
    let m877 = v2;
    match m877 {
        Adt19::C0(v3, v4) => {
            if f231(v3, match v0.clone() {
                Adt16::C0(h878, _, _, _) => {
                    h878
                }
            }) {
                f289(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f231(v5, match v0 {
                Adt16::C0(_, _, h879, _) => {
                    h879
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f227(v7, v1) {
                f231(v8, match v0 {
                    Adt16::C0(_, _, _, h880) => {
                        h880
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f226(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m881 = f136(v1.clone(), f8(Str::lit("std")));
    match m881 {
        None => {
            let m886 = f144(f136(v1.clone(), f8(Str::lit("req"))));
            match m886 {
                None => {
                    let m888 = f136(v1, f8(Str::lit("der")));
                    match m888 {
                        None => {
                            Ok(f290(v0))
                        }
                        Some(v5) => {
                            let m889 = f291(f139(v5.clone(), f8(Str::lit("rule"))));
                            match m889 {
                                None => {
                                    Ok(f290(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f160(f72(f140(f136(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h890) => {
                                            h890
                                        }
                                    })?, f229(f139(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h891, _) => {
                                            h891
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f229(v4, match v0 {
                        Adt12::C0(_, _, h887, _, _) => {
                            h887
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m882 = f144(f136(v2.clone(), f8(Str::lit("row"))));
            match m882 {
                None => {
                    Ok(Adt19::C0(f229(f139(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h883, _, _, _, _) => {
                            h883
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f229(f139(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h884, _, _, _, _) => {
                            h884
                        }
                    })?, Some(f229(v3, match v0 {
                        Adt12::C0(_, h885, _, _, _) => {
                            h885
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f227(p894: List<u64>, v1: List<u64>) -> bool {
    let m893 = p894;
    {
        let mut m893 = m893;
        loop {
            let step = {
                let v0 = m893;
                let m892 = v0;
                match m892.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f231(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f228(p899: List<u64>) -> bool {
    let m898 = p899;
    {
        let mut m898 = m898;
        loop {
            let step = {
                let v0 = m898;
                let m895 = v0;
                match m895.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a896 = v1;
                        let a897 = 7u64;
                        if nat_le(a896, a897) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m898 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m900 = f292(v0, v1.clone(), 0u64)?;
    match m900 {
        None => {
            let a901 = v1;
            Ok(length_list(a901))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f230(v0: Adt14) -> u64 {
    let m902 = f293(f136(v0, f8(Str::lit("stratum"))));
    match m902 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f231(v0: u64, p907: List<u64>) -> bool {
    let m906 = p907;
    {
        let mut m906 = m906;
        loop {
            let step = {
                let v1 = m906;
                let m903 = v1;
                match m903.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a904 = v2;
                        let a905 = v0;
                        if nat_eq(a904, a905) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f185(v0.clone(), v1) {
        let m908 = f30(v0, 34u8)?;
        match m908.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m909 = v3;
                match m909.uncons() {
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

pub fn f233(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f234(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m910 = v0.clone();
    match m910.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a911 = v2;
            let a912 = v1.clone();
            if equal(a911, a912) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f235(v0: List<(Bytes, ())>) -> List<Bytes> {
    f164(f294(Fn5::F295, List::<Bytes>::nil(), v0))
}

pub fn f236(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f296(f297(Fn6::F298, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m917 = {
        let (h913, _) = f299(Fn7::F300, {
            let a915 = {
                let a914 = v0;
                length_list(a914)
            };
            let a916 = 1u64;
            nat_add(a915, a916)?
        }, v1)?;
        h913
    };
    match m917.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f237(v0: List<Bytes>) -> List<(Bytes, ())> {
    f301(f302(Fn8::F303, List::<(Bytes, ())>::nil(), v0))
}

pub fn f238(p920: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m919 = p920;
    {
        let mut m919 = m919;
        loop {
            let step = {
                let v0 = m919;
                let m918 = v0;
                match m918.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f266(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m919 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f287(f136(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f240(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h921, _) = v1.clone();
        h921
    };
    let v3: u64 = {
        let (h922, _) = {
            let (_, h923) = v1.clone();
            h923
        };
        h922
    };
    let v4: List<Bytes> = {
        let (_, h924) = {
            let (_, h925) = v1;
            h925
        };
        h924
    };
    let m926 = f219(v0.clone(), v2);
    match m926 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a927 = v5;
            let a928 = 10u8;
            if equal(a927, a928) {
                Ok(Some(({
                    let a929 = v2;
                    let a930 = 1u64;
                    nat_add(a929, a930)?
                }, ({
                    let a931 = v2;
                    let a932 = 1u64;
                    nat_add(a931, a932)?
                }, List::cons(f304(f32(v0, v3, {
                    let a933 = v2;
                    let a934 = v3;
                    nat_sub(a933, a934)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a935 = v2;
                    let a936 = 1u64;
                    nat_add(a935, a936)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f241(v0: Bytes) -> R<Bytes> {
    let a937 = f8(Str::lit("Catalog."));
    let a938 = f209(v0)?;
    Ok(append_bytes(a937, a938))
}

pub fn f242(v0: List<Bytes>) -> List<Bytes> {
    let m939 = v0;
    match m939.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f125(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f121(f129(), v1), f242(v2))
            } else {
                f242(v2)
            }
        }
    }
}

pub fn f243(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m940 = v1;
    match m940.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f305(v0.clone(), v2)?;
            let (h941, _) = v4.clone();
            if h941 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f243(v0, v3)?;
                let (h942, _) = v5.clone();
                if h942 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a945 = {
                            let (_, h943) = v4;
                            h943
                        };
                        let a946 = {
                            let (_, h944) = v5;
                            h944
                        };
                        append_bytes(a945, a946)
                    }))
                }
            }
        }
    }
}

pub fn f244(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m947 = v1;
    match m947.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f183(v2.clone(), f8(Str::lit("subject"))) {
                if f184(v0.clone(), v2.clone()) {
                    f244(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f139(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f139(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f139(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f244(v0, v3))
                }
            } else {
                if f253(v0.clone(), v2.clone()) {
                    f244(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f139(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f254(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f244(v0, v3))
                }
            }
        }
    }
}

pub fn f245(v0: Adt1) -> R<Bytes> {
    let a949 = {
        let a948 = f101(f118(f119(f108(f68(v0.clone(), f113()))?)), f102(f71(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a948)
    };
    let a950 = 0u64;
    if nat_eq(a949, a950) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f246(v0: List<Adt14>) -> R<List<Bytes>> {
    let m951 = v0;
    match m951.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f117(f72(f140(f136(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f139(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f139(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f306(f81(f139(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f246(v2)?))
        }
    }
}

pub fn f247(v0: Bytes, v1: List<Adt21>) -> List<Bytes> {
    let m952 = v1;
    match m952.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(match v2.clone() {
                Adt21::C0(h953, _, _) => {
                    h953
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt21::C0(_, h954, _) => {
                        h954
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt21::C0(_, _, h955) => {
                        h955
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f247(v0, v3))
            } else {
                f247(v0, v3)
            }
        }
    }
}

pub fn f248(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m956 = v1;
    match m956.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(f139(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f139(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f139(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f248(v0, v3))
            } else {
                if f36(f139(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f139(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f139(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f248(v0, v3))
                } else {
                    f248(v0, v3)
                }
            }
        }
    }
}

pub fn f249(p961: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m960 = p961;
    {
        let mut m960 = m960;
        loop {
            let step = {
                let v0 = m960;
                let m957 = v0;
                match m957.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m958 = v2;
                        match m958.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f36(v4, v1.clone()) {
                                    let m959 = v5;
                                    match m959.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f307(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m960 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m962 = v0;
    match m962.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m963 = f136(v1, f8(Str::lit("std")));
            match m963 {
                None => {
                    f250(v2)
                }
                Some(v3) => {
                    if f183(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f139(v3.clone(), f8(Str::lit("row"))), f139(v3, f8(Str::lit("authority")))), f250(v2))
                    } else {
                        f250(v2)
                    }
                }
            }
        }
    }
}

pub fn f251(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m964 = v1;
    match m964.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f139(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f139(v2, f8(Str::lit("object")));
            if if f84(v4.clone(), match v0.clone() {
                Adt15::C0(h965, _, _) => {
                    h965
                }
            }) {
                f84(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h966, _) => {
                        h966
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f251(v0, v3))
            } else {
                if if f84(v5.clone(), match v0.clone() {
                    Adt15::C0(h967, _, _) => {
                        h967
                    }
                }) {
                    f84(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h968, _) => {
                            h968
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f251(v0, v3))
                } else {
                    f251(v0, v3)
                }
            }
        }
    }
}

pub fn f252(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt21>> {
    let m969 = v0;
    match m969.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v4, v5)) => {
            if if f308(v4.clone(), v2.clone()) {
                true
            } else {
                f308(v4.clone(), v3.clone())
            } {
                f252(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt21::C0({
                    let (h970, _) = v4.clone();
                    h970
                }, {
                    let a973 = {
                        let a972 = f76(f309({
                            let (h971, _) = v4.clone();
                            h971
                        }, v1.clone()))?;
                        length_list(a972)
                    };
                    let a974 = 1u64;
                    if nat_eq(a973, a974) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h975) = v4.clone();
                    h975
                }), f252(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f253(v0: Adt15, v1: Adt14) -> bool {
    if f84(f139(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h976, _, _) => {
            h976
        }
    }) {
        f84(f254(v1), match v0 {
            Adt15::C0(_, h977, _) => {
                h977
            }
        })
    } else {
        false
    }
}

pub fn f254(v0: Adt14) -> Bytes {
    let m978 = f136(v0, f8(Str::lit("evidence")));
    match m978 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f139(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f255(v0: Adt14) -> Bytes {
    if f183(v0.clone(), f8(Str::lit("req"))) {
        let a979 = f8(Str::lit("req "));
        let a980 = f139(v0, f8(Str::lit("req")));
        append_bytes(a979, a980)
    } else {
        let m981 = f136(v0.clone(), f8(Str::lit("std")));
        match m981 {
            None => {
                let m984 = f136(v0, f8(Str::lit("der")));
                match m984 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f139(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f117(f72(f140(f136(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f139(v1.clone(), f8(Str::lit("authority"))), List::cons(if f183(v1.clone(), f8(Str::lit("row"))) {
                    let a982 = f8(Str::lit(" row "));
                    let a983 = f139(v1, f8(Str::lit("row")));
                    append_bytes(a982, a983)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f256(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m985 = v1;
    match m985.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84({
                let (h986, _) = v2.clone();
                h986
            }, v0.clone()) {
                List::cons({
                    let (_, h987) = v2;
                    h987
                }, f256(v0, v3))
            } else {
                if f84({
                    let (_, h988) = v2;
                    h988
                }, v0.clone()) {
                    List::cons({
                        let (h989, _) = v2;
                        h989
                    }, f256(v0, v3))
                } else {
                    f256(v0, v3)
                }
            }
        }
    }
}

pub fn f257(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f84(v2.clone(), match v0.clone() {
        Adt15::C0(h990, _, _) => {
            h990
        }
    }) {
        true
    } else {
        if f84(v2.clone(), match v0.clone() {
            Adt15::C0(_, h991, _) => {
                h991
            }
        }) {
            true
        } else {
            if f84(v2, match v0.clone() {
                Adt15::C0(_, _, h992) => {
                    h992
                }
            }) {
                true
            } else {
                if f183(v1.clone(), f8(Str::lit("scheme"))) {
                    f84(f139(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h993, _) => {
                            h993
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f258(v0: Bytes, v1: u64) -> R<u64> {
    let m996 = {
        let a994 = v0;
        let a995 = v1;
        index_bytes(a994, a995)
    };
    match m996 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1041 = {
                let a1017 = {
                    let a1005 = {
                        let a999 = {
                            let a997 = v3;
                            let a998 = 128u8;
                            fixed_u8::bit_and(a997, a998)
                        };
                        let a1000 = 128u8;
                        if equal(a999, a1000) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1006 = {
                        let a1003 = {
                            let a1001 = v3;
                            let a1002 = 64u8;
                            fixed_u8::bit_and(a1001, a1002)
                        };
                        let a1004 = 64u8;
                        if equal(a1003, a1004) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1005, a1006)?
                };
                let a1018 = {
                    let a1015 = {
                        let a1009 = {
                            let a1007 = v3;
                            let a1008 = 32u8;
                            fixed_u8::bit_and(a1007, a1008)
                        };
                        let a1010 = 32u8;
                        if equal(a1009, a1010) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1016 = {
                        let a1013 = {
                            let a1011 = v3;
                            let a1012 = 16u8;
                            fixed_u8::bit_and(a1011, a1012)
                        };
                        let a1014 = 16u8;
                        if equal(a1013, a1014) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1015, a1016)?
                };
                nat_add(a1017, a1018)?
            };
            let a1042 = {
                let a1039 = {
                    let a1027 = {
                        let a1021 = {
                            let a1019 = v3;
                            let a1020 = 8u8;
                            fixed_u8::bit_and(a1019, a1020)
                        };
                        let a1022 = 8u8;
                        if equal(a1021, a1022) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1028 = {
                        let a1025 = {
                            let a1023 = v3;
                            let a1024 = 4u8;
                            fixed_u8::bit_and(a1023, a1024)
                        };
                        let a1026 = 4u8;
                        if equal(a1025, a1026) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1027, a1028)?
                };
                let a1040 = {
                    let a1037 = {
                        let a1031 = {
                            let a1029 = v3;
                            let a1030 = 2u8;
                            fixed_u8::bit_and(a1029, a1030)
                        };
                        let a1032 = 2u8;
                        if equal(a1031, a1032) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1038 = {
                        let a1035 = {
                            let a1033 = v3;
                            let a1034 = 1u8;
                            fixed_u8::bit_and(a1033, a1034)
                        };
                        let a1036 = 1u8;
                        if equal(a1035, a1036) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1037, a1038)?
                };
                nat_add(a1039, a1040)?
            };
            nat_add(a1041, a1042)
        }
    }
}

pub fn f259(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1046 = {
        let a1043 = v1;
        let a1044 = v2;
        let a1045 = 1u64;
        slice_bytes(a1043, a1044, a1045)
    };
    match m1046 {
        None => {
            v0
        }
        Some(v3) => {
            let a1047 = v0;
            let a1048 = v3;
            append_bytes(a1047, a1048)
        }
    }
}

pub fn f260(p1051: List<Bytes>, p1052: List<Bytes>) -> List<Bytes> {
    let m1050 = (p1051, p1052);
    {
        let mut m1050 = m1050;
        loop {
            let step = {
                let (v0, v1) = m1050;
                let m1049 = v0;
                match m1049.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: Bytes, p1124: u64, p1125: u64, p1126: Bytes) -> R<Bytes> {
    let m1123 = (p1124, (p1125, p1126));
    {
        let mut m1123 = m1123;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1123;
                let m1053 = v2;
                if m1053 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1053 - 1;
                    let m1056 = {
                        let a1054 = v0.clone();
                        let a1055 = v1;
                        index_bytes(a1054, a1055)
                    };
                    match m1056 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1101 = {
                                    let a1077 = {
                                        let a1065 = {
                                            let a1059 = {
                                                let a1057 = v6;
                                                let a1058 = 128u8;
                                                fixed_u8::bit_and(a1057, a1058)
                                            };
                                            let a1060 = 128u8;
                                            if equal(a1059, a1060) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1066 = {
                                            let a1063 = {
                                                let a1061 = v6;
                                                let a1062 = 64u8;
                                                fixed_u8::bit_and(a1061, a1062)
                                            };
                                            let a1064 = 64u8;
                                            if equal(a1063, a1064) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1065, a1066)?
                                    };
                                    let a1078 = {
                                        let a1075 = {
                                            let a1069 = {
                                                let a1067 = v6;
                                                let a1068 = 32u8;
                                                fixed_u8::bit_and(a1067, a1068)
                                            };
                                            let a1070 = 32u8;
                                            if equal(a1069, a1070) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1076 = {
                                            let a1073 = {
                                                let a1071 = v6;
                                                let a1072 = 16u8;
                                                fixed_u8::bit_and(a1071, a1072)
                                            };
                                            let a1074 = 16u8;
                                            if equal(a1073, a1074) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1075, a1076)?
                                    };
                                    nat_add(a1077, a1078)?
                                };
                                let a1102 = {
                                    let a1099 = {
                                        let a1087 = {
                                            let a1081 = {
                                                let a1079 = v6;
                                                let a1080 = 8u8;
                                                fixed_u8::bit_and(a1079, a1080)
                                            };
                                            let a1082 = 8u8;
                                            if equal(a1081, a1082) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1088 = {
                                            let a1085 = {
                                                let a1083 = v6;
                                                let a1084 = 4u8;
                                                fixed_u8::bit_and(a1083, a1084)
                                            };
                                            let a1086 = 4u8;
                                            if equal(a1085, a1086) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1087, a1088)?
                                    };
                                    let a1100 = {
                                        let a1097 = {
                                            let a1091 = {
                                                let a1089 = v6;
                                                let a1090 = 2u8;
                                                fixed_u8::bit_and(a1089, a1090)
                                            };
                                            let a1092 = 2u8;
                                            if equal(a1091, a1092) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1098 = {
                                            let a1095 = {
                                                let a1093 = v6;
                                                let a1094 = 1u8;
                                                fixed_u8::bit_and(a1093, a1094)
                                            };
                                            let a1096 = 1u8;
                                            if equal(a1095, a1096) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1097, a1098)?
                                    };
                                    nat_add(a1099, a1100)?
                                };
                                nat_add(a1101, a1102)?
                            };
                            let a1103 = v7;
                            let a1104 = 34u64;
                            if if nat_eq(a1103, a1104) {
                                true
                            } else {
                                let a1105 = v7;
                                let a1106 = 92u64;
                                nat_eq(a1105, a1106)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1107 = v1;
                                    let a1108 = 1u64;
                                    nat_add(a1107, a1108)?
                                }, (v4, {
                                    let a1111 = v3;
                                    let a1112 = {
                                        let a1109 = Bytes::lit(&[0x5c]);
                                        let a1110 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1109, a1110)
                                    };
                                    append_bytes(a1111, a1112)
                                })))
                            } else {
                                let a1113 = v7;
                                let a1114 = 10u64;
                                if nat_eq(a1113, a1114) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1115 = v1;
                                        let a1116 = 1u64;
                                        nat_add(a1115, a1116)?
                                    }, (v4, {
                                        let a1117 = v3;
                                        let a1118 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1117, a1118)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1119 = v1;
                                        let a1120 = 1u64;
                                        nat_add(a1119, a1120)?
                                    }, (v4, {
                                        let a1121 = v3;
                                        let a1122 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1121, a1122)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f262(v0: u32) -> Bytes {
    let a1155 = f310({
        let a1127 = f311(v0, 28u32);
        let a1128 = 15u32;
        fixed_u32::bit_and(a1127, a1128)
    });
    let a1156 = {
        let a1153 = f310({
            let a1129 = f311(v0, 24u32);
            let a1130 = 15u32;
            fixed_u32::bit_and(a1129, a1130)
        });
        let a1154 = {
            let a1151 = f310({
                let a1131 = f311(v0, 20u32);
                let a1132 = 15u32;
                fixed_u32::bit_and(a1131, a1132)
            });
            let a1152 = {
                let a1149 = f310({
                    let a1133 = f311(v0, 16u32);
                    let a1134 = 15u32;
                    fixed_u32::bit_and(a1133, a1134)
                });
                let a1150 = {
                    let a1147 = f310({
                        let a1135 = f311(v0, 12u32);
                        let a1136 = 15u32;
                        fixed_u32::bit_and(a1135, a1136)
                    });
                    let a1148 = {
                        let a1145 = f310({
                            let a1137 = f311(v0, 8u32);
                            let a1138 = 15u32;
                            fixed_u32::bit_and(a1137, a1138)
                        });
                        let a1146 = {
                            let a1143 = f310({
                                let a1139 = f311(v0, 4u32);
                                let a1140 = 15u32;
                                fixed_u32::bit_and(a1139, a1140)
                            });
                            let a1144 = f310({
                                let a1141 = v0;
                                let a1142 = 15u32;
                                fixed_u32::bit_and(a1141, a1142)
                            });
                            append_bytes(a1143, a1144)
                        };
                        append_bytes(a1145, a1146)
                    };
                    append_bytes(a1147, a1148)
                };
                append_bytes(a1149, a1150)
            };
            append_bytes(a1151, a1152)
        };
        append_bytes(a1153, a1154)
    };
    append_bytes(a1155, a1156)
}

pub fn f263(v0: Fn4, p1160: u64, p1161: (u64, Adt22)) -> R<(u64, Adt22)> {
    let m1159 = (p1160, p1161);
    {
        let mut m1159 = m1159;
        loop {
            let step = {
                let (v1, v2) = m1159;
                let m1157 = v1;
                if m1157 == 0 {
                    Err::<(u64, (u64, Adt22)), R<(u64, Adt22)>>(Ok(v2))
                } else {
                    let v3 = m1157 - 1;
                    Ok::<(u64, (u64, Adt22)), R<(u64, Adt22)>>((v3, {
                        let c1158: Fn4 = v0.clone();
                        c1158.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt22)) -> R<(u64, Adt22)> {
    Ok(({
        let a1163 = {
            let (h1162, _) = v3.clone();
            h1162
        };
        let a1164 = 64u64;
        nat_add(a1163, a1164)?
    }, f312({
        let (_, h1165) = v3;
        h1165
    }, f313(v0, v1, v2, {
        let (h1166, _) = v3;
        h1166
    })?)?))
}

pub fn f265() -> Adt22 {
    Adt22::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f266(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f84(v1, f67(f195(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f267(v0: List<Bytes>) -> List<Bytes> {
    let m1167 = v0;
    match m1167.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f267(v2)
            } else {
                List::cons(v1, f267(v2))
            }
        }
    }
}

pub fn f268(v0: Bytes) -> R<u64> {
    let (h1168, _) = f221(Fn3::F314(v0.clone()), {
        let a1169 = v0.clone();
        length_bytes(a1169)
    }, {
        let a1170 = v0;
        length_bytes(a1170)
    })?;
    Ok(h1168)
}

pub fn f269(v0: Bytes, v1: u64) -> R<u64> {
    let (h1171, _) = f221(Fn3::F315(v1, v0), v1, 0u64)?;
    Ok(h1171)
}

pub fn f270(v0: Bytes) -> R<Bytes> {
    f316(v0.clone(), 0u64, {
        let a1172 = v0;
        length_bytes(a1172)
    }, false, false, Bytes::lit(&[]))
}

pub fn f271(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1173 = v2;
    if m1173 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1173 - 1;
        let v5: u64 = f317(v0.clone(), v1)?;
        let a1174 = v3;
        let a1175 = 1u64;
        if nat_eq(a1174, a1175) {
            let m1176 = f271(v0.clone(), v1, v4, 0u64)?;
            match m1176 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f317(v0.clone(), {
                        let (_, h1177) = v6.clone();
                        h1177
                    })?;
                    let a1178 = v7;
                    let a1179 = 44u64;
                    if nat_eq(a1178, a1179) {
                        let m1183 = f271(v0, {
                            let a1181 = {
                                let (_, h1180) = v6;
                                h1180
                            };
                            let a1182 = 1u64;
                            nat_add(a1181, a1182)?
                        }, v4, 1u64)?;
                        match m1183 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1184, _) = v6;
                                    h1184
                                }), std::rc::Rc::new({
                                    let (h1185, _) = v8;
                                    h1185
                                })), {
                                    let (_, h1186) = v8;
                                    h1186
                                })))
                            }
                        }
                    } else {
                        let a1187 = v7;
                        let a1188 = 93u64;
                        if nat_eq(a1187, a1188) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1189, _) = v6;
                                h1189
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1191 = {
                                    let (_, h1190) = v6;
                                    h1190
                                };
                                let a1192 = 1u64;
                                nat_add(a1191, a1192)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1193 = v3;
            let a1194 = 2u64;
            if nat_eq(a1193, a1194) {
                let m1197 = f318(v0.clone(), {
                    let a1195 = v1;
                    let a1196 = 1u64;
                    nat_add(a1195, a1196)?
                })?;
                match m1197 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1198 = v5;
                        let a1199 = 34u64;
                        if if nat_eq(a1198, a1199) {
                            let a1201 = f317(v0.clone(), {
                                let (h1200, _) = v9.clone();
                                h1200
                            })?;
                            let a1202 = 58u64;
                            nat_eq(a1201, a1202)
                        } else {
                            false
                        } {
                            let m1206 = f271(v0.clone(), {
                                let a1204 = {
                                    let (h1203, _) = v9;
                                    h1203
                                };
                                let a1205 = 1u64;
                                nat_add(a1204, a1205)?
                            }, v4, 0u64)?;
                            match m1206 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f317(v0.clone(), {
                                        let (_, h1207) = v10.clone();
                                        h1207
                                    })?;
                                    let a1208 = v11;
                                    let a1209 = 44u64;
                                    if nat_eq(a1208, a1209) {
                                        let m1213 = f271(v0, {
                                            let a1211 = {
                                                let (_, h1210) = v10;
                                                h1210
                                            };
                                            let a1212 = 1u64;
                                            nat_add(a1211, a1212)?
                                        }, v4, 2u64)?;
                                        match m1213 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1214) = v9;
                                                    h1214
                                                }, std::rc::Rc::new({
                                                    let (h1215, _) = v10;
                                                    h1215
                                                }), std::rc::Rc::new({
                                                    let (h1216, _) = v12;
                                                    h1216
                                                })), {
                                                    let (_, h1217) = v12;
                                                    h1217
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1218 = v11;
                                        let a1219 = 125u64;
                                        if nat_eq(a1218, a1219) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1220) = v9;
                                                h1220
                                            }, std::rc::Rc::new({
                                                let (h1221, _) = v10;
                                                h1221
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1223 = {
                                                    let (_, h1222) = v10;
                                                    h1222
                                                };
                                                let a1224 = 1u64;
                                                nat_add(a1223, a1224)?
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
                let a1225 = v5;
                let a1226 = 34u64;
                if nat_eq(a1225, a1226) {
                    let m1229 = f318(v0, {
                        let a1227 = v1;
                        let a1228 = 1u64;
                        nat_add(a1227, a1228)?
                    })?;
                    match m1229 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1230) = v13;
                                h1230
                            }), {
                                let (h1231, _) = v13;
                                h1231
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1232 = v1;
                            let a1233 = 4u64;
                            nat_add(a1232, a1233)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1234 = v1;
                                let a1235 = 5u64;
                                nat_add(a1234, a1235)?
                            })))
                        } else {
                            let a1236 = v5;
                            let a1237 = 91u64;
                            if nat_eq(a1236, a1237) {
                                let a1240 = f317(v0.clone(), {
                                    let a1238 = v1;
                                    let a1239 = 1u64;
                                    nat_add(a1238, a1239)?
                                })?;
                                let a1241 = 93u64;
                                if nat_eq(a1240, a1241) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1242 = v1;
                                        let a1243 = 2u64;
                                        nat_add(a1242, a1243)?
                                    })))
                                } else {
                                    let m1246 = f271(v0, {
                                        let a1244 = v1;
                                        let a1245 = 1u64;
                                        nat_add(a1244, a1245)?
                                    }, v4, 1u64)?;
                                    match m1246 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1247, _) = v14;
                                                h1247
                                            })), {
                                                let (_, h1248) = v14;
                                                h1248
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1249 = v5;
                                let a1250 = 123u64;
                                if nat_eq(a1249, a1250) {
                                    let a1253 = f317(v0.clone(), {
                                        let a1251 = v1;
                                        let a1252 = 1u64;
                                        nat_add(a1251, a1252)?
                                    })?;
                                    let a1254 = 125u64;
                                    if nat_eq(a1253, a1254) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1255 = v1;
                                            let a1256 = 2u64;
                                            nat_add(a1255, a1256)?
                                        })))
                                    } else {
                                        let m1259 = f271(v0, {
                                            let a1257 = v1;
                                            let a1258 = 1u64;
                                            nat_add(a1257, a1258)?
                                        }, v4, 2u64)?;
                                        match m1259 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1260, _) = v15;
                                                    h1260
                                                })), {
                                                    let (_, h1261) = v15;
                                                    h1261
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1262 = f319(v0, v1)?;
                                    match m1262 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1263) = v16;
                                                h1263
                                            }, {
                                                let (h1264, _) = v16;
                                                h1264
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

pub fn f272() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f273(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1265 = f136(v0, v1);
    match m1265 {
        None => {
            false
        }
        Some(v3) => {
            f183(v3, v2)
        }
    }
}

pub fn f274() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f275(v0: Bytes, p1332: u64, p1333: u64, p1334: bool, p1335: Bytes) -> R<Bytes> {
    let m1331 = (p1332, (p1333, (p1334, p1335)));
    {
        let mut m1331 = m1331;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1331;
                let m1266 = v2;
                if m1266 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1266 - 1;
                    let m1269 = {
                        let a1267 = v0.clone();
                        let a1268 = v1;
                        index_bytes(a1267, a1268)
                    };
                    match m1269 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1314 = {
                                    let a1290 = {
                                        let a1278 = {
                                            let a1272 = {
                                                let a1270 = v7;
                                                let a1271 = 128u8;
                                                fixed_u8::bit_and(a1270, a1271)
                                            };
                                            let a1273 = 128u8;
                                            if equal(a1272, a1273) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1279 = {
                                            let a1276 = {
                                                let a1274 = v7;
                                                let a1275 = 64u8;
                                                fixed_u8::bit_and(a1274, a1275)
                                            };
                                            let a1277 = 64u8;
                                            if equal(a1276, a1277) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1278, a1279)?
                                    };
                                    let a1291 = {
                                        let a1288 = {
                                            let a1282 = {
                                                let a1280 = v7;
                                                let a1281 = 32u8;
                                                fixed_u8::bit_and(a1280, a1281)
                                            };
                                            let a1283 = 32u8;
                                            if equal(a1282, a1283) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1289 = {
                                            let a1286 = {
                                                let a1284 = v7;
                                                let a1285 = 16u8;
                                                fixed_u8::bit_and(a1284, a1285)
                                            };
                                            let a1287 = 16u8;
                                            if equal(a1286, a1287) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1288, a1289)?
                                    };
                                    nat_add(a1290, a1291)?
                                };
                                let a1315 = {
                                    let a1312 = {
                                        let a1300 = {
                                            let a1294 = {
                                                let a1292 = v7;
                                                let a1293 = 8u8;
                                                fixed_u8::bit_and(a1292, a1293)
                                            };
                                            let a1295 = 8u8;
                                            if equal(a1294, a1295) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1301 = {
                                            let a1298 = {
                                                let a1296 = v7;
                                                let a1297 = 4u8;
                                                fixed_u8::bit_and(a1296, a1297)
                                            };
                                            let a1299 = 4u8;
                                            if equal(a1298, a1299) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1300, a1301)?
                                    };
                                    let a1313 = {
                                        let a1310 = {
                                            let a1304 = {
                                                let a1302 = v7;
                                                let a1303 = 2u8;
                                                fixed_u8::bit_and(a1302, a1303)
                                            };
                                            let a1305 = 2u8;
                                            if equal(a1304, a1305) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1311 = {
                                            let a1308 = {
                                                let a1306 = v7;
                                                let a1307 = 1u8;
                                                fixed_u8::bit_and(a1306, a1307)
                                            };
                                            let a1309 = 1u8;
                                            if equal(a1308, a1309) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1310, a1311)?
                                    };
                                    nat_add(a1312, a1313)?
                                };
                                nat_add(a1314, a1315)?
                            };
                            if f320(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1316 = v1;
                                    let a1317 = 1u64;
                                    nat_add(a1316, a1317)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f321(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1318 = v1;
                                        let a1319 = 1u64;
                                        nat_add(a1318, a1319)?
                                    }, (v5, (false, {
                                        let a1323 = v4;
                                        let a1324 = f32({
                                            let a1320 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1320)
                                        }, {
                                            let a1321 = v8;
                                            let a1322 = 97u64;
                                            nat_sub(a1321, a1322)
                                        }, 1u64);
                                        append_bytes(a1323, a1324)
                                    }))))
                                } else {
                                    if if f321(v8) {
                                        true
                                    } else {
                                        if f322(v8) {
                                            true
                                        } else {
                                            f323(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1325 = v1;
                                            let a1326 = 1u64;
                                            nat_add(a1325, a1326)?
                                        }, (v5, (false, {
                                            let a1327 = v4;
                                            let a1328 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1327, a1328)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1329 = v1;
                                            let a1330 = 1u64;
                                            nat_add(a1329, a1330)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
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

pub fn f276(p1343: u64, p1344: List<Adt23>, p1345: List<Bytes>, p1346: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1342 = (p1343, (p1344, (p1345, p1346)));
    {
        let mut m1342 = m1342;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1342;
                let m1336 = v0;
                if m1336 == 0 {
                    Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1336 - 1;
                    let m1337 = v1;
                    match m1337.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1338 = v5;
                            match m1338 {
                                Adt23::C0(v8, v9) => {
                                    if f84(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m1339 = f324(v8.clone(), v9)?;
                                        match m1339 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f277({
                                                    let (_, h1340) = v11;
                                                    h1340
                                                }, List::cons(Adt23::C1({
                                                    let (h1341, _) = v11;
                                                    h1341
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
                Ok(next) => { m1342 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(v0: List<Adt23>, v1: List<Adt23>) -> List<Adt23> {
    let m1347 = v0;
    match m1347.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f277(v3, v1))
        }
    }
}

pub fn f278(v0: List<(Bytes, Adt14)>) -> R<List<Adt23>> {
    let m1348 = v0;
    match m1348.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt23::C0(f209({
                let (h1349, _) = v1;
                h1349
            })?, {
                let (_, h1350) = v1;
                h1350
            }), f278(v2)?))
        }
    }
}

pub fn f279(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1351 = v0;
    match m1351 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f325(v1)
        }
    }
}

pub fn f280(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1352 = v0;
    match m1352.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f185(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f195(f326(f76(List::cons(v1, f327(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f280(v3, v1)?))
            }
        }
    }
}

pub fn f281(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1353 = 0u64;
        let a1354 = v1;
        nat_lt(a1353, a1354)
    } else {
        false
    } {
        let a1355 = Bytes::lit(&[0x2d]);
        let a1356 = f328(v1)?;
        Ok(append_bytes(a1355, a1356))
    } else {
        f328(v1)
    }
}

pub fn f282(v0: Bytes) -> R<Bytes> {
    let a1362 = {
        let a1360 = Bytes::lit(&[0x22]);
        let a1361 = {
            let (_, h1357) = {
                let (h1358, _) = f329(Fn9::F330(v0.clone()), {
                    let a1359 = v0;
                    length_bytes(a1359)
                }, (0u64, Bytes::lit(&[])))?;
                h1358
            };
            h1357
        };
        append_bytes(a1360, a1361)
    };
    let a1363 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1362, a1363))
}

pub fn f283(v0: List<Bytes>) -> Bytes {
    let a1366 = {
        let a1364 = Bytes::lit(&[0x5b]);
        let a1365 = f117(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1364, a1365)
    };
    let a1367 = Bytes::lit(&[0x5d]);
    append_bytes(a1366, a1367)
}

pub fn f284(v0: Adt14) -> R<List<Bytes>> {
    let m1368 = v0;
    match m1368 {
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
        Adt14::C6(r1369, r1370) => {
            let v7 = (*r1369).clone();
            let v8 = (*r1370).clone();
            Ok(List::cons(f214(v7)?, f284(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f285(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f331(Fn10::F332, List::<(Bytes, Bytes)>::nil(), f333(v0)?);
    let v2: List<Bytes> = f334(Fn11::F335, List::<Bytes>::nil(), v1)?;
    let a1373 = {
        let a1371 = Bytes::lit(&[0x7b]);
        let a1372 = f117(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1371, a1372)
    };
    let a1374 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1373, a1374))
}

pub fn f286(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1375 = v0;
    match m1375 {
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
        Adt14::C7(v9, r1376, r1377) => {
            let v10 = (*r1376).clone();
            let v11 = (*r1377).clone();
            Ok(List::cons((v9, f214(v10)?), f286(v11)?))
        }
    }
}

pub fn f287(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1378 = v0;
    match m1378 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f139(v2, v1)
        }
    }
}

pub fn f288(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f195(f326(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f289(v0: Adt16, v1: Option<u64>) -> bool {
    let m1379 = v1;
    match m1379 {
        None => {
            true
        }
        Some(v2) => {
            f231(v2, match v0 {
                Adt16::C0(_, h1380, _, _) => {
                    h1380
                }
            })
        }
    }
}

pub fn f290(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a1382 = match v0 {
            Adt12::C0(_, _, h1381, _, _) => {
                h1381
            }
        };
        length_list(a1382)
    })
}

pub fn f291(v0: Bytes) -> Option<Adt20> {
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

pub fn f292(v0: Bytes, p1387: List<Bytes>, p1388: u64) -> R<Option<u64>> {
    let m1386 = (p1387, p1388);
    {
        let mut m1386 = m1386;
        loop {
            let step = {
                let (v1, v2) = m1386;
                let m1383 = v1;
                match m1383.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1384 = v2;
                                let a1385 = 1u64;
                                nat_add(a1384, a1385)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f293(v0: Option<Adt14>) -> Option<u64> {
    let m1389 = v0;
    match m1389 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1390 = v1;
            match m1390 {
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

pub fn f294(v0: Fn5, p1394: List<Bytes>, p1395: List<(Bytes, ())>) -> List<Bytes> {
    let m1393 = (p1394, p1395);
    {
        let mut m1393 = m1393;
        loop {
            let step = {
                let (v1, v2) = m1393;
                let m1391 = v2;
                match m1391.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1392: Fn5 = v0.clone();
                            c1392.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1393 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1396, _) = v1;
        h1396
    }, v0)
}

pub fn f296(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f336(Fn12::F337, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f297(v0: Fn6, p1400: List<List<(Bytes, ())>>, p1401: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1399 = (p1400, p1401);
    {
        let mut m1399 = m1399;
        loop {
            let step = {
                let (v1, v2) = m1399;
                let m1397 = v2;
                match m1397.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1398: Fn6 = v0.clone();
                            c1398.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f299(v0: Fn7, p1406: u64, p1407: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1405 = (p1406, p1407);
    {
        let mut m1405 = m1405;
        loop {
            let step = {
                let (v1, v2) = m1405;
                let m1402 = v1;
                if m1402 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1402 - 1;
                    let m1404 = {
                        let c1403: Fn7 = v0.clone();
                        c1403.apply(v2.clone())?
                    };
                    match m1404 {
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
                Ok(next) => { m1405 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f338(v0)
}

pub fn f301(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f339(Fn13::F340, List::<(Bytes, ())>::nil(), v0)
}

pub fn f302(v0: Fn8, p1411: List<(Bytes, ())>, p1412: List<Bytes>) -> List<(Bytes, ())> {
    let m1410 = (p1411, p1412);
    {
        let mut m1410 = m1410;
        loop {
            let step = {
                let (v1, v2) = m1410;
                let m1408 = v2;
                match m1408.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1409: Fn8 = v0.clone();
                            c1409.apply(v1, v3)
                        }, v4))
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

pub fn f303(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f304(v0: Bytes) -> Bytes {
    if f125(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1414 = {
                let a1413 = v0;
                length_bytes(a1413)
            };
            let a1415 = 1u64;
            nat_sub(a1414, a1415)
        })
    } else {
        v0
    }
}

pub fn f305(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f81(f68(v0.clone(), v1.clone()))?;
    let m1416 = f141(v2.clone())?;
    match m1416 {
        None => {
            f342(v0, v1, v2)
        }
        Some(v3) => {
            f341(v0, v1, v3)
        }
    }
}

pub fn f306(v0: List<Bytes>) -> List<Bytes> {
    let m1417 = v0;
    match m1417.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1419 = {
                    let a1418 = v1.clone();
                    length_bytes(a1418)
                };
                let a1420 = 0u64;
                if nat_eq(a1419, a1420) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f306(v2))
        }
    }
}

pub fn f307(v0: List<Bytes>) -> Bytes {
    let m1421 = v0;
    match m1421.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f308(v0: (Bytes, Bytes), p1424: List<(Bytes, Bytes)>) -> bool {
    let m1423 = p1424;
    {
        let mut m1423 = m1423;
        loop {
            let step = {
                let v1 = m1423;
                let m1422 = v1;
                match m1422.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f343(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f309(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1425 = v1;
    match m1425.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1426, _) = v2;
                h1426
            }, v0.clone()) {
                List::cons({
                    let (_, h1427) = v2;
                    h1427
                }, f309(v0, v3))
            } else {
                f309(v0, v3)
            }
        }
    }
}

pub fn f310(v0: u32) -> Bytes {
    let a1430 = {
        let a1428 = v0;
        let a1429 = 8u32;
        fixed_u32::bit_and(a1428, a1429)
    };
    let a1431 = 0u32;
    if equal(a1430, a1431) {
        let a1434 = {
            let a1432 = v0;
            let a1433 = 4u32;
            fixed_u32::bit_and(a1432, a1433)
        };
        let a1435 = 0u32;
        if equal(a1434, a1435) {
            let a1438 = {
                let a1436 = v0;
                let a1437 = 2u32;
                fixed_u32::bit_and(a1436, a1437)
            };
            let a1439 = 0u32;
            if equal(a1438, a1439) {
                let a1442 = {
                    let a1440 = v0;
                    let a1441 = 1u32;
                    fixed_u32::bit_and(a1440, a1441)
                };
                let a1443 = 0u32;
                if equal(a1442, a1443) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1446 = {
                    let a1444 = v0;
                    let a1445 = 1u32;
                    fixed_u32::bit_and(a1444, a1445)
                };
                let a1447 = 0u32;
                if equal(a1446, a1447) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1450 = {
                let a1448 = v0;
                let a1449 = 2u32;
                fixed_u32::bit_and(a1448, a1449)
            };
            let a1451 = 0u32;
            if equal(a1450, a1451) {
                let a1454 = {
                    let a1452 = v0;
                    let a1453 = 1u32;
                    fixed_u32::bit_and(a1452, a1453)
                };
                let a1455 = 0u32;
                if equal(a1454, a1455) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1458 = {
                    let a1456 = v0;
                    let a1457 = 1u32;
                    fixed_u32::bit_and(a1456, a1457)
                };
                let a1459 = 0u32;
                if equal(a1458, a1459) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1462 = {
            let a1460 = v0;
            let a1461 = 4u32;
            fixed_u32::bit_and(a1460, a1461)
        };
        let a1463 = 0u32;
        if equal(a1462, a1463) {
            let a1466 = {
                let a1464 = v0;
                let a1465 = 2u32;
                fixed_u32::bit_and(a1464, a1465)
            };
            let a1467 = 0u32;
            if equal(a1466, a1467) {
                let a1470 = {
                    let a1468 = v0;
                    let a1469 = 1u32;
                    fixed_u32::bit_and(a1468, a1469)
                };
                let a1471 = 0u32;
                if equal(a1470, a1471) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1474 = {
                    let a1472 = v0;
                    let a1473 = 1u32;
                    fixed_u32::bit_and(a1472, a1473)
                };
                let a1475 = 0u32;
                if equal(a1474, a1475) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1478 = {
                let a1476 = v0;
                let a1477 = 2u32;
                fixed_u32::bit_and(a1476, a1477)
            };
            let a1479 = 0u32;
            if equal(a1478, a1479) {
                let a1482 = {
                    let a1480 = v0;
                    let a1481 = 1u32;
                    fixed_u32::bit_and(a1480, a1481)
                };
                let a1483 = 0u32;
                if equal(a1482, a1483) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1486 = {
                    let a1484 = v0;
                    let a1485 = 1u32;
                    fixed_u32::bit_and(a1484, a1485)
                };
                let a1487 = 0u32;
                if equal(a1486, a1487) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f311(v0: u32, v1: u32) -> u32 {
    let m1490 = {
        let a1488 = v0;
        let a1489 = v1;
        fixed_u32::shift_right(a1488, a1489)
    };
    match m1490 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f312(v0: Adt22, v1: Adt24) -> R<Adt22> {
    let v2: Adt25 = f344(Fn14::F345, 64u64, Adt25::C0(match v0.clone() {
        Adt22::C0(h1491, _, _, _, _, _, _, _) => {
            h1491
        }
    }, match v0.clone() {
        Adt22::C0(_, h1492, _, _, _, _, _, _) => {
            h1492
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h1493, _, _, _, _, _) => {
            h1493
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h1494, _, _, _, _) => {
            h1494
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h1495, _, _, _) => {
            h1495
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1496, _, _) => {
            h1496
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1497, _) => {
            h1497
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h1498) => {
            h1498
        }
    }, v1, 0u64))?;
    Ok(Adt22::C0(f346(match v0.clone() {
        Adt22::C0(h1499, _, _, _, _, _, _, _) => {
            h1499
        }
    }, match v2.clone() {
        Adt25::C0(h1500, _, _, _, _, _, _, _, _, _) => {
            h1500
        }
    }), f346(match v0.clone() {
        Adt22::C0(_, h1501, _, _, _, _, _, _) => {
            h1501
        }
    }, match v2.clone() {
        Adt25::C0(_, h1502, _, _, _, _, _, _, _, _) => {
            h1502
        }
    }), f346(match v0.clone() {
        Adt22::C0(_, _, h1503, _, _, _, _, _) => {
            h1503
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h1504, _, _, _, _, _, _, _) => {
            h1504
        }
    }), f346(match v0.clone() {
        Adt22::C0(_, _, _, h1505, _, _, _, _) => {
            h1505
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h1506, _, _, _, _, _, _) => {
            h1506
        }
    }), f346(match v0.clone() {
        Adt22::C0(_, _, _, _, h1507, _, _, _) => {
            h1507
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h1508, _, _, _, _, _) => {
            h1508
        }
    }), f346(match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1509, _, _) => {
            h1509
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h1510, _, _, _, _) => {
            h1510
        }
    }), f346(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1511, _) => {
            h1511
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h1512, _, _, _) => {
            h1512
        }
    }), f346(match v0 {
        Adt22::C0(_, _, _, _, _, _, _, h1513) => {
            h1513
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h1514, _, _) => {
            h1514
        }
    })))
}

pub fn f313(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f347(v0.clone(), v1, v2, v3)?, f347(v0.clone(), v1, v2, {
        let a1515 = v3;
        let a1516 = 4u64;
        nat_add(a1515, a1516)?
    })?, f347(v0.clone(), v1, v2, {
        let a1517 = v3;
        let a1518 = 8u64;
        nat_add(a1517, a1518)?
    })?, f347(v0.clone(), v1, v2, {
        let a1519 = v3;
        let a1520 = 12u64;
        nat_add(a1519, a1520)?
    })?, f347(v0.clone(), v1, v2, {
        let a1521 = v3;
        let a1522 = 16u64;
        nat_add(a1521, a1522)?
    })?, f347(v0.clone(), v1, v2, {
        let a1523 = v3;
        let a1524 = 20u64;
        nat_add(a1523, a1524)?
    })?, f347(v0.clone(), v1, v2, {
        let a1525 = v3;
        let a1526 = 24u64;
        nat_add(a1525, a1526)?
    })?, f347(v0.clone(), v1, v2, {
        let a1527 = v3;
        let a1528 = 28u64;
        nat_add(a1527, a1528)?
    })?, f347(v0.clone(), v1, v2, {
        let a1529 = v3;
        let a1530 = 32u64;
        nat_add(a1529, a1530)?
    })?, f347(v0.clone(), v1, v2, {
        let a1531 = v3;
        let a1532 = 36u64;
        nat_add(a1531, a1532)?
    })?, f347(v0.clone(), v1, v2, {
        let a1533 = v3;
        let a1534 = 40u64;
        nat_add(a1533, a1534)?
    })?, f347(v0.clone(), v1, v2, {
        let a1535 = v3;
        let a1536 = 44u64;
        nat_add(a1535, a1536)?
    })?, f347(v0.clone(), v1, v2, {
        let a1537 = v3;
        let a1538 = 48u64;
        nat_add(a1537, a1538)?
    })?, f347(v0.clone(), v1, v2, {
        let a1539 = v3;
        let a1540 = 52u64;
        nat_add(a1539, a1540)?
    })?, f347(v0.clone(), v1, v2, {
        let a1541 = v3;
        let a1542 = 56u64;
        nat_add(a1541, a1542)?
    })?, f347(v0, v1, v2, {
        let a1543 = v3;
        let a1544 = 60u64;
        nat_add(a1543, a1544)?
    })?))
}

pub fn f314(v0: Bytes, v1: u64) -> Option<u64> {
    let a1545 = 0u64;
    let a1546 = v1;
    if nat_lt(a1545, a1546) {
        if f348(v0, {
            let a1547 = v1;
            let a1548 = 1u64;
            nat_sub(a1547, a1548)
        }) {
            Some({
                let a1549 = v1;
                let a1550 = 1u64;
                nat_sub(a1549, a1550)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f315(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1551 = v2;
    let a1552 = v0;
    if nat_lt(a1551, a1552) {
        if f348(v1, v2) {
            Ok(Some({
                let a1553 = v2;
                let a1554 = 1u64;
                nat_add(a1553, a1554)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f316(v0: Bytes, p1634: u64, p1635: u64, p1636: bool, p1637: bool, p1638: Bytes) -> R<Bytes> {
    let m1633 = (p1634, (p1635, (p1636, (p1637, p1638))));
    {
        let mut m1633 = m1633;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1633;
                let m1555 = v2;
                if m1555 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1555 - 1;
                    let m1558 = {
                        let a1556 = v0.clone();
                        let a1557 = v1;
                        index_bytes(a1556, a1557)
                    };
                    match m1558 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1603 = {
                                    let a1579 = {
                                        let a1567 = {
                                            let a1561 = {
                                                let a1559 = v8;
                                                let a1560 = 128u8;
                                                fixed_u8::bit_and(a1559, a1560)
                                            };
                                            let a1562 = 128u8;
                                            if equal(a1561, a1562) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1568 = {
                                            let a1565 = {
                                                let a1563 = v8;
                                                let a1564 = 64u8;
                                                fixed_u8::bit_and(a1563, a1564)
                                            };
                                            let a1566 = 64u8;
                                            if equal(a1565, a1566) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1567, a1568)?
                                    };
                                    let a1580 = {
                                        let a1577 = {
                                            let a1571 = {
                                                let a1569 = v8;
                                                let a1570 = 32u8;
                                                fixed_u8::bit_and(a1569, a1570)
                                            };
                                            let a1572 = 32u8;
                                            if equal(a1571, a1572) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1578 = {
                                            let a1575 = {
                                                let a1573 = v8;
                                                let a1574 = 16u8;
                                                fixed_u8::bit_and(a1573, a1574)
                                            };
                                            let a1576 = 16u8;
                                            if equal(a1575, a1576) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1577, a1578)?
                                    };
                                    nat_add(a1579, a1580)?
                                };
                                let a1604 = {
                                    let a1601 = {
                                        let a1589 = {
                                            let a1583 = {
                                                let a1581 = v8;
                                                let a1582 = 8u8;
                                                fixed_u8::bit_and(a1581, a1582)
                                            };
                                            let a1584 = 8u8;
                                            if equal(a1583, a1584) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1590 = {
                                            let a1587 = {
                                                let a1585 = v8;
                                                let a1586 = 4u8;
                                                fixed_u8::bit_and(a1585, a1586)
                                            };
                                            let a1588 = 4u8;
                                            if equal(a1587, a1588) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1589, a1590)?
                                    };
                                    let a1602 = {
                                        let a1599 = {
                                            let a1593 = {
                                                let a1591 = v8;
                                                let a1592 = 2u8;
                                                fixed_u8::bit_and(a1591, a1592)
                                            };
                                            let a1594 = 2u8;
                                            if equal(a1593, a1594) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1600 = {
                                            let a1597 = {
                                                let a1595 = v8;
                                                let a1596 = 1u8;
                                                fixed_u8::bit_and(a1595, a1596)
                                            };
                                            let a1598 = 1u8;
                                            if equal(a1597, a1598) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1599, a1600)?
                                    };
                                    nat_add(a1601, a1602)?
                                };
                                nat_add(a1603, a1604)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1605 = v1;
                                    let a1606 = 1u64;
                                    nat_add(a1605, a1606)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1607 = v9;
                                    let a1608 = 34u64;
                                    !nat_eq(a1607, a1608)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1609 = v9;
                                    let a1610 = 92u64;
                                    nat_eq(a1609, a1610)
                                }, {
                                    let a1611 = v5;
                                    let a1612 = v10;
                                    append_bytes(a1611, a1612)
                                })))))
                            } else {
                                let a1613 = v9;
                                let a1614 = 32u64;
                                if if if if nat_eq(a1613, a1614) {
                                    true
                                } else {
                                    let a1615 = v9;
                                    let a1616 = 9u64;
                                    nat_eq(a1615, a1616)
                                } {
                                    true
                                } else {
                                    let a1617 = v9;
                                    let a1618 = 10u64;
                                    nat_eq(a1617, a1618)
                                } {
                                    true
                                } else {
                                    let a1619 = v9;
                                    let a1620 = 13u64;
                                    nat_eq(a1619, a1620)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1621 = v1;
                                        let a1622 = 1u64;
                                        nat_add(a1621, a1622)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1623 = v1;
                                            let a1624 = 4u64;
                                            nat_add(a1623, a1624)?
                                        }, (v6, (false, (false, {
                                            let a1625 = v5;
                                            let a1626 = f8(Str::lit("[]"));
                                            append_bytes(a1625, a1626)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1627 = v1;
                                            let a1628 = 1u64;
                                            nat_add(a1627, a1628)?
                                        }, (v6, ({
                                            let a1629 = v9;
                                            let a1630 = 34u64;
                                            nat_eq(a1629, a1630)
                                        }, (false, {
                                            let a1631 = v5;
                                            let a1632 = v10;
                                            append_bytes(a1631, a1632)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1633 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: u64) -> R<u64> {
    let m1641 = {
        let a1639 = v0;
        let a1640 = v1;
        index_bytes(a1639, a1640)
    };
    match m1641 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1686 = {
                let a1662 = {
                    let a1650 = {
                        let a1644 = {
                            let a1642 = v3;
                            let a1643 = 128u8;
                            fixed_u8::bit_and(a1642, a1643)
                        };
                        let a1645 = 128u8;
                        if equal(a1644, a1645) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1651 = {
                        let a1648 = {
                            let a1646 = v3;
                            let a1647 = 64u8;
                            fixed_u8::bit_and(a1646, a1647)
                        };
                        let a1649 = 64u8;
                        if equal(a1648, a1649) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1650, a1651)?
                };
                let a1663 = {
                    let a1660 = {
                        let a1654 = {
                            let a1652 = v3;
                            let a1653 = 32u8;
                            fixed_u8::bit_and(a1652, a1653)
                        };
                        let a1655 = 32u8;
                        if equal(a1654, a1655) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1661 = {
                        let a1658 = {
                            let a1656 = v3;
                            let a1657 = 16u8;
                            fixed_u8::bit_and(a1656, a1657)
                        };
                        let a1659 = 16u8;
                        if equal(a1658, a1659) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1660, a1661)?
                };
                nat_add(a1662, a1663)?
            };
            let a1687 = {
                let a1684 = {
                    let a1672 = {
                        let a1666 = {
                            let a1664 = v3;
                            let a1665 = 8u8;
                            fixed_u8::bit_and(a1664, a1665)
                        };
                        let a1667 = 8u8;
                        if equal(a1666, a1667) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1673 = {
                        let a1670 = {
                            let a1668 = v3;
                            let a1669 = 4u8;
                            fixed_u8::bit_and(a1668, a1669)
                        };
                        let a1671 = 4u8;
                        if equal(a1670, a1671) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1672, a1673)?
                };
                let a1685 = {
                    let a1682 = {
                        let a1676 = {
                            let a1674 = v3;
                            let a1675 = 2u8;
                            fixed_u8::bit_and(a1674, a1675)
                        };
                        let a1677 = 2u8;
                        if equal(a1676, a1677) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1683 = {
                        let a1680 = {
                            let a1678 = v3;
                            let a1679 = 1u8;
                            fixed_u8::bit_and(a1678, a1679)
                        };
                        let a1681 = 1u8;
                        if equal(a1680, a1681) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1682, a1683)?
                };
                nat_add(a1684, a1685)?
            };
            nat_add(a1686, a1687)
        }
    }
}

pub fn f318(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f349(Fn15::F350(v0.clone()), {
        let a1689 = {
            let a1688 = v0;
            length_bytes(a1688)
        };
        let a1690 = 1u64;
        nat_add(a1689, a1690)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1694 = {
        let (_, h1691) = {
            let (_, h1692) = {
                let (h1693, _) = v2.clone();
                h1693
            };
            h1692
        };
        h1691
    };
    let a1695 = 1u64;
    if nat_eq(a1694, a1695) {
        Ok(Some(({
            let (h1696, _) = {
                let (h1697, _) = v2.clone();
                h1697
            };
            h1696
        }, {
            let (h1698, _) = {
                let (_, h1699) = {
                    let (h1700, _) = v2;
                    h1700
                };
                h1699
            };
            h1698
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1701 = f317(v0.clone(), v1)?;
        let a1702 = 45u64;
        nat_eq(a1701, a1702)
    };
    let v3: u64 = if v2 {
        let a1703 = v1;
        let a1704 = 1u64;
        nat_add(a1703, a1704)?
    } else {
        v1
    };
    let v4: u64 = f351(v0.clone(), v3)?;
    let a1707 = 19u64;
    let a1708 = {
        let a1705 = v4;
        let a1706 = v3;
        nat_sub(a1705, a1706)
    };
    if nat_lt(a1707, a1708) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1711 = f352(f32(v0, v3, {
            let a1709 = v4;
            let a1710 = v3;
            nat_sub(a1709, a1710)
        }))?;
        match m1711 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f353(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f320(v0: u64) -> bool {
    let a1712 = v0;
    let a1713 = 95u64;
    if nat_eq(a1712, a1713) {
        true
    } else {
        let a1714 = v0;
        let a1715 = 45u64;
        if nat_eq(a1714, a1715) {
            true
        } else {
            let a1716 = v0;
            let a1717 = 32u64;
            if nat_eq(a1716, a1717) {
                true
            } else {
                let a1718 = v0;
                let a1719 = 46u64;
                if nat_eq(a1718, a1719) {
                    true
                } else {
                    let a1720 = v0;
                    let a1721 = 47u64;
                    nat_eq(a1720, a1721)
                }
            }
        }
    }
}

pub fn f321(v0: u64) -> bool {
    let a1722 = 97u64;
    let a1723 = v0;
    if nat_le(a1722, a1723) {
        let a1724 = v0;
        let a1725 = 122u64;
        nat_le(a1724, a1725)
    } else {
        false
    }
}

pub fn f322(v0: u64) -> bool {
    let a1726 = 65u64;
    let a1727 = v0;
    if nat_le(a1726, a1727) {
        let a1728 = v0;
        let a1729 = 90u64;
        nat_le(a1728, a1729)
    } else {
        false
    }
}

pub fn f323(v0: u64) -> bool {
    let a1730 = 48u64;
    let a1731 = v0;
    if nat_le(a1730, a1731) {
        let a1732 = v0;
        let a1733 = 57u64;
        nat_le(a1732, a1733)
    } else {
        false
    }
}

pub fn f324(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m1734 = f136(v1.clone(), f8(Str::lit("enum")));
    match m1734 {
        None => {
            let m1736 = f357(v0.clone(), f279(f136(v1.clone(), f8(Str::lit("properties")))), f72(f140(f136(v1, f8(Str::lit("required"))))))?;
            match m1736 {
                Ok(v6) => {
                    Ok(f356(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m1738 = {
                            let (h1737, _) = v6.clone();
                            h1737
                        };
                        match m1738.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f195({
                                    let (h1739, _) = v6;
                                    h1739
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1740) = v6;
                        h1740
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt23>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m1735 = f354(f215(v2), List::<Bytes>::nil())?;
            match m1735 {
                Ok(v4) => {
                    Ok(f356(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f195(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt23>::nil()))
                }
                Err(v3) => {
                    Ok(f355(v3, v0))
                }
            }
        }
    }
}

pub fn f325(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1741 = v0;
    match m1741 {
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
        Adt14::C4(r1742) => {
            let v4 = (*r1742).clone();
            f325(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1743, r1744) => {
            let v2 = (*r1743).clone();
            let v3 = (*r1744).clone();
            List::cons((v1, v2), f325(v3))
        }
    }
}

pub fn f326(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1745 = v0;
    match m1745.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f190(v1)?, f326(v2)?))
        }
    }
}

pub fn f327(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1746 = v0;
    match m1746.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f327(v3, false))
            } else {
                f327(v3, true)
            }
        }
    }
}

pub fn f328(v0: u64) -> R<Bytes> {
    f358(f359(), 10u64, v0)
}

pub fn f329(v0: Fn9, p1751: u64, p1752: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1750 = (p1751, p1752);
    {
        let mut m1750 = m1750;
        loop {
            let step = {
                let (v1, v2) = m1750;
                let m1747 = v1;
                if m1747 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1747 - 1;
                    let m1749 = {
                        let c1748: Fn9 = v0.clone();
                        c1748.apply(v2.clone())?
                    };
                    match m1749 {
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
                Ok(next) => { m1750 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f360(v0, v1)
}

pub fn f331(v0: Fn10, p1756: List<(Bytes, Bytes)>, p1757: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1755 = (p1756, p1757);
    {
        let mut m1755 = m1755;
        loop {
            let step = {
                let (v1, v2) = m1755;
                let m1753 = v2;
                match m1753.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1754: Fn10 = v0.clone();
                            c1754.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f361(v0, v1)
}

pub fn f333(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f362(v0)
}

pub fn f334(v0: Fn11, p1761: List<Bytes>, p1762: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1760 = (p1761, p1762);
    {
        let mut m1760 = m1760;
        loop {
            let step = {
                let (v1, v2) = m1760;
                let m1758 = v2;
                match m1758.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1759: Fn11 = v0.clone();
                            c1759.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1760 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f363(v1)?, v0))
}

pub fn f336(v0: Fn12, p1766: List<List<(Bytes, ())>>, p1767: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1765 = (p1766, p1767);
    {
        let mut m1765 = m1765;
        loop {
            let step = {
                let (v1, v2) = m1765;
                let m1763 = v2;
                match m1763.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1764: Fn12 = v0.clone();
                            c1764.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1765 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f338(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1769 = {
        let a1768 = v0.clone();
        length_list(a1768)
    };
    let a1770 = 1u64;
    if nat_le(a1769, a1770) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f364(v0)?))
    }
}

pub fn f339(v0: Fn13, p1774: List<(Bytes, ())>, p1775: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1773 = (p1774, p1775);
    {
        let mut m1773 = m1773;
        loop {
            let step = {
                let (v1, v2) = m1773;
                let m1771 = v2;
                match m1771.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1772: Fn13 = v0.clone();
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

pub fn f340(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f341(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f70(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f365(v3.clone(), f70(f71(v0, f87())?, f8(Str::lit("at"))));
    let a1777 = {
        let a1776 = v4.clone();
        length_list(a1776)
    };
    let a1778 = 0u64;
    if nat_eq(a1777, a1778) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1781 = {
            let a1779 = v4;
            length_list(a1779)
        };
        let a1782 = {
            let a1780 = v3;
            length_list(a1780)
        };
        if nat_eq(a1781, a1782) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f366(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f342(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f205(f131(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f205(f131(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f62(f70(v3.clone(), f8(Str::lit("id"))), f70(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f70(f71(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f365(v5.clone(), f62(v6.clone(), f70(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1784 = {
        let a1783 = v7.clone();
        length_list(a1783)
    };
    let a1785 = 0u64;
    if nat_eq(a1784, a1785) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f73({
            let a1788 = {
                let a1786 = v7;
                length_list(a1786)
            };
            let a1789 = {
                let a1787 = v5;
                length_list(a1787)
            };
            nat_eq(a1788, a1789)
        }) {
            Ok((false, f366(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f76(f62(f365(f367(v4.clone()), f67(f68(v0, f8(Str::lit("model/authorities.toml"))))?), f365(f368(v4), f62(v6, f70(v3, f8(Str::lit("id")))))))?;
            let a1791 = {
                let a1790 = v8.clone();
                length_list(a1790)
            };
            let a1792 = 0u64;
            if nat_eq(a1791, a1792) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f366(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f117(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f343(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h1793, _) = v0;
        h1793
    }, {
        let (h1794, _) = v1;
        h1794
    }) {
        f36({
            let (_, h1795) = v0;
            h1795
        }, {
            let (_, h1796) = v1;
            h1796
        })
    } else {
        false
    }
}

pub fn f344(v0: Fn14, p1800: u64, p1801: Adt25) -> R<Adt25> {
    let m1799 = (p1800, p1801);
    {
        let mut m1799 = m1799;
        loop {
            let step = {
                let (v1, v2) = m1799;
                let m1797 = v1;
                if m1797 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m1797 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c1798: Fn14 = v0.clone();
                        c1798.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1799 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: Adt25) -> R<Adt25> {
    f369(v0)
}

pub fn f346(v0: u32, v1: u32) -> u32 {
    let m1804 = {
        let a1802 = f370(v0);
        let a1803 = f370(v1);
        fixed_u64::checked_add(a1802, a1803)
    };
    match m1804 {
        None => {
            0u32
        }
        Some(v2) => {
            f371(v2)
        }
    }
}

pub fn f347(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1807 = {
        let a1805 = v3;
        let a1806 = 8u64;
        nat_add(a1805, a1806)?
    };
    let a1808 = v2;
    if nat_eq(a1807, a1808) {
        Ok(f372({
            let a1811 = {
                let a1809 = v1;
                let a1810 = 8u64;
                nat_mul(a1809, a1810)?
            };
            let a1812 = 4294967296u64;
            let a1813 = 0u64;
            nat_quot(a1811, a1812, a1813)
        }))
    } else {
        let a1816 = {
            let a1814 = v3;
            let a1815 = 4u64;
            nat_add(a1814, a1815)?
        };
        let a1817 = v2;
        if nat_eq(a1816, a1817) {
            Ok(f372({
                let a1820 = {
                    let a1818 = v1;
                    let a1819 = 8u64;
                    nat_mul(a1818, a1819)?
                };
                let a1821 = 4294967296u64;
                let a1822 = 0u64;
                nat_rem(a1820, a1821, a1822)
            }))
        } else {
            Ok(f373(f374(v0.clone(), v1, v3), f374(v0.clone(), v1, {
                let a1823 = v3;
                let a1824 = 1u64;
                nat_add(a1823, a1824)?
            }), f374(v0.clone(), v1, {
                let a1825 = v3;
                let a1826 = 2u64;
                nat_add(a1825, a1826)?
            }), f374(v0, v1, {
                let a1827 = v3;
                let a1828 = 3u64;
                nat_add(a1827, a1828)?
            })))
        }
    }
}

pub fn f348(v0: Bytes, v1: u64) -> bool {
    let m1829 = f219(v0, v1);
    match m1829 {
        None => {
            false
        }
        Some(v2) => {
            f375(v2)
        }
    }
}

pub fn f349(v0: Fn15, p1834: u64, p1835: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1833 = (p1834, p1835);
    {
        let mut m1833 = m1833;
        loop {
            let step = {
                let (v1, v2) = m1833;
                let m1830 = v1;
                if m1830 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1830 - 1;
                    let m1832 = {
                        let c1831: Fn15 = v0.clone();
                        c1831.apply(v2.clone())?
                    };
                    match m1832 {
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
                Ok(next) => { m1833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f376(v0, v1)
}

pub fn f351(v0: Bytes, v1: u64) -> R<u64> {
    let (h1836, _) = f221(Fn3::F377(v0.clone()), {
        let a1838 = {
            let a1837 = v0;
            length_bytes(a1837)
        };
        let a1839 = 1u64;
        nat_add(a1838, a1839)?
    }, v1)?;
    Ok(h1836)
}

pub fn f352(v0: Bytes) -> R<Option<u64>> {
    if f378(v0.clone())? {
        Ok(Some({
            let (_, h1840) = {
                let (h1841, _) = f379(Fn16::F380(v0.clone()), {
                    let a1842 = v0;
                    length_bytes(a1842)
                }, (0u64, 0u64))?;
                h1841
            };
            h1840
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f353(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1843 = 0u64;
        let a1844 = v1;
        if nat_lt(a1843, a1844) {
            let a1845 = v1;
            let a1846 = 9223372036854775808u64;
            nat_le(a1845, a1846)
        } else {
            false
        }
    } else {
        let a1847 = v1;
        let a1848 = 9223372036854775807u64;
        nat_le(a1847, a1848)
    }
}

pub fn f354(v0: List<Adt14>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1849 = v0;
    match m1849.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v2, v3)) => {
            let m1850 = f381(v2.clone())?;
            match m1850 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f382(v2)?, List::<Bytes>::nil())))))
                }
                Some(v4) => {
                    if f84(v4.clone(), v1.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v4, List::cons(f8(Str::lit(" (a string is named in camel case, an integer n is named nN, and a negative one nMinusN)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1851 = f354(v3, List::cons(v4.clone(), v1))?;
                        match m1851 {
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

pub fn f355(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt23>), Bytes> {
    Err::<(Bytes, List<Adt23>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f356(v0: Bytes, v1: List<Adt23>) -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((v0, v1))
}

pub fn f357(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m1852 = v1;
    match m1852.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f383({
                let (h1853, _) = v3.clone();
                h1853
            })?;
            if f384(v5.clone())? {
                let m1858 = f385({
                    let (_, h1854) = v3.clone();
                    h1854
                }, {
                    let a1856 = v0.clone();
                    let a1857 = f209({
                        let (h1855, _) = v3.clone();
                        h1855
                    })?;
                    append_bytes(a1856, a1857)
                })?;
                match m1858 {
                    Ok(v7) => {
                        let v8: Bytes = if f84({
                            let (h1859, _) = v3;
                            h1859
                        }, v2.clone()) {
                            let (h1860, _) = v7;
                            h1860
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1861, _) = v7;
                                h1861
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1862 = f357(v0, v4, v2)?;
                        match m1862 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1863, _) = v10;
                                    h1863
                                }), f277({
                                    let (_, h1864) = v7;
                                    h1864
                                }, {
                                    let (_, h1865) = v10;
                                    h1865
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
                    let (h1866, _) = v3;
                    h1866
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f358(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1867 = v2;
    let a1868 = 0u64;
    if nat_eq(a1867, a1868) {
        Ok(f386(v0, 0u64))
    } else {
        let (_, h1869) = {
            let (h1870, _) = f329(Fn9::F387(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1870
        };
        Ok(h1869)
    }
}

pub fn f359() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f360(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1874 = {
        let a1872 = v0.clone();
        let a1873 = {
            let (h1871, _) = v1.clone();
            h1871
        };
        index_bytes(a1872, a1873)
    };
    match m1874 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1876 = {
                    let (h1875, _) = v1.clone();
                    h1875
                };
                let a1877 = 1u64;
                nat_add(a1876, a1877)?
            }, {
                let a1880 = {
                    let (_, h1878) = v1;
                    h1878
                };
                let a1881 = f388(v0, {
                    let (h1879, _) = v1;
                    h1879
                }, v2)?;
                append_bytes(a1880, a1881)
            })))
        }
    }
}

pub fn f361(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1882 = v0.clone();
    match m1882.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1885 = {
                let (h1883, _) = v2;
                h1883
            };
            let a1886 = {
                let (h1884, _) = v1.clone();
                h1884
            };
            if equal(a1885, a1886) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f362(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f389(f390(Fn17::F391, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1891 = {
        let (h1887, _) = f392(Fn18::F393, {
            let a1889 = {
                let a1888 = v0;
                length_list(a1888)
            };
            let a1890 = 1u64;
            nat_add(a1889, a1890)?
        }, v1)?;
        h1887
    };
    match m1891.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f363(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1896 = {
        let a1893 = f282({
            let (h1892, _) = v0;
            h1892
        })?;
        let a1894 = Bytes::lit(&[0x3a]);
        append_bytes(a1893, a1894)
    };
    let a1897 = {
        let (_, h1895) = v0;
        h1895
    };
    Ok(append_bytes(a1896, a1897))
}

pub fn f364(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f296({
        let (_, h1898) = {
            let (h1899, _) = f394(Fn19::F395, {
                let a1901 = {
                    let a1900 = v0.clone();
                    length_list(a1900)
                };
                let a1902 = 1u64;
                nat_add(a1901, a1902)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1899
        };
        h1898
    }))
}

pub fn f365(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1903 = v0;
    match m1903.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f84(v2.clone(), v1.clone()) {
                f365(v3, v1)
            } else {
                List::cons(v2, f365(v3, v1))
            }
        }
    }
}

pub fn f366(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f367(v0: List<Adt14>) -> List<Bytes> {
    let m1904 = v0;
    match m1904.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1905 = f136(v1, f8(Str::lit("std")));
            match m1905 {
                None => {
                    f367(v2)
                }
                Some(v3) => {
                    List::cons(f139(v3, f8(Str::lit("authority"))), f367(v2))
                }
            }
        }
    }
}

pub fn f368(v0: List<Adt14>) -> List<Bytes> {
    let m1906 = v0;
    match m1906.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1907 = f144(f136(v1, f8(Str::lit("req"))));
            match m1907 {
                None => {
                    f368(v2)
                }
                Some(v3) => {
                    List::cons(v3, f368(v2))
                }
            }
        }
    }
}

pub fn f369(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f396(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h1908) => {
            h1908
        }
    });
    let v2: u32 = f346(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h1909, _, _) => {
            h1909
        }
    }, f346(f397(match v0.clone() {
        Adt25::C0(_, _, _, _, h1910, _, _, _, _, _) => {
            h1910
        }
    }), f346(f398(match v0.clone() {
        Adt25::C0(_, _, _, _, h1911, _, _, _, _, _) => {
            h1911
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1912, _, _, _, _) => {
            h1912
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1913, _, _, _) => {
            h1913
        }
    }), f346(v1, {
        let m1916 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h1915, _) => {
                h1915
            }
        };
        match m1916 {
            Adt24::C0(h1914, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1914
            }
        }
    }))));
    let v3: u32 = f346(f399(match v0.clone() {
        Adt25::C0(h1917, _, _, _, _, _, _, _, _, _) => {
            h1917
        }
    }), f400(match v0.clone() {
        Adt25::C0(h1918, _, _, _, _, _, _, _, _, _) => {
            h1918
        }
    }, match v0.clone() {
        Adt25::C0(_, h1919, _, _, _, _, _, _, _, _) => {
            h1919
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1920, _, _, _, _, _, _, _) => {
            h1920
        }
    }));
    Ok(Adt25::C0(f346(v2, v3), match v0.clone() {
        Adt25::C0(h1921, _, _, _, _, _, _, _, _, _) => {
            h1921
        }
    }, match v0.clone() {
        Adt25::C0(_, h1922, _, _, _, _, _, _, _, _) => {
            h1922
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1923, _, _, _, _, _, _, _) => {
            h1923
        }
    }, f346(match v0.clone() {
        Adt25::C0(_, _, _, h1924, _, _, _, _, _, _) => {
            h1924
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h1925, _, _, _, _, _) => {
            h1925
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1926, _, _, _, _) => {
            h1926
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1927, _, _, _) => {
            h1927
        }
    }, f401(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h1928, _) => {
            h1928
        }
    }), {
        let a1930 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h1929) => {
                h1929
            }
        };
        let a1931 = 1u64;
        nat_add(a1930, a1931)?
    }))
}

pub fn f370(v0: u32) -> u64 {
    let m1933 = {
        let a1932 = v0;
        fixed_u64::convert(i128::from(a1932))
    };
    match m1933 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f371(v0: u64) -> u32 {
    let m1937 = {
        let a1936 = {
            let a1934 = v0;
            let a1935 = 4294967295u64;
            fixed_u64::bit_and(a1934, a1935)
        };
        fixed_u32::convert(i128::from(a1936))
    };
    match m1937 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f372(v0: u64) -> u32 {
    let (_, h1938) = {
        let (_, h1939) = f402(Fn20::F403, 32u64, (v0, (1u32, 0u32)));
        h1939
    };
    h1938
}

pub fn f373(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1944 = f404(v0, 24u32);
    let a1945 = {
        let a1942 = f404(v1, 16u32);
        let a1943 = {
            let a1940 = f404(v2, 8u32);
            let a1941 = v3;
            fixed_u32::bit_or(a1940, a1941)
        };
        fixed_u32::bit_or(a1942, a1943)
    };
    fixed_u32::bit_or(a1944, a1945)
}

pub fn f374(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1946 = v2;
    let a1947 = v1;
    if nat_lt(a1946, a1947) {
        let m1950 = {
            let a1948 = v0;
            let a1949 = v2;
            index_bytes(a1948, a1949)
        };
        match m1950 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1952 = {
                    let a1951 = v3;
                    fixed_u32::convert(i128::from(a1951))
                };
                match m1952 {
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
        let a1953 = v2;
        let a1954 = v1;
        if nat_eq(a1953, a1954) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f375(v0: u8) -> bool {
    let a1955 = v0;
    let a1956 = 32u8;
    if if if equal(a1955, a1956) {
        true
    } else {
        let a1957 = v0;
        let a1958 = 9u8;
        equal(a1957, a1958)
    } {
        true
    } else {
        let a1959 = v0;
        let a1960 = 10u8;
        equal(a1959, a1960)
    } {
        true
    } else {
        let a1961 = v0;
        let a1962 = 12u8;
        if equal(a1961, a1962) {
            true
        } else {
            let a1963 = v0;
            let a1964 = 13u8;
            equal(a1963, a1964)
        }
    }
}

pub fn f376(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1965, _) = v1.clone();
        h1965
    };
    let v3: Bytes = {
        let (h1966, _) = {
            let (_, h1967) = v1.clone();
            h1967
        };
        h1966
    };
    let a1970 = {
        let (_, h1968) = {
            let (_, h1969) = v1;
            h1969
        };
        h1968
    };
    let a1971 = 0u64;
    if nat_eq(a1970, a1971) {
        let v4: u64 = f317(v0.clone(), v2)?;
        let a1972 = v4;
        let a1973 = 34u64;
        if nat_eq(a1972, a1973) {
            Ok(Some(({
                let a1974 = v2;
                let a1975 = 1u64;
                nat_add(a1974, a1975)?
            }, (v3, 1u64))))
        } else {
            let a1976 = v4;
            let a1977 = 92u64;
            if nat_eq(a1976, a1977) {
                f405(v0, v2, v3)
            } else {
                let a1978 = v4;
                let a1979 = 32u64;
                if if nat_lt(a1978, a1979) {
                    true
                } else {
                    let a1980 = v4;
                    let a1981 = 256u64;
                    nat_eq(a1980, a1981)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1982 = v2;
                        let a1983 = 1u64;
                        nat_add(a1982, a1983)?
                    }, ({
                        let a1984 = v3;
                        let a1985 = f32(v0, v2, 1u64);
                        append_bytes(a1984, a1985)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f377(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f406(v0, v1)
}

pub fn f378(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1986 = v0.clone();
        length_bytes(a1986)
    };
    let a1987 = 0u64;
    let a1988 = v1;
    if nat_lt(a1987, a1988) {
        let a1989 = f407(v0.clone())?;
        let a1990 = v1;
        if nat_eq(a1989, a1990) {
            let a1991 = 1u64;
            let a1992 = v1;
            if nat_lt(a1991, a1992) {
                let m1995 = {
                    let a1993 = v0;
                    let a1994 = 0u64;
                    index_bytes(a1993, a1994)
                };
                match m1995 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1996 = v2;
                        let a1997 = 48u8;
                        Ok(!equal(a1996, a1997))
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

pub fn f379(v0: Fn16, p2002: u64, p2003: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2001 = (p2002, p2003);
    {
        let mut m2001 = m2001;
        loop {
            let step = {
                let (v1, v2) = m2001;
                let m1998 = v1;
                if m1998 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1998 - 1;
                    let m2000 = {
                        let c1999: Fn16 = v0.clone();
                        c1999.apply(v2)?
                    };
                    match m2000 {
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
                Ok(next) => { m2001 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f408(v0, v1)
}

pub fn f381(v0: Adt14) -> R<Option<Bytes>> {
    let m2004 = v0;
    match m2004 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v3, v4) => {
            Ok(Some({
                let a2007 = if if v3 {
                    let a2005 = 0u64;
                    let a2006 = v4;
                    nat_lt(a2005, a2006)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2008 = f281(false, v4)?;
                append_bytes(a2007, a2008)
            }))
        }
        Adt14::C2(v1) => {
            let v2: Bytes = f383(v1)?;
            if f384(v2.clone())? {
                Ok(Some(v2))
            } else {
                Ok(None::<Bytes>)
            }
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

pub fn f382(v0: Adt14) -> R<Bytes> {
    f214(v0)
}

pub fn f383(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f275(v0.clone(), 0u64, {
        let a2009 = v0;
        length_bytes(a2009)
    }, false, Bytes::lit(&[]))?;
    let m2012 = {
        let a2010 = v1.clone();
        let a2011 = 0u64;
        index_bytes(a2010, a2011)
    };
    match m2012 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2057 = {
                    let a2033 = {
                        let a2021 = {
                            let a2015 = {
                                let a2013 = v3;
                                let a2014 = 128u8;
                                fixed_u8::bit_and(a2013, a2014)
                            };
                            let a2016 = 128u8;
                            if equal(a2015, a2016) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2022 = {
                            let a2019 = {
                                let a2017 = v3;
                                let a2018 = 64u8;
                                fixed_u8::bit_and(a2017, a2018)
                            };
                            let a2020 = 64u8;
                            if equal(a2019, a2020) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2021, a2022)?
                    };
                    let a2034 = {
                        let a2031 = {
                            let a2025 = {
                                let a2023 = v3;
                                let a2024 = 32u8;
                                fixed_u8::bit_and(a2023, a2024)
                            };
                            let a2026 = 32u8;
                            if equal(a2025, a2026) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2032 = {
                            let a2029 = {
                                let a2027 = v3;
                                let a2028 = 16u8;
                                fixed_u8::bit_and(a2027, a2028)
                            };
                            let a2030 = 16u8;
                            if equal(a2029, a2030) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2031, a2032)?
                    };
                    nat_add(a2033, a2034)?
                };
                let a2058 = {
                    let a2055 = {
                        let a2043 = {
                            let a2037 = {
                                let a2035 = v3;
                                let a2036 = 8u8;
                                fixed_u8::bit_and(a2035, a2036)
                            };
                            let a2038 = 8u8;
                            if equal(a2037, a2038) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2044 = {
                            let a2041 = {
                                let a2039 = v3;
                                let a2040 = 4u8;
                                fixed_u8::bit_and(a2039, a2040)
                            };
                            let a2042 = 4u8;
                            if equal(a2041, a2042) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2043, a2044)?
                    };
                    let a2056 = {
                        let a2053 = {
                            let a2047 = {
                                let a2045 = v3;
                                let a2046 = 2u8;
                                fixed_u8::bit_and(a2045, a2046)
                            };
                            let a2048 = 2u8;
                            if equal(a2047, a2048) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2054 = {
                            let a2051 = {
                                let a2049 = v3;
                                let a2050 = 1u8;
                                fixed_u8::bit_and(a2049, a2050)
                            };
                            let a2052 = 1u8;
                            if equal(a2051, a2052) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2053, a2054)?
                    };
                    nat_add(a2055, a2056)?
                };
                nat_add(a2057, a2058)?
            };
            if f322(v4) {
                let a2065 = f32({
                    let a2059 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2059)
                }, {
                    let a2060 = v4;
                    let a2061 = 65u64;
                    nat_sub(a2060, a2061)
                }, 1u64);
                let a2066 = f32(v1.clone(), 1u64, {
                    let a2063 = {
                        let a2062 = v1;
                        length_bytes(a2062)
                    };
                    let a2064 = 1u64;
                    nat_sub(a2063, a2064)
                });
                Ok(append_bytes(a2065, a2066))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f384(v0: Bytes) -> R<bool> {
    let m2069 = {
        let a2067 = v0;
        let a2068 = 0u64;
        index_bytes(a2067, a2068)
    };
    match m2069 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f321({
                let v2: u8 = v1;
                let a2114 = {
                    let a2090 = {
                        let a2078 = {
                            let a2072 = {
                                let a2070 = v2;
                                let a2071 = 128u8;
                                fixed_u8::bit_and(a2070, a2071)
                            };
                            let a2073 = 128u8;
                            if equal(a2072, a2073) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2079 = {
                            let a2076 = {
                                let a2074 = v2;
                                let a2075 = 64u8;
                                fixed_u8::bit_and(a2074, a2075)
                            };
                            let a2077 = 64u8;
                            if equal(a2076, a2077) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2078, a2079)?
                    };
                    let a2091 = {
                        let a2088 = {
                            let a2082 = {
                                let a2080 = v2;
                                let a2081 = 32u8;
                                fixed_u8::bit_and(a2080, a2081)
                            };
                            let a2083 = 32u8;
                            if equal(a2082, a2083) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2089 = {
                            let a2086 = {
                                let a2084 = v2;
                                let a2085 = 16u8;
                                fixed_u8::bit_and(a2084, a2085)
                            };
                            let a2087 = 16u8;
                            if equal(a2086, a2087) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2088, a2089)?
                    };
                    nat_add(a2090, a2091)?
                };
                let a2115 = {
                    let a2112 = {
                        let a2100 = {
                            let a2094 = {
                                let a2092 = v2;
                                let a2093 = 8u8;
                                fixed_u8::bit_and(a2092, a2093)
                            };
                            let a2095 = 8u8;
                            if equal(a2094, a2095) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2101 = {
                            let a2098 = {
                                let a2096 = v2;
                                let a2097 = 4u8;
                                fixed_u8::bit_and(a2096, a2097)
                            };
                            let a2099 = 4u8;
                            if equal(a2098, a2099) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2100, a2101)?
                    };
                    let a2113 = {
                        let a2110 = {
                            let a2104 = {
                                let a2102 = v2;
                                let a2103 = 2u8;
                                fixed_u8::bit_and(a2102, a2103)
                            };
                            let a2105 = 2u8;
                            if equal(a2104, a2105) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2111 = {
                            let a2108 = {
                                let a2106 = v2;
                                let a2107 = 1u8;
                                fixed_u8::bit_and(a2106, a2107)
                            };
                            let a2109 = 1u8;
                            if equal(a2108, a2109) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2110, a2111)?
                    };
                    nat_add(a2112, a2113)?
                };
                nat_add(a2114, a2115)?
            }) {
                Ok(true)
            } else {
                Ok(f322({
                    let v3: u8 = v1;
                    let a2160 = {
                        let a2136 = {
                            let a2124 = {
                                let a2118 = {
                                    let a2116 = v3;
                                    let a2117 = 128u8;
                                    fixed_u8::bit_and(a2116, a2117)
                                };
                                let a2119 = 128u8;
                                if equal(a2118, a2119) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a2125 = {
                                let a2122 = {
                                    let a2120 = v3;
                                    let a2121 = 64u8;
                                    fixed_u8::bit_and(a2120, a2121)
                                };
                                let a2123 = 64u8;
                                if equal(a2122, a2123) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2124, a2125)?
                        };
                        let a2137 = {
                            let a2134 = {
                                let a2128 = {
                                    let a2126 = v3;
                                    let a2127 = 32u8;
                                    fixed_u8::bit_and(a2126, a2127)
                                };
                                let a2129 = 32u8;
                                if equal(a2128, a2129) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a2135 = {
                                let a2132 = {
                                    let a2130 = v3;
                                    let a2131 = 16u8;
                                    fixed_u8::bit_and(a2130, a2131)
                                };
                                let a2133 = 16u8;
                                if equal(a2132, a2133) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2134, a2135)?
                        };
                        nat_add(a2136, a2137)?
                    };
                    let a2161 = {
                        let a2158 = {
                            let a2146 = {
                                let a2140 = {
                                    let a2138 = v3;
                                    let a2139 = 8u8;
                                    fixed_u8::bit_and(a2138, a2139)
                                };
                                let a2141 = 8u8;
                                if equal(a2140, a2141) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a2147 = {
                                let a2144 = {
                                    let a2142 = v3;
                                    let a2143 = 4u8;
                                    fixed_u8::bit_and(a2142, a2143)
                                };
                                let a2145 = 4u8;
                                if equal(a2144, a2145) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2146, a2147)?
                        };
                        let a2159 = {
                            let a2156 = {
                                let a2150 = {
                                    let a2148 = v3;
                                    let a2149 = 2u8;
                                    fixed_u8::bit_and(a2148, a2149)
                                };
                                let a2151 = 2u8;
                                if equal(a2150, a2151) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2157 = {
                                let a2154 = {
                                    let a2152 = v3;
                                    let a2153 = 1u8;
                                    fixed_u8::bit_and(a2152, a2153)
                                };
                                let a2155 = 1u8;
                                if equal(a2154, a2155) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2156, a2157)?
                        };
                        nat_add(a2158, a2159)?
                    };
                    nat_add(a2160, a2161)?
                }))
            }
        }
    }
}

pub fn f385(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2162 = f409(v0.clone());
    match m2162 {
        None => {
            if f36(f139(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2163 = f136(v0, f8(Str::lit("items")));
                match m2163 {
                    None => {
                        Ok(f355(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f410(f8(Str::lit("List")), f411(v3, {
                            let a2164 = v1;
                            let a2165 = f8(Str::lit("Item"));
                            append_bytes(a2164, a2165)
                        })?))
                    }
                }
            } else {
                f411(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f410(f8(Str::lit("Option")), f411(v2, v1)?))
        }
    }
}

pub fn f386(v0: Bytes, v1: u64) -> Bytes {
    let m2169 = {
        let a2166 = v0;
        let a2167 = v1;
        let a2168 = 1u64;
        slice_bytes(a2166, a2167, a2168)
    };
    match m2169 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f387(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f412(v1, v0, v2)
}

pub fn f388(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2214 = {
            let a2190 = {
                let a2178 = {
                    let a2172 = {
                        let a2170 = v3;
                        let a2171 = 128u8;
                        fixed_u8::bit_and(a2170, a2171)
                    };
                    let a2173 = 128u8;
                    if equal(a2172, a2173) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2179 = {
                    let a2176 = {
                        let a2174 = v3;
                        let a2175 = 64u8;
                        fixed_u8::bit_and(a2174, a2175)
                    };
                    let a2177 = 64u8;
                    if equal(a2176, a2177) {
                        64u64
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
                        let a2181 = 32u8;
                        fixed_u8::bit_and(a2180, a2181)
                    };
                    let a2183 = 32u8;
                    if equal(a2182, a2183) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2189 = {
                    let a2186 = {
                        let a2184 = v3;
                        let a2185 = 16u8;
                        fixed_u8::bit_and(a2184, a2185)
                    };
                    let a2187 = 16u8;
                    if equal(a2186, a2187) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2188, a2189)?
            };
            nat_add(a2190, a2191)?
        };
        let a2215 = {
            let a2212 = {
                let a2200 = {
                    let a2194 = {
                        let a2192 = v3;
                        let a2193 = 8u8;
                        fixed_u8::bit_and(a2192, a2193)
                    };
                    let a2195 = 8u8;
                    if equal(a2194, a2195) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2201 = {
                    let a2198 = {
                        let a2196 = v3;
                        let a2197 = 4u8;
                        fixed_u8::bit_and(a2196, a2197)
                    };
                    let a2199 = 4u8;
                    if equal(a2198, a2199) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2200, a2201)?
            };
            let a2213 = {
                let a2210 = {
                    let a2204 = {
                        let a2202 = v3;
                        let a2203 = 2u8;
                        fixed_u8::bit_and(a2202, a2203)
                    };
                    let a2205 = 2u8;
                    if equal(a2204, a2205) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2211 = {
                    let a2208 = {
                        let a2206 = v3;
                        let a2207 = 1u8;
                        fixed_u8::bit_and(a2206, a2207)
                    };
                    let a2209 = 1u8;
                    if equal(a2208, a2209) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2210, a2211)?
            };
            nat_add(a2212, a2213)?
        };
        nat_add(a2214, a2215)?
    };
    let a2216 = v4;
    let a2217 = 34u64;
    if nat_eq(a2216, a2217) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2218 = v4;
        let a2219 = 92u64;
        if nat_eq(a2218, a2219) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2220 = v4;
            let a2221 = 8u64;
            if nat_eq(a2220, a2221) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2222 = v4;
                let a2223 = 12u64;
                if nat_eq(a2222, a2223) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2224 = v4;
                    let a2225 = 10u64;
                    if nat_eq(a2224, a2225) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2226 = v4;
                        let a2227 = 13u64;
                        if nat_eq(a2226, a2227) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2228 = v4;
                            let a2229 = 9u64;
                            if nat_eq(a2228, a2229) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2230 = v4;
                                let a2231 = 32u64;
                                if nat_lt(a2230, a2231) {
                                    let a2232 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2233 = f413(v2)?;
                                    Ok(append_bytes(a2232, a2233))
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

pub fn f389(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f414(Fn21::F415, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f390(v0: Fn17, p2237: List<List<(Bytes, Bytes)>>, p2238: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2236 = (p2237, p2238);
    {
        let mut m2236 = m2236;
        loop {
            let step = {
                let (v1, v2) = m2236;
                let m2234 = v2;
                match m2234.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2235: Fn17 = v0.clone();
                            c2235.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f392(v0: Fn18, p2243: u64, p2244: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2242 = (p2243, p2244);
    {
        let mut m2242 = m2242;
        loop {
            let step = {
                let (v1, v2) = m2242;
                let m2239 = v1;
                if m2239 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2239 - 1;
                    let m2241 = {
                        let c2240: Fn18 = v0.clone();
                        c2240.apply(v2.clone())?
                    };
                    match m2241 {
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
                Ok(next) => { m2242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f416(v0)
}

pub fn f394(v0: Fn19, p2249: u64, p2250: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2248 = (p2249, p2250);
    {
        let mut m2248 = m2248;
        loop {
            let step = {
                let (v1, v2) = m2248;
                let m2245 = v1;
                if m2245 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2245 - 1;
                    let m2247 = {
                        let c2246: Fn19 = v0.clone();
                        c2246.apply(v2.clone())?
                    };
                    match m2247 {
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
                Ok(next) => { m2248 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f417(v0)
}

pub fn f396(v0: u64) -> u32 {
    let a2251 = v0;
    let a2252 = 32u64;
    if nat_lt(a2251, a2252) {
        let a2253 = v0;
        let a2254 = 16u64;
        if nat_lt(a2253, a2254) {
            let a2255 = v0;
            let a2256 = 8u64;
            if nat_lt(a2255, a2256) {
                let a2257 = v0;
                let a2258 = 4u64;
                if nat_lt(a2257, a2258) {
                    let a2259 = v0;
                    let a2260 = 2u64;
                    if nat_lt(a2259, a2260) {
                        let a2261 = v0;
                        let a2262 = 1u64;
                        if nat_lt(a2261, a2262) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2263 = v0;
                        let a2264 = 3u64;
                        if nat_lt(a2263, a2264) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2265 = v0;
                    let a2266 = 6u64;
                    if nat_lt(a2265, a2266) {
                        let a2267 = v0;
                        let a2268 = 5u64;
                        if nat_lt(a2267, a2268) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2269 = v0;
                        let a2270 = 7u64;
                        if nat_lt(a2269, a2270) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2271 = v0;
                let a2272 = 12u64;
                if nat_lt(a2271, a2272) {
                    let a2273 = v0;
                    let a2274 = 10u64;
                    if nat_lt(a2273, a2274) {
                        let a2275 = v0;
                        let a2276 = 9u64;
                        if nat_lt(a2275, a2276) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2277 = v0;
                        let a2278 = 11u64;
                        if nat_lt(a2277, a2278) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2279 = v0;
                    let a2280 = 14u64;
                    if nat_lt(a2279, a2280) {
                        let a2281 = v0;
                        let a2282 = 13u64;
                        if nat_lt(a2281, a2282) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2283 = v0;
                        let a2284 = 15u64;
                        if nat_lt(a2283, a2284) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2285 = v0;
            let a2286 = 24u64;
            if nat_lt(a2285, a2286) {
                let a2287 = v0;
                let a2288 = 20u64;
                if nat_lt(a2287, a2288) {
                    let a2289 = v0;
                    let a2290 = 18u64;
                    if nat_lt(a2289, a2290) {
                        let a2291 = v0;
                        let a2292 = 17u64;
                        if nat_lt(a2291, a2292) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2293 = v0;
                        let a2294 = 19u64;
                        if nat_lt(a2293, a2294) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2295 = v0;
                    let a2296 = 22u64;
                    if nat_lt(a2295, a2296) {
                        let a2297 = v0;
                        let a2298 = 21u64;
                        if nat_lt(a2297, a2298) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2299 = v0;
                        let a2300 = 23u64;
                        if nat_lt(a2299, a2300) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2301 = v0;
                let a2302 = 28u64;
                if nat_lt(a2301, a2302) {
                    let a2303 = v0;
                    let a2304 = 26u64;
                    if nat_lt(a2303, a2304) {
                        let a2305 = v0;
                        let a2306 = 25u64;
                        if nat_lt(a2305, a2306) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2307 = v0;
                        let a2308 = 27u64;
                        if nat_lt(a2307, a2308) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2309 = v0;
                    let a2310 = 30u64;
                    if nat_lt(a2309, a2310) {
                        let a2311 = v0;
                        let a2312 = 29u64;
                        if nat_lt(a2311, a2312) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2313 = v0;
                        let a2314 = 31u64;
                        if nat_lt(a2313, a2314) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2315 = v0;
        let a2316 = 48u64;
        if nat_lt(a2315, a2316) {
            let a2317 = v0;
            let a2318 = 40u64;
            if nat_lt(a2317, a2318) {
                let a2319 = v0;
                let a2320 = 36u64;
                if nat_lt(a2319, a2320) {
                    let a2321 = v0;
                    let a2322 = 34u64;
                    if nat_lt(a2321, a2322) {
                        let a2323 = v0;
                        let a2324 = 33u64;
                        if nat_lt(a2323, a2324) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2325 = v0;
                        let a2326 = 35u64;
                        if nat_lt(a2325, a2326) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2327 = v0;
                    let a2328 = 38u64;
                    if nat_lt(a2327, a2328) {
                        let a2329 = v0;
                        let a2330 = 37u64;
                        if nat_lt(a2329, a2330) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2331 = v0;
                        let a2332 = 39u64;
                        if nat_lt(a2331, a2332) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2333 = v0;
                let a2334 = 44u64;
                if nat_lt(a2333, a2334) {
                    let a2335 = v0;
                    let a2336 = 42u64;
                    if nat_lt(a2335, a2336) {
                        let a2337 = v0;
                        let a2338 = 41u64;
                        if nat_lt(a2337, a2338) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2339 = v0;
                        let a2340 = 43u64;
                        if nat_lt(a2339, a2340) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2341 = v0;
                    let a2342 = 46u64;
                    if nat_lt(a2341, a2342) {
                        let a2343 = v0;
                        let a2344 = 45u64;
                        if nat_lt(a2343, a2344) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2345 = v0;
                        let a2346 = 47u64;
                        if nat_lt(a2345, a2346) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2347 = v0;
            let a2348 = 56u64;
            if nat_lt(a2347, a2348) {
                let a2349 = v0;
                let a2350 = 52u64;
                if nat_lt(a2349, a2350) {
                    let a2351 = v0;
                    let a2352 = 50u64;
                    if nat_lt(a2351, a2352) {
                        let a2353 = v0;
                        let a2354 = 49u64;
                        if nat_lt(a2353, a2354) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2355 = v0;
                        let a2356 = 51u64;
                        if nat_lt(a2355, a2356) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2357 = v0;
                    let a2358 = 54u64;
                    if nat_lt(a2357, a2358) {
                        let a2359 = v0;
                        let a2360 = 53u64;
                        if nat_lt(a2359, a2360) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2361 = v0;
                        let a2362 = 55u64;
                        if nat_lt(a2361, a2362) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2363 = v0;
                let a2364 = 60u64;
                if nat_lt(a2363, a2364) {
                    let a2365 = v0;
                    let a2366 = 58u64;
                    if nat_lt(a2365, a2366) {
                        let a2367 = v0;
                        let a2368 = 57u64;
                        if nat_lt(a2367, a2368) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2369 = v0;
                        let a2370 = 59u64;
                        if nat_lt(a2369, a2370) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2371 = v0;
                    let a2372 = 62u64;
                    if nat_lt(a2371, a2372) {
                        let a2373 = v0;
                        let a2374 = 61u64;
                        if nat_lt(a2373, a2374) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2375 = v0;
                        let a2376 = 63u64;
                        if nat_lt(a2375, a2376) {
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

pub fn f397(v0: u32) -> u32 {
    let a2379 = {
        let a2377 = f418(v0, 6u32, 26u32);
        let a2378 = f418(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2377, a2378)
    };
    let a2380 = f418(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2379, a2380)
}

pub fn f398(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2386 = {
        let a2381 = v0;
        let a2382 = v1;
        fixed_u32::bit_and(a2381, a2382)
    };
    let a2387 = {
        let a2384 = {
            let a2383 = v0;
            fixed_u32::bit_not(a2383)
        };
        let a2385 = v2;
        fixed_u32::bit_and(a2384, a2385)
    };
    fixed_u32::bit_xor(a2386, a2387)
}

pub fn f399(v0: u32) -> u32 {
    let a2390 = {
        let a2388 = f418(v0, 2u32, 30u32);
        let a2389 = f418(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2388, a2389)
    };
    let a2391 = f418(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2390, a2391)
}

pub fn f400(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2400 = {
        let a2396 = {
            let a2392 = v0;
            let a2393 = v1;
            fixed_u32::bit_and(a2392, a2393)
        };
        let a2397 = {
            let a2394 = v0;
            let a2395 = v2;
            fixed_u32::bit_and(a2394, a2395)
        };
        fixed_u32::bit_xor(a2396, a2397)
    };
    let a2401 = {
        let a2398 = v1;
        let a2399 = v2;
        fixed_u32::bit_and(a2398, a2399)
    };
    fixed_u32::bit_xor(a2400, a2401)
}

pub fn f401(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h2402, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2402
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2403, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2403
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2404, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2404
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2405, _, _, _, _, _, _, _, _, _, _, _) => {
            h2405
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2406, _, _, _, _, _, _, _, _, _, _) => {
            h2406
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2407, _, _, _, _, _, _, _, _, _) => {
            h2407
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2408, _, _, _, _, _, _, _, _) => {
            h2408
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h2409, _, _, _, _, _, _, _) => {
            h2409
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2410, _, _, _, _, _, _) => {
            h2410
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h2411, _, _, _, _, _) => {
            h2411
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h2412, _, _, _, _) => {
            h2412
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2413, _, _, _) => {
            h2413
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2414, _, _) => {
            h2414
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2415, _) => {
            h2415
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2416) => {
            h2416
        }
    }, f346(f419(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2417, _) => {
            h2417
        }
    }), f346(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2418, _, _, _, _, _, _) => {
            h2418
        }
    }, f346(f420(match v0.clone() {
        Adt24::C0(_, h2419, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2419
        }
    }), match v0 {
        Adt24::C0(h2420, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2420
        }
    }))))
}

pub fn f402(v0: Fn20, p2424: u64, p2425: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2423 = (p2424, p2425);
    {
        let mut m2423 = m2423;
        loop {
            let step = {
                let (v1, v2) = m2423;
                let m2421 = v1;
                if m2421 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2421 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2422: Fn20 = v0.clone();
                        c2422.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2426, _) = v0;
        h2426
    };
    let v2: u32 = {
        let (h2427, _) = {
            let (_, h2428) = v0;
            h2428
        };
        h2427
    };
    let v3: u32 = {
        let (_, h2429) = {
            let (_, h2430) = v0;
            h2430
        };
        h2429
    };
    ({
        let a2431 = v1;
        let a2432 = 2u64;
        let a2433 = 0u64;
        nat_quot(a2431, a2432, a2433)
    }, (f404(v2, 1u32), {
        let a2437 = {
            let a2434 = v1;
            let a2435 = 2u64;
            let a2436 = 0u64;
            nat_rem(a2434, a2435, a2436)
        };
        let a2438 = 1u64;
        if nat_eq(a2437, a2438) {
            let a2439 = v3;
            let a2440 = v2;
            fixed_u32::bit_or(a2439, a2440)
        } else {
            v3
        }
    }))
}

pub fn f404(v0: u32, v1: u32) -> u32 {
    let m2443 = {
        let a2441 = v0;
        let a2442 = v1;
        fixed_u32::shift_left(a2441, a2442)
    };
    match m2443 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f405(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f317(v0.clone(), {
        let a2444 = v1;
        let a2445 = 1u64;
        nat_add(a2444, a2445)?
    })?;
    let v4: Bytes = {
        let a2446 = v3;
        let a2447 = 117u64;
        if nat_eq(a2446, a2447) {
            f421(v0, v1)?
        } else {
            f422(v3)
        }
    };
    let a2448 = v4.clone();
    let a2449 = Bytes::lit(&[]);
    if equal(a2448, a2449) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2450 = v3;
            let a2451 = 117u64;
            if nat_eq(a2450, a2451) {
                let a2452 = v1;
                let a2453 = 6u64;
                nat_add(a2452, a2453)?
            } else {
                let a2454 = v1;
                let a2455 = 2u64;
                nat_add(a2454, a2455)?
            }
        }, ({
            let a2456 = v2;
            let a2457 = v4;
            append_bytes(a2456, a2457)
        }, 0u64))))
    }
}

pub fn f406(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f317(v0, v1)?;
    let a2458 = 47u64;
    let a2459 = v2;
    if if nat_lt(a2458, a2459) {
        let a2460 = v2;
        let a2461 = 58u64;
        nat_lt(a2460, a2461)
    } else {
        false
    } {
        Ok(Some({
            let a2462 = v1;
            let a2463 = 1u64;
            nat_add(a2462, a2463)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f407(v0: Bytes) -> R<u64> {
    let (h2464, _) = f221(Fn3::F423(v0.clone()), {
        let a2465 = v0;
        length_bytes(a2465)
    }, 0u64)?;
    Ok(h2464)
}

pub fn f408(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2469 = {
        let a2467 = v0;
        let a2468 = {
            let (h2466, _) = v1;
            h2466
        };
        index_bytes(a2467, a2468)
    };
    match m2469 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2471 = {
                    let (h2470, _) = v1;
                    h2470
                };
                let a2472 = 1u64;
                nat_add(a2471, a2472)?
            }, {
                let a2476 = {
                    let a2474 = {
                        let (_, h2473) = v1;
                        h2473
                    };
                    let a2475 = 10u64;
                    nat_mul(a2474, a2475)?
                };
                let a2477 = f424(v2)?;
                nat_add(a2476, a2477)?
            })))
        }
    }
}

pub fn f409(v0: Adt14) -> Option<Adt14> {
    let m2478 = f140(f136(v0, f8(Str::lit("oneOf"))));
    match m2478.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2479 = v2;
            match m2479.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2480 = v4;
                    match m2480.uncons() {
                        None => {
                            if f425(v3.clone()) {
                                Some(v1)
                            } else {
                                if f425(v1) {
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

pub fn f410(v0: Bytes, v1: Result<(Bytes, List<Adt23>), Bytes>) -> Result<(Bytes, List<Adt23>), Bytes> {
    let m2481 = v1;
    match m2481 {
        Ok(v3) => {
            f356(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2482, _) = v3;
                h2482
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2483) = v3;
                h2483
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v2)
        }
    }
}

pub fn f411(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2484 = f409(v0.clone());
    match m2484 {
        None => {
            if f36(f139(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2485 = f136(v0, f8(Str::lit("items")));
                match m2485 {
                    None => {
                        Ok(f355(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f410(f8(Str::lit("List")), f426(v3, {
                            let a2486 = v1;
                            let a2487 = f8(Str::lit("Item"));
                            append_bytes(a2486, a2487)
                        })?))
                    }
                }
            } else {
                f426(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f410(f8(Str::lit("Option")), f426(v2, v1)?))
        }
    }
}

pub fn f412(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2488, _) = v2;
        h2488
    };
    let a2489 = v3;
    let a2490 = 0u64;
    if nat_eq(a2489, a2490) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2491 = v3;
            let a2492 = v1;
            let a2493 = 0u64;
            nat_quot(a2491, a2492, a2493)
        }, {
            let a2498 = f386(v0, {
                let a2494 = v3;
                let a2495 = v1;
                let a2496 = 0u64;
                nat_rem(a2494, a2495, a2496)
            });
            let a2499 = {
                let (_, h2497) = v2;
                h2497
            };
            append_bytes(a2498, a2499)
        }))
    }
}

pub fn f413(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2544 = {
            let a2520 = {
                let a2508 = {
                    let a2502 = {
                        let a2500 = v1;
                        let a2501 = 128u8;
                        fixed_u8::bit_and(a2500, a2501)
                    };
                    let a2503 = 128u8;
                    if equal(a2502, a2503) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2509 = {
                    let a2506 = {
                        let a2504 = v1;
                        let a2505 = 64u8;
                        fixed_u8::bit_and(a2504, a2505)
                    };
                    let a2507 = 64u8;
                    if equal(a2506, a2507) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2508, a2509)?
            };
            let a2521 = {
                let a2518 = {
                    let a2512 = {
                        let a2510 = v1;
                        let a2511 = 32u8;
                        fixed_u8::bit_and(a2510, a2511)
                    };
                    let a2513 = 32u8;
                    if equal(a2512, a2513) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2519 = {
                    let a2516 = {
                        let a2514 = v1;
                        let a2515 = 16u8;
                        fixed_u8::bit_and(a2514, a2515)
                    };
                    let a2517 = 16u8;
                    if equal(a2516, a2517) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2518, a2519)?
            };
            nat_add(a2520, a2521)?
        };
        let a2545 = {
            let a2542 = {
                let a2530 = {
                    let a2524 = {
                        let a2522 = v1;
                        let a2523 = 8u8;
                        fixed_u8::bit_and(a2522, a2523)
                    };
                    let a2525 = 8u8;
                    if equal(a2524, a2525) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2531 = {
                    let a2528 = {
                        let a2526 = v1;
                        let a2527 = 4u8;
                        fixed_u8::bit_and(a2526, a2527)
                    };
                    let a2529 = 4u8;
                    if equal(a2528, a2529) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2530, a2531)?
            };
            let a2543 = {
                let a2540 = {
                    let a2534 = {
                        let a2532 = v1;
                        let a2533 = 2u8;
                        fixed_u8::bit_and(a2532, a2533)
                    };
                    let a2535 = 2u8;
                    if equal(a2534, a2535) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2541 = {
                    let a2538 = {
                        let a2536 = v1;
                        let a2537 = 1u8;
                        fixed_u8::bit_and(a2536, a2537)
                    };
                    let a2539 = 1u8;
                    if equal(a2538, a2539) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2540, a2541)?
            };
            nat_add(a2542, a2543)?
        };
        nat_add(a2544, a2545)?
    };
    let a2552 = f386(f427(), {
        let a2546 = v2;
        let a2547 = 16u64;
        let a2548 = 0u64;
        nat_quot(a2546, a2547, a2548)
    });
    let a2553 = f386(f427(), {
        let a2549 = v2;
        let a2550 = 16u64;
        let a2551 = 0u64;
        nat_rem(a2549, a2550, a2551)
    });
    Ok(append_bytes(a2552, a2553))
}

pub fn f414(v0: Fn21, p2557: List<List<(Bytes, Bytes)>>, p2558: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2556 = (p2557, p2558);
    {
        let mut m2556 = m2556;
        loop {
            let step = {
                let (v1, v2) = m2556;
                let m2554 = v2;
                match m2554.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2555: Fn21 = v0.clone();
                            c2555.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2556 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f415(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f416(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2560 = {
        let a2559 = v0.clone();
        length_list(a2559)
    };
    let a2561 = 1u64;
    if nat_le(a2560, a2561) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f428(v0)?))
    }
}

pub fn f417(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2562, _) = v0;
        h2562
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2563) = v0;
        h2563
    };
    let m2564 = v1;
    match m2564.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2565 = v4;
            match m2565.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f429(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f418(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2566 = f311(v0, v1);
    let a2567 = f404(v0, v2);
    fixed_u32::bit_or(a2566, a2567)
}

pub fn f419(v0: u32) -> u32 {
    let a2570 = {
        let a2568 = f418(v0, 17u32, 15u32);
        let a2569 = f418(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2568, a2569)
    };
    let a2571 = f311(v0, 10u32);
    fixed_u32::bit_xor(a2570, a2571)
}

pub fn f420(v0: u32) -> u32 {
    let a2574 = {
        let a2572 = f418(v0, 7u32, 25u32);
        let a2573 = f418(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2572, a2573)
    };
    let a2575 = f311(v0, 3u32);
    fixed_u32::bit_xor(a2574, a2575)
}

pub fn f421(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2578 = f32(v0.clone(), {
        let a2576 = v1;
        let a2577 = 2u64;
        nat_add(a2576, a2577)?
    }, 2u64);
    let a2579 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2578, a2579) {
        let m2582 = f430(f32(v0, {
            let a2580 = v1;
            let a2581 = 4u64;
            nat_add(a2580, a2581)?
        }, 2u64))?;
        match m2582 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f317(v2.clone(), 0u64)?;
                let a2583 = v3;
                let a2584 = 32u64;
                if nat_lt(a2583, a2584) {
                    if f431(v3) {
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

pub fn f422(v0: u64) -> Bytes {
    let a2585 = v0;
    let a2586 = 34u64;
    if nat_eq(a2585, a2586) {
        Bytes::lit(&[0x22])
    } else {
        let a2587 = v0;
        let a2588 = 92u64;
        if nat_eq(a2587, a2588) {
            Bytes::lit(&[0x5c])
        } else {
            let a2589 = v0;
            let a2590 = 98u64;
            if nat_eq(a2589, a2590) {
                Bytes::lit(&[0x08])
            } else {
                let a2591 = v0;
                let a2592 = 102u64;
                if nat_eq(a2591, a2592) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2593 = v0;
                    let a2594 = 110u64;
                    if nat_eq(a2593, a2594) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2595 = v0;
                        let a2596 = 114u64;
                        if nat_eq(a2595, a2596) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2597 = v0;
                            let a2598 = 116u64;
                            if nat_eq(a2597, a2598) {
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

pub fn f423(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2601 = {
        let a2599 = v0;
        let a2600 = v1;
        index_bytes(a2599, a2600)
    };
    match m2601 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f432(v2)? {
                Ok(Some({
                    let a2602 = v1;
                    let a2603 = 1u64;
                    nat_add(a2602, a2603)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f424(v0: u8) -> R<u64> {
    let a2650 = {
        let v1: u8 = v0;
        let a2648 = {
            let a2624 = {
                let a2612 = {
                    let a2606 = {
                        let a2604 = v1;
                        let a2605 = 128u8;
                        fixed_u8::bit_and(a2604, a2605)
                    };
                    let a2607 = 128u8;
                    if equal(a2606, a2607) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2613 = {
                    let a2610 = {
                        let a2608 = v1;
                        let a2609 = 64u8;
                        fixed_u8::bit_and(a2608, a2609)
                    };
                    let a2611 = 64u8;
                    if equal(a2610, a2611) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2612, a2613)?
            };
            let a2625 = {
                let a2622 = {
                    let a2616 = {
                        let a2614 = v1;
                        let a2615 = 32u8;
                        fixed_u8::bit_and(a2614, a2615)
                    };
                    let a2617 = 32u8;
                    if equal(a2616, a2617) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2623 = {
                    let a2620 = {
                        let a2618 = v1;
                        let a2619 = 16u8;
                        fixed_u8::bit_and(a2618, a2619)
                    };
                    let a2621 = 16u8;
                    if equal(a2620, a2621) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2622, a2623)?
            };
            nat_add(a2624, a2625)?
        };
        let a2649 = {
            let a2646 = {
                let a2634 = {
                    let a2628 = {
                        let a2626 = v1;
                        let a2627 = 8u8;
                        fixed_u8::bit_and(a2626, a2627)
                    };
                    let a2629 = 8u8;
                    if equal(a2628, a2629) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2635 = {
                    let a2632 = {
                        let a2630 = v1;
                        let a2631 = 4u8;
                        fixed_u8::bit_and(a2630, a2631)
                    };
                    let a2633 = 4u8;
                    if equal(a2632, a2633) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2634, a2635)?
            };
            let a2647 = {
                let a2644 = {
                    let a2638 = {
                        let a2636 = v1;
                        let a2637 = 2u8;
                        fixed_u8::bit_and(a2636, a2637)
                    };
                    let a2639 = 2u8;
                    if equal(a2638, a2639) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2645 = {
                    let a2642 = {
                        let a2640 = v1;
                        let a2641 = 1u8;
                        fixed_u8::bit_and(a2640, a2641)
                    };
                    let a2643 = 1u8;
                    if equal(a2642, a2643) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2644, a2645)?
            };
            nat_add(a2646, a2647)?
        };
        nat_add(a2648, a2649)?
    };
    let a2651 = 48u64;
    Ok(nat_sub(a2650, a2651))
}

pub fn f425(v0: Adt14) -> bool {
    f36(f139(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f426(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2652 = f433(f325(v0.clone()));
    match m2652 {
        None => {
            let m2653 = f144(f136(v0.clone(), f8(Str::lit("$ref"))));
            match m2653 {
                None => {
                    let m2655 = f136(v0.clone(), f8(Str::lit("enum")));
                    match m2655 {
                        None => {
                            let m2656 = f136(v0.clone(), f8(Str::lit("const")));
                            match m2656 {
                                None => {
                                    let v8: Bytes = f139(v0.clone(), f8(Str::lit("type")));
                                    if f36(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f356(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                    } else {
                                        if f36(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f356(f435(v0), List::<Adt23>::nil()))
                                        } else {
                                            if f36(v8.clone(), f8(Str::lit("number"))) {
                                                Ok(f356(f436(), List::<Adt23>::nil()))
                                            } else {
                                                if f36(v8.clone(), f8(Str::lit("boolean"))) {
                                                    Ok(f356(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                                } else {
                                                    if f36(v8.clone(), f8(Str::lit("object"))) {
                                                        Ok(f356(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                    } else {
                                                        Ok(f355(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2657 = f144(Some(v6));
                                    match m2657 {
                                        None => {
                                            Ok(f355(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f356(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f356(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2654 = f434(v3)?;
                    match m2654 {
                        None => {
                            Ok(f355(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f356(v4, List::<Adt23>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f355(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f427() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f428(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f389({
        let (_, h2658) = {
            let (h2659, _) = f437(Fn22::F438, {
                let a2661 = {
                    let a2660 = v0.clone();
                    length_list(a2660)
                };
                let a2662 = 1u64;
                nat_add(a2661, a2662)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2659
        };
        h2658
    }))
}

pub fn f429(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2667 = {
            let a2665 = {
                let a2663 = v0.clone();
                length_list(a2663)
            };
            let a2666 = {
                let a2664 = v1.clone();
                length_list(a2664)
            };
            nat_add(a2665, a2666)?
        };
        let a2668 = 1u64;
        nat_add(a2667, a2668)?
    };
    Ok(f301({
        let (_, h2669) = {
            let (_, h2670) = {
                let (h2671, _) = f439(Fn23::F440, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2671
            };
            h2670
        };
        h2669
    }))
}

pub fn f430(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2672 = v0.clone();
        length_bytes(a2672)
    };
    let a2676 = {
        let a2673 = v1;
        let a2674 = 2u64;
        let a2675 = 0u64;
        nat_rem(a2673, a2674, a2675)
    };
    let a2677 = 0u64;
    if nat_eq(a2676, a2677) {
        let (_, h2678) = {
            let (h2679, _) = f441(Fn24::F442(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2679
        };
        Ok(h2678)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f431(v0: u64) -> bool {
    let a2680 = v0;
    let a2681 = 8u64;
    if nat_eq(a2680, a2681) {
        true
    } else {
        let a2682 = v0;
        let a2683 = 9u64;
        if nat_eq(a2682, a2683) {
            true
        } else {
            let a2684 = v0;
            let a2685 = 10u64;
            if nat_eq(a2684, a2685) {
                true
            } else {
                let a2686 = v0;
                let a2687 = 12u64;
                if nat_eq(a2686, a2687) {
                    true
                } else {
                    let a2688 = v0;
                    let a2689 = 13u64;
                    nat_eq(a2688, a2689)
                }
            }
        }
    }
}

pub fn f432(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2734 = {
            let a2710 = {
                let a2698 = {
                    let a2692 = {
                        let a2690 = v1;
                        let a2691 = 128u8;
                        fixed_u8::bit_and(a2690, a2691)
                    };
                    let a2693 = 128u8;
                    if equal(a2692, a2693) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2699 = {
                    let a2696 = {
                        let a2694 = v1;
                        let a2695 = 64u8;
                        fixed_u8::bit_and(a2694, a2695)
                    };
                    let a2697 = 64u8;
                    if equal(a2696, a2697) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2698, a2699)?
            };
            let a2711 = {
                let a2708 = {
                    let a2702 = {
                        let a2700 = v1;
                        let a2701 = 32u8;
                        fixed_u8::bit_and(a2700, a2701)
                    };
                    let a2703 = 32u8;
                    if equal(a2702, a2703) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2709 = {
                    let a2706 = {
                        let a2704 = v1;
                        let a2705 = 16u8;
                        fixed_u8::bit_and(a2704, a2705)
                    };
                    let a2707 = 16u8;
                    if equal(a2706, a2707) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2708, a2709)?
            };
            nat_add(a2710, a2711)?
        };
        let a2735 = {
            let a2732 = {
                let a2720 = {
                    let a2714 = {
                        let a2712 = v1;
                        let a2713 = 8u8;
                        fixed_u8::bit_and(a2712, a2713)
                    };
                    let a2715 = 8u8;
                    if equal(a2714, a2715) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2721 = {
                    let a2718 = {
                        let a2716 = v1;
                        let a2717 = 4u8;
                        fixed_u8::bit_and(a2716, a2717)
                    };
                    let a2719 = 4u8;
                    if equal(a2718, a2719) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2720, a2721)?
            };
            let a2733 = {
                let a2730 = {
                    let a2724 = {
                        let a2722 = v1;
                        let a2723 = 2u8;
                        fixed_u8::bit_and(a2722, a2723)
                    };
                    let a2725 = 2u8;
                    if equal(a2724, a2725) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2731 = {
                    let a2728 = {
                        let a2726 = v1;
                        let a2727 = 1u8;
                        fixed_u8::bit_and(a2726, a2727)
                    };
                    let a2729 = 1u8;
                    if equal(a2728, a2729) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2730, a2731)?
            };
            nat_add(a2732, a2733)?
        };
        nat_add(a2734, a2735)?
    };
    let a2736 = 48u64;
    let a2737 = v2;
    if nat_le(a2736, a2737) {
        let a2738 = v2;
        let a2739 = 57u64;
        Ok(nat_le(a2738, a2739))
    } else {
        Ok(false)
    }
}

pub fn f433(p2745: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2744 = p2745;
    {
        let mut m2744 = m2744;
        loop {
            let step = {
                let v0 = m2744;
                let m2740 = v0;
                match m2740.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f84({
                            let (h2741, _) = v1.clone();
                            h2741
                        }, f443()) {
                            true
                        } else {
                            f84({
                                let (h2742, _) = v1.clone();
                                h2742
                            }, f444())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2743, _) = v1;
                                h2743
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2744 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Bytes) -> R<Option<Bytes>> {
    if f185(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f209(f32(v0.clone(), 8u64, {
            let a2747 = {
                let a2746 = v0;
                length_bytes(a2746)
            };
            let a2748 = 8u64;
            nat_sub(a2747, a2748)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f435(v0: Adt14) -> Bytes {
    let m2749 = f293(f136(v0, f8(Str::lit("minimum"))));
    match m2749 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f436() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f437(v0: Fn22, p2754: u64, p2755: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2753 = (p2754, p2755);
    {
        let mut m2753 = m2753;
        loop {
            let step = {
                let (v1, v2) = m2753;
                let m2750 = v1;
                if m2750 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2750 - 1;
                    let m2752 = {
                        let c2751: Fn22 = v0.clone();
                        c2751.apply(v2.clone())?
                    };
                    match m2752 {
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
                Ok(next) => { m2753 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f445(v0)
}

pub fn f439(v0: Fn23, p2760: u64, p2761: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2759 = (p2760, p2761);
    {
        let mut m2759 = m2759;
        loop {
            let step = {
                let (v1, v2) = m2759;
                let m2756 = v1;
                if m2756 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2756 - 1;
                    let m2758 = {
                        let c2757: Fn23 = v0.clone();
                        c2757.apply(v2.clone())
                    };
                    match m2758 {
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
                Ok(next) => { m2759 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f446(v0)
}

pub fn f441(v0: Fn24, p2766: u64, p2767: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2765 = (p2766, p2767);
    {
        let mut m2765 = m2765;
        loop {
            let step = {
                let (v1, v2) = m2765;
                let m2762 = v1;
                if m2762 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2762 - 1;
                    let m2764 = {
                        let c2763: Fn24 = v0.clone();
                        c2763.apply(v2.clone())?
                    };
                    match m2764 {
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
                Ok(next) => { m2765 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f447(v0, v1)
}

pub fn f443() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f444() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f445(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2768, _) = v0;
        h2768
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2769) = v0;
        h2769
    };
    let m2770 = v1;
    match m2770.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2771 = v4;
            match m2771.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f448(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f446(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2772, _) = v0.clone();
        h2772
    };
    let v2: List<(Bytes, ())> = {
        let (h2773, _) = {
            let (_, h2774) = v0.clone();
            h2774
        };
        h2773
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2775) = {
            let (_, h2776) = v0;
            h2776
        };
        h2775
    };
    let m2777 = v1.clone();
    match m2777.uncons() {
        None => {
            let m2778 = v2;
            match m2778.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2779 = v2.clone();
            match m2779.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f449({
                        let (h2780, _) = v8.clone();
                        h2780
                    }, {
                        let (h2781, _) = v6.clone();
                        h2781
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

pub fn f447(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2782, _) = v1;
        h2782
    };
    let m2784 = {
        let (_, h2783) = v1;
        h2783
    };
    match m2784 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2787 = {
                let a2785 = v0.clone();
                let a2786 = v2;
                index_bytes(a2785, a2786)
            };
            match m2787 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2792 = {
                        let a2790 = v0;
                        let a2791 = {
                            let a2788 = v2;
                            let a2789 = 1u64;
                            nat_add(a2788, a2789)?
                        };
                        index_bytes(a2790, a2791)
                    };
                    match m2792 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f450(v4)? {
                                f450(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2793 = v2;
                                    let a2794 = 2u64;
                                    nat_add(a2793, a2794)?
                                }, Some({
                                    let a2799 = v3;
                                    let a2800 = f386(f451(), {
                                        let a2797 = {
                                            let a2795 = f452(v4)?;
                                            let a2796 = 16u64;
                                            nat_mul(a2795, a2796)?
                                        };
                                        let a2798 = f452(v5)?;
                                        nat_add(a2797, a2798)?
                                    });
                                    append_bytes(a2799, a2800)
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

pub fn f448(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a2805 = {
            let a2803 = {
                let a2801 = v0.clone();
                length_list(a2801)
            };
            let a2804 = {
                let a2802 = v1.clone();
                length_list(a2802)
            };
            nat_add(a2803, a2804)?
        };
        let a2806 = 1u64;
        nat_add(a2805, a2806)?
    };
    Ok(f453({
        let (_, h2807) = {
            let (_, h2808) = {
                let (h2809, _) = f454(Fn25::F455, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h2809
            };
            h2808
        };
        h2807
    }))
}

pub fn f449(v0: Bytes, v1: Bytes) -> bool {
    let a2814 = {
        let a2810 = v0;
        let a2811 = v1;
        compare_bytes(a2810, a2811)
    };
    let a2815 = {
        let a2812 = Bytes::lit(&[]);
        let a2813 = Bytes::lit(&[0x01]);
        compare_bytes(a2812, a2813)
    };
    equal(a2814, a2815)
}

pub fn f450(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2860 = {
            let a2836 = {
                let a2824 = {
                    let a2818 = {
                        let a2816 = v1;
                        let a2817 = 128u8;
                        fixed_u8::bit_and(a2816, a2817)
                    };
                    let a2819 = 128u8;
                    if equal(a2818, a2819) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2825 = {
                    let a2822 = {
                        let a2820 = v1;
                        let a2821 = 64u8;
                        fixed_u8::bit_and(a2820, a2821)
                    };
                    let a2823 = 64u8;
                    if equal(a2822, a2823) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2824, a2825)?
            };
            let a2837 = {
                let a2834 = {
                    let a2828 = {
                        let a2826 = v1;
                        let a2827 = 32u8;
                        fixed_u8::bit_and(a2826, a2827)
                    };
                    let a2829 = 32u8;
                    if equal(a2828, a2829) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2835 = {
                    let a2832 = {
                        let a2830 = v1;
                        let a2831 = 16u8;
                        fixed_u8::bit_and(a2830, a2831)
                    };
                    let a2833 = 16u8;
                    if equal(a2832, a2833) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2834, a2835)?
            };
            nat_add(a2836, a2837)?
        };
        let a2861 = {
            let a2858 = {
                let a2846 = {
                    let a2840 = {
                        let a2838 = v1;
                        let a2839 = 8u8;
                        fixed_u8::bit_and(a2838, a2839)
                    };
                    let a2841 = 8u8;
                    if equal(a2840, a2841) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2847 = {
                    let a2844 = {
                        let a2842 = v1;
                        let a2843 = 4u8;
                        fixed_u8::bit_and(a2842, a2843)
                    };
                    let a2845 = 4u8;
                    if equal(a2844, a2845) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2846, a2847)?
            };
            let a2859 = {
                let a2856 = {
                    let a2850 = {
                        let a2848 = v1;
                        let a2849 = 2u8;
                        fixed_u8::bit_and(a2848, a2849)
                    };
                    let a2851 = 2u8;
                    if equal(a2850, a2851) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2857 = {
                    let a2854 = {
                        let a2852 = v1;
                        let a2853 = 1u8;
                        fixed_u8::bit_and(a2852, a2853)
                    };
                    let a2855 = 1u8;
                    if equal(a2854, a2855) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2856, a2857)?
            };
            nat_add(a2858, a2859)?
        };
        nat_add(a2860, a2861)?
    };
    let a2862 = 48u64;
    let a2863 = v2;
    if if nat_le(a2862, a2863) {
        let a2864 = v2;
        let a2865 = 57u64;
        nat_le(a2864, a2865)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2866 = 97u64;
        let a2867 = v2;
        if nat_le(a2866, a2867) {
            let a2868 = v2;
            let a2869 = 102u64;
            Ok(nat_le(a2868, a2869))
        } else {
            Ok(false)
        }
    }
}

pub fn f451() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f452(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2914 = {
            let a2890 = {
                let a2878 = {
                    let a2872 = {
                        let a2870 = v1;
                        let a2871 = 128u8;
                        fixed_u8::bit_and(a2870, a2871)
                    };
                    let a2873 = 128u8;
                    if equal(a2872, a2873) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2879 = {
                    let a2876 = {
                        let a2874 = v1;
                        let a2875 = 64u8;
                        fixed_u8::bit_and(a2874, a2875)
                    };
                    let a2877 = 64u8;
                    if equal(a2876, a2877) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2878, a2879)?
            };
            let a2891 = {
                let a2888 = {
                    let a2882 = {
                        let a2880 = v1;
                        let a2881 = 32u8;
                        fixed_u8::bit_and(a2880, a2881)
                    };
                    let a2883 = 32u8;
                    if equal(a2882, a2883) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2889 = {
                    let a2886 = {
                        let a2884 = v1;
                        let a2885 = 16u8;
                        fixed_u8::bit_and(a2884, a2885)
                    };
                    let a2887 = 16u8;
                    if equal(a2886, a2887) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2888, a2889)?
            };
            nat_add(a2890, a2891)?
        };
        let a2915 = {
            let a2912 = {
                let a2900 = {
                    let a2894 = {
                        let a2892 = v1;
                        let a2893 = 8u8;
                        fixed_u8::bit_and(a2892, a2893)
                    };
                    let a2895 = 8u8;
                    if equal(a2894, a2895) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2901 = {
                    let a2898 = {
                        let a2896 = v1;
                        let a2897 = 4u8;
                        fixed_u8::bit_and(a2896, a2897)
                    };
                    let a2899 = 4u8;
                    if equal(a2898, a2899) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2900, a2901)?
            };
            let a2913 = {
                let a2910 = {
                    let a2904 = {
                        let a2902 = v1;
                        let a2903 = 2u8;
                        fixed_u8::bit_and(a2902, a2903)
                    };
                    let a2905 = 2u8;
                    if equal(a2904, a2905) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2911 = {
                    let a2908 = {
                        let a2906 = v1;
                        let a2907 = 1u8;
                        fixed_u8::bit_and(a2906, a2907)
                    };
                    let a2909 = 1u8;
                    if equal(a2908, a2909) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2910, a2911)?
            };
            nat_add(a2912, a2913)?
        };
        nat_add(a2914, a2915)?
    };
    let a2916 = v2;
    let a2917 = 57u64;
    if nat_le(a2916, a2917) {
        let a2918 = v2;
        let a2919 = 48u64;
        Ok(nat_sub(a2918, a2919))
    } else {
        let a2920 = v2;
        let a2921 = 87u64;
        Ok(nat_sub(a2920, a2921))
    }
}

pub fn f453(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f331(Fn10::F456, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f454(v0: Fn25, p2926: u64, p2927: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m2925 = (p2926, p2927);
    {
        let mut m2925 = m2925;
        loop {
            let step = {
                let (v1, v2) = m2925;
                let m2922 = v1;
                if m2922 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m2922 - 1;
                    let m2924 = {
                        let c2923: Fn25 = v0.clone();
                        c2923.apply(v2.clone())
                    };
                    match m2924 {
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
                Ok(next) => { m2925 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f457(v0)
}

pub fn f456(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f457(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h2928, _) = v0.clone();
        h2928
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h2929, _) = {
            let (_, h2930) = v0.clone();
            h2930
        };
        h2929
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h2931) = {
            let (_, h2932) = v0;
            h2932
        };
        h2931
    };
    let m2933 = v1.clone();
    match m2933.uncons() {
        None => {
            let m2934 = v2;
            match m2934.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2935 = v2.clone();
            match m2935.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f449({
                        let (h2936, _) = v8.clone();
                        h2936
                    }, {
                        let (h2937, _) = v6.clone();
                        h2937
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
