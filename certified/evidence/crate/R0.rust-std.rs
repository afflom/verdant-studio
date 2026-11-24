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
    C2,
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
    F56(u8, Bytes),
    F176(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F56(k0, k1) => f56(*k0, k1.clone(), p0),
            Fn0::F176(k0) => f176(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F154,
    F172,
    F240,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F154 => f154(p0, p1),
            Fn1::F172 => f172(p0, p1),
            Fn1::F240 => f240(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F207(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F207(k0) => f207(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F229(Bytes, u64, Bytes, u64),
    F321(Bytes),
    F322(u64, Bytes),
    F388(Bytes),
    F445(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F229(k0, k1, k2, k3) => f229(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F321(k0) => Ok(f321(k0.clone(), p0)),
            Fn3::F322(k0, k1) => f322(*k0, k1.clone(), p0),
            Fn3::F388(k0) => f388(k0.clone(), p0),
            Fn3::F445(k0) => f445(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F273(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt22)) -> R<(u64, Adt22)> {
        match self {
            Fn4::F273(k0, k1, k2) => f273(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F302,
}

impl Fn5 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn5::F302 => f302(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F305,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn6::F305 => f305(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F307,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn7::F307 => f307(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F310,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn8::F310 => f310(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F339(Bytes),
    F401(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F339(k0) => f339(k0.clone(), p0),
            Fn9::F401(k0, k1) => Ok(f401(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F341,
    F496,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F341 => f341(p0, p1),
            Fn10::F496 => f496(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F344,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F344 => f344(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F347,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F347 => f347(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F350,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F350 => f350(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F355,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn14::F355 => f355(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F360(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F360(k0) => f360(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F391(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F391(k0) => f391(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F405,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F405 => f405(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F407,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F407 => f407(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F409,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F409 => f409(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F417,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F417 => f417(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F437,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F437 => f437(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F472,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F472 => f472(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F474,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F474 => f474(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F476(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F476(k0) => f476(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F495,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F495 => f495(p0),
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
                                                f43(v1.clone(), v4.clone(), f44(v1, v4)?)
                                            } else {
                                                if f36(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                    f43(v1.clone(), v4.clone(), f45(v1, v4, false)?)
                                                } else {
                                                    if f36(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                        f43(v1.clone(), v4.clone(), f45(v1, v4, true)?)
                                                    } else {
                                                        if f36(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                            f43(v1.clone(), v4.clone(), f46(v1, v4, false)?)
                                                        } else {
                                                            if f36(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                f43(v1.clone(), v4.clone(), f46(v1, v4, true)?)
                                                            } else {
                                                                if f36(v2.clone(), f8(Str::lit("admit"))) {
                                                                    f47(v1, v4)
                                                                } else {
                                                                    if f36(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                        f43(v1.clone(), v4.clone(), f48(v1, v4)?)
                                                                    } else {
                                                                        if f36(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                            f49(v1, v4)
                                                                        } else {
                                                                            if f36(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                f50(v1, v4)
                                                                            } else {
                                                                                if f36(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                    f51(v1, v4)
                                                                                } else {
                                                                                    if f36(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                        f52(v1, v4)
                                                                                    } else {
                                                                                        if f36(v2, f8(Str::lit("propose-encoding"))) {
                                                                                            let m179 = v5;
                                                                                            match m179.uncons() {
                                                                                                None => {
                                                                                                    Ok(Adt11::C4(f35()))
                                                                                                }
                                                                                                Some((v6, v7)) => {
                                                                                                    f53(v1, v4, v6, v7)
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
        let a226 = f54();
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
        let (h230, _) = f55(Fn0::F56(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h230
    };
    let v4: u64 = {
        let (h231, _) = {
            let (_, h232) = v3.clone();
            h232
        };
        h231
    };
    Ok(f57(List::cons(f32(v0, v4, {
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
                        if f58(v1.clone(), 0u64, {
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
    let m253 = f59(v0.clone(), f60());
    match m253.uncons() {
        None => {
            let m254 = f61(v0.clone(), f60())?;
            match m254.uncons() {
                None => {
                    let m257 = f62(match v0.clone() {
                        Adt1::C0(_, h255, _) => {
                            h255
                        }
                    }, f63(f64(), f65(match v0.clone() {
                        Adt1::C0(_, h256, _) => {
                            h256
                        }
                    }, false)));
                    match m257.uncons() {
                        None => {
                            let m259 = f59(v0.clone(), f65(match v0.clone() {
                                Adt1::C0(_, h258, _) => {
                                    h258
                                }
                            }, true));
                            match m259.uncons() {
                                None => {
                                    let m261 = f66(v0.clone(), f65(match v0.clone() {
                                        Adt1::C0(_, h260, _) => {
                                            h260
                                        }
                                    }, true))?;
                                    match m261 {
                                        None => {
                                            let v10: (bool, Bytes) = f67(Adt13::C0(Adt12::C0(f68(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f70(v0.clone())?, f71(f72(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f68(f69(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f71(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f72(v0.clone(), f8(Str::lit("model/strata.json")))?, f73(f72(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f65(match v0 {
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
    if f74(f75(match v0.clone() {
        Adt1::C0(_, h266, _) => {
            h266
        }
    }, f76())) {
        Ok(Adt11::C1(List::cons(f76(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f77(f78(f76(), f79(match v0.clone() {
            Adt1::C0(_, h267, _) => {
                h267
            }
        }, f76())))?;
        let m268 = f59(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m268.uncons() {
            None => {
                let m269 = f80(v0.clone(), f81(f82(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f83(v2, v1.clone()))?;
                match m269 {
                    None => {
                        f84(v0, v1)
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
    if f74(f85(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h270) => {
            h270
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f86(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f74(f85(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h271) => {
                h271
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f86(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f40(v0: Adt1) -> R<Adt11> {
    let m272 = f87(v0.clone());
    match m272.uncons() {
        None => {
            let m273 = f63(f59(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f88(), List::<Bytes>::nil())))), f89(v0.clone())?);
            match m273.uncons() {
                None => {
                    Ok(Adt11::C3(f90(f91(v0.clone())?, List::cons((f92(), f93()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f94()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f95()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f96()), List::cons((f8(Str::lit("docs/catalog.md")), f97(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f98(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f99(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f100()), List::<(Bytes, Bytes)>::nil()))))))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m274 = f87(v0.clone());
    match m274.uncons() {
        None => {
            let m275 = f63(f59(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f89(v0.clone())?);
            match m275.uncons() {
                None => {
                    let v5: Adt15 = f101(v0.clone())?;
                    let v6: List<Adt14> = f72(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f63(f102(f103(match v5.clone() {
                        Adt15::C0(h276, _, _) => {
                            h276
                        }
                    }, f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f105(v6.clone())), f106(f103(match v5.clone() {
                        Adt15::C0(h277, _, _) => {
                            h277
                        }
                    }, f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f107(v5.clone(), v6, f108(v5, f109(v0.clone(), f110(v0)?)?))?)))))
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
    let m278 = f59(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m278.uncons() {
        None => {
            let v4: Bytes = f69(v0.clone(), v1);
            let v5: List<List<Bytes>> = f111(v4.clone())?;
            if f74(f112(f113(v5.clone()), f114())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f115(Str::lit("id"), v6.clone())?, List::cons(f115(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f115(Str::lit("artifact"), f116())?, List::cons(f115(Str::lit("checksum"), f117(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f116(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f118(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f119(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f120(f121(f122(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f43(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m279 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m279.uncons() {
        None => {
            if f123(f82(f69(v0, v1))?)? {
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

pub fn f44(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m280 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m280.uncons() {
        None => {
            let m281 = f124(f82(f69(v0, v1))?)?;
            match m281 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f120(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f45(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m282 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m282.uncons() {
        None => {
            let v5: Bytes = f125(if v2 {
                f126()
            } else {
                f127()
            }, f128(v1.clone())?);
            if f74(f129(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m283 = f130(f82(f69(v0.clone(), v1.clone()))?, v2)?;
                match m283 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a285 = {
                            let a284 = v6;
                            length_list(a284)
                        };
                        let a286 = 0u64;
                        if nat_eq(a285, a286) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f69(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f46(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m287 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m287.uncons() {
        None => {
            let v5: List<Bytes> = f82(f69(v0.clone(), v1.clone()))?;
            if f74(f129(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f131(v5)?
                } else {
                    f132(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f125(f133(), f128(v1.clone())?), f69(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f125(f133(), f128(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f134()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f47(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m288 = f59(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m288.uncons() {
        None => {
            let v4: List<Bytes> = f82(f69(v0.clone(), v1))?;
            let m289 = f135(v4.clone(), f8(Str::lit("requirement")))?;
            match m289 {
                None => {
                    Ok(Adt11::C4(f134()))
                }
                Some(v5) => {
                    let m290 = f135(v4, f8(Str::lit("admission")))?;
                    match m290 {
                        None => {
                            Ok(Adt11::C4(f134()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f72(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f72(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f63(f71(v5.clone(), f8(Str::lit("id"))), f71(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f77(f63(f71(v7.clone(), f8(Str::lit("id"))), f71(v8.clone(), f8(Str::lit("construct")))))?;
                            let m291 = f136(f105(v6.clone()), f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m291 {
                                None => {
                                    let a293 = {
                                        let a292 = v9.clone();
                                        length_list(a292)
                                    };
                                    let a294 = 0u64;
                                    if nat_eq(a293, a294) {
                                        Ok(Adt11::C4(f134()))
                                    } else {
                                        if f74({
                                            let a297 = {
                                                let a295 = f77(f63(v9.clone(), v10.clone()))?;
                                                length_list(a295)
                                            };
                                            let a298 = {
                                                let a296 = f63(v9.clone(), v10);
                                                length_list(a296)
                                            };
                                            nat_eq(a297, a298)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f138(f8(Str::lit("model/requirements.json")), f139(v7, v5))?, List::cons(f138(f8(Str::lit("model/admissions.json")), f139(v8, v6))?, List::cons(f119(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f120(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f137(v11))
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

pub fn f48(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m299 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m299.uncons() {
        None => {
            let m300 = f140(f82(f69(v0, v1))?)?;
            match m300 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f120(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f49(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m301 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m301.uncons() {
        None => {
            let m302 = f141(v0.clone(), v1.clone())?;
            match m302 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m303 = f142(v4.clone(), f8(Str::lit("schema")));
                    match m303 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f143(v0, f144(v5.clone())?, v5, f145(v4.clone(), f8(Str::lit("name"))), f73(f146(f142(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
    let m304 = f59(v0.clone(), List::cons(v1.clone(), List::cons(f88(), List::<Bytes>::nil())));
    match m304.uncons() {
        None => {
            let m305 = f147(f82(f69(v0.clone(), v1))?)?;
            match m305 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f72(v0, f88())?;
                    let v6: List<Bytes> = f71(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f77(f71(v5.clone(), f8(Str::lit("at"))))?;
                    let a307 = {
                        let a306 = v6.clone();
                        length_list(a306)
                    };
                    let a308 = 0u64;
                    if if nat_eq(a307, a308) {
                        true
                    } else {
                        f74({
                            let a311 = {
                                let a309 = f77(f63(v6.clone(), v7.clone()))?;
                                length_list(a309)
                            };
                            let a312 = {
                                let a310 = f63(v6.clone(), v7);
                                length_list(a310)
                            };
                            nat_eq(a311, a312)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f138(f88(), f139(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f120(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m313 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m313.uncons() {
        None => {
            let m314 = f141(v0, v1)?;
            match m314 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f148(f146(Some(v4))) {
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

pub fn f52(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m315 = f59(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m315.uncons() {
        None => {
            let m316 = f141(v0.clone(), v1.clone())?;
            match m316 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f74(f36(f145(v4.clone(), f8(Str::lit("schema"))), f149())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f145(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m317 = f142(v4.clone(), f8(Str::lit("source")));
                            match m317 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f150(f142(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f125(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f125(v9.clone(), f8(Str::lit("binding.json"))), f69(v0.clone(), v1));
                        let m318 = v8;
                        match m318 {
                            None => {
                                f151(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f74(f36(f145(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f151(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f125(v9, f128(f145(v7.clone(), f8(Str::lit("url"))))?);
                                    if f74(f85(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h319) => {
                                            h319
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f145(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m320 = f59(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m320.uncons() {
                                            None => {
                                                if f36(f117(f69(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f151(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f53(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m321 = f59(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m321.uncons() {
        None => {
            let m322 = f141(v0.clone(), v1.clone())?;
            match m322 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f143(v0.clone(), f69(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f54() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f55(v0: Fn0, p327: u64, p328: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m326 = (p327, p328);
    {
        let mut m326 = m326;
        loop {
            let step = {
                let (v1, v2) = m326;
                let m323 = v1;
                if m323 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m323 - 1;
                    let m325 = {
                        let c324: Fn0 = v0.clone();
                        c324.apply(v2.clone())?
                    };
                    match m325 {
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
                Ok(next) => { m326 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f152(v1, v0, v2)
}

pub fn f57(v0: List<Bytes>) -> List<Bytes> {
    f153(Fn1::F154, List::<Bytes>::nil(), v0)
}

pub fn f58(v0: Bytes, p384: u64, p385: u64) -> R<bool> {
    let m383 = (p384, p385);
    {
        let mut m383 = m383;
        loop {
            let step = {
                let (v1, v2) = m383;
                let m329 = v2;
                if m329 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m329 - 1;
                    let m332 = {
                        let a330 = v0.clone();
                        let a331 = v1;
                        index_bytes(a330, a331)
                    };
                    match m332 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a379 = {
                                let v5: u8 = v4;
                                let a377 = {
                                    let a353 = {
                                        let a341 = {
                                            let a335 = {
                                                let a333 = v5;
                                                let a334 = 128u8;
                                                fixed_u8::bit_and(a333, a334)
                                            };
                                            let a336 = 128u8;
                                            if equal(a335, a336) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a342 = {
                                            let a339 = {
                                                let a337 = v5;
                                                let a338 = 64u8;
                                                fixed_u8::bit_and(a337, a338)
                                            };
                                            let a340 = 64u8;
                                            if equal(a339, a340) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a341, a342)?
                                    };
                                    let a354 = {
                                        let a351 = {
                                            let a345 = {
                                                let a343 = v5;
                                                let a344 = 32u8;
                                                fixed_u8::bit_and(a343, a344)
                                            };
                                            let a346 = 32u8;
                                            if equal(a345, a346) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a352 = {
                                            let a349 = {
                                                let a347 = v5;
                                                let a348 = 16u8;
                                                fixed_u8::bit_and(a347, a348)
                                            };
                                            let a350 = 16u8;
                                            if equal(a349, a350) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a351, a352)?
                                    };
                                    nat_add(a353, a354)?
                                };
                                let a378 = {
                                    let a375 = {
                                        let a363 = {
                                            let a357 = {
                                                let a355 = v5;
                                                let a356 = 8u8;
                                                fixed_u8::bit_and(a355, a356)
                                            };
                                            let a358 = 8u8;
                                            if equal(a357, a358) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a364 = {
                                            let a361 = {
                                                let a359 = v5;
                                                let a360 = 4u8;
                                                fixed_u8::bit_and(a359, a360)
                                            };
                                            let a362 = 4u8;
                                            if equal(a361, a362) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a363, a364)?
                                    };
                                    let a376 = {
                                        let a373 = {
                                            let a367 = {
                                                let a365 = v5;
                                                let a366 = 2u8;
                                                fixed_u8::bit_and(a365, a366)
                                            };
                                            let a368 = 2u8;
                                            if equal(a367, a368) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a374 = {
                                            let a371 = {
                                                let a369 = v5;
                                                let a370 = 1u8;
                                                fixed_u8::bit_and(a369, a370)
                                            };
                                            let a372 = 1u8;
                                            if equal(a371, a372) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a373, a374)?
                                    };
                                    nat_add(a375, a376)?
                                };
                                nat_add(a377, a378)?
                            };
                            let a380 = 127u64;
                            if nat_le(a379, a380) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a381 = v1;
                                    let a382 = 1u64;
                                    nat_add(a381, a382)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m386 = v1;
    match m386.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m388 = f155(match v0.clone() {
                Adt1::C0(h387, _, _) => {
                    h387
                }
            }, v2.clone());
            match m388 {
                None => {
                    List::cons(v2, f59(v0, v3))
                }
                Some(_) => {
                    f59(v0, v3)
                }
            }
        }
    }
}

pub fn f60() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f116(), List::<Bytes>::nil())))))))
}

pub fn f61(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m389 = v1;
    match m389.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f156(v0.clone(), v2.clone()) {
                f129(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m390 = f141(v0.clone(), v2.clone())?;
                match m390 {
                    None => {
                        Ok(List::cons(v2, f61(v0, v3)?))
                    }
                    Some(_) => {
                        f61(v0, v3)
                    }
                }
            } else {
                f61(v0, v3)
            }
        }
    }
}

pub fn f62(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m391 = v1;
    match m391.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f75(v0.clone(), v2.clone()) {
                f62(v0, v3)
            } else {
                List::cons(v2, f62(v0, v3))
            }
        }
    }
}

pub fn f63(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m392 = v0;
    match m392.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f63(v3, v1))
        }
    }
}

pub fn f64() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f65(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m393 = v0;
    match m393.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f63(f157({
                let (h394, _) = v2;
                h394
            }, {
                let (_, h395) = v2;
                h395
            }, v1), f65(v3, v1))
        }
    }
}

pub fn f66(v0: Adt1, p399: List<Bytes>) -> R<Option<Bytes>> {
    let m398 = p399;
    {
        let mut m398 = m398;
        loop {
            let step = {
                let v1 = m398;
                let m396 = v1;
                match m396.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m397 = f158(f69(v0.clone(), v2.clone()), f159())?;
                        match m397 {
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
                Ok(next) => { m398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f67(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h400, _, _, _, _) => {
            h400
        }
    };
    let v2: Adt16 = f160(v1.clone())?;
    let v3: List<Bytes> = f161(v1.clone());
    let v4: List<Bytes> = f71(match v0.clone() {
        Adt13::C0(_, _, h401, _, _) => {
            h401
        }
    }, f8(Str::lit("path")));
    let m403 = f162(v2, List::<u64>::nil(), f163(v1.clone(), match v0.clone() {
        Adt13::C0(_, h402, _, _, _) => {
            h402
        }
    }, 0u64)?);
    match m403 {
        None => {
            if f165(f166(match v0.clone() {
                Adt13::C0(_, _, _, _, h405) => {
                    h405
                }
            }, f63(match v0.clone() {
                Adt13::C0(_, _, _, _, h406) => {
                    h406
                }
            }, v4.clone()))?, f166(v4.clone(), f63(match v0.clone() {
                Adt13::C0(_, _, _, _, h407) => {
                    h407
                }
            }, v4.clone()))?, f167(match v0.clone() {
                Adt13::C0(_, _, h408, _, _) => {
                    h408
                }
            })) {
                let m412 = f168(f166(match v0.clone() {
                    Adt13::C0(_, _, _, h409, _) => {
                        h409
                    }
                }, f63(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h410, _) => {
                        h410
                    }
                }))?, f166(v3.clone(), f63(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h411, _) => {
                        h411
                    }
                }))?);
                match m412 {
                    None => {
                        let m415 = f169(f71(match v0 {
                            Adt13::C0(_, h414, _, _, _) => {
                                h414
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m415 {
                            None => {
                                let m416 = f169(v4, List::<Bytes>::nil());
                                match m416 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f120(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f164(f63(v3, match v0 {
                            Adt13::C0(_, _, _, h413, _) => {
                                h413
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f164(match v1 {
                Adt12::C0(_, _, _, _, h404) => {
                    h404
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f68(v0: Bytes) -> R<List<Bytes>> {
    f170(f82(v0)?, f8(Str::lit("id = \"")))
}

pub fn f69(v0: Adt1, v1: Bytes) -> Bytes {
    let m418 = f155(match v0 {
        Adt1::C0(h417, _, _) => {
            h417
        }
    }, v1);
    match m418 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m419 = v2;
            match m419 {
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

pub fn f70(v0: Adt1) -> R<List<Bytes>> {
    let m420 = f111(f69(v0, f116()))?;
    match m420.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f121(v2))
        }
    }
}

pub fn f71(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m421 = v0;
    match m421.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f145(v2, v1.clone()), f71(v3, v1))
        }
    }
}

pub fn f72(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f146(f141(v0, v1)?))
}

pub fn f73(v0: List<Adt14>) -> List<Bytes> {
    let m422 = v0;
    match m422.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m423 = f150(Some(v1));
            match m423 {
                None => {
                    f73(v2)
                }
                Some(v3) => {
                    List::cons(v3, f73(v2))
                }
            }
        }
    }
}

pub fn f74(v0: bool) -> bool {
    !v0
}

pub fn f75(p427: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m426 = p427;
    {
        let mut m426 = m426;
        loop {
            let step = {
                let v0 = m426;
                let m424 = v0;
                match m424.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h425, _) = v2;
                            h425
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f77(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f171(f153(Fn1::F172, List::<Bytes>::nil(), f173(v0)?)))
}

pub fn f78(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m428 = v1;
    match m428.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f129(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f125(v0.clone(), v2), f78(v0, v3))
            } else {
                f78(v0, v3)
            }
        }
    }
}

pub fn f79(p433: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m432 = p433;
    {
        let mut m432 = m432;
        loop {
            let step = {
                let v0 = m432;
                let m429 = v0;
                match m429.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h430, _) = v2;
                            h430
                        }, v1.clone()) {
                            let (_, h431) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h431)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Adt1, v1: List<List<Bytes>>, p436: List<Bytes>) -> R<Option<Bytes>> {
    let m435 = p436;
    {
        let mut m435 = m435;
        loop {
            let step = {
                let v2 = m435;
                let m434 = v2;
                match m434.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f174(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(p439: List<Bytes>, p440: List<Bytes>, p441: List<List<Bytes>>) -> List<List<Bytes>> {
    let m438 = (p439, (p440, p441));
    {
        let mut m438 = m438;
        loop {
            let step = {
                let (v0, (v1, v2)) = m438;
                let m437 = v0;
                match m437.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f175(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f36(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f175(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m438 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a442 = v0.clone();
        length_bytes(a442)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h443, _) = f55(Fn0::F176(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h443
    };
    let v3: u64 = {
        let (h444, _) = {
            let (_, h445) = v2.clone();
            h445
        };
        h444
    };
    let v4: List<Bytes> = {
        let (_, h446) = {
            let (_, h447) = v2;
            h447
        };
        h446
    };
    let a448 = v3;
    let a449 = v1;
    if nat_lt(a448, a449) {
        Ok(f57(List::cons(f32(v0, v3, {
            let a450 = v1;
            let a451 = v3;
            nat_sub(a450, a451)
        }), v4)))
    } else {
        Ok(f57(v4))
    }
}

pub fn f83(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m452 = v0;
    match m452.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f83(v3, v1)
            } else {
                List::cons(v2, f83(v3, v1))
            }
        }
    }
}

pub fn f84(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f74(f75(match v0.clone() {
        Adt1::C0(_, h453, _) => {
            h453
        }
    }, f177())) {
        Ok(Adt11::C1(List::cons(f177(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f83(f77(f78(f177(), f79(match v0.clone() {
            Adt1::C0(_, h454, _) => {
                h454
            }
        }, f177())))?, v1.clone());
        let m455 = f59(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m455.uncons() {
            None => {
                let v5: List<Bytes> = f71(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m456 = f59(v0.clone(), f178(v0.clone(), v2.clone(), v5.clone())?);
                match m456.uncons() {
                    None => {
                        let m457 = f179(v0.clone(), v2, v5)?;
                        match m457 {
                            None => {
                                f180(v0, v1)
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

pub fn f85(v0: Bytes, p460: List<Bytes>) -> bool {
    let m459 = p460;
    {
        let mut m459 = m459;
        loop {
            let step = {
                let v1 = m459;
                let m458 = v1;
                match m458.uncons() {
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
                Ok(next) => { m459 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f87(v0: Adt1) -> List<Bytes> {
    if f75(match v0.clone() {
        Adt1::C0(_, h461, _) => {
            h461
        }
    }, f127()) {
        if f75(match v0 {
            Adt1::C0(_, h462, _) => {
                h462
            }
        }, f126()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f126(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f127(), List::<Bytes>::nil())
    }
}

pub fn f88() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f89(v0: Adt1) -> R<List<Bytes>> {
    Ok(f59(v0.clone(), List::cons(f116(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f110(v0)?)))))
}

pub fn f90(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m463 = v0;
    match m463.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f90(v3, v1))
        }
    }
}

pub fn f91(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m464 = f72(v0.clone(), f88())?;
    match m464.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f181(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f92() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f93() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f94() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f182(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f95() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src and\u{a}# README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught when it opens, too.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  pull_request:\u{a}    types: [opened]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  refresh:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - if: steps.stale.outputs.stale != ''\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply each change to the new main, export it again, and push it to its own pull request\u{a}        if: steps.stale.outputs.stale != ''\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ steps.stale.outputs.stale }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again either way.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f182(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              gh pr ready \"$pr\" || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f96() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f97(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f101(v0.clone())?;
    let v2: List<Adt14> = f72(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f108(v1.clone(), f109(v0.clone(), f110(v0.clone())?)?);
    let v4: List<List<Bytes>> = f122(f111(f69(v0, f116()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f183(v4.clone(), f121(v4), f184(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f107(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f98(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f185(f101(v0.clone())?, f109(v0.clone(), f110(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f99(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f186(f72(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f187(f72(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f100() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f101(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f70(v0.clone())?, f68(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f71(f72(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f102(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m465 = v0;
    match m465.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f102(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f102(v3, v1))
            }
        }
    }
}

pub fn f103(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m466 = v0;
    match m466.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f103(v3, v1)
            } else {
                List::cons(v2, f103(v3, v1))
            }
        }
    }
}

pub fn f104(v0: List<Adt14>) -> List<Bytes> {
    let m467 = v0;
    match m467.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f63(f73(f146(f142(v1, f8(Str::lit("defers"))))), f104(v2))
        }
    }
}

pub fn f105(v0: List<Adt14>) -> List<Bytes> {
    let m468 = v0;
    match m468.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m469 = f142(v1, f8(Str::lit("std")));
            match m469 {
                None => {
                    f105(v2)
                }
                Some(v3) => {
                    List::cons(f145(v3, f8(Str::lit("row"))), f105(v2))
                }
            }
        }
    }
}

pub fn f106(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m470 = v0;
    match m470.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f106(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f106(v3, v1))
            }
        }
    }
}

pub fn f107(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f90(f188(v2.clone()), f189(f184(v0, v1.clone(), v2)?));
    f190(f77(f191(v1))?, v3.clone(), {
        let a472 = {
            let a471 = v3;
            length_list(a471)
        };
        let a473 = 1u64;
        nat_add(a472, a473)?
    })
}

pub fn f108(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m474 = v1;
    match m474.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f192(v2.clone(), f8(Str::lit("subject"))) {
                f193(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f108(v0, v3))
            } else {
                f108(v0, v3)
            }
        }
    }
}

pub fn f109(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m475 = v1;
    match m475.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m476 = f130(f82(f69(v0.clone(), v2.clone()))?, f194(v2, f126()))?;
            match m476 {
                None => {
                    f109(v0, v3)
                }
                Some(v4) => {
                    Ok(f139(v4, f109(v0, v3)?))
                }
            }
        }
    }
}

pub fn f110(v0: Adt1) -> R<List<Bytes>> {
    Ok(f63(f77(f195(f127(), f79(match v0.clone() {
        Adt1::C0(_, h477, _) => {
            h477
        }
    }, f127())))?, f77(f195(f126(), f79(match v0 {
        Adt1::C0(_, h478, _) => {
            h478
        }
    }, f126())))?))
}

pub fn f111(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f196(v0.clone(), 0u64, {
        let a479 = v0;
        length_bytes(a479)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a482 = {
            let a481 = match v1.clone() {
                Adt17::C0(_, _, h480, _, _) => {
                    h480
                }
            };
            length_bytes(a481)
        };
        let a483 = 0u64;
        if if nat_eq(a482, a483) {
            let m485 = match v1.clone() {
                Adt17::C0(_, _, _, h484, _) => {
                    h484
                }
            };
            match m485.uncons() {
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
            f197(v1)
        }
    };
    Ok(f198(match v4 {
        Adt17::C0(_, _, _, _, h486) => {
            h486
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f112(p491: List<Bytes>, p492: List<Bytes>) -> bool {
    let m490 = (p491, p492);
    {
        let mut m490 = m490;
        loop {
            let step = {
                let (v0, v1) = m490;
                let m487 = v0;
                match m487.uncons() {
                    None => {
                        let m488 = v1;
                        match m488.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m489 = v1;
                        match m489.uncons() {
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
                Ok(next) => { m490 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: List<List<Bytes>>) -> List<Bytes> {
    let m493 = v0;
    match m493.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f114() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f115(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f199(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f116() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f117(v0: Bytes) -> R<Bytes> {
    Ok(f200(f201(v0)?))
}

pub fn f118(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f202(f81(f82(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f203(f204(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f119(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f144(Adt14::C3(std::rc::Rc::new(f205(f77(f63(v1, f73(f72(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f120(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m494 = f206(Fn2::F207(v1), None::<Bytes>, v0);
    match m494 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f121(v0: List<List<Bytes>>) -> List<Bytes> {
    let m495 = v0;
    match m495.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m496 = v1;
            match m496.uncons() {
                None => {
                    f121(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f121(v2))
                }
            }
        }
    }
}

pub fn f122(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m497 = v0;
    match m497.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f123(p503: List<Bytes>) -> R<bool> {
    let m502 = p503;
    {
        let mut m502 = m502;
        loop {
            let step = {
                let v0 = m502;
                let m498 = v0;
                match m498.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a500 = {
                            let a499 = f208(v1)?;
                            length_bytes(a499)
                        };
                        let a501 = 0u64;
                        if nat_eq(a500, a501) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m504 = v0;
    match m504.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f208(v1)?;
            let a506 = {
                let a505 = v3.clone();
                length_bytes(a505)
            };
            let a507 = 0u64;
            if nat_eq(a506, a507) {
                f124(v2)
            } else {
                let m508 = f209(v3.clone())?;
                match m508 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f145(v4.clone(), f8(Str::lit("id")));
                        if if f36(f145(v4, f8(Str::lit("schema"))), f149()) {
                            f74({
                                let a510 = {
                                    let a509 = v5.clone();
                                    length_bytes(a509)
                                };
                                let a511 = 0u64;
                                nat_eq(a510, a511)
                            })
                        } else {
                            false
                        } {
                            let m512 = f124(v2)?;
                            match m512 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f210(v5), {
                                        let a513 = v3;
                                        let a514 = Bytes::lit(&[0x0a]);
                                        append_bytes(a513, a514)
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

pub fn f125(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f126() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f127() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f128(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f129(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a515 = v0.clone();
        length_bytes(a515)
    };
    let v3: u64 = {
        let a516 = v1.clone();
        length_bytes(a516)
    };
    let a517 = v3;
    let a518 = v2;
    if nat_le(a517, a518) {
        let m524 = {
            let a521 = v0;
            let a522 = {
                let a519 = v2;
                let a520 = v3;
                nat_sub(a519, a520)
            };
            let a523 = v3;
            slice_bytes(a521, a522, a523)
        };
        match m524 {
            None => {
                false
            }
            Some(v4) => {
                let a525 = v4;
                let a526 = v1;
                equal(a525, a526)
            }
        }
    } else {
        false
    }
}

pub fn f130(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m527 = v0;
    match m527.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f208(v2)?;
            let a529 = {
                let a528 = v4.clone();
                length_bytes(a528)
            };
            let a530 = 0u64;
            if nat_eq(a529, a530) {
                f130(v3, v1)
            } else {
                let m531 = f209(v4)?;
                match m531 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f211(v5.clone())
                        } else {
                            f212(v5.clone())
                        } {
                            let m532 = f130(v3, v1)?;
                            match m532 {
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

pub fn f131(v0: List<Bytes>) -> R<bool> {
    if f213(f147(v0.clone())?) {
        Ok(f74({
            let a534 = {
                let a533 = f214(f147(v0)?);
                length_list(a533)
            };
            let a535 = 0u64;
            nat_eq(a534, a535)
        }))
    } else {
        Ok(false)
    }
}

pub fn f132(v0: List<Bytes>) -> R<bool> {
    if f213(f135(v0.clone(), f8(Str::lit("requirement")))?) {
        if f213(f135(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f74({
                let a537 = {
                    let a536 = f139(f214(f135(v0.clone(), f8(Str::lit("requirement")))?), f214(f135(v0, f8(Str::lit("admission")))?));
                    length_list(a536)
                };
                let a538 = 0u64;
                nat_eq(a537, a538)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f133() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f134() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f135(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m539 = v0;
    match m539.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f208(v2)?;
            let a541 = {
                let a540 = v4.clone();
                length_bytes(a540)
            };
            let a542 = 0u64;
            if nat_eq(a541, a542) {
                f135(v3, v1)
            } else {
                let m543 = f209(v4)?;
                match m543 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f215(v5.clone());
                        let a545 = {
                            let a544 = v6.clone();
                            length_bytes(a544)
                        };
                        let a546 = 0u64;
                        if nat_eq(a545, a546) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m547 = f135(v3, v1.clone())?;
                            match m547 {
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

pub fn f136(p550: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m549 = p550;
    {
        let mut m549 = m549;
        loop {
            let step = {
                let v0 = m549;
                let m548 = v0;
                match m548.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f85(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m549 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f138(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f144(Adt14::C3(std::rc::Rc::new(f216(v1))))?))
}

pub fn f139(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m551 = v0;
    match m551.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f139(v3, v1))
        }
    }
}

pub fn f140(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m552 = v0;
    match m552.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f208(v1)?;
            let a554 = {
                let a553 = v3.clone();
                length_bytes(a553)
            };
            let a555 = 0u64;
            if nat_eq(a554, a555) {
                f140(v2)
            } else {
                let m556 = f209(v3.clone())?;
                match m556 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f217(v4.clone()) {
                            let m557 = f140(v2)?;
                            match m557 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f177(), List::cons(f8(Str::lit("/")), List::cons(f218(f145(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a558 = v3;
                                        let a559 = Bytes::lit(&[0x0a]);
                                        append_bytes(a558, a559)
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

pub fn f141(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f209(f69(v0, v1))
}

pub fn f142(p565: Adt14, v1: Bytes) -> Option<Adt14> {
    let m564 = p565;
    {
        let mut m564 = m564;
        loop {
            let step = {
                let v0 = m564;
                let m560 = v0;
                match m560 {
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
                    Adt14::C4(r561) => {
                        let v5 = (*r561).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r562, r563) => {
                        let v3 = (*r562).clone();
                        let v4 = (*r563).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m564 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m566 = f59(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m566.uncons() {
        None => {
            let m568 = f136(f105(List::cons(Adt14::C4(std::rc::Rc::new({
                let m567 = f219(v4.clone());
                match m567 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m568 {
                None => {
                    f220(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f137(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f144(v0: Adt14) -> R<Bytes> {
    let a569 = f221(v0)?;
    let a570 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a569, a570))
}

pub fn f145(v0: Adt14, v1: Bytes) -> Bytes {
    let m571 = f150(f142(v0, v1));
    match m571 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f146(v0: Option<Adt14>) -> List<Adt14> {
    let m572 = v0;
    match m572 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f222(v1)
        }
    }
}

pub fn f147(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m573 = v0;
    match m573.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f208(v1)?;
            let a575 = {
                let a574 = v3.clone();
                length_bytes(a574)
            };
            let a576 = 0u64;
            if nat_eq(a575, a576) {
                f147(v2)
            } else {
                let m577 = f209(v3)?;
                match m577 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f192(v4.clone(), f8(Str::lit("agent"))) {
                            if f192(v4.clone(), f8(Str::lit("at"))) {
                                if f192(v4.clone(), f8(Str::lit("quote"))) {
                                    f223(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m578 = f147(v2)?;
                            match m578 {
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

pub fn f148(p581: List<Adt14>) -> bool {
    let m580 = p581;
    {
        let mut m580 = m580;
        loop {
            let step = {
                let v0 = m580;
                let m579 = v0;
                match m579.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f145(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f85(f8(Str::lit("studio")), f71(f224(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m580 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f150(v0: Option<Adt14>) -> Option<Bytes> {
    let m582 = v0;
    match m582 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m583 = v1;
            match m583 {
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

pub fn f151(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f118(f69(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f225(v1, v4)?)?), List::cons(f119(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f152(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h584, _) = v2.clone();
        h584
    };
    let v4: u64 = {
        let (h585, _) = {
            let (_, h586) = v2.clone();
            h586
        };
        h585
    };
    let v5: List<Bytes> = {
        let (_, h587) = {
            let (_, h588) = v2;
            h588
        };
        h587
    };
    let m589 = f226(v0.clone(), v3);
    match m589 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a590 = v6;
            let a591 = v1;
            if equal(a590, a591) {
                Ok(Some(({
                    let a592 = v3;
                    let a593 = 1u64;
                    nat_add(a592, a593)?
                }, ({
                    let a594 = v3;
                    let a595 = 1u64;
                    nat_add(a594, a595)?
                }, List::cons(f32(v0, v4, {
                    let a596 = v3;
                    let a597 = v4;
                    nat_sub(a596, a597)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a598 = v3;
                    let a599 = 1u64;
                    nat_add(a598, a599)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f153(v0: Fn1, p603: List<Bytes>, p604: List<Bytes>) -> List<Bytes> {
    let m602 = (p603, p604);
    {
        let mut m602 = m602;
        loop {
            let step = {
                let (v1, v2) = m602;
                let m600 = v2;
                match m600.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c601: Fn1 = v0.clone();
                            c601.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m602 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f155(p609: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m608 = p609;
    {
        let mut m608 = m608;
        loop {
            let step = {
                let v0 = m608;
                let m605 = v0;
                match m605.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h606, _) = v2;
                            h606
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h607) = v2;
                                h607
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m608 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Adt1, v1: Bytes) -> bool {
    let m611 = f155(match v0 {
        Adt1::C0(h610, _, _) => {
            h610
        }
    }, v1);
    match m611 {
        None => {
            false
        }
        Some(v2) => {
            let m612 = v2;
            match m612 {
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

pub fn f157(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m613 = v1;
    match m613.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f227(v3.clone())
            } else {
                f74(f227(v3.clone()))
            } {
                List::cons(f125(v0.clone(), v3), f157(v0, v4, v2))
            } else {
                f157(v0, v4, v2)
            }
        }
    }
}

pub fn f158(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a614 = v0.clone();
        length_bytes(a614)
    };
    let v3: u64 = {
        let a615 = v1.clone();
        length_bytes(a615)
    };
    let v4: u64 = {
        let (h616, _) = f228(Fn3::F229(v1.clone(), v3, v0.clone(), v2), {
            let a617 = v2;
            let a618 = 1u64;
            nat_add(a617, a618)?
        }, 0u64)?;
        h616
    };
    if f230(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f159() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f160(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f231(match v0.clone() {
        Adt12::C0(h619, _, _, _, _) => {
            h619
        }
    }, 0u64)?, f231(match v0.clone() {
        Adt12::C0(_, h620, _, _, _) => {
            h620
        }
    }, 0u64)?, f231(match v0.clone() {
        Adt12::C0(_, _, h621, _, _) => {
            h621
        }
    }, 0u64)?, f231(match v0 {
        Adt12::C0(_, _, _, h622, _) => {
            h622
        }
    }, 0u64)?))
}

pub fn f161(v0: Adt12) -> List<Bytes> {
    f63(match v0.clone() {
        Adt12::C0(h623, _, _, _, _) => {
            h623
        }
    }, f63(match v0.clone() {
        Adt12::C0(_, _, h624, _, _) => {
            h624
        }
    }, f63(match v0.clone() {
        Adt12::C0(_, _, _, h625, _) => {
            h625
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h626) => {
            h626
        }
    })))
}

pub fn f162(v0: Adt16, p632: List<u64>, p633: List<Adt18>) -> Option<u64> {
    let m631 = (p632, p633);
    {
        let mut m631 = m631;
        loop {
            let step = {
                let (v1, v2) = m631;
                let m627 = v2;
                match m627.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt18>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f232(v0.clone(), v1.clone(), match v3.clone() {
                            Adt18::C0(_, h628) => {
                                h628
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt18>), Option<u64>>((List::cons(match v3 {
                                Adt18::C0(h629, _) => {
                                    h629
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt18>), Option<u64>>(Some(match v3 {
                                Adt18::C0(h630, _) => {
                                    h630
                                }
                            }))
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

pub fn f163(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt18>> {
    let m634 = v1;
    match m634.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt18::C0(v2, f233(v0.clone(), v3)?), f163(v0, v4, {
                let a635 = v2;
                let a636 = 1u64;
                nat_add(a635, a636)?
            })?))
        }
    }
}

pub fn f164(p640: List<Bytes>, p641: u64) -> Bytes {
    let m639 = (p640, p641);
    {
        let mut m639 = m639;
        loop {
            let step = {
                let (v0, v1) = m639;
                let m637 = v0;
                match m637.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m638 = v1;
                        if m638 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m638 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m639 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f234(v0.clone(), v1.clone()) {
        if f234(v1, v0) {
            f235(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f166(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m642 = v0;
    match m642.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f236(v2, v1.clone())?, f166(v3, v1)?))
        }
    }
}

pub fn f167(v0: List<Adt14>) -> List<u64> {
    let m643 = v0;
    match m643.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f237(v1), f167(v2))
        }
    }
}

pub fn f168(p646: List<u64>, v1: List<u64>) -> Option<u64> {
    let m645 = p646;
    {
        let mut m645 = m645;
        loop {
            let step = {
                let v0 = m645;
                let m644 = v0;
                match m644.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f238(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
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

pub fn f169(p649: List<Bytes>, p650: List<Bytes>) -> Option<Bytes> {
    let m648 = (p649, p650);
    {
        let mut m648 = m648;
        loop {
            let step = {
                let (v0, v1) = m648;
                let m647 = v0;
                match m647.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f85(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
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

pub fn f170(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m651 = v0;
    match m651.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m652 = f239(v2, v1.clone())?;
            match m652 {
                None => {
                    f170(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f170(v3, v1)?))
                }
            }
        }
    }
}

pub fn f171(v0: List<Bytes>) -> List<Bytes> {
    f153(Fn1::F240, List::<Bytes>::nil(), v0)
}

pub fn f172(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f241(v0, v1)
}

pub fn f173(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f242(f243(f244(v0))?))
}

pub fn f174(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m653 = f141(v0, v2)?;
    match m653 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f245(v1, f145(v3.clone(), f8(Str::lit("id"))))?;
            let m654 = v4.clone();
            match m654.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m656 = f150(f142({
                        let m655 = f142(v3.clone(), f8(Str::lit("source")));
                        match m655 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m656 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f74(f85(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f199(f246(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f175(p659: List<Bytes>, p660: List<Bytes>) -> List<Bytes> {
    let m658 = (p659, p660);
    {
        let mut m658 = m658;
        loop {
            let step = {
                let (v0, v1) = m658;
                let m657 = v0;
                match m657.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f247(v0, v1)
}

pub fn f177() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f178(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m661 = v1;
    match m661.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt14 = f248(v0.clone(), v3)?;
            if f85(f249(f145(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f250(v5)?, f178(v0, v4, v2)?))
            } else {
                f178(v0, v4, v2)
            }
        }
    }
}

pub fn f179(v0: Adt1, p665: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
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
                    Some((v3, v4)) => {
                        let v5: Adt14 = f248(v0.clone(), v3.clone())?;
                        if if f85(f249(f145(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f36(f144({
                                let m663 = f142(v5.clone(), f8(Str::lit("schema")));
                                match m663 {
                                    None => {
                                        Adt14::C5
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f69(v0.clone(), f250(v5)?))
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
                Ok(next) => { m664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f74(f75(match v0.clone() {
        Adt1::C0(_, h666, _) => {
            h666
        }
    }, f133())) {
        Ok(Adt11::C1(List::cons(f133(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f83(f77(f251(f79(match v0.clone() {
            Adt1::C0(_, h667, _) => {
                h667
            }
        }, f133())))?, v1);
        let m668 = f59(v0.clone(), List::cons(f88(), List::cons(f116(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m668.uncons() {
            None => {
                let v5: (bool, Bytes) = f252(v0.clone(), v2)?;
                let (h669, _) = v5;
                if h669 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h670) = v5;
                        h670
                    }))
                } else {
                    let m671 = f87(v0.clone());
                    match m671.uncons() {
                        None => {
                            let m672 = f89(v0.clone())?;
                            match m672.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h673) = v5;
                                        h673
                                    }, List::cons(f7(f77(f253(f101(v0.clone())?, f109(v0.clone(), f110(v0.clone())?)?))?), List::cons(f254(v0)?, List::<Bytes>::nil()))))))
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

pub fn f181(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f255(f72(v0, f88())?)?)))
}

pub fn f182() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f183(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt21>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m674 = v1;
    match m674.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f63(f256(v5.clone(), v2.clone()), f257(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f258(v0.clone(), v5.clone()), List::cons(if f85(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a676 = {
                    let a675 = v7.clone();
                    length_list(a675)
                };
                let a677 = 0u64;
                if nat_eq(a676, a677) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f120(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f183(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f184(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt21>> {
    let v3: List<(Bytes, Bytes)> = f259(v1);
    let v4: List<(Bytes, Bytes)> = f260(v0, v2);
    f261(v3.clone(), f90(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f185(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m678 = v1;
    match m678.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f192(v2.clone(), f8(Str::lit("domain"))) {
                f262(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f145(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f145(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m679 = f142(v2.clone(), f8(Str::lit("role")));
                    match m679 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f145(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f145(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f263(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m680 = f142(v2.clone(), f8(Str::lit("origin")));
                    match m680 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f145(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m681 = f142(v2, f8(Str::lit("evidence")));
                    match m681 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f145(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f185(v0, v3))
            } else {
                f185(v0, v3)
            }
        }
    }
}

pub fn f186(v0: List<Adt14>) -> List<Bytes> {
    let m682 = v0;
    match m682.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f145(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f145(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f145(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f186(v2))
        }
    }
}

pub fn f187(v0: List<Adt14>) -> List<Bytes> {
    let m683 = v0;
    match m683.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f145(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f264(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f187(v2))
        }
    }
}

pub fn f188(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m684 = v0;
    match m684.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f145(v1.clone(), f8(Str::lit("subject"))), f145(v1, f8(Str::lit("object")))), f188(v2))
        }
    }
}

pub fn f189(v0: List<Adt21>) -> List<(Bytes, Bytes)> {
    let m685 = v0;
    match m685.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt21::C0(h686, _, _) => {
                    h686
                }
            }, match v1 {
                Adt21::C0(_, _, h687) => {
                    h687
                }
            }), f189(v2))
        }
    }
}

pub fn f190(p694: List<Bytes>, v1: List<(Bytes, Bytes)>, p695: u64) -> R<List<Bytes>> {
    let m693 = (p694, p695);
    {
        let mut m693 = m693;
        loop {
            let step = {
                let (v0, v2) = m693;
                let m688 = v2;
                if m688 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m688 - 1;
                    let v4: List<Bytes> = f77(f63(v0.clone(), f265(v0.clone(), v1.clone())))?;
                    let a691 = {
                        let a689 = v4.clone();
                        length_list(a689)
                    };
                    let a692 = {
                        let a690 = v0.clone();
                        length_list(a690)
                    };
                    if nat_eq(a691, a692) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m693 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: List<Adt14>) -> List<Bytes> {
    let m696 = v0;
    match m696.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m697 = f142(v1, f8(Str::lit("std")));
            match m697 {
                None => {
                    f191(v2)
                }
                Some(v3) => {
                    List::cons(f145(v3.clone(), f8(Str::lit("authority"))), if f192(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f145(v3, f8(Str::lit("row"))), f191(v2))
                    } else {
                        f191(v2)
                    })
                }
            }
        }
    }
}

pub fn f192(v0: Adt14, v1: Bytes) -> bool {
    f74({
        let a699 = {
            let a698 = f145(v0, v1);
            length_bytes(a698)
        };
        let a700 = 0u64;
        nat_eq(a699, a700)
    })
}

pub fn f193(v0: Adt15, v1: Adt14) -> bool {
    if f266(v0.clone(), v1.clone(), f145(v1.clone(), f8(Str::lit("subject")))) {
        f266(v0, v1.clone(), f145(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f194(v0: Bytes, v1: Bytes) -> bool {
    let m705 = {
        let a702 = v0;
        let a703 = 0u64;
        let a704 = {
            let a701 = v1.clone();
            length_bytes(a701)
        };
        slice_bytes(a702, a703, a704)
    };
    match m705 {
        None => {
            false
        }
        Some(v2) => {
            let a706 = v2;
            let a707 = v1;
            equal(a706, a707)
        }
    }
}

pub fn f195(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m708 = v1;
    match m708.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f129(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f125(v0.clone(), v2), f195(v0, v3))
            } else {
                f195(v0, v3)
            }
        }
    }
}

pub fn f196(v0: Bytes, p766: u64, p767: u64, p768: Adt17) -> R<Adt17> {
    let m765 = (p766, (p767, p768));
    {
        let mut m765 = m765;
        loop {
            let step = {
                let (v1, (v2, v3)) = m765;
                let m709 = v2;
                if m709 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m709 - 1;
                    let v5: u64 = f267(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h710, _, _, _) => {
                            h710
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a711 = v1;
                            let a712 = 1u64;
                            nat_add(a711, a712)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h713, _, _) => {
                                h713
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h714, _) => {
                                h714
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h715) => {
                                h715
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h716, _, _, _, _) => {
                                h716
                            }
                        } {
                            let a717 = v5;
                            let a718 = 34u64;
                            if nat_eq(a717, a718) {
                                let a721 = f267(v0.clone(), {
                                    let a719 = v1;
                                    let a720 = 1u64;
                                    nat_add(a719, a720)?
                                })?;
                                let a722 = 34u64;
                                if nat_eq(a721, a722) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a723 = v1;
                                        let a724 = 1u64;
                                        nat_add(a723, a724)?
                                    }, (v4, Adt17::C0(true, true, f268(match v3.clone() {
                                        Adt17::C0(_, _, h725, _, _) => {
                                            h725
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h726, _) => {
                                            h726
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h727) => {
                                            h727
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a728 = v1;
                                        let a729 = 1u64;
                                        nat_add(a728, a729)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h730, _, _) => {
                                            h730
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h731, _) => {
                                            h731
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h732) => {
                                            h732
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a733 = v1;
                                    let a734 = 1u64;
                                    nat_add(a733, a734)?
                                }, (v4, Adt17::C0(true, false, f268(match v3.clone() {
                                    Adt17::C0(_, _, h735, _, _) => {
                                        h735
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h736, _) => {
                                        h736
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h737) => {
                                        h737
                                    }
                                }))))
                            }
                        } else {
                            let a738 = v5;
                            let a739 = 34u64;
                            if nat_eq(a738, a739) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a740 = v1;
                                    let a741 = 1u64;
                                    nat_add(a740, a741)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h742, _, _) => {
                                        h742
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h743, _) => {
                                        h743
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h744) => {
                                        h744
                                    }
                                }))))
                            } else {
                                let a745 = v5;
                                let a746 = 44u64;
                                if nat_eq(a745, a746) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a747 = v1;
                                        let a748 = 1u64;
                                        nat_add(a747, a748)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h749, _, _) => {
                                            h749
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h750, _) => {
                                            h750
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h751) => {
                                            h751
                                        }
                                    }))))
                                } else {
                                    let a752 = v5;
                                    let a753 = 13u64;
                                    if nat_eq(a752, a753) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a754 = v1;
                                            let a755 = 1u64;
                                            nat_add(a754, a755)?
                                        }, (v4, v3)))
                                    } else {
                                        let a756 = v5;
                                        let a757 = 10u64;
                                        if nat_eq(a756, a757) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a758 = v1;
                                                let a759 = 1u64;
                                                nat_add(a758, a759)?
                                            }, (v4, f197(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a760 = v1;
                                                let a761 = 1u64;
                                                nat_add(a760, a761)?
                                            }, (v4, Adt17::C0(false, false, f268(match v3.clone() {
                                                Adt17::C0(_, _, h762, _, _) => {
                                                    h762
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h763, _) => {
                                                    h763
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h764) => {
                                                    h764
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
                Ok(next) => { m765 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f269(List::cons(match v0.clone() {
        Adt17::C0(_, _, h769, _, _) => {
            h769
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h770, _) => {
            h770
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h771) => {
            h771
        }
    }))
}

pub fn f198(p774: List<List<Bytes>>, p775: List<List<Bytes>>) -> List<List<Bytes>> {
    let m773 = (p774, p775);
    {
        let mut m773 = m773;
        loop {
            let step = {
                let (v0, v1) = m773;
                let m772 = v0;
                match m772.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m773 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f270(v0.clone(), 0u64, {
        let a776 = v0;
        length_bytes(a776)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f200(v0: Adt22) -> Bytes {
    let a797 = f271(match v0.clone() {
        Adt22::C0(h777, _, _, _, _, _, _, _) => {
            h777
        }
    });
    let a798 = {
        let a795 = f271(match v0.clone() {
            Adt22::C0(_, h778, _, _, _, _, _, _) => {
                h778
            }
        });
        let a796 = {
            let a793 = f271(match v0.clone() {
                Adt22::C0(_, _, h779, _, _, _, _, _) => {
                    h779
                }
            });
            let a794 = {
                let a791 = f271(match v0.clone() {
                    Adt22::C0(_, _, _, h780, _, _, _, _) => {
                        h780
                    }
                });
                let a792 = {
                    let a789 = f271(match v0.clone() {
                        Adt22::C0(_, _, _, _, h781, _, _, _) => {
                            h781
                        }
                    });
                    let a790 = {
                        let a787 = f271(match v0.clone() {
                            Adt22::C0(_, _, _, _, _, h782, _, _) => {
                                h782
                            }
                        });
                        let a788 = {
                            let a785 = f271(match v0.clone() {
                                Adt22::C0(_, _, _, _, _, _, h783, _) => {
                                    h783
                                }
                            });
                            let a786 = f271(match v0 {
                                Adt22::C0(_, _, _, _, _, _, _, h784) => {
                                    h784
                                }
                            });
                            append_bytes(a785, a786)
                        };
                        append_bytes(a787, a788)
                    };
                    append_bytes(a789, a790)
                };
                append_bytes(a791, a792)
            };
            append_bytes(a793, a794)
        };
        append_bytes(a795, a796)
    };
    append_bytes(a797, a798)
}

pub fn f201(v0: Bytes) -> R<Adt22> {
    let v1: u64 = {
        let a799 = v0.clone();
        length_bytes(a799)
    };
    let v2: u64 = {
        let a805 = {
            let a802 = {
                let a800 = v1;
                let a801 = 72u64;
                nat_add(a800, a801)?
            };
            let a803 = 64u64;
            let a804 = 0u64;
            nat_quot(a802, a803, a804)
        };
        let a806 = 64u64;
        nat_mul(a805, a806)?
    };
    let (_, h807) = f272(Fn4::F273(v0, v1, v2), {
        let a808 = v2;
        let a809 = 64u64;
        let a810 = 0u64;
        nat_quot(a808, a809, a810)
    }, (0u64, f274()))?;
    Ok(h807)
}

pub fn f202(p813: List<List<Bytes>>, v1: Bytes, p814: List<Bytes>) -> R<List<Bytes>> {
    let m812 = (p813, p814);
    {
        let mut m812 = m812;
        loop {
            let step = {
                let (v0, v2) = m812;
                let m811 = v0;
                match m811.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f275(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f204(f276(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f277(v0)?))
}

pub fn f204(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m815 = v0;
    match m815.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m816 = v3.clone();
            match m816.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a819 = v2;
                    let a820 = {
                        let a817 = v1.clone();
                        let a818 = f204(v3, v1);
                        append_bytes(a817, a818)
                    };
                    append_bytes(a819, a820)
                }
            }
        }
    }
}

pub fn f205(v0: List<Bytes>) -> Adt14 {
    let m821 = v0;
    match m821.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f205(v2)))
        }
    }
}

pub fn f206(v0: Fn2, p825: Option<Bytes>, p826: List<Bytes>) -> Option<Bytes> {
    let m824 = (p825, p826);
    {
        let mut m824 = m824;
        loop {
            let step = {
                let (v1, v2) = m824;
                let m822 = v2;
                match m822.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c823: Fn2 = v0.clone();
                            c823.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m827 = v1;
    match m827 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a830 = {
                    let a828 = v3;
                    let a829 = v0;
                    append_bytes(a828, a829)
                };
                let a831 = v2;
                append_bytes(a830, a831)
            })
        }
    }
}

pub fn f208(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f277(v0.clone())?;
    let v2: u64 = f278(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a832 = v1;
        let a833 = v2;
        nat_sub(a832, a833)
    }))
}

pub fn f209(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f279(v0)?;
    let m837 = f280(v1.clone(), 0u64, {
        let a835 = {
            let a834 = v1.clone();
            length_bytes(a834)
        };
        let a836 = 2u64;
        nat_add(a835, a836)?
    }, 0u64)?;
    match m837 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a840 = {
                let (_, h838) = v2;
                h838
            };
            let a841 = {
                let a839 = v1;
                length_bytes(a839)
            };
            if nat_eq(a840, a841) {
                Ok(Some({
                    let (h842, _) = v2;
                    h842
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f210(v0: Bytes) -> Bytes {
    f7(List::cons(f76(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f211(v0: Adt14) -> bool {
    if f192(v0.clone(), f8(Str::lit("domain"))) {
        if f192(v0.clone(), f8(Str::lit("row"))) {
            if f85(f145(v0.clone(), f8(Str::lit("relation"))), f281()) {
                if f282(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f282(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f282(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f282(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f212(v0: Adt14) -> bool {
    if f192(v0.clone(), f8(Str::lit("subject"))) {
        if f192(v0.clone(), f8(Str::lit("object"))) {
            if f192(v0.clone(), f8(Str::lit("evidence"))) {
                f85(f145(v0, f8(Str::lit("relation"))), f283())
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

pub fn f213(v0: Option<List<Adt14>>) -> bool {
    let m843 = v0;
    match m843 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f214(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m844 = v0;
    match m844 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f215(v0: Adt14) -> Bytes {
    if f192(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f192(v0.clone(), f8(Str::lit("id"))) {
            if f192(v0.clone(), f8(Str::lit("statement"))) {
                if f192(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f223(v0, f8(Str::lit("evidence")))
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

pub fn f216(v0: List<Adt14>) -> Adt14 {
    let m845 = v0;
    match m845.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f216(v2)))
        }
    }
}

pub fn f217(v0: Adt14) -> bool {
    if f192(v0.clone(), f8(Str::lit("name"))) {
        if f223(v0.clone(), f8(Str::lit("schema"))) {
            let m846 = f219(f73(f146(f142(v0, f8(Str::lit("citation"))))));
            match m846 {
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

pub fn f218(v0: Bytes) -> R<Bytes> {
    f284(v0.clone(), 0u64, {
        let a847 = v0;
        length_bytes(a847)
    }, true, Bytes::lit(&[]))
}

pub fn f219(v0: List<Bytes>) -> Option<Adt14> {
    let m848 = v0;
    match m848.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m849 = v2;
                match m849.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m850 = v2;
                    match m850.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m851 = v6;
                                match m851.uncons() {
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
                        let m852 = v2;
                        match m852.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m853 = v10;
                                match m853.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f205(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f220(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m854 = f59(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m854.uncons() {
        None => {
            let m855 = f219(v4);
            match m855 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a856 = f8(Str::lit("Catalog."));
                        let a857 = f218(v3.clone())?;
                        append_bytes(a856, a857)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f218(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m858 = f285(v2, v9.clone(), f218(v3.clone())?)?;
                    match m858 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f218(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f144(Adt14::C3(std::rc::Rc::new(f216(f286(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f144(Adt14::C3(std::rc::Rc::new(f216(f286(f72(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a862 = f287(f69(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a860 = {
                                        let a859 = v10;
                                        length_bytes(a859)
                                    };
                                    let a861 = 4u64;
                                    nat_sub(a860, a861)
                                }))?;
                                let a863 = Bytes::lit(&[0x0a]);
                                append_bytes(a862, a863)
                            }), List::cons(f119(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f221(v0: Adt14) -> R<Bytes> {
    let m864 = v0;
    match m864 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f288(v2, v3)
        }
        Adt14::C2(v4) => {
            f289(v4)
        }
        Adt14::C3(r865) => {
            let v5 = (*r865).clone();
            Ok(f290(f291(v5)?))
        }
        Adt14::C4(r866) => {
            let v6 = (*r866).clone();
            f292(f293(v6)?)
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

pub fn f222(v0: Adt14) -> List<Adt14> {
    let m867 = v0;
    match m867 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r868) => {
            let v3 = (*r868).clone();
            f222(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r869, r870) => {
            let v1 = (*r869).clone();
            let v2 = (*r870).clone();
            List::cons(v1, f222(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f223(v0: Adt14, v1: Bytes) -> bool {
    let m871 = f142(v0, v1);
    match m871 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f224(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m872 = f142(v0, v1);
    match m872 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f146(f142(v3, v2))
        }
    }
}

pub fn f225(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f142(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f115(Str::lit("id"), f145(v0.clone(), f8(Str::lit("id"))))?, List::cons(f115(Str::lit("name"), f145(v0.clone(), f8(Str::lit("name"))))?, List::cons(f115(Str::lit("issuer"), f145(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f115(Str::lit("edition"), f145(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f115(Str::lit("canonical_id"), f145(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f115(Str::lit("source_role"), f145(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f115(Str::lit("retrieval_date"), f145(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f115(Str::lit("url"), f294(v2.clone(), f8(Str::lit("url"))))?, List::cons(f115(Str::lit("media_type"), f294(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f115(Str::lit("license"), f294(v2.clone(), f8(Str::lit("license"))))?, List::cons(f115(Str::lit("redistribution"), f294(v2, f8(Str::lit("redistribution"))))?, List::cons(f115(Str::lit("statement"), f145(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m873 = v1;
        match m873 {
            None => {
                f7(List::cons(f115(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f115(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f115(Str::lit("artifact"), {
                    let (h874, _) = v3;
                    h874
                })?, List::cons(f115(Str::lit("checksum"), {
                    let (_, h875) = v3;
                    h875
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f295(f73(f146(f142(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f226(v0: Bytes, v1: u64) -> Option<u8> {
    let a876 = v0;
    let a877 = v1;
    index_bytes(a876, a877)
}

pub fn f227(v0: Bytes) -> bool {
    f129(v0, f8(Str::lit(".lex.tex")))
}

pub fn f228(v0: Fn3, p882: u64, p883: u64) -> R<(u64, bool)> {
    let m881 = (p882, p883);
    {
        let mut m881 = m881;
        loop {
            let step = {
                let (v1, v2) = m881;
                let m878 = v1;
                if m878 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m878 - 1;
                    let m880 = {
                        let c879: Fn3 = v0.clone();
                        c879.apply(v2)?
                    };
                    match m880 {
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
                Ok(next) => { m881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f229(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f230(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a886 = {
            let a884 = v4;
            let a885 = v1;
            nat_add(a884, a885)?
        };
        let a887 = v3;
        if nat_lt(a886, a887) {
            Ok(Some({
                let a888 = v4;
                let a889 = 1u64;
                nat_add(a888, a889)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f230(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m894 = {
        let a891 = v0;
        let a892 = v2;
        let a893 = {
            let a890 = v1.clone();
            length_bytes(a890)
        };
        slice_bytes(a891, a892, a893)
    };
    match m894 {
        None => {
            false
        }
        Some(v3) => {
            let a895 = v3;
            let a896 = v1;
            equal(a895, a896)
        }
    }
}

pub fn f231(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m897 = v0;
    match m897.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f231(v3, {
                let a898 = v1;
                let a899 = 1u64;
                nat_add(a898, a899)?
            })?))
        }
    }
}

pub fn f232(v0: Adt16, v1: List<u64>, v2: Adt19) -> bool {
    let m900 = v2;
    match m900 {
        Adt19::C0(v3, v4) => {
            if f238(v3, match v0.clone() {
                Adt16::C0(h901, _, _, _) => {
                    h901
                }
            }) {
                f296(v0, v4)
            } else {
                false
            }
        }
        Adt19::C1(v5) => {
            f238(v5, match v0 {
                Adt16::C0(_, _, h902, _) => {
                    h902
                }
            })
        }
        Adt19::C2(_, v7, v8) => {
            if f234(v7, v1) {
                f238(v8, match v0 {
                    Adt16::C0(_, _, _, h903) => {
                        h903
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f233(v0: Adt12, v1: Adt14) -> R<Adt19> {
    let m904 = f142(v1.clone(), f8(Str::lit("std")));
    match m904 {
        None => {
            let m909 = f150(f142(v1.clone(), f8(Str::lit("req"))));
            match m909 {
                None => {
                    let m911 = f142(v1, f8(Str::lit("der")));
                    match m911 {
                        None => {
                            Ok(f297(v0))
                        }
                        Some(v5) => {
                            let m912 = f298(f145(v5.clone(), f8(Str::lit("rule"))));
                            match m912 {
                                None => {
                                    Ok(f297(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt19::C2(v6, f166(f73(f146(f142(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h913) => {
                                            h913
                                        }
                                    })?, f236(f145(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h914, _) => {
                                            h914
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt19::C1(f236(v4, match v0 {
                        Adt12::C0(_, _, h910, _, _) => {
                            h910
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m905 = f150(f142(v2.clone(), f8(Str::lit("row"))));
            match m905 {
                None => {
                    Ok(Adt19::C0(f236(f145(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h906, _, _, _, _) => {
                            h906
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt19::C0(f236(f145(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h907, _, _, _, _) => {
                            h907
                        }
                    })?, Some(f236(v3, match v0 {
                        Adt12::C0(_, h908, _, _, _) => {
                            h908
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f234(p917: List<u64>, v1: List<u64>) -> bool {
    let m916 = p917;
    {
        let mut m916 = m916;
        loop {
            let step = {
                let v0 = m916;
                let m915 = v0;
                match m915.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f238(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(p922: List<u64>) -> bool {
    let m921 = p922;
    {
        let mut m921 = m921;
        loop {
            let step = {
                let v0 = m921;
                let m918 = v0;
                match m918.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a919 = v1;
                        let a920 = 7u64;
                        if nat_le(a919, a920) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m921 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m923 = f299(v0, v1.clone(), 0u64)?;
    match m923 {
        None => {
            let a924 = v1;
            Ok(length_list(a924))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f237(v0: Adt14) -> u64 {
    let m925 = f300(f142(v0, f8(Str::lit("stratum"))));
    match m925 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f238(v0: u64, p930: List<u64>) -> bool {
    let m929 = p930;
    {
        let mut m929 = m929;
        loop {
            let step = {
                let v1 = m929;
                let m926 = v1;
                match m926.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a927 = v2;
                        let a928 = v0;
                        if nat_eq(a927, a928) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m929 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f194(v0.clone(), v1) {
        let m931 = f30(v0, 34u8)?;
        match m931.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m932 = v3;
                match m932.uncons() {
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

pub fn f240(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f241(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m933 = v0.clone();
    match m933.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a934 = v2;
            let a935 = v1.clone();
            if equal(a934, a935) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f242(v0: List<(Bytes, ())>) -> List<Bytes> {
    f171(f301(Fn5::F302, List::<Bytes>::nil(), v0))
}

pub fn f243(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f303(f304(Fn6::F305, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m940 = {
        let (h936, _) = f306(Fn7::F307, {
            let a938 = {
                let a937 = v0;
                length_list(a937)
            };
            let a939 = 1u64;
            nat_add(a938, a939)?
        }, v1)?;
        h936
    };
    match m940.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f244(v0: List<Bytes>) -> List<(Bytes, ())> {
    f308(f309(Fn8::F310, List::<(Bytes, ())>::nil(), v0))
}

pub fn f245(p943: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m942 = p943;
    {
        let mut m942 = m942;
        loop {
            let step = {
                let v0 = m942;
                let m941 = v0;
                match m941.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f275(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m942 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f294(f142(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f247(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h944, _) = v1.clone();
        h944
    };
    let v3: u64 = {
        let (h945, _) = {
            let (_, h946) = v1.clone();
            h946
        };
        h945
    };
    let v4: List<Bytes> = {
        let (_, h947) = {
            let (_, h948) = v1;
            h948
        };
        h947
    };
    let m949 = f226(v0.clone(), v2);
    match m949 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a950 = v5;
            let a951 = 10u8;
            if equal(a950, a951) {
                Ok(Some(({
                    let a952 = v2;
                    let a953 = 1u64;
                    nat_add(a952, a953)?
                }, ({
                    let a954 = v2;
                    let a955 = 1u64;
                    nat_add(a954, a955)?
                }, List::cons(f311(f32(v0, v3, {
                    let a956 = v2;
                    let a957 = v3;
                    nat_sub(a956, a957)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a958 = v2;
                    let a959 = 1u64;
                    nat_add(a958, a959)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f248(v0: Adt1, v1: Bytes) -> R<Adt14> {
    let m960 = f141(v0, v1)?;
    match m960 {
        None => {
            Ok(Adt14::C5)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f249(v0: Bytes) -> R<Bytes> {
    let a961 = f8(Str::lit("Catalog."));
    let a962 = f218(v0)?;
    Ok(append_bytes(a961, a962))
}

pub fn f250(v0: Adt14) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f218(f145(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f251(v0: List<Bytes>) -> List<Bytes> {
    let m963 = v0;
    match m963.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f129(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f125(f133(), v1), f251(v2))
            } else {
                f251(v2)
            }
        }
    }
}

pub fn f252(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m964 = v1;
    match m964.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f312(v0.clone(), v2)?;
            let (h965, _) = v4.clone();
            if h965 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f252(v0, v3)?;
                let (h966, _) = v5.clone();
                if h966 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a969 = {
                            let (_, h967) = v4;
                            h967
                        };
                        let a970 = {
                            let (_, h968) = v5;
                            h968
                        };
                        append_bytes(a969, a970)
                    }))
                }
            }
        }
    }
}

pub fn f253(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m971 = v1;
    match m971.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f192(v2.clone(), f8(Str::lit("subject"))) {
                if f193(v0.clone(), v2.clone()) {
                    f253(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f145(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f145(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f253(v0, v3))
                }
            } else {
                if f262(v0.clone(), v2.clone()) {
                    f253(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f145(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f263(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f253(v0, v3))
                }
            }
        }
    }
}

pub fn f254(v0: Adt1) -> R<Bytes> {
    let a973 = {
        let a972 = f102(f103(f121(f122(f111(f69(v0.clone(), f116()))?)), f104(f72(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f105(f72(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a972)
    };
    let a974 = 0u64;
    if nat_eq(a973, a974) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f255(v0: List<Adt14>) -> R<List<Bytes>> {
    let m975 = v0;
    match m975.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f120(f73(f146(f142(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f145(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f145(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f313(f82(f145(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f255(v2)?))
        }
    }
}

pub fn f256(v0: Bytes, v1: List<Adt21>) -> List<Bytes> {
    let m976 = v1;
    match m976.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(match v2.clone() {
                Adt21::C0(h977, _, _) => {
                    h977
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt21::C0(_, h978, _) => {
                        h978
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt21::C0(_, _, h979) => {
                        h979
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f256(v0, v3))
            } else {
                f256(v0, v3)
            }
        }
    }
}

pub fn f257(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m980 = v1;
    match m980.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(f145(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f145(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f257(v0, v3))
            } else {
                if f36(f145(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f145(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f145(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f257(v0, v3))
                } else {
                    f257(v0, v3)
                }
            }
        }
    }
}

pub fn f258(p985: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m984 = p985;
    {
        let mut m984 = m984;
        loop {
            let step = {
                let v0 = m984;
                let m981 = v0;
                match m981.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m982 = v2;
                        match m982.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f36(v4, v1.clone()) {
                                    let m983 = v5;
                                    match m983.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f314(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m984 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m986 = v0;
    match m986.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m987 = f142(v1, f8(Str::lit("std")));
            match m987 {
                None => {
                    f259(v2)
                }
                Some(v3) => {
                    if f192(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f145(v3.clone(), f8(Str::lit("row"))), f145(v3, f8(Str::lit("authority")))), f259(v2))
                    } else {
                        f259(v2)
                    }
                }
            }
        }
    }
}

pub fn f260(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m988 = v1;
    match m988.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f145(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f145(v2, f8(Str::lit("object")));
            if if f85(v4.clone(), match v0.clone() {
                Adt15::C0(h989, _, _) => {
                    h989
                }
            }) {
                f85(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h990, _) => {
                        h990
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f260(v0, v3))
            } else {
                if if f85(v5.clone(), match v0.clone() {
                    Adt15::C0(h991, _, _) => {
                        h991
                    }
                }) {
                    f85(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h992, _) => {
                            h992
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f260(v0, v3))
                } else {
                    f260(v0, v3)
                }
            }
        }
    }
}

pub fn f261(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt21>> {
    let m993 = v0;
    match m993.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v4, v5)) => {
            if if f315(v4.clone(), v2.clone()) {
                true
            } else {
                f315(v4.clone(), v3.clone())
            } {
                f261(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt21::C0({
                    let (h994, _) = v4.clone();
                    h994
                }, {
                    let a997 = {
                        let a996 = f77(f316({
                            let (h995, _) = v4.clone();
                            h995
                        }, v1.clone()))?;
                        length_list(a996)
                    };
                    let a998 = 1u64;
                    if nat_eq(a997, a998) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h999) = v4.clone();
                    h999
                }), f261(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f262(v0: Adt15, v1: Adt14) -> bool {
    if f85(f145(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1000, _, _) => {
            h1000
        }
    }) {
        f85(f263(v1), match v0 {
            Adt15::C0(_, h1001, _) => {
                h1001
            }
        })
    } else {
        false
    }
}

pub fn f263(v0: Adt14) -> Bytes {
    let m1002 = f142(v0, f8(Str::lit("evidence")));
    match m1002 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f145(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f264(v0: Adt14) -> Bytes {
    if f192(v0.clone(), f8(Str::lit("req"))) {
        let a1003 = f8(Str::lit("req "));
        let a1004 = f145(v0, f8(Str::lit("req")));
        append_bytes(a1003, a1004)
    } else {
        let m1005 = f142(v0.clone(), f8(Str::lit("std")));
        match m1005 {
            None => {
                let m1008 = f142(v0, f8(Str::lit("der")));
                match m1008 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f145(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f120(f73(f146(f142(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f145(v1.clone(), f8(Str::lit("authority"))), List::cons(if f192(v1.clone(), f8(Str::lit("row"))) {
                    let a1006 = f8(Str::lit(" row "));
                    let a1007 = f145(v1, f8(Str::lit("row")));
                    append_bytes(a1006, a1007)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f265(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1009 = v1;
    match m1009.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85({
                let (h1010, _) = v2.clone();
                h1010
            }, v0.clone()) {
                List::cons({
                    let (_, h1011) = v2;
                    h1011
                }, f265(v0, v3))
            } else {
                if f85({
                    let (_, h1012) = v2;
                    h1012
                }, v0.clone()) {
                    List::cons({
                        let (h1013, _) = v2;
                        h1013
                    }, f265(v0, v3))
                } else {
                    f265(v0, v3)
                }
            }
        }
    }
}

pub fn f266(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f85(v2.clone(), match v0.clone() {
        Adt15::C0(h1014, _, _) => {
            h1014
        }
    }) {
        true
    } else {
        if f85(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1015, _) => {
                h1015
            }
        }) {
            true
        } else {
            if f85(v2, match v0.clone() {
                Adt15::C0(_, _, h1016) => {
                    h1016
                }
            }) {
                true
            } else {
                if f192(v1.clone(), f8(Str::lit("scheme"))) {
                    f85(f145(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1017, _) => {
                            h1017
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f267(v0: Bytes, v1: u64) -> R<u64> {
    let m1020 = {
        let a1018 = v0;
        let a1019 = v1;
        index_bytes(a1018, a1019)
    };
    match m1020 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1065 = {
                let a1041 = {
                    let a1029 = {
                        let a1023 = {
                            let a1021 = v3;
                            let a1022 = 128u8;
                            fixed_u8::bit_and(a1021, a1022)
                        };
                        let a1024 = 128u8;
                        if equal(a1023, a1024) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1030 = {
                        let a1027 = {
                            let a1025 = v3;
                            let a1026 = 64u8;
                            fixed_u8::bit_and(a1025, a1026)
                        };
                        let a1028 = 64u8;
                        if equal(a1027, a1028) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1029, a1030)?
                };
                let a1042 = {
                    let a1039 = {
                        let a1033 = {
                            let a1031 = v3;
                            let a1032 = 32u8;
                            fixed_u8::bit_and(a1031, a1032)
                        };
                        let a1034 = 32u8;
                        if equal(a1033, a1034) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1040 = {
                        let a1037 = {
                            let a1035 = v3;
                            let a1036 = 16u8;
                            fixed_u8::bit_and(a1035, a1036)
                        };
                        let a1038 = 16u8;
                        if equal(a1037, a1038) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1039, a1040)?
                };
                nat_add(a1041, a1042)?
            };
            let a1066 = {
                let a1063 = {
                    let a1051 = {
                        let a1045 = {
                            let a1043 = v3;
                            let a1044 = 8u8;
                            fixed_u8::bit_and(a1043, a1044)
                        };
                        let a1046 = 8u8;
                        if equal(a1045, a1046) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1052 = {
                        let a1049 = {
                            let a1047 = v3;
                            let a1048 = 4u8;
                            fixed_u8::bit_and(a1047, a1048)
                        };
                        let a1050 = 4u8;
                        if equal(a1049, a1050) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1051, a1052)?
                };
                let a1064 = {
                    let a1061 = {
                        let a1055 = {
                            let a1053 = v3;
                            let a1054 = 2u8;
                            fixed_u8::bit_and(a1053, a1054)
                        };
                        let a1056 = 2u8;
                        if equal(a1055, a1056) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1062 = {
                        let a1059 = {
                            let a1057 = v3;
                            let a1058 = 1u8;
                            fixed_u8::bit_and(a1057, a1058)
                        };
                        let a1060 = 1u8;
                        if equal(a1059, a1060) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1061, a1062)?
                };
                nat_add(a1063, a1064)?
            };
            nat_add(a1065, a1066)
        }
    }
}

pub fn f268(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1070 = {
        let a1067 = v1;
        let a1068 = v2;
        let a1069 = 1u64;
        slice_bytes(a1067, a1068, a1069)
    };
    match m1070 {
        None => {
            v0
        }
        Some(v3) => {
            let a1071 = v0;
            let a1072 = v3;
            append_bytes(a1071, a1072)
        }
    }
}

pub fn f269(p1075: List<Bytes>, p1076: List<Bytes>) -> List<Bytes> {
    let m1074 = (p1075, p1076);
    {
        let mut m1074 = m1074;
        loop {
            let step = {
                let (v0, v1) = m1074;
                let m1073 = v0;
                match m1073.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1074 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Bytes, p1148: u64, p1149: u64, p1150: Bytes) -> R<Bytes> {
    let m1147 = (p1148, (p1149, p1150));
    {
        let mut m1147 = m1147;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1147;
                let m1077 = v2;
                if m1077 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1077 - 1;
                    let m1080 = {
                        let a1078 = v0.clone();
                        let a1079 = v1;
                        index_bytes(a1078, a1079)
                    };
                    match m1080 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1125 = {
                                    let a1101 = {
                                        let a1089 = {
                                            let a1083 = {
                                                let a1081 = v6;
                                                let a1082 = 128u8;
                                                fixed_u8::bit_and(a1081, a1082)
                                            };
                                            let a1084 = 128u8;
                                            if equal(a1083, a1084) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1090 = {
                                            let a1087 = {
                                                let a1085 = v6;
                                                let a1086 = 64u8;
                                                fixed_u8::bit_and(a1085, a1086)
                                            };
                                            let a1088 = 64u8;
                                            if equal(a1087, a1088) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1089, a1090)?
                                    };
                                    let a1102 = {
                                        let a1099 = {
                                            let a1093 = {
                                                let a1091 = v6;
                                                let a1092 = 32u8;
                                                fixed_u8::bit_and(a1091, a1092)
                                            };
                                            let a1094 = 32u8;
                                            if equal(a1093, a1094) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1100 = {
                                            let a1097 = {
                                                let a1095 = v6;
                                                let a1096 = 16u8;
                                                fixed_u8::bit_and(a1095, a1096)
                                            };
                                            let a1098 = 16u8;
                                            if equal(a1097, a1098) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1099, a1100)?
                                    };
                                    nat_add(a1101, a1102)?
                                };
                                let a1126 = {
                                    let a1123 = {
                                        let a1111 = {
                                            let a1105 = {
                                                let a1103 = v6;
                                                let a1104 = 8u8;
                                                fixed_u8::bit_and(a1103, a1104)
                                            };
                                            let a1106 = 8u8;
                                            if equal(a1105, a1106) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1112 = {
                                            let a1109 = {
                                                let a1107 = v6;
                                                let a1108 = 4u8;
                                                fixed_u8::bit_and(a1107, a1108)
                                            };
                                            let a1110 = 4u8;
                                            if equal(a1109, a1110) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1111, a1112)?
                                    };
                                    let a1124 = {
                                        let a1121 = {
                                            let a1115 = {
                                                let a1113 = v6;
                                                let a1114 = 2u8;
                                                fixed_u8::bit_and(a1113, a1114)
                                            };
                                            let a1116 = 2u8;
                                            if equal(a1115, a1116) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1122 = {
                                            let a1119 = {
                                                let a1117 = v6;
                                                let a1118 = 1u8;
                                                fixed_u8::bit_and(a1117, a1118)
                                            };
                                            let a1120 = 1u8;
                                            if equal(a1119, a1120) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1121, a1122)?
                                    };
                                    nat_add(a1123, a1124)?
                                };
                                nat_add(a1125, a1126)?
                            };
                            let a1127 = v7;
                            let a1128 = 34u64;
                            if if nat_eq(a1127, a1128) {
                                true
                            } else {
                                let a1129 = v7;
                                let a1130 = 92u64;
                                nat_eq(a1129, a1130)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1131 = v1;
                                    let a1132 = 1u64;
                                    nat_add(a1131, a1132)?
                                }, (v4, {
                                    let a1135 = v3;
                                    let a1136 = {
                                        let a1133 = Bytes::lit(&[0x5c]);
                                        let a1134 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1133, a1134)
                                    };
                                    append_bytes(a1135, a1136)
                                })))
                            } else {
                                let a1137 = v7;
                                let a1138 = 10u64;
                                if nat_eq(a1137, a1138) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1139 = v1;
                                        let a1140 = 1u64;
                                        nat_add(a1139, a1140)?
                                    }, (v4, {
                                        let a1141 = v3;
                                        let a1142 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1141, a1142)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1143 = v1;
                                        let a1144 = 1u64;
                                        nat_add(a1143, a1144)?
                                    }, (v4, {
                                        let a1145 = v3;
                                        let a1146 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1145, a1146)
                                    })))
                                }
                            }
                        }
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

pub fn f271(v0: u32) -> Bytes {
    let a1179 = f317({
        let a1151 = f318(v0, 28u32);
        let a1152 = 15u32;
        fixed_u32::bit_and(a1151, a1152)
    });
    let a1180 = {
        let a1177 = f317({
            let a1153 = f318(v0, 24u32);
            let a1154 = 15u32;
            fixed_u32::bit_and(a1153, a1154)
        });
        let a1178 = {
            let a1175 = f317({
                let a1155 = f318(v0, 20u32);
                let a1156 = 15u32;
                fixed_u32::bit_and(a1155, a1156)
            });
            let a1176 = {
                let a1173 = f317({
                    let a1157 = f318(v0, 16u32);
                    let a1158 = 15u32;
                    fixed_u32::bit_and(a1157, a1158)
                });
                let a1174 = {
                    let a1171 = f317({
                        let a1159 = f318(v0, 12u32);
                        let a1160 = 15u32;
                        fixed_u32::bit_and(a1159, a1160)
                    });
                    let a1172 = {
                        let a1169 = f317({
                            let a1161 = f318(v0, 8u32);
                            let a1162 = 15u32;
                            fixed_u32::bit_and(a1161, a1162)
                        });
                        let a1170 = {
                            let a1167 = f317({
                                let a1163 = f318(v0, 4u32);
                                let a1164 = 15u32;
                                fixed_u32::bit_and(a1163, a1164)
                            });
                            let a1168 = f317({
                                let a1165 = v0;
                                let a1166 = 15u32;
                                fixed_u32::bit_and(a1165, a1166)
                            });
                            append_bytes(a1167, a1168)
                        };
                        append_bytes(a1169, a1170)
                    };
                    append_bytes(a1171, a1172)
                };
                append_bytes(a1173, a1174)
            };
            append_bytes(a1175, a1176)
        };
        append_bytes(a1177, a1178)
    };
    append_bytes(a1179, a1180)
}

pub fn f272(v0: Fn4, p1184: u64, p1185: (u64, Adt22)) -> R<(u64, Adt22)> {
    let m1183 = (p1184, p1185);
    {
        let mut m1183 = m1183;
        loop {
            let step = {
                let (v1, v2) = m1183;
                let m1181 = v1;
                if m1181 == 0 {
                    Err::<(u64, (u64, Adt22)), R<(u64, Adt22)>>(Ok(v2))
                } else {
                    let v3 = m1181 - 1;
                    Ok::<(u64, (u64, Adt22)), R<(u64, Adt22)>>((v3, {
                        let c1182: Fn4 = v0.clone();
                        c1182.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1183 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt22)) -> R<(u64, Adt22)> {
    Ok(({
        let a1187 = {
            let (h1186, _) = v3.clone();
            h1186
        };
        let a1188 = 64u64;
        nat_add(a1187, a1188)?
    }, f319({
        let (_, h1189) = v3;
        h1189
    }, f320(v0, v1, v2, {
        let (h1190, _) = v3;
        h1190
    })?)?))
}

pub fn f274() -> Adt22 {
    Adt22::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f275(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f85(v1, f68(f204(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f276(v0: List<Bytes>) -> List<Bytes> {
    let m1191 = v0;
    match m1191.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f276(v2)
            } else {
                List::cons(v1, f276(v2))
            }
        }
    }
}

pub fn f277(v0: Bytes) -> R<u64> {
    let (h1192, _) = f228(Fn3::F321(v0.clone()), {
        let a1193 = v0.clone();
        length_bytes(a1193)
    }, {
        let a1194 = v0;
        length_bytes(a1194)
    })?;
    Ok(h1192)
}

pub fn f278(v0: Bytes, v1: u64) -> R<u64> {
    let (h1195, _) = f228(Fn3::F322(v1, v0), v1, 0u64)?;
    Ok(h1195)
}

pub fn f279(v0: Bytes) -> R<Bytes> {
    f323(v0.clone(), 0u64, {
        let a1196 = v0;
        length_bytes(a1196)
    }, false, false, Bytes::lit(&[]))
}

pub fn f280(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1197 = v2;
    if m1197 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1197 - 1;
        let v5: u64 = f324(v0.clone(), v1)?;
        let a1198 = v3;
        let a1199 = 1u64;
        if nat_eq(a1198, a1199) {
            let m1200 = f280(v0.clone(), v1, v4, 0u64)?;
            match m1200 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f324(v0.clone(), {
                        let (_, h1201) = v6.clone();
                        h1201
                    })?;
                    let a1202 = v7;
                    let a1203 = 44u64;
                    if nat_eq(a1202, a1203) {
                        let m1207 = f280(v0, {
                            let a1205 = {
                                let (_, h1204) = v6;
                                h1204
                            };
                            let a1206 = 1u64;
                            nat_add(a1205, a1206)?
                        }, v4, 1u64)?;
                        match m1207 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1208, _) = v6;
                                    h1208
                                }), std::rc::Rc::new({
                                    let (h1209, _) = v8;
                                    h1209
                                })), {
                                    let (_, h1210) = v8;
                                    h1210
                                })))
                            }
                        }
                    } else {
                        let a1211 = v7;
                        let a1212 = 93u64;
                        if nat_eq(a1211, a1212) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1213, _) = v6;
                                h1213
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1215 = {
                                    let (_, h1214) = v6;
                                    h1214
                                };
                                let a1216 = 1u64;
                                nat_add(a1215, a1216)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1217 = v3;
            let a1218 = 2u64;
            if nat_eq(a1217, a1218) {
                let m1221 = f325(v0.clone(), {
                    let a1219 = v1;
                    let a1220 = 1u64;
                    nat_add(a1219, a1220)?
                })?;
                match m1221 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1222 = v5;
                        let a1223 = 34u64;
                        if if nat_eq(a1222, a1223) {
                            let a1225 = f324(v0.clone(), {
                                let (h1224, _) = v9.clone();
                                h1224
                            })?;
                            let a1226 = 58u64;
                            nat_eq(a1225, a1226)
                        } else {
                            false
                        } {
                            let m1230 = f280(v0.clone(), {
                                let a1228 = {
                                    let (h1227, _) = v9;
                                    h1227
                                };
                                let a1229 = 1u64;
                                nat_add(a1228, a1229)?
                            }, v4, 0u64)?;
                            match m1230 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f324(v0.clone(), {
                                        let (_, h1231) = v10.clone();
                                        h1231
                                    })?;
                                    let a1232 = v11;
                                    let a1233 = 44u64;
                                    if nat_eq(a1232, a1233) {
                                        let m1237 = f280(v0, {
                                            let a1235 = {
                                                let (_, h1234) = v10;
                                                h1234
                                            };
                                            let a1236 = 1u64;
                                            nat_add(a1235, a1236)?
                                        }, v4, 2u64)?;
                                        match m1237 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1238) = v9;
                                                    h1238
                                                }, std::rc::Rc::new({
                                                    let (h1239, _) = v10;
                                                    h1239
                                                }), std::rc::Rc::new({
                                                    let (h1240, _) = v12;
                                                    h1240
                                                })), {
                                                    let (_, h1241) = v12;
                                                    h1241
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1242 = v11;
                                        let a1243 = 125u64;
                                        if nat_eq(a1242, a1243) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1244) = v9;
                                                h1244
                                            }, std::rc::Rc::new({
                                                let (h1245, _) = v10;
                                                h1245
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1247 = {
                                                    let (_, h1246) = v10;
                                                    h1246
                                                };
                                                let a1248 = 1u64;
                                                nat_add(a1247, a1248)?
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
                let a1249 = v5;
                let a1250 = 34u64;
                if nat_eq(a1249, a1250) {
                    let m1253 = f325(v0, {
                        let a1251 = v1;
                        let a1252 = 1u64;
                        nat_add(a1251, a1252)?
                    })?;
                    match m1253 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1254) = v13;
                                h1254
                            }), {
                                let (h1255, _) = v13;
                                h1255
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1256 = v1;
                            let a1257 = 4u64;
                            nat_add(a1256, a1257)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1258 = v1;
                                let a1259 = 5u64;
                                nat_add(a1258, a1259)?
                            })))
                        } else {
                            let a1260 = v5;
                            let a1261 = 91u64;
                            if nat_eq(a1260, a1261) {
                                let a1264 = f324(v0.clone(), {
                                    let a1262 = v1;
                                    let a1263 = 1u64;
                                    nat_add(a1262, a1263)?
                                })?;
                                let a1265 = 93u64;
                                if nat_eq(a1264, a1265) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1266 = v1;
                                        let a1267 = 2u64;
                                        nat_add(a1266, a1267)?
                                    })))
                                } else {
                                    let m1270 = f280(v0, {
                                        let a1268 = v1;
                                        let a1269 = 1u64;
                                        nat_add(a1268, a1269)?
                                    }, v4, 1u64)?;
                                    match m1270 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1271, _) = v14;
                                                h1271
                                            })), {
                                                let (_, h1272) = v14;
                                                h1272
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1273 = v5;
                                let a1274 = 123u64;
                                if nat_eq(a1273, a1274) {
                                    let a1277 = f324(v0.clone(), {
                                        let a1275 = v1;
                                        let a1276 = 1u64;
                                        nat_add(a1275, a1276)?
                                    })?;
                                    let a1278 = 125u64;
                                    if nat_eq(a1277, a1278) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1279 = v1;
                                            let a1280 = 2u64;
                                            nat_add(a1279, a1280)?
                                        })))
                                    } else {
                                        let m1283 = f280(v0, {
                                            let a1281 = v1;
                                            let a1282 = 1u64;
                                            nat_add(a1281, a1282)?
                                        }, v4, 2u64)?;
                                        match m1283 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1284, _) = v15;
                                                    h1284
                                                })), {
                                                    let (_, h1285) = v15;
                                                    h1285
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1286 = f326(v0, v1)?;
                                    match m1286 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1287) = v16;
                                                h1287
                                            }, {
                                                let (h1288, _) = v16;
                                                h1288
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

pub fn f281() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f282(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1289 = f142(v0, v1);
    match m1289 {
        None => {
            false
        }
        Some(v3) => {
            f192(v3, v2)
        }
    }
}

pub fn f283() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f284(v0: Bytes, p1356: u64, p1357: u64, p1358: bool, p1359: Bytes) -> R<Bytes> {
    let m1355 = (p1356, (p1357, (p1358, p1359)));
    {
        let mut m1355 = m1355;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1355;
                let m1290 = v2;
                if m1290 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1290 - 1;
                    let m1293 = {
                        let a1291 = v0.clone();
                        let a1292 = v1;
                        index_bytes(a1291, a1292)
                    };
                    match m1293 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1338 = {
                                    let a1314 = {
                                        let a1302 = {
                                            let a1296 = {
                                                let a1294 = v7;
                                                let a1295 = 128u8;
                                                fixed_u8::bit_and(a1294, a1295)
                                            };
                                            let a1297 = 128u8;
                                            if equal(a1296, a1297) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1303 = {
                                            let a1300 = {
                                                let a1298 = v7;
                                                let a1299 = 64u8;
                                                fixed_u8::bit_and(a1298, a1299)
                                            };
                                            let a1301 = 64u8;
                                            if equal(a1300, a1301) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1302, a1303)?
                                    };
                                    let a1315 = {
                                        let a1312 = {
                                            let a1306 = {
                                                let a1304 = v7;
                                                let a1305 = 32u8;
                                                fixed_u8::bit_and(a1304, a1305)
                                            };
                                            let a1307 = 32u8;
                                            if equal(a1306, a1307) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1313 = {
                                            let a1310 = {
                                                let a1308 = v7;
                                                let a1309 = 16u8;
                                                fixed_u8::bit_and(a1308, a1309)
                                            };
                                            let a1311 = 16u8;
                                            if equal(a1310, a1311) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1312, a1313)?
                                    };
                                    nat_add(a1314, a1315)?
                                };
                                let a1339 = {
                                    let a1336 = {
                                        let a1324 = {
                                            let a1318 = {
                                                let a1316 = v7;
                                                let a1317 = 8u8;
                                                fixed_u8::bit_and(a1316, a1317)
                                            };
                                            let a1319 = 8u8;
                                            if equal(a1318, a1319) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1325 = {
                                            let a1322 = {
                                                let a1320 = v7;
                                                let a1321 = 4u8;
                                                fixed_u8::bit_and(a1320, a1321)
                                            };
                                            let a1323 = 4u8;
                                            if equal(a1322, a1323) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1324, a1325)?
                                    };
                                    let a1337 = {
                                        let a1334 = {
                                            let a1328 = {
                                                let a1326 = v7;
                                                let a1327 = 2u8;
                                                fixed_u8::bit_and(a1326, a1327)
                                            };
                                            let a1329 = 2u8;
                                            if equal(a1328, a1329) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1335 = {
                                            let a1332 = {
                                                let a1330 = v7;
                                                let a1331 = 1u8;
                                                fixed_u8::bit_and(a1330, a1331)
                                            };
                                            let a1333 = 1u8;
                                            if equal(a1332, a1333) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1334, a1335)?
                                    };
                                    nat_add(a1336, a1337)?
                                };
                                nat_add(a1338, a1339)?
                            };
                            if f327(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1340 = v1;
                                    let a1341 = 1u64;
                                    nat_add(a1340, a1341)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f328(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1342 = v1;
                                        let a1343 = 1u64;
                                        nat_add(a1342, a1343)?
                                    }, (v5, (false, {
                                        let a1347 = v4;
                                        let a1348 = f32({
                                            let a1344 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1344)
                                        }, {
                                            let a1345 = v8;
                                            let a1346 = 97u64;
                                            nat_sub(a1345, a1346)
                                        }, 1u64);
                                        append_bytes(a1347, a1348)
                                    }))))
                                } else {
                                    if if f328(v8) {
                                        true
                                    } else {
                                        if f329(v8) {
                                            true
                                        } else {
                                            f330(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1349 = v1;
                                            let a1350 = 1u64;
                                            nat_add(a1349, a1350)?
                                        }, (v5, (false, {
                                            let a1351 = v4;
                                            let a1352 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1351, a1352)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1353 = v1;
                                            let a1354 = 1u64;
                                            nat_add(a1353, a1354)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1360 = f331(4096u64, f332(f333(f334(f142(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt23::C0(v2, v0), List::<Adt23>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m1360 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f204(f175(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f286(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1361 = v0;
    match m1361.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f36(f145(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f335(v5, v1, v2))
            } else {
                List::cons(v4, f286(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f287(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f204(f336(f82(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f288(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1362 = 0u64;
        let a1363 = v1;
        nat_lt(a1362, a1363)
    } else {
        false
    } {
        let a1364 = Bytes::lit(&[0x2d]);
        let a1365 = f337(v1)?;
        Ok(append_bytes(a1364, a1365))
    } else {
        f337(v1)
    }
}

pub fn f289(v0: Bytes) -> R<Bytes> {
    let a1371 = {
        let a1369 = Bytes::lit(&[0x22]);
        let a1370 = {
            let (_, h1366) = {
                let (h1367, _) = f338(Fn9::F339(v0.clone()), {
                    let a1368 = v0;
                    length_bytes(a1368)
                }, (0u64, Bytes::lit(&[])))?;
                h1367
            };
            h1366
        };
        append_bytes(a1369, a1370)
    };
    let a1372 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1371, a1372))
}

pub fn f290(v0: List<Bytes>) -> Bytes {
    let a1375 = {
        let a1373 = Bytes::lit(&[0x5b]);
        let a1374 = f120(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1373, a1374)
    };
    let a1376 = Bytes::lit(&[0x5d]);
    append_bytes(a1375, a1376)
}

pub fn f291(v0: Adt14) -> R<List<Bytes>> {
    let m1377 = v0;
    match m1377 {
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
        Adt14::C6(r1378, r1379) => {
            let v7 = (*r1378).clone();
            let v8 = (*r1379).clone();
            Ok(List::cons(f221(v7)?, f291(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f292(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f340(Fn10::F341, List::<(Bytes, Bytes)>::nil(), f342(v0)?);
    let v2: List<Bytes> = f343(Fn11::F344, List::<Bytes>::nil(), v1)?;
    let a1382 = {
        let a1380 = Bytes::lit(&[0x7b]);
        let a1381 = f120(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1380, a1381)
    };
    let a1383 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1382, a1383))
}

pub fn f293(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1384 = v0;
    match m1384 {
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
        Adt14::C7(v9, r1385, r1386) => {
            let v10 = (*r1385).clone();
            let v11 = (*r1386).clone();
            Ok(List::cons((v9, f221(v10)?), f293(v11)?))
        }
    }
}

pub fn f294(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1387 = v0;
    match m1387 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f145(v2, v1)
        }
    }
}

pub fn f295(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f204(f345(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f296(v0: Adt16, v1: Option<u64>) -> bool {
    let m1388 = v1;
    match m1388 {
        None => {
            true
        }
        Some(v2) => {
            f238(v2, match v0 {
                Adt16::C0(_, h1389, _, _) => {
                    h1389
                }
            })
        }
    }
}

pub fn f297(v0: Adt12) -> Adt19 {
    Adt19::C1({
        let a1391 = match v0 {
            Adt12::C0(_, _, h1390, _, _) => {
                h1390
            }
        };
        length_list(a1391)
    })
}

pub fn f298(v0: Bytes) -> Option<Adt20> {
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

pub fn f299(v0: Bytes, p1396: List<Bytes>, p1397: u64) -> R<Option<u64>> {
    let m1395 = (p1396, p1397);
    {
        let mut m1395 = m1395;
        loop {
            let step = {
                let (v1, v2) = m1395;
                let m1392 = v1;
                match m1392.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1393 = v2;
                                let a1394 = 1u64;
                                nat_add(a1393, a1394)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: Option<Adt14>) -> Option<u64> {
    let m1398 = v0;
    match m1398 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1399 = v1;
            match m1399 {
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

pub fn f301(v0: Fn5, p1403: List<Bytes>, p1404: List<(Bytes, ())>) -> List<Bytes> {
    let m1402 = (p1403, p1404);
    {
        let mut m1402 = m1402;
        loop {
            let step = {
                let (v1, v2) = m1402;
                let m1400 = v2;
                match m1400.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1401: Fn5 = v0.clone();
                            c1401.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1405, _) = v1;
        h1405
    }, v0)
}

pub fn f303(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f346(Fn12::F347, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f304(v0: Fn6, p1409: List<List<(Bytes, ())>>, p1410: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1408 = (p1409, p1410);
    {
        let mut m1408 = m1408;
        loop {
            let step = {
                let (v1, v2) = m1408;
                let m1406 = v2;
                match m1406.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1407: Fn6 = v0.clone();
                            c1407.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1408 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f305(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f306(v0: Fn7, p1415: u64, p1416: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1414 = (p1415, p1416);
    {
        let mut m1414 = m1414;
        loop {
            let step = {
                let (v1, v2) = m1414;
                let m1411 = v1;
                if m1411 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1411 - 1;
                    let m1413 = {
                        let c1412: Fn7 = v0.clone();
                        c1412.apply(v2.clone())?
                    };
                    match m1413 {
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
                Ok(next) => { m1414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f307(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f348(v0)
}

pub fn f308(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f349(Fn13::F350, List::<(Bytes, ())>::nil(), v0)
}

pub fn f309(v0: Fn8, p1420: List<(Bytes, ())>, p1421: List<Bytes>) -> List<(Bytes, ())> {
    let m1419 = (p1420, p1421);
    {
        let mut m1419 = m1419;
        loop {
            let step = {
                let (v1, v2) = m1419;
                let m1417 = v2;
                match m1417.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1418: Fn8 = v0.clone();
                            c1418.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1419 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f311(v0: Bytes) -> Bytes {
    if f129(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1423 = {
                let a1422 = v0;
                length_bytes(a1422)
            };
            let a1424 = 1u64;
            nat_sub(a1423, a1424)
        })
    } else {
        v0
    }
}

pub fn f312(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f82(f69(v0.clone(), v1.clone()))?;
    let m1425 = f147(v2.clone())?;
    match m1425 {
        None => {
            f352(v0, v1, v2)
        }
        Some(v3) => {
            f351(v0, v1, v3)
        }
    }
}

pub fn f313(v0: List<Bytes>) -> List<Bytes> {
    let m1426 = v0;
    match m1426.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1428 = {
                    let a1427 = v1.clone();
                    length_bytes(a1427)
                };
                let a1429 = 0u64;
                if nat_eq(a1428, a1429) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f313(v2))
        }
    }
}

pub fn f314(v0: List<Bytes>) -> Bytes {
    let m1430 = v0;
    match m1430.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f315(v0: (Bytes, Bytes), p1433: List<(Bytes, Bytes)>) -> bool {
    let m1432 = p1433;
    {
        let mut m1432 = m1432;
        loop {
            let step = {
                let v1 = m1432;
                let m1431 = v1;
                match m1431.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f353(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
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

pub fn f316(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1434 = v1;
    match m1434.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1435, _) = v2;
                h1435
            }, v0.clone()) {
                List::cons({
                    let (_, h1436) = v2;
                    h1436
                }, f316(v0, v3))
            } else {
                f316(v0, v3)
            }
        }
    }
}

pub fn f317(v0: u32) -> Bytes {
    let a1439 = {
        let a1437 = v0;
        let a1438 = 8u32;
        fixed_u32::bit_and(a1437, a1438)
    };
    let a1440 = 0u32;
    if equal(a1439, a1440) {
        let a1443 = {
            let a1441 = v0;
            let a1442 = 4u32;
            fixed_u32::bit_and(a1441, a1442)
        };
        let a1444 = 0u32;
        if equal(a1443, a1444) {
            let a1447 = {
                let a1445 = v0;
                let a1446 = 2u32;
                fixed_u32::bit_and(a1445, a1446)
            };
            let a1448 = 0u32;
            if equal(a1447, a1448) {
                let a1451 = {
                    let a1449 = v0;
                    let a1450 = 1u32;
                    fixed_u32::bit_and(a1449, a1450)
                };
                let a1452 = 0u32;
                if equal(a1451, a1452) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1455 = {
                    let a1453 = v0;
                    let a1454 = 1u32;
                    fixed_u32::bit_and(a1453, a1454)
                };
                let a1456 = 0u32;
                if equal(a1455, a1456) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1459 = {
                let a1457 = v0;
                let a1458 = 2u32;
                fixed_u32::bit_and(a1457, a1458)
            };
            let a1460 = 0u32;
            if equal(a1459, a1460) {
                let a1463 = {
                    let a1461 = v0;
                    let a1462 = 1u32;
                    fixed_u32::bit_and(a1461, a1462)
                };
                let a1464 = 0u32;
                if equal(a1463, a1464) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1467 = {
                    let a1465 = v0;
                    let a1466 = 1u32;
                    fixed_u32::bit_and(a1465, a1466)
                };
                let a1468 = 0u32;
                if equal(a1467, a1468) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1471 = {
            let a1469 = v0;
            let a1470 = 4u32;
            fixed_u32::bit_and(a1469, a1470)
        };
        let a1472 = 0u32;
        if equal(a1471, a1472) {
            let a1475 = {
                let a1473 = v0;
                let a1474 = 2u32;
                fixed_u32::bit_and(a1473, a1474)
            };
            let a1476 = 0u32;
            if equal(a1475, a1476) {
                let a1479 = {
                    let a1477 = v0;
                    let a1478 = 1u32;
                    fixed_u32::bit_and(a1477, a1478)
                };
                let a1480 = 0u32;
                if equal(a1479, a1480) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1483 = {
                    let a1481 = v0;
                    let a1482 = 1u32;
                    fixed_u32::bit_and(a1481, a1482)
                };
                let a1484 = 0u32;
                if equal(a1483, a1484) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1487 = {
                let a1485 = v0;
                let a1486 = 2u32;
                fixed_u32::bit_and(a1485, a1486)
            };
            let a1488 = 0u32;
            if equal(a1487, a1488) {
                let a1491 = {
                    let a1489 = v0;
                    let a1490 = 1u32;
                    fixed_u32::bit_and(a1489, a1490)
                };
                let a1492 = 0u32;
                if equal(a1491, a1492) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1495 = {
                    let a1493 = v0;
                    let a1494 = 1u32;
                    fixed_u32::bit_and(a1493, a1494)
                };
                let a1496 = 0u32;
                if equal(a1495, a1496) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f318(v0: u32, v1: u32) -> u32 {
    let m1499 = {
        let a1497 = v0;
        let a1498 = v1;
        fixed_u32::shift_right(a1497, a1498)
    };
    match m1499 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f319(v0: Adt22, v1: Adt24) -> R<Adt22> {
    let v2: Adt25 = f354(Fn14::F355, 64u64, Adt25::C0(match v0.clone() {
        Adt22::C0(h1500, _, _, _, _, _, _, _) => {
            h1500
        }
    }, match v0.clone() {
        Adt22::C0(_, h1501, _, _, _, _, _, _) => {
            h1501
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h1502, _, _, _, _, _) => {
            h1502
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h1503, _, _, _, _) => {
            h1503
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h1504, _, _, _) => {
            h1504
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1505, _, _) => {
            h1505
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1506, _) => {
            h1506
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h1507) => {
            h1507
        }
    }, v1, 0u64))?;
    Ok(Adt22::C0(f356(match v0.clone() {
        Adt22::C0(h1508, _, _, _, _, _, _, _) => {
            h1508
        }
    }, match v2.clone() {
        Adt25::C0(h1509, _, _, _, _, _, _, _, _, _) => {
            h1509
        }
    }), f356(match v0.clone() {
        Adt22::C0(_, h1510, _, _, _, _, _, _) => {
            h1510
        }
    }, match v2.clone() {
        Adt25::C0(_, h1511, _, _, _, _, _, _, _, _) => {
            h1511
        }
    }), f356(match v0.clone() {
        Adt22::C0(_, _, h1512, _, _, _, _, _) => {
            h1512
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h1513, _, _, _, _, _, _, _) => {
            h1513
        }
    }), f356(match v0.clone() {
        Adt22::C0(_, _, _, h1514, _, _, _, _) => {
            h1514
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h1515, _, _, _, _, _, _) => {
            h1515
        }
    }), f356(match v0.clone() {
        Adt22::C0(_, _, _, _, h1516, _, _, _) => {
            h1516
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h1517, _, _, _, _, _) => {
            h1517
        }
    }), f356(match v0.clone() {
        Adt22::C0(_, _, _, _, _, h1518, _, _) => {
            h1518
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h1519, _, _, _, _) => {
            h1519
        }
    }), f356(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h1520, _) => {
            h1520
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h1521, _, _, _) => {
            h1521
        }
    }), f356(match v0 {
        Adt22::C0(_, _, _, _, _, _, _, h1522) => {
            h1522
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h1523, _, _) => {
            h1523
        }
    })))
}

pub fn f320(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f357(v0.clone(), v1, v2, v3)?, f357(v0.clone(), v1, v2, {
        let a1524 = v3;
        let a1525 = 4u64;
        nat_add(a1524, a1525)?
    })?, f357(v0.clone(), v1, v2, {
        let a1526 = v3;
        let a1527 = 8u64;
        nat_add(a1526, a1527)?
    })?, f357(v0.clone(), v1, v2, {
        let a1528 = v3;
        let a1529 = 12u64;
        nat_add(a1528, a1529)?
    })?, f357(v0.clone(), v1, v2, {
        let a1530 = v3;
        let a1531 = 16u64;
        nat_add(a1530, a1531)?
    })?, f357(v0.clone(), v1, v2, {
        let a1532 = v3;
        let a1533 = 20u64;
        nat_add(a1532, a1533)?
    })?, f357(v0.clone(), v1, v2, {
        let a1534 = v3;
        let a1535 = 24u64;
        nat_add(a1534, a1535)?
    })?, f357(v0.clone(), v1, v2, {
        let a1536 = v3;
        let a1537 = 28u64;
        nat_add(a1536, a1537)?
    })?, f357(v0.clone(), v1, v2, {
        let a1538 = v3;
        let a1539 = 32u64;
        nat_add(a1538, a1539)?
    })?, f357(v0.clone(), v1, v2, {
        let a1540 = v3;
        let a1541 = 36u64;
        nat_add(a1540, a1541)?
    })?, f357(v0.clone(), v1, v2, {
        let a1542 = v3;
        let a1543 = 40u64;
        nat_add(a1542, a1543)?
    })?, f357(v0.clone(), v1, v2, {
        let a1544 = v3;
        let a1545 = 44u64;
        nat_add(a1544, a1545)?
    })?, f357(v0.clone(), v1, v2, {
        let a1546 = v3;
        let a1547 = 48u64;
        nat_add(a1546, a1547)?
    })?, f357(v0.clone(), v1, v2, {
        let a1548 = v3;
        let a1549 = 52u64;
        nat_add(a1548, a1549)?
    })?, f357(v0.clone(), v1, v2, {
        let a1550 = v3;
        let a1551 = 56u64;
        nat_add(a1550, a1551)?
    })?, f357(v0, v1, v2, {
        let a1552 = v3;
        let a1553 = 60u64;
        nat_add(a1552, a1553)?
    })?))
}

pub fn f321(v0: Bytes, v1: u64) -> Option<u64> {
    let a1554 = 0u64;
    let a1555 = v1;
    if nat_lt(a1554, a1555) {
        if f358(v0, {
            let a1556 = v1;
            let a1557 = 1u64;
            nat_sub(a1556, a1557)
        }) {
            Some({
                let a1558 = v1;
                let a1559 = 1u64;
                nat_sub(a1558, a1559)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f322(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1560 = v2;
    let a1561 = v0;
    if nat_lt(a1560, a1561) {
        if f358(v1, v2) {
            Ok(Some({
                let a1562 = v2;
                let a1563 = 1u64;
                nat_add(a1562, a1563)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f323(v0: Bytes, p1643: u64, p1644: u64, p1645: bool, p1646: bool, p1647: Bytes) -> R<Bytes> {
    let m1642 = (p1643, (p1644, (p1645, (p1646, p1647))));
    {
        let mut m1642 = m1642;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1642;
                let m1564 = v2;
                if m1564 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1564 - 1;
                    let m1567 = {
                        let a1565 = v0.clone();
                        let a1566 = v1;
                        index_bytes(a1565, a1566)
                    };
                    match m1567 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1612 = {
                                    let a1588 = {
                                        let a1576 = {
                                            let a1570 = {
                                                let a1568 = v8;
                                                let a1569 = 128u8;
                                                fixed_u8::bit_and(a1568, a1569)
                                            };
                                            let a1571 = 128u8;
                                            if equal(a1570, a1571) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1577 = {
                                            let a1574 = {
                                                let a1572 = v8;
                                                let a1573 = 64u8;
                                                fixed_u8::bit_and(a1572, a1573)
                                            };
                                            let a1575 = 64u8;
                                            if equal(a1574, a1575) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1576, a1577)?
                                    };
                                    let a1589 = {
                                        let a1586 = {
                                            let a1580 = {
                                                let a1578 = v8;
                                                let a1579 = 32u8;
                                                fixed_u8::bit_and(a1578, a1579)
                                            };
                                            let a1581 = 32u8;
                                            if equal(a1580, a1581) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1587 = {
                                            let a1584 = {
                                                let a1582 = v8;
                                                let a1583 = 16u8;
                                                fixed_u8::bit_and(a1582, a1583)
                                            };
                                            let a1585 = 16u8;
                                            if equal(a1584, a1585) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1586, a1587)?
                                    };
                                    nat_add(a1588, a1589)?
                                };
                                let a1613 = {
                                    let a1610 = {
                                        let a1598 = {
                                            let a1592 = {
                                                let a1590 = v8;
                                                let a1591 = 8u8;
                                                fixed_u8::bit_and(a1590, a1591)
                                            };
                                            let a1593 = 8u8;
                                            if equal(a1592, a1593) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1599 = {
                                            let a1596 = {
                                                let a1594 = v8;
                                                let a1595 = 4u8;
                                                fixed_u8::bit_and(a1594, a1595)
                                            };
                                            let a1597 = 4u8;
                                            if equal(a1596, a1597) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1598, a1599)?
                                    };
                                    let a1611 = {
                                        let a1608 = {
                                            let a1602 = {
                                                let a1600 = v8;
                                                let a1601 = 2u8;
                                                fixed_u8::bit_and(a1600, a1601)
                                            };
                                            let a1603 = 2u8;
                                            if equal(a1602, a1603) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1609 = {
                                            let a1606 = {
                                                let a1604 = v8;
                                                let a1605 = 1u8;
                                                fixed_u8::bit_and(a1604, a1605)
                                            };
                                            let a1607 = 1u8;
                                            if equal(a1606, a1607) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1608, a1609)?
                                    };
                                    nat_add(a1610, a1611)?
                                };
                                nat_add(a1612, a1613)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1614 = v1;
                                    let a1615 = 1u64;
                                    nat_add(a1614, a1615)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1616 = v9;
                                    let a1617 = 34u64;
                                    !nat_eq(a1616, a1617)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1618 = v9;
                                    let a1619 = 92u64;
                                    nat_eq(a1618, a1619)
                                }, {
                                    let a1620 = v5;
                                    let a1621 = v10;
                                    append_bytes(a1620, a1621)
                                })))))
                            } else {
                                let a1622 = v9;
                                let a1623 = 32u64;
                                if if if if nat_eq(a1622, a1623) {
                                    true
                                } else {
                                    let a1624 = v9;
                                    let a1625 = 9u64;
                                    nat_eq(a1624, a1625)
                                } {
                                    true
                                } else {
                                    let a1626 = v9;
                                    let a1627 = 10u64;
                                    nat_eq(a1626, a1627)
                                } {
                                    true
                                } else {
                                    let a1628 = v9;
                                    let a1629 = 13u64;
                                    nat_eq(a1628, a1629)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1630 = v1;
                                        let a1631 = 1u64;
                                        nat_add(a1630, a1631)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1632 = v1;
                                            let a1633 = 4u64;
                                            nat_add(a1632, a1633)?
                                        }, (v6, (false, (false, {
                                            let a1634 = v5;
                                            let a1635 = f8(Str::lit("[]"));
                                            append_bytes(a1634, a1635)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1636 = v1;
                                            let a1637 = 1u64;
                                            nat_add(a1636, a1637)?
                                        }, (v6, ({
                                            let a1638 = v9;
                                            let a1639 = 34u64;
                                            nat_eq(a1638, a1639)
                                        }, (false, {
                                            let a1640 = v5;
                                            let a1641 = v10;
                                            append_bytes(a1640, a1641)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: Bytes, v1: u64) -> R<u64> {
    let m1650 = {
        let a1648 = v0;
        let a1649 = v1;
        index_bytes(a1648, a1649)
    };
    match m1650 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1695 = {
                let a1671 = {
                    let a1659 = {
                        let a1653 = {
                            let a1651 = v3;
                            let a1652 = 128u8;
                            fixed_u8::bit_and(a1651, a1652)
                        };
                        let a1654 = 128u8;
                        if equal(a1653, a1654) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1660 = {
                        let a1657 = {
                            let a1655 = v3;
                            let a1656 = 64u8;
                            fixed_u8::bit_and(a1655, a1656)
                        };
                        let a1658 = 64u8;
                        if equal(a1657, a1658) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1659, a1660)?
                };
                let a1672 = {
                    let a1669 = {
                        let a1663 = {
                            let a1661 = v3;
                            let a1662 = 32u8;
                            fixed_u8::bit_and(a1661, a1662)
                        };
                        let a1664 = 32u8;
                        if equal(a1663, a1664) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1670 = {
                        let a1667 = {
                            let a1665 = v3;
                            let a1666 = 16u8;
                            fixed_u8::bit_and(a1665, a1666)
                        };
                        let a1668 = 16u8;
                        if equal(a1667, a1668) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1669, a1670)?
                };
                nat_add(a1671, a1672)?
            };
            let a1696 = {
                let a1693 = {
                    let a1681 = {
                        let a1675 = {
                            let a1673 = v3;
                            let a1674 = 8u8;
                            fixed_u8::bit_and(a1673, a1674)
                        };
                        let a1676 = 8u8;
                        if equal(a1675, a1676) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1682 = {
                        let a1679 = {
                            let a1677 = v3;
                            let a1678 = 4u8;
                            fixed_u8::bit_and(a1677, a1678)
                        };
                        let a1680 = 4u8;
                        if equal(a1679, a1680) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1681, a1682)?
                };
                let a1694 = {
                    let a1691 = {
                        let a1685 = {
                            let a1683 = v3;
                            let a1684 = 2u8;
                            fixed_u8::bit_and(a1683, a1684)
                        };
                        let a1686 = 2u8;
                        if equal(a1685, a1686) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1692 = {
                        let a1689 = {
                            let a1687 = v3;
                            let a1688 = 1u8;
                            fixed_u8::bit_and(a1687, a1688)
                        };
                        let a1690 = 1u8;
                        if equal(a1689, a1690) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1691, a1692)?
                };
                nat_add(a1693, a1694)?
            };
            nat_add(a1695, a1696)
        }
    }
}

pub fn f325(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f359(Fn15::F360(v0.clone()), {
        let a1698 = {
            let a1697 = v0;
            length_bytes(a1697)
        };
        let a1699 = 1u64;
        nat_add(a1698, a1699)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1703 = {
        let (_, h1700) = {
            let (_, h1701) = {
                let (h1702, _) = v2.clone();
                h1702
            };
            h1701
        };
        h1700
    };
    let a1704 = 1u64;
    if nat_eq(a1703, a1704) {
        Ok(Some(({
            let (h1705, _) = {
                let (h1706, _) = v2.clone();
                h1706
            };
            h1705
        }, {
            let (h1707, _) = {
                let (_, h1708) = {
                    let (h1709, _) = v2;
                    h1709
                };
                h1708
            };
            h1707
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f326(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1710 = f324(v0.clone(), v1)?;
        let a1711 = 45u64;
        nat_eq(a1710, a1711)
    };
    let v3: u64 = if v2 {
        let a1712 = v1;
        let a1713 = 1u64;
        nat_add(a1712, a1713)?
    } else {
        v1
    };
    let v4: u64 = f361(v0.clone(), v3)?;
    let a1716 = 19u64;
    let a1717 = {
        let a1714 = v4;
        let a1715 = v3;
        nat_sub(a1714, a1715)
    };
    if nat_lt(a1716, a1717) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1720 = f362(f32(v0, v3, {
            let a1718 = v4;
            let a1719 = v3;
            nat_sub(a1718, a1719)
        }))?;
        match m1720 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f363(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f327(v0: u64) -> bool {
    let a1721 = v0;
    let a1722 = 95u64;
    if nat_eq(a1721, a1722) {
        true
    } else {
        let a1723 = v0;
        let a1724 = 45u64;
        if nat_eq(a1723, a1724) {
            true
        } else {
            let a1725 = v0;
            let a1726 = 32u64;
            if nat_eq(a1725, a1726) {
                true
            } else {
                let a1727 = v0;
                let a1728 = 46u64;
                if nat_eq(a1727, a1728) {
                    true
                } else {
                    let a1729 = v0;
                    let a1730 = 47u64;
                    nat_eq(a1729, a1730)
                }
            }
        }
    }
}

pub fn f328(v0: u64) -> bool {
    let a1731 = 97u64;
    let a1732 = v0;
    if nat_le(a1731, a1732) {
        let a1733 = v0;
        let a1734 = 122u64;
        nat_le(a1733, a1734)
    } else {
        false
    }
}

pub fn f329(v0: u64) -> bool {
    let a1735 = 65u64;
    let a1736 = v0;
    if nat_le(a1735, a1736) {
        let a1737 = v0;
        let a1738 = 90u64;
        nat_le(a1737, a1738)
    } else {
        false
    }
}

pub fn f330(v0: u64) -> bool {
    let a1739 = 48u64;
    let a1740 = v0;
    if nat_le(a1739, a1740) {
        let a1741 = v0;
        let a1742 = 57u64;
        nat_le(a1741, a1742)
    } else {
        false
    }
}

pub fn f331(p1750: u64, p1751: List<Adt23>, p1752: List<Bytes>, p1753: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1749 = (p1750, (p1751, (p1752, p1753)));
    {
        let mut m1749 = m1749;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1749;
                let m1743 = v0;
                if m1743 == 0 {
                    Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1743 - 1;
                    let m1744 = v1;
                    match m1744.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1745 = v5;
                            match m1745 {
                                Adt23::C0(v8, v9) => {
                                    if f36(v8.clone(), f364()) {
                                        Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f85(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1746 = f366(v8.clone(), v9)?;
                                            match m1746 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f332({
                                                        let (_, h1747) = v11;
                                                        h1747
                                                    }, List::cons(Adt23::C1({
                                                        let (h1748, _) = v11;
                                                        h1748
                                                    }), v6)), (List::cons(v8, v2), v3))))
                                                }
                                                Err(v10) => {
                                                    Err::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt23::C1(v7) => {
                                    Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                                Adt23::C2 => {
                                    if f85(f364(), v2.clone()) {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt23>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f364(), v2), List::cons(f365(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1749 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: List<Adt23>, v1: List<Adt23>) -> List<Adt23> {
    let m1754 = v0;
    match m1754.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f332(v3, v1))
        }
    }
}

pub fn f333(v0: List<(Bytes, Adt14)>) -> R<List<Adt23>> {
    let m1755 = v0;
    match m1755.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt23::C0(f218({
                let (h1756, _) = v1;
                h1756
            })?, {
                let (_, h1757) = v1;
                h1757
            }), f333(v2)?))
        }
    }
}

pub fn f334(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1758 = v0;
    match m1758 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f367(v1)
        }
    }
}

pub fn f335(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1759 = v0;
    match m1759.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f36(f145(v3.clone(), v1.clone()), v2.clone()) {
                f335(v4, v1, v2)
            } else {
                List::cons(v3, f335(v4, v1, v2))
            }
        }
    }
}

pub fn f336(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1760 = v0;
    match m1760.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f194(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f204(f345(f77(List::cons(v1, f368(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f336(v3, v1)?))
            }
        }
    }
}

pub fn f337(v0: u64) -> R<Bytes> {
    f369(f370(), 10u64, v0)
}

pub fn f338(v0: Fn9, p1765: u64, p1766: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1764 = (p1765, p1766);
    {
        let mut m1764 = m1764;
        loop {
            let step = {
                let (v1, v2) = m1764;
                let m1761 = v1;
                if m1761 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1761 - 1;
                    let m1763 = {
                        let c1762: Fn9 = v0.clone();
                        c1762.apply(v2.clone())?
                    };
                    match m1763 {
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
                Ok(next) => { m1764 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f371(v0, v1)
}

pub fn f340(v0: Fn10, p1770: List<(Bytes, Bytes)>, p1771: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1769 = (p1770, p1771);
    {
        let mut m1769 = m1769;
        loop {
            let step = {
                let (v1, v2) = m1769;
                let m1767 = v2;
                match m1767.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1768: Fn10 = v0.clone();
                            c1768.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f372(v0, v1)
}

pub fn f342(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f373(v0)
}

pub fn f343(v0: Fn11, p1775: List<Bytes>, p1776: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1774 = (p1775, p1776);
    {
        let mut m1774 = m1774;
        loop {
            let step = {
                let (v1, v2) = m1774;
                let m1772 = v2;
                match m1772.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1773: Fn11 = v0.clone();
                            c1773.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f374(v1)?, v0))
}

pub fn f345(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1777 = v0;
    match m1777.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f199(v1)?, f345(v2)?))
        }
    }
}

pub fn f346(v0: Fn12, p1781: List<List<(Bytes, ())>>, p1782: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1780 = (p1781, p1782);
    {
        let mut m1780 = m1780;
        loop {
            let step = {
                let (v1, v2) = m1780;
                let m1778 = v2;
                match m1778.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1779: Fn12 = v0.clone();
                            c1779.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1780 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f348(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1784 = {
        let a1783 = v0.clone();
        length_list(a1783)
    };
    let a1785 = 1u64;
    if nat_le(a1784, a1785) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f375(v0)?))
    }
}

pub fn f349(v0: Fn13, p1789: List<(Bytes, ())>, p1790: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1788 = (p1789, p1790);
    {
        let mut m1788 = m1788;
        loop {
            let step = {
                let (v1, v2) = m1788;
                let m1786 = v2;
                match m1786.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1787: Fn13 = v0.clone();
                            c1787.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1788 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f351(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f71(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f376(v3.clone(), f71(f72(v0, f88())?, f8(Str::lit("at"))));
    let a1792 = {
        let a1791 = v4.clone();
        length_list(a1791)
    };
    let a1793 = 0u64;
    if nat_eq(a1792, a1793) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a1796 = {
            let a1794 = v4;
            length_list(a1794)
        };
        let a1797 = {
            let a1795 = v3;
            length_list(a1795)
        };
        if nat_eq(a1796, a1797) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f377(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f352(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f214(f135(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f214(f135(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f63(f71(v3.clone(), f8(Str::lit("id"))), f71(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f71(f72(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f376(v5.clone(), f63(v6.clone(), f71(f72(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a1799 = {
        let a1798 = v7.clone();
        length_list(a1798)
    };
    let a1800 = 0u64;
    if nat_eq(a1799, a1800) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f74({
            let a1803 = {
                let a1801 = v7;
                length_list(a1801)
            };
            let a1804 = {
                let a1802 = v5;
                length_list(a1802)
            };
            nat_eq(a1803, a1804)
        }) {
            Ok((false, f377(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f77(f63(f376(f378(v4.clone()), f68(f69(v0, f8(Str::lit("model/authorities.toml"))))?), f376(f379(v4), f63(v6, f71(v3, f8(Str::lit("id")))))))?;
            let a1806 = {
                let a1805 = v8.clone();
                length_list(a1805)
            };
            let a1807 = 0u64;
            if nat_eq(a1806, a1807) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f377(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f120(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f353(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h1808, _) = v0;
        h1808
    }, {
        let (h1809, _) = v1;
        h1809
    }) {
        f36({
            let (_, h1810) = v0;
            h1810
        }, {
            let (_, h1811) = v1;
            h1811
        })
    } else {
        false
    }
}

pub fn f354(v0: Fn14, p1815: u64, p1816: Adt25) -> R<Adt25> {
    let m1814 = (p1815, p1816);
    {
        let mut m1814 = m1814;
        loop {
            let step = {
                let (v1, v2) = m1814;
                let m1812 = v1;
                if m1812 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m1812 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c1813: Fn14 = v0.clone();
                        c1813.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: Adt25) -> R<Adt25> {
    f380(v0)
}

pub fn f356(v0: u32, v1: u32) -> u32 {
    let m1819 = {
        let a1817 = f381(v0);
        let a1818 = f381(v1);
        fixed_u64::checked_add(a1817, a1818)
    };
    match m1819 {
        None => {
            0u32
        }
        Some(v2) => {
            f382(v2)
        }
    }
}

pub fn f357(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1822 = {
        let a1820 = v3;
        let a1821 = 8u64;
        nat_add(a1820, a1821)?
    };
    let a1823 = v2;
    if nat_eq(a1822, a1823) {
        Ok(f383({
            let a1826 = {
                let a1824 = v1;
                let a1825 = 8u64;
                nat_mul(a1824, a1825)?
            };
            let a1827 = 4294967296u64;
            let a1828 = 0u64;
            nat_quot(a1826, a1827, a1828)
        }))
    } else {
        let a1831 = {
            let a1829 = v3;
            let a1830 = 4u64;
            nat_add(a1829, a1830)?
        };
        let a1832 = v2;
        if nat_eq(a1831, a1832) {
            Ok(f383({
                let a1835 = {
                    let a1833 = v1;
                    let a1834 = 8u64;
                    nat_mul(a1833, a1834)?
                };
                let a1836 = 4294967296u64;
                let a1837 = 0u64;
                nat_rem(a1835, a1836, a1837)
            }))
        } else {
            Ok(f384(f385(v0.clone(), v1, v3), f385(v0.clone(), v1, {
                let a1838 = v3;
                let a1839 = 1u64;
                nat_add(a1838, a1839)?
            }), f385(v0.clone(), v1, {
                let a1840 = v3;
                let a1841 = 2u64;
                nat_add(a1840, a1841)?
            }), f385(v0, v1, {
                let a1842 = v3;
                let a1843 = 3u64;
                nat_add(a1842, a1843)?
            })))
        }
    }
}

pub fn f358(v0: Bytes, v1: u64) -> bool {
    let m1844 = f226(v0, v1);
    match m1844 {
        None => {
            false
        }
        Some(v2) => {
            f386(v2)
        }
    }
}

pub fn f359(v0: Fn15, p1849: u64, p1850: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1848 = (p1849, p1850);
    {
        let mut m1848 = m1848;
        loop {
            let step = {
                let (v1, v2) = m1848;
                let m1845 = v1;
                if m1845 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1845 - 1;
                    let m1847 = {
                        let c1846: Fn15 = v0.clone();
                        c1846.apply(v2.clone())?
                    };
                    match m1847 {
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
                Ok(next) => { m1848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f360(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f387(v0, v1)
}

pub fn f361(v0: Bytes, v1: u64) -> R<u64> {
    let (h1851, _) = f228(Fn3::F388(v0.clone()), {
        let a1853 = {
            let a1852 = v0;
            length_bytes(a1852)
        };
        let a1854 = 1u64;
        nat_add(a1853, a1854)?
    }, v1)?;
    Ok(h1851)
}

pub fn f362(v0: Bytes) -> R<Option<u64>> {
    if f389(v0.clone())? {
        Ok(Some({
            let (_, h1855) = {
                let (h1856, _) = f390(Fn16::F391(v0.clone()), {
                    let a1857 = v0;
                    length_bytes(a1857)
                }, (0u64, 0u64))?;
                h1856
            };
            h1855
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f363(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1858 = 0u64;
        let a1859 = v1;
        if nat_lt(a1858, a1859) {
            let a1860 = v1;
            let a1861 = 9223372036854775808u64;
            nat_le(a1860, a1861)
        } else {
            false
        }
    } else {
        let a1862 = v1;
        let a1863 = 9223372036854775807u64;
        nat_le(a1862, a1863)
    }
}

pub fn f364() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f365() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f366(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m1864 = f142(v1.clone(), f8(Str::lit("enum")));
    match m1864 {
        None => {
            let m1866 = f142(v1.clone(), f8(Str::lit("oneOf")));
            match m1866 {
                None => {
                    if f396(v1.clone(), v0.clone())? {
                        let m1867 = f397(v0.clone(), f334(f142(v1.clone(), f8(Str::lit("properties")))), f73(f146(f142(v1, f8(Str::lit("required"))))), true)?;
                        match m1867 {
                            Ok(v7) => {
                                Ok(f394(f398(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f204({
                                    let (h1868, _) = v7;
                                    h1868
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h1869) = v7;
                                    h1869
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt23>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m1870 = f334(f142(v1.clone(), f8(Str::lit("properties"))));
                        match m1870.uncons() {
                            None => {
                                let m1873 = f399(v1, {
                                    let a1871 = v0.clone();
                                    let a1872 = f8(Str::lit("Value"));
                                    append_bytes(a1871, a1872)
                                })?;
                                match m1873 {
                                    Ok(v9) => {
                                        Ok(f394(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h1874, _) = v9;
                                            h1874
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h1875) = v9;
                                            h1875
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt23>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m1876 = f397(v0.clone(), f334(f142(v1.clone(), f8(Str::lit("properties")))), f73(f146(f142(v1, f8(Str::lit("required"))))), false)?;
                                match m1876 {
                                    Ok(v13) => {
                                        Ok(f394(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m1878 = {
                                                let (h1877, _) = v13.clone();
                                                h1877
                                            };
                                            match m1878.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f204({
                                                        let (h1879, _) = v13;
                                                        h1879
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h1880) = v13;
                                            h1880
                                        }))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<(Bytes, List<Adt23>), Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f395(v0, f222(v5))
                }
            }
        }
        Some(v2) => {
            let m1865 = f392(f222(v2), List::<Bytes>::nil())?;
            match m1865 {
                Ok(v4) => {
                    Ok(f394(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f204(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt23>::nil()))
                }
                Err(v3) => {
                    Ok(f393(v3, v0))
                }
            }
        }
    }
}

pub fn f367(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1881 = v0;
    match m1881 {
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
        Adt14::C4(r1882) => {
            let v4 = (*r1882).clone();
            f367(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1883, r1884) => {
            let v2 = (*r1883).clone();
            let v3 = (*r1884).clone();
            List::cons((v1, v2), f367(v3))
        }
    }
}

pub fn f368(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1885 = v0;
    match m1885.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f368(v3, false))
            } else {
                f368(v3, true)
            }
        }
    }
}

pub fn f369(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1886 = v2;
    let a1887 = 0u64;
    if nat_eq(a1886, a1887) {
        Ok(f400(v0, 0u64))
    } else {
        let (_, h1888) = {
            let (h1889, _) = f338(Fn9::F401(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1889
        };
        Ok(h1888)
    }
}

pub fn f370() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f371(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1893 = {
        let a1891 = v0.clone();
        let a1892 = {
            let (h1890, _) = v1.clone();
            h1890
        };
        index_bytes(a1891, a1892)
    };
    match m1893 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1895 = {
                    let (h1894, _) = v1.clone();
                    h1894
                };
                let a1896 = 1u64;
                nat_add(a1895, a1896)?
            }, {
                let a1899 = {
                    let (_, h1897) = v1;
                    h1897
                };
                let a1900 = f402(v0, {
                    let (h1898, _) = v1;
                    h1898
                }, v2)?;
                append_bytes(a1899, a1900)
            })))
        }
    }
}

pub fn f372(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1901 = v0.clone();
    match m1901.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1904 = {
                let (h1902, _) = v2;
                h1902
            };
            let a1905 = {
                let (h1903, _) = v1.clone();
                h1903
            };
            if equal(a1904, a1905) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f373(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f403(f404(Fn17::F405, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1910 = {
        let (h1906, _) = f406(Fn18::F407, {
            let a1908 = {
                let a1907 = v0;
                length_list(a1907)
            };
            let a1909 = 1u64;
            nat_add(a1908, a1909)?
        }, v1)?;
        h1906
    };
    match m1910.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f374(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1915 = {
        let a1912 = f289({
            let (h1911, _) = v0;
            h1911
        })?;
        let a1913 = Bytes::lit(&[0x3a]);
        append_bytes(a1912, a1913)
    };
    let a1916 = {
        let (_, h1914) = v0;
        h1914
    };
    Ok(append_bytes(a1915, a1916))
}

pub fn f375(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f303({
        let (_, h1917) = {
            let (h1918, _) = f408(Fn19::F409, {
                let a1920 = {
                    let a1919 = v0.clone();
                    length_list(a1919)
                };
                let a1921 = 1u64;
                nat_add(a1920, a1921)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1918
        };
        h1917
    }))
}

pub fn f376(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1922 = v0;
    match m1922.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), v1.clone()) {
                f376(v3, v1)
            } else {
                List::cons(v2, f376(v3, v1))
            }
        }
    }
}

pub fn f377(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f378(v0: List<Adt14>) -> List<Bytes> {
    let m1923 = v0;
    match m1923.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1924 = f142(v1, f8(Str::lit("std")));
            match m1924 {
                None => {
                    f378(v2)
                }
                Some(v3) => {
                    List::cons(f145(v3, f8(Str::lit("authority"))), f378(v2))
                }
            }
        }
    }
}

pub fn f379(v0: List<Adt14>) -> List<Bytes> {
    let m1925 = v0;
    match m1925.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1926 = f150(f142(v1, f8(Str::lit("req"))));
            match m1926 {
                None => {
                    f379(v2)
                }
                Some(v3) => {
                    List::cons(v3, f379(v2))
                }
            }
        }
    }
}

pub fn f380(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f410(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h1927) => {
            h1927
        }
    });
    let v2: u32 = f356(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h1928, _, _) => {
            h1928
        }
    }, f356(f411(match v0.clone() {
        Adt25::C0(_, _, _, _, h1929, _, _, _, _, _) => {
            h1929
        }
    }), f356(f412(match v0.clone() {
        Adt25::C0(_, _, _, _, h1930, _, _, _, _, _) => {
            h1930
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1931, _, _, _, _) => {
            h1931
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1932, _, _, _) => {
            h1932
        }
    }), f356(v1, {
        let m1935 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h1934, _) => {
                h1934
            }
        };
        match m1935 {
            Adt24::C0(h1933, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1933
            }
        }
    }))));
    let v3: u32 = f356(f413(match v0.clone() {
        Adt25::C0(h1936, _, _, _, _, _, _, _, _, _) => {
            h1936
        }
    }), f414(match v0.clone() {
        Adt25::C0(h1937, _, _, _, _, _, _, _, _, _) => {
            h1937
        }
    }, match v0.clone() {
        Adt25::C0(_, h1938, _, _, _, _, _, _, _, _) => {
            h1938
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1939, _, _, _, _, _, _, _) => {
            h1939
        }
    }));
    Ok(Adt25::C0(f356(v2, v3), match v0.clone() {
        Adt25::C0(h1940, _, _, _, _, _, _, _, _, _) => {
            h1940
        }
    }, match v0.clone() {
        Adt25::C0(_, h1941, _, _, _, _, _, _, _, _) => {
            h1941
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h1942, _, _, _, _, _, _, _) => {
            h1942
        }
    }, f356(match v0.clone() {
        Adt25::C0(_, _, _, h1943, _, _, _, _, _, _) => {
            h1943
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h1944, _, _, _, _, _) => {
            h1944
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h1945, _, _, _, _) => {
            h1945
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h1946, _, _, _) => {
            h1946
        }
    }, f415(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h1947, _) => {
            h1947
        }
    }), {
        let a1949 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h1948) => {
                h1948
            }
        };
        let a1950 = 1u64;
        nat_add(a1949, a1950)?
    }))
}

pub fn f381(v0: u32) -> u64 {
    let m1952 = {
        let a1951 = v0;
        fixed_u64::convert(i128::from(a1951))
    };
    match m1952 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f382(v0: u64) -> u32 {
    let m1956 = {
        let a1955 = {
            let a1953 = v0;
            let a1954 = 4294967295u64;
            fixed_u64::bit_and(a1953, a1954)
        };
        fixed_u32::convert(i128::from(a1955))
    };
    match m1956 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f383(v0: u64) -> u32 {
    let (_, h1957) = {
        let (_, h1958) = f416(Fn20::F417, 32u64, (v0, (1u32, 0u32)));
        h1958
    };
    h1957
}

pub fn f384(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1963 = f418(v0, 24u32);
    let a1964 = {
        let a1961 = f418(v1, 16u32);
        let a1962 = {
            let a1959 = f418(v2, 8u32);
            let a1960 = v3;
            fixed_u32::bit_or(a1959, a1960)
        };
        fixed_u32::bit_or(a1961, a1962)
    };
    fixed_u32::bit_or(a1963, a1964)
}

pub fn f385(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1965 = v2;
    let a1966 = v1;
    if nat_lt(a1965, a1966) {
        let m1969 = {
            let a1967 = v0;
            let a1968 = v2;
            index_bytes(a1967, a1968)
        };
        match m1969 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1971 = {
                    let a1970 = v3;
                    fixed_u32::convert(i128::from(a1970))
                };
                match m1971 {
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
        let a1972 = v2;
        let a1973 = v1;
        if nat_eq(a1972, a1973) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f386(v0: u8) -> bool {
    let a1974 = v0;
    let a1975 = 32u8;
    if if if equal(a1974, a1975) {
        true
    } else {
        let a1976 = v0;
        let a1977 = 9u8;
        equal(a1976, a1977)
    } {
        true
    } else {
        let a1978 = v0;
        let a1979 = 10u8;
        equal(a1978, a1979)
    } {
        true
    } else {
        let a1980 = v0;
        let a1981 = 12u8;
        if equal(a1980, a1981) {
            true
        } else {
            let a1982 = v0;
            let a1983 = 13u8;
            equal(a1982, a1983)
        }
    }
}

pub fn f387(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1984, _) = v1.clone();
        h1984
    };
    let v3: Bytes = {
        let (h1985, _) = {
            let (_, h1986) = v1.clone();
            h1986
        };
        h1985
    };
    let a1989 = {
        let (_, h1987) = {
            let (_, h1988) = v1;
            h1988
        };
        h1987
    };
    let a1990 = 0u64;
    if nat_eq(a1989, a1990) {
        let v4: u64 = f324(v0.clone(), v2)?;
        let a1991 = v4;
        let a1992 = 34u64;
        if nat_eq(a1991, a1992) {
            Ok(Some(({
                let a1993 = v2;
                let a1994 = 1u64;
                nat_add(a1993, a1994)?
            }, (v3, 1u64))))
        } else {
            let a1995 = v4;
            let a1996 = 92u64;
            if nat_eq(a1995, a1996) {
                f419(v0, v2, v3)
            } else {
                let a1997 = v4;
                let a1998 = 32u64;
                if if nat_lt(a1997, a1998) {
                    true
                } else {
                    let a1999 = v4;
                    let a2000 = 256u64;
                    nat_eq(a1999, a2000)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a2001 = v2;
                        let a2002 = 1u64;
                        nat_add(a2001, a2002)?
                    }, ({
                        let a2003 = v3;
                        let a2004 = f32(v0, v2, 1u64);
                        append_bytes(a2003, a2004)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f388(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f420(v0, v1)
}

pub fn f389(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2005 = v0.clone();
        length_bytes(a2005)
    };
    let a2006 = 0u64;
    let a2007 = v1;
    if nat_lt(a2006, a2007) {
        let a2008 = f421(v0.clone())?;
        let a2009 = v1;
        if nat_eq(a2008, a2009) {
            let a2010 = 1u64;
            let a2011 = v1;
            if nat_lt(a2010, a2011) {
                let m2014 = {
                    let a2012 = v0;
                    let a2013 = 0u64;
                    index_bytes(a2012, a2013)
                };
                match m2014 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2015 = v2;
                        let a2016 = 48u8;
                        Ok(!equal(a2015, a2016))
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

pub fn f390(v0: Fn16, p2021: u64, p2022: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2020 = (p2021, p2022);
    {
        let mut m2020 = m2020;
        loop {
            let step = {
                let (v1, v2) = m2020;
                let m2017 = v1;
                if m2017 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2017 - 1;
                    let m2019 = {
                        let c2018: Fn16 = v0.clone();
                        c2018.apply(v2)?
                    };
                    match m2019 {
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
                Ok(next) => { m2020 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f422(v0, v1)
}

pub fn f392(v0: List<Adt14>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2023 = v0;
    match m2023.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v2, v3)) => {
            let m2024 = f423(v2.clone())?;
            match m2024 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f424(v2)?, List::<Bytes>::nil())))))
                }
                Some(v4) => {
                    if f85(v4.clone(), v1.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v4, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2025 = f392(v3, List::cons(v4.clone(), v1))?;
                        match m2025 {
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

pub fn f393(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt23>), Bytes> {
    Err::<(Bytes, List<Adt23>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f394(v0: Bytes, v1: List<Adt23>) -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((v0, v1))
}

pub fn f395(v0: Bytes, v1: List<Adt14>) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2026 = v1.clone();
    match m2026.uncons() {
        None => {
            Ok(f393(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2027 = f425(f334(f142(v2, f8(Str::lit("properties")))), v1.clone());
            match m2027.uncons() {
                None => {
                    Ok(f393(f8(Str::lit("untagged oneOf: no property is required by every branch with a const string")), v0))
                }
                Some((v4, v5)) => {
                    let m2028 = v5;
                    match m2028.uncons() {
                        None => {
                            let m2029 = f426(v0.clone(), v1, v4, List::<Bytes>::nil())?;
                            match m2029 {
                                Ok(v9) => {
                                    Ok(f394(f398(v0, {
                                        let (h2030, _) = v9;
                                        h2030
                                    }), {
                                        let (_, h2031) = v9;
                                        h2031
                                    }))
                                }
                                Err(v8) => {
                                    Ok(f393(v8, v0))
                                }
                            }
                        }
                        Some((v6, _)) => {
                            Ok(f393(f7(List::cons(f8(Str::lit("ambiguous oneOf: more than one property tags every branch (")), List::cons(v4, List::cons(f8(Str::lit(", ")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f396(v0: Adt14, v1: Bytes) -> R<bool> {
    let m2032 = v0;
    match m2032 {
        Adt14::C0(_) => {
            Ok(false)
        }
        Adt14::C1(_, _) => {
            Ok(false)
        }
        Adt14::C2(_) => {
            Ok(false)
        }
        Adt14::C3(r2033) => {
            let v2 = (*r2033).clone();
            f396(v2, v1)
        }
        Adt14::C4(r2034) => {
            let v3 = (*r2034).clone();
            f396(v3, v1)
        }
        Adt14::C5 => {
            Ok(false)
        }
        Adt14::C6(r2035, r2036) => {
            let v4 = (*r2035).clone();
            let v5 = (*r2036).clone();
            if f396(v4, v1.clone())? {
                Ok(true)
            } else {
                f396(v5, v1)
            }
        }
        Adt14::C7(v6, r2037, r2038) => {
            let v7 = (*r2037).clone();
            let v8 = (*r2038).clone();
            if if f36(v6, f8(Str::lit("$ref"))) {
                f427(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f396(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f396(v8, v1)
                }
            }
        }
    }
}

pub fn f397(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2039 = v1;
    match m2039.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v4, v5)) => {
            let m2041 = f428({
                let (h2040, _) = v4.clone();
                h2040
            })?;
            match m2041 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h2042, _) = v4;
                        h2042
                    }, List::<Bytes>::nil())))))
                }
                Some(v6) => {
                    if f85(v6.clone(), f429(v5.clone())?) {
                        Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v6, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2047 = f399({
                            let (_, h2043) = v4.clone();
                            h2043
                        }, {
                            let a2045 = v0.clone();
                            let a2046 = f218({
                                let (h2044, _) = v4.clone();
                                h2044
                            })?;
                            append_bytes(a2045, a2046)
                        })?;
                        match m2047 {
                            Ok(v8) => {
                                let v9: Bytes = if f85({
                                    let (h2048, _) = v4;
                                    h2048
                                }, v2.clone()) {
                                    let (h2049, _) = v8;
                                    h2049
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h2050, _) = v8;
                                        h2050
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m2051 = f397(v0, v5, v2, v3)?;
                                match m2051 {
                                    Ok(v11) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(if v3 {
                                            v9
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v6, List::cons(f8(Str::lit(", type: ")), List::cons(v9, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h2052, _) = v11;
                                            h2052
                                        }), f332({
                                            let (_, h2053) = v8;
                                            h2053
                                        }, {
                                            let (_, h2054) = v11;
                                            h2054
                                        }))))
                                    }
                                    Err(v10) => {
                                        Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v10))
                                    }
                                }
                            }
                            Err(v7) => {
                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v7))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f398(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f204(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f399(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2055 = f430(v0.clone());
    match m2055 {
        None => {
            if f36(f145(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2056 = f142(v0, f8(Str::lit("items")));
                match m2056 {
                    None => {
                        Ok(f431(f8(Str::lit("List")), f433()))
                    }
                    Some(v3) => {
                        Ok(f431(f8(Str::lit("List")), f432(v3, {
                            let a2057 = v1;
                            let a2058 = f8(Str::lit("Item"));
                            append_bytes(a2057, a2058)
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

pub fn f400(v0: Bytes, v1: u64) -> Bytes {
    let m2062 = {
        let a2059 = v0;
        let a2060 = v1;
        let a2061 = 1u64;
        slice_bytes(a2059, a2060, a2061)
    };
    match m2062 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f401(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f434(v1, v0, v2)
}

pub fn f402(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
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
                                    let a2126 = f435(v2)?;
                                    Ok(append_bytes(a2125, a2126))
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

pub fn f403(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f436(Fn21::F437, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f404(v0: Fn17, p2130: List<List<(Bytes, Bytes)>>, p2131: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2129 = (p2130, p2131);
    {
        let mut m2129 = m2129;
        loop {
            let step = {
                let (v1, v2) = m2129;
                let m2127 = v2;
                match m2127.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2128: Fn17 = v0.clone();
                            c2128.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f406(v0: Fn18, p2136: u64, p2137: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2135 = (p2136, p2137);
    {
        let mut m2135 = m2135;
        loop {
            let step = {
                let (v1, v2) = m2135;
                let m2132 = v1;
                if m2132 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2132 - 1;
                    let m2134 = {
                        let c2133: Fn18 = v0.clone();
                        c2133.apply(v2.clone())?
                    };
                    match m2134 {
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
                Ok(next) => { m2135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f438(v0)
}

pub fn f408(v0: Fn19, p2142: u64, p2143: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2141 = (p2142, p2143);
    {
        let mut m2141 = m2141;
        loop {
            let step = {
                let (v1, v2) = m2141;
                let m2138 = v1;
                if m2138 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2138 - 1;
                    let m2140 = {
                        let c2139: Fn19 = v0.clone();
                        c2139.apply(v2.clone())?
                    };
                    match m2140 {
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
                Ok(next) => { m2141 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f409(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f439(v0)
}

pub fn f410(v0: u64) -> u32 {
    let a2144 = v0;
    let a2145 = 32u64;
    if nat_lt(a2144, a2145) {
        let a2146 = v0;
        let a2147 = 16u64;
        if nat_lt(a2146, a2147) {
            let a2148 = v0;
            let a2149 = 8u64;
            if nat_lt(a2148, a2149) {
                let a2150 = v0;
                let a2151 = 4u64;
                if nat_lt(a2150, a2151) {
                    let a2152 = v0;
                    let a2153 = 2u64;
                    if nat_lt(a2152, a2153) {
                        let a2154 = v0;
                        let a2155 = 1u64;
                        if nat_lt(a2154, a2155) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2156 = v0;
                        let a2157 = 3u64;
                        if nat_lt(a2156, a2157) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2158 = v0;
                    let a2159 = 6u64;
                    if nat_lt(a2158, a2159) {
                        let a2160 = v0;
                        let a2161 = 5u64;
                        if nat_lt(a2160, a2161) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2162 = v0;
                        let a2163 = 7u64;
                        if nat_lt(a2162, a2163) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2164 = v0;
                let a2165 = 12u64;
                if nat_lt(a2164, a2165) {
                    let a2166 = v0;
                    let a2167 = 10u64;
                    if nat_lt(a2166, a2167) {
                        let a2168 = v0;
                        let a2169 = 9u64;
                        if nat_lt(a2168, a2169) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2170 = v0;
                        let a2171 = 11u64;
                        if nat_lt(a2170, a2171) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2172 = v0;
                    let a2173 = 14u64;
                    if nat_lt(a2172, a2173) {
                        let a2174 = v0;
                        let a2175 = 13u64;
                        if nat_lt(a2174, a2175) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2176 = v0;
                        let a2177 = 15u64;
                        if nat_lt(a2176, a2177) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2178 = v0;
            let a2179 = 24u64;
            if nat_lt(a2178, a2179) {
                let a2180 = v0;
                let a2181 = 20u64;
                if nat_lt(a2180, a2181) {
                    let a2182 = v0;
                    let a2183 = 18u64;
                    if nat_lt(a2182, a2183) {
                        let a2184 = v0;
                        let a2185 = 17u64;
                        if nat_lt(a2184, a2185) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2186 = v0;
                        let a2187 = 19u64;
                        if nat_lt(a2186, a2187) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2188 = v0;
                    let a2189 = 22u64;
                    if nat_lt(a2188, a2189) {
                        let a2190 = v0;
                        let a2191 = 21u64;
                        if nat_lt(a2190, a2191) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2192 = v0;
                        let a2193 = 23u64;
                        if nat_lt(a2192, a2193) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2194 = v0;
                let a2195 = 28u64;
                if nat_lt(a2194, a2195) {
                    let a2196 = v0;
                    let a2197 = 26u64;
                    if nat_lt(a2196, a2197) {
                        let a2198 = v0;
                        let a2199 = 25u64;
                        if nat_lt(a2198, a2199) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2200 = v0;
                        let a2201 = 27u64;
                        if nat_lt(a2200, a2201) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2202 = v0;
                    let a2203 = 30u64;
                    if nat_lt(a2202, a2203) {
                        let a2204 = v0;
                        let a2205 = 29u64;
                        if nat_lt(a2204, a2205) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2206 = v0;
                        let a2207 = 31u64;
                        if nat_lt(a2206, a2207) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2208 = v0;
        let a2209 = 48u64;
        if nat_lt(a2208, a2209) {
            let a2210 = v0;
            let a2211 = 40u64;
            if nat_lt(a2210, a2211) {
                let a2212 = v0;
                let a2213 = 36u64;
                if nat_lt(a2212, a2213) {
                    let a2214 = v0;
                    let a2215 = 34u64;
                    if nat_lt(a2214, a2215) {
                        let a2216 = v0;
                        let a2217 = 33u64;
                        if nat_lt(a2216, a2217) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2218 = v0;
                        let a2219 = 35u64;
                        if nat_lt(a2218, a2219) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2220 = v0;
                    let a2221 = 38u64;
                    if nat_lt(a2220, a2221) {
                        let a2222 = v0;
                        let a2223 = 37u64;
                        if nat_lt(a2222, a2223) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2224 = v0;
                        let a2225 = 39u64;
                        if nat_lt(a2224, a2225) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2226 = v0;
                let a2227 = 44u64;
                if nat_lt(a2226, a2227) {
                    let a2228 = v0;
                    let a2229 = 42u64;
                    if nat_lt(a2228, a2229) {
                        let a2230 = v0;
                        let a2231 = 41u64;
                        if nat_lt(a2230, a2231) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2232 = v0;
                        let a2233 = 43u64;
                        if nat_lt(a2232, a2233) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2234 = v0;
                    let a2235 = 46u64;
                    if nat_lt(a2234, a2235) {
                        let a2236 = v0;
                        let a2237 = 45u64;
                        if nat_lt(a2236, a2237) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2238 = v0;
                        let a2239 = 47u64;
                        if nat_lt(a2238, a2239) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2240 = v0;
            let a2241 = 56u64;
            if nat_lt(a2240, a2241) {
                let a2242 = v0;
                let a2243 = 52u64;
                if nat_lt(a2242, a2243) {
                    let a2244 = v0;
                    let a2245 = 50u64;
                    if nat_lt(a2244, a2245) {
                        let a2246 = v0;
                        let a2247 = 49u64;
                        if nat_lt(a2246, a2247) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2248 = v0;
                        let a2249 = 51u64;
                        if nat_lt(a2248, a2249) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2250 = v0;
                    let a2251 = 54u64;
                    if nat_lt(a2250, a2251) {
                        let a2252 = v0;
                        let a2253 = 53u64;
                        if nat_lt(a2252, a2253) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2254 = v0;
                        let a2255 = 55u64;
                        if nat_lt(a2254, a2255) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2256 = v0;
                let a2257 = 60u64;
                if nat_lt(a2256, a2257) {
                    let a2258 = v0;
                    let a2259 = 58u64;
                    if nat_lt(a2258, a2259) {
                        let a2260 = v0;
                        let a2261 = 57u64;
                        if nat_lt(a2260, a2261) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2262 = v0;
                        let a2263 = 59u64;
                        if nat_lt(a2262, a2263) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2264 = v0;
                    let a2265 = 62u64;
                    if nat_lt(a2264, a2265) {
                        let a2266 = v0;
                        let a2267 = 61u64;
                        if nat_lt(a2266, a2267) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2268 = v0;
                        let a2269 = 63u64;
                        if nat_lt(a2268, a2269) {
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

pub fn f411(v0: u32) -> u32 {
    let a2272 = {
        let a2270 = f440(v0, 6u32, 26u32);
        let a2271 = f440(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2270, a2271)
    };
    let a2273 = f440(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2272, a2273)
}

pub fn f412(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2279 = {
        let a2274 = v0;
        let a2275 = v1;
        fixed_u32::bit_and(a2274, a2275)
    };
    let a2280 = {
        let a2277 = {
            let a2276 = v0;
            fixed_u32::bit_not(a2276)
        };
        let a2278 = v2;
        fixed_u32::bit_and(a2277, a2278)
    };
    fixed_u32::bit_xor(a2279, a2280)
}

pub fn f413(v0: u32) -> u32 {
    let a2283 = {
        let a2281 = f440(v0, 2u32, 30u32);
        let a2282 = f440(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2281, a2282)
    };
    let a2284 = f440(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2283, a2284)
}

pub fn f414(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2293 = {
        let a2289 = {
            let a2285 = v0;
            let a2286 = v1;
            fixed_u32::bit_and(a2285, a2286)
        };
        let a2290 = {
            let a2287 = v0;
            let a2288 = v2;
            fixed_u32::bit_and(a2287, a2288)
        };
        fixed_u32::bit_xor(a2289, a2290)
    };
    let a2294 = {
        let a2291 = v1;
        let a2292 = v2;
        fixed_u32::bit_and(a2291, a2292)
    };
    fixed_u32::bit_xor(a2293, a2294)
}

pub fn f415(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h2295, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2295
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2296, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2296
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2297, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2297
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2298, _, _, _, _, _, _, _, _, _, _, _) => {
            h2298
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2299, _, _, _, _, _, _, _, _, _, _) => {
            h2299
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2300, _, _, _, _, _, _, _, _, _) => {
            h2300
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2301, _, _, _, _, _, _, _, _) => {
            h2301
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h2302, _, _, _, _, _, _, _) => {
            h2302
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2303, _, _, _, _, _, _) => {
            h2303
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h2304, _, _, _, _, _) => {
            h2304
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h2305, _, _, _, _) => {
            h2305
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2306, _, _, _) => {
            h2306
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2307, _, _) => {
            h2307
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2308, _) => {
            h2308
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2309) => {
            h2309
        }
    }, f356(f441(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2310, _) => {
            h2310
        }
    }), f356(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h2311, _, _, _, _, _, _) => {
            h2311
        }
    }, f356(f442(match v0.clone() {
        Adt24::C0(_, h2312, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2312
        }
    }), match v0 {
        Adt24::C0(h2313, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2313
        }
    }))))
}

pub fn f416(v0: Fn20, p2317: u64, p2318: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2316 = (p2317, p2318);
    {
        let mut m2316 = m2316;
        loop {
            let step = {
                let (v1, v2) = m2316;
                let m2314 = v1;
                if m2314 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2314 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2315: Fn20 = v0.clone();
                        c2315.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2319, _) = v0;
        h2319
    };
    let v2: u32 = {
        let (h2320, _) = {
            let (_, h2321) = v0;
            h2321
        };
        h2320
    };
    let v3: u32 = {
        let (_, h2322) = {
            let (_, h2323) = v0;
            h2323
        };
        h2322
    };
    ({
        let a2324 = v1;
        let a2325 = 2u64;
        let a2326 = 0u64;
        nat_quot(a2324, a2325, a2326)
    }, (f418(v2, 1u32), {
        let a2330 = {
            let a2327 = v1;
            let a2328 = 2u64;
            let a2329 = 0u64;
            nat_rem(a2327, a2328, a2329)
        };
        let a2331 = 1u64;
        if nat_eq(a2330, a2331) {
            let a2332 = v3;
            let a2333 = v2;
            fixed_u32::bit_or(a2332, a2333)
        } else {
            v3
        }
    }))
}

pub fn f418(v0: u32, v1: u32) -> u32 {
    let m2336 = {
        let a2334 = v0;
        let a2335 = v1;
        fixed_u32::shift_left(a2334, a2335)
    };
    match m2336 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f419(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f324(v0.clone(), {
        let a2337 = v1;
        let a2338 = 1u64;
        nat_add(a2337, a2338)?
    })?;
    let v4: Bytes = {
        let a2339 = v3;
        let a2340 = 117u64;
        if nat_eq(a2339, a2340) {
            f443(v0, v1)?
        } else {
            f444(v3)
        }
    };
    let a2341 = v4.clone();
    let a2342 = Bytes::lit(&[]);
    if equal(a2341, a2342) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2343 = v3;
            let a2344 = 117u64;
            if nat_eq(a2343, a2344) {
                let a2345 = v1;
                let a2346 = 6u64;
                nat_add(a2345, a2346)?
            } else {
                let a2347 = v1;
                let a2348 = 2u64;
                nat_add(a2347, a2348)?
            }
        }, ({
            let a2349 = v2;
            let a2350 = v4;
            append_bytes(a2349, a2350)
        }, 0u64))))
    }
}

pub fn f420(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f324(v0, v1)?;
    let a2351 = 47u64;
    let a2352 = v2;
    if if nat_lt(a2351, a2352) {
        let a2353 = v2;
        let a2354 = 58u64;
        nat_lt(a2353, a2354)
    } else {
        false
    } {
        Ok(Some({
            let a2355 = v1;
            let a2356 = 1u64;
            nat_add(a2355, a2356)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f421(v0: Bytes) -> R<u64> {
    let (h2357, _) = f228(Fn3::F445(v0.clone()), {
        let a2358 = v0;
        length_bytes(a2358)
    }, 0u64)?;
    Ok(h2357)
}

pub fn f422(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2362 = {
        let a2360 = v0;
        let a2361 = {
            let (h2359, _) = v1;
            h2359
        };
        index_bytes(a2360, a2361)
    };
    match m2362 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2364 = {
                    let (h2363, _) = v1;
                    h2363
                };
                let a2365 = 1u64;
                nat_add(a2364, a2365)?
            }, {
                let a2369 = {
                    let a2367 = {
                        let (_, h2366) = v1;
                        h2366
                    };
                    let a2368 = 10u64;
                    nat_mul(a2367, a2368)?
                };
                let a2370 = f446(v2)?;
                nat_add(a2369, a2370)?
            })))
        }
    }
}

pub fn f423(v0: Adt14) -> R<Option<Bytes>> {
    let m2371 = v0;
    match m2371 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v2, v3) => {
            Ok(Some({
                let a2374 = if if v2 {
                    let a2372 = 0u64;
                    let a2373 = v3;
                    nat_lt(a2372, a2373)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2375 = f288(false, v3)?;
                append_bytes(a2374, a2375)
            }))
        }
        Adt14::C2(v1) => {
            f428(v1)
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

pub fn f424(v0: Adt14) -> R<Bytes> {
    f221(v0)
}

pub fn f425(v0: List<(Bytes, Adt14)>, v1: List<Adt14>) -> List<Bytes> {
    let m2376 = v0;
    match m2376.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f447(v1.clone(), {
                let (h2377, _) = v2.clone();
                h2377
            }) {
                List::cons({
                    let (h2378, _) = v2;
                    h2378
                }, f425(v3, v1))
            } else {
                f425(v3, v1)
            }
        }
    }
}

pub fn f426(v0: Bytes, v1: List<Adt14>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt23>), Bytes>> {
    let m2379 = v1;
    match m2379.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::<Bytes>::nil(), List::<Adt23>::nil())))
        }
        Some((v4, v5)) => {
            let m2380 = f448(v4.clone(), v2.clone());
            match m2380 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v6) => {
                    let m2381 = f428(v6.clone())?;
                    match m2381 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f85(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2384 = f397({
                                    let a2382 = v0.clone();
                                    let a2383 = f218(v6)?;
                                    append_bytes(a2382, a2383)
                                }, f449(f334(f142(v4.clone(), f8(Str::lit("properties")))), v2.clone()), f73(f146(f142(v4, f8(Str::lit("required"))))), true)?;
                                match m2384 {
                                    Ok(v9) => {
                                        let m2385 = f426(v0, v5, v2, List::cons(v7.clone(), v3))?;
                                        match m2385 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt23>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f204({
                                                    let (h2386, _) = v9;
                                                    h2386
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h2387, _) = v11;
                                                    h2387
                                                }), f332({
                                                    let (_, h2388) = v9;
                                                    h2388
                                                }, {
                                                    let (_, h2389) = v11;
                                                    h2389
                                                }))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(List<Bytes>, List<Adt23>), Bytes>(v8))
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

pub fn f427(v0: Adt14, v1: Bytes) -> R<bool> {
    let m2390 = f150(Some(v0));
    match m2390 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2391 = f450(v2)?;
            match m2391 {
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

pub fn f428(v0: Bytes) -> R<Option<Bytes>> {
    if f451({
        let a2392 = v0.clone();
        length_bytes(a2392)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f452(v0)?;
        if f453(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2395 = {
                let a2393 = v1.clone();
                let a2394 = 0u64;
                index_bytes(a2393, a2394)
            };
            match m2395 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f330({
                        let v3: u8 = v2;
                        let a2440 = {
                            let a2416 = {
                                let a2404 = {
                                    let a2398 = {
                                        let a2396 = v3;
                                        let a2397 = 128u8;
                                        fixed_u8::bit_and(a2396, a2397)
                                    };
                                    let a2399 = 128u8;
                                    if equal(a2398, a2399) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2405 = {
                                    let a2402 = {
                                        let a2400 = v3;
                                        let a2401 = 64u8;
                                        fixed_u8::bit_and(a2400, a2401)
                                    };
                                    let a2403 = 64u8;
                                    if equal(a2402, a2403) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2404, a2405)?
                            };
                            let a2417 = {
                                let a2414 = {
                                    let a2408 = {
                                        let a2406 = v3;
                                        let a2407 = 32u8;
                                        fixed_u8::bit_and(a2406, a2407)
                                    };
                                    let a2409 = 32u8;
                                    if equal(a2408, a2409) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2415 = {
                                    let a2412 = {
                                        let a2410 = v3;
                                        let a2411 = 16u8;
                                        fixed_u8::bit_and(a2410, a2411)
                                    };
                                    let a2413 = 16u8;
                                    if equal(a2412, a2413) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2414, a2415)?
                            };
                            nat_add(a2416, a2417)?
                        };
                        let a2441 = {
                            let a2438 = {
                                let a2426 = {
                                    let a2420 = {
                                        let a2418 = v3;
                                        let a2419 = 8u8;
                                        fixed_u8::bit_and(a2418, a2419)
                                    };
                                    let a2421 = 8u8;
                                    if equal(a2420, a2421) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2427 = {
                                    let a2424 = {
                                        let a2422 = v3;
                                        let a2423 = 4u8;
                                        fixed_u8::bit_and(a2422, a2423)
                                    };
                                    let a2425 = 4u8;
                                    if equal(a2424, a2425) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2426, a2427)?
                            };
                            let a2439 = {
                                let a2436 = {
                                    let a2430 = {
                                        let a2428 = v3;
                                        let a2429 = 2u8;
                                        fixed_u8::bit_and(a2428, a2429)
                                    };
                                    let a2431 = 2u8;
                                    if equal(a2430, a2431) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a2437 = {
                                    let a2434 = {
                                        let a2432 = v3;
                                        let a2433 = 1u8;
                                        fixed_u8::bit_and(a2432, a2433)
                                    };
                                    let a2435 = 1u8;
                                    if equal(a2434, a2435) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a2436, a2437)?
                            };
                            nat_add(a2438, a2439)?
                        };
                        nat_add(a2440, a2441)?
                    }) {
                        Ok(Some({
                            let a2442 = f8(Str::lit("n"));
                            let a2443 = v1;
                            append_bytes(a2442, a2443)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f454({
            let a2444 = v0.clone();
            length_bytes(a2444)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f429(v0: List<(Bytes, Adt14)>) -> R<List<Bytes>> {
    let m2445 = v0;
    match m2445.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m2447 = f428({
                let (h2446, _) = v1;
                h2446
            })?;
            match m2447 {
                None => {
                    f429(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f429(v2)?))
                }
            }
        }
    }
}

pub fn f430(v0: Adt14) -> Option<Adt14> {
    let m2448 = f146(f142(v0, f8(Str::lit("oneOf"))));
    match m2448.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2449 = v2;
            match m2449.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2450 = v4;
                    match m2450.uncons() {
                        None => {
                            if f455(v3.clone()) {
                                Some(v1)
                            } else {
                                if f455(v1) {
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
    let m2451 = v1;
    match m2451 {
        Ok(v3) => {
            f394(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2452, _) = v3;
                h2452
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2453) = v3;
                h2453
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v2)
        }
    }
}

pub fn f432(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2454 = f430(v0.clone());
    match m2454 {
        None => {
            if f36(f145(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2455 = f142(v0, f8(Str::lit("items")));
                match m2455 {
                    None => {
                        Ok(f431(f8(Str::lit("List")), f433()))
                    }
                    Some(v3) => {
                        Ok(f431(f8(Str::lit("List")), f456(v3, {
                            let a2456 = v1;
                            let a2457 = f8(Str::lit("Item"));
                            append_bytes(a2456, a2457)
                        })?))
                    }
                }
            } else {
                f456(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f431(f8(Str::lit("Option")), f456(v2, v1)?))
        }
    }
}

pub fn f433() -> Result<(Bytes, List<Adt23>), Bytes> {
    Ok::<(Bytes, List<Adt23>), Bytes>((f364(), List::cons(Adt23::C2, List::<Adt23>::nil())))
}

pub fn f434(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
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
            let a2468 = f400(v0, {
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

pub fn f435(v0: u8) -> R<Bytes> {
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
    let a2522 = f400(f457(), {
        let a2516 = v2;
        let a2517 = 16u64;
        let a2518 = 0u64;
        nat_quot(a2516, a2517, a2518)
    });
    let a2523 = f400(f457(), {
        let a2519 = v2;
        let a2520 = 16u64;
        let a2521 = 0u64;
        nat_rem(a2519, a2520, a2521)
    });
    Ok(append_bytes(a2522, a2523))
}

pub fn f436(v0: Fn21, p2527: List<List<(Bytes, Bytes)>>, p2528: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
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
                            let c2525: Fn21 = v0.clone();
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

pub fn f437(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f438(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2530 = {
        let a2529 = v0.clone();
        length_list(a2529)
    };
    let a2531 = 1u64;
    if nat_le(a2530, a2531) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f458(v0)?))
    }
}

pub fn f439(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2532, _) = v0;
        h2532
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2533) = v0;
        h2533
    };
    let m2534 = v1;
    match m2534.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2535 = v4;
            match m2535.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f459(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f440(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2536 = f318(v0, v1);
    let a2537 = f418(v0, v2);
    fixed_u32::bit_or(a2536, a2537)
}

pub fn f441(v0: u32) -> u32 {
    let a2540 = {
        let a2538 = f440(v0, 17u32, 15u32);
        let a2539 = f440(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2538, a2539)
    };
    let a2541 = f318(v0, 10u32);
    fixed_u32::bit_xor(a2540, a2541)
}

pub fn f442(v0: u32) -> u32 {
    let a2544 = {
        let a2542 = f440(v0, 7u32, 25u32);
        let a2543 = f440(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2542, a2543)
    };
    let a2545 = f318(v0, 3u32);
    fixed_u32::bit_xor(a2544, a2545)
}

pub fn f443(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2548 = f32(v0.clone(), {
        let a2546 = v1;
        let a2547 = 2u64;
        nat_add(a2546, a2547)?
    }, 2u64);
    let a2549 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2548, a2549) {
        let m2552 = f460(f32(v0, {
            let a2550 = v1;
            let a2551 = 4u64;
            nat_add(a2550, a2551)?
        }, 2u64))?;
        match m2552 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f324(v2.clone(), 0u64)?;
                let a2553 = v3;
                let a2554 = 32u64;
                if nat_lt(a2553, a2554) {
                    if f461(v3) {
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

pub fn f444(v0: u64) -> Bytes {
    let a2555 = v0;
    let a2556 = 34u64;
    if nat_eq(a2555, a2556) {
        Bytes::lit(&[0x22])
    } else {
        let a2557 = v0;
        let a2558 = 92u64;
        if nat_eq(a2557, a2558) {
            Bytes::lit(&[0x5c])
        } else {
            let a2559 = v0;
            let a2560 = 98u64;
            if nat_eq(a2559, a2560) {
                Bytes::lit(&[0x08])
            } else {
                let a2561 = v0;
                let a2562 = 102u64;
                if nat_eq(a2561, a2562) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2563 = v0;
                    let a2564 = 110u64;
                    if nat_eq(a2563, a2564) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2565 = v0;
                        let a2566 = 114u64;
                        if nat_eq(a2565, a2566) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2567 = v0;
                            let a2568 = 116u64;
                            if nat_eq(a2567, a2568) {
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

pub fn f445(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2571 = {
        let a2569 = v0;
        let a2570 = v1;
        index_bytes(a2569, a2570)
    };
    match m2571 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f462(v2)? {
                Ok(Some({
                    let a2572 = v1;
                    let a2573 = 1u64;
                    nat_add(a2572, a2573)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f446(v0: u8) -> R<u64> {
    let a2620 = {
        let v1: u8 = v0;
        let a2618 = {
            let a2594 = {
                let a2582 = {
                    let a2576 = {
                        let a2574 = v1;
                        let a2575 = 128u8;
                        fixed_u8::bit_and(a2574, a2575)
                    };
                    let a2577 = 128u8;
                    if equal(a2576, a2577) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2583 = {
                    let a2580 = {
                        let a2578 = v1;
                        let a2579 = 64u8;
                        fixed_u8::bit_and(a2578, a2579)
                    };
                    let a2581 = 64u8;
                    if equal(a2580, a2581) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2582, a2583)?
            };
            let a2595 = {
                let a2592 = {
                    let a2586 = {
                        let a2584 = v1;
                        let a2585 = 32u8;
                        fixed_u8::bit_and(a2584, a2585)
                    };
                    let a2587 = 32u8;
                    if equal(a2586, a2587) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2593 = {
                    let a2590 = {
                        let a2588 = v1;
                        let a2589 = 16u8;
                        fixed_u8::bit_and(a2588, a2589)
                    };
                    let a2591 = 16u8;
                    if equal(a2590, a2591) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2592, a2593)?
            };
            nat_add(a2594, a2595)?
        };
        let a2619 = {
            let a2616 = {
                let a2604 = {
                    let a2598 = {
                        let a2596 = v1;
                        let a2597 = 8u8;
                        fixed_u8::bit_and(a2596, a2597)
                    };
                    let a2599 = 8u8;
                    if equal(a2598, a2599) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2605 = {
                    let a2602 = {
                        let a2600 = v1;
                        let a2601 = 4u8;
                        fixed_u8::bit_and(a2600, a2601)
                    };
                    let a2603 = 4u8;
                    if equal(a2602, a2603) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2604, a2605)?
            };
            let a2617 = {
                let a2614 = {
                    let a2608 = {
                        let a2606 = v1;
                        let a2607 = 2u8;
                        fixed_u8::bit_and(a2606, a2607)
                    };
                    let a2609 = 2u8;
                    if equal(a2608, a2609) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2615 = {
                    let a2612 = {
                        let a2610 = v1;
                        let a2611 = 1u8;
                        fixed_u8::bit_and(a2610, a2611)
                    };
                    let a2613 = 1u8;
                    if equal(a2612, a2613) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2614, a2615)?
            };
            nat_add(a2616, a2617)?
        };
        nat_add(a2618, a2619)?
    };
    let a2621 = 48u64;
    Ok(nat_sub(a2620, a2621))
}

pub fn f447(p2625: List<Adt14>, v1: Bytes) -> bool {
    let m2624 = p2625;
    {
        let mut m2624 = m2624;
        loop {
            let step = {
                let v0 = m2624;
                let m2622 = v0;
                match m2622.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m2623 = f448(v2, v1.clone());
                        if match m2623 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt14>, bool>(v3)
                        } else {
                            Err::<List<Adt14>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2624 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f448(v0: Adt14, v1: Bytes) -> Option<Bytes> {
    if f85(v1.clone(), f73(f146(f142(v0.clone(), f8(Str::lit("required")))))) {
        let m2626 = f142(v0, f8(Str::lit("properties")));
        match m2626 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m2627 = f142(v2, v1);
                match m2627 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f150(f142(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f449(v0: List<(Bytes, Adt14)>, v1: Bytes) -> List<(Bytes, Adt14)> {
    let m2628 = v0;
    match m2628.uncons() {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h2629, _) = v2.clone();
                h2629
            }, v1.clone()) {
                f449(v3, v1)
            } else {
                List::cons(v2, f449(v3, v1))
            }
        }
    }
}

pub fn f450(v0: Bytes) -> R<Option<Bytes>> {
    if f194(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f218(f32(v0.clone(), 8u64, {
            let a2631 = {
                let a2630 = v0;
                length_bytes(a2630)
            };
            let a2632 = 8u64;
            nat_sub(a2631, a2632)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f451(p2688: u64, v1: Bytes, p2689: u64) -> R<bool> {
    let m2687 = (p2688, p2689);
    {
        let mut m2687 = m2687;
        loop {
            let step = {
                let (v0, v2) = m2687;
                let m2633 = v0;
                if m2633 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2633 - 1;
                    let m2636 = {
                        let a2634 = v1.clone();
                        let a2635 = v2;
                        index_bytes(a2634, a2635)
                    };
                    match m2636 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2683 = {
                                let v5: u8 = v4;
                                let a2681 = {
                                    let a2657 = {
                                        let a2645 = {
                                            let a2639 = {
                                                let a2637 = v5;
                                                let a2638 = 128u8;
                                                fixed_u8::bit_and(a2637, a2638)
                                            };
                                            let a2640 = 128u8;
                                            if equal(a2639, a2640) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2646 = {
                                            let a2643 = {
                                                let a2641 = v5;
                                                let a2642 = 64u8;
                                                fixed_u8::bit_and(a2641, a2642)
                                            };
                                            let a2644 = 64u8;
                                            if equal(a2643, a2644) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2645, a2646)?
                                    };
                                    let a2658 = {
                                        let a2655 = {
                                            let a2649 = {
                                                let a2647 = v5;
                                                let a2648 = 32u8;
                                                fixed_u8::bit_and(a2647, a2648)
                                            };
                                            let a2650 = 32u8;
                                            if equal(a2649, a2650) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2656 = {
                                            let a2653 = {
                                                let a2651 = v5;
                                                let a2652 = 16u8;
                                                fixed_u8::bit_and(a2651, a2652)
                                            };
                                            let a2654 = 16u8;
                                            if equal(a2653, a2654) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2655, a2656)?
                                    };
                                    nat_add(a2657, a2658)?
                                };
                                let a2682 = {
                                    let a2679 = {
                                        let a2667 = {
                                            let a2661 = {
                                                let a2659 = v5;
                                                let a2660 = 8u8;
                                                fixed_u8::bit_and(a2659, a2660)
                                            };
                                            let a2662 = 8u8;
                                            if equal(a2661, a2662) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2668 = {
                                            let a2665 = {
                                                let a2663 = v5;
                                                let a2664 = 4u8;
                                                fixed_u8::bit_and(a2663, a2664)
                                            };
                                            let a2666 = 4u8;
                                            if equal(a2665, a2666) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2667, a2668)?
                                    };
                                    let a2680 = {
                                        let a2677 = {
                                            let a2671 = {
                                                let a2669 = v5;
                                                let a2670 = 2u8;
                                                fixed_u8::bit_and(a2669, a2670)
                                            };
                                            let a2672 = 2u8;
                                            if equal(a2671, a2672) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2678 = {
                                            let a2675 = {
                                                let a2673 = v5;
                                                let a2674 = 1u8;
                                                fixed_u8::bit_and(a2673, a2674)
                                            };
                                            let a2676 = 1u8;
                                            if equal(a2675, a2676) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2677, a2678)?
                                    };
                                    nat_add(a2679, a2680)?
                                };
                                nat_add(a2681, a2682)?
                            };
                            let a2684 = 128u64;
                            if nat_lt(a2683, a2684) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2685 = v2;
                                    let a2686 = 1u64;
                                    nat_add(a2685, a2686)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2687 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f452(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f284(v0.clone(), 0u64, {
        let a2690 = v0;
        length_bytes(a2690)
    }, false, Bytes::lit(&[]))?;
    let m2693 = {
        let a2691 = v1.clone();
        let a2692 = 0u64;
        index_bytes(a2691, a2692)
    };
    match m2693 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2738 = {
                    let a2714 = {
                        let a2702 = {
                            let a2696 = {
                                let a2694 = v3;
                                let a2695 = 128u8;
                                fixed_u8::bit_and(a2694, a2695)
                            };
                            let a2697 = 128u8;
                            if equal(a2696, a2697) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2703 = {
                            let a2700 = {
                                let a2698 = v3;
                                let a2699 = 64u8;
                                fixed_u8::bit_and(a2698, a2699)
                            };
                            let a2701 = 64u8;
                            if equal(a2700, a2701) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2702, a2703)?
                    };
                    let a2715 = {
                        let a2712 = {
                            let a2706 = {
                                let a2704 = v3;
                                let a2705 = 32u8;
                                fixed_u8::bit_and(a2704, a2705)
                            };
                            let a2707 = 32u8;
                            if equal(a2706, a2707) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2713 = {
                            let a2710 = {
                                let a2708 = v3;
                                let a2709 = 16u8;
                                fixed_u8::bit_and(a2708, a2709)
                            };
                            let a2711 = 16u8;
                            if equal(a2710, a2711) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2712, a2713)?
                    };
                    nat_add(a2714, a2715)?
                };
                let a2739 = {
                    let a2736 = {
                        let a2724 = {
                            let a2718 = {
                                let a2716 = v3;
                                let a2717 = 8u8;
                                fixed_u8::bit_and(a2716, a2717)
                            };
                            let a2719 = 8u8;
                            if equal(a2718, a2719) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2725 = {
                            let a2722 = {
                                let a2720 = v3;
                                let a2721 = 4u8;
                                fixed_u8::bit_and(a2720, a2721)
                            };
                            let a2723 = 4u8;
                            if equal(a2722, a2723) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2724, a2725)?
                    };
                    let a2737 = {
                        let a2734 = {
                            let a2728 = {
                                let a2726 = v3;
                                let a2727 = 2u8;
                                fixed_u8::bit_and(a2726, a2727)
                            };
                            let a2729 = 2u8;
                            if equal(a2728, a2729) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2735 = {
                            let a2732 = {
                                let a2730 = v3;
                                let a2731 = 1u8;
                                fixed_u8::bit_and(a2730, a2731)
                            };
                            let a2733 = 1u8;
                            if equal(a2732, a2733) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2734, a2735)?
                    };
                    nat_add(a2736, a2737)?
                };
                nat_add(a2738, a2739)?
            };
            if f329(v4) {
                let a2746 = f32({
                    let a2740 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2740)
                }, {
                    let a2741 = v4;
                    let a2742 = 65u64;
                    nat_sub(a2741, a2742)
                }, 1u64);
                let a2747 = f32(v1.clone(), 1u64, {
                    let a2744 = {
                        let a2743 = v1;
                        length_bytes(a2743)
                    };
                    let a2745 = 1u64;
                    nat_sub(a2744, a2745)
                });
                Ok(append_bytes(a2746, a2747))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f453(v0: Bytes) -> R<bool> {
    let m2750 = {
        let a2748 = v0;
        let a2749 = 0u64;
        index_bytes(a2748, a2749)
    };
    match m2750 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f328({
                let v2: u8 = v1;
                let a2795 = {
                    let a2771 = {
                        let a2759 = {
                            let a2753 = {
                                let a2751 = v2;
                                let a2752 = 128u8;
                                fixed_u8::bit_and(a2751, a2752)
                            };
                            let a2754 = 128u8;
                            if equal(a2753, a2754) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2760 = {
                            let a2757 = {
                                let a2755 = v2;
                                let a2756 = 64u8;
                                fixed_u8::bit_and(a2755, a2756)
                            };
                            let a2758 = 64u8;
                            if equal(a2757, a2758) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2759, a2760)?
                    };
                    let a2772 = {
                        let a2769 = {
                            let a2763 = {
                                let a2761 = v2;
                                let a2762 = 32u8;
                                fixed_u8::bit_and(a2761, a2762)
                            };
                            let a2764 = 32u8;
                            if equal(a2763, a2764) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2770 = {
                            let a2767 = {
                                let a2765 = v2;
                                let a2766 = 16u8;
                                fixed_u8::bit_and(a2765, a2766)
                            };
                            let a2768 = 16u8;
                            if equal(a2767, a2768) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2769, a2770)?
                    };
                    nat_add(a2771, a2772)?
                };
                let a2796 = {
                    let a2793 = {
                        let a2781 = {
                            let a2775 = {
                                let a2773 = v2;
                                let a2774 = 8u8;
                                fixed_u8::bit_and(a2773, a2774)
                            };
                            let a2776 = 8u8;
                            if equal(a2775, a2776) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2782 = {
                            let a2779 = {
                                let a2777 = v2;
                                let a2778 = 4u8;
                                fixed_u8::bit_and(a2777, a2778)
                            };
                            let a2780 = 4u8;
                            if equal(a2779, a2780) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2781, a2782)?
                    };
                    let a2794 = {
                        let a2791 = {
                            let a2785 = {
                                let a2783 = v2;
                                let a2784 = 2u8;
                                fixed_u8::bit_and(a2783, a2784)
                            };
                            let a2786 = 2u8;
                            if equal(a2785, a2786) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2792 = {
                            let a2789 = {
                                let a2787 = v2;
                                let a2788 = 1u8;
                                fixed_u8::bit_and(a2787, a2788)
                            };
                            let a2790 = 1u8;
                            if equal(a2789, a2790) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2791, a2792)?
                    };
                    nat_add(a2793, a2794)?
                };
                nat_add(a2795, a2796)?
            }) {
                Ok(true)
            } else {
                Ok(f329({
                    let v3: u8 = v1;
                    let a2841 = {
                        let a2817 = {
                            let a2805 = {
                                let a2799 = {
                                    let a2797 = v3;
                                    let a2798 = 128u8;
                                    fixed_u8::bit_and(a2797, a2798)
                                };
                                let a2800 = 128u8;
                                if equal(a2799, a2800) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a2806 = {
                                let a2803 = {
                                    let a2801 = v3;
                                    let a2802 = 64u8;
                                    fixed_u8::bit_and(a2801, a2802)
                                };
                                let a2804 = 64u8;
                                if equal(a2803, a2804) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2805, a2806)?
                        };
                        let a2818 = {
                            let a2815 = {
                                let a2809 = {
                                    let a2807 = v3;
                                    let a2808 = 32u8;
                                    fixed_u8::bit_and(a2807, a2808)
                                };
                                let a2810 = 32u8;
                                if equal(a2809, a2810) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a2816 = {
                                let a2813 = {
                                    let a2811 = v3;
                                    let a2812 = 16u8;
                                    fixed_u8::bit_and(a2811, a2812)
                                };
                                let a2814 = 16u8;
                                if equal(a2813, a2814) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2815, a2816)?
                        };
                        nat_add(a2817, a2818)?
                    };
                    let a2842 = {
                        let a2839 = {
                            let a2827 = {
                                let a2821 = {
                                    let a2819 = v3;
                                    let a2820 = 8u8;
                                    fixed_u8::bit_and(a2819, a2820)
                                };
                                let a2822 = 8u8;
                                if equal(a2821, a2822) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a2828 = {
                                let a2825 = {
                                    let a2823 = v3;
                                    let a2824 = 4u8;
                                    fixed_u8::bit_and(a2823, a2824)
                                };
                                let a2826 = 4u8;
                                if equal(a2825, a2826) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2827, a2828)?
                        };
                        let a2840 = {
                            let a2837 = {
                                let a2831 = {
                                    let a2829 = v3;
                                    let a2830 = 2u8;
                                    fixed_u8::bit_and(a2829, a2830)
                                };
                                let a2832 = 2u8;
                                if equal(a2831, a2832) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a2838 = {
                                let a2835 = {
                                    let a2833 = v3;
                                    let a2834 = 1u8;
                                    fixed_u8::bit_and(a2833, a2834)
                                };
                                let a2836 = 1u8;
                                if equal(a2835, a2836) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a2837, a2838)?
                        };
                        nat_add(a2839, a2840)?
                    };
                    nat_add(a2841, a2842)?
                }))
            }
        }
    }
}

pub fn f454(p2855: u64, v1: Bytes, p2856: u64, p2857: Bytes) -> R<Option<Bytes>> {
    let m2854 = (p2855, (p2856, p2857));
    {
        let mut m2854 = m2854;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2854;
                let m2843 = v0;
                if m2843 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2843 - 1;
                    let a2845 = {
                        let a2844 = v1.clone();
                        length_bytes(a2844)
                    };
                    let a2846 = v2;
                    if nat_le(a2845, a2846) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2847 = f463(v1.clone(), v2)?;
                        match m2847 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2849 = v2;
                                    let a2850 = {
                                        let (_, h2848) = v5;
                                        h2848
                                    };
                                    nat_add(a2849, a2850)?
                                }, {
                                    let a2852 = v3;
                                    let a2853 = f464({
                                        let (h2851, _) = v5;
                                        h2851
                                    });
                                    append_bytes(a2852, a2853)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2854 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455(v0: Adt14) -> bool {
    f36(f145(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f456(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m2858 = v0.clone();
    match m2858 {
        Adt14::C0(v2) => {
            if v2 {
                Ok(f433())
            } else {
                Ok(f393(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt14::C1(_, _) => {
            Ok(f393(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C2(_) => {
            Ok(f393(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C3(_) => {
            Ok(f393(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C4(_) => {
            let m2859 = f465(f367(v0.clone()));
            match m2859 {
                None => {
                    let m2860 = f150(f142(v0.clone(), f8(Str::lit("$ref"))));
                    match m2860 {
                        None => {
                            let m2862 = f142(v0.clone(), f8(Str::lit("allOf")));
                            match m2862 {
                                None => {
                                    let m2867 = f142(v0.clone(), f8(Str::lit("enum")));
                                    match m2867 {
                                        None => {
                                            let m2868 = f142(v0.clone(), f8(Str::lit("const")));
                                            match m2868 {
                                                None => {
                                                    let m2870 = f142(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m2870 {
                                                        None => {
                                                            let v27: Bytes = f145(v0.clone(), f8(Str::lit("type")));
                                                            if if f36(v27.clone(), Bytes::lit(&[])) {
                                                                !f466(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f433())
                                                            } else {
                                                                if f36(v27.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f394(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                                                } else {
                                                                    if f36(v27.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f394(f467(v0), List::<Adt23>::nil()))
                                                                    } else {
                                                                        if f36(v27.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f394(f468(), List::<Adt23>::nil()))
                                                                        } else {
                                                                            if f36(v27.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f394(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                                                            } else {
                                                                                if f36(v27.clone(), f8(Str::lit("object"))) {
                                                                                    let m2871 = f334(f142(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m2871.uncons() {
                                                                                        None => {
                                                                                            let m2872 = f142(v0, f8(Str::lit("additionalProperties")));
                                                                                            match m2872 {
                                                                                                None => {
                                                                                                    Ok(f431(f8(Str::lit("List")), f469(f433())))
                                                                                                }
                                                                                                Some(v30) => {
                                                                                                    Ok(f431(f8(Str::lit("List")), f469(f470(v30, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f394(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f393(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v27, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f394(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v24) => {
                                                    let m2869 = f150(Some(v24));
                                                    match m2869 {
                                                        None => {
                                                            Ok(f393(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f394(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f394(v1.clone(), List::cons(Adt23::C0(v1, v0), List::<Adt23>::nil())))
                                        }
                                    }
                                }
                                Some(v16) => {
                                    let m2863 = f222(v16);
                                    match m2863.uncons() {
                                        None => {
                                            Ok(f393(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v17, v18)) => {
                                            let m2864 = v18;
                                            match m2864.uncons() {
                                                None => {
                                                    let m2865 = f150(f142(v17, f8(Str::lit("$ref"))));
                                                    match m2865 {
                                                        None => {
                                                            Ok(f393(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v21) => {
                                                            let m2866 = f450(v21)?;
                                                            match m2866 {
                                                                None => {
                                                                    Ok(f393(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v22) => {
                                                                    Ok(f394(v22, List::<Adt23>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f393(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v14) => {
                            let m2861 = f450(v14)?;
                            match m2861 {
                                None => {
                                    Ok(f393(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v15) => {
                                    Ok(f394(v15, List::<Adt23>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v13) => {
                    Ok(f393(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v13, List::<Bytes>::nil()))), v1))
                }
            }
        }
        Adt14::C5 => {
            Ok(f433())
        }
        Adt14::C6(_, _) => {
            Ok(f393(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt14::C7(_, _, _) => {
            Ok(f393(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f457() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f458(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f403({
        let (_, h2873) = {
            let (h2874, _) = f471(Fn22::F472, {
                let a2876 = {
                    let a2875 = v0.clone();
                    length_list(a2875)
                };
                let a2877 = 1u64;
                nat_add(a2876, a2877)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2874
        };
        h2873
    }))
}

pub fn f459(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2882 = {
            let a2880 = {
                let a2878 = v0.clone();
                length_list(a2878)
            };
            let a2881 = {
                let a2879 = v1.clone();
                length_list(a2879)
            };
            nat_add(a2880, a2881)?
        };
        let a2883 = 1u64;
        nat_add(a2882, a2883)?
    };
    Ok(f308({
        let (_, h2884) = {
            let (_, h2885) = {
                let (h2886, _) = f473(Fn23::F474, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2886
            };
            h2885
        };
        h2884
    }))
}

pub fn f460(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2887 = v0.clone();
        length_bytes(a2887)
    };
    let a2891 = {
        let a2888 = v1;
        let a2889 = 2u64;
        let a2890 = 0u64;
        nat_rem(a2888, a2889, a2890)
    };
    let a2892 = 0u64;
    if nat_eq(a2891, a2892) {
        let (_, h2893) = {
            let (h2894, _) = f475(Fn24::F476(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2894
        };
        Ok(h2893)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f461(v0: u64) -> bool {
    let a2895 = v0;
    let a2896 = 8u64;
    if nat_eq(a2895, a2896) {
        true
    } else {
        let a2897 = v0;
        let a2898 = 9u64;
        if nat_eq(a2897, a2898) {
            true
        } else {
            let a2899 = v0;
            let a2900 = 10u64;
            if nat_eq(a2899, a2900) {
                true
            } else {
                let a2901 = v0;
                let a2902 = 12u64;
                if nat_eq(a2901, a2902) {
                    true
                } else {
                    let a2903 = v0;
                    let a2904 = 13u64;
                    nat_eq(a2903, a2904)
                }
            }
        }
    }
}

pub fn f462(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2949 = {
            let a2925 = {
                let a2913 = {
                    let a2907 = {
                        let a2905 = v1;
                        let a2906 = 128u8;
                        fixed_u8::bit_and(a2905, a2906)
                    };
                    let a2908 = 128u8;
                    if equal(a2907, a2908) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2914 = {
                    let a2911 = {
                        let a2909 = v1;
                        let a2910 = 64u8;
                        fixed_u8::bit_and(a2909, a2910)
                    };
                    let a2912 = 64u8;
                    if equal(a2911, a2912) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2913, a2914)?
            };
            let a2926 = {
                let a2923 = {
                    let a2917 = {
                        let a2915 = v1;
                        let a2916 = 32u8;
                        fixed_u8::bit_and(a2915, a2916)
                    };
                    let a2918 = 32u8;
                    if equal(a2917, a2918) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2924 = {
                    let a2921 = {
                        let a2919 = v1;
                        let a2920 = 16u8;
                        fixed_u8::bit_and(a2919, a2920)
                    };
                    let a2922 = 16u8;
                    if equal(a2921, a2922) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2923, a2924)?
            };
            nat_add(a2925, a2926)?
        };
        let a2950 = {
            let a2947 = {
                let a2935 = {
                    let a2929 = {
                        let a2927 = v1;
                        let a2928 = 8u8;
                        fixed_u8::bit_and(a2927, a2928)
                    };
                    let a2930 = 8u8;
                    if equal(a2929, a2930) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2936 = {
                    let a2933 = {
                        let a2931 = v1;
                        let a2932 = 4u8;
                        fixed_u8::bit_and(a2931, a2932)
                    };
                    let a2934 = 4u8;
                    if equal(a2933, a2934) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2935, a2936)?
            };
            let a2948 = {
                let a2945 = {
                    let a2939 = {
                        let a2937 = v1;
                        let a2938 = 2u8;
                        fixed_u8::bit_and(a2937, a2938)
                    };
                    let a2940 = 2u8;
                    if equal(a2939, a2940) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2946 = {
                    let a2943 = {
                        let a2941 = v1;
                        let a2942 = 1u8;
                        fixed_u8::bit_and(a2941, a2942)
                    };
                    let a2944 = 1u8;
                    if equal(a2943, a2944) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2945, a2946)?
            };
            nat_add(a2947, a2948)?
        };
        nat_add(a2949, a2950)?
    };
    let a2951 = 48u64;
    let a2952 = v2;
    if nat_le(a2951, a2952) {
        let a2953 = v2;
        let a2954 = 57u64;
        Ok(nat_le(a2953, a2954))
    } else {
        Ok(false)
    }
}

pub fn f463(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2957 = {
        let a2955 = v0.clone();
        let a2956 = v1;
        index_bytes(a2955, a2956)
    };
    match m2957 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3002 = {
                    let a2978 = {
                        let a2966 = {
                            let a2960 = {
                                let a2958 = v3;
                                let a2959 = 128u8;
                                fixed_u8::bit_and(a2958, a2959)
                            };
                            let a2961 = 128u8;
                            if equal(a2960, a2961) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2967 = {
                            let a2964 = {
                                let a2962 = v3;
                                let a2963 = 64u8;
                                fixed_u8::bit_and(a2962, a2963)
                            };
                            let a2965 = 64u8;
                            if equal(a2964, a2965) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2966, a2967)?
                    };
                    let a2979 = {
                        let a2976 = {
                            let a2970 = {
                                let a2968 = v3;
                                let a2969 = 32u8;
                                fixed_u8::bit_and(a2968, a2969)
                            };
                            let a2971 = 32u8;
                            if equal(a2970, a2971) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2977 = {
                            let a2974 = {
                                let a2972 = v3;
                                let a2973 = 16u8;
                                fixed_u8::bit_and(a2972, a2973)
                            };
                            let a2975 = 16u8;
                            if equal(a2974, a2975) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2976, a2977)?
                    };
                    nat_add(a2978, a2979)?
                };
                let a3003 = {
                    let a3000 = {
                        let a2988 = {
                            let a2982 = {
                                let a2980 = v3;
                                let a2981 = 8u8;
                                fixed_u8::bit_and(a2980, a2981)
                            };
                            let a2983 = 8u8;
                            if equal(a2982, a2983) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2989 = {
                            let a2986 = {
                                let a2984 = v3;
                                let a2985 = 4u8;
                                fixed_u8::bit_and(a2984, a2985)
                            };
                            let a2987 = 4u8;
                            if equal(a2986, a2987) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2988, a2989)?
                    };
                    let a3001 = {
                        let a2998 = {
                            let a2992 = {
                                let a2990 = v3;
                                let a2991 = 2u8;
                                fixed_u8::bit_and(a2990, a2991)
                            };
                            let a2993 = 2u8;
                            if equal(a2992, a2993) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2999 = {
                            let a2996 = {
                                let a2994 = v3;
                                let a2995 = 1u8;
                                fixed_u8::bit_and(a2994, a2995)
                            };
                            let a2997 = 1u8;
                            if equal(a2996, a2997) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2998, a2999)?
                    };
                    nat_add(a3000, a3001)?
                };
                nat_add(a3002, a3003)?
            };
            let a3004 = v4;
            let a3005 = 128u64;
            if nat_lt(a3004, a3005) {
                Ok(Some((v4, 1u64)))
            } else {
                let a3006 = 194u64;
                let a3007 = v4;
                if if nat_le(a3006, a3007) {
                    let a3008 = v4;
                    let a3009 = 223u64;
                    nat_le(a3008, a3009)
                } else {
                    false
                } {
                    let m3012 = f477(v0, {
                        let a3010 = v1;
                        let a3011 = 1u64;
                        nat_add(a3010, a3011)?
                    })?;
                    match m3012 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f478({
                                let a3017 = {
                                    let a3015 = {
                                        let a3013 = v4;
                                        let a3014 = 192u64;
                                        nat_sub(a3013, a3014)
                                    };
                                    let a3016 = 64u64;
                                    nat_mul(a3015, a3016)?
                                };
                                let a3018 = v5;
                                nat_add(a3017, a3018)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a3019 = 224u64;
                    let a3020 = v4;
                    if if nat_le(a3019, a3020) {
                        let a3021 = v4;
                        let a3022 = 239u64;
                        nat_le(a3021, a3022)
                    } else {
                        false
                    } {
                        let m3025 = f477(v0.clone(), {
                            let a3023 = v1;
                            let a3024 = 1u64;
                            nat_add(a3023, a3024)?
                        })?;
                        match m3025 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m3028 = f477(v0, {
                                    let a3026 = v1;
                                    let a3027 = 2u64;
                                    nat_add(a3026, a3027)?
                                })?;
                                match m3028 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f478({
                                            let a3037 = {
                                                let a3031 = {
                                                    let a3029 = v4;
                                                    let a3030 = 224u64;
                                                    nat_sub(a3029, a3030)
                                                };
                                                let a3032 = 4096u64;
                                                nat_mul(a3031, a3032)?
                                            };
                                            let a3038 = {
                                                let a3035 = {
                                                    let a3033 = v6;
                                                    let a3034 = 64u64;
                                                    nat_mul(a3033, a3034)?
                                                };
                                                let a3036 = v7;
                                                nat_add(a3035, a3036)?
                                            };
                                            nat_add(a3037, a3038)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a3039 = 240u64;
                        let a3040 = v4;
                        if if nat_le(a3039, a3040) {
                            let a3041 = v4;
                            let a3042 = 244u64;
                            nat_le(a3041, a3042)
                        } else {
                            false
                        } {
                            let m3045 = f477(v0.clone(), {
                                let a3043 = v1;
                                let a3044 = 1u64;
                                nat_add(a3043, a3044)?
                            })?;
                            match m3045 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m3048 = f477(v0.clone(), {
                                        let a3046 = v1;
                                        let a3047 = 2u64;
                                        nat_add(a3046, a3047)?
                                    })?;
                                    match m3048 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m3051 = f477(v0, {
                                                let a3049 = v1;
                                                let a3050 = 3u64;
                                                nat_add(a3049, a3050)?
                                            })?;
                                            match m3051 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f478({
                                                        let a3064 = {
                                                            let a3054 = {
                                                                let a3052 = v4;
                                                                let a3053 = 240u64;
                                                                nat_sub(a3052, a3053)
                                                            };
                                                            let a3055 = 262144u64;
                                                            nat_mul(a3054, a3055)?
                                                        };
                                                        let a3065 = {
                                                            let a3062 = {
                                                                let a3056 = v8;
                                                                let a3057 = 4096u64;
                                                                nat_mul(a3056, a3057)?
                                                            };
                                                            let a3063 = {
                                                                let a3060 = {
                                                                    let a3058 = v9;
                                                                    let a3059 = 64u64;
                                                                    nat_mul(a3058, a3059)?
                                                                };
                                                                let a3061 = v10;
                                                                nat_add(a3060, a3061)?
                                                            };
                                                            nat_add(a3062, a3063)?
                                                        };
                                                        nat_add(a3064, a3065)?
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

pub fn f464(v0: u64) -> Bytes {
    let a3066 = f8(Str::lit("u"));
    let a3067 = f479(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3066, a3067)
}

pub fn f465(p3073: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m3072 = p3073;
    {
        let mut m3072 = m3072;
        loop {
            let step = {
                let v0 = m3072;
                let m3068 = v0;
                match m3068.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f85({
                            let (h3069, _) = v1.clone();
                            h3069
                        }, f480()) {
                            true
                        } else {
                            f85({
                                let (h3070, _) = v1.clone();
                                h3070
                            }, f481())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h3071, _) = v1;
                                h3071
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3072 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(v0: Adt14) -> bool {
    if f482(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f482(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f482(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f482(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    f482(v0, f8(Str::lit("additionalProperties")))
                }
            }
        }
    }
}

pub fn f467(v0: Adt14) -> Bytes {
    let m3074 = f300(f142(v0, f8(Str::lit("minimum"))));
    match m3074 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f468() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f469(v0: Result<(Bytes, List<Adt23>), Bytes>) -> Result<(Bytes, List<Adt23>), Bytes> {
    let m3075 = v0;
    match m3075 {
        Ok(v2) => {
            f394(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h3076, _) = v2;
                h3076
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h3077) = v2;
                h3077
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt23>), Bytes>(v1)
        }
    }
}

pub fn f470(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    f483(16u64, v0, v1)
}

pub fn f471(v0: Fn22, p3082: u64, p3083: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3081 = (p3082, p3083);
    {
        let mut m3081 = m3081;
        loop {
            let step = {
                let (v1, v2) = m3081;
                let m3078 = v1;
                if m3078 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3078 - 1;
                    let m3080 = {
                        let c3079: Fn22 = v0.clone();
                        c3079.apply(v2.clone())?
                    };
                    match m3080 {
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
                Ok(next) => { m3081 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f484(v0)
}

pub fn f473(v0: Fn23, p3088: u64, p3089: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3087 = (p3088, p3089);
    {
        let mut m3087 = m3087;
        loop {
            let step = {
                let (v1, v2) = m3087;
                let m3084 = v1;
                if m3084 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3084 - 1;
                    let m3086 = {
                        let c3085: Fn23 = v0.clone();
                        c3085.apply(v2.clone())
                    };
                    match m3086 {
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
                Ok(next) => { m3087 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f474(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f485(v0)
}

pub fn f475(v0: Fn24, p3094: u64, p3095: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3093 = (p3094, p3095);
    {
        let mut m3093 = m3093;
        loop {
            let step = {
                let (v1, v2) = m3093;
                let m3090 = v1;
                if m3090 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3090 - 1;
                    let m3092 = {
                        let c3091: Fn24 = v0.clone();
                        c3091.apply(v2.clone())?
                    };
                    match m3092 {
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
                Ok(next) => { m3093 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f486(v0, v1)
}

pub fn f477(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3098 = {
        let a3096 = v0;
        let a3097 = v1;
        index_bytes(a3096, a3097)
    };
    match m3098 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a3143 = {
                    let a3119 = {
                        let a3107 = {
                            let a3101 = {
                                let a3099 = v3;
                                let a3100 = 128u8;
                                fixed_u8::bit_and(a3099, a3100)
                            };
                            let a3102 = 128u8;
                            if equal(a3101, a3102) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3108 = {
                            let a3105 = {
                                let a3103 = v3;
                                let a3104 = 64u8;
                                fixed_u8::bit_and(a3103, a3104)
                            };
                            let a3106 = 64u8;
                            if equal(a3105, a3106) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3107, a3108)?
                    };
                    let a3120 = {
                        let a3117 = {
                            let a3111 = {
                                let a3109 = v3;
                                let a3110 = 32u8;
                                fixed_u8::bit_and(a3109, a3110)
                            };
                            let a3112 = 32u8;
                            if equal(a3111, a3112) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3118 = {
                            let a3115 = {
                                let a3113 = v3;
                                let a3114 = 16u8;
                                fixed_u8::bit_and(a3113, a3114)
                            };
                            let a3116 = 16u8;
                            if equal(a3115, a3116) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3117, a3118)?
                    };
                    nat_add(a3119, a3120)?
                };
                let a3144 = {
                    let a3141 = {
                        let a3129 = {
                            let a3123 = {
                                let a3121 = v3;
                                let a3122 = 8u8;
                                fixed_u8::bit_and(a3121, a3122)
                            };
                            let a3124 = 8u8;
                            if equal(a3123, a3124) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3130 = {
                            let a3127 = {
                                let a3125 = v3;
                                let a3126 = 4u8;
                                fixed_u8::bit_and(a3125, a3126)
                            };
                            let a3128 = 4u8;
                            if equal(a3127, a3128) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3129, a3130)?
                    };
                    let a3142 = {
                        let a3139 = {
                            let a3133 = {
                                let a3131 = v3;
                                let a3132 = 2u8;
                                fixed_u8::bit_and(a3131, a3132)
                            };
                            let a3134 = 2u8;
                            if equal(a3133, a3134) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3140 = {
                            let a3137 = {
                                let a3135 = v3;
                                let a3136 = 1u8;
                                fixed_u8::bit_and(a3135, a3136)
                            };
                            let a3138 = 1u8;
                            if equal(a3137, a3138) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3139, a3140)?
                    };
                    nat_add(a3141, a3142)?
                };
                nat_add(a3143, a3144)?
            };
            let a3145 = 128u64;
            let a3146 = v4;
            if if nat_le(a3145, a3146) {
                let a3147 = v4;
                let a3148 = 191u64;
                nat_le(a3147, a3148)
            } else {
                false
            } {
                Ok(Some({
                    let a3149 = v4;
                    let a3150 = 128u64;
                    nat_sub(a3149, a3150)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f478(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a3151 = v1;
    let a3152 = v0;
    if if nat_le(a3151, a3152) {
        let a3153 = v0;
        let a3154 = v2;
        if nat_le(a3153, a3154) {
            let a3155 = v0;
            let a3156 = 55296u64;
            if nat_lt(a3155, a3156) {
                true
            } else {
                let a3157 = 57343u64;
                let a3158 = v0;
                nat_lt(a3157, a3158)
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

pub fn f479(p3175: u64, p3176: u64, p3177: u64, p3178: Bytes) -> Bytes {
    let m3174 = (p3175, (p3176, (p3177, p3178)));
    {
        let mut m3174 = m3174;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3174;
                let m3159 = v0;
                if m3159 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3159 - 1;
                    let a3160 = v1;
                    let a3161 = 0u64;
                    if if nat_eq(a3160, a3161) {
                        let a3162 = v2;
                        let a3163 = 0u64;
                        nat_eq(a3162, a3163)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3164 = v1;
                            let a3165 = 16u64;
                            let a3166 = 0u64;
                            nat_quot(a3164, a3165, a3166)
                        }, ({
                            let a3167 = v2;
                            let a3168 = 1u64;
                            nat_sub(a3167, a3168)
                        }, {
                            let a3172 = f487({
                                let a3169 = v1;
                                let a3170 = 16u64;
                                let a3171 = 0u64;
                                nat_rem(a3169, a3170, a3171)
                            });
                            let a3173 = v3;
                            append_bytes(a3172, a3173)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil())))))))))))))))
}

pub fn f481() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil()))))))))))))))))))))
}

pub fn f482(v0: Adt14, v1: Bytes) -> bool {
    let m3179 = f142(v0, v1);
    match m3179 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f483(v0: u64, v1: Adt14, v2: Bytes) -> R<Result<(Bytes, List<Adt23>), Bytes>> {
    let m3180 = v0;
    if m3180 == 0 {
        Ok(f393(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3180 - 1;
        let m3181 = v1.clone();
        match m3181 {
            Adt14::C0(v4) => {
                if v4 {
                    Ok(f433())
                } else {
                    Ok(f393(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt14::C1(_, _) => {
                Ok(f393(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C2(_) => {
                Ok(f393(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C3(_) => {
                Ok(f393(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C4(_) => {
                let m3182 = f150(f142(v1.clone(), f8(Str::lit("$ref"))));
                match m3182 {
                    None => {
                        if if f482(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f482(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f482(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f394(v2.clone(), List::cons(Adt23::C0(v2, v1), List::<Adt23>::nil())))
                        } else {
                            let v17: Bytes = f145(v1.clone(), f8(Str::lit("type")));
                            if if f36(v17.clone(), Bytes::lit(&[])) {
                                !f466(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f433())
                            } else {
                                if f36(v17.clone(), f8(Str::lit("string"))) {
                                    Ok(f394(f8(Str::lit("Bytes")), List::<Adt23>::nil()))
                                } else {
                                    if f36(v17.clone(), f8(Str::lit("integer"))) {
                                        Ok(f394(f467(v1), List::<Adt23>::nil()))
                                    } else {
                                        if f36(v17.clone(), f8(Str::lit("number"))) {
                                            Ok(f394(f468(), List::<Adt23>::nil()))
                                        } else {
                                            if f36(v17.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f394(f8(Str::lit("Bool")), List::<Adt23>::nil()))
                                            } else {
                                                if f36(v17.clone(), f8(Str::lit("array"))) {
                                                    let m3184 = f142(v1, f8(Str::lit("items")));
                                                    match m3184 {
                                                        None => {
                                                            Ok(f431(f8(Str::lit("List")), f433()))
                                                        }
                                                        Some(v18) => {
                                                            Ok(f431(f8(Str::lit("List")), f483(v3, v18, {
                                                                let a3185 = v2;
                                                                let a3186 = f8(Str::lit("Item"));
                                                                append_bytes(a3185, a3186)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f36(v17.clone(), f8(Str::lit("object"))) {
                                                        let m3187 = f142(v1, f8(Str::lit("additionalProperties")));
                                                        match m3187 {
                                                            None => {
                                                                Ok(f431(f8(Str::lit("List")), f469(f433())))
                                                            }
                                                            Some(v19) => {
                                                                Ok(f431(f8(Str::lit("List")), f469(f483(v3, v19, {
                                                                    let a3188 = v2;
                                                                    let a3189 = f8(Str::lit("Value"));
                                                                    append_bytes(a3188, a3189)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f393(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v17, List::<Bytes>::nil()))), v2))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(v15) => {
                        let m3183 = f450(v15)?;
                        match m3183 {
                            None => {
                                Ok(f393(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v16) => {
                                Ok(f394(v16, List::<Adt23>::nil()))
                            }
                        }
                    }
                }
            }
            Adt14::C5 => {
                Ok(f433())
            }
            Adt14::C6(_, _) => {
                Ok(f393(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt14::C7(_, _, _) => {
                Ok(f393(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f484(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3190, _) = v0;
        h3190
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3191) = v0;
        h3191
    };
    let m3192 = v1;
    match m3192.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3193 = v4;
            match m3193.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f488(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f485(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3194, _) = v0.clone();
        h3194
    };
    let v2: List<(Bytes, ())> = {
        let (h3195, _) = {
            let (_, h3196) = v0.clone();
            h3196
        };
        h3195
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3197) = {
            let (_, h3198) = v0;
            h3198
        };
        h3197
    };
    let m3199 = v1.clone();
    match m3199.uncons() {
        None => {
            let m3200 = v2;
            match m3200.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3201 = v2.clone();
            match m3201.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f489({
                        let (h3202, _) = v8.clone();
                        h3202
                    }, {
                        let (h3203, _) = v6.clone();
                        h3203
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

pub fn f486(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h3204, _) = v1;
        h3204
    };
    let m3206 = {
        let (_, h3205) = v1;
        h3205
    };
    match m3206 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m3209 = {
                let a3207 = v0.clone();
                let a3208 = v2;
                index_bytes(a3207, a3208)
            };
            match m3209 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m3214 = {
                        let a3212 = v0;
                        let a3213 = {
                            let a3210 = v2;
                            let a3211 = 1u64;
                            nat_add(a3210, a3211)?
                        };
                        index_bytes(a3212, a3213)
                    };
                    match m3214 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f490(v4)? {
                                f490(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a3215 = v2;
                                    let a3216 = 2u64;
                                    nat_add(a3215, a3216)?
                                }, Some({
                                    let a3221 = v3;
                                    let a3222 = f400(f491(), {
                                        let a3219 = {
                                            let a3217 = f492(v4)?;
                                            let a3218 = 16u64;
                                            nat_mul(a3217, a3218)?
                                        };
                                        let a3220 = f492(v5)?;
                                        nat_add(a3219, a3220)?
                                    });
                                    append_bytes(a3221, a3222)
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

pub fn f487(v0: u64) -> Bytes {
    f32({
        let a3223 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3223)
    }, v0, 1u64)
}

pub fn f488(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3228 = {
            let a3226 = {
                let a3224 = v0.clone();
                length_list(a3224)
            };
            let a3227 = {
                let a3225 = v1.clone();
                length_list(a3225)
            };
            nat_add(a3226, a3227)?
        };
        let a3229 = 1u64;
        nat_add(a3228, a3229)?
    };
    Ok(f493({
        let (_, h3230) = {
            let (_, h3231) = {
                let (h3232, _) = f494(Fn25::F495, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3232
            };
            h3231
        };
        h3230
    }))
}

pub fn f489(v0: Bytes, v1: Bytes) -> bool {
    let a3237 = {
        let a3233 = v0;
        let a3234 = v1;
        compare_bytes(a3233, a3234)
    };
    let a3238 = {
        let a3235 = Bytes::lit(&[]);
        let a3236 = Bytes::lit(&[0x01]);
        compare_bytes(a3235, a3236)
    };
    equal(a3237, a3238)
}

pub fn f490(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3283 = {
            let a3259 = {
                let a3247 = {
                    let a3241 = {
                        let a3239 = v1;
                        let a3240 = 128u8;
                        fixed_u8::bit_and(a3239, a3240)
                    };
                    let a3242 = 128u8;
                    if equal(a3241, a3242) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3248 = {
                    let a3245 = {
                        let a3243 = v1;
                        let a3244 = 64u8;
                        fixed_u8::bit_and(a3243, a3244)
                    };
                    let a3246 = 64u8;
                    if equal(a3245, a3246) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3247, a3248)?
            };
            let a3260 = {
                let a3257 = {
                    let a3251 = {
                        let a3249 = v1;
                        let a3250 = 32u8;
                        fixed_u8::bit_and(a3249, a3250)
                    };
                    let a3252 = 32u8;
                    if equal(a3251, a3252) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3258 = {
                    let a3255 = {
                        let a3253 = v1;
                        let a3254 = 16u8;
                        fixed_u8::bit_and(a3253, a3254)
                    };
                    let a3256 = 16u8;
                    if equal(a3255, a3256) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3257, a3258)?
            };
            nat_add(a3259, a3260)?
        };
        let a3284 = {
            let a3281 = {
                let a3269 = {
                    let a3263 = {
                        let a3261 = v1;
                        let a3262 = 8u8;
                        fixed_u8::bit_and(a3261, a3262)
                    };
                    let a3264 = 8u8;
                    if equal(a3263, a3264) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3270 = {
                    let a3267 = {
                        let a3265 = v1;
                        let a3266 = 4u8;
                        fixed_u8::bit_and(a3265, a3266)
                    };
                    let a3268 = 4u8;
                    if equal(a3267, a3268) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3269, a3270)?
            };
            let a3282 = {
                let a3279 = {
                    let a3273 = {
                        let a3271 = v1;
                        let a3272 = 2u8;
                        fixed_u8::bit_and(a3271, a3272)
                    };
                    let a3274 = 2u8;
                    if equal(a3273, a3274) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3280 = {
                    let a3277 = {
                        let a3275 = v1;
                        let a3276 = 1u8;
                        fixed_u8::bit_and(a3275, a3276)
                    };
                    let a3278 = 1u8;
                    if equal(a3277, a3278) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3279, a3280)?
            };
            nat_add(a3281, a3282)?
        };
        nat_add(a3283, a3284)?
    };
    let a3285 = 48u64;
    let a3286 = v2;
    if if nat_le(a3285, a3286) {
        let a3287 = v2;
        let a3288 = 57u64;
        nat_le(a3287, a3288)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3289 = 97u64;
        let a3290 = v2;
        if nat_le(a3289, a3290) {
            let a3291 = v2;
            let a3292 = 102u64;
            Ok(nat_le(a3291, a3292))
        } else {
            Ok(false)
        }
    }
}

pub fn f491() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f492(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3337 = {
            let a3313 = {
                let a3301 = {
                    let a3295 = {
                        let a3293 = v1;
                        let a3294 = 128u8;
                        fixed_u8::bit_and(a3293, a3294)
                    };
                    let a3296 = 128u8;
                    if equal(a3295, a3296) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3302 = {
                    let a3299 = {
                        let a3297 = v1;
                        let a3298 = 64u8;
                        fixed_u8::bit_and(a3297, a3298)
                    };
                    let a3300 = 64u8;
                    if equal(a3299, a3300) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3301, a3302)?
            };
            let a3314 = {
                let a3311 = {
                    let a3305 = {
                        let a3303 = v1;
                        let a3304 = 32u8;
                        fixed_u8::bit_and(a3303, a3304)
                    };
                    let a3306 = 32u8;
                    if equal(a3305, a3306) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3312 = {
                    let a3309 = {
                        let a3307 = v1;
                        let a3308 = 16u8;
                        fixed_u8::bit_and(a3307, a3308)
                    };
                    let a3310 = 16u8;
                    if equal(a3309, a3310) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3311, a3312)?
            };
            nat_add(a3313, a3314)?
        };
        let a3338 = {
            let a3335 = {
                let a3323 = {
                    let a3317 = {
                        let a3315 = v1;
                        let a3316 = 8u8;
                        fixed_u8::bit_and(a3315, a3316)
                    };
                    let a3318 = 8u8;
                    if equal(a3317, a3318) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3324 = {
                    let a3321 = {
                        let a3319 = v1;
                        let a3320 = 4u8;
                        fixed_u8::bit_and(a3319, a3320)
                    };
                    let a3322 = 4u8;
                    if equal(a3321, a3322) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3323, a3324)?
            };
            let a3336 = {
                let a3333 = {
                    let a3327 = {
                        let a3325 = v1;
                        let a3326 = 2u8;
                        fixed_u8::bit_and(a3325, a3326)
                    };
                    let a3328 = 2u8;
                    if equal(a3327, a3328) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3334 = {
                    let a3331 = {
                        let a3329 = v1;
                        let a3330 = 1u8;
                        fixed_u8::bit_and(a3329, a3330)
                    };
                    let a3332 = 1u8;
                    if equal(a3331, a3332) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3333, a3334)?
            };
            nat_add(a3335, a3336)?
        };
        nat_add(a3337, a3338)?
    };
    let a3339 = v2;
    let a3340 = 57u64;
    if nat_le(a3339, a3340) {
        let a3341 = v2;
        let a3342 = 48u64;
        Ok(nat_sub(a3341, a3342))
    } else {
        let a3343 = v2;
        let a3344 = 87u64;
        Ok(nat_sub(a3343, a3344))
    }
}

pub fn f493(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f340(Fn10::F496, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f494(v0: Fn25, p3349: u64, p3350: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3348 = (p3349, p3350);
    {
        let mut m3348 = m3348;
        loop {
            let step = {
                let (v1, v2) = m3348;
                let m3345 = v1;
                if m3345 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3345 - 1;
                    let m3347 = {
                        let c3346: Fn25 = v0.clone();
                        c3346.apply(v2.clone())
                    };
                    match m3347 {
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
                Ok(next) => { m3348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f497(v0)
}

pub fn f496(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f497(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3351, _) = v0.clone();
        h3351
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3352, _) = {
            let (_, h3353) = v0.clone();
            h3353
        };
        h3352
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3354) = {
            let (_, h3355) = v0;
            h3355
        };
        h3354
    };
    let m3356 = v1.clone();
    match m3356.uncons() {
        None => {
            let m3357 = v2;
            match m3357.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3358 = v2.clone();
            match m3358.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f489({
                        let (h3359, _) = v8.clone();
                        h3359
                    }, {
                        let (h3360, _) = v6.clone();
                        h3360
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
