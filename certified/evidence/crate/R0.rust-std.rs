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
    C5(Bytes),
    C6(std::rc::Rc<Adt2>),
    C7(std::rc::Rc<Adt2>),
    C8,
    C9(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C10(Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
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
    C0(List<Bytes>, List<Bytes>, List<Bytes>, List<Bytes>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt16, List<Adt14>, List<Adt14>, List<Bytes>, List<Bytes>),
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
    C0(Bytes, Adt2),
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
    F77(u8, Bytes),
    F187(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F77(k0, k1) => f77(*k0, k1.clone(), p0),
            Fn0::F187(k0) => f187(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F122,
    F238,
    F348,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F122 => f122(p0, p1),
            Fn1::F238 => f238(p0, p1),
            Fn1::F348 => f348(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F210(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F210(k0) => f210(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F317(Bytes),
    F318(Bytes),
    F416(Bytes),
    F417(u64, Bytes),
    F424(Bytes),
    F432(Bytes, u64, Bytes, u64),
    F496(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F317(k0) => f317(k0.clone(), p0),
            Fn3::F318(k0) => f318(k0.clone(), p0),
            Fn3::F416(k0) => Ok(f416(k0.clone(), p0)),
            Fn3::F417(k0, k1) => f417(*k0, k1.clone(), p0),
            Fn3::F424(k0) => f424(k0.clone(), p0),
            Fn3::F432(k0, k1, k2, k3) => f432(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F496(k0) => f496(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F321(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F321(k0) => f321(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F406(Bytes),
    F543(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F406(k0) => f406(k0.clone(), p0),
            Fn5::F543(k0, k1) => Ok(f543(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F408,
    F528,
    F739,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F408 => f408(p0, p1),
            Fn6::F528 => f528(p0, p1),
            Fn6::F739 => f739(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F411,
    F529,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F411 => f411(p0, p1),
            Fn7::F529 => f529(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F443,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn8::F443 => f443(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F446,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn9::F446 => f446(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F448,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn10::F448 => f448(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F451,
}

impl Fn11 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn11::F451 => f451(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F475(Bytes, u64, u64),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn12::F475(k0, k1, k2) => f475(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F507,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn13::F507 => f507(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F510,
}

impl Fn14 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn14::F510 => f510(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F547,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F547 => f547(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F549,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F549 => f549(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F552(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F552(k0) => f552(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F569,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F569 => f569(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F588,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F588 => f588(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F592,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F592 => f592(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F655,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F655 => f655(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F666,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F666 => f666(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F697,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F697 => f697(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F725,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F725 => f725(p0),
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
                                        if f85(v2.clone(), f8(Str::lit("read-"))) {
                                            Ok(Adt12::C4(f8(Str::lit("read-webidl, read-xml, read-xsd, read-xsd-components and read-yaml run in verdant_readers, the command of the readers project\u{a}"))))
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

pub fn f57(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    let m276 = f91(v0, v1)?;
    match m276 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            Ok(f92(v2))
        }
    }
}

pub fn f58(v0: Adt14, v1: Bytes) -> Bytes {
    let m277 = f93(f94(v0, v1));
    match m277 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f59(v0: List<Adt13>, v1: Bytes) -> List<Bytes> {
    let m278 = v0;
    match m278.uncons() {
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
            let a280 = {
                let a279 = v0;
                length_bytes(a279)
            };
            let a281 = 1u64;
            nat_sub(a280, a281)
        })
    } else {
        v0
    }
}

pub fn f62(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m282 = v0;
    match m282.uncons() {
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
    f95(f96(v0)?, f8(Str::lit("id = \"")))
}

pub fn f64(v0: Adt1, v1: Bytes) -> Bytes {
    let m283 = f89(f90(v0, v1.clone()), v1);
    match m283 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m284 = v2;
            match m284 {
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
    let v2: List<Adt14> = f97(v0, f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f98(v2.clone(), f61(v1.clone()));
    if f99(f100(v2), f100(v3.clone())) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" is pinned, and every admission of its registers cites it\u{a}")), List::<Bytes>::nil()))))))
    } else {
        Ok(Adt12::C3(List::cons((f8(Str::lit("model/admissions.json")), f101(Adt14::C3(std::rc::Rc::new(f102(v3))))?), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("re-cited the admissions of the verdant-engine registers at ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
    }
}

pub fn f66(v0: List<Bytes>) -> List<Bytes> {
    let m285 = v0;
    match m285.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f68(v1), f66(v2))
        }
    }
}

pub fn f67(v0: Adt1, v1: Bytes) -> bool {
    let m286 = f89(f90(v0, v1.clone()), v1);
    match m286 {
        None => {
            false
        }
        Some(v2) => {
            let m287 = v2;
            match m287 {
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
    f7(List::cons(f103(v0), List::cons(v1, List::<Bytes>::nil())))
}

pub fn f70(v0: Adt1, v1: Bytes) -> R<Adt12> {
    if f51(f52(f104(), match v0.clone() {
        Adt1::C0(_, _, h288, _) => {
            h288
        }
    })) {
        Ok(f54(f104(), f55(f7(List::cons(f8(Str::lit("git/ref/tags/")), List::cons(v1, List::<Bytes>::nil()))))))
    } else {
        let m289 = f56(v0.clone(), List::cons(f104(), List::<Bytes>::nil()));
        match m289.uncons() {
            None => {
                let v6: Adt14 = {
                    let m291 = f94({
                        let m290 = f57(v0.clone(), f104())?;
                        match m290 {
                            None => {
                                Adt14::C5
                            }
                            Some(v4) => {
                                v4
                            }
                        }
                    }, f8(Str::lit("object")));
                    match m291 {
                        None => {
                            Adt14::C5
                        }
                        Some(v5) => {
                            v5
                        }
                    }
                };
                if f32(f58(v6.clone(), f8(Str::lit("type"))), f8(Str::lit("tag"))) {
                    if f51(f52(f105(), match v0.clone() {
                        Adt1::C0(_, _, h292, _) => {
                            h292
                        }
                    })) {
                        Ok(f54(f105(), f55(f7(List::cons(f8(Str::lit("git/tags/")), List::cons(f58(v6, f8(Str::lit("sha"))), List::<Bytes>::nil()))))))
                    } else {
                        let m293 = f56(v0.clone(), List::cons(f105(), List::<Bytes>::nil()));
                        match m293.uncons() {
                            None => {
                                f106(v0.clone(), v1, {
                                    let m295 = f94({
                                        let m294 = f57(v0, f105())?;
                                        match m294 {
                                            None => {
                                                Adt14::C5
                                            }
                                            Some(v9) => {
                                                v9
                                            }
                                        }
                                    }, f8(Str::lit("object")));
                                    match m295 {
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
                    f106(v0, v1, f58(v6, f8(Str::lit("sha"))))
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
    let a340 = {
        let a316 = {
            let a304 = {
                let a298 = {
                    let a296 = v1;
                    let a297 = 128u8;
                    fixed_u8::bit_and(a296, a297)
                };
                let a299 = 128u8;
                if equal(a298, a299) {
                    128u64
                } else {
                    0u64
                }
            };
            let a305 = {
                let a302 = {
                    let a300 = v1;
                    let a301 = 64u8;
                    fixed_u8::bit_and(a300, a301)
                };
                let a303 = 64u8;
                if equal(a302, a303) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a304, a305)?
        };
        let a317 = {
            let a314 = {
                let a308 = {
                    let a306 = v1;
                    let a307 = 32u8;
                    fixed_u8::bit_and(a306, a307)
                };
                let a309 = 32u8;
                if equal(a308, a309) {
                    32u64
                } else {
                    0u64
                }
            };
            let a315 = {
                let a312 = {
                    let a310 = v1;
                    let a311 = 16u8;
                    fixed_u8::bit_and(a310, a311)
                };
                let a313 = 16u8;
                if equal(a312, a313) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a314, a315)?
        };
        nat_add(a316, a317)?
    };
    let a341 = {
        let a338 = {
            let a326 = {
                let a320 = {
                    let a318 = v1;
                    let a319 = 8u8;
                    fixed_u8::bit_and(a318, a319)
                };
                let a321 = 8u8;
                if equal(a320, a321) {
                    8u64
                } else {
                    0u64
                }
            };
            let a327 = {
                let a324 = {
                    let a322 = v1;
                    let a323 = 4u8;
                    fixed_u8::bit_and(a322, a323)
                };
                let a325 = 4u8;
                if equal(a324, a325) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a326, a327)?
        };
        let a339 = {
            let a336 = {
                let a330 = {
                    let a328 = v1;
                    let a329 = 2u8;
                    fixed_u8::bit_and(a328, a329)
                };
                let a331 = 2u8;
                if equal(a330, a331) {
                    2u64
                } else {
                    0u64
                }
            };
            let a337 = {
                let a334 = {
                    let a332 = v1;
                    let a333 = 1u8;
                    fixed_u8::bit_and(a332, a333)
                };
                let a335 = 1u8;
                if equal(a334, a335) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a336, a337)?
        };
        nat_add(a338, a339)?
    };
    nat_add(a340, a341)
}

pub fn f73(v0: u64) -> List<Str> {
    let a342 = v0;
    let a343 = 0u64;
    if nat_eq(a342, a343) {
        f107()
    } else {
        let a344 = v0;
        let a345 = 1u64;
        if nat_eq(a344, a345) {
            f108()
        } else {
            let a346 = v0;
            let a347 = 2u64;
            if nat_eq(a346, a347) {
                f109()
            } else {
                let a348 = v0;
                let a349 = 3u64;
                if nat_eq(a348, a349) {
                    f110()
                } else {
                    let a350 = v0;
                    let a351 = 4u64;
                    if nat_eq(a350, a351) {
                        f111()
                    } else {
                        let a352 = v0;
                        let a353 = 5u64;
                        if nat_eq(a352, a353) {
                            f112()
                        } else {
                            let a354 = v0;
                            let a355 = 6u64;
                            if nat_eq(a354, a355) {
                                f113()
                            } else {
                                let a356 = v0;
                                let a357 = 7u64;
                                if nat_eq(a356, a357) {
                                    f114()
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
    f115(v0.clone(), 0u64, {
        let a358 = v0;
        length_bytes(a358)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f75(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m359 = v2;
    if m359 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m359 - 1;
        let v5: u64 = f116(v0.clone(), v1)?;
        let a360 = v3;
        let a361 = 1u64;
        if nat_eq(a360, a361) {
            let m362 = f75(v0.clone(), v1, v4, 0u64)?;
            match m362 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f116(v0.clone(), {
                        let (_, h363) = v6.clone();
                        h363
                    })?;
                    let a364 = v7;
                    let a365 = 44u64;
                    if nat_eq(a364, a365) {
                        let m369 = f75(v0, {
                            let a367 = {
                                let (_, h366) = v6;
                                h366
                            };
                            let a368 = 1u64;
                            nat_add(a367, a368)?
                        }, v4, 1u64)?;
                        match m369 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C9(std::rc::Rc::new({
                                    let (h370, _) = v6;
                                    h370
                                }), std::rc::Rc::new({
                                    let (h371, _) = v8;
                                    h371
                                })), {
                                    let (_, h372) = v8;
                                    h372
                                })))
                            }
                        }
                    } else {
                        let a373 = v7;
                        let a374 = 93u64;
                        if nat_eq(a373, a374) {
                            Ok(Some((Adt2::C9(std::rc::Rc::new({
                                let (h375, _) = v6;
                                h375
                            }), std::rc::Rc::new(Adt2::C8)), {
                                let a377 = {
                                    let (_, h376) = v6;
                                    h376
                                };
                                let a378 = 1u64;
                                nat_add(a377, a378)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a379 = v3;
            let a380 = 2u64;
            if nat_eq(a379, a380) {
                let m383 = f117(v0.clone(), {
                    let a381 = v1;
                    let a382 = 1u64;
                    nat_add(a381, a382)?
                })?;
                match m383 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f118({
                            let a384 = v5;
                            let a385 = 34u64;
                            nat_eq(a384, a385)
                        }, {
                            let a387 = f116(v0.clone(), {
                                let (h386, _) = v9.clone();
                                h386
                            })?;
                            let a388 = 58u64;
                            nat_eq(a387, a388)
                        }) {
                            let m392 = f75(v0.clone(), {
                                let a390 = {
                                    let (h389, _) = v9;
                                    h389
                                };
                                let a391 = 1u64;
                                nat_add(a390, a391)?
                            }, v4, 0u64)?;
                            match m392 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f116(v0.clone(), {
                                        let (_, h393) = v10.clone();
                                        h393
                                    })?;
                                    let a394 = v11;
                                    let a395 = 44u64;
                                    if nat_eq(a394, a395) {
                                        let m399 = f75(v0, {
                                            let a397 = {
                                                let (_, h396) = v10;
                                                h396
                                            };
                                            let a398 = 1u64;
                                            nat_add(a397, a398)?
                                        }, v4, 2u64)?;
                                        match m399 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C10({
                                                    let (_, h400) = v9;
                                                    h400
                                                }, std::rc::Rc::new({
                                                    let (h401, _) = v10;
                                                    h401
                                                }), std::rc::Rc::new({
                                                    let (h402, _) = v12;
                                                    h402
                                                })), {
                                                    let (_, h403) = v12;
                                                    h403
                                                })))
                                            }
                                        }
                                    } else {
                                        let a404 = v11;
                                        let a405 = 125u64;
                                        if nat_eq(a404, a405) {
                                            Ok(Some((Adt2::C10({
                                                let (_, h406) = v9;
                                                h406
                                            }, std::rc::Rc::new({
                                                let (h407, _) = v10;
                                                h407
                                            }), std::rc::Rc::new(Adt2::C8)), {
                                                let a409 = {
                                                    let (_, h408) = v10;
                                                    h408
                                                };
                                                let a410 = 1u64;
                                                nat_add(a409, a410)?
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
                let a411 = v5;
                let a412 = 34u64;
                if nat_eq(a411, a412) {
                    let m415 = f117(v0, {
                        let a413 = v1;
                        let a414 = 1u64;
                        nat_add(a413, a414)?
                    })?;
                    match m415 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C5({
                                let (_, h416) = v13;
                                h416
                            }), {
                                let (h417, _) = v13;
                                h417
                            })))
                        }
                    }
                } else {
                    if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a418 = v1;
                            let a419 = 4u64;
                            nat_add(a418, a419)?
                        })))
                    } else {
                        if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a420 = v1;
                                let a421 = 4u64;
                                nat_add(a420, a421)?
                            })))
                        } else {
                            if f32(f46(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a422 = v1;
                                    let a423 = 5u64;
                                    nat_add(a422, a423)?
                                })))
                            } else {
                                let a424 = v5;
                                let a425 = 91u64;
                                if nat_eq(a424, a425) {
                                    let a428 = f116(v0.clone(), {
                                        let a426 = v1;
                                        let a427 = 1u64;
                                        nat_add(a426, a427)?
                                    })?;
                                    let a429 = 93u64;
                                    if nat_eq(a428, a429) {
                                        Ok(Some((Adt2::C6(std::rc::Rc::new(Adt2::C8)), {
                                            let a430 = v1;
                                            let a431 = 2u64;
                                            nat_add(a430, a431)?
                                        })))
                                    } else {
                                        let m434 = f75(v0, {
                                            let a432 = v1;
                                            let a433 = 1u64;
                                            nat_add(a432, a433)?
                                        }, v4, 1u64)?;
                                        match m434 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C6(std::rc::Rc::new({
                                                    let (h435, _) = v14;
                                                    h435
                                                })), {
                                                    let (_, h436) = v14;
                                                    h436
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a437 = v5;
                                    let a438 = 123u64;
                                    if nat_eq(a437, a438) {
                                        let a441 = f116(v0.clone(), {
                                            let a439 = v1;
                                            let a440 = 1u64;
                                            nat_add(a439, a440)?
                                        })?;
                                        let a442 = 125u64;
                                        if nat_eq(a441, a442) {
                                            Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C8)), {
                                                let a443 = v1;
                                                let a444 = 2u64;
                                                nat_add(a443, a444)?
                                            })))
                                        } else {
                                            let m447 = f75(v0, {
                                                let a445 = v1;
                                                let a446 = 1u64;
                                                nat_add(a445, a446)?
                                            }, v4, 2u64)?;
                                            match m447 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                        let (h448, _) = v15;
                                                        h448
                                                    })), {
                                                        let (_, h449) = v15;
                                                        h449
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m450 = f119(v0, v1)?;
                                        match m450 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h451) = v16;
                                                    h451
                                                }, {
                                                    let (h452, _) = v16;
                                                    h452
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

pub fn f76(v0: Fn0, p457: u64, p458: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m456 = (p457, p458);
    {
        let mut m456 = m456;
        loop {
            let step = {
                let (v1, v2) = m456;
                let m453 = v1;
                if m453 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m453 - 1;
                    let m455 = {
                        let c454: Fn0 = v0.clone();
                        c454.apply(v2.clone())?
                    };
                    match m455 {
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
                Ok(next) => { m456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f120(v1, v0, v2)
}

pub fn f78(v0: List<Bytes>) -> List<Bytes> {
    f121(Fn1::F122, List::<Bytes>::nil(), v0)
}

pub fn f79() -> Bytes {
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f123(f124(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f80(v0: Adt1) -> R<Adt12> {
    let m459 = f125(v0.clone());
    match m459.uncons() {
        None => {
            let m460 = f126(f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f127(v0.clone())?);
            match m460.uncons() {
                None => {
                    let v5: Adt15 = f128(v0.clone())?;
                    let v6: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(f126(f129(f130(match v5.clone() {
                        Adt15::C0(h461, _, _) => {
                            h461
                        }
                    }, f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f132(v6.clone())), f133(f130(match v5.clone() {
                        Adt15::C0(h462, _, _) => {
                            h462
                        }
                    }, f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f134(v5.clone(), v6, f135(v5, f136(v0.clone(), f137(v0)?)?))?)))))
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
        Adt1::C0(_, _, h463, _) => {
            h463
        }
    })) {
        Adt12::C2(f8(Str::lit("npx")), f138(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f51(f52(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h464, _) => {
                h464
            }
        })) {
            Adt12::C2(f8(Str::lit("npx")), f138(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f82(v0: Adt1) -> R<Adt12> {
    if f51(f139(match v0.clone() {
        Adt1::C0(_, h465, _, _) => {
            h465
        }
    }, f140())) {
        Ok(Adt12::C1(List::cons(f140(), List::<Bytes>::nil())))
    } else {
        let m466 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f141(v0.clone())?)));
        match m466.uncons() {
            None => {
                let m467 = f142(v0.clone(), f141(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m467.uncons() {
                    None => {
                        f144(v0)
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
    if f51(f139(match v0.clone() {
        Adt1::C0(_, h468, _, _) => {
            h468
        }
    }, f145())) {
        Ok(Adt12::C1(List::cons(f145(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f146(f147(f145(), f148(match v0.clone() {
            Adt1::C0(_, h469, _, _) => {
                h469
            }
        }, f145())))?;
        let m470 = f56(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m470.uncons() {
            None => {
                let m471 = f149(v0.clone(), f150(f151(f96(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?, f152(v2, v1.clone()))?;
                match m471 {
                    None => {
                        f153(v0, v1)
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
    let m472 = f125(v0.clone());
    match m472.uncons() {
        None => {
            let m473 = f126(f56(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f154(), List::<Bytes>::nil())))), f127(v0.clone())?);
            match m473.uncons() {
                None => {
                    Ok(Adt12::C3(f155(f156(v0.clone())?, List::cons((f157(), f158()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f159()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f160()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f161()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f162()), List::cons((f8(Str::lit("docs/catalog.md")), f163(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f164(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f165(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f166()), List::<(Bytes, Bytes)>::nil())))))))))), f8(Str::lit("published the workflows, the export script, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m478 = {
        let a475 = v0;
        let a476 = 0u64;
        let a477 = {
            let a474 = v1.clone();
            length_bytes(a474)
        };
        slice_bytes(a475, a476, a477)
    };
    match m478 {
        None => {
            false
        }
        Some(v2) => {
            let a479 = v2;
            let a480 = v1;
            equal(a479, a480)
        }
    }
}

pub fn f86(v0: Bytes, v1: Bytes, _: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("stage-admissions"))) {
        f167(v3.clone(), v1.clone(), f168(v3, v1, false)?)
    } else {
        if f32(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f167(v3.clone(), v1.clone(), f169(v3, v1)?)
        } else {
            if f32(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f167(v3.clone(), v1.clone(), f168(v3, v1, true)?)
            } else {
                if f32(v0.clone(), f8(Str::lit("stage-registries"))) {
                    f167(v3.clone(), v1.clone(), f170(v3, v1)?)
                } else {
                    if f32(v0.clone(), f8(Str::lit("stage-encodings"))) {
                        f167(v3.clone(), v1.clone(), f171(v3, v1)?)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("stage-mappings"))) {
                            f167(v3.clone(), v1.clone(), f172(v3, v1, false)?)
                        } else {
                            if f32(v0, f8(Str::lit("stage-matrix"))) {
                                f167(v3.clone(), v1.clone(), f172(v3, v1, true)?)
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
    if f32(v0.clone(), f8(Str::lit("admit"))) {
        f173(v3, v1)
    } else {
        if f32(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f174(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("admit-registry"))) {
                f175(v3, v1, v2)
            } else {
                if f32(v0.clone(), f8(Str::lit("ingest-authority"))) {
                    f176(v3, v1)
                } else {
                    if f32(v0.clone(), f8(Str::lit("ingest-index"))) {
                        f177(v3, v1)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("merge-gate"))) {
                            f178(v3, v1)
                        } else {
                            if f32(v0.clone(), f8(Str::lit("propose-encoding"))) {
                                let m481 = v2;
                                match m481.uncons() {
                                    None => {
                                        Ok(Adt12::C4(f79()))
                                    }
                                    Some((v4, v5)) => {
                                        f179(v3, v1, v4, v5)
                                    }
                                }
                            } else {
                                if f32(v0.clone(), f8(Str::lit("admit-translated"))) {
                                    let m482 = v2;
                                    match m482.uncons() {
                                        None => {
                                            Ok(Adt12::C4(f79()))
                                        }
                                        Some((v6, v7)) => {
                                            f180(v3, v1, v6, v7)
                                        }
                                    }
                                } else {
                                    if f32(v0.clone(), f8(Str::lit("record-decision"))) {
                                        f181(v3, v1)
                                    } else {
                                        if f32(v0, f8(Str::lit("record-error"))) {
                                            f182(v3, v1, v2)
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
        }
    }
}

pub fn f88(v0: Bytes) -> R<bool> {
    f183(v0.clone(), 0u64, {
        let a483 = v0;
        length_bytes(a483)
    })
}

pub fn f89(p488: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m487 = p488;
    {
        let mut m487 = m487;
        loop {
            let step = {
                let v0 = m487;
                let m484 = v0;
                match m484.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h485, _) = v2;
                            h485
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h486) = v2;
                                h486
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m492 = {
        let a490 = match v0 {
            Adt1::C0(h489, _, _, _) => {
                h489
            }
        };
        let a491 = f40(v1);
        index_list(a490, a491)
    };
    match m492 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m493 = f184(f185(v0.clone(), v1.clone()), v1.clone());
    match m493 {
        None => {
            f43(f64(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f92(v0: Adt2) -> Option<Adt14> {
    let m494 = v0;
    match m494 {
        Adt2::C0 => {
            Some(Adt14::C3(std::rc::Rc::new(Adt14::C5)))
        }
        Adt2::C1(_, _) => {
            None::<Adt14>
        }
        Adt2::C2(v3) => {
            Some(Adt14::C0(v3))
        }
        Adt2::C3(v4, v5) => {
            Some(Adt14::C1(v4, v5))
        }
        Adt2::C4(_, _, _) => {
            None::<Adt14>
        }
        Adt2::C5(v9) => {
            Some(Adt14::C2(v9))
        }
        Adt2::C6(r501) => {
            let v10 = (*r501).clone();
            let m495 = f92(v10);
            match m495 {
                None => {
                    None::<Adt14>
                }
                Some(v11) => {
                    Some(Adt14::C3(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt2::C7(r502) => {
            let v12 = (*r502).clone();
            let m496 = f92(v12);
            match m496 {
                None => {
                    None::<Adt14>
                }
                Some(v13) => {
                    Some(Adt14::C4(std::rc::Rc::new(v13)))
                }
            }
        }
        Adt2::C8 => {
            Some(Adt14::C5)
        }
        Adt2::C9(r503, r504) => {
            let v14 = (*r503).clone();
            let v15 = (*r504).clone();
            let m497 = f92(v14);
            match m497 {
                None => {
                    None::<Adt14>
                }
                Some(v16) => {
                    let m498 = f92(v15);
                    match m498 {
                        None => {
                            None::<Adt14>
                        }
                        Some(v17) => {
                            Some(Adt14::C6(std::rc::Rc::new(v16), std::rc::Rc::new(v17)))
                        }
                    }
                }
            }
        }
        Adt2::C10(v18, r505, r506) => {
            let v19 = (*r505).clone();
            let v20 = (*r506).clone();
            let m499 = f92(v19);
            match m499 {
                None => {
                    None::<Adt14>
                }
                Some(v21) => {
                    let m500 = f92(v20);
                    match m500 {
                        None => {
                            None::<Adt14>
                        }
                        Some(v22) => {
                            Some(Adt14::C7(v18, std::rc::Rc::new(v21), std::rc::Rc::new(v22)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f93(v0: Option<Adt14>) -> Option<Bytes> {
    let m507 = v0;
    match m507 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m508 = v1;
            match m508 {
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

pub fn f94(p514: Adt14, v1: Bytes) -> Option<Adt14> {
    let m513 = p514;
    {
        let mut m513 = m513;
        loop {
            let step = {
                let v0 = m513;
                let m509 = v0;
                match m509 {
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
                    Adt14::C4(r510) => {
                        let v5 = (*r510).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r511, r512) => {
                        let v3 = (*r511).clone();
                        let v4 = (*r512).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m513 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m515 = v0;
    match m515.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m516 = f186(v2, v1.clone())?;
            match m516 {
                None => {
                    f95(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f95(v3, v1)?))
                }
            }
        }
    }
}

pub fn f96(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a517 = v0.clone();
        length_bytes(a517)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h518, _) = f76(Fn0::F187(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h518
    };
    let v3: u64 = {
        let (h519, _) = {
            let (_, h520) = v2.clone();
            h520
        };
        h519
    };
    let v4: List<Bytes> = {
        let (_, h521) = {
            let (_, h522) = v2;
            h522
        };
        h521
    };
    let a523 = v3;
    let a524 = v1;
    if nat_lt(a523, a524) {
        Ok(f78(List::cons(f46(v0, v3, {
            let a525 = v1;
            let a526 = v3;
            nat_sub(a525, a526)
        }), v4)))
    } else {
        Ok(f78(v4))
    }
}

pub fn f97(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f188(f57(v0, v1)?))
}

pub fn f98(v0: List<Adt14>, v1: Bytes) -> List<Adt14> {
    let m527 = v0;
    match m527.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f189(v2, v1.clone()), f98(v3, v1))
        }
    }
}

pub fn f99(p532: List<Bytes>, p533: List<Bytes>) -> bool {
    let m531 = (p532, p533);
    {
        let mut m531 = m531;
        loop {
            let step = {
                let (v0, v1) = m531;
                let m528 = v0;
                match m528.uncons() {
                    None => {
                        let m529 = v1;
                        match m529.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m530 = v1;
                        match m530.uncons() {
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
                Ok(next) => { m531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: List<Adt14>) -> List<Bytes> {
    let m534 = v0;
    match m534.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m535 = f94(v1, f8(Str::lit("std")));
            match m535 {
                None => {
                    f100(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f100(v2))
                }
            }
        }
    }
}

pub fn f101(v0: Adt14) -> R<Bytes> {
    let a536 = f190(v0)?;
    let a537 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a536, a537))
}

pub fn f102(v0: List<Adt14>) -> Adt14 {
    let m538 = v0;
    match m538.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f102(v2)))
        }
    }
}

pub fn f103(v0: Adt13) -> Bytes {
    let m539 = v0;
    match m539 {
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

pub fn f104() -> Bytes {
    f8(Str::lit("target/engine/ref.json"))
}

pub fn f105() -> Bytes {
    f8(Str::lit("target/engine/tag.json"))
}

pub fn f106(v0: Adt1, v1: Bytes, v2: Bytes) -> R<Adt12> {
    let a541 = {
        let a540 = v2.clone();
        length_bytes(a540)
    };
    let a542 = 0u64;
    if nat_eq(a541, a542) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m543 = f191(v0.clone(), f60())?;
        match m543 {
            None => {
                if f51(f52(f194(), match v0.clone() {
                    Adt1::C0(_, _, h544, _) => {
                        h544
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f194()))
                } else {
                    let m545 = f56(v0.clone(), List::cons(f195(), f196(f60())?));
                    match m545.uncons() {
                        None => {
                            Ok(Adt12::C3(f197(v0.clone(), f60(), v1.clone(), v2.clone(), f198(f64(v0, f195()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f192(v3.clone())?, f193(v3, v2)))
            }
        }
    }
}

pub fn f107() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f108() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f109() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f110() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f111() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f112() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f113() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f114() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f115(v0: Bytes, p573: u64, p574: u64, p575: bool, p576: bool, p577: u64, p578: Bytes) -> R<Bytes> {
    let m572 = (p573, (p574, (p575, (p576, (p577, p578)))));
    {
        let mut m572 = m572;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m572;
                let m546 = v2;
                if m546 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f199(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m546 - 1;
                    let m549 = {
                        let a547 = v0.clone();
                        let a548 = v1;
                        index_bytes(a547, a548)
                    };
                    match m549 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f199(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f72(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a550 = v1;
                                    let a551 = 1u64;
                                    nat_add(a550, a551)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a552 = v9;
                                    let a553 = 34u64;
                                    !nat_eq(a552, a553)
                                }, (if v4 {
                                    false
                                } else {
                                    let a554 = v9;
                                    let a555 = 92u64;
                                    nat_eq(a554, a555)
                                }, (v5, v6))))))
                            } else {
                                let a556 = v9;
                                let a557 = 32u64;
                                if if if if nat_eq(a556, a557) {
                                    true
                                } else {
                                    let a558 = v9;
                                    let a559 = 9u64;
                                    nat_eq(a558, a559)
                                } {
                                    true
                                } else {
                                    let a560 = v9;
                                    let a561 = 10u64;
                                    nat_eq(a560, a561)
                                } {
                                    true
                                } else {
                                    let a562 = v9;
                                    let a563 = 13u64;
                                    nat_eq(a562, a563)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a564 = v1;
                                        let a565 = 1u64;
                                        nat_add(a564, a565)?
                                    }, (v7, (false, (false, ({
                                        let a566 = v1;
                                        let a567 = 1u64;
                                        nat_add(a566, a567)?
                                    }, f199(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a568 = v1;
                                        let a569 = 1u64;
                                        nat_add(a568, a569)?
                                    }, (v7, ({
                                        let a570 = v9;
                                        let a571 = 34u64;
                                        nat_eq(a570, a571)
                                    }, (false, (v5, v6))))))
                                }
                            }
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

pub fn f116(v0: Bytes, v1: u64) -> R<u64> {
    let m581 = {
        let a579 = v0;
        let a580 = v1;
        index_bytes(a579, a580)
    };
    match m581 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a626 = {
                let a602 = {
                    let a590 = {
                        let a584 = {
                            let a582 = v3;
                            let a583 = 128u8;
                            fixed_u8::bit_and(a582, a583)
                        };
                        let a585 = 128u8;
                        if equal(a584, a585) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a591 = {
                        let a588 = {
                            let a586 = v3;
                            let a587 = 64u8;
                            fixed_u8::bit_and(a586, a587)
                        };
                        let a589 = 64u8;
                        if equal(a588, a589) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a590, a591)?
                };
                let a603 = {
                    let a600 = {
                        let a594 = {
                            let a592 = v3;
                            let a593 = 32u8;
                            fixed_u8::bit_and(a592, a593)
                        };
                        let a595 = 32u8;
                        if equal(a594, a595) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a601 = {
                        let a598 = {
                            let a596 = v3;
                            let a597 = 16u8;
                            fixed_u8::bit_and(a596, a597)
                        };
                        let a599 = 16u8;
                        if equal(a598, a599) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a600, a601)?
                };
                nat_add(a602, a603)?
            };
            let a627 = {
                let a624 = {
                    let a612 = {
                        let a606 = {
                            let a604 = v3;
                            let a605 = 8u8;
                            fixed_u8::bit_and(a604, a605)
                        };
                        let a607 = 8u8;
                        if equal(a606, a607) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a613 = {
                        let a610 = {
                            let a608 = v3;
                            let a609 = 4u8;
                            fixed_u8::bit_and(a608, a609)
                        };
                        let a611 = 4u8;
                        if equal(a610, a611) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a612, a613)?
                };
                let a625 = {
                    let a622 = {
                        let a616 = {
                            let a614 = v3;
                            let a615 = 2u8;
                            fixed_u8::bit_and(a614, a615)
                        };
                        let a617 = 2u8;
                        if equal(a616, a617) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a623 = {
                        let a620 = {
                            let a618 = v3;
                            let a619 = 1u8;
                            fixed_u8::bit_and(a618, a619)
                        };
                        let a621 = 1u8;
                        if equal(a620, a621) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a622, a623)?
                };
                nat_add(a624, a625)?
            };
            nat_add(a626, a627)
        }
    }
}

pub fn f117(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f200(v0.clone(), v1, {
        let a629 = {
            let a628 = v0.clone();
            length_bytes(a628)
        };
        let a630 = 1u64;
        nat_add(a629, a630)?
    })?;
    let a631 = f116(v0.clone(), v2)?;
    let a632 = 34u64;
    if nat_eq(a631, a632) {
        Ok(Some(({
            let a633 = v2;
            let a634 = 1u64;
            nat_add(a633, a634)?
        }, f46(v0, v1, {
            let a635 = v2;
            let a636 = v1;
            nat_sub(a635, a636)
        }))))
    } else {
        f201(v0.clone(), v1, {
            let a638 = {
                let a637 = v0;
                length_bytes(a637)
            };
            let a639 = 1u64;
            nat_add(a638, a639)?
        }, Bytes::lit(&[]))
    }
}

pub fn f118(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f119(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a640 = f116(v0.clone(), v1)?;
        let a641 = 45u64;
        nat_eq(a640, a641)
    };
    let v3: u64 = if v2 {
        let a642 = v1;
        let a643 = 1u64;
        nat_add(a642, a643)?
    } else {
        v1
    };
    let v4: u64 = f202(v0.clone(), v3)?;
    let v5: u64 = {
        let a644 = v4;
        let a645 = v3;
        nat_sub(a644, a645)
    };
    let v6: bool = {
        let a646 = f116(v0.clone(), v4)?;
        let a647 = 46u64;
        nat_eq(a646, a647)
    };
    let v7: u64 = if v6 {
        let a648 = v4;
        let a649 = 1u64;
        nat_add(a648, a649)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f202(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f116(v0.clone(), v8)?;
    let v10: bool = {
        let a650 = v9;
        let a651 = 101u64;
        if nat_eq(a650, a651) {
            true
        } else {
            let a652 = v9;
            let a653 = 69u64;
            nat_eq(a652, a653)
        }
    };
    let v11: u64 = f116(v0.clone(), {
        let a654 = v8;
        let a655 = 1u64;
        nat_add(a654, a655)?
    })?;
    let v12: bool = f118(v10, f203({
        let a656 = v11;
        let a657 = 43u64;
        nat_eq(a656, a657)
    }, {
        let a658 = v11;
        let a659 = 45u64;
        nat_eq(a658, a659)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a660 = v8;
            let a661 = 2u64;
            nat_add(a660, a661)?
        } else {
            let a662 = v8;
            let a663 = 1u64;
            nat_add(a662, a663)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f202(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a664 = v14;
        let a665 = v13;
        nat_sub(a664, a665)
    });
    let v16: u64 = f204(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a667 = {
            let a666 = v15;
            length_bytes(a666)
        };
        let a668 = v16;
        nat_sub(a667, a668)
    });
    let a669 = v5;
    let a670 = 0u64;
    if if nat_eq(a669, a670) {
        true
    } else {
        if f118({
            let a671 = 1u64;
            let a672 = v5;
            nat_lt(a671, a672)
        }, {
            let a673 = f116(v0.clone(), v3)?;
            let a674 = 48u64;
            nat_eq(a673, a674)
        }) {
            true
        } else {
            if f118(v6, {
                let a675 = v8;
                let a676 = v7;
                nat_eq(a675, a676)
            }) {
                true
            } else {
                f203(if v10 {
                    let a677 = v14;
                    let a678 = v13;
                    nat_eq(a677, a678)
                } else {
                    false
                }, {
                    let a680 = 18u64;
                    let a681 = {
                        let a679 = v17.clone();
                        length_bytes(a679)
                    };
                    nat_lt(a680, a681)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m685 = {
            let a683 = {
                let a682 = v17.clone();
                length_bytes(a682)
            };
            let a684 = 0u64;
            if nat_eq(a683, a684) {
                Some(0u64)
            } else {
                f205(v17)?
            }
        };
        match m685 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m690 = f206(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a686 = v8;
                    let a687 = v7;
                    nat_sub(a686, a687)
                }), f118(v12, {
                    let a688 = v11;
                    let a689 = 45u64;
                    nat_eq(a688, a689)
                }), v18)?;
                match m690 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a693 = 19u64;
                        let a694 = {
                            let a692 = {
                                let (h691, _) = v19.clone();
                                h691
                            };
                            length_bytes(a692)
                        };
                        if nat_lt(a693, a694) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m696 = f205({
                                let (h695, _) = v19.clone();
                                h695
                            })?;
                            match m696 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f118(v2, {
                                        let a697 = 0u64;
                                        let a698 = v20;
                                        nat_lt(a697, a698)
                                    });
                                    if f207(v21, v20) {
                                        Ok(Some((v14, {
                                            let a700 = {
                                                let (_, h699) = v19.clone();
                                                h699
                                            };
                                            let a701 = 0u64;
                                            if nat_eq(a700, a701) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h702) = v19;
                                                    h702
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

pub fn f120(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h703, _) = v2.clone();
        h703
    };
    let v4: u64 = {
        let (h704, _) = {
            let (_, h705) = v2.clone();
            h705
        };
        h704
    };
    let v5: List<Bytes> = {
        let (_, h706) = {
            let (_, h707) = v2;
            h707
        };
        h706
    };
    let m708 = f208(v0.clone(), v3);
    match m708 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a709 = v6;
            let a710 = v1;
            if equal(a709, a710) {
                Ok(Some(({
                    let a711 = v3;
                    let a712 = 1u64;
                    nat_add(a711, a712)?
                }, ({
                    let a713 = v3;
                    let a714 = 1u64;
                    nat_add(a713, a714)?
                }, List::cons(f46(v0, v4, {
                    let a715 = v3;
                    let a716 = v4;
                    nat_sub(a715, a716)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a717 = v3;
                    let a718 = 1u64;
                    nat_add(a717, a718)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f121(v0: Fn1, p722: List<Bytes>, p723: List<Bytes>) -> List<Bytes> {
    let m721 = (p722, p723);
    {
        let mut m721 = m721;
        loop {
            let step = {
                let (v1, v2) = m721;
                let m719 = v2;
                match m719.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c720: Fn1 = v0.clone();
                            c720.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m721 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f123(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m724 = f209(Fn2::F210(v1), None::<Bytes>, v0);
    match m724 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f124() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::<Bytes>::nil()))))))))))))))))))))))))))))
}

pub fn f125(v0: Adt1) -> List<Bytes> {
    if f139(match v0.clone() {
        Adt1::C0(_, h725, _, _) => {
            h725
        }
    }, f211()) {
        if f139(match v0 {
            Adt1::C0(_, h726, _, _) => {
                h726
            }
        }, f212()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f212(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f211(), List::<Bytes>::nil())
    }
}

pub fn f126(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m727 = v0;
    match m727.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt1) -> R<List<Bytes>> {
    Ok(f56(v0.clone(), List::cons(f213(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f137(v0)?)))))
}

pub fn f128(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f214(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f143(f97(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f129(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m728 = v0;
    match m728.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f129(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f129(v3, v1))
            }
        }
    }
}

pub fn f130(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m729 = v0;
    match m729.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f130(v3, v1)
            } else {
                List::cons(v2, f130(v3, v1))
            }
        }
    }
}

pub fn f131(v0: List<Adt14>) -> List<Bytes> {
    let m730 = v0;
    match m730.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f126(f215(f188(f94(v1, f8(Str::lit("defers"))))), f131(v2))
        }
    }
}

pub fn f132(v0: List<Adt14>) -> List<Bytes> {
    let m731 = v0;
    match m731.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m732 = f94(v1, f8(Str::lit("std")));
            match m732 {
                None => {
                    f132(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("row"))), f132(v2))
                }
            }
        }
    }
}

pub fn f133(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m733 = v0;
    match m733.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f133(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), f133(v3, v1))
            }
        }
    }
}

pub fn f134(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f155(f216(v2.clone()), f217(f218(v0, v1.clone(), v2)?));
    f219(f146(f220(v1))?, v3.clone(), {
        let a735 = {
            let a734 = v3;
            length_list(a734)
        };
        let a736 = 1u64;
        nat_add(a735, a736)?
    })
}

pub fn f135(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m737 = v1;
    match m737.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f221(v2.clone(), f8(Str::lit("subject"))) {
                f222(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f135(v0, v3))
            } else {
                f135(v0, v3)
            }
        }
    }
}

pub fn f136(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m738 = v1;
    match m738.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m739 = f223(f96(f64(v0.clone(), v2.clone()))?, f85(v2, f212()))?;
            match m739 {
                None => {
                    f136(v0, v3)
                }
                Some(v4) => {
                    Ok(f224(v4, f136(v0, v3)?))
                }
            }
        }
    }
}

pub fn f137(v0: Adt1) -> R<List<Bytes>> {
    Ok(f126(f146(f225(f211(), f148(match v0.clone() {
        Adt1::C0(_, h740, _, _) => {
            h740
        }
    }, f211())))?, f146(f225(f212(), f148(match v0 {
        Adt1::C0(_, h741, _, _) => {
            h741
        }
    }, f212())))?))
}

pub fn f138(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f139(p745: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m744 = p745;
    {
        let mut m744 = m744;
        loop {
            let step = {
                let v0 = m744;
                let m742 = v0;
                match m742.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h743, _) = v2;
                            h743
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m744 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140() -> Bytes {
    f8(Str::lit("model/registries"))
}

pub fn f141(v0: Adt1) -> R<List<Bytes>> {
    f146(f225(f140(), f148(match v0 {
        Adt1::C0(_, h746, _, _) => {
            h746
        }
    }, f140())))
}

pub fn f142(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m747 = v1;
    match m747.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f126(if f52(v4.clone(), v3.clone()) {
                f226(v4.clone(), f96(f64(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f227(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
            }, f142(v0, v5, v2, v3)?))
        }
    }
}

pub fn f143(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m748 = v0;
    match m748.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f143(v3, v1))
        }
    }
}

pub fn f144(v0: Adt1) -> R<Adt12> {
    let m749 = f56(v0.clone(), f228());
    match m749.uncons() {
        None => {
            let m750 = f229(v0.clone(), f228())?;
            match m750.uncons() {
                None => {
                    let m753 = f230(match v0.clone() {
                        Adt1::C0(_, h751, _, _) => {
                            h751
                        }
                    }, f126(f231(), f232(match v0.clone() {
                        Adt1::C0(_, h752, _, _) => {
                            h752
                        }
                    }, false)));
                    match m753.uncons() {
                        None => {
                            let m755 = f56(v0.clone(), f232(match v0.clone() {
                                Adt1::C0(_, h754, _, _) => {
                                    h754
                                }
                            }, true));
                            match m755.uncons() {
                                None => {
                                    let m756 = f56(v0.clone(), f233());
                                    match m756.uncons() {
                                        None => {
                                            let m757 = f234(v0.clone())?;
                                            match m757 {
                                                None => {
                                                    let m759 = f235(v0.clone(), f232(match v0.clone() {
                                                        Adt1::C0(_, h758, _, _) => {
                                                            h758
                                                        }
                                                    }, true))?;
                                                    match m759 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f236(Adt17::C0(Adt16::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f214(v0.clone())?, f143(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f97(v0.clone(), f8(Str::lit("model/strata.json")))?, f215(f97(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f232(match v0 {
                                                                Adt1::C0(_, h760, _, _) => {
                                                                    h760
                                                                }
                                                            }, true)))?;
                                                            let (h761, _) = v13;
                                                            if h761 {
                                                                Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h762) = v13;
                                                                    h762
                                                                }))
                                                            } else {
                                                                Ok(Adt12::C4({
                                                                    let (_, h763) = v13;
                                                                    h763
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

pub fn f145() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f146(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f237(f121(Fn1::F238, List::<Bytes>::nil(), f239(v0)?)))
}

pub fn f147(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m764 = v1;
    match m764.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f240(v0.clone(), v2), f147(v0, v3))
            } else {
                f147(v0, v3)
            }
        }
    }
}

pub fn f148(p769: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m768 = p769;
    {
        let mut m768 = m768;
        loop {
            let step = {
                let v0 = m768;
                let m765 = v0;
                match m765.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h766, _) = v2;
                            h766
                        }, v1.clone()) {
                            let (_, h767) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h767)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m768 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, p772: List<Bytes>) -> R<Option<Bytes>> {
    let m771 = p772;
    {
        let mut m771 = m771;
        loop {
            let step = {
                let v2 = m771;
                let m770 = v2;
                match m770.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f241(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: List<List<Bytes>>) -> R<List<(List<Bytes>, List<Bytes>)>> {
    let m773 = v0;
    match m773.uncons() {
        None => {
            Ok(List::<(List<Bytes>, List<Bytes>)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f242(v1.clone())?, v1), f150(v2)?))
        }
    }
}

pub fn f151(p776: List<Bytes>, p777: List<Bytes>, p778: List<List<Bytes>>) -> List<List<Bytes>> {
    let m775 = (p776, (p777, p778));
    {
        let mut m775 = m775;
        loop {
            let step = {
                let (v0, (v1, v2)) = m775;
                let m774 = v0;
                match m774.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f243(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f243(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m775 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m779 = v0;
    match m779.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f152(v3, v1)
            } else {
                List::cons(v2, f152(v3, v1))
            }
        }
    }
}

pub fn f153(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f139(match v0.clone() {
        Adt1::C0(_, h780, _, _) => {
            h780
        }
    }, f244())) {
        Ok(Adt12::C1(List::cons(f244(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f152(f146(f147(f244(), f148(match v0.clone() {
            Adt1::C0(_, h781, _, _) => {
                h781
            }
        }, f244())))?, v1.clone());
        let m782 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m782.uncons() {
            None => {
                let v5: List<Bytes> = f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m783 = f56(v0.clone(), f245(v0.clone(), v2.clone(), v5.clone())?);
                match m783.uncons() {
                    None => {
                        let m784 = f246(v0.clone(), v2, v5)?;
                        match m784 {
                            None => {
                                f247(v0, v1)
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

pub fn f154() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f155(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m785 = v0;
    match m785.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f155(v3, v1))
        }
    }
}

pub fn f156(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m786 = f97(v0.clone(), f154())?;
    match m786.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f248(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f157() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f158() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/ or certified-readers/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- certified certified-readers | head -20\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f159() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f249(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op > target/attempt.txt 2>&1; then\u{a}                tee -a target/operation.txt < target/attempt.txt\u{a}                last=$op\u{a}              else\u{a}                cat target/attempt.txt\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f160() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}      merges: ${{ steps.stale.outputs.merges }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          merges=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # Drafting drops a pull request's auto-merge, so the owner's merge method is kept\u{a}              # here and restored once the regenerated pull request is ready again.\u{a}              method=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge.merge_method // \"\"')\u{a}              if [ -n \"$method\" ]; then merges=\"$merges $pr:$method\"; fi\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"merges=$merges\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}          MERGES: ${{ needs.find.outputs.merges }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f249(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            # The owner's auto-merge comes back with the same method, and only where it was set.\u{a}            for entry in $MERGES; do\u{a}              if [ \"${entry/:*/}\" = \"$pr\" ] && ! gh pr merge \"$pr\" --auto \"--${entry#*:}\"; then\u{a}                echo \"::error::#${pr}: its auto-merge (${entry#*:}) could not be restored\"\u{a}                failed=\"$failed $pr\"\u{a}              fi\u{a}            done\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f161() -> Bytes {
    f8(Str::lit("#!/usr/bin/env bash\u{a}# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one Lean export of the certified command. operate's propose-command and refresh both run it,\u{a}# and the studio check runs propose-command end to end, so a change that breaks it fails there.\u{a}# Export time is a requirement: every Lean phase runs under child_timeout_ms (300 s, studio/lexlean.toml),\u{a}# so this script measures each phase and fails at 240 s, 80 percent of that limit, before time can creep up to it.\u{a}set -euo pipefail\u{a}ll=\"$1\"\u{a}log=\"$2\"\u{a}phases=\"$log.phases\"\u{a}: > \"$phases.raw\"\u{a}( while :; do ps -eo etimes=,args= | awk '$2 == \"lean\" && $3 == \"-R\" { m = $NF; sub(\".*-src/\", \"\", m); print $1, m }' >> \"$phases.raw\" || true; sleep 1; done ) &\u{a}watch=$!\u{a}status=0\u{a}# Each LexLean project exports in parallel under its own limits: studio, and readers when it exists.\u{a}pids=\"\"\u{a}for project in studio readers; do\u{a}  if [ -f \"$project/lexlean.toml\" ]; then\u{a}    (cd \"$project\" && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > \"$log.$project\" 2>&1 &\u{a}    pids=\"$pids $!\"\u{a}  fi\u{a}done\u{a}for pid in $pids; do wait \"$pid\" || status=$?; done\u{a}cat \"$log.studio\" > \"$log\"\u{a}if [ -f \"$log.readers\" ]; then cat \"$log.readers\" >> \"$log\"; fi\u{a}kill \"$watch\" 2> \"$phases.err\" || true\u{a}awk '{ if ($1 > t[$2]) t[$2] = $1 } END { for (m in t) print t[m], m }' \"$phases.raw\" | sort -rn > \"$phases\"\u{a}echo \"Lean phases by wall time in seconds, longest first (the limit is 300, and the export fails at 240):\"\u{a}head -n 8 \"$phases\"\u{a}if [ \"$status\" -ne 0 ]; then exit \"$status\"; fi\u{a}worst=$(head -n 1 \"$phases\" | cut -d ' ' -f 1)\u{a}if [ -n \"$worst\" ] && [ \"$worst\" -ge 240 ]; then\u{a}  echo \"::error::a Lean phase took ${worst} s, within 20 percent of the 300 s limit: make its declarations cheaper to prove before adding more\"\u{a}  exit 1\u{a}fi\u{a}rm -rf certified\u{a}cp -r studio/.lexlean/production/*/ certified\u{a}rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}\"$ll\" production verify-artifact certified\u{a}if [ -f readers/lexlean.toml ]; then\u{a}  rm -rf certified-readers\u{a}  cp -r readers/.lexlean/production/*/ certified-readers\u{a}  rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}  \"$ll\" production verify-artifact certified-readers\u{a}fi\u{a}"))
}

pub fn f162() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f163(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f128(v0.clone())?;
    let v2: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f135(v1.clone(), f136(v0.clone(), f137(v0.clone())?)?);
    let v4: List<List<Bytes>> = f250(f251(f64(v0, f213()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f252(v4.clone(), f253(v4), f218(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f134(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f164(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f254(f128(v0.clone())?, f136(v0.clone(), f137(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f165(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f255(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f256(f97(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f166() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f167(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m787 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m787.uncons() {
        None => {
            if f257(f96(f64(v0, v1))?)? {
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

pub fn f168(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m788 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m788.uncons() {
        None => {
            let v5: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f258(v5)?
                } else {
                    f259(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f240(f260(), f261(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f240(f260(), f261(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f262()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f169(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m789 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m789.uncons() {
        None => {
            let m790 = f263(f96(f64(v0.clone(), v1.clone()))?)?;
            match m790 {
                None => {
                    Ok(Adt12::C4(f264()))
                }
                Some(v4) => {
                    let m791 = f265(f96(f64(v0, v1))?)?;
                    match m791 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f266(v5)))
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

pub fn f170(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let v2: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
    if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
        Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
    } else {
        if f118(f267(f268(v2.clone())?), f51({
            let a793 = {
                let a792 = f269(f268(v2)?);
                length_list(a792)
            };
            let a794 = 0u64;
            nat_eq(a793, a794)
        })) {
            Ok(Adt12::C3(List::cons((f240(f260(), f261(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f240(f260(), f261(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt12::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f171(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m795 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m795.uncons() {
        None => {
            let m796 = f270(f96(f64(v0, v1))?)?;
            match m796 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f172(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m797 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m797.uncons() {
        None => {
            let v5: Bytes = f240(if v2 {
                f212()
            } else {
                f211()
            }, f261(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m798 = f223(f96(f64(v0.clone(), v1.clone()))?, v2)?;
                match m798 {
                    None => {
                        Ok(Adt12::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a800 = {
                            let a799 = v6;
                            length_list(a799)
                        };
                        let a801 = 0u64;
                        if nat_eq(a800, a801) {
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

pub fn f173(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m802 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m802.uncons() {
        None => {
            let v4: List<Bytes> = f96(f64(v0.clone(), v1))?;
            let m803 = f271(v4.clone(), f8(Str::lit("requirement")))?;
            match m803 {
                None => {
                    Ok(Adt12::C4(f262()))
                }
                Some(v5) => {
                    let m804 = f271(v4, f8(Str::lit("admission")))?;
                    match m804 {
                        None => {
                            Ok(Adt12::C4(f262()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f126(f143(v5.clone(), f8(Str::lit("id"))), f143(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f146(f126(f143(v7.clone(), f8(Str::lit("id"))), f143(v8.clone(), f8(Str::lit("construct")))))?;
                            let m805 = f272(f132(v6.clone()), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m805 {
                                None => {
                                    let a807 = {
                                        let a806 = v9.clone();
                                        length_list(a806)
                                    };
                                    let a808 = 0u64;
                                    if nat_eq(a807, a808) {
                                        Ok(Adt12::C4(f262()))
                                    } else {
                                        if f51({
                                            let a811 = {
                                                let a809 = f146(f126(v9.clone(), v10.clone()))?;
                                                length_list(a809)
                                            };
                                            let a812 = {
                                                let a810 = f126(v9.clone(), v10);
                                                length_list(a810)
                                            };
                                            nat_eq(a811, a812)
                                        }) {
                                            Ok(Adt12::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt12::C3(List::cons(f274(f8(Str::lit("model/requirements.json")), f224(v7, v5))?, List::cons(f274(f8(Str::lit("model/admissions.json")), f224(v8, v6))?, List::cons(f275(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f123(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f273(v11))
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

pub fn f174(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m813 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m813.uncons() {
        None => {
            let m814 = f91(v0.clone(), v1.clone())?;
            match m814 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m815 = f276(v4.clone(), f8(Str::lit("schema")));
                    match m815 {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f277(v0, f278(v5.clone())?, v5, f279(v4.clone(), f8(Str::lit("name"))), f280(f281(f276(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f175(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m816 = v2;
    match m816.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m817 = v4;
            match m817.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m818 = v6.clone();
                    match m818.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((v7, v8)) => {
                            let m819 = v8;
                            match m819.uncons() {
                                None => {
                                    Ok(Adt12::C4(f79()))
                                }
                                Some((v9, _)) => {
                                    if f51(f85(v7, f8(Str::lit("read-")))) {
                                        Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that reads the artifact: admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT\u{a}"))))
                                    } else {
                                        let m820 = f56(v0.clone(), List::cons(f282(v1.clone()), List::cons(v9.clone(), List::cons(f8(Str::lit("model/admissions.json")), if f32(v5.clone(), f8(Str::lit("none"))) {
                                            List::<Bytes>::nil()
                                        } else {
                                            List::cons(f7(List::cons(f8(Str::lit("encodings/")), List::cons(v5.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), List::<Bytes>::nil())
                                        }))));
                                        match m820.uncons() {
                                            None => {
                                                if f51(f67(v0.clone(), f282(v1.clone()))) {
                                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v1.clone(), List::cons(f8(Str::lit(" is not a bound authority, because ")), List::cons(f282(v1), List::cons(f8(Str::lit(" is absent, and every admitted value cites a bound standard (REQ-ADMISSION)\u{a}")), List::<Bytes>::nil()))))))))
                                                } else {
                                                    let m821 = f57(v0.clone(), f282(v1.clone()))?;
                                                    match m821 {
                                                        None => {
                                                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(f282(v1), List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                        Some(v13) => {
                                                            let v15: Bytes = {
                                                                let m822 = f94(v13, f8(Str::lit("source")));
                                                                match m822 {
                                                                    None => {
                                                                        Bytes::lit(&[])
                                                                    }
                                                                    Some(v14) => {
                                                                        f58(v14, f8(Str::lit("sha256")))
                                                                    }
                                                                }
                                                            };
                                                            let v16: Bytes = f283(f64(v0.clone(), v9.clone()))?;
                                                            if f51(f32(v16.clone(), v15.clone())) {
                                                                Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v9, List::cons(f8(Str::lit(" is not the artifact ")), List::cons(v1, List::cons(f8(Str::lit(" binds: its SHA-256 is ")), List::cons(v16, List::cons(f8(Str::lit(" and the binding's source.sha256 is ")), List::cons(v15, List::cons(f8(Str::lit(" (FIPS 180-4 section 6.2)\u{a}")), List::<Bytes>::nil()))))))))))))
                                                            } else {
                                                                f284(v0, v1, v3, v5, v6, v9)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Some((v11, v12)) => {
                                                Ok(Adt12::C0(List::cons(v11, v12)))
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

pub fn f176(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m823 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m823.uncons() {
        None => {
            let m824 = f57(v0.clone(), v1.clone())?;
            match m824 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f285())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m825 = f94(v4.clone(), f8(Str::lit("source")));
                            match m825 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f93(f94(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f240(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f240(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m826 = v8;
                        match m826 {
                            None => {
                                f286(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f286(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f240(v9, f261(f58(v7.clone(), f8(Str::lit("url"))))?);
                                    if f51(f52(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h827, _) => {
                                            h827
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f58(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m828 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m828.uncons() {
                                            None => {
                                                if f32(f283(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f286(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f177(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m829 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m829.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f251(v4.clone())?;
            if f51(f99(f287(v5.clone()), f288())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f289(Str::lit("id"), v6.clone())?, List::cons(f289(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f289(Str::lit("artifact"), f213())?, List::cons(f289(Str::lit("checksum"), f283(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f213(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f290(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f275(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f123(f253(f250(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f178(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m830 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m830.uncons() {
        None => {
            let m831 = f57(v0, v1)?;
            match m831 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f291(f188(Some(v4))) {
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

pub fn f179(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt12> {
    let m832 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m832.uncons() {
        None => {
            let m833 = f91(v0.clone(), v1.clone())?;
            match m833 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f277(v0.clone(), f64(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f180(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt12> {
    let m834 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil())))))));
    match m834.uncons() {
        None => {
            let m835 = f292(v3.clone());
            match m835.uncons() {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                }
                Some((v6, v7)) => {
                    if f85(v6.clone(), f8(Str::lit("read-"))) {
                        f293(v0.clone(), f64(v0, v1), v2, List::cons(v6, v7), f294(v3))
                    } else {
                        Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                    }
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f181(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m836 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f154(), List::<Bytes>::nil())));
    match m836.uncons() {
        None => {
            let m837 = f295(f96(f64(v0.clone(), v1))?)?;
            match m837 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f97(v0, f154())?;
                    let v6: List<Bytes> = f143(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f146(f143(v5.clone(), f8(Str::lit("at"))))?;
                    let a839 = {
                        let a838 = v6.clone();
                        length_list(a838)
                    };
                    let a840 = 0u64;
                    if if nat_eq(a839, a840) {
                        true
                    } else {
                        f51({
                            let a843 = {
                                let a841 = f146(f126(v6.clone(), v7.clone()))?;
                                length_list(a841)
                            };
                            let a844 = {
                                let a842 = f126(v6.clone(), v7);
                                length_list(a842)
                            };
                            nat_eq(a843, a844)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f274(f154(), f224(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f123(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f182(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m845 = v2;
    match m845.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m846 = v4;
            match m846.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m847 = v6.clone();
                    match m847.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((_, _)) => {
                            let m848 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f296(), List::<Bytes>::nil())));
                            match m848.uncons() {
                                None => {
                                    let v11: Bytes = f123(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt14> = f97(v0, f296())?;
                                    if f297(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f274(f296(), f224(v13, List::cons(f298(v11.clone(), v12, v5, v3), List::<Adt14>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f183(v0: Bytes, p902: u64, p903: u64) -> R<bool> {
    let m901 = (p902, p903);
    {
        let mut m901 = m901;
        loop {
            let step = {
                let (v1, v2) = m901;
                let m849 = v2;
                if m849 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m849 - 1;
                    let m852 = {
                        let a850 = v0.clone();
                        let a851 = v1;
                        index_bytes(a850, a851)
                    };
                    match m852 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f299({
                                let v5: u8 = v4;
                                let a897 = {
                                    let a873 = {
                                        let a861 = {
                                            let a855 = {
                                                let a853 = v5;
                                                let a854 = 128u8;
                                                fixed_u8::bit_and(a853, a854)
                                            };
                                            let a856 = 128u8;
                                            if equal(a855, a856) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a862 = {
                                            let a859 = {
                                                let a857 = v5;
                                                let a858 = 64u8;
                                                fixed_u8::bit_and(a857, a858)
                                            };
                                            let a860 = 64u8;
                                            if equal(a859, a860) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a861, a862)?
                                    };
                                    let a874 = {
                                        let a871 = {
                                            let a865 = {
                                                let a863 = v5;
                                                let a864 = 32u8;
                                                fixed_u8::bit_and(a863, a864)
                                            };
                                            let a866 = 32u8;
                                            if equal(a865, a866) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a872 = {
                                            let a869 = {
                                                let a867 = v5;
                                                let a868 = 16u8;
                                                fixed_u8::bit_and(a867, a868)
                                            };
                                            let a870 = 16u8;
                                            if equal(a869, a870) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a871, a872)?
                                    };
                                    nat_add(a873, a874)?
                                };
                                let a898 = {
                                    let a895 = {
                                        let a883 = {
                                            let a877 = {
                                                let a875 = v5;
                                                let a876 = 8u8;
                                                fixed_u8::bit_and(a875, a876)
                                            };
                                            let a878 = 8u8;
                                            if equal(a877, a878) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a884 = {
                                            let a881 = {
                                                let a879 = v5;
                                                let a880 = 4u8;
                                                fixed_u8::bit_and(a879, a880)
                                            };
                                            let a882 = 4u8;
                                            if equal(a881, a882) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a883, a884)?
                                    };
                                    let a896 = {
                                        let a893 = {
                                            let a887 = {
                                                let a885 = v5;
                                                let a886 = 2u8;
                                                fixed_u8::bit_and(a885, a886)
                                            };
                                            let a888 = 2u8;
                                            if equal(a887, a888) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a894 = {
                                            let a891 = {
                                                let a889 = v5;
                                                let a890 = 1u8;
                                                fixed_u8::bit_and(a889, a890)
                                            };
                                            let a892 = 1u8;
                                            if equal(a891, a892) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a893, a894)?
                                    };
                                    nat_add(a895, a896)?
                                };
                                nat_add(a897, a898)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a899 = v1;
                                    let a900 = 1u64;
                                    nat_add(a899, a900)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(p908: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m907 = p908;
    {
        let mut m907 = m907;
        loop {
            let step = {
                let v0 = m907;
                let m904 = v0;
                match m904.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h905, _) = v2;
                            h905
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h906) = v2;
                                h906
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
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

pub fn f185(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m912 = {
        let a910 = match v0 {
            Adt1::C0(_, _, _, h909) => {
                h909
            }
        };
        let a911 = f40(v1);
        index_list(a910, a911)
    };
    match m912 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f186(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), v1) {
        let m913 = f44(v0, 34u8)?;
        match m913.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m914 = v3;
                match m914.uncons() {
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

pub fn f187(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f300(v0, v1)
}

pub fn f188(v0: Option<Adt14>) -> List<Adt14> {
    let m915 = v0;
    match m915 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f301(v1)
        }
    }
}

pub fn f189(v0: Adt14, v1: Bytes) -> Adt14 {
    let m916 = v0;
    match m916 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r917) => {
            let v10 = (*r917).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r918) => {
            let v5 = (*r918).clone();
            Adt14::C4(std::rc::Rc::new(f189(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r919, r920) => {
            let v11 = (*r919).clone();
            let v12 = (*r920).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r921, r922) => {
            let v3 = (*r921).clone();
            let v4 = (*r922).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt14::C7(v2, std::rc::Rc::new(f302(v3, v1.clone())), std::rc::Rc::new(f189(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f189(v4, v1)))
            }
        }
    }
}

pub fn f190(v0: Adt14) -> R<Bytes> {
    let m923 = v0;
    match m923 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f303(v2, v3)
        }
        Adt14::C2(v4) => {
            f304(v4)
        }
        Adt14::C3(r924) => {
            let v5 = (*r924).clone();
            Ok(f305(f306(v5)?))
        }
        Adt14::C4(r925) => {
            let v6 = (*r925).clone();
            f307(f308(v6)?)
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

pub fn f191(v0: Adt1, p929: List<Adt13>) -> R<Option<Adt13>> {
    let m928 = p929;
    {
        let mut m928 = m928;
        loop {
            let step = {
                let v1 = m928;
                let m926 = v1;
                match m926.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f192(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h927, _) => {
                                h927
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
                Ok(next) => { m928 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: Adt13) -> R<Bytes> {
    Ok(f240(f309(), f261(f310(v0))?))
}

pub fn f193(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f310(v0), List::<Bytes>::nil())))))
}

pub fn f194() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f195() -> Bytes {
    f311()
}

pub fn f196(v0: List<Adt13>) -> R<List<Bytes>> {
    let m930 = v0;
    match m930.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f192(v1)?, f196(v2)?))
        }
    }
}

pub fn f197(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m931 = v1;
    match m931.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f312(v5.clone(), v2.clone(), v3.clone(), f283(f64(v0.clone(), f192(v5)?))?, v4.clone())?), f197(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f198(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f313(v0.clone())?;
    let v2: u64 = f314(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a932 = v1;
        let a933 = v2;
        nat_sub(a932, a933)
    }))
}

pub fn f199(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a936 = v3;
    let a937 = f46(v0, v1, {
        let a934 = v2;
        let a935 = v1;
        nat_sub(a934, a935)
    });
    append_bytes(a936, a937)
}

pub fn f200(v0: Bytes, p950: u64, p951: u64) -> R<u64> {
    let m949 = (p950, p951);
    {
        let mut m949 = m949;
        loop {
            let step = {
                let (v1, v2) = m949;
                let m938 = v2;
                if m938 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m938 - 1;
                    let v4: u64 = f116(v0.clone(), v1)?;
                    let a939 = v4;
                    let a940 = 34u64;
                    if nat_eq(a939, a940) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a941 = v4;
                        let a942 = 92u64;
                        if nat_eq(a941, a942) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a943 = v4;
                            let a944 = 32u64;
                            if nat_lt(a943, a944) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a945 = v4;
                                let a946 = 256u64;
                                if nat_eq(a945, a946) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a947 = v1;
                                        let a948 = 1u64;
                                        nat_add(a947, a948)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m949 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: Bytes, p977: u64, p978: u64, p979: Bytes) -> R<Option<(u64, Bytes)>> {
    let m976 = (p977, (p978, p979));
    {
        let mut m976 = m976;
        loop {
            let step = {
                let (v1, (v2, v3)) = m976;
                let m952 = v2;
                if m952 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m952 - 1;
                    let v5: u64 = f200(v0.clone(), v1, {
                        let a956 = {
                            let a954 = {
                                let a953 = v0.clone();
                                length_bytes(a953)
                            };
                            let a955 = v1;
                            nat_sub(a954, a955)
                        };
                        let a957 = 1u64;
                        nat_add(a956, a957)?
                    })?;
                    let v6: Bytes = {
                        let a960 = v3;
                        let a961 = f46(v0.clone(), v1, {
                            let a958 = v5;
                            let a959 = v1;
                            nat_sub(a958, a959)
                        });
                        append_bytes(a960, a961)
                    };
                    let v7: u64 = f116(v0.clone(), v5)?;
                    let a962 = v7;
                    let a963 = 34u64;
                    if nat_eq(a962, a963) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a964 = v5;
                            let a965 = 1u64;
                            nat_add(a964, a965)?
                        }, v6))))
                    } else {
                        let a966 = v7;
                        let a967 = 92u64;
                        if nat_eq(a966, a967) {
                            let m968 = f315(v0.clone(), v5, v6)?;
                            match m968 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a971 = {
                                        let (_, h969) = {
                                            let (_, h970) = v8.clone();
                                            h970
                                        };
                                        h969
                                    };
                                    let a972 = 0u64;
                                    if nat_eq(a971, a972) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h973, _) = v8;
                                            h973
                                        }, (v4, {
                                            let (h974, _) = {
                                                let (_, h975) = v8;
                                                h975
                                            };
                                            h974
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
                Ok(next) => { m976 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: Bytes, v1: u64) -> R<u64> {
    let (h980, _) = f316(Fn3::F317(v0.clone()), {
        let a982 = {
            let a981 = v0;
            length_bytes(a981)
        };
        let a983 = 1u64;
        nat_add(a982, a983)?
    }, v1)?;
    Ok(h980)
}

pub fn f203(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f204(v0: Bytes) -> R<u64> {
    let (h984, _) = f316(Fn3::F318(v0.clone()), {
        let a986 = {
            let a985 = v0;
            length_bytes(a985)
        };
        let a987 = 1u64;
        nat_add(a986, a987)?
    }, 0u64)?;
    Ok(h984)
}

pub fn f205(v0: Bytes) -> R<Option<u64>> {
    if f319(v0.clone())? {
        Ok(Some({
            let (_, h988) = {
                let (h989, _) = f320(Fn4::F321(v0.clone()), {
                    let a990 = v0;
                    length_bytes(a990)
                }, (0u64, 0u64))?;
                h989
            };
            h988
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f206(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a992 = {
            let a991 = v1.clone();
            length_bytes(a991)
        };
        let a993 = f322(v1)?;
        nat_sub(a992, a993)
    });
    let v5: Bytes = {
        let a994 = v0;
        let a995 = v4.clone();
        append_bytes(a994, a995)
    };
    let v6: u64 = f204(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a997 = {
            let a996 = v5;
            length_bytes(a996)
        };
        let a998 = v6;
        nat_sub(a997, a998)
    });
    let v8: u64 = {
        let a999 = v4;
        length_bytes(a999)
    };
    let v9: u64 = {
        let a1000 = v7.clone();
        length_bytes(a1000)
    };
    let a1001 = v9;
    let a1002 = 0u64;
    if nat_eq(a1001, a1002) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f203(v2, {
            let a1003 = v3;
            let a1004 = v8;
            nat_lt(a1003, a1004)
        }) {
            let v10: u64 = if v2 {
                let a1005 = v3;
                let a1006 = v8;
                nat_add(a1005, a1006)?
            } else {
                let a1007 = v8;
                let a1008 = v3;
                nat_sub(a1007, a1008)
            };
            let v11: u64 = f322(v7.clone())?;
            let a1009 = v10;
            let a1010 = v11;
            if nat_le(a1009, a1010) {
                Ok(Some((f46(v7, 0u64, {
                    let a1011 = v9;
                    let a1012 = v10;
                    nat_sub(a1011, a1012)
                }), 0u64)))
            } else {
                let a1015 = {
                    let a1013 = v10;
                    let a1014 = v11;
                    nat_sub(a1013, a1014)
                };
                let a1016 = 19u64;
                if nat_le(a1015, a1016) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1017 = v9;
                        let a1018 = v11;
                        nat_sub(a1017, a1018)
                    }), {
                        let a1019 = v10;
                        let a1020 = v11;
                        nat_sub(a1019, a1020)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1021 = v3;
                let a1022 = v8;
                nat_sub(a1021, a1022)
            };
            let a1025 = {
                let a1023 = v9;
                let a1024 = v12;
                nat_add(a1023, a1024)?
            };
            let a1026 = 19u64;
            if nat_le(a1025, a1026) {
                Ok(Some(({
                    let a1027 = v7;
                    let a1028 = f323(v12);
                    append_bytes(a1027, a1028)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f207(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1029 = 0u64;
        let a1030 = v1;
        if nat_lt(a1029, a1030) {
            let a1031 = v1;
            let a1032 = 9223372036854775808u64;
            nat_le(a1031, a1032)
        } else {
            false
        }
    } else {
        let a1033 = v1;
        let a1034 = 9223372036854775807u64;
        nat_le(a1033, a1034)
    }
}

pub fn f208(v0: Bytes, v1: u64) -> Option<u8> {
    let a1035 = v0;
    let a1036 = v1;
    index_bytes(a1035, a1036)
}

pub fn f209(v0: Fn2, p1040: Option<Bytes>, p1041: List<Bytes>) -> Option<Bytes> {
    let m1039 = (p1040, p1041);
    {
        let mut m1039 = m1039;
        loop {
            let step = {
                let (v1, v2) = m1039;
                let m1037 = v2;
                match m1037.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1038: Fn2 = v0.clone();
                            c1038.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1039 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f210(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1042 = v1;
    match m1042 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1045 = {
                    let a1043 = v3;
                    let a1044 = v0;
                    append_bytes(a1043, a1044)
                };
                let a1046 = v2;
                append_bytes(a1045, a1046)
            })
        }
    }
}

pub fn f211() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f212() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f213() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f214(v0: Adt1) -> R<List<Bytes>> {
    let m1047 = f251(f64(v0, f213()))?;
    match m1047.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f253(v2))
        }
    }
}

pub fn f215(v0: List<Adt14>) -> List<Bytes> {
    let m1048 = v0;
    match m1048.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1049 = f93(Some(v1));
            match m1049 {
                None => {
                    f215(v2)
                }
                Some(v3) => {
                    List::cons(v3, f215(v2))
                }
            }
        }
    }
}

pub fn f216(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1050 = v0;
    match m1050.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f216(v2))
        }
    }
}

pub fn f217(v0: List<Adt18>) -> List<(Bytes, Bytes)> {
    let m1051 = v0;
    match m1051.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt18::C0(h1052, _, _) => {
                    h1052
                }
            }, match v1 {
                Adt18::C0(_, _, h1053) => {
                    h1053
                }
            }), f217(v2))
        }
    }
}

pub fn f218(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt18>> {
    let v3: List<(Bytes, Bytes)> = f324(v1);
    let v4: List<(Bytes, Bytes)> = f325(v0, v2);
    f326(v3.clone(), f155(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f219(p1060: List<Bytes>, v1: List<(Bytes, Bytes)>, p1061: u64) -> R<List<Bytes>> {
    let m1059 = (p1060, p1061);
    {
        let mut m1059 = m1059;
        loop {
            let step = {
                let (v0, v2) = m1059;
                let m1054 = v2;
                if m1054 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1054 - 1;
                    let v4: List<Bytes> = f146(f126(v0.clone(), f327(v0.clone(), v1.clone())))?;
                    let a1057 = {
                        let a1055 = v4.clone();
                        length_list(a1055)
                    };
                    let a1058 = {
                        let a1056 = v0.clone();
                        length_list(a1056)
                    };
                    if nat_eq(a1057, a1058) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1059 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: List<Adt14>) -> List<Bytes> {
    let m1062 = v0;
    match m1062.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1063 = f94(v1, f8(Str::lit("std")));
            match m1063 {
                None => {
                    f220(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f221(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f220(v2))
                    } else {
                        f220(v2)
                    })
                }
            }
        }
    }
}

pub fn f221(v0: Adt14, v1: Bytes) -> bool {
    f51({
        let a1065 = {
            let a1064 = f58(v0, v1);
            length_bytes(a1064)
        };
        let a1066 = 0u64;
        nat_eq(a1065, a1066)
    })
}

pub fn f222(v0: Adt15, v1: Adt14) -> bool {
    if f328(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f328(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f223(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m1067 = v0;
    match m1067.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f198(v2)?;
            let a1069 = {
                let a1068 = v4.clone();
                length_bytes(a1068)
            };
            let a1070 = 0u64;
            if nat_eq(a1069, a1070) {
                f223(v3, v1)
            } else {
                let m1071 = f329(v4)?;
                match m1071 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f330(v5.clone())
                        } else {
                            f331(v5.clone())
                        } {
                            let m1072 = f223(v3, v1)?;
                            match m1072 {
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

pub fn f224(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m1073 = v0;
    match m1073.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f224(v3, v1))
        }
    }
}

pub fn f225(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1074 = v1;
    match m1074.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f240(v0.clone(), v2), f225(v0, v3))
            } else {
                f225(v0, v3)
            }
        }
    }
}

pub fn f226(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1075 = v1;
    match m1075.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f126({
                let a1077 = {
                    let a1076 = f198(v4.clone())?;
                    length_bytes(a1076)
                };
                let a1078 = 0u64;
                if nat_eq(a1077, a1078) {
                    List::<Bytes>::nil()
                } else {
                    f332(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f226(v0, v5, v2, v3)?))
        }
    }
}

pub fn f227(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f228() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f213(), List::<Bytes>::nil())))))))
}

pub fn f229(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1079 = v1;
    match m1079.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m1080 = f57(v0.clone(), v2.clone())?;
                match m1080 {
                    None => {
                        Ok(List::cons(v2, f229(v0, v3)?))
                    }
                    Some(_) => {
                        f229(v0, v3)
                    }
                }
            } else {
                f229(v0, v3)
            }
        }
    }
}

pub fn f230(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m1081 = v1;
    match m1081.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f139(v0.clone(), v2.clone()) {
                f230(v0, v3)
            } else {
                List::cons(v2, f230(v0, v3))
            }
        }
    }
}

pub fn f231() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f232(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m1082 = v0;
    match m1082.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f126(f333({
                let (h1083, _) = v2;
                h1083
            }, {
                let (_, h1084) = v2;
                h1084
            }, v1), f232(v3, v1))
        }
    }
}

pub fn f233() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f234(v0: Adt1) -> R<Option<Bytes>> {
    f334(f335(v0, f233())?)
}

pub fn f235(v0: Adt1, p1088: List<Bytes>) -> R<Option<Bytes>> {
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
                    Some((v2, v3)) => {
                        let m1086 = f336(f64(v0.clone(), v2.clone()), f337())?;
                        match m1086 {
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
                Ok(next) => { m1087 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: Adt17) -> R<(bool, Bytes)> {
    let v1: Adt16 = match v0.clone() {
        Adt17::C0(h1089, _, _, _, _) => {
            h1089
        }
    };
    let v2: Adt19 = f338(v1.clone())?;
    let v3: List<Bytes> = f339(v1.clone());
    let v4: List<Bytes> = f143(match v0.clone() {
        Adt17::C0(_, _, h1090, _, _) => {
            h1090
        }
    }, f8(Str::lit("path")));
    let m1092 = f340(v2, List::<u64>::nil(), f341(v1.clone(), match v0.clone() {
        Adt17::C0(_, h1091, _, _, _) => {
            h1091
        }
    }, 0u64)?);
    match m1092 {
        None => {
            if f343(f344(match v0.clone() {
                Adt17::C0(_, _, _, _, h1094) => {
                    h1094
                }
            }, f126(match v0.clone() {
                Adt17::C0(_, _, _, _, h1095) => {
                    h1095
                }
            }, v4.clone()))?, f344(v4.clone(), f126(match v0.clone() {
                Adt17::C0(_, _, _, _, h1096) => {
                    h1096
                }
            }, v4.clone()))?, f345(match v0.clone() {
                Adt17::C0(_, _, h1097, _, _) => {
                    h1097
                }
            })) {
                let m1101 = f346(f344(match v0.clone() {
                    Adt17::C0(_, _, _, h1098, _) => {
                        h1098
                    }
                }, f126(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h1099, _) => {
                        h1099
                    }
                }))?, f344(v3.clone(), f126(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h1100, _) => {
                        h1100
                    }
                }))?);
                match m1101 {
                    None => {
                        let m1104 = f347(f143(match v0 {
                            Adt17::C0(_, h1103, _, _, _) => {
                                h1103
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m1104 {
                            None => {
                                let m1105 = f347(v4, List::<Bytes>::nil());
                                match m1105 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f123(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f342(f126(v3, match v0 {
                            Adt17::C0(_, _, _, h1102, _) => {
                                h1102
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f342(match v1 {
                Adt16::C0(_, _, _, _, h1093) => {
                    h1093
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f237(v0: List<Bytes>) -> List<Bytes> {
    f121(Fn1::F348, List::<Bytes>::nil(), v0)
}

pub fn f238(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f349(v0, v1)
}

pub fn f239(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f350(f351(f352(v0))?))
}

pub fn f240(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f241(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1106 = f57(v0, v2)?;
    match m1106 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f353(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1107 = v4.clone();
            match m1107.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1109 = f93(f94({
                        let m1108 = f94(v3.clone(), f8(Str::lit("source")));
                        match m1108 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1109 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f354(f355(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f242(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1110 = v0;
    match m1110.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f356(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1114 = f44(f46(v1.clone(), 6u64, {
                    let a1112 = {
                        let a1111 = v1;
                        length_bytes(a1111)
                    };
                    let a1113 = 6u64;
                    nat_sub(a1112, a1113)
                }), 34u8)?;
                match m1114.uncons() {
                    None => {
                        f242(v2)
                    }
                    Some((v3, v4)) => {
                        let m1115 = v4;
                        match m1115.uncons() {
                            None => {
                                f242(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f242(v2)?))
                            }
                        }
                    }
                }
            } else {
                f242(v2)
            }
        }
    }
}

pub fn f243(p1118: List<Bytes>, p1119: List<Bytes>) -> List<Bytes> {
    let m1117 = (p1118, p1119);
    {
        let mut m1117 = m1117;
        loop {
            let step = {
                let (v0, v1) = m1117;
                let m1116 = v0;
                match m1116.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1117 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f245(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1120 = v1;
    match m1120.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f357(v0.clone(), v3)?;
            if f52(f358(f279(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f359(v5)?, f245(v0, v4, v2)?))
            } else {
                f245(v0, v4, v2)
            }
        }
    }
}

pub fn f246(v0: Adt1, p1124: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1123 = p1124;
    {
        let mut m1123 = m1123;
        loop {
            let step = {
                let v1 = m1123;
                let m1121 = v1;
                match m1121.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f357(v0.clone(), v3.clone())?;
                        if if f52(f358(f279(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f278({
                                let m1122 = f276(v5.clone(), f8(Str::lit("schema")));
                                match m1122 {
                                    None => {
                                        Adt2::C8
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f359(v5)?))
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
                Ok(next) => { m1123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f247(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f139(match v0.clone() {
        Adt1::C0(_, h1125, _, _) => {
            h1125
        }
    }, f260())) {
        Ok(Adt12::C1(List::cons(f260(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f152(f146(f360(f148(match v0.clone() {
            Adt1::C0(_, h1126, _, _) => {
                h1126
            }
        }, f260())))?, v1.clone());
        let m1127 = f56(v0.clone(), List::cons(f154(), List::cons(f213(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1127.uncons() {
            None => {
                let m1128 = f56(v0.clone(), f361(v0.clone(), v2.clone())?);
                match m1128.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f362(v0.clone(), v2, v1)?;
                        let (h1129, _) = v7;
                        if h1129 {
                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                let (_, h1130) = v7;
                                h1130
                            }))
                        } else {
                            let m1131 = f125(v0.clone());
                            match m1131.uncons() {
                                None => {
                                    let m1132 = f127(v0.clone())?;
                                    match m1132.uncons() {
                                        None => {
                                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                                let (_, h1133) = v7;
                                                h1133
                                            }, List::cons(f7(f146(f363(f128(v0.clone())?, f136(v0.clone(), f137(v0.clone())?)?))?), List::cons(f364(v0)?, List::<Bytes>::nil()))))))
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

pub fn f248(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f365(f97(v0, f154())?)?)))
}

pub fn f249() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f250(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1134 = v0;
    match m1134.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f251(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt20 = f366(v0.clone(), 0u64, {
        let a1135 = v0;
        length_bytes(a1135)
    }, Adt20::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt20 = {
        let a1138 = {
            let a1137 = match v1.clone() {
                Adt20::C0(_, _, h1136, _, _) => {
                    h1136
                }
            };
            length_bytes(a1137)
        };
        let a1139 = 0u64;
        if if nat_eq(a1138, a1139) {
            let m1141 = match v1.clone() {
                Adt20::C0(_, _, _, h1140, _) => {
                    h1140
                }
            };
            match m1141.uncons() {
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
            f367(v1)
        }
    };
    Ok(f368(match v4 {
        Adt20::C0(_, _, _, _, h1142) => {
            h1142
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f252(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt18>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m1143 = v1;
    match m1143.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f126(f369(v5.clone(), v2.clone()), f370(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f371(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1145 = {
                    let a1144 = v7.clone();
                    length_list(a1144)
                };
                let a1146 = 0u64;
                if nat_eq(a1145, a1146) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f123(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f252(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f253(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1147 = v0;
    match m1147.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1148 = v1;
            match m1148.uncons() {
                None => {
                    f253(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f253(v2))
                }
            }
        }
    }
}

pub fn f254(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1149 = v1;
    match m1149.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f221(v2.clone(), f8(Str::lit("domain"))) {
                f372(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1150 = f94(v2.clone(), f8(Str::lit("role")));
                    match m1150 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f373(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1151 = f94(v2.clone(), f8(Str::lit("origin")));
                    match m1151 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1152 = f94(v2, f8(Str::lit("evidence")));
                    match m1152 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f254(v0, v3))
            } else {
                f254(v0, v3)
            }
        }
    }
}

pub fn f255(v0: List<Adt14>) -> List<Bytes> {
    let m1153 = v0;
    match m1153.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f255(v2))
        }
    }
}

pub fn f256(v0: List<Adt14>) -> List<Bytes> {
    let m1154 = v0;
    match m1154.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f374(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f256(v2))
        }
    }
}

pub fn f257(p1160: List<Bytes>) -> R<bool> {
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
                            let a1156 = f198(v1)?;
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

pub fn f258(v0: List<Bytes>) -> R<bool> {
    if f267(f295(v0.clone())?) {
        Ok(f51({
            let a1162 = {
                let a1161 = f269(f295(v0)?);
                length_list(a1161)
            };
            let a1163 = 0u64;
            nat_eq(a1162, a1163)
        }))
    } else {
        Ok(false)
    }
}

pub fn f259(v0: List<Bytes>) -> R<bool> {
    if f267(f271(v0.clone(), f8(Str::lit("requirement")))?) {
        if f267(f271(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1165 = {
                    let a1164 = f224(f269(f271(v0.clone(), f8(Str::lit("requirement")))?), f269(f271(v0, f8(Str::lit("admission")))?));
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

pub fn f260() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f261(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f262() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f263(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1167 = v0;
    match m1167.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1169 = {
                let a1168 = v3.clone();
                length_bytes(a1168)
            };
            let a1170 = 0u64;
            if nat_eq(a1169, a1170) {
                f263(v2)
            } else {
                let m1171 = f329(v3.clone())?;
                match m1171 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f375(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1172 = f263(v2)?;
                            match m1172 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f376(v5), {
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

pub fn f264() -> Bytes {
    let a1175 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1175)
}

pub fn f265(p1180: List<Bytes>) -> R<Option<Bytes>> {
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
                        let m1177 = f329(f198(v1)?)?;
                        match m1177 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1178 = f94(v3.clone(), f8(Str::lit("source")));
                                match m1178 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f377(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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

pub fn f266(v0: Bytes) -> Bytes {
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

pub fn f267(v0: Option<List<Adt14>>) -> bool {
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

pub fn f268(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m1188 = v0;
    match m1188.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1190 = {
                let a1189 = v3.clone();
                length_bytes(a1189)
            };
            let a1191 = 0u64;
            if nat_eq(a1190, a1191) {
                f268(v2)
            } else {
                let m1192 = f329(v3)?;
                match m1192 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if f118(f118(f221(v4.clone(), f8(Str::lit("authority"))), f221(v4.clone(), f8(Str::lit("row")))), f118(f118(f221(v4.clone(), f8(Str::lit("construct"))), f221(v4.clone(), f8(Str::lit("artifact")))), f85(f58(v4.clone(), f8(Str::lit("reader"))), f8(Str::lit("read-"))))) {
                            let m1193 = f268(v2)?;
                            match m1193 {
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

pub fn f269(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1194 = v0;
    match m1194 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f270(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1195 = v0;
    match m1195.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1197 = {
                let a1196 = v3.clone();
                length_bytes(a1196)
            };
            let a1198 = 0u64;
            if nat_eq(a1197, a1198) {
                f270(v2)
            } else {
                let m1199 = f43(v3.clone())?;
                match m1199 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f378(v4.clone()) {
                            let m1200 = f270(v2)?;
                            match m1200 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f244(), List::cons(f8(Str::lit("/")), List::cons(f379(f279(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
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

pub fn f271(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m1203 = v0;
    match m1203.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f198(v2)?;
            let a1205 = {
                let a1204 = v4.clone();
                length_bytes(a1204)
            };
            let a1206 = 0u64;
            if nat_eq(a1205, a1206) {
                f271(v3, v1)
            } else {
                let m1207 = f329(v4)?;
                match m1207 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f380(v5.clone());
                        let a1209 = {
                            let a1208 = v6.clone();
                            length_bytes(a1208)
                        };
                        let a1210 = 0u64;
                        if nat_eq(a1209, a1210) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m1211 = f271(v3, v1.clone())?;
                            match m1211 {
                                None => {
                                    Ok(None::<List<Adt14>>)
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

pub fn f272(p1214: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1213 = p1214;
    {
        let mut m1213 = m1213;
        loop {
            let step = {
                let v0 = m1213;
                let m1212 = v0;
                match m1212.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f52(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f274(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f101(Adt14::C3(std::rc::Rc::new(f102(v1))))?))
}

pub fn f275(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f101(Adt14::C3(std::rc::Rc::new(f381(f146(f126(v1, f215(f97(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f276(p1220: Adt2, v1: Bytes) -> Option<Adt2> {
    let m1219 = p1220;
    {
        let mut m1219 = m1219;
        loop {
            let step = {
                let v0 = m1219;
                let m1215 = v0;
                match m1215 {
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
                    Adt2::C5(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C6(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C7(r1216) => {
                        let v5 = (*r1216).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C8 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C9(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(v2, r1217, r1218) => {
                        let v3 = (*r1217).clone();
                        let v4 = (*r1218).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1219 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1221 = f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m1221.uncons() {
        None => {
            let m1223 = f272(f132(List::cons(Adt14::C4(std::rc::Rc::new({
                let m1222 = f382(v4.clone());
                match m1222 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1223 {
                None => {
                    f383(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f273(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f278(v0: Adt2) -> R<Bytes> {
    let a1224 = f384(v0)?;
    let a1225 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a1224, a1225))
}

pub fn f279(v0: Adt2, v1: Bytes) -> Bytes {
    let m1226 = f385(f276(v0, v1));
    match m1226 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f280(v0: List<Adt2>) -> List<Bytes> {
    let m1227 = v0;
    match m1227.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1228 = f385(Some(v1));
            match m1228 {
                None => {
                    f280(v2)
                }
                Some(v3) => {
                    List::cons(v3, f280(v2))
                }
            }
        }
    }
}

pub fn f281(v0: Option<Adt2>) -> List<Adt2> {
    let m1229 = v0;
    match m1229 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f386(v1)
        }
    }
}

pub fn f282(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f283(v0: Bytes) -> R<Bytes> {
    Ok(f387(f388(v0)?))
}

pub fn f284(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: List<Bytes>, v5: Bytes) -> R<Adt12> {
    let v6: Bytes = {
        let a1230 = f8(Str::lit("registry-read "));
        let a1231 = v5;
        append_bytes(a1230, a1231)
    };
    if f51(f52(v6.clone(), match v0.clone() {
        Adt1::C0(_, _, h1232, _) => {
            h1232
        }
    })) {
        Ok(Adt12::C2(f389(), v4, v6))
    } else {
        let m1233 = f56(v0.clone(), List::cons(f311(), List::<Bytes>::nil()));
        match m1233.uncons() {
            None => {
                f390(v0, v1, f311(), v2, v3, v4)
            }
            Some((v7, v8)) => {
                Ok(Adt12::C0(List::cons(v7, v8)))
            }
        }
    }
}

pub fn f285() -> Bytes {
    f391()
}

pub fn f286(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f290(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f392(v1, v4)?)?), List::cons(f275(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f287(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1234 = v0;
    match m1234.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f288() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f289(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f354(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f290(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f393(f151(f96(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f394(f395(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f291(p1237: List<Adt14>) -> bool {
    let m1236 = p1237;
    {
        let mut m1236 = m1236;
        loop {
            let step = {
                let v0 = m1236;
                let m1235 = v0;
                match m1235.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f143(f396(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f292(v0: List<Bytes>) -> List<Bytes> {
    let m1238 = v0;
    match m1238.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f397(v1.clone()) {
                List::<Bytes>::nil()
            } else {
                List::cons(v1, f292(v2))
            }
        }
    }
}

pub fn f293(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>, v4: List<Bytes>) -> R<Adt12> {
    let m1239 = f382(v4);
    match m1239 {
        None => {
            Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m1240 = f272(f132(List::cons(Adt14::C4(std::rc::Rc::new(v5.clone())), List::<Adt14>::nil())), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1240 {
                None => {
                    f398(v0, v2.clone(), Adt14::C7(f8(Str::lit("translation")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("command")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f381(v3)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt14::C2(f283(v1.clone())?)), std::rc::Rc::new(Adt14::C5))))))), std::rc::Rc::new(v5)), f399(v2, f400(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f273(v6))
                }
            }
        }
    }
}

pub fn f294(p1243: List<Bytes>) -> List<Bytes> {
    let m1242 = p1243;
    {
        let mut m1242 = m1242;
        loop {
            let step = {
                let v0 = m1242;
                let m1241 = v0;
                match m1241.uncons() {
                    None => {
                        Err::<List<Bytes>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v1, v2)) => {
                        if f397(v1.clone()) {
                            Err::<List<Bytes>, List<Bytes>>(List::cons(v1, v2))
                        } else {
                            Ok::<List<Bytes>, List<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m1244 = v0;
    match m1244.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1246 = {
                let a1245 = v3.clone();
                length_bytes(a1245)
            };
            let a1247 = 0u64;
            if nat_eq(a1246, a1247) {
                f295(v2)
            } else {
                let m1248 = f329(v3)?;
                match m1248 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f221(v4.clone(), f8(Str::lit("agent"))) {
                            if f221(v4.clone(), f8(Str::lit("at"))) {
                                if f221(v4.clone(), f8(Str::lit("quote"))) {
                                    f401(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1249 = f295(v2)?;
                            match m1249 {
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

pub fn f296() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f297(p1252: List<Adt14>, v1: Bytes, v2: Bytes) -> bool {
    let m1251 = p1252;
    {
        let mut m1251 = m1251;
        loop {
            let step = {
                let v0 = m1251;
                let m1250 = v0;
                match m1250.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f118(f32(f58(v3.clone(), f8(Str::lit("observedProperty"))), v1.clone()), f32(f58(v3, f8(Str::lit("hasSimpleResult"))), v2.clone())) {
                            Err::<List<Adt14>, bool>(true)
                        } else {
                            Ok::<List<Adt14>, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1251 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt14 {
    Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt14::C2(v2)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt14::C2(v0)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5))))))))))))
}

pub fn f299(v0: u64) -> bool {
    let a1253 = v0;
    let a1254 = 127u64;
    nat_le(a1253, a1254)
}

pub fn f300(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1255, _) = v1.clone();
        h1255
    };
    let v3: u64 = {
        let (h1256, _) = {
            let (_, h1257) = v1.clone();
            h1257
        };
        h1256
    };
    let v4: List<Bytes> = {
        let (_, h1258) = {
            let (_, h1259) = v1;
            h1259
        };
        h1258
    };
    let m1260 = f208(v0.clone(), v2);
    match m1260 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1261 = v5;
            let a1262 = 10u8;
            if equal(a1261, a1262) {
                Ok(Some(({
                    let a1263 = v2;
                    let a1264 = 1u64;
                    nat_add(a1263, a1264)?
                }, ({
                    let a1265 = v2;
                    let a1266 = 1u64;
                    nat_add(a1265, a1266)?
                }, List::cons(f402(f46(v0, v3, {
                    let a1267 = v2;
                    let a1268 = v3;
                    nat_sub(a1267, a1268)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1269 = v2;
                    let a1270 = 1u64;
                    nat_add(a1269, a1270)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f301(v0: Adt14) -> List<Adt14> {
    let m1271 = v0;
    match m1271 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1272) => {
            let v3 = (*r1272).clone();
            f301(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1273, r1274) => {
            let v1 = (*r1273).clone();
            let v2 = (*r1274).clone();
            List::cons(v1, f301(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f302(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1275 = v0;
    match m1275 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r1276) => {
            let v10 = (*r1276).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r1277) => {
            let v5 = (*r1277).clone();
            Adt14::C4(std::rc::Rc::new(f302(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1278, r1279) => {
            let v11 = (*r1278).clone();
            let v12 = (*r1279).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r1280, r1281) => {
            let v3 = (*r1280).clone();
            let v4 = (*r1281).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt14::C7(v2, std::rc::Rc::new(f403(v3, v1.clone())), std::rc::Rc::new(f302(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f302(v4, v1)))
            }
        }
    }
}

pub fn f303(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1282 = 0u64;
        let a1283 = v1;
        nat_lt(a1282, a1283)
    } else {
        false
    } {
        let a1284 = Bytes::lit(&[0x2d]);
        let a1285 = f404(v1)?;
        Ok(append_bytes(a1284, a1285))
    } else {
        f404(v1)
    }
}

pub fn f304(v0: Bytes) -> R<Bytes> {
    let a1291 = {
        let a1289 = Bytes::lit(&[0x22]);
        let a1290 = {
            let (_, h1286) = {
                let (h1287, _) = f405(Fn5::F406(v0.clone()), {
                    let a1288 = v0;
                    length_bytes(a1288)
                }, (0u64, Bytes::lit(&[])))?;
                h1287
            };
            h1286
        };
        append_bytes(a1289, a1290)
    };
    let a1292 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1291, a1292))
}

pub fn f305(v0: List<Bytes>) -> Bytes {
    let a1295 = {
        let a1293 = Bytes::lit(&[0x5b]);
        let a1294 = f123(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1293, a1294)
    };
    let a1296 = Bytes::lit(&[0x5d]);
    append_bytes(a1295, a1296)
}

pub fn f306(v0: Adt14) -> R<List<Bytes>> {
    let m1297 = v0;
    match m1297 {
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
        Adt14::C6(r1298, r1299) => {
            let v7 = (*r1298).clone();
            let v8 = (*r1299).clone();
            Ok(List::cons(f190(v7)?, f306(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f307(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f407(Fn6::F408, List::<(Bytes, Bytes)>::nil(), f409(v0)?);
    let v2: List<Bytes> = f410(Fn7::F411, List::<Bytes>::nil(), v1)?;
    let a1302 = {
        let a1300 = Bytes::lit(&[0x7b]);
        let a1301 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1300, a1301)
    };
    let a1303 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1302, a1303))
}

pub fn f308(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1304 = v0;
    match m1304 {
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
        Adt14::C7(v9, r1305, r1306) => {
            let v10 = (*r1305).clone();
            let v11 = (*r1306).clone();
            Ok(List::cons((v9, f190(v10)?), f308(v11)?))
        }
    }
}

pub fn f309() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f310(v0: Adt13) -> Bytes {
    let m1307 = v0;
    match m1307 {
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

pub fn f311() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f312(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f412(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f412(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f412(f8(Str::lit("name")), f413(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f412(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f412(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f412(f8(Str::lit("statement")), f414(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f412(f8(Str::lit("media_type")), f415(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f412(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f412(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f412(f8(Str::lit("url")), f193(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f313(v0: Bytes) -> R<u64> {
    let (h1308, _) = f316(Fn3::F416(v0.clone()), {
        let a1309 = v0.clone();
        length_bytes(a1309)
    }, {
        let a1310 = v0;
        length_bytes(a1310)
    })?;
    Ok(h1308)
}

pub fn f314(v0: Bytes, v1: u64) -> R<u64> {
    let (h1311, _) = f316(Fn3::F417(v1, v0), v1, 0u64)?;
    Ok(h1311)
}

pub fn f315(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f116(v0.clone(), {
        let a1312 = v1;
        let a1313 = 1u64;
        nat_add(a1312, a1313)?
    })?;
    let v4: Bytes = {
        let a1314 = v3;
        let a1315 = 117u64;
        if nat_eq(a1314, a1315) {
            f418(v0, v1)?
        } else {
            f419(v3)
        }
    };
    let a1316 = v4.clone();
    let a1317 = Bytes::lit(&[]);
    if equal(a1316, a1317) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1318 = v3;
            let a1319 = 117u64;
            if nat_eq(a1318, a1319) {
                let a1320 = v1;
                let a1321 = 6u64;
                nat_add(a1320, a1321)?
            } else {
                let a1322 = v1;
                let a1323 = 2u64;
                nat_add(a1322, a1323)?
            }
        }, ({
            let a1324 = v2;
            let a1325 = v4;
            append_bytes(a1324, a1325)
        }, 0u64))))
    }
}

pub fn f316(v0: Fn3, p1330: u64, p1331: u64) -> R<(u64, bool)> {
    let m1329 = (p1330, p1331);
    {
        let mut m1329 = m1329;
        loop {
            let step = {
                let (v1, v2) = m1329;
                let m1326 = v1;
                if m1326 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1326 - 1;
                    let m1328 = {
                        let c1327: Fn3 = v0.clone();
                        c1327.apply(v2)?
                    };
                    match m1328 {
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
                Ok(next) => { m1329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f420(v0, v1)
}

pub fn f318(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f421(v0, v1)
}

pub fn f319(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1332 = v0.clone();
        length_bytes(a1332)
    };
    let a1333 = 0u64;
    let a1334 = v1;
    if nat_lt(a1333, a1334) {
        let a1335 = f422(v0.clone())?;
        let a1336 = v1;
        if nat_eq(a1335, a1336) {
            let a1337 = 1u64;
            let a1338 = v1;
            if nat_lt(a1337, a1338) {
                let m1341 = {
                    let a1339 = v0;
                    let a1340 = 0u64;
                    index_bytes(a1339, a1340)
                };
                match m1341 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1342 = v2;
                        let a1343 = 48u8;
                        Ok(!equal(a1342, a1343))
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

pub fn f320(v0: Fn4, p1348: u64, p1349: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1347 = (p1348, p1349);
    {
        let mut m1347 = m1347;
        loop {
            let step = {
                let (v1, v2) = m1347;
                let m1344 = v1;
                if m1344 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1344 - 1;
                    let m1346 = {
                        let c1345: Fn4 = v0.clone();
                        c1345.apply(v2)?
                    };
                    match m1346 {
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
                Ok(next) => { m1347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f321(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f423(v0, v1)
}

pub fn f322(v0: Bytes) -> R<u64> {
    let (h1350, _) = f316(Fn3::F424(v0.clone()), {
        let a1352 = {
            let a1351 = v0;
            length_bytes(a1351)
        };
        let a1353 = 1u64;
        nat_add(a1352, a1353)?
    }, 0u64)?;
    Ok(h1350)
}

pub fn f323(v0: u64) -> Bytes {
    let m1354 = v0;
    if m1354 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1354 - 1;
        let a1355 = f8(Str::lit("0"));
        let a1356 = f323(v1);
        append_bytes(a1355, a1356)
    }
}

pub fn f324(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1357 = v0;
    match m1357.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1358 = f94(v1, f8(Str::lit("std")));
            match m1358 {
                None => {
                    f324(v2)
                }
                Some(v3) => {
                    if f221(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f324(v2))
                    } else {
                        f324(v2)
                    }
                }
            }
        }
    }
}

pub fn f325(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1359 = v1;
    match m1359.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt15::C0(h1360, _, _) => {
                    h1360
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1361, _) => {
                        h1361
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f325(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt15::C0(h1362, _, _) => {
                        h1362
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1363, _) => {
                            h1363
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f325(v0, v3))
                } else {
                    f325(v0, v3)
                }
            }
        }
    }
}

pub fn f326(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt18>> {
    let m1364 = v0;
    match m1364.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v4, v5)) => {
            if if f425(v4.clone(), v2.clone()) {
                true
            } else {
                f425(v4.clone(), v3.clone())
            } {
                f326(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt18::C0({
                    let (h1365, _) = v4.clone();
                    h1365
                }, {
                    let a1368 = {
                        let a1367 = f146(f426({
                            let (h1366, _) = v4.clone();
                            h1366
                        }, v1.clone()))?;
                        length_list(a1367)
                    };
                    let a1369 = 1u64;
                    if nat_eq(a1368, a1369) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1370) = v4.clone();
                    h1370
                }), f326(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f327(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1371 = v1;
    match m1371.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1372, _) = v2.clone();
                h1372
            }, v0.clone()) {
                List::cons({
                    let (_, h1373) = v2;
                    h1373
                }, f327(v0, v3))
            } else {
                if f52({
                    let (_, h1374) = v2;
                    h1374
                }, v0.clone()) {
                    List::cons({
                        let (h1375, _) = v2;
                        h1375
                    }, f327(v0, v3))
                } else {
                    f327(v0, v3)
                }
            }
        }
    }
}

pub fn f328(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt15::C0(h1376, _, _) => {
            h1376
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1377, _) => {
                h1377
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt15::C0(_, _, h1378) => {
                    h1378
                }
            }) {
                true
            } else {
                if f221(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1379, _) => {
                            h1379
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f329(v0: Bytes) -> R<Option<Adt14>> {
    let m1380 = f43(v0)?;
    match m1380 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v1) => {
            Ok(f92(v1))
        }
    }
}

pub fn f330(v0: Adt14) -> bool {
    if f221(v0.clone(), f8(Str::lit("domain"))) {
        if f221(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f427()) {
                if f428(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f428(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f428(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f428(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f331(v0: Adt14) -> bool {
    if f221(v0.clone(), f8(Str::lit("subject"))) {
        if f221(v0.clone(), f8(Str::lit("object"))) {
            if f221(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f429())
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

pub fn f332(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1381 = f43(v1)?;
    match m1381 {
        None => {
            Ok(f227(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f280(f281(f276(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1382 = f385(f276(v4.clone(), f8(Str::lit("construct"))));
                match m1382 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1383 = v5;
            match m1383.uncons() {
                None => {
                    Ok(f227(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1384 = v9;
                    match m1384.uncons() {
                        None => {
                            Ok(f227(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f51(f32(v8, f8(Str::lit("std")))) {
                                Ok(f227(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f51(f52(v10.clone(), v2)) {
                                    Ok(f227(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f118(f51(f32(v7.clone(), f8(Str::lit("none")))), f51(f52(v7.clone(), v3))) {
                                        Ok(f227(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1385 = f385(f276(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1385 {
                                            None => {
                                                Ok(f227(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1386 = f276(v4, f8(Str::lit("value")));
                                                match m1386 {
                                                    None => {
                                                        Ok(f227(v0, f8(Str::lit("a row has no value"))))
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

pub fn f333(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1387 = v1;
    match m1387.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f430(v3.clone())
            } else {
                f51(f430(v3.clone()))
            } {
                List::cons(f240(v0.clone(), v3), f333(v0, v4, v2))
            } else {
                f333(v0, v4, v2)
            }
        }
    }
}

pub fn f334(p1390: List<Bytes>) -> R<Option<Bytes>> {
    let m1389 = p1390;
    {
        let mut m1389 = m1389;
        loop {
            let step = {
                let v0 = m1389;
                let m1388 = v0;
                match m1388.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f431(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f198(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1389 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1391 = v1;
    match m1391.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f126(f96(f64(v0.clone(), v2))?, f335(v0, v3)?))
        }
    }
}

pub fn f336(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1392 = v0.clone();
        length_bytes(a1392)
    };
    let v3: u64 = {
        let a1393 = v1.clone();
        length_bytes(a1393)
    };
    let v4: u64 = {
        let (h1394, _) = f316(Fn3::F432(v1.clone(), v3, v0.clone(), v2), {
            let a1395 = v2;
            let a1396 = 1u64;
            nat_add(a1395, a1396)?
        }, 0u64)?;
        h1394
    };
    if f433(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f337() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f338(v0: Adt16) -> R<Adt19> {
    Ok(Adt19::C0(f434(match v0.clone() {
        Adt16::C0(h1397, _, _, _, _) => {
            h1397
        }
    }, 0u64)?, f434(match v0.clone() {
        Adt16::C0(_, h1398, _, _, _) => {
            h1398
        }
    }, 0u64)?, f434(match v0.clone() {
        Adt16::C0(_, _, h1399, _, _) => {
            h1399
        }
    }, 0u64)?, f434(match v0 {
        Adt16::C0(_, _, _, h1400, _) => {
            h1400
        }
    }, 0u64)?))
}

pub fn f339(v0: Adt16) -> List<Bytes> {
    f126(match v0.clone() {
        Adt16::C0(h1401, _, _, _, _) => {
            h1401
        }
    }, f126(match v0.clone() {
        Adt16::C0(_, _, h1402, _, _) => {
            h1402
        }
    }, f126(match v0.clone() {
        Adt16::C0(_, _, _, h1403, _) => {
            h1403
        }
    }, match v0 {
        Adt16::C0(_, _, _, _, h1404) => {
            h1404
        }
    })))
}

pub fn f340(v0: Adt19, p1410: List<u64>, p1411: List<Adt21>) -> Option<u64> {
    let m1409 = (p1410, p1411);
    {
        let mut m1409 = m1409;
        loop {
            let step = {
                let (v1, v2) = m1409;
                let m1405 = v2;
                match m1405.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt21>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f435(v0.clone(), v1.clone(), match v3.clone() {
                            Adt21::C0(_, h1406) => {
                                h1406
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt21>), Option<u64>>((List::cons(match v3 {
                                Adt21::C0(h1407, _) => {
                                    h1407
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt21>), Option<u64>>(Some(match v3 {
                                Adt21::C0(h1408, _) => {
                                    h1408
                                }
                            }))
                        }
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

pub fn f341(v0: Adt16, v1: List<Adt14>, v2: u64) -> R<List<Adt21>> {
    let m1412 = v1;
    match m1412.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt21::C0(v2, f436(v0.clone(), v3)?), f341(v0, v4, {
                let a1413 = v2;
                let a1414 = 1u64;
                nat_add(a1413, a1414)?
            })?))
        }
    }
}

pub fn f342(p1418: List<Bytes>, p1419: u64) -> Bytes {
    let m1417 = (p1418, p1419);
    {
        let mut m1417 = m1417;
        loop {
            let step = {
                let (v0, v1) = m1417;
                let m1415 = v0;
                match m1415.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1416 = v1;
                        if m1416 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1416 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f437(v0.clone(), v1.clone()) {
        if f437(v1, v0) {
            f438(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f344(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1420 = v0;
    match m1420.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f439(v2, v1.clone())?, f344(v3, v1)?))
        }
    }
}

pub fn f345(v0: List<Adt14>) -> List<u64> {
    let m1421 = v0;
    match m1421.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f440(v1), f345(v2))
        }
    }
}

pub fn f346(p1424: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1423 = p1424;
    {
        let mut m1423 = m1423;
        loop {
            let step = {
                let v0 = m1423;
                let m1422 = v0;
                match m1422.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f441(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f347(p1427: List<Bytes>, p1428: List<Bytes>) -> Option<Bytes> {
    let m1426 = (p1427, p1428);
    {
        let mut m1426 = m1426;
        loop {
            let step = {
                let (v0, v1) = m1426;
                let m1425 = v0;
                match m1425.uncons() {
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
                Ok(next) => { m1426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f348(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f349(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1429 = v0.clone();
    match m1429.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1430 = v2;
            let a1431 = v1.clone();
            if equal(a1430, a1431) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f350(v0: List<(Bytes, ())>) -> List<Bytes> {
    f237(f442(Fn8::F443, List::<Bytes>::nil(), v0))
}

pub fn f351(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f444(f445(Fn9::F446, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1436 = {
        let (h1432, _) = f447(Fn10::F448, {
            let a1434 = {
                let a1433 = v0;
                length_list(a1433)
            };
            let a1435 = 1u64;
            nat_add(a1434, a1435)?
        }, v1)?;
        h1432
    };
    match m1436.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f352(v0: List<Bytes>) -> List<(Bytes, ())> {
    f449(f450(Fn11::F451, List::<(Bytes, ())>::nil(), v0))
}

pub fn f353(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1437 = f44(v1, 34u8)?;
    match m1437.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1438 = v3;
            match m1438.uncons() {
                None => {
                    Ok(f452(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f354(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f453(v0.clone(), 0u64, {
        let a1439 = v0;
        length_bytes(a1439)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f355(v0: Adt14, v1: Bytes) -> Bytes {
    if f32(f454(f94(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f356(v0: Bytes, v1: Bytes, p1452: u64, p1453: u64) -> R<bool> {
    let m1451 = (p1452, p1453);
    {
        let mut m1451 = m1451;
        loop {
            let step = {
                let (v2, v3) = m1451;
                let m1440 = v3;
                if m1440 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1440 - 1;
                    let m1443 = {
                        let a1441 = v1.clone();
                        let a1442 = v2;
                        index_bytes(a1441, a1442)
                    };
                    match m1443 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1446 = {
                                let a1444 = v0.clone();
                                let a1445 = v2;
                                index_bytes(a1444, a1445)
                            };
                            match m1446 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1447 = v5;
                                    let a1448 = v6;
                                    if equal(a1447, a1448) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1449 = v2;
                                            let a1450 = 1u64;
                                            nat_add(a1449, a1450)?
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
                Ok(next) => { m1451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f357(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1454 = f91(v0, v1)?;
    match m1454 {
        None => {
            Ok(Adt2::C8)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f358(v0: Bytes) -> R<Bytes> {
    let a1455 = f8(Str::lit("Catalog."));
    let a1456 = f379(v0)?;
    Ok(append_bytes(a1455, a1456))
}

pub fn f359(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f379(f279(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f360(v0: List<Bytes>) -> List<Bytes> {
    let m1457 = v0;
    match m1457.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f240(f260(), v1), f360(v2))
            } else {
                f360(v2)
            }
        }
    }
}

pub fn f361(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1458 = v1;
    match m1458.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f126(f455(f456(f96(f64(v0.clone(), v2))?)?), f361(v0, v3)?))
        }
    }
}

pub fn f362(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1459 = v1;
    match m1459.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f457(v0.clone(), v3, v2.clone())?;
            let (h1460, _) = v5.clone();
            if h1460 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f362(v0, v4, v2)?;
                let (h1461, _) = v6.clone();
                if h1461 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1464 = {
                            let (_, h1462) = v5;
                            h1462
                        };
                        let a1465 = {
                            let (_, h1463) = v6;
                            h1463
                        };
                        append_bytes(a1464, a1465)
                    }))
                }
            }
        }
    }
}

pub fn f363(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1466 = v1;
    match m1466.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f221(v2.clone(), f8(Str::lit("subject"))) {
                if f222(v0.clone(), v2.clone()) {
                    f363(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f363(v0, v3))
                }
            } else {
                if f372(v0.clone(), v2.clone()) {
                    f363(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f373(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f363(v0, v3))
                }
            }
        }
    }
}

pub fn f364(v0: Adt1) -> R<Bytes> {
    let a1468 = {
        let a1467 = f129(f130(f253(f250(f251(f64(v0.clone(), f213()))?)), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f132(f97(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1467)
    };
    let a1469 = 0u64;
    if nat_eq(a1468, a1469) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f365(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1470 = v0;
    match m1470.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f123(f215(f188(f94(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f458(f96(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f365(v2)?))
        }
    }
}

pub fn f366(v0: Bytes, p1528: u64, p1529: u64, p1530: Adt20) -> R<Adt20> {
    let m1527 = (p1528, (p1529, p1530));
    {
        let mut m1527 = m1527;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1527;
                let m1471 = v2;
                if m1471 == 0 {
                    Err::<(u64, (u64, Adt20)), R<Adt20>>(Ok(v3))
                } else {
                    let v4 = m1471 - 1;
                    let v5: u64 = f459(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt20::C0(_, h1472, _, _, _) => {
                            h1472
                        }
                    } {
                        Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                            let a1473 = v1;
                            let a1474 = 1u64;
                            nat_add(a1473, a1474)?
                        }, (v4, Adt20::C0(true, false, match v3.clone() {
                            Adt20::C0(_, _, h1475, _, _) => {
                                h1475
                            }
                        }, match v3.clone() {
                            Adt20::C0(_, _, _, h1476, _) => {
                                h1476
                            }
                        }, match v3 {
                            Adt20::C0(_, _, _, _, h1477) => {
                                h1477
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt20::C0(h1478, _, _, _, _) => {
                                h1478
                            }
                        } {
                            let a1479 = v5;
                            let a1480 = 34u64;
                            if nat_eq(a1479, a1480) {
                                let a1483 = f459(v0.clone(), {
                                    let a1481 = v1;
                                    let a1482 = 1u64;
                                    nat_add(a1481, a1482)?
                                })?;
                                let a1484 = 34u64;
                                if nat_eq(a1483, a1484) {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1485 = v1;
                                        let a1486 = 1u64;
                                        nat_add(a1485, a1486)?
                                    }, (v4, Adt20::C0(true, true, f460(match v3.clone() {
                                        Adt20::C0(_, _, h1487, _, _) => {
                                            h1487
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt20::C0(_, _, _, h1488, _) => {
                                            h1488
                                        }
                                    }, match v3 {
                                        Adt20::C0(_, _, _, _, h1489) => {
                                            h1489
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1490 = v1;
                                        let a1491 = 1u64;
                                        nat_add(a1490, a1491)?
                                    }, (v4, Adt20::C0(false, false, match v3.clone() {
                                        Adt20::C0(_, _, h1492, _, _) => {
                                            h1492
                                        }
                                    }, match v3.clone() {
                                        Adt20::C0(_, _, _, h1493, _) => {
                                            h1493
                                        }
                                    }, match v3 {
                                        Adt20::C0(_, _, _, _, h1494) => {
                                            h1494
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                    let a1495 = v1;
                                    let a1496 = 1u64;
                                    nat_add(a1495, a1496)?
                                }, (v4, Adt20::C0(true, false, f460(match v3.clone() {
                                    Adt20::C0(_, _, h1497, _, _) => {
                                        h1497
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt20::C0(_, _, _, h1498, _) => {
                                        h1498
                                    }
                                }, match v3 {
                                    Adt20::C0(_, _, _, _, h1499) => {
                                        h1499
                                    }
                                }))))
                            }
                        } else {
                            let a1500 = v5;
                            let a1501 = 34u64;
                            if nat_eq(a1500, a1501) {
                                Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                    let a1502 = v1;
                                    let a1503 = 1u64;
                                    nat_add(a1502, a1503)?
                                }, (v4, Adt20::C0(true, false, match v3.clone() {
                                    Adt20::C0(_, _, h1504, _, _) => {
                                        h1504
                                    }
                                }, match v3.clone() {
                                    Adt20::C0(_, _, _, h1505, _) => {
                                        h1505
                                    }
                                }, match v3 {
                                    Adt20::C0(_, _, _, _, h1506) => {
                                        h1506
                                    }
                                }))))
                            } else {
                                let a1507 = v5;
                                let a1508 = 44u64;
                                if nat_eq(a1507, a1508) {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1509 = v1;
                                        let a1510 = 1u64;
                                        nat_add(a1509, a1510)?
                                    }, (v4, Adt20::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt20::C0(_, _, h1511, _, _) => {
                                            h1511
                                        }
                                    }, match v3.clone() {
                                        Adt20::C0(_, _, _, h1512, _) => {
                                            h1512
                                        }
                                    }), match v3 {
                                        Adt20::C0(_, _, _, _, h1513) => {
                                            h1513
                                        }
                                    }))))
                                } else {
                                    let a1514 = v5;
                                    let a1515 = 13u64;
                                    if nat_eq(a1514, a1515) {
                                        Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                            let a1516 = v1;
                                            let a1517 = 1u64;
                                            nat_add(a1516, a1517)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1518 = v5;
                                        let a1519 = 10u64;
                                        if nat_eq(a1518, a1519) {
                                            Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                                let a1520 = v1;
                                                let a1521 = 1u64;
                                                nat_add(a1520, a1521)?
                                            }, (v4, f367(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                                let a1522 = v1;
                                                let a1523 = 1u64;
                                                nat_add(a1522, a1523)?
                                            }, (v4, Adt20::C0(false, false, f460(match v3.clone() {
                                                Adt20::C0(_, _, h1524, _, _) => {
                                                    h1524
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt20::C0(_, _, _, h1525, _) => {
                                                    h1525
                                                }
                                            }, match v3 {
                                                Adt20::C0(_, _, _, _, h1526) => {
                                                    h1526
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
                Ok(next) => { m1527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f367(v0: Adt20) -> Adt20 {
    Adt20::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f461(List::cons(match v0.clone() {
        Adt20::C0(_, _, h1531, _, _) => {
            h1531
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1532, _) => {
            h1532
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt20::C0(_, _, _, _, h1533) => {
            h1533
        }
    }))
}

pub fn f368(p1536: List<List<Bytes>>, p1537: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1535 = (p1536, p1537);
    {
        let mut m1535 = m1535;
        loop {
            let step = {
                let (v0, v1) = m1535;
                let m1534 = v0;
                match m1534.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(v0: Bytes, v1: List<Adt18>) -> List<Bytes> {
    let m1538 = v1;
    match m1538.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt18::C0(h1539, _, _) => {
                    h1539
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt18::C0(_, h1540, _) => {
                        h1540
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt18::C0(_, _, h1541) => {
                        h1541
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f369(v0, v3))
            } else {
                f369(v0, v3)
            }
        }
    }
}

pub fn f370(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1542 = v1;
    match m1542.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f370(v0, v3))
            } else {
                if f32(f58(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f58(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f370(v0, v3))
                } else {
                    f370(v0, v3)
                }
            }
        }
    }
}

pub fn f371(p1547: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1546 = p1547;
    {
        let mut m1546 = m1546;
        loop {
            let step = {
                let v0 = m1546;
                let m1543 = v0;
                match m1543.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1544 = v2;
                        match m1544.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1545 = v5;
                                    match m1545.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f462(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1546 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(v0: Adt15, v1: Adt14) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1548, _, _) => {
            h1548
        }
    }) {
        f52(f373(v1), match v0 {
            Adt15::C0(_, h1549, _) => {
                h1549
            }
        })
    } else {
        false
    }
}

pub fn f373(v0: Adt14) -> Bytes {
    let m1550 = f94(v0, f8(Str::lit("evidence")));
    match m1550 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f374(v0: Adt14) -> Bytes {
    if f221(v0.clone(), f8(Str::lit("req"))) {
        let a1551 = f8(Str::lit("req "));
        let a1552 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1551, a1552)
    } else {
        let m1553 = f94(v0.clone(), f8(Str::lit("std")));
        match m1553 {
            None => {
                let m1556 = f94(v0, f8(Str::lit("der")));
                match m1556 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f123(f215(f188(f94(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f221(v1.clone(), f8(Str::lit("row"))) {
                    let a1554 = f8(Str::lit(" row "));
                    let a1555 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1554, a1555)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f375(v0: Bytes, v1: Bytes) -> bool {
    let a1557 = v0;
    let a1558 = f391();
    if equal(a1557, a1558) {
        let a1562 = {
            let a1560 = {
                let a1559 = v1;
                length_bytes(a1559)
            };
            let a1561 = 0u64;
            nat_eq(a1560, a1561)
        };
        bool_not(a1562)
    } else {
        false
    }
}

pub fn f376(v0: Bytes) -> Bytes {
    f7(List::cons(f145(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f377(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1564 = v0;
    let a1565 = {
        let a1563 = Str::lit("redistributable");
        utf8_encode(a1563)
    };
    if equal(a1564, a1565) {
        f463(v1)
    } else {
        Ok(false)
    }
}

pub fn f378(v0: Adt2) -> bool {
    if f464(v0.clone(), f8(Str::lit("name"))) {
        if f465(v0.clone(), f8(Str::lit("schema"))) {
            let m1566 = f382(f280(f281(f276(v0, f8(Str::lit("citation"))))));
            match m1566 {
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

pub fn f379(v0: Bytes) -> R<Bytes> {
    f466(v0.clone(), 0u64, {
        let a1567 = v0;
        length_bytes(a1567)
    }, true, Bytes::lit(&[]))
}

pub fn f380(v0: Adt14) -> Bytes {
    if f221(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f221(v0.clone(), f8(Str::lit("id"))) {
            if f221(v0.clone(), f8(Str::lit("statement"))) {
                if f221(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f401(v0, f8(Str::lit("evidence")))
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

pub fn f381(v0: List<Bytes>) -> Adt14 {
    let m1568 = v0;
    match m1568.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f381(v2)))
        }
    }
}

pub fn f382(v0: List<Bytes>) -> Option<Adt14> {
    let m1569 = v0;
    match m1569.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1570 = v2;
                match m1570.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1571 = v2;
                    match m1571.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1572 = v6;
                                match m1572.uncons() {
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
                    if f32(v1, f8(Str::lit("der"))) {
                        let m1573 = v2;
                        match m1573.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1574 = v10;
                                match m1574.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f381(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f383(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1575 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1575.uncons() {
        None => {
            let m1576 = f382(v4);
            match m1576 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1577 = f8(Str::lit("Catalog."));
                        let a1578 = f379(v3.clone())?;
                        append_bytes(a1577, a1578)
                    };
                    let m1579 = f467(v2, v9, f379(v3.clone())?)?;
                    match m1579 {
                        Ok(v11) => {
                            f398(v0, v3.clone(), v8, v11, f8(Str::lit("Encoding entered through propose-encoding.")), List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f379(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::<(Bytes, Bytes)>::nil()))
                        }
                        Err(v10) => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v10, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                    }
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f384(v0: Adt2) -> R<Bytes> {
    let m1580 = v0;
    match m1580 {
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
            f303(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f468(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f469(v9)
        }
        Adt2::C6(r1581) => {
            let v10 = (*r1581).clone();
            Ok(f305(f470(v10)?))
        }
        Adt2::C7(r1582) => {
            let v11 = (*r1582).clone();
            f471(f472(v11)?)
        }
        Adt2::C8 => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C9(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C10(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f385(v0: Option<Adt2>) -> Option<Bytes> {
    let m1583 = v0;
    match m1583 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1584 = v1;
            match m1584 {
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
                Adt2::C5(v2) => {
                    Some(v2)
                }
                Adt2::C6(_) => {
                    None::<Bytes>
                }
                Adt2::C7(_) => {
                    None::<Bytes>
                }
                Adt2::C8 => {
                    None::<Bytes>
                }
                Adt2::C9(_, _) => {
                    None::<Bytes>
                }
                Adt2::C10(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f386(v0: Adt2) -> List<Adt2> {
    let m1585 = v0;
    match m1585 {
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
        Adt2::C5(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C6(r1586) => {
            let v3 = (*r1586).clone();
            f386(v3)
        }
        Adt2::C7(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C8 => {
            List::<Adt2>::nil()
        }
        Adt2::C9(r1587, r1588) => {
            let v1 = (*r1587).clone();
            let v2 = (*r1588).clone();
            List::cons(v1, f386(v2))
        }
        Adt2::C10(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f387(v0: Adt24) -> Bytes {
    let a1609 = f473(match v0.clone() {
        Adt24::C0(h1589, _, _, _, _, _, _, _) => {
            h1589
        }
    });
    let a1610 = {
        let a1607 = f473(match v0.clone() {
            Adt24::C0(_, h1590, _, _, _, _, _, _) => {
                h1590
            }
        });
        let a1608 = {
            let a1605 = f473(match v0.clone() {
                Adt24::C0(_, _, h1591, _, _, _, _, _) => {
                    h1591
                }
            });
            let a1606 = {
                let a1603 = f473(match v0.clone() {
                    Adt24::C0(_, _, _, h1592, _, _, _, _) => {
                        h1592
                    }
                });
                let a1604 = {
                    let a1601 = f473(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1593, _, _, _) => {
                            h1593
                        }
                    });
                    let a1602 = {
                        let a1599 = f473(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1594, _, _) => {
                                h1594
                            }
                        });
                        let a1600 = {
                            let a1597 = f473(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1595, _) => {
                                    h1595
                                }
                            });
                            let a1598 = f473(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1596) => {
                                    h1596
                                }
                            });
                            append_bytes(a1597, a1598)
                        };
                        append_bytes(a1599, a1600)
                    };
                    append_bytes(a1601, a1602)
                };
                append_bytes(a1603, a1604)
            };
            append_bytes(a1605, a1606)
        };
        append_bytes(a1607, a1608)
    };
    append_bytes(a1609, a1610)
}

pub fn f388(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1611 = v0.clone();
        length_bytes(a1611)
    };
    let v2: u64 = {
        let a1617 = {
            let a1614 = {
                let a1612 = v1;
                let a1613 = 72u64;
                nat_add(a1612, a1613)?
            };
            let a1615 = 64u64;
            let a1616 = 0u64;
            nat_quot(a1614, a1615, a1616)
        };
        let a1618 = 64u64;
        nat_mul(a1617, a1618)?
    };
    let (_, h1619) = f474(Fn12::F475(v0, v1, v2), {
        let a1620 = v2;
        let a1621 = 64u64;
        let a1622 = 0u64;
        nat_quot(a1620, a1621, a1622)
    }, (0u64, f476()))?;
    Ok(h1619)
}

pub fn f389() -> Bytes {
    f8(Str::lit("target/release/verdant_readers"))
}

pub fn f390(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>) -> R<Adt12> {
    let m1623 = f91(v0.clone(), v2.clone())?;
    match m1623 {
        None => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
        }
        Some(v6) => {
            if f32(v4.clone(), f8(Str::lit("none"))) {
                f477(v0, v1, v2, v3, v4, v5, v6)
            } else {
                let v7: Bytes = {
                    let a1624 = f8(Str::lit("Catalog."));
                    let a1625 = v4.clone();
                    append_bytes(a1624, a1625)
                };
                if f51(f478(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, v7.clone(), v3.clone())) {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v3, List::cons(f8(Str::lit(" in model/admissions.json, so it cannot check the entries\u{a}")), List::<Bytes>::nil()))))))))
                } else {
                    let m1626 = f91(v0.clone(), f7(List::cons(f8(Str::lit("encodings/")), List::cons(v4.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))?;
                    match m1626 {
                        None => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: encodings/")), List::cons(v4, List::cons(f8(Str::lit(".schema.json is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v8) => {
                            let m1627 = f479(v8, v6.clone())?;
                            match m1627 {
                                None => {
                                    f477(v0, v1, v2, v3, {
                                        let a1628 = f8(Str::lit("Catalog."));
                                        let a1629 = v4;
                                        append_bytes(a1628, a1629)
                                    }, v5, v6)
                                }
                                Some(v9) => {
                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" does not conform to ")), List::cons(v7, List::cons(f8(Str::lit(": ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f391() -> Bytes {
    let a1630 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1630)
}

pub fn f392(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f94(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f289(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f289(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f289(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f289(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f289(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f289(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f289(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f289(Str::lit("url"), f454(v2.clone(), f8(Str::lit("url"))))?, List::cons(f289(Str::lit("media_type"), f454(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f289(Str::lit("license"), f454(v2.clone(), f8(Str::lit("license"))))?, List::cons(f289(Str::lit("redistribution"), f454(v2, f8(Str::lit("redistribution"))))?, List::cons(f289(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1631 = v1;
        match m1631 {
            None => {
                f7(List::cons(f289(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f289(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f289(Str::lit("artifact"), {
                    let (h1632, _) = v3;
                    h1632
                })?, List::cons(f289(Str::lit("checksum"), {
                    let (_, h1633) = v3;
                    h1633
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f480(f215(f188(f94(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f393(p1636: List<List<Bytes>>, v1: Bytes, p1637: List<Bytes>) -> R<List<Bytes>> {
    let m1635 = (p1636, p1637);
    {
        let mut m1635 = m1635;
        loop {
            let step = {
                let (v0, v2) = m1635;
                let m1634 = v0;
                match m1634.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f481(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f395(f482(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f313(v0)?))
}

pub fn f395(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1638 = v0;
    match m1638.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1639 = v3.clone();
            match m1639.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1642 = v2;
                    let a1643 = {
                        let a1640 = v1.clone();
                        let a1641 = f395(v3, v1);
                        append_bytes(a1640, a1641)
                    };
                    append_bytes(a1642, a1643)
                }
            }
        }
    }
}

pub fn f396(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1644 = f94(v0, v1);
    match m1644 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f188(f94(v3, v2))
        }
    }
}

pub fn f397(v0: Bytes) -> bool {
    if f32(v0.clone(), f8(Str::lit("std"))) {
        true
    } else {
        if f32(v0.clone(), f8(Str::lit("req"))) {
            true
        } else {
            f32(v0, f8(Str::lit("der")))
        }
    }
}

pub fn f398(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a1645 = f8(Str::lit("Catalog."));
        let a1646 = f379(v1.clone())?;
        append_bytes(a1645, a1646)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f379(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f483(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f484(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f484(f97(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v7.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(v4)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a1650 = f485(f64(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v7.clone(), 4u64, {
            let a1648 = {
                let a1647 = v7;
                length_bytes(a1647)
            };
            let a1649 = 4u64;
            nat_sub(a1648, a1649)
        }))?;
        let a1651 = f8(Str::lit("\u{a}"));
        append_bytes(a1650, a1651)
    }), List::cons(f275(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f399(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f379(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f400(v0: Bytes) -> R<Bytes> {
    if f85(v0.clone(), f8(Str::lit("-- "))) {
        let m1652 = f336(v0.clone(), f8(Str::lit("\u{a}")))?;
        match m1652 {
            None => {
                Ok(v0)
            }
            Some(v1) => {
                Ok(f46(v0.clone(), {
                    let a1653 = v1;
                    let a1654 = 1u64;
                    nat_add(a1653, a1654)?
                }, {
                    let a1658 = {
                        let a1655 = v0;
                        length_bytes(a1655)
                    };
                    let a1659 = {
                        let a1656 = v1;
                        let a1657 = 1u64;
                        nat_add(a1656, a1657)?
                    };
                    nat_sub(a1658, a1659)
                }))
            }
        }
    } else {
        Ok(v0)
    }
}

pub fn f401(v0: Adt14, v1: Bytes) -> bool {
    let m1660 = f94(v0, v1);
    match m1660 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f402(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1662 = {
                let a1661 = v0;
                length_bytes(a1661)
            };
            let a1663 = 1u64;
            nat_sub(a1662, a1663)
        })
    } else {
        v0
    }
}

pub fn f403(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1664 = v0;
    match m1664 {
        Adt14::C0(v3) => {
            Adt14::C0(v3)
        }
        Adt14::C1(v4, v5) => {
            Adt14::C1(v4, v5)
        }
        Adt14::C2(v2) => {
            Adt14::C2(f486(f60(), v2, v1))
        }
        Adt14::C3(r1665) => {
            let v6 = (*r1665).clone();
            Adt14::C3(std::rc::Rc::new(v6))
        }
        Adt14::C4(r1666) => {
            let v7 = (*r1666).clone();
            Adt14::C4(std::rc::Rc::new(v7))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1667, r1668) => {
            let v8 = (*r1667).clone();
            let v9 = (*r1668).clone();
            Adt14::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt14::C7(v10, r1669, r1670) => {
            let v11 = (*r1669).clone();
            let v12 = (*r1670).clone();
            Adt14::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f404(v0: u64) -> R<Bytes> {
    f487(f488(), 10u64, v0)
}

pub fn f405(v0: Fn5, p1675: u64, p1676: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1674 = (p1675, p1676);
    {
        let mut m1674 = m1674;
        loop {
            let step = {
                let (v1, v2) = m1674;
                let m1671 = v1;
                if m1671 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1671 - 1;
                    let m1673 = {
                        let c1672: Fn5 = v0.clone();
                        c1672.apply(v2.clone())?
                    };
                    match m1673 {
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
                Ok(next) => { m1674 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f489(v0, v1)
}

pub fn f407(v0: Fn6, p1680: List<(Bytes, Bytes)>, p1681: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1679 = (p1680, p1681);
    {
        let mut m1679 = m1679;
        loop {
            let step = {
                let (v1, v2) = m1679;
                let m1677 = v2;
                match m1677.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1678: Fn6 = v0.clone();
                            c1678.apply(v1, v3)
                        }, v4))
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

pub fn f408(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f490(v0, v1)
}

pub fn f409(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f491(v0)
}

pub fn f410(v0: Fn7, p1685: List<Bytes>, p1686: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1684 = (p1685, p1686);
    {
        let mut m1684 = m1684;
        loop {
            let step = {
                let (v1, v2) = m1684;
                let m1682 = v2;
                match m1682.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1683: Fn7 = v0.clone();
                            c1683.apply(v1, v3)?
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

pub fn f411(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f492(v1)?, v0))
}

pub fn f412(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f304(v0)?, List::cons(f8(Str::lit(":")), List::cons(f304(v1)?, List::<Bytes>::nil())))))
}

pub fn f413(v0: Adt13) -> Bytes {
    let m1687 = v0;
    match m1687 {
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

pub fn f414(v0: Adt13) -> Bytes {
    let m1688 = v0;
    match m1688 {
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

pub fn f415(v0: Adt13) -> Bytes {
    let m1689 = v0;
    match m1689 {
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

pub fn f416(v0: Bytes, v1: u64) -> Option<u64> {
    let a1690 = 0u64;
    let a1691 = v1;
    if nat_lt(a1690, a1691) {
        if f493(v0, {
            let a1692 = v1;
            let a1693 = 1u64;
            nat_sub(a1692, a1693)
        }) {
            Some({
                let a1694 = v1;
                let a1695 = 1u64;
                nat_sub(a1694, a1695)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f417(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1696 = v2;
    let a1697 = v0;
    if nat_lt(a1696, a1697) {
        if f493(v1, v2) {
            Ok(Some({
                let a1698 = v2;
                let a1699 = 1u64;
                nat_add(a1698, a1699)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f418(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1702 = f46(v0.clone(), {
        let a1700 = v1;
        let a1701 = 2u64;
        nat_add(a1700, a1701)?
    }, 2u64);
    let a1703 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1702, a1703) {
        let m1706 = f494(f46(v0, {
            let a1704 = v1;
            let a1705 = 4u64;
            nat_add(a1704, a1705)?
        }, 2u64))?;
        match m1706 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f116(v2.clone(), 0u64)?;
                let a1707 = v3;
                let a1708 = 32u64;
                if nat_lt(a1707, a1708) {
                    if f495(v3) {
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

pub fn f419(v0: u64) -> Bytes {
    let a1709 = v0;
    let a1710 = 34u64;
    if nat_eq(a1709, a1710) {
        Bytes::lit(&[0x22])
    } else {
        let a1711 = v0;
        let a1712 = 92u64;
        if nat_eq(a1711, a1712) {
            Bytes::lit(&[0x5c])
        } else {
            let a1713 = v0;
            let a1714 = 98u64;
            if nat_eq(a1713, a1714) {
                Bytes::lit(&[0x08])
            } else {
                let a1715 = v0;
                let a1716 = 102u64;
                if nat_eq(a1715, a1716) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1717 = v0;
                    let a1718 = 110u64;
                    if nat_eq(a1717, a1718) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1719 = v0;
                        let a1720 = 114u64;
                        if nat_eq(a1719, a1720) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1721 = v0;
                            let a1722 = 116u64;
                            if nat_eq(a1721, a1722) {
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

pub fn f420(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f116(v0, v1)?;
    let a1723 = 47u64;
    let a1724 = v2;
    if if nat_lt(a1723, a1724) {
        let a1725 = v2;
        let a1726 = 58u64;
        nat_lt(a1725, a1726)
    } else {
        false
    } {
        Ok(Some({
            let a1727 = v1;
            let a1728 = 1u64;
            nat_add(a1727, a1728)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f421(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1729 = f116(v0, v1)?;
    let a1730 = 48u64;
    if nat_eq(a1729, a1730) {
        Ok(Some({
            let a1731 = v1;
            let a1732 = 1u64;
            nat_add(a1731, a1732)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f422(v0: Bytes) -> R<u64> {
    let (h1733, _) = f316(Fn3::F496(v0.clone()), {
        let a1734 = v0;
        length_bytes(a1734)
    }, 0u64)?;
    Ok(h1733)
}

pub fn f423(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1738 = {
        let a1736 = v0;
        let a1737 = {
            let (h1735, _) = v1;
            h1735
        };
        index_bytes(a1736, a1737)
    };
    match m1738 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1740 = {
                    let (h1739, _) = v1;
                    h1739
                };
                let a1741 = 1u64;
                nat_add(a1740, a1741)?
            }, {
                let a1745 = {
                    let a1743 = {
                        let (_, h1742) = v1;
                        h1742
                    };
                    let a1744 = 10u64;
                    nat_mul(a1743, a1744)?
                };
                let a1746 = f497(v2)?;
                nat_add(a1745, a1746)?
            })))
        }
    }
}

pub fn f424(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f498(v0, v1)
}

pub fn f425(v0: (Bytes, Bytes), p1749: List<(Bytes, Bytes)>) -> bool {
    let m1748 = p1749;
    {
        let mut m1748 = m1748;
        loop {
            let step = {
                let v1 = m1748;
                let m1747 = v1;
                match m1747.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f499(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1750 = v1;
    match m1750.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1751, _) = v2;
                h1751
            }, v0.clone()) {
                List::cons({
                    let (_, h1752) = v2;
                    h1752
                }, f426(v0, v3))
            } else {
                f426(v0, v3)
            }
        }
    }
}

pub fn f427() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f428(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1753 = f94(v0, v1);
    match m1753 {
        None => {
            false
        }
        Some(v3) => {
            f221(v3, v2)
        }
    }
}

pub fn f429() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f430(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f431(v0: Bytes) -> R<bool> {
    let v1: Bytes = f198(v0)?;
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
        let a1755 = {
            let a1754 = v1;
            length_bytes(a1754)
        };
        let a1756 = v2;
        nat_sub(a1755, a1756)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1757 = f336(v3.clone(), f8(Str::lit(")")))?;
        match m1757 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1758 = v4;
                    let a1759 = 1u64;
                    nat_add(a1758, a1759)?
                }, {
                    let a1763 = {
                        let a1760 = v3;
                        length_bytes(a1760)
                    };
                    let a1764 = {
                        let a1761 = v4;
                        let a1762 = 1u64;
                        nat_add(a1761, a1762)?
                    };
                    nat_sub(a1763, a1764)
                })
            }
        }
    } else {
        v3
    };
    let a1765 = 0u64;
    let a1766 = v2;
    if nat_lt(a1765, a1766) {
        f500(v5)
    } else {
        Ok(false)
    }
}

pub fn f432(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f433(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1769 = {
            let a1767 = v4;
            let a1768 = v1;
            nat_add(a1767, a1768)?
        };
        let a1770 = v3;
        if nat_lt(a1769, a1770) {
            Ok(Some({
                let a1771 = v4;
                let a1772 = 1u64;
                nat_add(a1771, a1772)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f433(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1777 = {
        let a1774 = v0;
        let a1775 = v2;
        let a1776 = {
            let a1773 = v1.clone();
            length_bytes(a1773)
        };
        slice_bytes(a1774, a1775, a1776)
    };
    match m1777 {
        None => {
            false
        }
        Some(v3) => {
            let a1778 = v3;
            let a1779 = v1;
            equal(a1778, a1779)
        }
    }
}

pub fn f434(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1780 = v0;
    match m1780.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f434(v3, {
                let a1781 = v1;
                let a1782 = 1u64;
                nat_add(a1781, a1782)?
            })?))
        }
    }
}

pub fn f435(v0: Adt19, v1: List<u64>, v2: Adt22) -> bool {
    let m1783 = v2;
    match m1783 {
        Adt22::C0(v3, v4) => {
            if f441(v3, match v0.clone() {
                Adt19::C0(h1784, _, _, _) => {
                    h1784
                }
            }) {
                f501(v0, v4)
            } else {
                false
            }
        }
        Adt22::C1(v5) => {
            f441(v5, match v0 {
                Adt19::C0(_, _, h1785, _) => {
                    h1785
                }
            })
        }
        Adt22::C2(_, v7, v8) => {
            if f437(v7, v1) {
                f441(v8, match v0 {
                    Adt19::C0(_, _, _, h1786) => {
                        h1786
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f436(v0: Adt16, v1: Adt14) -> R<Adt22> {
    let m1787 = f94(v1.clone(), f8(Str::lit("std")));
    match m1787 {
        None => {
            let m1792 = f93(f94(v1.clone(), f8(Str::lit("req"))));
            match m1792 {
                None => {
                    let m1794 = f94(v1, f8(Str::lit("der")));
                    match m1794 {
                        None => {
                            Ok(f502(v0))
                        }
                        Some(v5) => {
                            let m1795 = f503(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1795 {
                                None => {
                                    Ok(f502(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt22::C2(v6, f344(f215(f188(f94(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt16::C0(_, _, _, _, h1796) => {
                                            h1796
                                        }
                                    })?, f439(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt16::C0(_, _, _, h1797, _) => {
                                            h1797
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt22::C1(f439(v4, match v0 {
                        Adt16::C0(_, _, h1793, _, _) => {
                            h1793
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1788 = f93(f94(v2.clone(), f8(Str::lit("row"))));
            match m1788 {
                None => {
                    Ok(Adt22::C0(f439(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt16::C0(h1789, _, _, _, _) => {
                            h1789
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt22::C0(f439(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt16::C0(h1790, _, _, _, _) => {
                            h1790
                        }
                    })?, Some(f439(v3, match v0 {
                        Adt16::C0(_, h1791, _, _, _) => {
                            h1791
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f437(p1800: List<u64>, v1: List<u64>) -> bool {
    let m1799 = p1800;
    {
        let mut m1799 = m1799;
        loop {
            let step = {
                let v0 = m1799;
                let m1798 = v0;
                match m1798.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f441(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1799 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(p1805: List<u64>) -> bool {
    let m1804 = p1805;
    {
        let mut m1804 = m1804;
        loop {
            let step = {
                let v0 = m1804;
                let m1801 = v0;
                match m1801.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1802 = v1;
                        let a1803 = 7u64;
                        if nat_le(a1802, a1803) {
                            Ok::<List<u64>, bool>(v2)
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

pub fn f439(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1806 = f504(v0, v1.clone(), 0u64)?;
    match m1806 {
        None => {
            let a1807 = v1;
            Ok(length_list(a1807))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f440(v0: Adt14) -> u64 {
    let m1808 = f505(f94(v0, f8(Str::lit("stratum"))));
    match m1808 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f441(v0: u64, p1813: List<u64>) -> bool {
    let m1812 = p1813;
    {
        let mut m1812 = m1812;
        loop {
            let step = {
                let v1 = m1812;
                let m1809 = v1;
                match m1809.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1810 = v2;
                        let a1811 = v0;
                        if nat_eq(a1810, a1811) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: Fn8, p1817: List<Bytes>, p1818: List<(Bytes, ())>) -> List<Bytes> {
    let m1816 = (p1817, p1818);
    {
        let mut m1816 = m1816;
        loop {
            let step = {
                let (v1, v2) = m1816;
                let m1814 = v2;
                match m1814.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1815: Fn8 = v0.clone();
                            c1815.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1816 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1819, _) = v1;
        h1819
    }, v0)
}

pub fn f444(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f506(Fn13::F507, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f445(v0: Fn9, p1823: List<List<(Bytes, ())>>, p1824: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1822 = (p1823, p1824);
    {
        let mut m1822 = m1822;
        loop {
            let step = {
                let (v1, v2) = m1822;
                let m1820 = v2;
                match m1820.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1821: Fn9 = v0.clone();
                            c1821.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1822 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f447(v0: Fn10, p1829: u64, p1830: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1828 = (p1829, p1830);
    {
        let mut m1828 = m1828;
        loop {
            let step = {
                let (v1, v2) = m1828;
                let m1825 = v1;
                if m1825 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1825 - 1;
                    let m1827 = {
                        let c1826: Fn10 = v0.clone();
                        c1826.apply(v2.clone())?
                    };
                    match m1827 {
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
                Ok(next) => { m1828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f448(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f508(v0)
}

pub fn f449(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f509(Fn14::F510, List::<(Bytes, ())>::nil(), v0)
}

pub fn f450(v0: Fn11, p1834: List<(Bytes, ())>, p1835: List<Bytes>) -> List<(Bytes, ())> {
    let m1833 = (p1834, p1835);
    {
        let mut m1833 = m1833;
        loop {
            let step = {
                let (v1, v2) = m1833;
                let m1831 = v2;
                match m1831.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1832: Fn11 = v0.clone();
                            c1832.apply(v1, v3)
                        }, v4))
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

pub fn f451(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f452(p1840: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1839 = p1840;
    {
        let mut m1839 = m1839;
        loop {
            let step = {
                let v0 = m1839;
                let m1836 = v0;
                match m1836.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1837, _) = v2;
                            h1837
                        }) {
                            let (_, h1838) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1838)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1839 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: Bytes, p1866: u64, p1867: u64, p1868: Bytes) -> R<Bytes> {
    let m1865 = (p1866, (p1867, p1868));
    {
        let mut m1865 = m1865;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1865;
                let m1841 = v2;
                if m1841 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1841 - 1;
                    let m1844 = {
                        let a1842 = v0.clone();
                        let a1843 = v1;
                        index_bytes(a1842, a1843)
                    };
                    match m1844 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f203({
                                let a1845 = v6;
                                let a1846 = 34u64;
                                nat_eq(a1845, a1846)
                            }, {
                                let a1847 = v6;
                                let a1848 = 92u64;
                                nat_eq(a1847, a1848)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1849 = v1;
                                    let a1850 = 1u64;
                                    nat_add(a1849, a1850)?
                                }, (v4, {
                                    let a1853 = v3;
                                    let a1854 = {
                                        let a1851 = f8(Str::lit("\\"));
                                        let a1852 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1851, a1852)
                                    };
                                    append_bytes(a1853, a1854)
                                })))
                            } else {
                                let a1855 = v6;
                                let a1856 = 10u64;
                                if nat_eq(a1855, a1856) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1857 = v1;
                                        let a1858 = 1u64;
                                        nat_add(a1857, a1858)?
                                    }, (v4, {
                                        let a1859 = v3;
                                        let a1860 = f8(Str::lit("\\n"));
                                        append_bytes(a1859, a1860)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1861 = v1;
                                        let a1862 = 1u64;
                                        nat_add(a1861, a1862)?
                                    }, (v4, {
                                        let a1863 = v3;
                                        let a1864 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1863, a1864)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1869 = v0;
    match m1869 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f455(v0: List<Adt14>) -> List<Bytes> {
    let m1870 = v0;
    match m1870.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f511(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f282(f58(v1, f8(Str::lit("authority")))), f455(v2)))
        }
    }
}

pub fn f456(v0: List<Bytes>) -> R<List<Adt14>> {
    let m1871 = f295(v0.clone())?;
    match m1871 {
        None => {
            Ok(f269(f268(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt14>::nil())
        }
    }
}

pub fn f457(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
    let m1872 = f295(v3.clone())?;
    match m1872 {
        None => {
            let m1873 = f268(v3.clone())?;
            match m1873 {
                None => {
                    f514(v0, v1, v3)
                }
                Some(v5) => {
                    f513(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f512(v0, v1, v4)
        }
    }
}

pub fn f458(v0: List<Bytes>) -> List<Bytes> {
    let m1874 = v0;
    match m1874.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1876 = {
                    let a1875 = v1.clone();
                    length_bytes(a1875)
                };
                let a1877 = 0u64;
                if nat_eq(a1876, a1877) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f458(v2))
        }
    }
}

pub fn f459(v0: Bytes, v1: u64) -> R<u64> {
    let m1880 = {
        let a1878 = v0;
        let a1879 = v1;
        index_bytes(a1878, a1879)
    };
    match m1880 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f460(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1884 = {
        let a1881 = v1;
        let a1882 = v2;
        let a1883 = 1u64;
        slice_bytes(a1881, a1882, a1883)
    };
    match m1884 {
        None => {
            v0
        }
        Some(v3) => {
            let a1885 = v0;
            let a1886 = v3;
            append_bytes(a1885, a1886)
        }
    }
}

pub fn f461(p1889: List<Bytes>, p1890: List<Bytes>) -> List<Bytes> {
    let m1888 = (p1889, p1890);
    {
        let mut m1888 = m1888;
        loop {
            let step = {
                let (v0, v1) = m1888;
                let m1887 = v0;
                match m1887.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f462(v0: List<Bytes>) -> Bytes {
    let m1891 = v0;
    match m1891.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f463(v0: Bytes) -> R<bool> {
    let m1893 = {
        let a1892 = v0.clone();
        length_bytes(a1892)
    };
    if m1893 == 0 {
        Ok(true)
    } else {
        let v1 = m1893 - 1;
        let m1896 = {
            let a1894 = v0;
            let a1895 = v1;
            index_bytes(a1894, a1895)
        };
        match m1896 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f515({
                    let v3: u8 = v2;
                    let a1941 = {
                        let a1917 = {
                            let a1905 = {
                                let a1899 = {
                                    let a1897 = v3;
                                    let a1898 = 128u8;
                                    fixed_u8::bit_and(a1897, a1898)
                                };
                                let a1900 = 128u8;
                                if equal(a1899, a1900) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1906 = {
                                let a1903 = {
                                    let a1901 = v3;
                                    let a1902 = 64u8;
                                    fixed_u8::bit_and(a1901, a1902)
                                };
                                let a1904 = 64u8;
                                if equal(a1903, a1904) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1905, a1906)?
                        };
                        let a1918 = {
                            let a1915 = {
                                let a1909 = {
                                    let a1907 = v3;
                                    let a1908 = 32u8;
                                    fixed_u8::bit_and(a1907, a1908)
                                };
                                let a1910 = 32u8;
                                if equal(a1909, a1910) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1916 = {
                                let a1913 = {
                                    let a1911 = v3;
                                    let a1912 = 16u8;
                                    fixed_u8::bit_and(a1911, a1912)
                                };
                                let a1914 = 16u8;
                                if equal(a1913, a1914) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1915, a1916)?
                        };
                        nat_add(a1917, a1918)?
                    };
                    let a1942 = {
                        let a1939 = {
                            let a1927 = {
                                let a1921 = {
                                    let a1919 = v3;
                                    let a1920 = 8u8;
                                    fixed_u8::bit_and(a1919, a1920)
                                };
                                let a1922 = 8u8;
                                if equal(a1921, a1922) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1928 = {
                                let a1925 = {
                                    let a1923 = v3;
                                    let a1924 = 4u8;
                                    fixed_u8::bit_and(a1923, a1924)
                                };
                                let a1926 = 4u8;
                                if equal(a1925, a1926) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1927, a1928)?
                        };
                        let a1940 = {
                            let a1937 = {
                                let a1931 = {
                                    let a1929 = v3;
                                    let a1930 = 2u8;
                                    fixed_u8::bit_and(a1929, a1930)
                                };
                                let a1932 = 2u8;
                                if equal(a1931, a1932) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1938 = {
                                let a1935 = {
                                    let a1933 = v3;
                                    let a1934 = 1u8;
                                    fixed_u8::bit_and(a1933, a1934)
                                };
                                let a1936 = 1u8;
                                if equal(a1935, a1936) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1937, a1938)?
                        };
                        nat_add(a1939, a1940)?
                    };
                    nat_add(a1941, a1942)?
                }))
            }
        }
    }
}

pub fn f464(v0: Adt2, v1: Bytes) -> bool {
    let a1944 = {
        let a1943 = f279(v0, v1);
        length_bytes(a1943)
    };
    let a1945 = 0u64;
    !nat_eq(a1944, a1945)
}

pub fn f465(v0: Adt2, v1: Bytes) -> bool {
    let m1946 = f276(v0, v1);
    match m1946 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f466(v0: Bytes, p1967: u64, p1968: u64, p1969: bool, p1970: Bytes) -> R<Bytes> {
    let m1966 = (p1967, (p1968, (p1969, p1970)));
    {
        let mut m1966 = m1966;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1966;
                let m1947 = v2;
                if m1947 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1947 - 1;
                    let m1950 = {
                        let a1948 = v0.clone();
                        let a1949 = v1;
                        index_bytes(a1948, a1949)
                    };
                    match m1950 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f516(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1951 = v1;
                                    let a1952 = 1u64;
                                    nat_add(a1951, a1952)?
                                }, (v5, (true, v4))))
                            } else {
                                if f118(f517(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1953 = v1;
                                        let a1954 = 1u64;
                                        nat_add(a1953, a1954)?
                                    }, (v5, (false, {
                                        let a1958 = v4;
                                        let a1959 = f46({
                                            let a1955 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1955)
                                        }, {
                                            let a1956 = v7;
                                            let a1957 = 97u64;
                                            nat_sub(a1956, a1957)
                                        }, 1u64);
                                        append_bytes(a1958, a1959)
                                    }))))
                                } else {
                                    if f203(f517(v7), f203(f518(v7), f519(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1960 = v1;
                                            let a1961 = 1u64;
                                            nat_add(a1960, a1961)?
                                        }, (v5, (false, {
                                            let a1962 = v4;
                                            let a1963 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1962, a1963)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1964 = v1;
                                            let a1965 = 1u64;
                                            nat_add(a1964, a1965)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1966 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f520(f521(f276(v0.clone(), f8(Str::lit("$defs")))), f521(f276(v0.clone(), f8(Str::lit("definitions")))));
    let m1971 = f522(4096u64, v3.clone(), List::<Bytes>::nil(), f523(v3)?)?;
    match m1971 {
        Ok(v5) => {
            let m1972 = f524(4096u64, f525(f526(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1972 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f395(f243(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f468(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f404(v1)?;
    let v4: u64 = {
        let a1973 = v3.clone();
        length_bytes(a1973)
    };
    let v5: Bytes = {
        let a1974 = v2;
        let a1975 = v4;
        if nat_lt(a1974, a1975) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1976 = v4;
                let a1977 = v2;
                nat_sub(a1976, a1977)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1978 = v4;
                let a1979 = v2;
                nat_sub(a1978, a1979)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f323({
                let a1980 = v2;
                let a1981 = v4;
                nat_sub(a1980, a1981)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1982 = f8(Str::lit("-"));
        let a1983 = v5;
        Ok(append_bytes(a1982, a1983))
    } else {
        Ok(v5)
    }
}

pub fn f469(v0: Bytes) -> R<Bytes> {
    let a1988 = f200(v0.clone(), 0u64, {
        let a1985 = {
            let a1984 = v0.clone();
            length_bytes(a1984)
        };
        let a1986 = 1u64;
        nat_add(a1985, a1986)?
    })?;
    let a1989 = {
        let a1987 = v0.clone();
        length_bytes(a1987)
    };
    if nat_eq(a1988, a1989) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1995 = {
            let a1993 = Bytes::lit(&[0x22]);
            let a1994 = f527(v0.clone(), 0u64, {
                let a1991 = {
                    let a1990 = v0;
                    length_bytes(a1990)
                };
                let a1992 = 1u64;
                nat_add(a1991, a1992)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1993, a1994)
        };
        let a1996 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1995, a1996))
    }
}

pub fn f470(v0: Adt2) -> R<List<Bytes>> {
    let m1997 = v0;
    match m1997 {
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
        Adt2::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C8 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C9(r1998, r1999) => {
            let v12 = (*r1998).clone();
            let v13 = (*r1999).clone();
            Ok(List::cons(f384(v12)?, f470(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f471(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f407(Fn6::F528, List::<(Bytes, Bytes)>::nil(), f409(v0)?);
    let v2: List<Bytes> = f410(Fn7::F529, List::<Bytes>::nil(), v1)?;
    let a2002 = {
        let a2000 = Bytes::lit(&[0x7b]);
        let a2001 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a2000, a2001)
    };
    let a2003 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a2002, a2003))
}

pub fn f472(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m2004 = v0;
    match m2004 {
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
        Adt2::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C8 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C9(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C10(v14, r2005, r2006) => {
            let v15 = (*r2005).clone();
            let v16 = (*r2006).clone();
            Ok(List::cons((v14, f384(v15)?), f472(v16)?))
        }
    }
}

pub fn f473(v0: u32) -> Bytes {
    let a2035 = f530({
        let a2007 = f531(v0, 28u32);
        let a2008 = 15u32;
        fixed_u32::bit_and(a2007, a2008)
    });
    let a2036 = {
        let a2033 = f530({
            let a2009 = f531(v0, 24u32);
            let a2010 = 15u32;
            fixed_u32::bit_and(a2009, a2010)
        });
        let a2034 = {
            let a2031 = f530({
                let a2011 = f531(v0, 20u32);
                let a2012 = 15u32;
                fixed_u32::bit_and(a2011, a2012)
            });
            let a2032 = {
                let a2029 = f530({
                    let a2013 = f531(v0, 16u32);
                    let a2014 = 15u32;
                    fixed_u32::bit_and(a2013, a2014)
                });
                let a2030 = {
                    let a2027 = f530({
                        let a2015 = f531(v0, 12u32);
                        let a2016 = 15u32;
                        fixed_u32::bit_and(a2015, a2016)
                    });
                    let a2028 = {
                        let a2025 = f530({
                            let a2017 = f531(v0, 8u32);
                            let a2018 = 15u32;
                            fixed_u32::bit_and(a2017, a2018)
                        });
                        let a2026 = {
                            let a2023 = f530({
                                let a2019 = f531(v0, 4u32);
                                let a2020 = 15u32;
                                fixed_u32::bit_and(a2019, a2020)
                            });
                            let a2024 = f530({
                                let a2021 = v0;
                                let a2022 = 15u32;
                                fixed_u32::bit_and(a2021, a2022)
                            });
                            append_bytes(a2023, a2024)
                        };
                        append_bytes(a2025, a2026)
                    };
                    append_bytes(a2027, a2028)
                };
                append_bytes(a2029, a2030)
            };
            append_bytes(a2031, a2032)
        };
        append_bytes(a2033, a2034)
    };
    append_bytes(a2035, a2036)
}

pub fn f474(v0: Fn12, p2040: u64, p2041: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m2039 = (p2040, p2041);
    {
        let mut m2039 = m2039;
        loop {
            let step = {
                let (v1, v2) = m2039;
                let m2037 = v1;
                if m2037 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m2037 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c2038: Fn12 = v0.clone();
                        c2038.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2039 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a2043 = {
            let (h2042, _) = v3.clone();
            h2042
        };
        let a2044 = 64u64;
        nat_add(a2043, a2044)?
    }, f532({
        let (_, h2045) = v3;
        h2045
    }, f533(v0, v1, v2, {
        let (h2046, _) = v3;
        h2046
    })?)?))
}

pub fn f476() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f477(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Adt2) -> R<Adt12> {
    let m2047 = f534(f535(v1.clone(), v3.clone(), v4, v6))?;
    match m2047 {
        Ok(v8) => {
            let v9: Bytes = f511(v1.clone());
            Ok(Adt12::C3(List::cons((v9.clone(), v8), List::cons((f8(Str::lit("model/admissions.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f484(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("row")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5))))))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("translation")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("command")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f381(v5)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt14::C2(f283(f64(v0, v2))?)), std::rc::Rc::new(Adt14::C5))))))), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::<(Bytes, Bytes)>::nil())), f7(List::cons(f8(Str::lit("admitted ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        }
        Err(v7) => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(": ")), List::cons(v7, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
        }
    }
}

pub fn f478(p2051: List<Adt14>, v1: Bytes, v2: Bytes) -> bool {
    let m2050 = p2051;
    {
        let mut m2050 = m2050;
        loop {
            let step = {
                let v0 = m2050;
                let m2048 = v0;
                match m2048.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f118(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2049 = f94(v3, f8(Str::lit("std")));
                            match m2049 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    f32(f58(v5, f8(Str::lit("row"))), v2.clone())
                                }
                            }
                        }) {
                            Err::<List<Adt14>, bool>(true)
                        } else {
                            Ok::<List<Adt14>, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2050 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(v0: Adt2, v1: Adt2) -> R<Option<Bytes>> {
    f536({
        let a2058 = {
            let a2053 = {
                let a2052 = f537(v0.clone())?;
                length_bytes(a2052)
            };
            let a2054 = 2u64;
            nat_add(a2053, a2054)?
        };
        let a2059 = {
            let a2056 = {
                let a2055 = f537(v1.clone())?;
                length_bytes(a2055)
            };
            let a2057 = 2u64;
            nat_add(a2056, a2057)?
        };
        nat_mul(a2058, a2059)?
    }, v0.clone(), v0, v1, Bytes::lit(&[]))
}

pub fn f480(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f395(f538(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f481(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2060 = f44(v1, 34u8)?;
    match m2060.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2061 = v3;
            match m2061.uncons() {
                None => {
                    Ok(f539(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f482(v0: List<Bytes>) -> List<Bytes> {
    let m2062 = v0;
    match m2062.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f482(v2)
            } else {
                List::cons(v1, f482(v2))
            }
        }
    }
}

pub fn f483(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2063 = v0;
    match m2063.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f483(v3, v1))
        }
    }
}

pub fn f484(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m2064 = v0;
    match m2064.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f32(f58(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f540(v5, v1, v2))
            } else {
                List::cons(v4, f484(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f485(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f395(f541(f96(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f486(p2067: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2066 = p2067;
    {
        let mut m2066 = m2066;
        loop {
            let step = {
                let v0 = m2066;
                let m2065 = v0;
                match m2065.uncons() {
                    None => {
                        Err::<List<Adt13>, Bytes>(v1.clone())
                    }
                    Some((v3, v4)) => {
                        if f85(v1.clone(), f103(v3.clone())) {
                            Err::<List<Adt13>, Bytes>(f69(v3, v2.clone()))
                        } else {
                            Ok::<List<Adt13>, Bytes>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f487(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2068 = v2;
    let a2069 = 0u64;
    if nat_eq(a2068, a2069) {
        Ok(f542(v0, 0u64))
    } else {
        let (_, h2070) = {
            let (h2071, _) = f405(Fn5::F543(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2071
        };
        Ok(h2070)
    }
}

pub fn f488() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f489(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2075 = {
        let a2073 = v0.clone();
        let a2074 = {
            let (h2072, _) = v1.clone();
            h2072
        };
        index_bytes(a2073, a2074)
    };
    match m2075 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2077 = {
                    let (h2076, _) = v1.clone();
                    h2076
                };
                let a2078 = 1u64;
                nat_add(a2077, a2078)?
            }, {
                let a2081 = {
                    let (_, h2079) = v1;
                    h2079
                };
                let a2082 = f544(v0, {
                    let (h2080, _) = v1;
                    h2080
                }, v2)?;
                append_bytes(a2081, a2082)
            })))
        }
    }
}

pub fn f490(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2083 = v0.clone();
    match m2083.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2086 = {
                let (h2084, _) = v2;
                h2084
            };
            let a2087 = {
                let (h2085, _) = v1.clone();
                h2085
            };
            if equal(a2086, a2087) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f491(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f545(f546(Fn15::F547, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2092 = {
        let (h2088, _) = f548(Fn16::F549, {
            let a2090 = {
                let a2089 = v0;
                length_list(a2089)
            };
            let a2091 = 1u64;
            nat_add(a2090, a2091)?
        }, v1)?;
        h2088
    };
    match m2092.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f492(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2097 = {
        let a2094 = f304({
            let (h2093, _) = v0;
            h2093
        })?;
        let a2095 = Bytes::lit(&[0x3a]);
        append_bytes(a2094, a2095)
    };
    let a2098 = {
        let (_, h2096) = v0;
        h2096
    };
    Ok(append_bytes(a2097, a2098))
}

pub fn f493(v0: Bytes, v1: u64) -> bool {
    let m2099 = f208(v0, v1);
    match m2099 {
        None => {
            false
        }
        Some(v2) => {
            f550(v2)
        }
    }
}

pub fn f494(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2100 = v0.clone();
        length_bytes(a2100)
    };
    let a2104 = {
        let a2101 = v1;
        let a2102 = 2u64;
        let a2103 = 0u64;
        nat_rem(a2101, a2102, a2103)
    };
    let a2105 = 0u64;
    if nat_eq(a2104, a2105) {
        let (_, h2106) = {
            let (h2107, _) = f551(Fn17::F552(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2107
        };
        Ok(h2106)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f495(v0: u64) -> bool {
    let a2108 = v0;
    let a2109 = 8u64;
    if nat_eq(a2108, a2109) {
        true
    } else {
        let a2110 = v0;
        let a2111 = 9u64;
        if nat_eq(a2110, a2111) {
            true
        } else {
            let a2112 = v0;
            let a2113 = 10u64;
            if nat_eq(a2112, a2113) {
                true
            } else {
                let a2114 = v0;
                let a2115 = 12u64;
                if nat_eq(a2114, a2115) {
                    true
                } else {
                    let a2116 = v0;
                    let a2117 = 13u64;
                    nat_eq(a2116, a2117)
                }
            }
        }
    }
}

pub fn f496(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2120 = {
        let a2118 = v0;
        let a2119 = v1;
        index_bytes(a2118, a2119)
    };
    match m2120 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f553(v2)? {
                Ok(Some({
                    let a2121 = v1;
                    let a2122 = 1u64;
                    nat_add(a2121, a2122)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f497(v0: u8) -> R<u64> {
    let a2169 = {
        let v1: u8 = v0;
        let a2167 = {
            let a2143 = {
                let a2131 = {
                    let a2125 = {
                        let a2123 = v1;
                        let a2124 = 128u8;
                        fixed_u8::bit_and(a2123, a2124)
                    };
                    let a2126 = 128u8;
                    if equal(a2125, a2126) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2132 = {
                    let a2129 = {
                        let a2127 = v1;
                        let a2128 = 64u8;
                        fixed_u8::bit_and(a2127, a2128)
                    };
                    let a2130 = 64u8;
                    if equal(a2129, a2130) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2131, a2132)?
            };
            let a2144 = {
                let a2141 = {
                    let a2135 = {
                        let a2133 = v1;
                        let a2134 = 32u8;
                        fixed_u8::bit_and(a2133, a2134)
                    };
                    let a2136 = 32u8;
                    if equal(a2135, a2136) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2142 = {
                    let a2139 = {
                        let a2137 = v1;
                        let a2138 = 16u8;
                        fixed_u8::bit_and(a2137, a2138)
                    };
                    let a2140 = 16u8;
                    if equal(a2139, a2140) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2141, a2142)?
            };
            nat_add(a2143, a2144)?
        };
        let a2168 = {
            let a2165 = {
                let a2153 = {
                    let a2147 = {
                        let a2145 = v1;
                        let a2146 = 8u8;
                        fixed_u8::bit_and(a2145, a2146)
                    };
                    let a2148 = 8u8;
                    if equal(a2147, a2148) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2154 = {
                    let a2151 = {
                        let a2149 = v1;
                        let a2150 = 4u8;
                        fixed_u8::bit_and(a2149, a2150)
                    };
                    let a2152 = 4u8;
                    if equal(a2151, a2152) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2153, a2154)?
            };
            let a2166 = {
                let a2163 = {
                    let a2157 = {
                        let a2155 = v1;
                        let a2156 = 2u8;
                        fixed_u8::bit_and(a2155, a2156)
                    };
                    let a2158 = 2u8;
                    if equal(a2157, a2158) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2164 = {
                    let a2161 = {
                        let a2159 = v1;
                        let a2160 = 1u8;
                        fixed_u8::bit_and(a2159, a2160)
                    };
                    let a2162 = 1u8;
                    if equal(a2161, a2162) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2163, a2164)?
            };
            nat_add(a2165, a2166)?
        };
        nat_add(a2167, a2168)?
    };
    let a2170 = 48u64;
    Ok(nat_sub(a2169, a2170))
}

pub fn f498(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2171 = v0.clone();
        length_bytes(a2171)
    };
    let a2172 = v1;
    let a2173 = v2;
    if if nat_lt(a2172, a2173) {
        let a2178 = f116(v0, {
            let a2176 = v2;
            let a2177 = {
                let a2174 = v1;
                let a2175 = 1u64;
                nat_add(a2174, a2175)?
            };
            nat_sub(a2176, a2177)
        })?;
        let a2179 = 48u64;
        nat_eq(a2178, a2179)
    } else {
        false
    } {
        Ok(Some({
            let a2180 = v1;
            let a2181 = 1u64;
            nat_add(a2180, a2181)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f499(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f118(f32({
        let (h2182, _) = v0;
        h2182
    }, {
        let (h2183, _) = v1;
        h2183
    }), f32({
        let (_, h2184) = v0;
        h2184
    }, {
        let (_, h2185) = v1;
        h2185
    }))
}

pub fn f500(v0: Bytes) -> R<bool> {
    let m2186 = f336(v0.clone(), f8(Str::lit("&&")))?;
    if match m2186 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2187 = f336(v0, f8(Str::lit("||")))?;
        match m2187 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f501(v0: Adt19, v1: Option<u64>) -> bool {
    let m2188 = v1;
    match m2188 {
        None => {
            true
        }
        Some(v2) => {
            f441(v2, match v0 {
                Adt19::C0(_, h2189, _, _) => {
                    h2189
                }
            })
        }
    }
}

pub fn f502(v0: Adt16) -> Adt22 {
    Adt22::C1({
        let a2191 = match v0 {
            Adt16::C0(_, _, h2190, _, _) => {
                h2190
            }
        };
        length_list(a2191)
    })
}

pub fn f503(v0: Bytes) -> Option<Adt23> {
    if f32(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt23::C0)
    } else {
        if f32(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt23::C1)
        } else {
            if f32(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt23::C2)
            } else {
                None::<Adt23>
            }
        }
    }
}

pub fn f504(v0: Bytes, p2196: List<Bytes>, p2197: u64) -> R<Option<u64>> {
    let m2195 = (p2196, p2197);
    {
        let mut m2195 = m2195;
        loop {
            let step = {
                let (v1, v2) = m2195;
                let m2192 = v1;
                match m2192.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2193 = v2;
                                let a2194 = 1u64;
                                nat_add(a2193, a2194)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f505(v0: Option<Adt14>) -> Option<u64> {
    let m2198 = v0;
    match m2198 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2199 = v1;
            match m2199 {
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

pub fn f506(v0: Fn13, p2203: List<List<(Bytes, ())>>, p2204: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2202 = (p2203, p2204);
    {
        let mut m2202 = m2202;
        loop {
            let step = {
                let (v1, v2) = m2202;
                let m2200 = v2;
                match m2200.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2201: Fn13 = v0.clone();
                            c2201.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f508(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2206 = {
        let a2205 = v0.clone();
        length_list(a2205)
    };
    let a2207 = 1u64;
    if nat_le(a2206, a2207) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f554(v0)?))
    }
}

pub fn f509(v0: Fn14, p2211: List<(Bytes, ())>, p2212: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2210 = (p2211, p2212);
    {
        let mut m2210 = m2210;
        loop {
            let step = {
                let (v1, v2) = m2210;
                let m2208 = v2;
                match m2208.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2209: Fn14 = v0.clone();
                            c2209.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2210 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f510(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f511(v0: Bytes) -> Bytes {
    f7(List::cons(f140(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f512(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f143(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f143(f97(v0, f154())?, f8(Str::lit("at"))));
    let a2214 = {
        let a2213 = v4.clone();
        length_list(a2213)
    };
    let a2215 = 0u64;
    if nat_eq(a2214, a2215) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2218 = {
            let a2216 = v4;
            length_list(a2216)
        };
        let a2219 = {
            let a2217 = v3;
            length_list(a2217)
        };
        if nat_eq(a2218, a2219) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f555(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f513(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt14>) -> R<(bool, Bytes)> {
    let m2220 = v3;
    match m2220.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f203(f67(v0.clone(), f511(v6.clone())), f52(v6.clone(), v2.clone())) {
                f513(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f282(v6.clone()))) {
                    let v9: (bool, Bytes) = f513(v0, v1.clone(), v2, v5)?;
                    let (h2221, _) = v9.clone();
                    if h2221 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2223 = f555(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2224 = {
                                let (_, h2222) = v9;
                                h2222
                            };
                            append_bytes(a2223, a2224)
                        }))
                    }
                } else {
                    if f118(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f478(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2225 = f8(Str::lit("Catalog."));
                        let a2226 = v8.clone();
                        append_bytes(a2225, a2226)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f513(v0, v1.clone(), v2, v5)?;
                        let (h2227, _) = v10.clone();
                        if h2227 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2229 = f555(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2230 = {
                                    let (_, h2228) = v10;
                                    h2228
                                };
                                append_bytes(a2229, a2230)
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

pub fn f514(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f269(f271(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f269(f271(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f126(f143(v3.clone(), f8(Str::lit("id"))), f143(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f143(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f126(v6.clone(), f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2232 = {
        let a2231 = v7.clone();
        length_list(a2231)
    };
    let a2233 = 0u64;
    if nat_eq(a2232, a2233) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2236 = {
                let a2234 = v7;
                length_list(a2234)
            };
            let a2237 = {
                let a2235 = v5;
                length_list(a2235)
            };
            nat_eq(a2236, a2237)
        }) {
            Ok((false, f555(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f146(f126(f62(f556(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f557(v4), f126(v6, f143(v3, f8(Str::lit("id")))))))?;
            let a2239 = {
                let a2238 = v8.clone();
                length_list(a2238)
            };
            let a2240 = 0u64;
            if nat_eq(a2239, a2240) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f555(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f123(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f515(v0: u64) -> bool {
    let a2241 = v0;
    let a2242 = 47u64;
    nat_eq(a2241, a2242)
}

pub fn f516(v0: u64) -> bool {
    f203({
        let a2243 = v0;
        let a2244 = 95u64;
        nat_eq(a2243, a2244)
    }, f203({
        let a2245 = v0;
        let a2246 = 45u64;
        nat_eq(a2245, a2246)
    }, f203({
        let a2247 = v0;
        let a2248 = 32u64;
        nat_eq(a2247, a2248)
    }, f203({
        let a2249 = v0;
        let a2250 = 46u64;
        nat_eq(a2249, a2250)
    }, {
        let a2251 = v0;
        let a2252 = 47u64;
        nat_eq(a2251, a2252)
    }))))
}

pub fn f517(v0: u64) -> bool {
    f118({
        let a2253 = 97u64;
        let a2254 = v0;
        nat_le(a2253, a2254)
    }, {
        let a2255 = v0;
        let a2256 = 122u64;
        nat_le(a2255, a2256)
    })
}

pub fn f518(v0: u64) -> bool {
    f118({
        let a2257 = 65u64;
        let a2258 = v0;
        nat_le(a2257, a2258)
    }, {
        let a2259 = v0;
        let a2260 = 90u64;
        nat_le(a2259, a2260)
    })
}

pub fn f519(v0: u64) -> bool {
    f118({
        let a2261 = 48u64;
        let a2262 = v0;
        nat_le(a2261, a2262)
    }, {
        let a2263 = v0;
        let a2264 = 57u64;
        nat_le(a2263, a2264)
    })
}

pub fn f520(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m2265 = v0;
    match m2265.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f520(v3, v1))
        }
    }
}

pub fn f521(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m2266 = v0;
    match m2266 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f558(v1)
        }
    }
}

pub fn f522(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m2267 = v0;
    if m2267 == 0 {
        let m2268 = v1;
        match m2268.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2267 - 1;
        let m2269 = v1.clone();
        match m2269.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m2270 = f559(v1.clone(), v2.clone(), v3.clone())?;
                match m2270 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f395(f523(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2273 = f522(v6, f560(v1, {
                            let (h2271, _) = v9.clone();
                            h2271
                        }), List::cons(f561({
                            let (h2272, _) = v9.clone();
                            h2272
                        })?, v2), v3)?;
                        match m2273 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f523(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m2274 = v0;
    match m2274.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f561({
                let (h2275, _) = v1;
                h2275
            })?, f523(v2)?))
        }
    }
}

pub fn f524(p2281: u64, p2282: List<Adt25>, p2283: List<Bytes>, p2284: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2280 = (p2281, (p2282, (p2283, p2284)));
    {
        let mut m2280 = m2280;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2280;
                let m2276 = v0;
                if m2276 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2276 - 1;
                    let m2277 = v1;
                    match m2277.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2278 = v5;
                            match m2278 {
                                Adt25::C0(v8, v9) => {
                                    if f32(v8.clone(), f562()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f52(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2279 = f564(v8.clone(), v9)?;
                                            match m2279 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f525(f565(v11.clone()), List::cons(Adt25::C1(f566(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f52(f562(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f562(), v2), List::cons(f563(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2280 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2285 = v0;
    match m2285.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f525(v3, v1))
        }
    }
}

pub fn f526(v0: List<(Bytes, Adt2)>) -> R<List<Adt25>> {
    let m2286 = v0;
    match m2286.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f561({
                let (h2287, _) = v1;
                h2287
            })?, {
                let (_, h2288) = v1;
                h2288
            }), f526(v2)?))
        }
    }
}

pub fn f527(v0: Bytes, p2307: u64, p2308: u64, p2309: Bytes) -> R<Bytes> {
    let m2306 = (p2307, (p2308, p2309));
    {
        let mut m2306 = m2306;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2306;
                let m2289 = v2;
                if m2289 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2289 - 1;
                    let v5: u64 = f200(v0.clone(), v1, {
                        let a2293 = {
                            let a2291 = {
                                let a2290 = v0.clone();
                                length_bytes(a2290)
                            };
                            let a2292 = v1;
                            nat_sub(a2291, a2292)
                        };
                        let a2294 = 1u64;
                        nat_add(a2293, a2294)?
                    })?;
                    let v6: Bytes = {
                        let a2297 = v3;
                        let a2298 = f46(v0.clone(), v1, {
                            let a2295 = v5;
                            let a2296 = v1;
                            nat_sub(a2295, a2296)
                        });
                        append_bytes(a2297, a2298)
                    };
                    let m2301 = {
                        let a2299 = v0.clone();
                        let a2300 = v5;
                        index_bytes(a2299, a2300)
                    };
                    match m2301 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2302 = v5;
                                let a2303 = 1u64;
                                nat_add(a2302, a2303)?
                            }, (v4, {
                                let a2304 = v6;
                                let a2305 = f544(v0.clone(), v5, v7)?;
                                append_bytes(a2304, a2305)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f490(v0, v1)
}

pub fn f529(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f567(v1)?, v0))
}

pub fn f530(v0: u32) -> Bytes {
    let a2312 = {
        let a2310 = v0;
        let a2311 = 8u32;
        fixed_u32::bit_and(a2310, a2311)
    };
    let a2313 = 0u32;
    if equal(a2312, a2313) {
        let a2316 = {
            let a2314 = v0;
            let a2315 = 4u32;
            fixed_u32::bit_and(a2314, a2315)
        };
        let a2317 = 0u32;
        if equal(a2316, a2317) {
            let a2320 = {
                let a2318 = v0;
                let a2319 = 2u32;
                fixed_u32::bit_and(a2318, a2319)
            };
            let a2321 = 0u32;
            if equal(a2320, a2321) {
                let a2324 = {
                    let a2322 = v0;
                    let a2323 = 1u32;
                    fixed_u32::bit_and(a2322, a2323)
                };
                let a2325 = 0u32;
                if equal(a2324, a2325) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2328 = {
                    let a2326 = v0;
                    let a2327 = 1u32;
                    fixed_u32::bit_and(a2326, a2327)
                };
                let a2329 = 0u32;
                if equal(a2328, a2329) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2332 = {
                let a2330 = v0;
                let a2331 = 2u32;
                fixed_u32::bit_and(a2330, a2331)
            };
            let a2333 = 0u32;
            if equal(a2332, a2333) {
                let a2336 = {
                    let a2334 = v0;
                    let a2335 = 1u32;
                    fixed_u32::bit_and(a2334, a2335)
                };
                let a2337 = 0u32;
                if equal(a2336, a2337) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2340 = {
                    let a2338 = v0;
                    let a2339 = 1u32;
                    fixed_u32::bit_and(a2338, a2339)
                };
                let a2341 = 0u32;
                if equal(a2340, a2341) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2344 = {
            let a2342 = v0;
            let a2343 = 4u32;
            fixed_u32::bit_and(a2342, a2343)
        };
        let a2345 = 0u32;
        if equal(a2344, a2345) {
            let a2348 = {
                let a2346 = v0;
                let a2347 = 2u32;
                fixed_u32::bit_and(a2346, a2347)
            };
            let a2349 = 0u32;
            if equal(a2348, a2349) {
                let a2352 = {
                    let a2350 = v0;
                    let a2351 = 1u32;
                    fixed_u32::bit_and(a2350, a2351)
                };
                let a2353 = 0u32;
                if equal(a2352, a2353) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2356 = {
                    let a2354 = v0;
                    let a2355 = 1u32;
                    fixed_u32::bit_and(a2354, a2355)
                };
                let a2357 = 0u32;
                if equal(a2356, a2357) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2360 = {
                let a2358 = v0;
                let a2359 = 2u32;
                fixed_u32::bit_and(a2358, a2359)
            };
            let a2361 = 0u32;
            if equal(a2360, a2361) {
                let a2364 = {
                    let a2362 = v0;
                    let a2363 = 1u32;
                    fixed_u32::bit_and(a2362, a2363)
                };
                let a2365 = 0u32;
                if equal(a2364, a2365) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2368 = {
                    let a2366 = v0;
                    let a2367 = 1u32;
                    fixed_u32::bit_and(a2366, a2367)
                };
                let a2369 = 0u32;
                if equal(a2368, a2369) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f531(v0: u32, v1: u32) -> u32 {
    let m2372 = {
        let a2370 = v0;
        let a2371 = v1;
        fixed_u32::shift_right(a2370, a2371)
    };
    match m2372 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f532(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f568(Fn18::F569, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2373, _, _, _, _, _, _, _) => {
            h2373
        }
    }, match v0.clone() {
        Adt24::C0(_, h2374, _, _, _, _, _, _) => {
            h2374
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2375, _, _, _, _, _) => {
            h2375
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2376, _, _, _, _) => {
            h2376
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2377, _, _, _) => {
            h2377
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2378, _, _) => {
            h2378
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2379, _) => {
            h2379
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2380) => {
            h2380
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f570(match v0.clone() {
        Adt24::C0(h2381, _, _, _, _, _, _, _) => {
            h2381
        }
    }, match v2.clone() {
        Adt27::C0(h2382, _, _, _, _, _, _, _, _, _) => {
            h2382
        }
    }), f570(match v0.clone() {
        Adt24::C0(_, h2383, _, _, _, _, _, _) => {
            h2383
        }
    }, match v2.clone() {
        Adt27::C0(_, h2384, _, _, _, _, _, _, _, _) => {
            h2384
        }
    }), f570(match v0.clone() {
        Adt24::C0(_, _, h2385, _, _, _, _, _) => {
            h2385
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2386, _, _, _, _, _, _, _) => {
            h2386
        }
    }), f570(match v0.clone() {
        Adt24::C0(_, _, _, h2387, _, _, _, _) => {
            h2387
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2388, _, _, _, _, _, _) => {
            h2388
        }
    }), f570(match v0.clone() {
        Adt24::C0(_, _, _, _, h2389, _, _, _) => {
            h2389
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2390, _, _, _, _, _) => {
            h2390
        }
    }), f570(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2391, _, _) => {
            h2391
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2392, _, _, _, _) => {
            h2392
        }
    }), f570(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2393, _) => {
            h2393
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2394, _, _, _) => {
            h2394
        }
    }), f570(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2395) => {
            h2395
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2396, _, _) => {
            h2396
        }
    })))
}

pub fn f533(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f571(v0.clone(), v1, v2, v3)?, f571(v0.clone(), v1, v2, {
        let a2397 = v3;
        let a2398 = 4u64;
        nat_add(a2397, a2398)?
    })?, f571(v0.clone(), v1, v2, {
        let a2399 = v3;
        let a2400 = 8u64;
        nat_add(a2399, a2400)?
    })?, f571(v0.clone(), v1, v2, {
        let a2401 = v3;
        let a2402 = 12u64;
        nat_add(a2401, a2402)?
    })?, f571(v0.clone(), v1, v2, {
        let a2403 = v3;
        let a2404 = 16u64;
        nat_add(a2403, a2404)?
    })?, f571(v0.clone(), v1, v2, {
        let a2405 = v3;
        let a2406 = 20u64;
        nat_add(a2405, a2406)?
    })?, f571(v0.clone(), v1, v2, {
        let a2407 = v3;
        let a2408 = 24u64;
        nat_add(a2407, a2408)?
    })?, f571(v0.clone(), v1, v2, {
        let a2409 = v3;
        let a2410 = 28u64;
        nat_add(a2409, a2410)?
    })?, f571(v0.clone(), v1, v2, {
        let a2411 = v3;
        let a2412 = 32u64;
        nat_add(a2411, a2412)?
    })?, f571(v0.clone(), v1, v2, {
        let a2413 = v3;
        let a2414 = 36u64;
        nat_add(a2413, a2414)?
    })?, f571(v0.clone(), v1, v2, {
        let a2415 = v3;
        let a2416 = 40u64;
        nat_add(a2415, a2416)?
    })?, f571(v0.clone(), v1, v2, {
        let a2417 = v3;
        let a2418 = 44u64;
        nat_add(a2417, a2418)?
    })?, f571(v0.clone(), v1, v2, {
        let a2419 = v3;
        let a2420 = 48u64;
        nat_add(a2419, a2420)?
    })?, f571(v0.clone(), v1, v2, {
        let a2421 = v3;
        let a2422 = 52u64;
        nat_add(a2421, a2422)?
    })?, f571(v0.clone(), v1, v2, {
        let a2423 = v3;
        let a2424 = 56u64;
        nat_add(a2423, a2424)?
    })?, f571(v0, v1, v2, {
        let a2425 = v3;
        let a2426 = 60u64;
        nat_add(a2425, a2426)?
    })?))
}

pub fn f534(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m2427 = f572(v0.clone());
    match m2427 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f384(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f535(v0: Bytes, v1: Bytes, v2: Bytes, v3: Adt2) -> Adt2 {
    Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("citation")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f573(List::cons(f8(Str::lit("std")), List::cons(v0, List::cons(v1, List::<Bytes>::nil()))))))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C5(v2)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("pointer")), std::rc::Rc::new(Adt2::C5(Bytes::lit(&[]))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("value")), std::rc::Rc::new(v3), std::rc::Rc::new(Adt2::C8))))))))))
}

pub fn f536(v0: u64, v1: Adt2, v2: Adt2, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2428 = v0;
    if m2428 == 0 {
        Ok(Some(f7(List::cons(f8(Str::lit("the schema's references do not end before the instance at \"")), List::cons(v4, List::cons(f8(Str::lit("\" is checked (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
    } else {
        let v5 = m2428 - 1;
        let m2429 = v2.clone();
        match m2429 {
            Adt2::C0 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C1(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(None::<Bytes>)
                } else {
                    Ok(f574(v4, f8(Str::lit("is refused by the false schema (JSON Schema 2020-12 Core section 4.3.2)"))))
                }
            }
            Adt2::C3(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C4(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C5(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C6(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C7(r2443) => {
                let v7 = (*r2443).clone();
                let m2430 = f575(f558(v7));
                match m2430 {
                    None => {
                        Ok(f576(f577(v2.clone(), v3.clone(), v4.clone())?, f576({
                            let m2431 = f385(f276(v2.clone(), f8(Str::lit("$ref"))));
                            match m2431 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v9) => {
                                    let m2432 = f578(v1.clone(), v9)?;
                                    match m2432 {
                                        Ok(v11) => {
                                            f536(v5, v1.clone(), v11, v3.clone(), v4.clone())?
                                        }
                                        Err(v10) => {
                                            Some(v10)
                                        }
                                    }
                                }
                            }
                        }, f576(f579(v5, v1.clone(), f281(f276(v2.clone(), f8(Str::lit("allOf")))), v3.clone(), v4.clone())?, f576({
                            let m2433 = f276(v2.clone(), f8(Str::lit("anyOf")));
                            match m2433 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v12) => {
                                    let a2434 = 0u64;
                                    let a2435 = f580(v5, v1.clone(), f281(Some(v12)), v3.clone(), v4.clone())?;
                                    if nat_lt(a2434, a2435) {
                                        None::<Bytes>
                                    } else {
                                        f574(v4.clone(), f8(Str::lit("matches no anyOf branch (JSON Schema 2020-12 Core section 10.2.1.2)")))
                                    }
                                }
                            }
                        }, f576({
                            let m2436 = f276(v2.clone(), f8(Str::lit("oneOf")));
                            match m2436 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v13) => {
                                    let a2437 = f580(v5, v1.clone(), f281(Some(v13)), v3.clone(), v4.clone())?;
                                    let a2438 = 1u64;
                                    if nat_eq(a2437, a2438) {
                                        None::<Bytes>
                                    } else {
                                        f574(v4.clone(), f8(Str::lit("does not match exactly one oneOf branch (JSON Schema 2020-12 Core section 10.2.1.3)")))
                                    }
                                }
                            }
                        }, {
                            let m2439 = v3.clone();
                            match m2439 {
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
                                Adt2::C5(_) => {
                                    None::<Bytes>
                                }
                                Adt2::C6(r2441) => {
                                    let v15 = (*r2441).clone();
                                    let m2440 = f276(v2, f8(Str::lit("items")));
                                    match m2440 {
                                        None => {
                                            None::<Bytes>
                                        }
                                        Some(v16) => {
                                            f583(v5, v1, v16, f386(v15), v4, 0u64)?
                                        }
                                    }
                                }
                                Adt2::C7(r2442) => {
                                    let v14 = (*r2442).clone();
                                    f576(f581(v5, v1.clone(), f521(f276(v2.clone(), f8(Str::lit("properties")))), v3, v4.clone())?, f582(v5, v1, v2, f558(v14), v4)?)
                                }
                                Adt2::C8 => {
                                    None::<Bytes>
                                }
                                Adt2::C9(_, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C10(_, _, _) => {
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
            Adt2::C8 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C9(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C10(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
        }
    }
}

pub fn f537(v0: Adt2) -> R<Bytes> {
    f384(v0)
}

pub fn f538(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2444 = v0;
    match m2444.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f354(v1)?, f538(v2)?))
        }
    }
}

pub fn f539(p2447: List<Bytes>, v1: Bytes) -> bool {
    let m2446 = p2447;
    {
        let mut m2446 = m2446;
        loop {
            let step = {
                let v0 = m2446;
                let m2445 = v0;
                match m2445.uncons() {
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
                Ok(next) => { m2446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2448 = v0;
    match m2448.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f32(f58(v3.clone(), v1.clone()), v2.clone()) {
                f540(v4, v1, v2)
            } else {
                List::cons(v3, f540(v4, v1, v2))
            }
        }
    }
}

pub fn f541(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2449 = v0;
    match m2449.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f395(f538(f146(List::cons(v1, f584(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f541(v3, v1)?))
            }
        }
    }
}

pub fn f542(v0: Bytes, v1: u64) -> Bytes {
    let m2453 = {
        let a2450 = v0;
        let a2451 = v1;
        let a2452 = 1u64;
        slice_bytes(a2450, a2451, a2452)
    };
    match m2453 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f543(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f585(v1, v0, v2)
}

pub fn f544(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2498 = {
            let a2474 = {
                let a2462 = {
                    let a2456 = {
                        let a2454 = v3;
                        let a2455 = 128u8;
                        fixed_u8::bit_and(a2454, a2455)
                    };
                    let a2457 = 128u8;
                    if equal(a2456, a2457) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2463 = {
                    let a2460 = {
                        let a2458 = v3;
                        let a2459 = 64u8;
                        fixed_u8::bit_and(a2458, a2459)
                    };
                    let a2461 = 64u8;
                    if equal(a2460, a2461) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2462, a2463)?
            };
            let a2475 = {
                let a2472 = {
                    let a2466 = {
                        let a2464 = v3;
                        let a2465 = 32u8;
                        fixed_u8::bit_and(a2464, a2465)
                    };
                    let a2467 = 32u8;
                    if equal(a2466, a2467) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2473 = {
                    let a2470 = {
                        let a2468 = v3;
                        let a2469 = 16u8;
                        fixed_u8::bit_and(a2468, a2469)
                    };
                    let a2471 = 16u8;
                    if equal(a2470, a2471) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2472, a2473)?
            };
            nat_add(a2474, a2475)?
        };
        let a2499 = {
            let a2496 = {
                let a2484 = {
                    let a2478 = {
                        let a2476 = v3;
                        let a2477 = 8u8;
                        fixed_u8::bit_and(a2476, a2477)
                    };
                    let a2479 = 8u8;
                    if equal(a2478, a2479) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2485 = {
                    let a2482 = {
                        let a2480 = v3;
                        let a2481 = 4u8;
                        fixed_u8::bit_and(a2480, a2481)
                    };
                    let a2483 = 4u8;
                    if equal(a2482, a2483) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2484, a2485)?
            };
            let a2497 = {
                let a2494 = {
                    let a2488 = {
                        let a2486 = v3;
                        let a2487 = 2u8;
                        fixed_u8::bit_and(a2486, a2487)
                    };
                    let a2489 = 2u8;
                    if equal(a2488, a2489) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2495 = {
                    let a2492 = {
                        let a2490 = v3;
                        let a2491 = 1u8;
                        fixed_u8::bit_and(a2490, a2491)
                    };
                    let a2493 = 1u8;
                    if equal(a2492, a2493) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2494, a2495)?
            };
            nat_add(a2496, a2497)?
        };
        nat_add(a2498, a2499)?
    };
    let a2500 = v4;
    let a2501 = 34u64;
    if nat_eq(a2500, a2501) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2502 = v4;
        let a2503 = 92u64;
        if nat_eq(a2502, a2503) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2504 = v4;
            let a2505 = 8u64;
            if nat_eq(a2504, a2505) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2506 = v4;
                let a2507 = 12u64;
                if nat_eq(a2506, a2507) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2508 = v4;
                    let a2509 = 10u64;
                    if nat_eq(a2508, a2509) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2510 = v4;
                        let a2511 = 13u64;
                        if nat_eq(a2510, a2511) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2512 = v4;
                            let a2513 = 9u64;
                            if nat_eq(a2512, a2513) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2514 = v4;
                                let a2515 = 32u64;
                                if nat_lt(a2514, a2515) {
                                    let a2516 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2517 = f586(v2)?;
                                    Ok(append_bytes(a2516, a2517))
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

pub fn f545(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f587(Fn19::F588, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f546(v0: Fn15, p2521: List<List<(Bytes, Bytes)>>, p2522: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2520 = (p2521, p2522);
    {
        let mut m2520 = m2520;
        loop {
            let step = {
                let (v1, v2) = m2520;
                let m2518 = v2;
                match m2518.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2519: Fn15 = v0.clone();
                            c2519.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f547(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f548(v0: Fn16, p2527: u64, p2528: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2526 = (p2527, p2528);
    {
        let mut m2526 = m2526;
        loop {
            let step = {
                let (v1, v2) = m2526;
                let m2523 = v1;
                if m2523 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2523 - 1;
                    let m2525 = {
                        let c2524: Fn16 = v0.clone();
                        c2524.apply(v2.clone())?
                    };
                    match m2525 {
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
                Ok(next) => { m2526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f549(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f589(v0)
}

pub fn f550(v0: u8) -> bool {
    let a2529 = v0;
    let a2530 = 32u8;
    if if if equal(a2529, a2530) {
        true
    } else {
        let a2531 = v0;
        let a2532 = 9u8;
        equal(a2531, a2532)
    } {
        true
    } else {
        let a2533 = v0;
        let a2534 = 10u8;
        equal(a2533, a2534)
    } {
        true
    } else {
        let a2535 = v0;
        let a2536 = 12u8;
        if equal(a2535, a2536) {
            true
        } else {
            let a2537 = v0;
            let a2538 = 13u8;
            equal(a2537, a2538)
        }
    }
}

pub fn f551(v0: Fn17, p2543: u64, p2544: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2542 = (p2543, p2544);
    {
        let mut m2542 = m2542;
        loop {
            let step = {
                let (v1, v2) = m2542;
                let m2539 = v1;
                if m2539 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2539 - 1;
                    let m2541 = {
                        let c2540: Fn17 = v0.clone();
                        c2540.apply(v2.clone())?
                    };
                    match m2541 {
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
                Ok(next) => { m2542 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f552(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f590(v0, v1)
}

pub fn f553(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2589 = {
            let a2565 = {
                let a2553 = {
                    let a2547 = {
                        let a2545 = v1;
                        let a2546 = 128u8;
                        fixed_u8::bit_and(a2545, a2546)
                    };
                    let a2548 = 128u8;
                    if equal(a2547, a2548) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2554 = {
                    let a2551 = {
                        let a2549 = v1;
                        let a2550 = 64u8;
                        fixed_u8::bit_and(a2549, a2550)
                    };
                    let a2552 = 64u8;
                    if equal(a2551, a2552) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2553, a2554)?
            };
            let a2566 = {
                let a2563 = {
                    let a2557 = {
                        let a2555 = v1;
                        let a2556 = 32u8;
                        fixed_u8::bit_and(a2555, a2556)
                    };
                    let a2558 = 32u8;
                    if equal(a2557, a2558) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2564 = {
                    let a2561 = {
                        let a2559 = v1;
                        let a2560 = 16u8;
                        fixed_u8::bit_and(a2559, a2560)
                    };
                    let a2562 = 16u8;
                    if equal(a2561, a2562) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2563, a2564)?
            };
            nat_add(a2565, a2566)?
        };
        let a2590 = {
            let a2587 = {
                let a2575 = {
                    let a2569 = {
                        let a2567 = v1;
                        let a2568 = 8u8;
                        fixed_u8::bit_and(a2567, a2568)
                    };
                    let a2570 = 8u8;
                    if equal(a2569, a2570) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2576 = {
                    let a2573 = {
                        let a2571 = v1;
                        let a2572 = 4u8;
                        fixed_u8::bit_and(a2571, a2572)
                    };
                    let a2574 = 4u8;
                    if equal(a2573, a2574) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2575, a2576)?
            };
            let a2588 = {
                let a2585 = {
                    let a2579 = {
                        let a2577 = v1;
                        let a2578 = 2u8;
                        fixed_u8::bit_and(a2577, a2578)
                    };
                    let a2580 = 2u8;
                    if equal(a2579, a2580) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2586 = {
                    let a2583 = {
                        let a2581 = v1;
                        let a2582 = 1u8;
                        fixed_u8::bit_and(a2581, a2582)
                    };
                    let a2584 = 1u8;
                    if equal(a2583, a2584) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2585, a2586)?
            };
            nat_add(a2587, a2588)?
        };
        nat_add(a2589, a2590)?
    };
    let a2591 = 48u64;
    let a2592 = v2;
    if nat_le(a2591, a2592) {
        let a2593 = v2;
        let a2594 = 57u64;
        Ok(nat_le(a2593, a2594))
    } else {
        Ok(false)
    }
}

pub fn f554(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f444({
        let (_, h2595) = {
            let (h2596, _) = f591(Fn20::F592, {
                let a2598 = {
                    let a2597 = v0.clone();
                    length_list(a2597)
                };
                let a2599 = 1u64;
                nat_add(a2598, a2599)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2596
        };
        h2595
    }))
}

pub fn f555(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f556(v0: List<Adt14>) -> List<Bytes> {
    let m2600 = v0;
    match m2600.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2601 = f94(v1, f8(Str::lit("std")));
            match m2601 {
                None => {
                    f556(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f556(v2))
                }
            }
        }
    }
}

pub fn f557(v0: List<Adt14>) -> List<Bytes> {
    let m2602 = v0;
    match m2602.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2603 = f93(f94(v1, f8(Str::lit("req"))));
            match m2603 {
                None => {
                    f557(v2)
                }
                Some(v3) => {
                    List::cons(v3, f557(v2))
                }
            }
        }
    }
}

pub fn f558(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m2604 = v0;
    match m2604 {
        Adt2::C0 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C1(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C2(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C3(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C4(_, _, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C5(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C6(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C7(r2605) => {
            let v4 = (*r2605).clone();
            f558(v4)
        }
        Adt2::C8 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C9(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C10(v1, r2606, r2607) => {
            let v2 = (*r2606).clone();
            let v3 = (*r2607).clone();
            List::cons((v1, v2), f558(v3))
        }
    }
}

pub fn f559(p2612: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m2611 = p2612;
    {
        let mut m2611 = m2611;
        loop {
            let step = {
                let v0 = m2611;
                let m2608 = v0;
                match m2608.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f593(f594({
                            let (_, h2609) = v3.clone();
                            h2609
                        }, List::<Bytes>::nil())?, f561({
                            let (h2610, _) = v3.clone();
                            h2610
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2611 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f560(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m2613 = v0;
    match m2613.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h2614, _) = v2.clone();
                h2614
            }, v1.clone()) {
                f560(v3, v1)
            } else {
                List::cons(v2, f560(v3, v1))
            }
        }
    }
}

pub fn f561(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f379(v0)?;
    if f52(v1.clone(), f595()) {
        f596(v1)
    } else {
        Ok(v1)
    }
}

pub fn f562() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f563() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f564(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    f597(v0, f598(v1))
}

pub fn f565(v0: Adt28) -> List<Adt25> {
    let m2615 = v0;
    match m2615 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f566(v0: Adt28) -> Bytes {
    let m2616 = v0;
    match m2616 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f567(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2621 = {
        let a2618 = f469({
            let (h2617, _) = v0;
            h2617
        })?;
        let a2619 = Bytes::lit(&[0x3a]);
        append_bytes(a2618, a2619)
    };
    let a2622 = {
        let (_, h2620) = v0;
        h2620
    };
    Ok(append_bytes(a2621, a2622))
}

pub fn f568(v0: Fn18, p2626: u64, p2627: Adt27) -> R<Adt27> {
    let m2625 = (p2626, p2627);
    {
        let mut m2625 = m2625;
        loop {
            let step = {
                let (v1, v2) = m2625;
                let m2623 = v1;
                if m2623 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2623 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2624: Fn18 = v0.clone();
                        c2624.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f569(v0: Adt27) -> R<Adt27> {
    f599(v0)
}

pub fn f570(v0: u32, v1: u32) -> u32 {
    let m2630 = {
        let a2628 = f600(v0);
        let a2629 = f600(v1);
        fixed_u64::checked_add(a2628, a2629)
    };
    match m2630 {
        None => {
            0u32
        }
        Some(v2) => {
            f601(v2)
        }
    }
}

pub fn f571(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2633 = {
        let a2631 = v3;
        let a2632 = 8u64;
        nat_add(a2631, a2632)?
    };
    let a2634 = v2;
    if nat_eq(a2633, a2634) {
        Ok(f602({
            let a2637 = {
                let a2635 = v1;
                let a2636 = 8u64;
                nat_mul(a2635, a2636)?
            };
            let a2638 = 4294967296u64;
            let a2639 = 0u64;
            nat_quot(a2637, a2638, a2639)
        }))
    } else {
        let a2642 = {
            let a2640 = v3;
            let a2641 = 4u64;
            nat_add(a2640, a2641)?
        };
        let a2643 = v2;
        if nat_eq(a2642, a2643) {
            Ok(f602({
                let a2646 = {
                    let a2644 = v1;
                    let a2645 = 8u64;
                    nat_mul(a2644, a2645)?
                };
                let a2647 = 4294967296u64;
                let a2648 = 0u64;
                nat_rem(a2646, a2647, a2648)
            }))
        } else {
            Ok(f603(f604(v0.clone(), v1, v3), f604(v0.clone(), v1, {
                let a2649 = v3;
                let a2650 = 1u64;
                nat_add(a2649, a2650)?
            }), f604(v0.clone(), v1, {
                let a2651 = v3;
                let a2652 = 2u64;
                nat_add(a2651, a2652)?
            }), f604(v0, v1, {
                let a2653 = v3;
                let a2654 = 3u64;
                nat_add(a2653, a2654)?
            })))
        }
    }
}

pub fn f572(v0: Adt2) -> Option<Bytes> {
    let m2655 = v0;
    match m2655 {
        Adt2::C0 => {
            None::<Bytes>
        }
        Adt2::C1(v1, _) => {
            Some(v1)
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
        Adt2::C5(_) => {
            None::<Bytes>
        }
        Adt2::C6(r2658) => {
            let v3 = (*r2658).clone();
            f572(v3)
        }
        Adt2::C7(r2659) => {
            let v4 = (*r2659).clone();
            f572(v4)
        }
        Adt2::C8 => {
            None::<Bytes>
        }
        Adt2::C9(r2660, r2661) => {
            let v5 = (*r2660).clone();
            let v6 = (*r2661).clone();
            let m2656 = f572(v5);
            match m2656 {
                None => {
                    f572(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r2662, r2663) => {
            let v9 = (*r2662).clone();
            let v10 = (*r2663).clone();
            let m2657 = f572(v9);
            match m2657 {
                None => {
                    f572(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f573(v0: List<Bytes>) -> Adt2 {
    let m2664 = v0;
    match m2664.uncons() {
        None => {
            Adt2::C8
        }
        Some((v1, v2)) => {
            Adt2::C9(std::rc::Rc::new(Adt2::C5(v1)), std::rc::Rc::new(f573(v2)))
        }
    }
}

pub fn f574(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    Some(f7(List::cons(f8(Str::lit("the instance at \"")), List::cons(v0, List::cons(f8(Str::lit("\" ")), List::cons(v1, List::<Bytes>::nil()))))))
}

pub fn f575(p2669: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m2668 = p2669;
    {
        let mut m2668 = m2668;
        loop {
            let step = {
                let v0 = m2668;
                let m2665 = v0;
                match m2665.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h2666, _) = v1.clone();
                            h2666
                        }, f605()) {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h2667, _) = v1;
                                h2667
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2668 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f576(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2670 = v0;
    match m2670 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f577(v0: Adt2, v1: Adt2, v2: Bytes) -> R<Option<Bytes>> {
    let v3: Bytes = f606(v1.clone());
    let v7: Option<Bytes> = {
        let m2671 = f276(v0.clone(), f8(Str::lit("type")));
        match m2671 {
            None => {
                None::<Bytes>
            }
            Some(v4) => {
                let v6: List<Bytes> = {
                    let m2672 = f385(Some(v4.clone()));
                    match m2672 {
                        None => {
                            f280(f281(Some(v4)))
                        }
                        Some(v5) => {
                            List::cons(v5, List::<Bytes>::nil())
                        }
                    }
                };
                if f607(v6.clone(), v1.clone()) {
                    None::<Bytes>
                } else {
                    f574(v2.clone(), f7(List::cons(f8(Str::lit("is ")), List::cons(v3, List::cons(f8(Str::lit(", not of the type ")), List::cons(f395(v6, f8(Str::lit(" or "))), List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.1.1)")), List::<Bytes>::nil())))))))
                }
            }
        }
    };
    let v9: Option<Bytes> = {
        let m2673 = f276(v0.clone(), f8(Str::lit("enum")));
        match m2673 {
            None => {
                None::<Bytes>
            }
            Some(v8) => {
                if f608(f281(Some(v8)), v1.clone())? {
                    None::<Bytes>
                } else {
                    f574(v2.clone(), f8(Str::lit("is none of the enum values (JSON Schema 2020-12 Validation section 6.1.2)")))
                }
            }
        }
    };
    let v11: Option<Bytes> = {
        let m2674 = f276(v0.clone(), f8(Str::lit("const")));
        match m2674 {
            None => {
                None::<Bytes>
            }
            Some(v10) => {
                if f609(false, v10, v1.clone())? {
                    None::<Bytes>
                } else {
                    f574(v2.clone(), f8(Str::lit("is not the const value (JSON Schema 2020-12 Validation section 6.1.3)")))
                }
            }
        }
    };
    let v41: Option<Bytes> = {
        let m2675 = v1.clone();
        match m2675 {
            Adt2::C0 => {
                None::<Bytes>
            }
            Adt2::C1(v28, _) => {
                f574(v2.clone(), f7(List::cons(f8(Str::lit("is tagged ")), List::cons(v28, List::cons(f8(Str::lit(" and has no JSON type (RFC 8259 section 3)")), List::<Bytes>::nil())))))
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
            Adt2::C5(v12) => {
                let m2676 = f610(v12.clone())?;
                match m2676 {
                    None => {
                        f574(v2.clone(), f8(Str::lit("is not well-formed UTF-8 (RFC 3629 section 4)")))
                    }
                    Some(v13) => {
                        let v14: u64 = f611(v13)?;
                        f576({
                            let m2677 = f612(f276(v0.clone(), f8(Str::lit("minLength"))));
                            match m2677 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v15) => {
                                    let a2678 = v14;
                                    let a2679 = v15;
                                    if nat_lt(a2678, a2679) {
                                        f574(v2.clone(), f8(Str::lit("is shorter than minLength (JSON Schema 2020-12 Validation section 6.3.2)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, f576({
                            let m2680 = f612(f276(v0.clone(), f8(Str::lit("maxLength"))));
                            match m2680 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v16) => {
                                    let a2681 = v16;
                                    let a2682 = v14;
                                    if nat_lt(a2681, a2682) {
                                        f574(v2.clone(), f8(Str::lit("is longer than maxLength (JSON Schema 2020-12 Validation section 6.3.1)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, {
                            let m2683 = f385(f276(v0.clone(), f8(Str::lit("pattern"))));
                            match m2683 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v17) => {
                                    let m2684 = f613(v17.clone(), v12)?;
                                    match m2684 {
                                        Ok(v19) => {
                                            if v19 {
                                                None::<Bytes>
                                            } else {
                                                f574(v2.clone(), f7(List::cons(f8(Str::lit("does not match the pattern ")), List::cons(v17, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.3.3)")), List::<Bytes>::nil())))))
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
            Adt2::C6(r2692) => {
                let v20 = (*r2692).clone();
                let v21: u64 = f614(v20)?;
                f576({
                    let m2685 = f612(f276(v0.clone(), f8(Str::lit("minItems"))));
                    match m2685 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v22) => {
                            let a2686 = v21;
                            let a2687 = v22;
                            if nat_lt(a2686, a2687) {
                                f574(v2.clone(), f8(Str::lit("has fewer items than minItems (JSON Schema 2020-12 Validation section 6.4.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2688 = f612(f276(v0.clone(), f8(Str::lit("maxItems"))));
                    match m2688 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v23) => {
                            let a2689 = v23;
                            let a2690 = v21;
                            if nat_lt(a2689, a2690) {
                                f574(v2.clone(), f8(Str::lit("has more items than maxItems (JSON Schema 2020-12 Validation section 6.4.1)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
            Adt2::C7(r2693) => {
                let v24 = (*r2693).clone();
                let v25: List<Bytes> = f615(f280(f281(f276(v0.clone(), f8(Str::lit("required"))))), f616(f558(v24)));
                let m2691 = v25;
                match m2691.uncons() {
                    None => {
                        None::<Bytes>
                    }
                    Some((v26, _)) => {
                        f574(v2.clone(), f7(List::cons(f8(Str::lit("lacks the required member ")), List::cons(v26, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.5.3)")), List::<Bytes>::nil())))))
                    }
                }
            }
            Adt2::C8 => {
                None::<Bytes>
            }
            Adt2::C9(_, _) => {
                None::<Bytes>
            }
            Adt2::C10(_, _, _) => {
                None::<Bytes>
            }
        }
    };
    let v47: Option<Bytes> = {
        let m2694 = f617(v1);
        match m2694 {
            None => {
                None::<Bytes>
            }
            Some(v42) => {
                f576({
                    let m2696 = {
                        let m2695 = f276(v0.clone(), f8(Str::lit("minimum")));
                        match m2695 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v43) => {
                                f617(v43)
                            }
                        }
                    };
                    match m2696 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v44) => {
                            if f618(v42, v44)? {
                                f574(v2.clone(), f8(Str::lit("is less than minimum (JSON Schema 2020-12 Validation section 6.2.4)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2698 = {
                        let m2697 = f276(v0, f8(Str::lit("maximum")));
                        match m2697 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v45) => {
                                f617(v45)
                            }
                        }
                    };
                    match m2698 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v46) => {
                            if f618(v46, v42)? {
                                f574(v2, f8(Str::lit("is greater than maximum (JSON Schema 2020-12 Validation section 6.2.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
        }
    };
    Ok(f576(v7, f576(v9, f576(v11, f576(v41, v47)))))
}

pub fn f578(v0: Adt2, v1: Bytes) -> R<Result<Adt2, Bytes>> {
    let v2: u64 = {
        let a2699 = v1.clone();
        length_bytes(a2699)
    };
    if f32(v1.clone(), f8(Str::lit("#"))) {
        Ok(Ok::<Adt2, Bytes>(v0))
    } else {
        let m2704 = if f85(v1.clone(), f8(Str::lit("#/$defs/"))) {
            Some((f8(Str::lit("$defs")), f46(v1.clone(), 8u64, {
                let a2700 = v2;
                let a2701 = 8u64;
                nat_sub(a2700, a2701)
            })))
        } else {
            if f85(v1.clone(), f8(Str::lit("#/definitions/"))) {
                Some((f8(Str::lit("definitions")), f46(v1.clone(), 14u64, {
                    let a2702 = v2;
                    let a2703 = 14u64;
                    nat_sub(a2702, a2703)
                })))
            } else {
                None::<(Bytes, Bytes)>
            }
        };
        match m2704 {
            None => {
                Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" is not resolved: only #, #/$defs/NAME and #/definitions/NAME in the same document are (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
            }
            Some(v3) => {
                let v4: Bytes = f619({
                    let a2707 = {
                        let a2706 = {
                            let (_, h2705) = v3.clone();
                            h2705
                        };
                        length_bytes(a2706)
                    };
                    let a2708 = 1u64;
                    nat_add(a2707, a2708)?
                }, {
                    let (_, h2709) = v3;
                    h2709
                }, 0u64, Bytes::lit(&[]))?;
                let m2712 = {
                    let m2711 = f276(v0, {
                        let (h2710, _) = v3;
                        h2710
                    });
                    match m2711 {
                        None => {
                            None::<Adt2>
                        }
                        Some(v5) => {
                            f276(v5, v4)
                        }
                    }
                };
                match m2712 {
                    None => {
                        Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" names no definition of this schema (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok(Ok::<Adt2, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f579(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2713 = v0;
    if m2713 == 0 {
        Ok(Some(f8(Str::lit("the allOf branches do not end (JSON Schema 2020-12 Core section 10.2.1.1)"))))
    } else {
        let v5 = m2713 - 1;
        let m2714 = v2;
        match m2714.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f576(f536(v5, v1.clone(), v6, v3.clone(), v4.clone())?, f579(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f580(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<u64> {
    let m2715 = v0;
    if m2715 == 0 {
        Ok(0u64)
    } else {
        let v5 = m2715 - 1;
        let m2716 = v2;
        match m2716.uncons() {
            None => {
                Ok(0u64)
            }
            Some((v6, v7)) => {
                let a2718 = {
                    let m2717 = f536(v5, v1.clone(), v6, v3.clone(), v4.clone())?;
                    match m2717 {
                        None => {
                            1u64
                        }
                        Some(_) => {
                            0u64
                        }
                    }
                };
                let a2719 = f580(v5, v1, v7, v3, v4)?;
                nat_add(a2718, a2719)
            }
        }
    }
}

pub fn f581(v0: u64, v1: Adt2, v2: List<(Bytes, Adt2)>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2720 = v0;
    if m2720 == 0 {
        Ok(Some(f8(Str::lit("the properties do not end (JSON Schema 2020-12 Core section 10.3.2.1)"))))
    } else {
        let v5 = m2720 - 1;
        let m2721 = v2;
        match m2721.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f576({
                    let m2723 = f276(v3.clone(), {
                        let (h2722, _) = v6.clone();
                        h2722
                    });
                    match m2723 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v8) => {
                            f536(v5, v1.clone(), {
                                let (_, h2724) = v6;
                                h2724
                            }, v8, f620(v4.clone(), {
                                let (h2725, _) = v6;
                                h2725
                            })?)?
                        }
                    }
                }, f581(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f582(v0: u64, v1: Adt2, v2: Adt2, v3: List<(Bytes, Adt2)>, v4: Bytes) -> R<Option<Bytes>> {
    let m2726 = v0;
    if m2726 == 0 {
        Ok(Some(f8(Str::lit("the members do not end (JSON Schema 2020-12 Core section 10.3.2.2)"))))
    } else {
        let v5 = m2726 - 1;
        let m2727 = v3;
        match m2727.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                let v8: Bytes = f620(v4.clone(), {
                    let (h2728, _) = v6.clone();
                    h2728
                })?;
                let m2730 = f621(f521(f276(v2.clone(), f8(Str::lit("patternProperties")))), {
                    let (h2729, _) = v6.clone();
                    h2729
                })?;
                match m2730 {
                    Ok(v10) => {
                        Ok(f576(f579(v5, v1.clone(), v10.clone(), {
                            let (_, h2731) = v6.clone();
                            h2731
                        }, v8.clone())?, f576({
                            let m2732 = v10;
                            match m2732.uncons() {
                                None => {
                                    if f52({
                                        let (h2733, _) = v6;
                                        h2733
                                    }, f616(f558({
                                        let m2734 = f276(v2.clone(), f8(Str::lit("properties")));
                                        match m2734 {
                                            None => {
                                                Adt2::C7(std::rc::Rc::new(Adt2::C8))
                                            }
                                            Some(v13) => {
                                                v13
                                            }
                                        }
                                    }))) {
                                        None::<Bytes>
                                    } else {
                                        let m2735 = f276(v2.clone(), f8(Str::lit("additionalProperties")));
                                        match m2735 {
                                            None => {
                                                None::<Bytes>
                                            }
                                            Some(v14) => {
                                                f536(v5, v1.clone(), v14, {
                                                    let (_, h2736) = v6;
                                                    h2736
                                                }, v8)?
                                            }
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    None::<Bytes>
                                }
                            }
                        }, f582(v5, v1, v2, v7, v4)?)))
                    }
                    Err(v9) => {
                        Ok(Some(v9))
                    }
                }
            }
        }
    }
}

pub fn f583(v0: u64, v1: Adt2, v2: Adt2, v3: List<Adt2>, v4: Bytes, v5: u64) -> R<Option<Bytes>> {
    let m2737 = v0;
    if m2737 == 0 {
        Ok(Some(f8(Str::lit("the items do not end (JSON Schema 2020-12 Core section 10.3.1.2)"))))
    } else {
        let v6 = m2737 - 1;
        let m2738 = v3;
        match m2738.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v7, v8)) => {
                Ok(f576(f536(v6, v1.clone(), v2.clone(), v7, f620(v4.clone(), f404(v5)?)?)?, f583(v6, v1, v2, v8, v4, {
                    let a2739 = v5;
                    let a2740 = 1u64;
                    nat_add(a2739, a2740)?
                })?))
            }
        }
    }
}

pub fn f584(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2741 = v0;
    match m2741.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f584(v3, false))
            } else {
                f584(v3, true)
            }
        }
    }
}

pub fn f585(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2742, _) = v2;
        h2742
    };
    let a2743 = v3;
    let a2744 = 0u64;
    if nat_eq(a2743, a2744) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2745 = v3;
            let a2746 = v1;
            let a2747 = 0u64;
            nat_quot(a2745, a2746, a2747)
        }, {
            let a2752 = f542(v0, {
                let a2748 = v3;
                let a2749 = v1;
                let a2750 = 0u64;
                nat_rem(a2748, a2749, a2750)
            });
            let a2753 = {
                let (_, h2751) = v2;
                h2751
            };
            append_bytes(a2752, a2753)
        }))
    }
}

pub fn f586(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2798 = {
            let a2774 = {
                let a2762 = {
                    let a2756 = {
                        let a2754 = v1;
                        let a2755 = 128u8;
                        fixed_u8::bit_and(a2754, a2755)
                    };
                    let a2757 = 128u8;
                    if equal(a2756, a2757) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2763 = {
                    let a2760 = {
                        let a2758 = v1;
                        let a2759 = 64u8;
                        fixed_u8::bit_and(a2758, a2759)
                    };
                    let a2761 = 64u8;
                    if equal(a2760, a2761) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2762, a2763)?
            };
            let a2775 = {
                let a2772 = {
                    let a2766 = {
                        let a2764 = v1;
                        let a2765 = 32u8;
                        fixed_u8::bit_and(a2764, a2765)
                    };
                    let a2767 = 32u8;
                    if equal(a2766, a2767) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2773 = {
                    let a2770 = {
                        let a2768 = v1;
                        let a2769 = 16u8;
                        fixed_u8::bit_and(a2768, a2769)
                    };
                    let a2771 = 16u8;
                    if equal(a2770, a2771) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2772, a2773)?
            };
            nat_add(a2774, a2775)?
        };
        let a2799 = {
            let a2796 = {
                let a2784 = {
                    let a2778 = {
                        let a2776 = v1;
                        let a2777 = 8u8;
                        fixed_u8::bit_and(a2776, a2777)
                    };
                    let a2779 = 8u8;
                    if equal(a2778, a2779) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2785 = {
                    let a2782 = {
                        let a2780 = v1;
                        let a2781 = 4u8;
                        fixed_u8::bit_and(a2780, a2781)
                    };
                    let a2783 = 4u8;
                    if equal(a2782, a2783) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2784, a2785)?
            };
            let a2797 = {
                let a2794 = {
                    let a2788 = {
                        let a2786 = v1;
                        let a2787 = 2u8;
                        fixed_u8::bit_and(a2786, a2787)
                    };
                    let a2789 = 2u8;
                    if equal(a2788, a2789) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2795 = {
                    let a2792 = {
                        let a2790 = v1;
                        let a2791 = 1u8;
                        fixed_u8::bit_and(a2790, a2791)
                    };
                    let a2793 = 1u8;
                    if equal(a2792, a2793) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2794, a2795)?
            };
            nat_add(a2796, a2797)?
        };
        nat_add(a2798, a2799)?
    };
    let a2806 = f542(f622(), {
        let a2800 = v2;
        let a2801 = 16u64;
        let a2802 = 0u64;
        nat_quot(a2800, a2801, a2802)
    });
    let a2807 = f542(f622(), {
        let a2803 = v2;
        let a2804 = 16u64;
        let a2805 = 0u64;
        nat_rem(a2803, a2804, a2805)
    });
    Ok(append_bytes(a2806, a2807))
}

pub fn f587(v0: Fn19, p2811: List<List<(Bytes, Bytes)>>, p2812: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2810 = (p2811, p2812);
    {
        let mut m2810 = m2810;
        loop {
            let step = {
                let (v1, v2) = m2810;
                let m2808 = v2;
                match m2808.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2809: Fn19 = v0.clone();
                            c2809.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2810 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f589(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2814 = {
        let a2813 = v0.clone();
        length_list(a2813)
    };
    let a2815 = 1u64;
    if nat_le(a2814, a2815) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f623(v0)?))
    }
}

pub fn f590(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2816, _) = v1;
        h2816
    };
    let m2818 = {
        let (_, h2817) = v1;
        h2817
    };
    match m2818 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2821 = {
                let a2819 = v0.clone();
                let a2820 = v2;
                index_bytes(a2819, a2820)
            };
            match m2821 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2826 = {
                        let a2824 = v0;
                        let a2825 = {
                            let a2822 = v2;
                            let a2823 = 1u64;
                            nat_add(a2822, a2823)?
                        };
                        index_bytes(a2824, a2825)
                    };
                    match m2826 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f624(v4)? {
                                f624(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2827 = v2;
                                    let a2828 = 2u64;
                                    nat_add(a2827, a2828)?
                                }, Some({
                                    let a2833 = v3;
                                    let a2834 = f542(f625(), {
                                        let a2831 = {
                                            let a2829 = f626(v4)?;
                                            let a2830 = 16u64;
                                            nat_mul(a2829, a2830)?
                                        };
                                        let a2832 = f626(v5)?;
                                        nat_add(a2831, a2832)?
                                    });
                                    append_bytes(a2833, a2834)
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

pub fn f591(v0: Fn20, p2839: u64, p2840: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2838 = (p2839, p2840);
    {
        let mut m2838 = m2838;
        loop {
            let step = {
                let (v1, v2) = m2838;
                let m2835 = v1;
                if m2835 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2835 - 1;
                    let m2837 = {
                        let c2836: Fn20 = v0.clone();
                        c2836.apply(v2.clone())?
                    };
                    match m2837 {
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
                Ok(next) => { m2838 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f592(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f627(v0)
}

pub fn f593(p2843: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2842 = p2843;
    {
        let mut m2842 = m2842;
        loop {
            let step = {
                let v0 = m2842;
                let m2841 = v0;
                match m2841.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f32(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f52(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f52(v4, v3.clone())
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
                Ok(next) => { m2842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f594(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2844 = v0;
    match m2844 {
        Adt2::C0 => {
            Ok(v1)
        }
        Adt2::C1(_, _) => {
            Ok(v1)
        }
        Adt2::C2(_) => {
            Ok(v1)
        }
        Adt2::C3(_, _) => {
            Ok(v1)
        }
        Adt2::C4(_, _, _) => {
            Ok(v1)
        }
        Adt2::C5(_) => {
            Ok(v1)
        }
        Adt2::C6(r2847) => {
            let v2 = (*r2847).clone();
            f594(v2, v1)
        }
        Adt2::C7(r2848) => {
            let v3 = (*r2848).clone();
            f594(v3, v1)
        }
        Adt2::C8 => {
            Ok(v1)
        }
        Adt2::C9(r2849, r2850) => {
            let v4 = (*r2849).clone();
            let v5 = (*r2850).clone();
            f594(v4, f594(v5, v1)?)
        }
        Adt2::C10(v6, r2851, r2852) => {
            let v7 = (*r2851).clone();
            let v8 = (*r2852).clone();
            let v9: List<Bytes> = f594(v7.clone(), f594(v8, v1)?)?;
            if f32(v6, f8(Str::lit("$ref"))) {
                let m2845 = f385(Some(v7));
                match m2845 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2846 = f628(v10)?;
                        match m2846 {
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

pub fn f595() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f596(v0: Bytes) -> R<Bytes> {
    let m2855 = {
        let a2853 = v0.clone();
        let a2854 = 0u64;
        index_bytes(a2853, a2854)
    };
    match m2855 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2861 = {
                let a2856 = f8(Str::lit("u"));
                let a2857 = f629(8u64, f72(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2856, a2857)
            };
            let a2862 = f46(v0.clone(), 1u64, {
                let a2859 = {
                    let a2858 = v0;
                    length_bytes(a2858)
                };
                let a2860 = 1u64;
                nat_sub(a2859, a2860)
            });
            Ok(append_bytes(a2861, a2862))
        }
    }
}

pub fn f597(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    let m2863 = f276(v1.clone(), f8(Str::lit("enum")));
    match m2863 {
        None => {
            let m2865 = f276(v1.clone(), f8(Str::lit("oneOf")));
            match m2865 {
                None => {
                    if f634(v1.clone(), v0.clone())? {
                        let m2866 = f635(v0.clone(), f521(f276(v1.clone(), f8(Str::lit("properties")))), f280(f281(f276(v1, f8(Str::lit("required"))))), true)?;
                        match m2866 {
                            Ok(v7) => {
                                Ok(f632(f636(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f395(f637(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f638(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2867 = f521(f276(v1.clone(), f8(Str::lit("properties"))));
                        match m2867.uncons() {
                            None => {
                                if f639(v1.clone()) {
                                    Ok(f632(f640(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2870 = f641(v1, {
                                        let a2868 = v0.clone();
                                        let a2869 = f8(Str::lit("Value"));
                                        append_bytes(a2868, a2869)
                                    })?;
                                    match m2870 {
                                        Ok(v9) => {
                                            Ok(f632(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f566(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f565(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2871 = f635(v0.clone(), f521(f276(v1.clone(), f8(Str::lit("properties")))), f280(f281(f276(v1, f8(Str::lit("required"))))), false)?;
                                match m2871 {
                                    Ok(v13) => {
                                        Ok(f632(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2872 = f637(v13.clone());
                                            match m2872.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f395(f637(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f638(v13)))
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
                    f633(v0, f386(v5))
                }
            }
        }
        Some(v2) => {
            let m2864 = f630(f386(v2), List::<Bytes>::nil())?;
            match m2864 {
                Ok(v4) => {
                    Ok(f632(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f395(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f631(v3, v0))
                }
            }
        }
    }
}

pub fn f598(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m2873 = f276(v0.clone(), f8(Str::lit("anyOf")));
        match m2873 {
            None => {
                v0
            }
            Some(v1) => {
                if f642(f386(v1.clone())) {
                    f643(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f644(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f645(f386(v1.clone()))
                    } {
                        v0
                    } else {
                        f646(f643(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m2874 = f276(v2.clone(), f8(Str::lit("oneOf")));
        match m2874 {
            None => {
                v2
            }
            Some(v3) => {
                if f642(f386(v3)) {
                    f643(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2875 = f276(v4.clone(), f8(Str::lit("allOf")));
    match m2875 {
        None => {
            v4
        }
        Some(v5) => {
            if f647(f386(v5)) {
                f643(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f599(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f648(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2876) => {
            h2876
        }
    });
    let v2: u32 = f570(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2877, _, _) => {
            h2877
        }
    }, f570(f649(match v0.clone() {
        Adt27::C0(_, _, _, _, h2878, _, _, _, _, _) => {
            h2878
        }
    }), f570(f650(match v0.clone() {
        Adt27::C0(_, _, _, _, h2879, _, _, _, _, _) => {
            h2879
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2880, _, _, _, _) => {
            h2880
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2881, _, _, _) => {
            h2881
        }
    }), f570(v1, {
        let m2884 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2883, _) => {
                h2883
            }
        };
        match m2884 {
            Adt26::C0(h2882, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2882
            }
        }
    }))));
    let v3: u32 = f570(f651(match v0.clone() {
        Adt27::C0(h2885, _, _, _, _, _, _, _, _, _) => {
            h2885
        }
    }), f652(match v0.clone() {
        Adt27::C0(h2886, _, _, _, _, _, _, _, _, _) => {
            h2886
        }
    }, match v0.clone() {
        Adt27::C0(_, h2887, _, _, _, _, _, _, _, _) => {
            h2887
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2888, _, _, _, _, _, _, _) => {
            h2888
        }
    }));
    Ok(Adt27::C0(f570(v2, v3), match v0.clone() {
        Adt27::C0(h2889, _, _, _, _, _, _, _, _, _) => {
            h2889
        }
    }, match v0.clone() {
        Adt27::C0(_, h2890, _, _, _, _, _, _, _, _) => {
            h2890
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2891, _, _, _, _, _, _, _) => {
            h2891
        }
    }, f570(match v0.clone() {
        Adt27::C0(_, _, _, h2892, _, _, _, _, _, _) => {
            h2892
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2893, _, _, _, _, _) => {
            h2893
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2894, _, _, _, _) => {
            h2894
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2895, _, _, _) => {
            h2895
        }
    }, f653(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2896, _) => {
            h2896
        }
    }), {
        let a2898 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2897) => {
                h2897
            }
        };
        let a2899 = 1u64;
        nat_add(a2898, a2899)?
    }))
}

pub fn f600(v0: u32) -> u64 {
    let m2901 = {
        let a2900 = v0;
        fixed_u64::convert(i128::from(a2900))
    };
    match m2901 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f601(v0: u64) -> u32 {
    let m2905 = {
        let a2904 = {
            let a2902 = v0;
            let a2903 = 4294967295u64;
            fixed_u64::bit_and(a2902, a2903)
        };
        fixed_u32::convert(i128::from(a2904))
    };
    match m2905 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f602(v0: u64) -> u32 {
    let (_, h2906) = {
        let (_, h2907) = f654(Fn21::F655, 32u64, (v0, (1u32, 0u32)));
        h2907
    };
    h2906
}

pub fn f603(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2912 = f656(v0, 24u32);
    let a2913 = {
        let a2910 = f656(v1, 16u32);
        let a2911 = {
            let a2908 = f656(v2, 8u32);
            let a2909 = v3;
            fixed_u32::bit_or(a2908, a2909)
        };
        fixed_u32::bit_or(a2910, a2911)
    };
    fixed_u32::bit_or(a2912, a2913)
}

pub fn f604(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2914 = v2;
    let a2915 = v1;
    if nat_lt(a2914, a2915) {
        let m2918 = {
            let a2916 = v0;
            let a2917 = v2;
            index_bytes(a2916, a2917)
        };
        match m2918 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2920 = {
                    let a2919 = v3;
                    fixed_u32::convert(i128::from(a2919))
                };
                match m2920 {
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
        let a2921 = v2;
        let a2922 = v1;
        if nat_eq(a2921, a2922) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f605() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("patternProperties")), List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("definitions")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("format")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))
}

pub fn f606(v0: Adt2) -> Bytes {
    let m2923 = v0;
    match m2923 {
        Adt2::C0 => {
            f8(Str::lit("null"))
        }
        Adt2::C1(_, _) => {
            f8(Str::lit("tagged"))
        }
        Adt2::C2(_) => {
            f8(Str::lit("boolean"))
        }
        Adt2::C3(_, _) => {
            f8(Str::lit("integer"))
        }
        Adt2::C4(_, _, _) => {
            f8(Str::lit("number"))
        }
        Adt2::C5(_) => {
            f8(Str::lit("string"))
        }
        Adt2::C6(_) => {
            f8(Str::lit("array"))
        }
        Adt2::C7(_) => {
            f8(Str::lit("object"))
        }
        Adt2::C8 => {
            f8(Str::lit("spine"))
        }
        Adt2::C9(_, _) => {
            f8(Str::lit("spine"))
        }
        Adt2::C10(_, _, _) => {
            f8(Str::lit("spine"))
        }
    }
}

pub fn f607(p2926: List<Bytes>, v1: Adt2) -> bool {
    let m2925 = p2926;
    {
        let mut m2925 = m2925;
        loop {
            let step = {
                let v0 = m2925;
                let m2924 = v0;
                match m2924.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f657(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
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

pub fn f608(p2929: List<Adt2>, v1: Adt2) -> R<bool> {
    let m2928 = p2929;
    {
        let mut m2928 = m2928;
        loop {
            let step = {
                let v0 = m2928;
                let m2927 = v0;
                match m2927.uncons() {
                    None => {
                        Err::<List<Adt2>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f609(false, v2, v1.clone())? {
                            Err::<List<Adt2>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt2>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2928 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f609(v0: bool, v1: Adt2, v2: Adt2) -> R<bool> {
    let m2930 = v1;
    match m2930 {
        Adt2::C0 => {
            Ok(f32(f537(v2)?, f8(Str::lit("null"))))
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(v77) => {
            Ok(f32(f537(v2)?, f537(Adt2::C2(v77))?))
        }
        Adt2::C3(v78, v79) => {
            Ok(f32(f537(v2)?, f537(Adt2::C3(v78, v79))?))
        }
        Adt2::C4(v80, v81, v82) => {
            Ok(f32(f537(v2)?, f537(Adt2::C4(v80, v81, v82))?))
        }
        Adt2::C5(v83) => {
            let m2942 = v2;
            match m2942 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(v84) => {
                    Ok(f32(v83, v84))
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8 => {
                    Ok(false)
                }
                Adt2::C9(_, _) => {
                    Ok(false)
                }
                Adt2::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C6(r2943) => {
            let v58 = (*r2943).clone();
            let m2940 = v2;
            match m2940 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_) => {
                    Ok(false)
                }
                Adt2::C6(r2941) => {
                    let v59 = (*r2941).clone();
                    f609(false, v58, v59)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8 => {
                    Ok(false)
                }
                Adt2::C9(_, _) => {
                    Ok(false)
                }
                Adt2::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C7(r2944) => {
            let v41 = (*r2944).clone();
            let m2936 = v2;
            match m2936 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(r2939) => {
                    let v42 = (*r2939).clone();
                    if f118({
                        let a2937 = f614(v41.clone())?;
                        let a2938 = f614(v42.clone())?;
                        nat_eq(a2937, a2938)
                    }, true) {
                        f609(true, v41, v42)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C8 => {
                    Ok(false)
                }
                Adt2::C9(_, _) => {
                    Ok(false)
                }
                Adt2::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C8 => {
            if v0 {
                Ok(true)
            } else {
                let m2932 = v2;
                match m2932 {
                    Adt2::C0 => {
                        Ok(false)
                    }
                    Adt2::C1(_, _) => {
                        Ok(false)
                    }
                    Adt2::C2(_) => {
                        Ok(false)
                    }
                    Adt2::C3(_, _) => {
                        Ok(false)
                    }
                    Adt2::C4(_, _, _) => {
                        Ok(false)
                    }
                    Adt2::C5(_) => {
                        Ok(false)
                    }
                    Adt2::C6(_) => {
                        Ok(false)
                    }
                    Adt2::C7(_) => {
                        Ok(false)
                    }
                    Adt2::C8 => {
                        Ok(true)
                    }
                    Adt2::C9(_, _) => {
                        Ok(false)
                    }
                    Adt2::C10(_, _, _) => {
                        Ok(false)
                    }
                }
            }
        }
        Adt2::C9(r2945, r2946) => {
            let v23 = (*r2945).clone();
            let v24 = (*r2946).clone();
            let m2933 = v2;
            match m2933 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8 => {
                    Ok(false)
                }
                Adt2::C9(r2934, r2935) => {
                    let v25 = (*r2934).clone();
                    let v26 = (*r2935).clone();
                    if f609(false, v23, v25)? {
                        f609(false, v24, v26)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C10(v3, r2947, r2948) => {
            let v4 = (*r2947).clone();
            let v5 = (*r2948).clone();
            let m2931 = f276(v2.clone(), v3);
            if match m2931 {
                None => {
                    false
                }
                Some(v6) => {
                    f609(false, v4, v6)?
                }
            } {
                f609(true, v5, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f610(v0: Bytes) -> R<Option<List<u64>>> {
    f658({
        let a2950 = {
            let a2949 = v0.clone();
            length_bytes(a2949)
        };
        let a2951 = 1u64;
        nat_add(a2950, a2951)?
    }, v0, 0u64)
}

pub fn f611(v0: List<u64>) -> R<u64> {
    let m2952 = v0;
    match m2952.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a2953 = f611(v2)?;
            let a2954 = 1u64;
            nat_add(a2953, a2954)
        }
    }
}

pub fn f612(v0: Option<Adt2>) -> Option<u64> {
    let m2955 = v0;
    match m2955 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2956 = v1;
            match m2956 {
                Adt2::C0 => {
                    None::<u64>
                }
                Adt2::C1(_, _) => {
                    None::<u64>
                }
                Adt2::C2(_) => {
                    None::<u64>
                }
                Adt2::C3(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt2::C4(_, _, _) => {
                    None::<u64>
                }
                Adt2::C5(_) => {
                    None::<u64>
                }
                Adt2::C6(_) => {
                    None::<u64>
                }
                Adt2::C7(_) => {
                    None::<u64>
                }
                Adt2::C8 => {
                    None::<u64>
                }
                Adt2::C9(_, _) => {
                    None::<u64>
                }
                Adt2::C10(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f613(v0: Bytes, v1: Bytes) -> R<Result<bool, Bytes>> {
    let m2957 = f659(v0)?;
    match m2957 {
        Ok(v3) => {
            let m2958 = f610(v1)?;
            match m2958 {
                None => {
                    Ok(Err::<bool, Bytes>(f8(Str::lit("a string is not well-formed UTF-8 (RFC 3629 section 4)"))))
                }
                Some(v4) => {
                    Ok(Ok::<bool, Bytes>({
                        let (h2959, _) = {
                            let (h2960, _) = v3.clone();
                            h2960
                        };
                        if h2959 {
                            f660({
                                let a2964 = {
                                    let a2962 = f661({
                                        let (_, h2961) = v3.clone();
                                        h2961
                                    })?;
                                    let a2963 = f611(v4.clone())?;
                                    nat_add(a2962, a2963)?
                                };
                                let a2965 = 2u64;
                                nat_add(a2964, a2965)?
                            }, {
                                let (_, h2966) = v3;
                                h2966
                            }, v4, {
                                let (_, h2967) = {
                                    let (h2968, _) = v3;
                                    h2968
                                };
                                h2967
                            })?
                        } else {
                            f662({
                                let (_, h2969) = v3;
                                h2969
                            }, v4, {
                                let (_, h2970) = {
                                    let (h2971, _) = v3;
                                    h2971
                                };
                                h2970
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

pub fn f614(v0: Adt2) -> R<u64> {
    let m2972 = v0;
    match m2972 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1(_, _) => {
            Ok(0u64)
        }
        Adt2::C2(_) => {
            Ok(0u64)
        }
        Adt2::C3(_, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_) => {
            Ok(0u64)
        }
        Adt2::C6(_) => {
            Ok(0u64)
        }
        Adt2::C7(_) => {
            Ok(0u64)
        }
        Adt2::C8 => {
            Ok(0u64)
        }
        Adt2::C9(_, r2977) => {
            let v5 = (*r2977).clone();
            let a2975 = f614(v5)?;
            let a2976 = 1u64;
            nat_add(a2975, a2976)
        }
        Adt2::C10(_, _, r2978) => {
            let v3 = (*r2978).clone();
            let a2973 = f614(v3)?;
            let a2974 = 1u64;
            nat_add(a2973, a2974)
        }
    }
}

pub fn f615(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2979 = v0;
    match m2979.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f615(v3, v1)
            } else {
                List::cons(v2, f615(v3, v1))
            }
        }
    }
}

pub fn f616(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m2980 = v0;
    match m2980.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2981, _) = v1;
                h2981
            }, f616(v2))
        }
    }
}

pub fn f617(v0: Adt2) -> Option<(bool, (u64, u64))> {
    let m2982 = v0;
    match m2982 {
        Adt2::C0 => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C1(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C2(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C3(v1, v2) => {
            Some((v1, (v2, 0u64)))
        }
        Adt2::C4(v3, v4, v5) => {
            Some((v3, (v4, v5)))
        }
        Adt2::C5(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C6(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C7(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C8 => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C9(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C10(_, _, _) => {
            None::<(bool, (u64, u64))>
        }
    }
}

pub fn f618(v0: (bool, (u64, u64)), v1: (bool, (u64, u64))) -> R<bool> {
    let v2: u64 = {
        let a2987 = {
            let (h2983, _) = {
                let (_, h2984) = v0;
                h2984
            };
            h2983
        };
        let a2988 = f663({
            let (_, h2985) = {
                let (_, h2986) = v1;
                h2986
            };
            h2985
        })?;
        nat_mul(a2987, a2988)?
    };
    let v3: u64 = {
        let a2993 = {
            let (h2989, _) = {
                let (_, h2990) = v1;
                h2990
            };
            h2989
        };
        let a2994 = f663({
            let (_, h2991) = {
                let (_, h2992) = v0;
                h2992
            };
            h2991
        })?;
        nat_mul(a2993, a2994)?
    };
    let (h2995, _) = v0;
    if h2995 {
        let (h2996, _) = v1;
        if h2996 {
            let a2997 = v3;
            let a2998 = v2;
            Ok(nat_lt(a2997, a2998))
        } else {
            Ok(true)
        }
    } else {
        let (h2999, _) = v1;
        if h2999 {
            Ok(false)
        } else {
            let a3000 = v2;
            let a3001 = v3;
            Ok(nat_lt(a3000, a3001))
        }
    }
}

pub fn f619(p3031: u64, v1: Bytes, p3032: u64, p3033: Bytes) -> R<Bytes> {
    let m3030 = (p3031, (p3032, p3033));
    {
        let mut m3030 = m3030;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3030;
                let m3002 = v0;
                if m3002 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3002 - 1;
                    let a3004 = {
                        let a3003 = v1.clone();
                        length_bytes(a3003)
                    };
                    let a3005 = v2;
                    if nat_le(a3004, a3005) {
                        Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                    } else {
                        if f118({
                            let a3006 = f116(v1.clone(), v2)?;
                            let a3007 = 126u64;
                            nat_eq(a3006, a3007)
                        }, {
                            let a3010 = f116(v1.clone(), {
                                let a3008 = v2;
                                let a3009 = 1u64;
                                nat_add(a3008, a3009)?
                            })?;
                            let a3011 = 48u64;
                            nat_eq(a3010, a3011)
                        }) {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3012 = v2;
                                let a3013 = 2u64;
                                nat_add(a3012, a3013)?
                            }, {
                                let a3014 = v3;
                                let a3015 = f8(Str::lit("~"));
                                append_bytes(a3014, a3015)
                            })))
                        } else {
                            if f118({
                                let a3016 = f116(v1.clone(), v2)?;
                                let a3017 = 126u64;
                                nat_eq(a3016, a3017)
                            }, {
                                let a3020 = f116(v1.clone(), {
                                    let a3018 = v2;
                                    let a3019 = 1u64;
                                    nat_add(a3018, a3019)?
                                })?;
                                let a3021 = 49u64;
                                nat_eq(a3020, a3021)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a3022 = v2;
                                    let a3023 = 2u64;
                                    nat_add(a3022, a3023)?
                                }, {
                                    let a3024 = v3;
                                    let a3025 = f8(Str::lit("/"));
                                    append_bytes(a3024, a3025)
                                })))
                            } else {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a3026 = v2;
                                    let a3027 = 1u64;
                                    nat_add(a3026, a3027)?
                                }, {
                                    let a3028 = v3;
                                    let a3029 = f46(v1.clone(), v2, 1u64);
                                    append_bytes(a3028, a3029)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3030 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f620(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(f664({
        let a3035 = {
            let a3034 = v1.clone();
            length_bytes(a3034)
        };
        let a3036 = 1u64;
        nat_add(a3035, a3036)?
    }, v1, 0u64, Bytes::lit(&[]))?, List::<Bytes>::nil())))))
}

pub fn f621(v0: List<(Bytes, Adt2)>, v1: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let m3037 = v0;
    match m3037.uncons() {
        None => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let m3039 = f613({
                let (h3038, _) = v2;
                h3038
            }, v1.clone())?;
            match m3039 {
                Ok(v5) => {
                    let m3040 = f621(v3, v1)?;
                    match m3040 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt2>, Bytes>(if v5 {
                                List::cons({
                                    let (_, h3041) = v2;
                                    h3041
                                }, v7)
                            } else {
                                v7
                            }))
                        }
                        Err(v6) => {
                            Ok(Err::<List<Adt2>, Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt2>, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f622() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f623(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f545({
        let (_, h3042) = {
            let (h3043, _) = f665(Fn22::F666, {
                let a3045 = {
                    let a3044 = v0.clone();
                    length_list(a3044)
                };
                let a3046 = 1u64;
                nat_add(a3045, a3046)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3043
        };
        h3042
    }))
}

pub fn f624(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3091 = {
            let a3067 = {
                let a3055 = {
                    let a3049 = {
                        let a3047 = v1;
                        let a3048 = 128u8;
                        fixed_u8::bit_and(a3047, a3048)
                    };
                    let a3050 = 128u8;
                    if equal(a3049, a3050) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3056 = {
                    let a3053 = {
                        let a3051 = v1;
                        let a3052 = 64u8;
                        fixed_u8::bit_and(a3051, a3052)
                    };
                    let a3054 = 64u8;
                    if equal(a3053, a3054) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3055, a3056)?
            };
            let a3068 = {
                let a3065 = {
                    let a3059 = {
                        let a3057 = v1;
                        let a3058 = 32u8;
                        fixed_u8::bit_and(a3057, a3058)
                    };
                    let a3060 = 32u8;
                    if equal(a3059, a3060) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3066 = {
                    let a3063 = {
                        let a3061 = v1;
                        let a3062 = 16u8;
                        fixed_u8::bit_and(a3061, a3062)
                    };
                    let a3064 = 16u8;
                    if equal(a3063, a3064) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3065, a3066)?
            };
            nat_add(a3067, a3068)?
        };
        let a3092 = {
            let a3089 = {
                let a3077 = {
                    let a3071 = {
                        let a3069 = v1;
                        let a3070 = 8u8;
                        fixed_u8::bit_and(a3069, a3070)
                    };
                    let a3072 = 8u8;
                    if equal(a3071, a3072) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3078 = {
                    let a3075 = {
                        let a3073 = v1;
                        let a3074 = 4u8;
                        fixed_u8::bit_and(a3073, a3074)
                    };
                    let a3076 = 4u8;
                    if equal(a3075, a3076) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3077, a3078)?
            };
            let a3090 = {
                let a3087 = {
                    let a3081 = {
                        let a3079 = v1;
                        let a3080 = 2u8;
                        fixed_u8::bit_and(a3079, a3080)
                    };
                    let a3082 = 2u8;
                    if equal(a3081, a3082) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3088 = {
                    let a3085 = {
                        let a3083 = v1;
                        let a3084 = 1u8;
                        fixed_u8::bit_and(a3083, a3084)
                    };
                    let a3086 = 1u8;
                    if equal(a3085, a3086) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3087, a3088)?
            };
            nat_add(a3089, a3090)?
        };
        nat_add(a3091, a3092)?
    };
    let a3093 = 48u64;
    let a3094 = v2;
    if if nat_le(a3093, a3094) {
        let a3095 = v2;
        let a3096 = 57u64;
        nat_le(a3095, a3096)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3097 = 97u64;
        let a3098 = v2;
        if nat_le(a3097, a3098) {
            let a3099 = v2;
            let a3100 = 102u64;
            Ok(nat_le(a3099, a3100))
        } else {
            Ok(false)
        }
    }
}

pub fn f625() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f626(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3145 = {
            let a3121 = {
                let a3109 = {
                    let a3103 = {
                        let a3101 = v1;
                        let a3102 = 128u8;
                        fixed_u8::bit_and(a3101, a3102)
                    };
                    let a3104 = 128u8;
                    if equal(a3103, a3104) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3110 = {
                    let a3107 = {
                        let a3105 = v1;
                        let a3106 = 64u8;
                        fixed_u8::bit_and(a3105, a3106)
                    };
                    let a3108 = 64u8;
                    if equal(a3107, a3108) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3109, a3110)?
            };
            let a3122 = {
                let a3119 = {
                    let a3113 = {
                        let a3111 = v1;
                        let a3112 = 32u8;
                        fixed_u8::bit_and(a3111, a3112)
                    };
                    let a3114 = 32u8;
                    if equal(a3113, a3114) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3120 = {
                    let a3117 = {
                        let a3115 = v1;
                        let a3116 = 16u8;
                        fixed_u8::bit_and(a3115, a3116)
                    };
                    let a3118 = 16u8;
                    if equal(a3117, a3118) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3119, a3120)?
            };
            nat_add(a3121, a3122)?
        };
        let a3146 = {
            let a3143 = {
                let a3131 = {
                    let a3125 = {
                        let a3123 = v1;
                        let a3124 = 8u8;
                        fixed_u8::bit_and(a3123, a3124)
                    };
                    let a3126 = 8u8;
                    if equal(a3125, a3126) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3132 = {
                    let a3129 = {
                        let a3127 = v1;
                        let a3128 = 4u8;
                        fixed_u8::bit_and(a3127, a3128)
                    };
                    let a3130 = 4u8;
                    if equal(a3129, a3130) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3131, a3132)?
            };
            let a3144 = {
                let a3141 = {
                    let a3135 = {
                        let a3133 = v1;
                        let a3134 = 2u8;
                        fixed_u8::bit_and(a3133, a3134)
                    };
                    let a3136 = 2u8;
                    if equal(a3135, a3136) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3142 = {
                    let a3139 = {
                        let a3137 = v1;
                        let a3138 = 1u8;
                        fixed_u8::bit_and(a3137, a3138)
                    };
                    let a3140 = 1u8;
                    if equal(a3139, a3140) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3141, a3142)?
            };
            nat_add(a3143, a3144)?
        };
        nat_add(a3145, a3146)?
    };
    let a3147 = v2;
    let a3148 = 57u64;
    if nat_le(a3147, a3148) {
        let a3149 = v2;
        let a3150 = 48u64;
        Ok(nat_sub(a3149, a3150))
    } else {
        let a3151 = v2;
        let a3152 = 87u64;
        Ok(nat_sub(a3151, a3152))
    }
}

pub fn f627(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3153, _) = v0;
        h3153
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3154) = v0;
        h3154
    };
    let m3155 = v1;
    match m3155.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3156 = v4;
            match m3156.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f667(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f628(v0: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f561(f46(v0.clone(), 8u64, {
            let a3158 = {
                let a3157 = v0;
                length_bytes(a3157)
            };
            let a3159 = 8u64;
            nat_sub(a3158, a3159)
        }))?))
    } else {
        if f85(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f561(f46(v0.clone(), 14u64, {
                let a3161 = {
                    let a3160 = v0;
                    length_bytes(a3160)
                };
                let a3162 = 14u64;
                nat_sub(a3161, a3162)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f629(p3179: u64, p3180: u64, p3181: u64, p3182: Bytes) -> Bytes {
    let m3178 = (p3179, (p3180, (p3181, p3182)));
    {
        let mut m3178 = m3178;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3178;
                let m3163 = v0;
                if m3163 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3163 - 1;
                    if f118({
                        let a3164 = v1;
                        let a3165 = 0u64;
                        nat_eq(a3164, a3165)
                    }, {
                        let a3166 = v2;
                        let a3167 = 0u64;
                        nat_eq(a3166, a3167)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3168 = v1;
                            let a3169 = 16u64;
                            let a3170 = 0u64;
                            nat_quot(a3168, a3169, a3170)
                        }, ({
                            let a3171 = v2;
                            let a3172 = 1u64;
                            nat_sub(a3171, a3172)
                        }, {
                            let a3176 = f668({
                                let a3173 = v1;
                                let a3174 = 16u64;
                                let a3175 = 0u64;
                                nat_rem(a3173, a3174, a3175)
                            });
                            let a3177 = v3;
                            append_bytes(a3176, a3177)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f630(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f669(f670(f280(v0.clone()))?, v0, v1)
}

pub fn f631(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f632(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f633(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt28, Bytes>> {
    let m3183 = v1.clone();
    match m3183.uncons() {
        None => {
            Ok(f631(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3184 = f671(f521(f276(v2, f8(Str::lit("properties")))), v1.clone());
            match m3184.uncons() {
                None => {
                    let m3185 = f672(f670(f673(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3185 {
                        Ok(v5) => {
                            Ok(f632(f636(v0, f637(v5.clone())), f638(v5)))
                        }
                        Err(v4) => {
                            Ok(f631(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3186 = f674(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3186 {
                        Ok(v9) => {
                            Ok(f632(f636(v0, f637(v9.clone())), f638(v9)))
                        }
                        Err(v8) => {
                            Ok(f631(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f634(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3187 = v0;
    match m3187 {
        Adt2::C0 => {
            Ok(false)
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(_) => {
            Ok(false)
        }
        Adt2::C3(_, _) => {
            Ok(false)
        }
        Adt2::C4(_, _, _) => {
            Ok(false)
        }
        Adt2::C5(_) => {
            Ok(false)
        }
        Adt2::C6(r3188) => {
            let v2 = (*r3188).clone();
            f634(v2, v1)
        }
        Adt2::C7(r3189) => {
            let v3 = (*r3189).clone();
            f634(v3, v1)
        }
        Adt2::C8 => {
            Ok(false)
        }
        Adt2::C9(r3190, r3191) => {
            let v4 = (*r3190).clone();
            let v5 = (*r3191).clone();
            if f634(v4, v1.clone())? {
                Ok(true)
            } else {
                f634(v5, v1)
            }
        }
        Adt2::C10(v6, r3192, r3193) => {
            let v7 = (*r3192).clone();
            let v8 = (*r3193).clone();
            if if f32(v6, f8(Str::lit("$ref"))) {
                f675(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f634(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f634(v8, v1)
                }
            }
        }
    }
}

pub fn f635(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f676(f670(f616(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f636(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f395(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f637(v0: Adt29) -> List<Bytes> {
    let m3194 = v0;
    match m3194 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f638(v0: Adt29) -> List<Adt25> {
    let m3195 = v0;
    match m3195 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f639(v0: Adt2) -> bool {
    let m3196 = f521(f276(v0.clone(), f8(Str::lit("properties"))));
    match m3196.uncons() {
        None => {
            let m3197 = f677(v0);
            match m3197 {
                None => {
                    false
                }
                Some(v3) => {
                    f678(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f640(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f641(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3198 = f679(v0.clone());
    match m3198 {
        None => {
            if f32(f279(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3199 = f276(v0, f8(Str::lit("items")));
                match m3199 {
                    None => {
                        Ok(f680(f8(Str::lit("List")), f682()))
                    }
                    Some(v3) => {
                        Ok(f680(f8(Str::lit("List")), f681(v3, {
                            let a3200 = v1;
                            let a3201 = f8(Str::lit("Item"));
                            append_bytes(a3200, a3201)
                        })?))
                    }
                }
            } else {
                f681(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f680(f8(Str::lit("Option")), f681(v2, v1)?))
        }
    }
}

pub fn f642(p3204: List<Adt2>) -> bool {
    let m3203 = p3204;
    {
        let mut m3203 = m3203;
        loop {
            let step = {
                let v0 = m3203;
                let m3202 = v0;
                match m3202.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f683(f558(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
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

pub fn f643(v0: Adt2, v1: Bytes) -> Adt2 {
    let m3205 = v0;
    match m3205 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v6, r3206) => {
            let v7 = (*r3206).clone();
            Adt2::C1(v6, std::rc::Rc::new(v7))
        }
        Adt2::C2(v8) => {
            Adt2::C2(v8)
        }
        Adt2::C3(v12, v13) => {
            Adt2::C3(v12, v13)
        }
        Adt2::C4(v9, v10, v11) => {
            Adt2::C4(v9, v10, v11)
        }
        Adt2::C5(v14) => {
            Adt2::C5(v14)
        }
        Adt2::C6(r3207) => {
            let v15 = (*r3207).clone();
            Adt2::C6(std::rc::Rc::new(v15))
        }
        Adt2::C7(r3208) => {
            let v5 = (*r3208).clone();
            Adt2::C7(std::rc::Rc::new(f643(v5, v1)))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r3209, r3210) => {
            let v16 = (*r3209).clone();
            let v17 = (*r3210).clone();
            Adt2::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt2::C10(v2, r3211, r3212) => {
            let v3 = (*r3211).clone();
            let v4 = (*r3212).clone();
            if f32(v2.clone(), v1.clone()) {
                f643(v4, v1)
            } else {
                Adt2::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f643(v4, v1)))
            }
        }
    }
}

pub fn f644(v0: Adt2, v1: Bytes) -> bool {
    let m3213 = f276(v0, v1);
    match m3213 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f645(v0: List<Adt2>) -> bool {
    let m3214 = v0.clone();
    match m3214.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3215 = f671(f521(f276(v1, f8(Str::lit("properties")))), v0.clone());
            match m3215.uncons() {
                None => {
                    f685(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f684(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f646(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m3216 = v0;
    match m3216 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v7, r3217) => {
            let v8 = (*r3217).clone();
            Adt2::C1(v7, std::rc::Rc::new(v8))
        }
        Adt2::C2(v9) => {
            Adt2::C2(v9)
        }
        Adt2::C3(v13, v14) => {
            Adt2::C3(v13, v14)
        }
        Adt2::C4(v10, v11, v12) => {
            Adt2::C4(v10, v11, v12)
        }
        Adt2::C5(v15) => {
            Adt2::C5(v15)
        }
        Adt2::C6(r3218) => {
            let v16 = (*r3218).clone();
            Adt2::C6(std::rc::Rc::new(v16))
        }
        Adt2::C7(r3219) => {
            let v3 = (*r3219).clone();
            Adt2::C7(std::rc::Rc::new(Adt2::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r3220, r3221) => {
            let v17 = (*r3220).clone();
            let v18 = (*r3221).clone();
            Adt2::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt2::C10(v4, r3222, r3223) => {
            let v5 = (*r3222).clone();
            let v6 = (*r3223).clone();
            Adt2::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f647(p3226: List<Adt2>) -> bool {
    let m3225 = p3226;
    {
        let mut m3225 = m3225;
        loop {
            let step = {
                let v0 = m3225;
                let m3224 = v0;
                match m3224.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f686(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3225 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648(v0: u64) -> u32 {
    let a3227 = v0;
    let a3228 = 32u64;
    if nat_lt(a3227, a3228) {
        let a3229 = v0;
        let a3230 = 16u64;
        if nat_lt(a3229, a3230) {
            let a3231 = v0;
            let a3232 = 8u64;
            if nat_lt(a3231, a3232) {
                let a3233 = v0;
                let a3234 = 4u64;
                if nat_lt(a3233, a3234) {
                    let a3235 = v0;
                    let a3236 = 2u64;
                    if nat_lt(a3235, a3236) {
                        let a3237 = v0;
                        let a3238 = 1u64;
                        if nat_lt(a3237, a3238) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3239 = v0;
                        let a3240 = 3u64;
                        if nat_lt(a3239, a3240) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3241 = v0;
                    let a3242 = 6u64;
                    if nat_lt(a3241, a3242) {
                        let a3243 = v0;
                        let a3244 = 5u64;
                        if nat_lt(a3243, a3244) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3245 = v0;
                        let a3246 = 7u64;
                        if nat_lt(a3245, a3246) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3247 = v0;
                let a3248 = 12u64;
                if nat_lt(a3247, a3248) {
                    let a3249 = v0;
                    let a3250 = 10u64;
                    if nat_lt(a3249, a3250) {
                        let a3251 = v0;
                        let a3252 = 9u64;
                        if nat_lt(a3251, a3252) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3253 = v0;
                        let a3254 = 11u64;
                        if nat_lt(a3253, a3254) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3255 = v0;
                    let a3256 = 14u64;
                    if nat_lt(a3255, a3256) {
                        let a3257 = v0;
                        let a3258 = 13u64;
                        if nat_lt(a3257, a3258) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3259 = v0;
                        let a3260 = 15u64;
                        if nat_lt(a3259, a3260) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3261 = v0;
            let a3262 = 24u64;
            if nat_lt(a3261, a3262) {
                let a3263 = v0;
                let a3264 = 20u64;
                if nat_lt(a3263, a3264) {
                    let a3265 = v0;
                    let a3266 = 18u64;
                    if nat_lt(a3265, a3266) {
                        let a3267 = v0;
                        let a3268 = 17u64;
                        if nat_lt(a3267, a3268) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3269 = v0;
                        let a3270 = 19u64;
                        if nat_lt(a3269, a3270) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3271 = v0;
                    let a3272 = 22u64;
                    if nat_lt(a3271, a3272) {
                        let a3273 = v0;
                        let a3274 = 21u64;
                        if nat_lt(a3273, a3274) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3275 = v0;
                        let a3276 = 23u64;
                        if nat_lt(a3275, a3276) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3277 = v0;
                let a3278 = 28u64;
                if nat_lt(a3277, a3278) {
                    let a3279 = v0;
                    let a3280 = 26u64;
                    if nat_lt(a3279, a3280) {
                        let a3281 = v0;
                        let a3282 = 25u64;
                        if nat_lt(a3281, a3282) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3283 = v0;
                        let a3284 = 27u64;
                        if nat_lt(a3283, a3284) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3285 = v0;
                    let a3286 = 30u64;
                    if nat_lt(a3285, a3286) {
                        let a3287 = v0;
                        let a3288 = 29u64;
                        if nat_lt(a3287, a3288) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3289 = v0;
                        let a3290 = 31u64;
                        if nat_lt(a3289, a3290) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3291 = v0;
        let a3292 = 48u64;
        if nat_lt(a3291, a3292) {
            let a3293 = v0;
            let a3294 = 40u64;
            if nat_lt(a3293, a3294) {
                let a3295 = v0;
                let a3296 = 36u64;
                if nat_lt(a3295, a3296) {
                    let a3297 = v0;
                    let a3298 = 34u64;
                    if nat_lt(a3297, a3298) {
                        let a3299 = v0;
                        let a3300 = 33u64;
                        if nat_lt(a3299, a3300) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3301 = v0;
                        let a3302 = 35u64;
                        if nat_lt(a3301, a3302) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3303 = v0;
                    let a3304 = 38u64;
                    if nat_lt(a3303, a3304) {
                        let a3305 = v0;
                        let a3306 = 37u64;
                        if nat_lt(a3305, a3306) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3307 = v0;
                        let a3308 = 39u64;
                        if nat_lt(a3307, a3308) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3309 = v0;
                let a3310 = 44u64;
                if nat_lt(a3309, a3310) {
                    let a3311 = v0;
                    let a3312 = 42u64;
                    if nat_lt(a3311, a3312) {
                        let a3313 = v0;
                        let a3314 = 41u64;
                        if nat_lt(a3313, a3314) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3315 = v0;
                        let a3316 = 43u64;
                        if nat_lt(a3315, a3316) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3317 = v0;
                    let a3318 = 46u64;
                    if nat_lt(a3317, a3318) {
                        let a3319 = v0;
                        let a3320 = 45u64;
                        if nat_lt(a3319, a3320) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3321 = v0;
                        let a3322 = 47u64;
                        if nat_lt(a3321, a3322) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3323 = v0;
            let a3324 = 56u64;
            if nat_lt(a3323, a3324) {
                let a3325 = v0;
                let a3326 = 52u64;
                if nat_lt(a3325, a3326) {
                    let a3327 = v0;
                    let a3328 = 50u64;
                    if nat_lt(a3327, a3328) {
                        let a3329 = v0;
                        let a3330 = 49u64;
                        if nat_lt(a3329, a3330) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3331 = v0;
                        let a3332 = 51u64;
                        if nat_lt(a3331, a3332) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3333 = v0;
                    let a3334 = 54u64;
                    if nat_lt(a3333, a3334) {
                        let a3335 = v0;
                        let a3336 = 53u64;
                        if nat_lt(a3335, a3336) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3337 = v0;
                        let a3338 = 55u64;
                        if nat_lt(a3337, a3338) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3339 = v0;
                let a3340 = 60u64;
                if nat_lt(a3339, a3340) {
                    let a3341 = v0;
                    let a3342 = 58u64;
                    if nat_lt(a3341, a3342) {
                        let a3343 = v0;
                        let a3344 = 57u64;
                        if nat_lt(a3343, a3344) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3345 = v0;
                        let a3346 = 59u64;
                        if nat_lt(a3345, a3346) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3347 = v0;
                    let a3348 = 62u64;
                    if nat_lt(a3347, a3348) {
                        let a3349 = v0;
                        let a3350 = 61u64;
                        if nat_lt(a3349, a3350) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3351 = v0;
                        let a3352 = 63u64;
                        if nat_lt(a3351, a3352) {
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

pub fn f649(v0: u32) -> u32 {
    let a3355 = {
        let a3353 = f687(v0, 6u32, 26u32);
        let a3354 = f687(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3353, a3354)
    };
    let a3356 = f687(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3355, a3356)
}

pub fn f650(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3362 = {
        let a3357 = v0;
        let a3358 = v1;
        fixed_u32::bit_and(a3357, a3358)
    };
    let a3363 = {
        let a3360 = {
            let a3359 = v0;
            fixed_u32::bit_not(a3359)
        };
        let a3361 = v2;
        fixed_u32::bit_and(a3360, a3361)
    };
    fixed_u32::bit_xor(a3362, a3363)
}

pub fn f651(v0: u32) -> u32 {
    let a3366 = {
        let a3364 = f687(v0, 2u32, 30u32);
        let a3365 = f687(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3364, a3365)
    };
    let a3367 = f687(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3366, a3367)
}

pub fn f652(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3376 = {
        let a3372 = {
            let a3368 = v0;
            let a3369 = v1;
            fixed_u32::bit_and(a3368, a3369)
        };
        let a3373 = {
            let a3370 = v0;
            let a3371 = v2;
            fixed_u32::bit_and(a3370, a3371)
        };
        fixed_u32::bit_xor(a3372, a3373)
    };
    let a3377 = {
        let a3374 = v1;
        let a3375 = v2;
        fixed_u32::bit_and(a3374, a3375)
    };
    fixed_u32::bit_xor(a3376, a3377)
}

pub fn f653(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3378, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3378
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3379, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3379
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3380, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3380
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3381, _, _, _, _, _, _, _, _, _, _, _) => {
            h3381
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3382, _, _, _, _, _, _, _, _, _, _) => {
            h3382
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3383, _, _, _, _, _, _, _, _, _) => {
            h3383
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3384, _, _, _, _, _, _, _, _) => {
            h3384
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3385, _, _, _, _, _, _, _) => {
            h3385
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3386, _, _, _, _, _, _) => {
            h3386
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3387, _, _, _, _, _) => {
            h3387
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3388, _, _, _, _) => {
            h3388
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3389, _, _, _) => {
            h3389
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3390, _, _) => {
            h3390
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3391, _) => {
            h3391
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3392) => {
            h3392
        }
    }, f570(f688(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3393, _) => {
            h3393
        }
    }), f570(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3394, _, _, _, _, _, _) => {
            h3394
        }
    }, f570(f689(match v0.clone() {
        Adt26::C0(_, h3395, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3395
        }
    }), match v0 {
        Adt26::C0(h3396, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3396
        }
    }))))
}

pub fn f654(v0: Fn21, p3400: u64, p3401: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3399 = (p3400, p3401);
    {
        let mut m3399 = m3399;
        loop {
            let step = {
                let (v1, v2) = m3399;
                let m3397 = v1;
                if m3397 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3397 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3398: Fn21 = v0.clone();
                        c3398.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f655(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3402, _) = v0;
        h3402
    };
    let v2: u32 = {
        let (h3403, _) = {
            let (_, h3404) = v0;
            h3404
        };
        h3403
    };
    let v3: u32 = {
        let (_, h3405) = {
            let (_, h3406) = v0;
            h3406
        };
        h3405
    };
    ({
        let a3407 = v1;
        let a3408 = 2u64;
        let a3409 = 0u64;
        nat_quot(a3407, a3408, a3409)
    }, (f656(v2, 1u32), {
        let a3413 = {
            let a3410 = v1;
            let a3411 = 2u64;
            let a3412 = 0u64;
            nat_rem(a3410, a3411, a3412)
        };
        let a3414 = 1u64;
        if nat_eq(a3413, a3414) {
            let a3415 = v3;
            let a3416 = v2;
            fixed_u32::bit_or(a3415, a3416)
        } else {
            v3
        }
    }))
}

pub fn f656(v0: u32, v1: u32) -> u32 {
    let m3419 = {
        let a3417 = v0;
        let a3418 = v1;
        fixed_u32::shift_left(a3417, a3418)
    };
    match m3419 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f657(v0: Bytes, v1: Adt2) -> bool {
    let v2: Bytes = f606(v1);
    if f32(v0.clone(), f8(Str::lit("number"))) {
        if f32(v2.clone(), f8(Str::lit("integer"))) {
            true
        } else {
            f32(v2, f8(Str::lit("number")))
        }
    } else {
        f32(v0, v2)
    }
}

pub fn f658(v0: u64, v1: Bytes, v2: u64) -> R<Option<List<u64>>> {
    let m3420 = v0;
    if m3420 == 0 {
        Ok(Some(List::<u64>::nil()))
    } else {
        let v3 = m3420 - 1;
        let a3422 = {
            let a3421 = v1.clone();
            length_bytes(a3421)
        };
        let a3423 = v2;
        if nat_le(a3422, a3423) {
            Ok(Some(List::<u64>::nil()))
        } else {
            let m3424 = f690(v1.clone(), v2)?;
            match m3424 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v4) => {
                    let m3428 = f658(v3, v1, {
                        let a3426 = v2;
                        let a3427 = {
                            let (_, h3425) = v4;
                            h3425
                        };
                        nat_add(a3426, a3427)?
                    })?;
                    match m3428 {
                        None => {
                            Ok(None::<List<u64>>)
                        }
                        Some(v5) => {
                            Ok(Some(List::cons({
                                let (h3429, _) = v4;
                                h3429
                            }, v5)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f659(v0: Bytes) -> R<Result<((bool, bool), List<Adt30>), Bytes>> {
    let v1: u64 = {
        let a3430 = v0.clone();
        length_bytes(a3430)
    };
    let v2: bool = {
        let a3431 = 0u64;
        let a3432 = v1;
        if nat_lt(a3431, a3432) {
            let a3433 = f116(v0.clone(), 0u64)?;
            let a3434 = 94u64;
            nat_eq(a3433, a3434)
        } else {
            false
        }
    };
    let v3: bool = {
        let a3435 = 0u64;
        let a3436 = v1;
        if nat_lt(a3435, a3436) {
            f118({
                let a3439 = f116(v0.clone(), {
                    let a3437 = v1;
                    let a3438 = 1u64;
                    nat_sub(a3437, a3438)
                })?;
                let a3440 = 36u64;
                nat_eq(a3439, a3440)
            }, f691({
                let a3443 = f116(v0.clone(), {
                    let a3441 = v1;
                    let a3442 = 2u64;
                    nat_sub(a3441, a3442)
                })?;
                let a3444 = 92u64;
                nat_eq(a3443, a3444)
            }))
        } else {
            false
        }
    };
    let m3449 = f692({
        let a3445 = v1;
        let a3446 = 1u64;
        nat_add(a3445, a3446)?
    }, v0, if v2 {
        1u64
    } else {
        0u64
    }, if v3 {
        let a3447 = v1;
        let a3448 = 1u64;
        nat_sub(a3447, a3448)
    } else {
        v1
    })?;
    match m3449 {
        Ok(v5) => {
            Ok(Ok::<((bool, bool), List<Adt30>), Bytes>(((v2, v3), v5)))
        }
        Err(v4) => {
            Ok(Err::<((bool, bool), List<Adt30>), Bytes>(v4))
        }
    }
}

pub fn f660(v0: u64, v1: List<Adt30>, v2: List<u64>, v3: bool) -> R<bool> {
    let m3450 = v0;
    if m3450 == 0 {
        Ok(false)
    } else {
        let v4 = m3450 - 1;
        let m3451 = v1.clone();
        match m3451.uncons() {
            None => {
                if v3 {
                    let a3452 = f611(v2)?;
                    let a3453 = 0u64;
                    Ok(nat_eq(a3452, a3453))
                } else {
                    Ok(true)
                }
            }
            Some((v5, v6)) => {
                let m3454 = v5.clone();
                match m3454 {
                    Adt30::C0(v7, v8, v9) => {
                        let a3455 = v9;
                        let a3456 = 1u64;
                        if nat_eq(a3455, a3456) {
                            if f660(v4, v6, v2.clone(), v3)? {
                                Ok(true)
                            } else {
                                let m3457 = v2;
                                match m3457.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v10, v11)) => {
                                        if f693(v5, v10) {
                                            f660(v4, v1, v11, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a3458 = v9;
                            let a3459 = 2u64;
                            if nat_eq(a3458, a3459) {
                                let m3460 = v2;
                                match m3460.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v12, v13)) => {
                                        if f693(v5, v12) {
                                            f660(v4, List::cons(Adt30::C0(v7, v8, 1u64), v6), v13, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            } else {
                                let a3461 = v9;
                                let a3462 = 3u64;
                                if nat_eq(a3461, a3462) {
                                    if f660(v4, v6.clone(), v2.clone(), v3)? {
                                        Ok(true)
                                    } else {
                                        let m3463 = v2;
                                        match m3463.uncons() {
                                            None => {
                                                Ok(false)
                                            }
                                            Some((v14, v15)) => {
                                                if f693(v5, v14) {
                                                    f660(v4, v6, v15, v3)
                                                } else {
                                                    Ok(false)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let m3464 = v2;
                                    match m3464.uncons() {
                                        None => {
                                            Ok(false)
                                        }
                                        Some((v16, v17)) => {
                                            if f693(v5, v16) {
                                                f660(v4, v6, v17, v3)
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

pub fn f661(v0: List<Adt30>) -> R<u64> {
    let m3465 = v0;
    match m3465.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a3466 = f661(v2)?;
            let a3467 = 1u64;
            nat_add(a3466, a3467)
        }
    }
}

pub fn f662(v0: List<Adt30>, v1: List<u64>, v2: bool) -> R<bool> {
    if f660({
        let a3470 = {
            let a3468 = f661(v0.clone())?;
            let a3469 = f611(v1.clone())?;
            nat_add(a3468, a3469)?
        };
        let a3471 = 2u64;
        nat_add(a3470, a3471)?
    }, v0.clone(), v1.clone(), v2)? {
        Ok(true)
    } else {
        f694(v0, v1, v2)
    }
}

pub fn f663(v0: u64) -> R<u64> {
    let m3472 = v0;
    if m3472 == 0 {
        Ok(1u64)
    } else {
        let v1 = m3472 - 1;
        let a3473 = f663(v1)?;
        let a3474 = 10u64;
        nat_mul(a3473, a3474)
    }
}

pub fn f664(p3488: u64, v1: Bytes, p3489: u64, p3490: Bytes) -> R<Bytes> {
    let m3487 = (p3488, (p3489, p3490));
    {
        let mut m3487 = m3487;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3487;
                let m3475 = v0;
                if m3475 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3475 - 1;
                    let m3478 = {
                        let a3476 = v1.clone();
                        let a3477 = v2;
                        index_bytes(a3476, a3477)
                    };
                    match m3478 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3479 = v2;
                                let a3480 = 1u64;
                                nat_add(a3479, a3480)?
                            }, {
                                let a3485 = v3;
                                let a3486 = {
                                    let a3481 = v6;
                                    let a3482 = 126u64;
                                    if nat_eq(a3481, a3482) {
                                        f8(Str::lit("~0"))
                                    } else {
                                        let a3483 = v6;
                                        let a3484 = 47u64;
                                        if nat_eq(a3483, a3484) {
                                            f8(Str::lit("~1"))
                                        } else {
                                            f46(v1.clone(), v2, 1u64)
                                        }
                                    }
                                };
                                append_bytes(a3485, a3486)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f665(v0: Fn22, p3495: u64, p3496: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3494 = (p3495, p3496);
    {
        let mut m3494 = m3494;
        loop {
            let step = {
                let (v1, v2) = m3494;
                let m3491 = v1;
                if m3491 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3491 - 1;
                    let m3493 = {
                        let c3492: Fn22 = v0.clone();
                        c3492.apply(v2.clone())?
                    };
                    match m3493 {
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
                Ok(next) => { m3494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f666(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f695(v0)
}

pub fn f667(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3501 = {
            let a3499 = {
                let a3497 = v0.clone();
                length_list(a3497)
            };
            let a3500 = {
                let a3498 = v1.clone();
                length_list(a3498)
            };
            nat_add(a3499, a3500)?
        };
        let a3502 = 1u64;
        nat_add(a3501, a3502)?
    };
    Ok(f449({
        let (_, h3503) = {
            let (_, h3504) = {
                let (h3505, _) = f696(Fn23::F697, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3505
            };
            h3504
        };
        h3503
    }))
}

pub fn f668(v0: u64) -> Bytes {
    f46({
        let a3506 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3506)
    }, v0, 1u64)
}

pub fn f669(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3507 = v1;
    match m3507.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3508 = f698(v0, v3.clone())?;
            match m3508 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f537(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f52(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3509 = f669(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3509 {
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

pub fn f670(p3513: List<Bytes>) -> R<bool> {
    let m3512 = p3513;
    {
        let mut m3512 = m3512;
        loop {
            let step = {
                let v0 = m3512;
                let m3510 = v0;
                match m3510.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3511 = f699(v1)?;
                        match m3511 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f52(v3, f700(false, v2.clone())?) {
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
                Ok(next) => { m3512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f671(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m3514 = v0;
    match m3514.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f701(v1.clone(), {
                let (h3515, _) = v2.clone();
                h3515
            }) {
                List::cons({
                    let (h3516, _) = v2;
                    h3516
                }, f671(v3, v1))
            } else {
                f671(v3, v1)
            }
        }
    }
}

pub fn f672(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3517 = v2;
    match m3517.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3518 = f702(v4.clone())?;
            match m3518 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3519 = f703(v0, v6.clone())?;
                    match m3519 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f52(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3522 = f704(v4, {
                                    let a3520 = v1.clone();
                                    let a3521 = f379(v6)?;
                                    append_bytes(a3520, a3521)
                                })?;
                                match m3522 {
                                    Ok(v9) => {
                                        let m3523 = f672(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3523 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f566(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f637(v11.clone())), f525(f565(v9), f638(v11)))))
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

pub fn f673(v0: List<Adt2>) -> R<List<Bytes>> {
    let m3524 = v0;
    match m3524.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3525 = f702(v1)?;
            match m3525 {
                None => {
                    f673(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f673(v2)?))
                }
            }
        }
    }
}

pub fn f674(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f705(f670(f706(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f675(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3526 = f385(Some(v0));
    match m3526 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3527 = f628(v2)?;
            match m3527 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    Ok(f32(v3, v1))
                }
            }
        }
    }
}

pub fn f676(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m3528 = v2;
    match m3528.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3530 = f707(v0, {
                let (h3529, _) = v5.clone();
                h3529
            })?;
            match m3530 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3531, _) = v5;
                        h3531
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f52(v7.clone(), f708(v0, f616(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3536 = f641({
                            let (_, h3532) = v5.clone();
                            h3532
                        }, {
                            let a3534 = v1.clone();
                            let a3535 = f379({
                                let (h3533, _) = v5.clone();
                                h3533
                            })?;
                            append_bytes(a3534, a3535)
                        })?;
                        match m3536 {
                            Ok(v9) => {
                                let v10: Bytes = if f52({
                                    let (h3537, _) = v5;
                                    h3537
                                }, v3.clone()) {
                                    f566(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f566(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3538 = f676(v0, v1, v6, v3, v4)?;
                                match m3538 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f637(v12.clone())), f525(f565(v9), f638(v12)))))
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

pub fn f677(v0: Adt2) -> Option<Adt2> {
    let m3539 = f276(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3539 {
        None => {
            f709(v0)
        }
        Some(v1) => {
            if f678(v1.clone()) {
                let m3540 = f709(v0);
                match m3540 {
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

pub fn f678(v0: Adt2) -> bool {
    let m3541 = v0;
    match m3541 {
        Adt2::C0 => {
            false
        }
        Adt2::C1(_, _) => {
            false
        }
        Adt2::C2(v3) => {
            !v3
        }
        Adt2::C3(_, _) => {
            false
        }
        Adt2::C4(_, _, _) => {
            false
        }
        Adt2::C5(_) => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8 => {
            false
        }
        Adt2::C9(_, _) => {
            false
        }
        Adt2::C10(_, _, _) => {
            false
        }
    }
}

pub fn f679(v0: Adt2) -> Option<Adt2> {
    let m3542 = f281(f276(v0, f8(Str::lit("oneOf"))));
    match m3542.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3543 = v2;
            match m3543.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m3544 = v4;
                    match m3544.uncons() {
                        None => {
                            if f710(v3.clone()) {
                                Some(v1)
                            } else {
                                if f710(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt2>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt2>
                        }
                    }
                }
            }
        }
    }
}

pub fn f680(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3545 = v1;
    match m3545 {
        Ok(v3) => {
            f632(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f566(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f565(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f681(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3546 = f679(v0.clone());
    match m3546 {
        None => {
            if f32(f279(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3547 = f276(v0, f8(Str::lit("items")));
                match m3547 {
                    None => {
                        Ok(f680(f8(Str::lit("List")), f682()))
                    }
                    Some(v3) => {
                        Ok(f680(f8(Str::lit("List")), f704(v3, {
                            let a3548 = v1;
                            let a3549 = f8(Str::lit("Item"));
                            append_bytes(a3548, a3549)
                        })?))
                    }
                }
            } else {
                f704(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f680(f8(Str::lit("Option")), f704(v2, v1)?))
        }
    }
}

pub fn f682() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f562(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f683(p3553: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m3552 = p3553;
    {
        let mut m3552 = m3552;
        loop {
            let step = {
                let v0 = m3552;
                let m3550 = v0;
                match m3550.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f52({
                            let (h3551, _) = v2;
                            h3551
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f684(p3557: List<Adt2>, v1: Bytes, p3558: List<Bytes>) -> bool {
    let m3556 = (p3557, p3558);
    {
        let mut m3556 = m3556;
        loop {
            let step = {
                let (v0, v2) = m3556;
                let m3554 = v0;
                match m3554.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3555 = f711(v3, v1.clone());
                        match m3555 {
                            None => {
                                Err::<(List<Adt2>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f52(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt2>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt2>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3556 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f685(p3561: List<Adt2>, p3562: List<Bytes>) -> bool {
    let m3560 = (p3561, p3562);
    {
        let mut m3560 = m3560;
        loop {
            let step = {
                let (v0, v1) = m3560;
                let m3559 = v0;
                match m3559.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f279(v2, f8(Str::lit("type")));
                        if !f32(v4.clone(), Bytes::lit(&[])) {
                            if !f52(v4.clone(), v1.clone()) {
                                if !if if f32(v4.clone(), f8(Str::lit("integer"))) {
                                    f52(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f32(v4.clone(), f8(Str::lit("number"))) {
                                        f52(f8(Str::lit("integer")), v1.clone())
                                    } else {
                                        false
                                    }
                                } {
                                    Ok::<(List<Adt2>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt2>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt2>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt2>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m3563 = f276(v0.clone(), f8(Str::lit("anyOf")));
        match m3563 {
            None => {
                Adt2::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f683(f558(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f642(f386(v2))
    } else {
        false
    }
}

pub fn f687(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3564 = f531(v0, v1);
    let a3565 = f656(v0, v2);
    fixed_u32::bit_or(a3564, a3565)
}

pub fn f688(v0: u32) -> u32 {
    let a3568 = {
        let a3566 = f687(v0, 17u32, 15u32);
        let a3567 = f687(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3566, a3567)
    };
    let a3569 = f531(v0, 10u32);
    fixed_u32::bit_xor(a3568, a3569)
}

pub fn f689(v0: u32) -> u32 {
    let a3572 = {
        let a3570 = f687(v0, 7u32, 25u32);
        let a3571 = f687(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3570, a3571)
    };
    let a3573 = f531(v0, 3u32);
    fixed_u32::bit_xor(a3572, a3573)
}

pub fn f690(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3576 = {
        let a3574 = v0.clone();
        let a3575 = v1;
        index_bytes(a3574, a3575)
    };
    match m3576 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            let a3577 = v3;
            let a3578 = 128u64;
            if nat_lt(a3577, a3578) {
                Ok(Some((v3, 1u64)))
            } else {
                if f118({
                    let a3579 = 194u64;
                    let a3580 = v3;
                    nat_le(a3579, a3580)
                }, {
                    let a3581 = v3;
                    let a3582 = 223u64;
                    nat_le(a3581, a3582)
                }) {
                    let m3585 = f712(v0, {
                        let a3583 = v1;
                        let a3584 = 1u64;
                        nat_add(a3583, a3584)?
                    })?;
                    match m3585 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f713({
                                let a3590 = {
                                    let a3588 = {
                                        let a3586 = v3;
                                        let a3587 = 192u64;
                                        nat_sub(a3586, a3587)
                                    };
                                    let a3589 = 64u64;
                                    nat_mul(a3588, a3589)?
                                };
                                let a3591 = v4;
                                nat_add(a3590, a3591)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f118({
                        let a3592 = 224u64;
                        let a3593 = v3;
                        nat_le(a3592, a3593)
                    }, {
                        let a3594 = v3;
                        let a3595 = 239u64;
                        nat_le(a3594, a3595)
                    }) {
                        let m3598 = f712(v0.clone(), {
                            let a3596 = v1;
                            let a3597 = 1u64;
                            nat_add(a3596, a3597)?
                        })?;
                        match m3598 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3601 = f712(v0, {
                                    let a3599 = v1;
                                    let a3600 = 2u64;
                                    nat_add(a3599, a3600)?
                                })?;
                                match m3601 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f713({
                                            let a3610 = {
                                                let a3604 = {
                                                    let a3602 = v3;
                                                    let a3603 = 224u64;
                                                    nat_sub(a3602, a3603)
                                                };
                                                let a3605 = 4096u64;
                                                nat_mul(a3604, a3605)?
                                            };
                                            let a3611 = {
                                                let a3608 = {
                                                    let a3606 = v5;
                                                    let a3607 = 64u64;
                                                    nat_mul(a3606, a3607)?
                                                };
                                                let a3609 = v6;
                                                nat_add(a3608, a3609)?
                                            };
                                            nat_add(a3610, a3611)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f118({
                            let a3612 = 240u64;
                            let a3613 = v3;
                            nat_le(a3612, a3613)
                        }, {
                            let a3614 = v3;
                            let a3615 = 244u64;
                            nat_le(a3614, a3615)
                        }) {
                            let m3618 = f712(v0.clone(), {
                                let a3616 = v1;
                                let a3617 = 1u64;
                                nat_add(a3616, a3617)?
                            })?;
                            match m3618 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3621 = f712(v0.clone(), {
                                        let a3619 = v1;
                                        let a3620 = 2u64;
                                        nat_add(a3619, a3620)?
                                    })?;
                                    match m3621 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3624 = f712(v0, {
                                                let a3622 = v1;
                                                let a3623 = 3u64;
                                                nat_add(a3622, a3623)?
                                            })?;
                                            match m3624 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f713({
                                                        let a3637 = {
                                                            let a3627 = {
                                                                let a3625 = v3;
                                                                let a3626 = 240u64;
                                                                nat_sub(a3625, a3626)
                                                            };
                                                            let a3628 = 262144u64;
                                                            nat_mul(a3627, a3628)?
                                                        };
                                                        let a3638 = {
                                                            let a3635 = {
                                                                let a3629 = v7;
                                                                let a3630 = 4096u64;
                                                                nat_mul(a3629, a3630)?
                                                            };
                                                            let a3636 = {
                                                                let a3633 = {
                                                                    let a3631 = v8;
                                                                    let a3632 = 64u64;
                                                                    nat_mul(a3631, a3632)?
                                                                };
                                                                let a3634 = v9;
                                                                nat_add(a3633, a3634)?
                                                            };
                                                            nat_add(a3635, a3636)?
                                                        };
                                                        nat_add(a3637, a3638)?
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

pub fn f691(v0: bool) -> bool {
    !v0
}

pub fn f692(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Result<List<Adt30>, Bytes>> {
    let m3639 = v0;
    if m3639 == 0 {
        Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
    } else {
        let v4 = m3639 - 1;
        let a3640 = v3;
        let a3641 = v2;
        if nat_le(a3640, a3641) {
            Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
        } else {
            let v5: u64 = f116(v1.clone(), v2)?;
            let m3690 = {
                let a3642 = v5;
                let a3643 = 91u64;
                if nat_eq(a3642, a3643) {
                    let a3646 = f116(v1.clone(), {
                        let a3644 = v2;
                        let a3645 = 1u64;
                        nat_add(a3644, a3645)?
                    })?;
                    let a3647 = 94u64;
                    if nat_eq(a3646, a3647) {
                        let m3651 = f714({
                            let a3648 = v1.clone();
                            length_bytes(a3648)
                        }, v1.clone(), {
                            let a3649 = v2;
                            let a3650 = 2u64;
                            nat_add(a3649, a3650)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3651 {
                            Ok(v7) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, {
                                    let (h3652, _) = v7;
                                    h3652
                                }), {
                                    let (_, h3653) = v7;
                                    h3653
                                }))
                            }
                            Err(v6) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v6)
                            }
                        }
                    } else {
                        let m3657 = f714({
                            let a3654 = v1.clone();
                            length_bytes(a3654)
                        }, v1.clone(), {
                            let a3655 = v2;
                            let a3656 = 1u64;
                            nat_add(a3655, a3656)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3657 {
                            Ok(v9) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, {
                                    let (h3658, _) = v9;
                                    h3658
                                }), {
                                    let (_, h3659) = v9;
                                    h3659
                                }))
                            }
                            Err(v8) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v8)
                            }
                        }
                    }
                } else {
                    let a3660 = v5;
                    let a3661 = 46u64;
                    if nat_eq(a3660, a3661) {
                        Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, List::cons((10u64, 10u64), List::cons((13u64, 13u64), List::cons((8232u64, 8233u64), List::<(u64, u64)>::nil())))), {
                            let a3662 = v2;
                            let a3663 = 1u64;
                            nat_add(a3662, a3663)?
                        }))
                    } else {
                        let a3664 = v5;
                        let a3665 = 40u64;
                        if if if if if if nat_eq(a3664, a3665) {
                            true
                        } else {
                            let a3666 = v5;
                            let a3667 = 41u64;
                            nat_eq(a3666, a3667)
                        } {
                            true
                        } else {
                            let a3668 = v5;
                            let a3669 = 124u64;
                            if nat_eq(a3668, a3669) {
                                true
                            } else {
                                let a3670 = v5;
                                let a3671 = 123u64;
                                nat_eq(a3670, a3671)
                            }
                        } {
                            true
                        } else {
                            let a3672 = v5;
                            let a3673 = 125u64;
                            if if nat_eq(a3672, a3673) {
                                true
                            } else {
                                let a3674 = v5;
                                let a3675 = 92u64;
                                nat_eq(a3674, a3675)
                            } {
                                true
                            } else {
                                let a3676 = v5;
                                let a3677 = 94u64;
                                if nat_eq(a3676, a3677) {
                                    true
                                } else {
                                    let a3678 = v5;
                                    let a3679 = 36u64;
                                    nat_eq(a3678, a3679)
                                }
                            }
                        } {
                            true
                        } else {
                            let a3680 = v5;
                            let a3681 = 42u64;
                            if if nat_eq(a3680, a3681) {
                                true
                            } else {
                                let a3682 = v5;
                                let a3683 = 43u64;
                                nat_eq(a3682, a3683)
                            } {
                                true
                            } else {
                                let a3684 = v5;
                                let a3685 = 63u64;
                                nat_eq(a3684, a3685)
                            }
                        } {
                            true
                        } else {
                            let a3686 = 127u64;
                            let a3687 = v5;
                            nat_lt(a3686, a3687)
                        } {
                            Err::<((bool, List<(u64, u64)>), u64), Bytes>(f7(List::cons(f8(Str::lit("the pattern construct ")), List::cons(f46(v1.clone(), v2, 1u64), List::cons(f8(Str::lit(" is not evaluated: only ^, $, ., character classes, literal ASCII characters and the quantifiers *, + and ? are (ECMA-262 section 22.2.1)")), List::<Bytes>::nil())))))
                        } else {
                            Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, List::cons((v5, v5), List::<(u64, u64)>::nil())), {
                                let a3688 = v2;
                                let a3689 = 1u64;
                                nat_add(a3688, a3689)?
                            }))
                        }
                    }
                }
            };
            match m3690 {
                Ok(v11) => {
                    let v12: u64 = {
                        let (_, h3691) = v11.clone();
                        h3691
                    };
                    let v13: u64 = {
                        let a3692 = v12;
                        let a3693 = v3;
                        if nat_lt(a3692, a3693) {
                            f715(f116(v1.clone(), v12)?)
                        } else {
                            0u64
                        }
                    };
                    let a3694 = v12;
                    let a3695 = v3;
                    if if nat_lt(a3694, a3695) {
                        let a3696 = f116(v1.clone(), v12)?;
                        let a3697 = 123u64;
                        nat_eq(a3696, a3697)
                    } else {
                        false
                    } {
                        Ok(Err::<List<Adt30>, Bytes>(f8(Str::lit("a {n,m} quantifier in a pattern is not evaluated (ECMA-262 section 22.2.1)"))))
                    } else {
                        let m3702 = f692(v4, v1, {
                            let a3698 = 0u64;
                            let a3699 = v13;
                            if nat_lt(a3698, a3699) {
                                let a3700 = v12;
                                let a3701 = 1u64;
                                nat_add(a3700, a3701)?
                            } else {
                                v12
                            }
                        }, v3)?;
                        match m3702 {
                            Ok(v15) => {
                                Ok(Ok::<List<Adt30>, Bytes>(List::cons(Adt30::C0({
                                    let (h3703, _) = {
                                        let (h3704, _) = v11.clone();
                                        h3704
                                    };
                                    h3703
                                }, {
                                    let (_, h3705) = {
                                        let (h3706, _) = v11;
                                        h3706
                                    };
                                    h3705
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

pub fn f693(v0: Adt30, v1: u64) -> bool {
    let m3707 = v0;
    match m3707 {
        Adt30::C0(v2, v3, _) => {
            if v2 {
                f691(f716(v3, v1))
            } else {
                f716(v3, v1)
            }
        }
    }
}

pub fn f694(v0: List<Adt30>, p3714: List<u64>, v2: bool) -> R<bool> {
    let m3713 = p3714;
    {
        let mut m3713 = m3713;
        loop {
            let step = {
                let v1 = m3713;
                let m3708 = v1;
                match m3708.uncons() {
                    None => {
                        Err::<List<u64>, R<bool>>(Ok(false))
                    }
                    Some((_, v4)) => {
                        if f660({
                            let a3711 = {
                                let a3709 = f661(v0.clone())?;
                                let a3710 = f611(v4.clone())?;
                                nat_add(a3709, a3710)?
                            };
                            let a3712 = 2u64;
                            nat_add(a3711, a3712)?
                        }, v0.clone(), v4.clone(), v2)? {
                            Err::<List<u64>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<u64>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3713 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f695(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3715, _) = v0;
        h3715
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3716) = v0;
        h3716
    };
    let m3717 = v1;
    match m3717.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3718 = v4;
            match m3718.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f717(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f696(v0: Fn23, p3723: u64, p3724: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3722 = (p3723, p3724);
    {
        let mut m3722 = m3722;
        loop {
            let step = {
                let (v1, v2) = m3722;
                let m3719 = v1;
                if m3719 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3719 - 1;
                    let m3721 = {
                        let c3720: Fn23 = v0.clone();
                        c3720.apply(v2.clone())
                    };
                    match m3721 {
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
                Ok(next) => { m3722 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f697(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f718(v0)
}

pub fn f698(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3725 = v1;
    match m3725 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C3(v7, v8) => {
            Ok(Some({
                let a3736 = if f118(v7, {
                    let a3734 = 0u64;
                    let a3735 = v8;
                    nat_lt(a3734, a3735)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3737 = f303(false, v8)?;
                append_bytes(a3736, a3737)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3728 = f719({
                let a3726 = f8(Str::lit("n"));
                let a3727 = f468(false, v4, v5)?;
                append_bytes(a3726, a3727)
            })?;
            match m3728 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3732 = f8(Str::lit("nMinus"));
                        let a3733 = f46(v6.clone(), 1u64, {
                            let a3730 = {
                                let a3729 = v6;
                                length_bytes(a3729)
                            };
                            let a3731 = 1u64;
                            nat_sub(a3730, a3731)
                        });
                        append_bytes(a3732, a3733)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f703(v0, v2)
        }
        Adt2::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C8 => {
            Ok(None::<Bytes>)
        }
        Adt2::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f699(v0: Bytes) -> R<Option<Bytes>> {
    let m3738 = f720(v0)?;
    match m3738 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f721(v1)?))
        }
    }
}

pub fn f700(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3739 = v1;
    match m3739.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3740 = f703(v0, v2)?;
            match m3740 {
                None => {
                    f700(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f700(v0, v3)?))
                }
            }
        }
    }
}

pub fn f701(p3744: List<Adt2>, v1: Bytes) -> bool {
    let m3743 = p3744;
    {
        let mut m3743 = m3743;
        loop {
            let step = {
                let v0 = m3743;
                let m3741 = v0;
                match m3741.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3742 = f711(v2, v1.clone());
                        if match m3742 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt2>, bool>(v3)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f702(v0: Adt2) -> R<Option<Bytes>> {
    let m3745 = f385(f276(v0.clone(), f8(Str::lit("$ref"))));
    match m3745 {
        None => {
            Ok(f385(f276(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f628(v1)
        }
    }
}

pub fn f703(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f719(v1)
    } else {
        f699(v1)
    }
}

pub fn f704(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f722(f598(v0), v1)
}

pub fn f705(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3746 = v2;
    match m3746.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3747 = f711(v5.clone(), v3.clone());
            match m3747 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3748 = f703(v0, v7.clone())?;
                    match m3748 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f52(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3751 = f635({
                                    let a3749 = v1.clone();
                                    let a3750 = f379(v7)?;
                                    append_bytes(a3749, a3750)
                                }, f560(f521(f276(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f280(f281(f276(v5, f8(Str::lit("required"))))), true)?;
                                match m3751 {
                                    Ok(v10) => {
                                        let m3752 = f705(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3752 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f395(f637(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f637(v12.clone())), f525(f638(v10), f638(v12)))))
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

pub fn f706(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3753 = v0;
    match m3753.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3754 = f711(v2, v1.clone());
            match m3754 {
                None => {
                    f706(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f706(v3, v1))
                }
            }
        }
    }
}

pub fn f707(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3755 = f703(v0, v1)?;
    match m3755 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f32(v2.clone(), f8(Str::lit("mk"))) {
                f596(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f708(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3756 = v1;
    match m3756.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3757 = f707(v0, v2)?;
            match m3757 {
                None => {
                    f708(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f708(v0, v3)?))
                }
            }
        }
    }
}

pub fn f709(v0: Adt2) -> Option<Adt2> {
    let m3758 = f521(f276(v0, f8(Str::lit("patternProperties"))));
    match m3758.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3759 = v2;
            match m3759.uncons() {
                None => {
                    Some({
                        let (_, h3760) = v1;
                        h3760
                    })
                }
                Some((_, _)) => {
                    None::<Adt2>
                }
            }
        }
    }
}

pub fn f710(v0: Adt2) -> bool {
    f32(f279(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f711(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f52(v1.clone(), f280(f281(f276(v0.clone(), f8(Str::lit("required")))))) {
        let m3761 = f276(v0, f8(Str::lit("properties")));
        match m3761 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3762 = f276(v2, v1);
                match m3762 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f385(f276(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f712(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3765 = {
        let a3763 = v0;
        let a3764 = v1;
        index_bytes(a3763, a3764)
    };
    match m3765 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f118({
                let a3766 = 128u64;
                let a3767 = v3;
                nat_le(a3766, a3767)
            }, {
                let a3768 = v3;
                let a3769 = 191u64;
                nat_le(a3768, a3769)
            }) {
                Ok(Some({
                    let a3770 = v3;
                    let a3771 = 128u64;
                    nat_sub(a3770, a3771)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f713(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f118({
        let a3772 = v1;
        let a3773 = v0;
        nat_le(a3772, a3773)
    }, f118({
        let a3774 = v0;
        let a3775 = v2;
        nat_le(a3774, a3775)
    }, f203({
        let a3776 = v0;
        let a3777 = 55296u64;
        nat_lt(a3776, a3777)
    }, {
        let a3778 = 57343u64;
        let a3779 = v0;
        nat_lt(a3778, a3779)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f714(p3816: u64, v1: Bytes, p3817: u64, p3818: List<(u64, u64)>) -> R<Result<(List<(u64, u64)>, u64), Bytes>> {
    let m3815 = (p3816, (p3817, p3818));
    {
        let mut m3815 = m3815;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3815;
                let m3780 = v0;
                if m3780 == 0 {
                    Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                } else {
                    let v4 = m3780 - 1;
                    let v5: u64 = f116(v1.clone(), v2)?;
                    let a3782 = {
                        let a3781 = v1.clone();
                        length_bytes(a3781)
                    };
                    let a3783 = v2;
                    if nat_le(a3782, a3783) {
                        Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                    } else {
                        let a3784 = v5;
                        let a3785 = 93u64;
                        if nat_eq(a3784, a3785) {
                            Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Ok::<(List<(u64, u64)>, u64), Bytes>((v3, {
                                let a3786 = v2;
                                let a3787 = 1u64;
                                nat_add(a3786, a3787)?
                            }))))
                        } else {
                            let a3788 = v5;
                            let a3789 = 92u64;
                            if if nat_eq(a3788, a3789) {
                                true
                            } else {
                                let a3790 = 127u64;
                                let a3791 = v5;
                                nat_lt(a3790, a3791)
                            } {
                                Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("an escape or a non-ASCII character in a pattern's character class is not evaluated (ECMA-262 section 22.2.1)")))))
                            } else {
                                let a3794 = f116(v1.clone(), {
                                    let a3792 = v2;
                                    let a3793 = 1u64;
                                    nat_add(a3792, a3793)?
                                })?;
                                let a3795 = 45u64;
                                if if nat_eq(a3794, a3795) {
                                    f118({
                                        let a3799 = {
                                            let a3796 = v2;
                                            let a3797 = 2u64;
                                            nat_add(a3796, a3797)?
                                        };
                                        let a3800 = {
                                            let a3798 = v1.clone();
                                            length_bytes(a3798)
                                        };
                                        nat_lt(a3799, a3800)
                                    }, f118(f691({
                                        let a3803 = f116(v1.clone(), {
                                            let a3801 = v2;
                                            let a3802 = 2u64;
                                            nat_add(a3801, a3802)?
                                        })?;
                                        let a3804 = 93u64;
                                        nat_eq(a3803, a3804)
                                    }), f691({
                                        let a3807 = f116(v1.clone(), {
                                            let a3805 = v2;
                                            let a3806 = 2u64;
                                            nat_add(a3805, a3806)?
                                        })?;
                                        let a3808 = 92u64;
                                        nat_eq(a3807, a3808)
                                    })))
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3809 = v2;
                                        let a3810 = 3u64;
                                        nat_add(a3809, a3810)?
                                    }, List::cons((v5, f116(v1.clone(), {
                                        let a3811 = v2;
                                        let a3812 = 2u64;
                                        nat_add(a3811, a3812)?
                                    })?), v3))))
                                } else {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3813 = v2;
                                        let a3814 = 1u64;
                                        nat_add(a3813, a3814)?
                                    }, List::cons((v5, v5), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f715(v0: u64) -> u64 {
    let a3819 = v0;
    let a3820 = 42u64;
    if nat_eq(a3819, a3820) {
        1u64
    } else {
        let a3821 = v0;
        let a3822 = 43u64;
        if nat_eq(a3821, a3822) {
            2u64
        } else {
            let a3823 = v0;
            let a3824 = 63u64;
            if nat_eq(a3823, a3824) {
                3u64
            } else {
                0u64
            }
        }
    }
}

pub fn f716(p3833: List<(u64, u64)>, v1: u64) -> bool {
    let m3832 = p3833;
    {
        let mut m3832 = m3832;
        loop {
            let step = {
                let v0 = m3832;
                let m3825 = v0;
                match m3825.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3827 = {
                            let (h3826, _) = v2;
                            h3826
                        };
                        let a3828 = v1;
                        if if nat_le(a3827, a3828) {
                            let a3830 = v1;
                            let a3831 = {
                                let (_, h3829) = v2;
                                h3829
                            };
                            nat_le(a3830, a3831)
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
                Ok(next) => { m3832 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f717(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3838 = {
            let a3836 = {
                let a3834 = v0.clone();
                length_list(a3834)
            };
            let a3837 = {
                let a3835 = v1.clone();
                length_list(a3835)
            };
            nat_add(a3836, a3837)?
        };
        let a3839 = 1u64;
        nat_add(a3838, a3839)?
    };
    Ok(f723({
        let (_, h3840) = {
            let (_, h3841) = {
                let (h3842, _) = f724(Fn24::F725, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3842
            };
            h3841
        };
        h3840
    }))
}

pub fn f718(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3843, _) = v0.clone();
        h3843
    };
    let v2: List<(Bytes, ())> = {
        let (h3844, _) = {
            let (_, h3845) = v0.clone();
            h3845
        };
        h3844
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3846) = {
            let (_, h3847) = v0;
            h3847
        };
        h3846
    };
    let m3848 = v1.clone();
    match m3848.uncons() {
        None => {
            let m3849 = v2;
            match m3849.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3850 = v2.clone();
            match m3850.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f726({
                        let (h3851, _) = v8.clone();
                        h3851
                    }, {
                        let (h3852, _) = v6.clone();
                        h3852
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

pub fn f719(v0: Bytes) -> R<Option<Bytes>> {
    let m3854 = f727({
        let a3853 = v0.clone();
        length_bytes(a3853)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3854 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3856 = {
                let a3855 = v1.clone();
                length_bytes(a3855)
            };
            let a3857 = 0u64;
            if nat_eq(a3856, a3857) {
                let a3859 = {
                    let a3858 = v0;
                    length_bytes(a3858)
                };
                let a3860 = 0u64;
                if nat_eq(a3859, a3860) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f721(v1)?))
            }
        }
    }
}

pub fn f720(v0: Bytes) -> R<Option<Bytes>> {
    if f728({
        let a3861 = v0.clone();
        length_bytes(a3861)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f729(v0)?;
        if f730(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3864 = {
                let a3862 = v1.clone();
                let a3863 = 0u64;
                index_bytes(a3862, a3863)
            };
            match m3864 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f519(f72(v2)?) {
                        Ok(Some({
                            let a3865 = f8(Str::lit("n"));
                            let a3866 = v1;
                            append_bytes(a3865, a3866)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f731({
            let a3867 = v0.clone();
            length_bytes(a3867)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f721(v0: Bytes) -> R<Bytes> {
    if f52(v0.clone(), f732()) {
        f596(v0)
    } else {
        Ok(v0)
    }
}

pub fn f722(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3868 = v0.clone();
    match m3868 {
        Adt2::C0 => {
            Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C1(_, _) => {
            Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(v4) => {
            if v4 {
                Ok(f682())
            } else {
                Ok(f631(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C3(_, _) => {
            Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_, _, _) => {
            Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_) => {
            Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C6(_) => {
            Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C7(_) => {
            let m3869 = f733(f558(v0.clone()));
            match m3869 {
                None => {
                    let m3870 = f385(f276(v0.clone(), f8(Str::lit("$ref"))));
                    match m3870 {
                        None => {
                            let m3872 = f276(v0.clone(), f8(Str::lit("allOf")));
                            match m3872 {
                                None => {
                                    let m3877 = f276(v0.clone(), f8(Str::lit("enum")));
                                    match m3877 {
                                        None => {
                                            let m3878 = f276(v0.clone(), f8(Str::lit("const")));
                                            match m3878 {
                                                None => {
                                                    let m3880 = f276(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3880 {
                                                        None => {
                                                            let v32: Bytes = f279(v0.clone(), f8(Str::lit("type")));
                                                            if if f32(v32.clone(), Bytes::lit(&[])) {
                                                                !f734(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f682())
                                                            } else {
                                                                if f32(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f632(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f32(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f632(f735(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f32(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f632(f736(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f32(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f632(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f32(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3881 = f521(f276(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3881.uncons() {
                                                                                        None => {
                                                                                            if f639(v0.clone()) {
                                                                                                Ok(f632(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3882 = f677(v0);
                                                                                                match m3882 {
                                                                                                    None => {
                                                                                                        Ok(f680(f8(Str::lit("List")), f737(f682())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f680(f8(Str::lit("List")), f737(f738(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f632(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f631(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f632(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3879 = f385(Some(v29));
                                                    match m3879 {
                                                        None => {
                                                            Ok(f631(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f632(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f632(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3873 = f386(v21);
                                    match m3873.uncons() {
                                        None => {
                                            Ok(f631(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3874 = v23;
                                            match m3874.uncons() {
                                                None => {
                                                    let m3875 = f385(f276(v22, f8(Str::lit("$ref"))));
                                                    match m3875 {
                                                        None => {
                                                            Ok(f631(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3876 = f628(v26)?;
                                                            match m3876 {
                                                                None => {
                                                                    Ok(f631(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f632(v27, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f631(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3871 = f628(v19)?;
                            match m3871 {
                                None => {
                                    Ok(f631(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f632(v20, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f631(if f32(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C8 => {
            Ok(f682())
        }
        Adt2::C9(_, _) => {
            Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C10(_, _, _) => {
            Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f723(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f407(Fn6::F739, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f724(v0: Fn24, p3887: u64, p3888: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3886 = (p3887, p3888);
    {
        let mut m3886 = m3886;
        loop {
            let step = {
                let (v1, v2) = m3886;
                let m3883 = v1;
                if m3883 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3883 - 1;
                    let m3885 = {
                        let c3884: Fn24 = v0.clone();
                        c3884.apply(v2.clone())
                    };
                    match m3885 {
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
                Ok(next) => { m3886 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f725(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f740(v0)
}

pub fn f726(v0: Bytes, v1: Bytes) -> bool {
    let a3893 = {
        let a3889 = v0;
        let a3890 = v1;
        compare_bytes(a3889, a3890)
    };
    let a3894 = {
        let a3891 = Bytes::lit(&[]);
        let a3892 = Bytes::lit(&[0x01]);
        compare_bytes(a3891, a3892)
    };
    equal(a3893, a3894)
}

pub fn f727(p3907: u64, v1: Bytes, p3908: u64, p3909: Bytes) -> R<Option<Bytes>> {
    let m3906 = (p3907, (p3908, p3909));
    {
        let mut m3906 = m3906;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3906;
                let m3895 = v0;
                if m3895 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3895 - 1;
                    let a3897 = {
                        let a3896 = v1.clone();
                        length_bytes(a3896)
                    };
                    let a3898 = v2;
                    if nat_le(a3897, a3898) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3899 = f690(v1.clone(), v2)?;
                        match m3899 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3901 = v2;
                                    let a3902 = {
                                        let (_, h3900) = v5;
                                        h3900
                                    };
                                    nat_add(a3901, a3902)?
                                }, {
                                    let a3904 = v3;
                                    let a3905 = f741(v1.clone(), v2, {
                                        let (h3903, _) = v5;
                                        h3903
                                    });
                                    append_bytes(a3904, a3905)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3906 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f728(p3919: u64, v1: Bytes, p3920: u64) -> R<bool> {
    let m3918 = (p3919, p3920);
    {
        let mut m3918 = m3918;
        loop {
            let step = {
                let (v0, v2) = m3918;
                let m3910 = v0;
                if m3910 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3910 - 1;
                    let m3913 = {
                        let a3911 = v1.clone();
                        let a3912 = v2;
                        index_bytes(a3911, a3912)
                    };
                    match m3913 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3914 = f72(v4)?;
                            let a3915 = 128u64;
                            if nat_lt(a3914, a3915) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3916 = v2;
                                    let a3917 = 1u64;
                                    nat_add(a3916, a3917)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3918 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f729(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f466(v0.clone(), 0u64, {
        let a3921 = v0;
        length_bytes(a3921)
    }, false, Bytes::lit(&[]))?;
    let m3924 = {
        let a3922 = v1.clone();
        let a3923 = 0u64;
        index_bytes(a3922, a3923)
    };
    match m3924 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f518(v3) {
                let a3931 = f46({
                    let a3925 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3925)
                }, {
                    let a3926 = v3;
                    let a3927 = 65u64;
                    nat_sub(a3926, a3927)
                }, 1u64);
                let a3932 = f46(v1.clone(), 1u64, {
                    let a3929 = {
                        let a3928 = v1;
                        length_bytes(a3928)
                    };
                    let a3930 = 1u64;
                    nat_sub(a3929, a3930)
                });
                Ok(append_bytes(a3931, a3932))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f730(v0: Bytes) -> R<bool> {
    let m3935 = {
        let a3933 = v0;
        let a3934 = 0u64;
        index_bytes(a3933, a3934)
    };
    match m3935 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f203(f517(f72(v1)?), f518(f72(v1)?)))
        }
    }
}

pub fn f731(p3948: u64, v1: Bytes, p3949: u64, p3950: Bytes) -> R<Option<Bytes>> {
    let m3947 = (p3948, (p3949, p3950));
    {
        let mut m3947 = m3947;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3947;
                let m3936 = v0;
                if m3936 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3936 - 1;
                    let a3938 = {
                        let a3937 = v1.clone();
                        length_bytes(a3937)
                    };
                    let a3939 = v2;
                    if nat_le(a3938, a3939) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3940 = f690(v1.clone(), v2)?;
                        match m3940 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3942 = v2;
                                    let a3943 = {
                                        let (_, h3941) = v5;
                                        h3941
                                    };
                                    nat_add(a3942, a3943)?
                                }, {
                                    let a3945 = v3;
                                    let a3946 = f742({
                                        let (h3944, _) = v5;
                                        h3944
                                    });
                                    append_bytes(a3945, a3946)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3947 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f732() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f733(p3955: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m3954 = p3955;
    {
        let mut m3954 = m3954;
        loop {
            let step = {
                let v0 = m3954;
                let m3951 = v0;
                match m3951.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h3952, _) = v1.clone();
                            h3952
                        }, f743()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h3953, _) = v1;
                                h3953
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f734(v0: Adt2) -> bool {
    if f644(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f644(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f644(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f644(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f644(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f644(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f735(v0: Adt2) -> Bytes {
    let m3956 = f612(f276(v0, f8(Str::lit("minimum"))));
    match m3956 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f736() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f737(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3957 = v0;
    match m3957 {
        Ok(v2) => {
            f632(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f566(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f565(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f738(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f744(16u64, v0, v1)
}

pub fn f739(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f740(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3958, _) = v0.clone();
        h3958
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3959, _) = {
            let (_, h3960) = v0.clone();
            h3960
        };
        h3959
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3961) = {
            let (_, h3962) = v0;
            h3962
        };
        h3961
    };
    let m3963 = v1.clone();
    match m3963.uncons() {
        None => {
            let m3964 = v2;
            match m3964.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3965 = v2.clone();
            match m3965.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f726({
                        let (h3966, _) = v8.clone();
                        h3966
                    }, {
                        let (h3967, _) = v6.clone();
                        h3967
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

pub fn f741(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f203(f118(f203(f517(v2), f518(v2)), f118({
        let a3968 = v2;
        let a3969 = 117u64;
        !nat_eq(a3968, a3969)
    }, {
        let a3970 = v2;
        let a3971 = 85u64;
        !nat_eq(a3970, a3971)
    })), f118(f519(v2), {
        let a3972 = 0u64;
        let a3973 = v1;
        nat_lt(a3972, a3973)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a3974 = v2;
        let a3975 = 65536u64;
        if nat_lt(a3974, a3975) {
            let a3976 = f8(Str::lit("u"));
            let a3977 = f629(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3976, a3977)
        } else {
            let a3978 = f8(Str::lit("U"));
            let a3979 = f629(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3978, a3979)
        }
    }
}

pub fn f742(v0: u64) -> Bytes {
    let a3980 = f8(Str::lit("u"));
    let a3981 = f629(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3980, a3981)
}

pub fn f743() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f744(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3982 = v0;
    if m3982 == 0 {
        Ok(f631(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3982 - 1;
        let m3983 = v1.clone();
        match m3983 {
            Adt2::C0 => {
                Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C1(_, _) => {
                Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(f682())
                } else {
                    Ok(f631(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C3(_, _) => {
                Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_, _, _) => {
                Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_) => {
                Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C6(_) => {
                Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C7(_) => {
                let m3984 = f385(f276(v1.clone(), f8(Str::lit("$ref"))));
                match m3984 {
                    None => {
                        if if f644(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f644(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f644(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f632(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v22: Bytes = f279(v1.clone(), f8(Str::lit("type")));
                            if if f32(v22.clone(), Bytes::lit(&[])) {
                                !f734(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f682())
                            } else {
                                if f32(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f632(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f32(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f632(f735(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f32(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f632(f736(), List::<Adt25>::nil()))
                                        } else {
                                            if f32(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f632(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f32(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3986 = f276(v1, f8(Str::lit("items")));
                                                    match m3986 {
                                                        None => {
                                                            Ok(f680(f8(Str::lit("List")), f682()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f680(f8(Str::lit("List")), f744(v3, v23, {
                                                                let a3987 = v2;
                                                                let a3988 = f8(Str::lit("Item"));
                                                                append_bytes(a3987, a3988)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v22.clone(), f8(Str::lit("object"))) {
                                                        if f639(v1.clone()) {
                                                            Ok(f632(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3989 = f677(v1);
                                                            match m3989 {
                                                                None => {
                                                                    Ok(f680(f8(Str::lit("List")), f737(f682())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f680(f8(Str::lit("List")), f737(f744(v3, v24, {
                                                                        let a3990 = v2;
                                                                        let a3991 = f8(Str::lit("Value"));
                                                                        append_bytes(a3990, a3991)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f631(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
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
                        let m3985 = f628(v20)?;
                        match m3985 {
                            None => {
                                Ok(f631(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f632(v21, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C8 => {
                Ok(f682())
            }
            Adt2::C9(_, _) => {
                Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C10(_, _, _) => {
                Ok(f631(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}
