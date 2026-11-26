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
    F186(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F77(k0, k1) => f77(*k0, k1.clone(), p0),
            Fn0::F186(k0) => f186(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F122,
    F237,
    F344,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F122 => f122(p0, p1),
            Fn1::F237 => f237(p0, p1),
            Fn1::F344 => f344(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F209(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F209(k0) => f209(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F313(Bytes),
    F314(Bytes),
    F408(Bytes),
    F409(u64, Bytes),
    F416(Bytes),
    F424(Bytes, u64, Bytes, u64),
    F487(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F313(k0) => f313(k0.clone(), p0),
            Fn3::F314(k0) => f314(k0.clone(), p0),
            Fn3::F408(k0) => Ok(f408(k0.clone(), p0)),
            Fn3::F409(k0, k1) => f409(*k0, k1.clone(), p0),
            Fn3::F416(k0) => f416(k0.clone(), p0),
            Fn3::F424(k0, k1, k2, k3) => f424(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F487(k0) => f487(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F317(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F317(k0) => f317(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F398(Bytes),
    F534(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F398(k0) => f398(k0.clone(), p0),
            Fn5::F534(k0, k1) => Ok(f534(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F400,
    F521,
    F730,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F400 => f400(p0, p1),
            Fn6::F521 => f521(p0, p1),
            Fn6::F730 => f730(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F403,
    F522,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F403 => f403(p0, p1),
            Fn7::F522 => f522(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F435,
}

impl Fn8 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn8::F435 => f435(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F438,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn9::F438 => f438(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F440,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn10::F440 => f440(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F443,
}

impl Fn11 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn11::F443 => f443(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F469(Bytes, u64, u64),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn12::F469(k0, k1, k2) => f469(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F498,
}

impl Fn13 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn13::F498 => f498(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F501,
}

impl Fn14 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn14::F501 => f501(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F538,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F538 => f538(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F540,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F540 => f540(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F543(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F543(k0) => f543(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F561,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn18::F561 => f561(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F579,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F579 => f579(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F583,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F583 => f583(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F646,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F646 => f646(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F657,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F657 => f657(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F688,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F688 => f688(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F716,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F716 => f716(p0),
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
                                if f32(v0.clone(), f8(Str::lit("record-decision"))) {
                                    f180(v3, v1)
                                } else {
                                    if f32(v0, f8(Str::lit("record-error"))) {
                                        f181(v3, v1, v2)
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

pub fn f88(v0: Bytes) -> R<bool> {
    f182(v0.clone(), 0u64, {
        let a482 = v0;
        length_bytes(a482)
    })
}

pub fn f89(p487: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m486 = p487;
    {
        let mut m486 = m486;
        loop {
            let step = {
                let v0 = m486;
                let m483 = v0;
                match m483.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h484, _) = v2;
                            h484
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h485) = v2;
                                h485
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m486 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m491 = {
        let a489 = match v0 {
            Adt1::C0(h488, _, _, _) => {
                h488
            }
        };
        let a490 = f40(v1);
        index_list(a489, a490)
    };
    match m491 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m492 = f183(f184(v0.clone(), v1.clone()), v1.clone());
    match m492 {
        None => {
            f43(f64(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f92(v0: Adt2) -> Option<Adt14> {
    let m493 = v0;
    match m493 {
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
        Adt2::C6(r500) => {
            let v10 = (*r500).clone();
            let m494 = f92(v10);
            match m494 {
                None => {
                    None::<Adt14>
                }
                Some(v11) => {
                    Some(Adt14::C3(std::rc::Rc::new(v11)))
                }
            }
        }
        Adt2::C7(r501) => {
            let v12 = (*r501).clone();
            let m495 = f92(v12);
            match m495 {
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
        Adt2::C9(r502, r503) => {
            let v14 = (*r502).clone();
            let v15 = (*r503).clone();
            let m496 = f92(v14);
            match m496 {
                None => {
                    None::<Adt14>
                }
                Some(v16) => {
                    let m497 = f92(v15);
                    match m497 {
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
        Adt2::C10(v18, r504, r505) => {
            let v19 = (*r504).clone();
            let v20 = (*r505).clone();
            let m498 = f92(v19);
            match m498 {
                None => {
                    None::<Adt14>
                }
                Some(v21) => {
                    let m499 = f92(v20);
                    match m499 {
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
    let m506 = v0;
    match m506 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m507 = v1;
            match m507 {
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

pub fn f94(p513: Adt14, v1: Bytes) -> Option<Adt14> {
    let m512 = p513;
    {
        let mut m512 = m512;
        loop {
            let step = {
                let v0 = m512;
                let m508 = v0;
                match m508 {
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
                    Adt14::C4(r509) => {
                        let v5 = (*r509).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r510, r511) => {
                        let v3 = (*r510).clone();
                        let v4 = (*r511).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m514 = v0;
    match m514.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m515 = f185(v2, v1.clone())?;
            match m515 {
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
        let a516 = v0.clone();
        length_bytes(a516)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h517, _) = f76(Fn0::F186(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h517
    };
    let v3: u64 = {
        let (h518, _) = {
            let (_, h519) = v2.clone();
            h519
        };
        h518
    };
    let v4: List<Bytes> = {
        let (_, h520) = {
            let (_, h521) = v2;
            h521
        };
        h520
    };
    let a522 = v3;
    let a523 = v1;
    if nat_lt(a522, a523) {
        Ok(f78(List::cons(f46(v0, v3, {
            let a524 = v1;
            let a525 = v3;
            nat_sub(a524, a525)
        }), v4)))
    } else {
        Ok(f78(v4))
    }
}

pub fn f97(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f187(f57(v0, v1)?))
}

pub fn f98(v0: List<Adt14>, v1: Bytes) -> List<Adt14> {
    let m526 = v0;
    match m526.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f188(v2, v1.clone()), f98(v3, v1))
        }
    }
}

pub fn f99(p531: List<Bytes>, p532: List<Bytes>) -> bool {
    let m530 = (p531, p532);
    {
        let mut m530 = m530;
        loop {
            let step = {
                let (v0, v1) = m530;
                let m527 = v0;
                match m527.uncons() {
                    None => {
                        let m528 = v1;
                        match m528.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m529 = v1;
                        match m529.uncons() {
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
                Ok(next) => { m530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100(v0: List<Adt14>) -> List<Bytes> {
    let m533 = v0;
    match m533.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m534 = f94(v1, f8(Str::lit("std")));
            match m534 {
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
    let a535 = f189(v0)?;
    let a536 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a535, a536))
}

pub fn f102(v0: List<Adt14>) -> Adt14 {
    let m537 = v0;
    match m537.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f102(v2)))
        }
    }
}

pub fn f103(v0: Adt13) -> Bytes {
    let m538 = v0;
    match m538 {
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
    let a540 = {
        let a539 = v2.clone();
        length_bytes(a539)
    };
    let a541 = 0u64;
    if nat_eq(a540, a541) {
        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("waiting engine: the release tag ")), List::cons(v1, List::cons(f8(Str::lit(" names no commit\u{a}")), List::<Bytes>::nil()))))))
    } else {
        let m542 = f190(v0.clone(), f60())?;
        match m542 {
            None => {
                if f51(f52(f193(), match v0.clone() {
                    Adt1::C0(_, _, h543, _) => {
                        h543
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f193()))
                } else {
                    let m544 = f56(v0.clone(), List::cons(f194(), f195(f60())?));
                    match m544.uncons() {
                        None => {
                            Ok(Adt12::C3(f196(v0.clone(), f60(), v1.clone(), v2.clone(), f197(f64(v0, f194()))?)?, f7(List::cons(f8(Str::lit("staged the verdant-engine ")), List::cons(v1, List::cons(f8(Str::lit(" bindings at ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))
                        }
                        Some((v4, v5)) => {
                            Ok(Adt12::C0(List::cons(v4, v5)))
                        }
                    }
                }
            }
            Some(v3) => {
                Ok(f54(f191(v3.clone())?, f192(v3, v2)))
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

pub fn f115(v0: Bytes, p572: u64, p573: u64, p574: bool, p575: bool, p576: u64, p577: Bytes) -> R<Bytes> {
    let m571 = (p572, (p573, (p574, (p575, (p576, p577)))));
    {
        let mut m571 = m571;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m571;
                let m545 = v2;
                if m545 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f198(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m545 - 1;
                    let m548 = {
                        let a546 = v0.clone();
                        let a547 = v1;
                        index_bytes(a546, a547)
                    };
                    match m548 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f198(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f72(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a549 = v1;
                                    let a550 = 1u64;
                                    nat_add(a549, a550)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a551 = v9;
                                    let a552 = 34u64;
                                    !nat_eq(a551, a552)
                                }, (if v4 {
                                    false
                                } else {
                                    let a553 = v9;
                                    let a554 = 92u64;
                                    nat_eq(a553, a554)
                                }, (v5, v6))))))
                            } else {
                                let a555 = v9;
                                let a556 = 32u64;
                                if if if if nat_eq(a555, a556) {
                                    true
                                } else {
                                    let a557 = v9;
                                    let a558 = 9u64;
                                    nat_eq(a557, a558)
                                } {
                                    true
                                } else {
                                    let a559 = v9;
                                    let a560 = 10u64;
                                    nat_eq(a559, a560)
                                } {
                                    true
                                } else {
                                    let a561 = v9;
                                    let a562 = 13u64;
                                    nat_eq(a561, a562)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a563 = v1;
                                        let a564 = 1u64;
                                        nat_add(a563, a564)?
                                    }, (v7, (false, (false, ({
                                        let a565 = v1;
                                        let a566 = 1u64;
                                        nat_add(a565, a566)?
                                    }, f198(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a567 = v1;
                                        let a568 = 1u64;
                                        nat_add(a567, a568)?
                                    }, (v7, ({
                                        let a569 = v9;
                                        let a570 = 34u64;
                                        nat_eq(a569, a570)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(v0: Bytes, v1: u64) -> R<u64> {
    let m580 = {
        let a578 = v0;
        let a579 = v1;
        index_bytes(a578, a579)
    };
    match m580 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a625 = {
                let a601 = {
                    let a589 = {
                        let a583 = {
                            let a581 = v3;
                            let a582 = 128u8;
                            fixed_u8::bit_and(a581, a582)
                        };
                        let a584 = 128u8;
                        if equal(a583, a584) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a590 = {
                        let a587 = {
                            let a585 = v3;
                            let a586 = 64u8;
                            fixed_u8::bit_and(a585, a586)
                        };
                        let a588 = 64u8;
                        if equal(a587, a588) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a589, a590)?
                };
                let a602 = {
                    let a599 = {
                        let a593 = {
                            let a591 = v3;
                            let a592 = 32u8;
                            fixed_u8::bit_and(a591, a592)
                        };
                        let a594 = 32u8;
                        if equal(a593, a594) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a600 = {
                        let a597 = {
                            let a595 = v3;
                            let a596 = 16u8;
                            fixed_u8::bit_and(a595, a596)
                        };
                        let a598 = 16u8;
                        if equal(a597, a598) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a599, a600)?
                };
                nat_add(a601, a602)?
            };
            let a626 = {
                let a623 = {
                    let a611 = {
                        let a605 = {
                            let a603 = v3;
                            let a604 = 8u8;
                            fixed_u8::bit_and(a603, a604)
                        };
                        let a606 = 8u8;
                        if equal(a605, a606) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a612 = {
                        let a609 = {
                            let a607 = v3;
                            let a608 = 4u8;
                            fixed_u8::bit_and(a607, a608)
                        };
                        let a610 = 4u8;
                        if equal(a609, a610) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a611, a612)?
                };
                let a624 = {
                    let a621 = {
                        let a615 = {
                            let a613 = v3;
                            let a614 = 2u8;
                            fixed_u8::bit_and(a613, a614)
                        };
                        let a616 = 2u8;
                        if equal(a615, a616) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a622 = {
                        let a619 = {
                            let a617 = v3;
                            let a618 = 1u8;
                            fixed_u8::bit_and(a617, a618)
                        };
                        let a620 = 1u8;
                        if equal(a619, a620) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a621, a622)?
                };
                nat_add(a623, a624)?
            };
            nat_add(a625, a626)
        }
    }
}

pub fn f117(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f199(v0.clone(), v1, {
        let a628 = {
            let a627 = v0.clone();
            length_bytes(a627)
        };
        let a629 = 1u64;
        nat_add(a628, a629)?
    })?;
    let a630 = f116(v0.clone(), v2)?;
    let a631 = 34u64;
    if nat_eq(a630, a631) {
        Ok(Some(({
            let a632 = v2;
            let a633 = 1u64;
            nat_add(a632, a633)?
        }, f46(v0, v1, {
            let a634 = v2;
            let a635 = v1;
            nat_sub(a634, a635)
        }))))
    } else {
        f200(v0.clone(), v1, {
            let a637 = {
                let a636 = v0;
                length_bytes(a636)
            };
            let a638 = 1u64;
            nat_add(a637, a638)?
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
        let a639 = f116(v0.clone(), v1)?;
        let a640 = 45u64;
        nat_eq(a639, a640)
    };
    let v3: u64 = if v2 {
        let a641 = v1;
        let a642 = 1u64;
        nat_add(a641, a642)?
    } else {
        v1
    };
    let v4: u64 = f201(v0.clone(), v3)?;
    let v5: u64 = {
        let a643 = v4;
        let a644 = v3;
        nat_sub(a643, a644)
    };
    let v6: bool = {
        let a645 = f116(v0.clone(), v4)?;
        let a646 = 46u64;
        nat_eq(a645, a646)
    };
    let v7: u64 = if v6 {
        let a647 = v4;
        let a648 = 1u64;
        nat_add(a647, a648)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f201(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f116(v0.clone(), v8)?;
    let v10: bool = {
        let a649 = v9;
        let a650 = 101u64;
        if nat_eq(a649, a650) {
            true
        } else {
            let a651 = v9;
            let a652 = 69u64;
            nat_eq(a651, a652)
        }
    };
    let v11: u64 = f116(v0.clone(), {
        let a653 = v8;
        let a654 = 1u64;
        nat_add(a653, a654)?
    })?;
    let v12: bool = f118(v10, f202({
        let a655 = v11;
        let a656 = 43u64;
        nat_eq(a655, a656)
    }, {
        let a657 = v11;
        let a658 = 45u64;
        nat_eq(a657, a658)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a659 = v8;
            let a660 = 2u64;
            nat_add(a659, a660)?
        } else {
            let a661 = v8;
            let a662 = 1u64;
            nat_add(a661, a662)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f201(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a663 = v14;
        let a664 = v13;
        nat_sub(a663, a664)
    });
    let v16: u64 = f203(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a666 = {
            let a665 = v15;
            length_bytes(a665)
        };
        let a667 = v16;
        nat_sub(a666, a667)
    });
    let a668 = v5;
    let a669 = 0u64;
    if if nat_eq(a668, a669) {
        true
    } else {
        if f118({
            let a670 = 1u64;
            let a671 = v5;
            nat_lt(a670, a671)
        }, {
            let a672 = f116(v0.clone(), v3)?;
            let a673 = 48u64;
            nat_eq(a672, a673)
        }) {
            true
        } else {
            if f118(v6, {
                let a674 = v8;
                let a675 = v7;
                nat_eq(a674, a675)
            }) {
                true
            } else {
                f202(if v10 {
                    let a676 = v14;
                    let a677 = v13;
                    nat_eq(a676, a677)
                } else {
                    false
                }, {
                    let a679 = 18u64;
                    let a680 = {
                        let a678 = v17.clone();
                        length_bytes(a678)
                    };
                    nat_lt(a679, a680)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m684 = {
            let a682 = {
                let a681 = v17.clone();
                length_bytes(a681)
            };
            let a683 = 0u64;
            if nat_eq(a682, a683) {
                Some(0u64)
            } else {
                f204(v17)?
            }
        };
        match m684 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m689 = f205(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a685 = v8;
                    let a686 = v7;
                    nat_sub(a685, a686)
                }), f118(v12, {
                    let a687 = v11;
                    let a688 = 45u64;
                    nat_eq(a687, a688)
                }), v18)?;
                match m689 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a692 = 19u64;
                        let a693 = {
                            let a691 = {
                                let (h690, _) = v19.clone();
                                h690
                            };
                            length_bytes(a691)
                        };
                        if nat_lt(a692, a693) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m695 = f204({
                                let (h694, _) = v19.clone();
                                h694
                            })?;
                            match m695 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f118(v2, {
                                        let a696 = 0u64;
                                        let a697 = v20;
                                        nat_lt(a696, a697)
                                    });
                                    if f206(v21, v20) {
                                        Ok(Some((v14, {
                                            let a699 = {
                                                let (_, h698) = v19.clone();
                                                h698
                                            };
                                            let a700 = 0u64;
                                            if nat_eq(a699, a700) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h701) = v19;
                                                    h701
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
        let (h702, _) = v2.clone();
        h702
    };
    let v4: u64 = {
        let (h703, _) = {
            let (_, h704) = v2.clone();
            h704
        };
        h703
    };
    let v5: List<Bytes> = {
        let (_, h705) = {
            let (_, h706) = v2;
            h706
        };
        h705
    };
    let m707 = f207(v0.clone(), v3);
    match m707 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a708 = v6;
            let a709 = v1;
            if equal(a708, a709) {
                Ok(Some(({
                    let a710 = v3;
                    let a711 = 1u64;
                    nat_add(a710, a711)?
                }, ({
                    let a712 = v3;
                    let a713 = 1u64;
                    nat_add(a712, a713)?
                }, List::cons(f46(v0, v4, {
                    let a714 = v3;
                    let a715 = v4;
                    nat_sub(a714, a715)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a716 = v3;
                    let a717 = 1u64;
                    nat_add(a716, a717)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f121(v0: Fn1, p721: List<Bytes>, p722: List<Bytes>) -> List<Bytes> {
    let m720 = (p721, p722);
    {
        let mut m720 = m720;
        loop {
            let step = {
                let (v1, v2) = m720;
                let m718 = v2;
                match m718.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c719: Fn1 = v0.clone();
                            c719.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m720 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f123(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m723 = f208(Fn2::F209(v1), None::<Bytes>, v0);
    match m723 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f124() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))))
}

pub fn f125(v0: Adt1) -> List<Bytes> {
    if f139(match v0.clone() {
        Adt1::C0(_, h724, _, _) => {
            h724
        }
    }, f210()) {
        if f139(match v0 {
            Adt1::C0(_, h725, _, _) => {
                h725
            }
        }, f211()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f211(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f210(), List::<Bytes>::nil())
    }
}

pub fn f126(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m726 = v0;
    match m726.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f126(v3, v1))
        }
    }
}

pub fn f127(v0: Adt1) -> R<List<Bytes>> {
    Ok(f56(v0.clone(), List::cons(f212(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f137(v0)?)))))
}

pub fn f128(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f213(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f143(f97(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f129(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m727 = v0;
    match m727.uncons() {
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
    let m728 = v0;
    match m728.uncons() {
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
    let m729 = v0;
    match m729.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f126(f214(f187(f94(v1, f8(Str::lit("defers"))))), f131(v2))
        }
    }
}

pub fn f132(v0: List<Adt14>) -> List<Bytes> {
    let m730 = v0;
    match m730.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m731 = f94(v1, f8(Str::lit("std")));
            match m731 {
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
    let m732 = v0;
    match m732.uncons() {
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
    let v3: List<(Bytes, Bytes)> = f155(f215(v2.clone()), f216(f217(v0, v1.clone(), v2)?));
    f218(f146(f219(v1))?, v3.clone(), {
        let a734 = {
            let a733 = v3;
            length_list(a733)
        };
        let a735 = 1u64;
        nat_add(a734, a735)?
    })
}

pub fn f135(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m736 = v1;
    match m736.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f220(v2.clone(), f8(Str::lit("subject"))) {
                f221(v0.clone(), v2.clone())
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
    let m737 = v1;
    match m737.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m738 = f222(f96(f64(v0.clone(), v2.clone()))?, f85(v2, f211()))?;
            match m738 {
                None => {
                    f136(v0, v3)
                }
                Some(v4) => {
                    Ok(f223(v4, f136(v0, v3)?))
                }
            }
        }
    }
}

pub fn f137(v0: Adt1) -> R<List<Bytes>> {
    Ok(f126(f146(f224(f210(), f148(match v0.clone() {
        Adt1::C0(_, h739, _, _) => {
            h739
        }
    }, f210())))?, f146(f224(f211(), f148(match v0 {
        Adt1::C0(_, h740, _, _) => {
            h740
        }
    }, f211())))?))
}

pub fn f138(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f139(p744: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m743 = p744;
    {
        let mut m743 = m743;
        loop {
            let step = {
                let v0 = m743;
                let m741 = v0;
                match m741.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h742, _) = v2;
                            h742
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140() -> Bytes {
    f8(Str::lit("model/registries"))
}

pub fn f141(v0: Adt1) -> R<List<Bytes>> {
    f146(f224(f140(), f148(match v0 {
        Adt1::C0(_, h745, _, _) => {
            h745
        }
    }, f140())))
}

pub fn f142(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m746 = v1;
    match m746.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f126(if f52(v4.clone(), v3.clone()) {
                f225(v4.clone(), f96(f64(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f226(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
            }, f142(v0, v5, v2, v3)?))
        }
    }
}

pub fn f143(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m747 = v0;
    match m747.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f58(v2, v1.clone()), f143(v3, v1))
        }
    }
}

pub fn f144(v0: Adt1) -> R<Adt12> {
    let m748 = f56(v0.clone(), f227());
    match m748.uncons() {
        None => {
            let m749 = f228(v0.clone(), f227())?;
            match m749.uncons() {
                None => {
                    let m752 = f229(match v0.clone() {
                        Adt1::C0(_, h750, _, _) => {
                            h750
                        }
                    }, f126(f230(), f231(match v0.clone() {
                        Adt1::C0(_, h751, _, _) => {
                            h751
                        }
                    }, false)));
                    match m752.uncons() {
                        None => {
                            let m754 = f56(v0.clone(), f231(match v0.clone() {
                                Adt1::C0(_, h753, _, _) => {
                                    h753
                                }
                            }, true));
                            match m754.uncons() {
                                None => {
                                    let m755 = f56(v0.clone(), f232());
                                    match m755.uncons() {
                                        None => {
                                            let m756 = f233(v0.clone())?;
                                            match m756 {
                                                None => {
                                                    let m758 = f234(v0.clone(), f231(match v0.clone() {
                                                        Adt1::C0(_, h757, _, _) => {
                                                            h757
                                                        }
                                                    }, true))?;
                                                    match m758 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f235(Adt17::C0(Adt16::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f213(v0.clone())?, f143(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f97(v0.clone(), f8(Str::lit("model/strata.json")))?, f214(f97(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f231(match v0 {
                                                                Adt1::C0(_, h759, _, _) => {
                                                                    h759
                                                                }
                                                            }, true)))?;
                                                            let (h760, _) = v13;
                                                            if h760 {
                                                                Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                                                    let (_, h761) = v13;
                                                                    h761
                                                                }))
                                                            } else {
                                                                Ok(Adt12::C4({
                                                                    let (_, h762) = v13;
                                                                    h762
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
    Ok(f236(f121(Fn1::F237, List::<Bytes>::nil(), f238(v0)?)))
}

pub fn f147(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m763 = v1;
    match m763.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f239(v0.clone(), v2), f147(v0, v3))
            } else {
                f147(v0, v3)
            }
        }
    }
}

pub fn f148(p768: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m767 = p768;
    {
        let mut m767 = m767;
        loop {
            let step = {
                let v0 = m767;
                let m764 = v0;
                match m764.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h765, _) = v2;
                            h765
                        }, v1.clone()) {
                            let (_, h766) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h766)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m767 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, p771: List<Bytes>) -> R<Option<Bytes>> {
    let m770 = p771;
    {
        let mut m770 = m770;
        loop {
            let step = {
                let v2 = m770;
                let m769 = v2;
                match m769.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f240(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: List<List<Bytes>>) -> R<List<(List<Bytes>, List<Bytes>)>> {
    let m772 = v0;
    match m772.uncons() {
        None => {
            Ok(List::<(List<Bytes>, List<Bytes>)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f241(v1.clone())?, v1), f150(v2)?))
        }
    }
}

pub fn f151(p775: List<Bytes>, p776: List<Bytes>, p777: List<List<Bytes>>) -> List<List<Bytes>> {
    let m774 = (p775, (p776, p777));
    {
        let mut m774 = m774;
        loop {
            let step = {
                let (v0, (v1, v2)) = m774;
                let m773 = v0;
                match m773.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f242(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f242(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m778 = v0;
    match m778.uncons() {
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
        Adt1::C0(_, h779, _, _) => {
            h779
        }
    }, f243())) {
        Ok(Adt12::C1(List::cons(f243(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f152(f146(f147(f243(), f148(match v0.clone() {
            Adt1::C0(_, h780, _, _) => {
                h780
            }
        }, f243())))?, v1.clone());
        let m781 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m781.uncons() {
            None => {
                let v5: List<Bytes> = f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m782 = f56(v0.clone(), f244(v0.clone(), v2.clone(), v5.clone())?);
                match m782.uncons() {
                    None => {
                        let m783 = f245(v0.clone(), v2, v5)?;
                        match m783 {
                            None => {
                                f246(v0, v1)
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
    let m784 = v0;
    match m784.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f155(v3, v1))
        }
    }
}

pub fn f156(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m785 = f97(v0.clone(), f154())?;
    match m785.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f247(v0)?), List::<(Bytes, Bytes)>::nil()))
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
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^lexlean-workspace.toml$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers and lexlean-workspace.toml, not: $outside\"; exit 1; fi\u{a}          # readers/lexlean.lock is generated, and the export below writes it again.\u{a}          git apply --exclude=readers/lexlean.lock target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          # The one export, shared with refresh and run end to end by the studio check.\u{a}          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f248(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}            echo\u{a}            echo '```'\u{a}            head -n 8 target/export.log.phases || true\u{a}            echo '```'\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # read-* operations run in the reader project's command when it is built, every other operation in the studio command.\u{a}          vs() { case \"$1\" in read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if vs $op > target/attempt.txt 2>&1; then\u{a}                tee -a target/operation.txt < target/attempt.txt\u{a}                last=$op\u{a}              else\u{a}                cat target/attempt.txt\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}            # good main commit, it reverts the merge on main as the App, for the owner's review.\u{a}            merge=${ARGS#revert-merge }\u{a}            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}            git fetch -q origin main\u{a}            git checkout -q -B revert origin/main\u{a}            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}            git revert -m 1 --no-commit \"$merge\"\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}          else\u{a}            vs $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}          pr=\"\"\u{a}          for try in 1 2 3 4; do\u{a}            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}            if [ -n \"$pr\" ]; then break; fi\u{a}            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}            pr=\"\"\u{a}            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}            sleep $((2 ** try))\u{a}          done\u{a}          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}          echo \"opened #${pr}\"\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ] || [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}            # refresh starts when this run completes (its workflow_run trigger), and checks the new pull request then.\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f160() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}# operate run completes, which needs no token that may dispatch workflows, so every\u{a}# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_run:\u{a}    workflows: [operate]\u{a}    types: [completed]\u{a}  workflow_dispatch:\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}      merges: ${{ steps.stale.outputs.merges }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          merges=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # Drafting drops a pull request's auto-merge, so the owner's merge method is kept\u{a}              # here and restored once the regenerated pull request is ready again.\u{a}              method=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge.merge_method // \"\"')\u{a}              if [ -n \"$method\" ]; then merges=\"$merges $pr:$method\"; fi\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"merges=$merges\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}          MERGES: ${{ needs.find.outputs.merges }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            # readers/lexlean.lock is generated: the export below writes it again, so it is left out of the\u{a}            # change, and two reader pull requests never conflict on it.\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml ':!readers/lexlean.lock' > target/change.patch\u{a}            # model/strata.json is one line, so two changes that each add a stratum row always\u{a}            # conflict there. Its rows are keyed by path (VS-05), so the merge is main's rows, then\u{a}            # each row the pull request adds whose path main does not have, in the pull request's order.\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              if [ \"$(git diff --name-only --diff-filter=U)\" = model/strata.json ]; then\u{a}                base=$(git merge-base origin/main \"origin/$branch\")\u{a}                git show \"$base:model/strata.json\" > target/strata-base.json\u{a}                git show \"origin/$branch:model/strata.json\" > target/strata-pr.json\u{a}                git show origin/main:model/strata.json | jq -c --slurpfile pr target/strata-pr.json --slurpfile base target/strata-base.json '(map(.path)) as $have | ($base[0] | map(.path)) as $old | . + [$pr[0][] | select(.path as $p | (($have | index($p)) == null) and (($old | index($p)) == null))]' > model/strata.json\u{a}                git add model/strata.json\u{a}                echo \"::notice::#${pr}: merged its stratum rows into main's by path\"\u{a}              else\u{a}                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}                git reset -q --hard origin/main\u{a}                failed=\"$failed $pr\"\u{a}                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}                continue\u{a}              fi\u{a}            fi\u{a}            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f248(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf studio/.lexlean/production readers/.lexlean/production\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers lexlean-workspace.toml; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            # The owner's auto-merge comes back with the same method, and only where it was set.\u{a}            for entry in $MERGES; do\u{a}              if [ \"${entry/:*/}\" = \"$pr\" ] && ! gh pr merge \"$pr\" --auto \"--${entry#*:}\"; then\u{a}                echo \"::error::#${pr}: its auto-merge (${entry#*:}) could not be restored\"\u{a}                failed=\"$failed $pr\"\u{a}              fi\u{a}            done\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
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
    let v4: List<List<Bytes>> = f249(f250(f64(v0, f212()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f251(v4.clone(), f252(v4), f217(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f134(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f164(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f253(f128(v0.clone())?, f136(v0.clone(), f137(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f165(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f254(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f255(f97(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f166() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f167(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m786 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m786.uncons() {
        None => {
            if f256(f96(f64(v0, v1))?)? {
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
    let m787 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m787.uncons() {
        None => {
            let v5: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f257(v5)?
                } else {
                    f258(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f239(f259(), f260(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f239(f259(), f260(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f261()
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
    let m788 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m788.uncons() {
        None => {
            let m789 = f262(f96(f64(v0.clone(), v1.clone()))?)?;
            match m789 {
                None => {
                    Ok(Adt12::C4(f263()))
                }
                Some(v4) => {
                    let m790 = f264(f96(f64(v0, v1))?)?;
                    match m790 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f123(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f265(v5)))
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
        if f118(f266(f267(v2.clone())?), f51({
            let a792 = {
                let a791 = f268(f267(v2)?);
                length_list(a791)
            };
            let a793 = 0u64;
            nat_eq(a792, a793)
        })) {
            Ok(Adt12::C3(List::cons((f239(f259(), f260(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f239(f259(), f260(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt12::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f171(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m794 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m794.uncons() {
        None => {
            let m795 = f269(f96(f64(v0, v1))?)?;
            match m795 {
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
    let m796 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m796.uncons() {
        None => {
            let v5: Bytes = f239(if v2 {
                f211()
            } else {
                f210()
            }, f260(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m797 = f222(f96(f64(v0.clone(), v1.clone()))?, v2)?;
                match m797 {
                    None => {
                        Ok(Adt12::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a799 = {
                            let a798 = v6;
                            length_list(a798)
                        };
                        let a800 = 0u64;
                        if nat_eq(a799, a800) {
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
    let m801 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m801.uncons() {
        None => {
            let v4: List<Bytes> = f96(f64(v0.clone(), v1))?;
            let m802 = f270(v4.clone(), f8(Str::lit("requirement")))?;
            match m802 {
                None => {
                    Ok(Adt12::C4(f261()))
                }
                Some(v5) => {
                    let m803 = f270(v4, f8(Str::lit("admission")))?;
                    match m803 {
                        None => {
                            Ok(Adt12::C4(f261()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f97(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f126(f143(v5.clone(), f8(Str::lit("id"))), f143(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f146(f126(f143(v7.clone(), f8(Str::lit("id"))), f143(v8.clone(), f8(Str::lit("construct")))))?;
                            let m804 = f271(f132(v6.clone()), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m804 {
                                None => {
                                    let a806 = {
                                        let a805 = v9.clone();
                                        length_list(a805)
                                    };
                                    let a807 = 0u64;
                                    if nat_eq(a806, a807) {
                                        Ok(Adt12::C4(f261()))
                                    } else {
                                        if f51({
                                            let a810 = {
                                                let a808 = f146(f126(v9.clone(), v10.clone()))?;
                                                length_list(a808)
                                            };
                                            let a811 = {
                                                let a809 = f126(v9.clone(), v10);
                                                length_list(a809)
                                            };
                                            nat_eq(a810, a811)
                                        }) {
                                            Ok(Adt12::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt12::C3(List::cons(f273(f8(Str::lit("model/requirements.json")), f223(v7, v5))?, List::cons(f273(f8(Str::lit("model/admissions.json")), f223(v8, v6))?, List::cons(f274(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f123(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f272(v11))
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
    let m812 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m812.uncons() {
        None => {
            let m813 = f91(v0.clone(), v1.clone())?;
            match m813 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m814 = f275(v4.clone(), f8(Str::lit("schema")));
                    match m814 {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f276(v0, f277(v5.clone())?, v5, f278(v4.clone(), f8(Str::lit("name"))), f279(f280(f275(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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
    let m815 = v2;
    match m815.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m816 = v4;
            match m816.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m817 = v6.clone();
                    match m817.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((v7, v8)) => {
                            let m818 = v8;
                            match m818.uncons() {
                                None => {
                                    Ok(Adt12::C4(f79()))
                                }
                                Some((v9, _)) => {
                                    if f51(f85(v7, f8(Str::lit("read-")))) {
                                        Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that reads the artifact: admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT\u{a}"))))
                                    } else {
                                        let m819 = f56(v0.clone(), List::cons(f281(v1.clone()), List::cons(v9.clone(), List::cons(f8(Str::lit("model/admissions.json")), if f32(v5.clone(), f8(Str::lit("none"))) {
                                            List::<Bytes>::nil()
                                        } else {
                                            List::cons(f7(List::cons(f8(Str::lit("encodings/")), List::cons(v5.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), List::<Bytes>::nil())
                                        }))));
                                        match m819.uncons() {
                                            None => {
                                                if f51(f67(v0.clone(), f281(v1.clone()))) {
                                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v1.clone(), List::cons(f8(Str::lit(" is not a bound authority, because ")), List::cons(f281(v1), List::cons(f8(Str::lit(" is absent, and every admitted value cites a bound standard (REQ-ADMISSION)\u{a}")), List::<Bytes>::nil()))))))))
                                                } else {
                                                    let m820 = f57(v0.clone(), f281(v1.clone()))?;
                                                    match m820 {
                                                        None => {
                                                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(f281(v1), List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                        Some(v13) => {
                                                            let v15: Bytes = {
                                                                let m821 = f94(v13, f8(Str::lit("source")));
                                                                match m821 {
                                                                    None => {
                                                                        Bytes::lit(&[])
                                                                    }
                                                                    Some(v14) => {
                                                                        f58(v14, f8(Str::lit("sha256")))
                                                                    }
                                                                }
                                                            };
                                                            let v16: Bytes = f282(f64(v0.clone(), v9.clone()))?;
                                                            if f51(f32(v16.clone(), v15.clone())) {
                                                                Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v9, List::cons(f8(Str::lit(" is not the artifact ")), List::cons(v1, List::cons(f8(Str::lit(" binds: its SHA-256 is ")), List::cons(v16, List::cons(f8(Str::lit(" and the binding's source.sha256 is ")), List::cons(v15, List::cons(f8(Str::lit(" (FIPS 180-4 section 6.2)\u{a}")), List::<Bytes>::nil()))))))))))))
                                                            } else {
                                                                f283(v0, v1, v3, v5, v6, v9)
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
    let m822 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m822.uncons() {
        None => {
            let m823 = f57(v0.clone(), v1.clone())?;
            match m823 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f284())) {
                        Ok(Adt12::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m824 = f94(v4.clone(), f8(Str::lit("source")));
                            match m824 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f93(f94(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f239(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f239(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m825 = v8;
                        match m825 {
                            None => {
                                f285(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f285(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f239(v9, f260(f58(v7.clone(), f8(Str::lit("url"))))?);
                                    if f51(f52(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h826, _) => {
                                            h826
                                        }
                                    })) {
                                        Ok(Adt12::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f58(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m827 = f56(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m827.uncons() {
                                            None => {
                                                if f32(f282(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f285(v0, v4, v5, v10, Some((v12, v11)))
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
    let m828 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m828.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f250(v4.clone())?;
            if f51(f99(f286(v5.clone()), f287())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f288(Str::lit("id"), v6.clone())?, List::cons(f288(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f288(Str::lit("artifact"), f212())?, List::cons(f288(Str::lit("checksum"), f282(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f212(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f289(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f274(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f123(f252(f249(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f178(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m829 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m829.uncons() {
        None => {
            let m830 = f57(v0, v1)?;
            match m830 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f290(f187(Some(v4))) {
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
    let m831 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m831.uncons() {
        None => {
            let m832 = f91(v0.clone(), v1.clone())?;
            match m832 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f276(v0.clone(), f64(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f180(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m833 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f154(), List::<Bytes>::nil())));
    match m833.uncons() {
        None => {
            let m834 = f291(f96(f64(v0.clone(), v1))?)?;
            match m834 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f97(v0, f154())?;
                    let v6: List<Bytes> = f143(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f146(f143(v5.clone(), f8(Str::lit("at"))))?;
                    let a836 = {
                        let a835 = v6.clone();
                        length_list(a835)
                    };
                    let a837 = 0u64;
                    if if nat_eq(a836, a837) {
                        true
                    } else {
                        f51({
                            let a840 = {
                                let a838 = f146(f126(v6.clone(), v7.clone()))?;
                                length_list(a838)
                            };
                            let a841 = {
                                let a839 = f126(v6.clone(), v7);
                                length_list(a839)
                            };
                            nat_eq(a840, a841)
                        })
                    } {
                        Ok(Adt12::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt12::C3(List::cons(f273(f154(), f223(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f123(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f181(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m842 = v2;
    match m842.uncons() {
        None => {
            Ok(Adt12::C4(f79()))
        }
        Some((v3, v4)) => {
            let m843 = v4;
            match m843.uncons() {
                None => {
                    Ok(Adt12::C4(f79()))
                }
                Some((v5, v6)) => {
                    let m844 = v6.clone();
                    match m844.uncons() {
                        None => {
                            Ok(Adt12::C4(f79()))
                        }
                        Some((_, _)) => {
                            let m845 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f292(), List::<Bytes>::nil())));
                            match m845.uncons() {
                                None => {
                                    let v11: Bytes = f123(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt14> = f97(v0, f292())?;
                                    if f293(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f273(f292(), f223(v13, List::cons(f294(v11.clone(), v12, v5, v3), List::<Adt14>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f182(v0: Bytes, p899: u64, p900: u64) -> R<bool> {
    let m898 = (p899, p900);
    {
        let mut m898 = m898;
        loop {
            let step = {
                let (v1, v2) = m898;
                let m846 = v2;
                if m846 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m846 - 1;
                    let m849 = {
                        let a847 = v0.clone();
                        let a848 = v1;
                        index_bytes(a847, a848)
                    };
                    match m849 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f295({
                                let v5: u8 = v4;
                                let a894 = {
                                    let a870 = {
                                        let a858 = {
                                            let a852 = {
                                                let a850 = v5;
                                                let a851 = 128u8;
                                                fixed_u8::bit_and(a850, a851)
                                            };
                                            let a853 = 128u8;
                                            if equal(a852, a853) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a859 = {
                                            let a856 = {
                                                let a854 = v5;
                                                let a855 = 64u8;
                                                fixed_u8::bit_and(a854, a855)
                                            };
                                            let a857 = 64u8;
                                            if equal(a856, a857) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a858, a859)?
                                    };
                                    let a871 = {
                                        let a868 = {
                                            let a862 = {
                                                let a860 = v5;
                                                let a861 = 32u8;
                                                fixed_u8::bit_and(a860, a861)
                                            };
                                            let a863 = 32u8;
                                            if equal(a862, a863) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a869 = {
                                            let a866 = {
                                                let a864 = v5;
                                                let a865 = 16u8;
                                                fixed_u8::bit_and(a864, a865)
                                            };
                                            let a867 = 16u8;
                                            if equal(a866, a867) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a868, a869)?
                                    };
                                    nat_add(a870, a871)?
                                };
                                let a895 = {
                                    let a892 = {
                                        let a880 = {
                                            let a874 = {
                                                let a872 = v5;
                                                let a873 = 8u8;
                                                fixed_u8::bit_and(a872, a873)
                                            };
                                            let a875 = 8u8;
                                            if equal(a874, a875) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a881 = {
                                            let a878 = {
                                                let a876 = v5;
                                                let a877 = 4u8;
                                                fixed_u8::bit_and(a876, a877)
                                            };
                                            let a879 = 4u8;
                                            if equal(a878, a879) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a880, a881)?
                                    };
                                    let a893 = {
                                        let a890 = {
                                            let a884 = {
                                                let a882 = v5;
                                                let a883 = 2u8;
                                                fixed_u8::bit_and(a882, a883)
                                            };
                                            let a885 = 2u8;
                                            if equal(a884, a885) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a891 = {
                                            let a888 = {
                                                let a886 = v5;
                                                let a887 = 1u8;
                                                fixed_u8::bit_and(a886, a887)
                                            };
                                            let a889 = 1u8;
                                            if equal(a888, a889) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a890, a891)?
                                    };
                                    nat_add(a892, a893)?
                                };
                                nat_add(a894, a895)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a896 = v1;
                                    let a897 = 1u64;
                                    nat_add(a896, a897)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
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

pub fn f183(p905: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m904 = p905;
    {
        let mut m904 = m904;
        loop {
            let step = {
                let v0 = m904;
                let m901 = v0;
                match m901.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h902, _) = v2;
                            h902
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h903) = v2;
                                h903
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m904 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m909 = {
        let a907 = match v0 {
            Adt1::C0(_, _, _, h906) => {
                h906
            }
        };
        let a908 = f40(v1);
        index_list(a907, a908)
    };
    match m909 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f185(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), v1) {
        let m910 = f44(v0, 34u8)?;
        match m910.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m911 = v3;
                match m911.uncons() {
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

pub fn f186(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f296(v0, v1)
}

pub fn f187(v0: Option<Adt14>) -> List<Adt14> {
    let m912 = v0;
    match m912 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f297(v1)
        }
    }
}

pub fn f188(v0: Adt14, v1: Bytes) -> Adt14 {
    let m913 = v0;
    match m913 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r914) => {
            let v10 = (*r914).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r915) => {
            let v5 = (*r915).clone();
            Adt14::C4(std::rc::Rc::new(f188(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r916, r917) => {
            let v11 = (*r916).clone();
            let v12 = (*r917).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r918, r919) => {
            let v3 = (*r918).clone();
            let v4 = (*r919).clone();
            if f32(v2.clone(), f8(Str::lit("std"))) {
                Adt14::C7(v2, std::rc::Rc::new(f298(v3, v1.clone())), std::rc::Rc::new(f188(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f188(v4, v1)))
            }
        }
    }
}

pub fn f189(v0: Adt14) -> R<Bytes> {
    let m920 = v0;
    match m920 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f299(v2, v3)
        }
        Adt14::C2(v4) => {
            f300(v4)
        }
        Adt14::C3(r921) => {
            let v5 = (*r921).clone();
            Ok(f301(f302(v5)?))
        }
        Adt14::C4(r922) => {
            let v6 = (*r922).clone();
            f303(f304(v6)?)
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

pub fn f190(v0: Adt1, p926: List<Adt13>) -> R<Option<Adt13>> {
    let m925 = p926;
    {
        let mut m925 = m925;
        loop {
            let step = {
                let v1 = m925;
                let m923 = v1;
                match m923.uncons() {
                    None => {
                        Err::<List<Adt13>, R<Option<Adt13>>>(Ok(None::<Adt13>))
                    }
                    Some((v2, v3)) => {
                        if f52(f191(v2.clone())?, match v0.clone() {
                            Adt1::C0(_, _, h924, _) => {
                                h924
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
                Ok(next) => { m925 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Adt13) -> R<Bytes> {
    Ok(f239(f305(), f260(f306(v0))?))
}

pub fn f192(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f306(v0), List::<Bytes>::nil())))))
}

pub fn f193() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f194() -> Bytes {
    f307()
}

pub fn f195(v0: List<Adt13>) -> R<List<Bytes>> {
    let m927 = v0;
    match m927.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f191(v1)?, f195(v2)?))
        }
    }
}

pub fn f196(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m928 = v1;
    match m928.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f308(v5.clone(), v2.clone(), v3.clone(), f282(f64(v0.clone(), f191(v5)?))?, v4.clone())?), f196(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f197(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f309(v0.clone())?;
    let v2: u64 = f310(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a929 = v1;
        let a930 = v2;
        nat_sub(a929, a930)
    }))
}

pub fn f198(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a933 = v3;
    let a934 = f46(v0, v1, {
        let a931 = v2;
        let a932 = v1;
        nat_sub(a931, a932)
    });
    append_bytes(a933, a934)
}

pub fn f199(v0: Bytes, p947: u64, p948: u64) -> R<u64> {
    let m946 = (p947, p948);
    {
        let mut m946 = m946;
        loop {
            let step = {
                let (v1, v2) = m946;
                let m935 = v2;
                if m935 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m935 - 1;
                    let v4: u64 = f116(v0.clone(), v1)?;
                    let a936 = v4;
                    let a937 = 34u64;
                    if nat_eq(a936, a937) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a938 = v4;
                        let a939 = 92u64;
                        if nat_eq(a938, a939) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a940 = v4;
                            let a941 = 32u64;
                            if nat_lt(a940, a941) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a942 = v4;
                                let a943 = 256u64;
                                if nat_eq(a942, a943) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a944 = v1;
                                        let a945 = 1u64;
                                        nat_add(a944, a945)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m946 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: Bytes, p974: u64, p975: u64, p976: Bytes) -> R<Option<(u64, Bytes)>> {
    let m973 = (p974, (p975, p976));
    {
        let mut m973 = m973;
        loop {
            let step = {
                let (v1, (v2, v3)) = m973;
                let m949 = v2;
                if m949 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m949 - 1;
                    let v5: u64 = f199(v0.clone(), v1, {
                        let a953 = {
                            let a951 = {
                                let a950 = v0.clone();
                                length_bytes(a950)
                            };
                            let a952 = v1;
                            nat_sub(a951, a952)
                        };
                        let a954 = 1u64;
                        nat_add(a953, a954)?
                    })?;
                    let v6: Bytes = {
                        let a957 = v3;
                        let a958 = f46(v0.clone(), v1, {
                            let a955 = v5;
                            let a956 = v1;
                            nat_sub(a955, a956)
                        });
                        append_bytes(a957, a958)
                    };
                    let v7: u64 = f116(v0.clone(), v5)?;
                    let a959 = v7;
                    let a960 = 34u64;
                    if nat_eq(a959, a960) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a961 = v5;
                            let a962 = 1u64;
                            nat_add(a961, a962)?
                        }, v6))))
                    } else {
                        let a963 = v7;
                        let a964 = 92u64;
                        if nat_eq(a963, a964) {
                            let m965 = f311(v0.clone(), v5, v6)?;
                            match m965 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a968 = {
                                        let (_, h966) = {
                                            let (_, h967) = v8.clone();
                                            h967
                                        };
                                        h966
                                    };
                                    let a969 = 0u64;
                                    if nat_eq(a968, a969) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h970, _) = v8;
                                            h970
                                        }, (v4, {
                                            let (h971, _) = {
                                                let (_, h972) = v8;
                                                h972
                                            };
                                            h971
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
                Ok(next) => { m973 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: Bytes, v1: u64) -> R<u64> {
    let (h977, _) = f312(Fn3::F313(v0.clone()), {
        let a979 = {
            let a978 = v0;
            length_bytes(a978)
        };
        let a980 = 1u64;
        nat_add(a979, a980)?
    }, v1)?;
    Ok(h977)
}

pub fn f202(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f203(v0: Bytes) -> R<u64> {
    let (h981, _) = f312(Fn3::F314(v0.clone()), {
        let a983 = {
            let a982 = v0;
            length_bytes(a982)
        };
        let a984 = 1u64;
        nat_add(a983, a984)?
    }, 0u64)?;
    Ok(h981)
}

pub fn f204(v0: Bytes) -> R<Option<u64>> {
    if f315(v0.clone())? {
        Ok(Some({
            let (_, h985) = {
                let (h986, _) = f316(Fn4::F317(v0.clone()), {
                    let a987 = v0;
                    length_bytes(a987)
                }, (0u64, 0u64))?;
                h986
            };
            h985
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f205(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a989 = {
            let a988 = v1.clone();
            length_bytes(a988)
        };
        let a990 = f318(v1)?;
        nat_sub(a989, a990)
    });
    let v5: Bytes = {
        let a991 = v0;
        let a992 = v4.clone();
        append_bytes(a991, a992)
    };
    let v6: u64 = f203(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a994 = {
            let a993 = v5;
            length_bytes(a993)
        };
        let a995 = v6;
        nat_sub(a994, a995)
    });
    let v8: u64 = {
        let a996 = v4;
        length_bytes(a996)
    };
    let v9: u64 = {
        let a997 = v7.clone();
        length_bytes(a997)
    };
    let a998 = v9;
    let a999 = 0u64;
    if nat_eq(a998, a999) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f202(v2, {
            let a1000 = v3;
            let a1001 = v8;
            nat_lt(a1000, a1001)
        }) {
            let v10: u64 = if v2 {
                let a1002 = v3;
                let a1003 = v8;
                nat_add(a1002, a1003)?
            } else {
                let a1004 = v8;
                let a1005 = v3;
                nat_sub(a1004, a1005)
            };
            let v11: u64 = f318(v7.clone())?;
            let a1006 = v10;
            let a1007 = v11;
            if nat_le(a1006, a1007) {
                Ok(Some((f46(v7, 0u64, {
                    let a1008 = v9;
                    let a1009 = v10;
                    nat_sub(a1008, a1009)
                }), 0u64)))
            } else {
                let a1012 = {
                    let a1010 = v10;
                    let a1011 = v11;
                    nat_sub(a1010, a1011)
                };
                let a1013 = 19u64;
                if nat_le(a1012, a1013) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1014 = v9;
                        let a1015 = v11;
                        nat_sub(a1014, a1015)
                    }), {
                        let a1016 = v10;
                        let a1017 = v11;
                        nat_sub(a1016, a1017)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1018 = v3;
                let a1019 = v8;
                nat_sub(a1018, a1019)
            };
            let a1022 = {
                let a1020 = v9;
                let a1021 = v12;
                nat_add(a1020, a1021)?
            };
            let a1023 = 19u64;
            if nat_le(a1022, a1023) {
                Ok(Some(({
                    let a1024 = v7;
                    let a1025 = f319(v12);
                    append_bytes(a1024, a1025)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f206(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1026 = 0u64;
        let a1027 = v1;
        if nat_lt(a1026, a1027) {
            let a1028 = v1;
            let a1029 = 9223372036854775808u64;
            nat_le(a1028, a1029)
        } else {
            false
        }
    } else {
        let a1030 = v1;
        let a1031 = 9223372036854775807u64;
        nat_le(a1030, a1031)
    }
}

pub fn f207(v0: Bytes, v1: u64) -> Option<u8> {
    let a1032 = v0;
    let a1033 = v1;
    index_bytes(a1032, a1033)
}

pub fn f208(v0: Fn2, p1037: Option<Bytes>, p1038: List<Bytes>) -> Option<Bytes> {
    let m1036 = (p1037, p1038);
    {
        let mut m1036 = m1036;
        loop {
            let step = {
                let (v1, v2) = m1036;
                let m1034 = v2;
                match m1034.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c1035: Fn2 = v0.clone();
                            c1035.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1036 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m1039 = v1;
    match m1039 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a1042 = {
                    let a1040 = v3;
                    let a1041 = v0;
                    append_bytes(a1040, a1041)
                };
                let a1043 = v2;
                append_bytes(a1042, a1043)
            })
        }
    }
}

pub fn f210() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f211() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f212() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f213(v0: Adt1) -> R<List<Bytes>> {
    let m1044 = f250(f64(v0, f212()))?;
    match m1044.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f252(v2))
        }
    }
}

pub fn f214(v0: List<Adt14>) -> List<Bytes> {
    let m1045 = v0;
    match m1045.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1046 = f93(Some(v1));
            match m1046 {
                None => {
                    f214(v2)
                }
                Some(v3) => {
                    List::cons(v3, f214(v2))
                }
            }
        }
    }
}

pub fn f215(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1047 = v0;
    match m1047.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f215(v2))
        }
    }
}

pub fn f216(v0: List<Adt18>) -> List<(Bytes, Bytes)> {
    let m1048 = v0;
    match m1048.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt18::C0(h1049, _, _) => {
                    h1049
                }
            }, match v1 {
                Adt18::C0(_, _, h1050) => {
                    h1050
                }
            }), f216(v2))
        }
    }
}

pub fn f217(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt18>> {
    let v3: List<(Bytes, Bytes)> = f320(v1);
    let v4: List<(Bytes, Bytes)> = f321(v0, v2);
    f322(v3.clone(), f155(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f218(p1057: List<Bytes>, v1: List<(Bytes, Bytes)>, p1058: u64) -> R<List<Bytes>> {
    let m1056 = (p1057, p1058);
    {
        let mut m1056 = m1056;
        loop {
            let step = {
                let (v0, v2) = m1056;
                let m1051 = v2;
                if m1051 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m1051 - 1;
                    let v4: List<Bytes> = f146(f126(v0.clone(), f323(v0.clone(), v1.clone())))?;
                    let a1054 = {
                        let a1052 = v4.clone();
                        length_list(a1052)
                    };
                    let a1055 = {
                        let a1053 = v0.clone();
                        length_list(a1053)
                    };
                    if nat_eq(a1054, a1055) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1056 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: List<Adt14>) -> List<Bytes> {
    let m1059 = v0;
    match m1059.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1060 = f94(v1, f8(Str::lit("std")));
            match m1060 {
                None => {
                    f219(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3.clone(), f8(Str::lit("authority"))), if f220(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f58(v3, f8(Str::lit("row"))), f219(v2))
                    } else {
                        f219(v2)
                    })
                }
            }
        }
    }
}

pub fn f220(v0: Adt14, v1: Bytes) -> bool {
    f51({
        let a1062 = {
            let a1061 = f58(v0, v1);
            length_bytes(a1061)
        };
        let a1063 = 0u64;
        nat_eq(a1062, a1063)
    })
}

pub fn f221(v0: Adt15, v1: Adt14) -> bool {
    if f324(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f324(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f222(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m1064 = v0;
    match m1064.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f197(v2)?;
            let a1066 = {
                let a1065 = v4.clone();
                length_bytes(a1065)
            };
            let a1067 = 0u64;
            if nat_eq(a1066, a1067) {
                f222(v3, v1)
            } else {
                let m1068 = f325(v4)?;
                match m1068 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f326(v5.clone())
                        } else {
                            f327(v5.clone())
                        } {
                            let m1069 = f222(v3, v1)?;
                            match m1069 {
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

pub fn f223(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m1070 = v0;
    match m1070.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f223(v3, v1))
        }
    }
}

pub fn f224(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1071 = v1;
    match m1071.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f239(v0.clone(), v2), f224(v0, v3))
            } else {
                f224(v0, v3)
            }
        }
    }
}

pub fn f225(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1072 = v1;
    match m1072.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f126({
                let a1074 = {
                    let a1073 = f197(v4.clone())?;
                    length_bytes(a1073)
                };
                let a1075 = 0u64;
                if nat_eq(a1074, a1075) {
                    List::<Bytes>::nil()
                } else {
                    f328(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f225(v0, v5, v2, v3)?))
        }
    }
}

pub fn f226(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f227() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f212(), List::<Bytes>::nil())))))))
}

pub fn f228(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1076 = v1;
    match m1076.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f67(v0.clone(), v2.clone()) {
                f41(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m1077 = f57(v0.clone(), v2.clone())?;
                match m1077 {
                    None => {
                        Ok(List::cons(v2, f228(v0, v3)?))
                    }
                    Some(_) => {
                        f228(v0, v3)
                    }
                }
            } else {
                f228(v0, v3)
            }
        }
    }
}

pub fn f229(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m1078 = v1;
    match m1078.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f139(v0.clone(), v2.clone()) {
                f229(v0, v3)
            } else {
                List::cons(v2, f229(v0, v3))
            }
        }
    }
}

pub fn f230() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f231(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m1079 = v0;
    match m1079.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f126(f329({
                let (h1080, _) = v2;
                h1080
            }, {
                let (_, h1081) = v2;
                h1081
            }, v1), f231(v3, v1))
        }
    }
}

pub fn f232() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::<Bytes>::nil()))))
}

pub fn f233(v0: Adt1) -> R<Option<Bytes>> {
    f330(f331(v0, f232())?)
}

pub fn f234(v0: Adt1, p1085: List<Bytes>) -> R<Option<Bytes>> {
    let m1084 = p1085;
    {
        let mut m1084 = m1084;
        loop {
            let step = {
                let v1 = m1084;
                let m1082 = v1;
                match m1082.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1083 = f332(f64(v0.clone(), v2.clone()), f333())?;
                        match m1083 {
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
                Ok(next) => { m1084 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: Adt17) -> R<(bool, Bytes)> {
    let v1: Adt16 = match v0.clone() {
        Adt17::C0(h1086, _, _, _, _) => {
            h1086
        }
    };
    let v2: Adt19 = f334(v1.clone())?;
    let v3: List<Bytes> = f335(v1.clone());
    let v4: List<Bytes> = f143(match v0.clone() {
        Adt17::C0(_, _, h1087, _, _) => {
            h1087
        }
    }, f8(Str::lit("path")));
    let m1089 = f336(v2, List::<u64>::nil(), f337(v1.clone(), match v0.clone() {
        Adt17::C0(_, h1088, _, _, _) => {
            h1088
        }
    }, 0u64)?);
    match m1089 {
        None => {
            if f339(f340(match v0.clone() {
                Adt17::C0(_, _, _, _, h1091) => {
                    h1091
                }
            }, f126(match v0.clone() {
                Adt17::C0(_, _, _, _, h1092) => {
                    h1092
                }
            }, v4.clone()))?, f340(v4.clone(), f126(match v0.clone() {
                Adt17::C0(_, _, _, _, h1093) => {
                    h1093
                }
            }, v4.clone()))?, f341(match v0.clone() {
                Adt17::C0(_, _, h1094, _, _) => {
                    h1094
                }
            })) {
                let m1098 = f342(f340(match v0.clone() {
                    Adt17::C0(_, _, _, h1095, _) => {
                        h1095
                    }
                }, f126(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h1096, _) => {
                        h1096
                    }
                }))?, f340(v3.clone(), f126(v3.clone(), match v0.clone() {
                    Adt17::C0(_, _, _, h1097, _) => {
                        h1097
                    }
                }))?);
                match m1098 {
                    None => {
                        let m1101 = f343(f143(match v0 {
                            Adt17::C0(_, h1100, _, _, _) => {
                                h1100
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m1101 {
                            None => {
                                let m1102 = f343(v4, List::<Bytes>::nil());
                                match m1102 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f338(f126(v3, match v0 {
                            Adt17::C0(_, _, _, h1099, _) => {
                                h1099
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f338(match v1 {
                Adt16::C0(_, _, _, _, h1090) => {
                    h1090
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f236(v0: List<Bytes>) -> List<Bytes> {
    f121(Fn1::F344, List::<Bytes>::nil(), v0)
}

pub fn f237(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f345(v0, v1)
}

pub fn f238(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f346(f347(f348(v0))?))
}

pub fn f239(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f240(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1103 = f57(v0, v2)?;
    match m1103 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f349(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
            let m1104 = v4.clone();
            match m1104.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m1106 = f93(f94({
                        let m1105 = f94(v3.clone(), f8(Str::lit("source")));
                        match m1105 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m1106 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f350(f351(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f241(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1107 = v0;
    match m1107.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f352(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
                let m1111 = f44(f46(v1.clone(), 6u64, {
                    let a1109 = {
                        let a1108 = v1;
                        length_bytes(a1108)
                    };
                    let a1110 = 6u64;
                    nat_sub(a1109, a1110)
                }), 34u8)?;
                match m1111.uncons() {
                    None => {
                        f241(v2)
                    }
                    Some((v3, v4)) => {
                        let m1112 = v4;
                        match m1112.uncons() {
                            None => {
                                f241(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f241(v2)?))
                            }
                        }
                    }
                }
            } else {
                f241(v2)
            }
        }
    }
}

pub fn f242(p1115: List<Bytes>, p1116: List<Bytes>) -> List<Bytes> {
    let m1114 = (p1115, p1116);
    {
        let mut m1114 = m1114;
        loop {
            let step = {
                let (v0, v1) = m1114;
                let m1113 = v0;
                match m1113.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f243() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f244(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1117 = v1;
    match m1117.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f353(v0.clone(), v3)?;
            if f52(f354(f278(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f355(v5)?, f244(v0, v4, v2)?))
            } else {
                f244(v0, v4, v2)
            }
        }
    }
}

pub fn f245(v0: Adt1, p1121: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m1120 = p1121;
    {
        let mut m1120 = m1120;
        loop {
            let step = {
                let v1 = m1120;
                let m1118 = v1;
                match m1118.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt2 = f353(v0.clone(), v3.clone())?;
                        if if f52(f354(f278(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f32(f277({
                                let m1119 = f275(v5.clone(), f8(Str::lit("schema")));
                                match m1119 {
                                    None => {
                                        Adt2::C8
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f64(v0.clone(), f355(v5)?))
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
                Ok(next) => { m1120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f139(match v0.clone() {
        Adt1::C0(_, h1122, _, _) => {
            h1122
        }
    }, f259())) {
        Ok(Adt12::C1(List::cons(f259(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f152(f146(f356(f148(match v0.clone() {
            Adt1::C0(_, h1123, _, _) => {
                h1123
            }
        }, f259())))?, v1.clone());
        let m1124 = f56(v0.clone(), List::cons(f154(), List::cons(f212(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1124.uncons() {
            None => {
                let m1125 = f56(v0.clone(), f357(v0.clone(), v2.clone())?);
                match m1125.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f358(v0.clone(), v2, v1)?;
                        let (h1126, _) = v7;
                        if h1126 {
                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), {
                                let (_, h1127) = v7;
                                h1127
                            }))
                        } else {
                            let m1128 = f125(v0.clone());
                            match m1128.uncons() {
                                None => {
                                    let m1129 = f127(v0.clone())?;
                                    match m1129.uncons() {
                                        None => {
                                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                                let (_, h1130) = v7;
                                                h1130
                                            }, List::cons(f7(f146(f359(f128(v0.clone())?, f136(v0.clone(), f137(v0.clone())?)?))?), List::cons(f360(v0)?, List::<Bytes>::nil()))))))
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

pub fn f247(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f361(f97(v0, f154())?)?)))
}

pub fn f248() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f249(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1131 = v0;
    match m1131.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f250(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt20 = f362(v0.clone(), 0u64, {
        let a1132 = v0;
        length_bytes(a1132)
    }, Adt20::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt20 = {
        let a1135 = {
            let a1134 = match v1.clone() {
                Adt20::C0(_, _, h1133, _, _) => {
                    h1133
                }
            };
            length_bytes(a1134)
        };
        let a1136 = 0u64;
        if if nat_eq(a1135, a1136) {
            let m1138 = match v1.clone() {
                Adt20::C0(_, _, _, h1137, _) => {
                    h1137
                }
            };
            match m1138.uncons() {
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
        Adt20::C0(_, _, _, _, h1139) => {
            h1139
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f251(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt18>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m1140 = v1;
    match m1140.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f126(f365(v5.clone(), v2.clone()), f366(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f367(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a1142 = {
                    let a1141 = v7.clone();
                    length_list(a1141)
                };
                let a1143 = 0u64;
                if nat_eq(a1142, a1143) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f123(v7, f8(Str::lit("; ")))
                }
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f251(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f252(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1144 = v0;
    match m1144.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1145 = v1;
            match m1145.uncons() {
                None => {
                    f252(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f252(v2))
                }
            }
        }
    }
}

pub fn f253(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1146 = v1;
    match m1146.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f220(v2.clone(), f8(Str::lit("domain"))) {
                f368(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f58(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m1147 = f94(v2.clone(), f8(Str::lit("role")));
                    match m1147 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f58(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f369(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m1148 = f94(v2.clone(), f8(Str::lit("origin")));
                    match m1148 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f58(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m1149 = f94(v2, f8(Str::lit("evidence")));
                    match m1149 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f58(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f253(v0, v3))
            } else {
                f253(v0, v3)
            }
        }
    }
}

pub fn f254(v0: List<Adt14>) -> List<Bytes> {
    let m1150 = v0;
    match m1150.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f254(v2))
        }
    }
}

pub fn f255(v0: List<Adt14>) -> List<Bytes> {
    let m1151 = v0;
    match m1151.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f370(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f255(v2))
        }
    }
}

pub fn f256(p1157: List<Bytes>) -> R<bool> {
    let m1156 = p1157;
    {
        let mut m1156 = m1156;
        loop {
            let step = {
                let v0 = m1156;
                let m1152 = v0;
                match m1152.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1154 = {
                            let a1153 = f197(v1)?;
                            length_bytes(a1153)
                        };
                        let a1155 = 0u64;
                        if nat_eq(a1154, a1155) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
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

pub fn f257(v0: List<Bytes>) -> R<bool> {
    if f266(f291(v0.clone())?) {
        Ok(f51({
            let a1159 = {
                let a1158 = f268(f291(v0)?);
                length_list(a1158)
            };
            let a1160 = 0u64;
            nat_eq(a1159, a1160)
        }))
    } else {
        Ok(false)
    }
}

pub fn f258(v0: List<Bytes>) -> R<bool> {
    if f266(f270(v0.clone(), f8(Str::lit("requirement")))?) {
        if f266(f270(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1162 = {
                    let a1161 = f223(f268(f270(v0.clone(), f8(Str::lit("requirement")))?), f268(f270(v0, f8(Str::lit("admission")))?));
                    length_list(a1161)
                };
                let a1163 = 0u64;
                nat_eq(a1162, a1163)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f259() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f260(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f261() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f262(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1164 = v0;
    match m1164.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f197(v1)?;
            let a1166 = {
                let a1165 = v3.clone();
                length_bytes(a1165)
            };
            let a1167 = 0u64;
            if nat_eq(a1166, a1167) {
                f262(v2)
            } else {
                let m1168 = f325(v3.clone())?;
                match m1168 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f371(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1169 = f262(v2)?;
                            match m1169 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f372(v5), {
                                        let a1170 = v3;
                                        let a1171 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1170, a1171)
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

pub fn f263() -> Bytes {
    let a1172 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1172)
}

pub fn f264(p1177: List<Bytes>) -> R<Option<Bytes>> {
    let m1176 = p1177;
    {
        let mut m1176 = m1176;
        loop {
            let step = {
                let v0 = m1176;
                let m1173 = v0;
                match m1173.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1174 = f325(f197(v1)?)?;
                        match m1174 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1175 = f94(v3.clone(), f8(Str::lit("source")));
                                match m1175 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f373(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m1176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f265(v0: Bytes) -> Bytes {
    let a1182 = {
        let a1178 = Str::lit("binding ");
        utf8_encode(a1178)
    };
    let a1183 = {
        let a1180 = v0;
        let a1181 = {
            let a1179 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1179)
        };
        append_bytes(a1180, a1181)
    };
    append_bytes(a1182, a1183)
}

pub fn f266(v0: Option<List<Adt14>>) -> bool {
    let m1184 = v0;
    match m1184 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f267(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m1185 = v0;
    match m1185.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f197(v1)?;
            let a1187 = {
                let a1186 = v3.clone();
                length_bytes(a1186)
            };
            let a1188 = 0u64;
            if nat_eq(a1187, a1188) {
                f267(v2)
            } else {
                let m1189 = f325(v3)?;
                match m1189 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if f118(f118(f220(v4.clone(), f8(Str::lit("authority"))), f220(v4.clone(), f8(Str::lit("row")))), f118(f118(f220(v4.clone(), f8(Str::lit("construct"))), f220(v4.clone(), f8(Str::lit("artifact")))), f85(f58(v4.clone(), f8(Str::lit("reader"))), f8(Str::lit("read-"))))) {
                            let m1190 = f267(v2)?;
                            match m1190 {
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

pub fn f268(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1191 = v0;
    match m1191 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f269(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1192 = v0;
    match m1192.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f197(v1)?;
            let a1194 = {
                let a1193 = v3.clone();
                length_bytes(a1193)
            };
            let a1195 = 0u64;
            if nat_eq(a1194, a1195) {
                f269(v2)
            } else {
                let m1196 = f43(v3.clone())?;
                match m1196 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f374(v4.clone()) {
                            let m1197 = f269(v2)?;
                            match m1197 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f243(), List::cons(f8(Str::lit("/")), List::cons(f375(f278(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1198 = v3;
                                        let a1199 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1198, a1199)
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

pub fn f270(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m1200 = v0;
    match m1200.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f197(v2)?;
            let a1202 = {
                let a1201 = v4.clone();
                length_bytes(a1201)
            };
            let a1203 = 0u64;
            if nat_eq(a1202, a1203) {
                f270(v3, v1)
            } else {
                let m1204 = f325(v4)?;
                match m1204 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f376(v5.clone());
                        let a1206 = {
                            let a1205 = v6.clone();
                            length_bytes(a1205)
                        };
                        let a1207 = 0u64;
                        if nat_eq(a1206, a1207) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m1208 = f270(v3, v1.clone())?;
                            match m1208 {
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

pub fn f271(p1211: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1210 = p1211;
    {
        let mut m1210 = m1210;
        loop {
            let step = {
                let v0 = m1210;
                let m1209 = v0;
                match m1209.uncons() {
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
                Ok(next) => { m1210 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f272(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f273(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f101(Adt14::C3(std::rc::Rc::new(f102(v1))))?))
}

pub fn f274(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f101(Adt14::C3(std::rc::Rc::new(f377(f146(f126(v1, f214(f97(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f275(p1217: Adt2, v1: Bytes) -> Option<Adt2> {
    let m1216 = p1217;
    {
        let mut m1216 = m1216;
        loop {
            let step = {
                let v0 = m1216;
                let m1212 = v0;
                match m1212 {
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
                    Adt2::C7(r1213) => {
                        let v5 = (*r1213).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C8 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C9(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(v2, r1214, r1215) => {
                        let v3 = (*r1214).clone();
                        let v4 = (*r1215).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1216 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1218 = f56(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m1218.uncons() {
        None => {
            let m1220 = f271(f132(List::cons(Adt14::C4(std::rc::Rc::new({
                let m1219 = f378(v4.clone());
                match m1219 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m1220 {
                None => {
                    f379(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f272(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f277(v0: Adt2) -> R<Bytes> {
    let a1221 = f380(v0)?;
    let a1222 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a1221, a1222))
}

pub fn f278(v0: Adt2, v1: Bytes) -> Bytes {
    let m1223 = f381(f275(v0, v1));
    match m1223 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f279(v0: List<Adt2>) -> List<Bytes> {
    let m1224 = v0;
    match m1224.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1225 = f381(Some(v1));
            match m1225 {
                None => {
                    f279(v2)
                }
                Some(v3) => {
                    List::cons(v3, f279(v2))
                }
            }
        }
    }
}

pub fn f280(v0: Option<Adt2>) -> List<Adt2> {
    let m1226 = v0;
    match m1226 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f382(v1)
        }
    }
}

pub fn f281(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f282(v0: Bytes) -> R<Bytes> {
    Ok(f383(f384(v0)?))
}

pub fn f283(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: List<Bytes>, v5: Bytes) -> R<Adt12> {
    let v6: Bytes = {
        let a1227 = f8(Str::lit("registry-read "));
        let a1228 = v5;
        append_bytes(a1227, a1228)
    };
    if f51(f52(v6.clone(), match v0.clone() {
        Adt1::C0(_, _, h1229, _) => {
            h1229
        }
    })) {
        Ok(Adt12::C2(f385(), v4, v6))
    } else {
        let m1230 = f56(v0.clone(), List::cons(f307(), List::<Bytes>::nil()));
        match m1230.uncons() {
            None => {
                f386(v0, v1, f307(), v2, v3, v4)
            }
            Some((v7, v8)) => {
                Ok(Adt12::C0(List::cons(v7, v8)))
            }
        }
    }
}

pub fn f284() -> Bytes {
    f387()
}

pub fn f285(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f289(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f388(v1, v4)?)?), List::cons(f274(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f286(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1231 = v0;
    match m1231.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f287() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f288(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f350(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f289(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f389(f151(f96(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f390(f391(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f290(p1234: List<Adt14>) -> bool {
    let m1233 = p1234;
    {
        let mut m1233 = m1233;
        loop {
            let step = {
                let v0 = m1233;
                let m1232 = v0;
                match m1232.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f143(f392(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m1235 = v0;
    match m1235.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f197(v1)?;
            let a1237 = {
                let a1236 = v3.clone();
                length_bytes(a1236)
            };
            let a1238 = 0u64;
            if nat_eq(a1237, a1238) {
                f291(v2)
            } else {
                let m1239 = f325(v3)?;
                match m1239 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f220(v4.clone(), f8(Str::lit("agent"))) {
                            if f220(v4.clone(), f8(Str::lit("at"))) {
                                if f220(v4.clone(), f8(Str::lit("quote"))) {
                                    f393(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m1240 = f291(v2)?;
                            match m1240 {
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

pub fn f292() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f293(p1243: List<Adt14>, v1: Bytes, v2: Bytes) -> bool {
    let m1242 = p1243;
    {
        let mut m1242 = m1242;
        loop {
            let step = {
                let v0 = m1242;
                let m1241 = v0;
                match m1241.uncons() {
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
                Ok(next) => { m1242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt14 {
    Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt14::C2(v2)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt14::C2(v0)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5))))))))))))
}

pub fn f295(v0: u64) -> bool {
    let a1244 = v0;
    let a1245 = 127u64;
    nat_le(a1244, a1245)
}

pub fn f296(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1246, _) = v1.clone();
        h1246
    };
    let v3: u64 = {
        let (h1247, _) = {
            let (_, h1248) = v1.clone();
            h1248
        };
        h1247
    };
    let v4: List<Bytes> = {
        let (_, h1249) = {
            let (_, h1250) = v1;
            h1250
        };
        h1249
    };
    let m1251 = f207(v0.clone(), v2);
    match m1251 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1252 = v5;
            let a1253 = 10u8;
            if equal(a1252, a1253) {
                Ok(Some(({
                    let a1254 = v2;
                    let a1255 = 1u64;
                    nat_add(a1254, a1255)?
                }, ({
                    let a1256 = v2;
                    let a1257 = 1u64;
                    nat_add(a1256, a1257)?
                }, List::cons(f394(f46(v0, v3, {
                    let a1258 = v2;
                    let a1259 = v3;
                    nat_sub(a1258, a1259)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1260 = v2;
                    let a1261 = 1u64;
                    nat_add(a1260, a1261)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f297(v0: Adt14) -> List<Adt14> {
    let m1262 = v0;
    match m1262 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1263) => {
            let v3 = (*r1263).clone();
            f297(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1264, r1265) => {
            let v1 = (*r1264).clone();
            let v2 = (*r1265).clone();
            List::cons(v1, f297(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f298(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1266 = v0;
    match m1266 {
        Adt14::C0(v6) => {
            Adt14::C0(v6)
        }
        Adt14::C1(v7, v8) => {
            Adt14::C1(v7, v8)
        }
        Adt14::C2(v9) => {
            Adt14::C2(v9)
        }
        Adt14::C3(r1267) => {
            let v10 = (*r1267).clone();
            Adt14::C3(std::rc::Rc::new(v10))
        }
        Adt14::C4(r1268) => {
            let v5 = (*r1268).clone();
            Adt14::C4(std::rc::Rc::new(f298(v5, v1)))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1269, r1270) => {
            let v11 = (*r1269).clone();
            let v12 = (*r1270).clone();
            Adt14::C6(std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
        Adt14::C7(v2, r1271, r1272) => {
            let v3 = (*r1271).clone();
            let v4 = (*r1272).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt14::C7(v2, std::rc::Rc::new(f395(v3, v1.clone())), std::rc::Rc::new(f298(v4, v1)))
            } else {
                Adt14::C7(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f298(v4, v1)))
            }
        }
    }
}

pub fn f299(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1273 = 0u64;
        let a1274 = v1;
        nat_lt(a1273, a1274)
    } else {
        false
    } {
        let a1275 = Bytes::lit(&[0x2d]);
        let a1276 = f396(v1)?;
        Ok(append_bytes(a1275, a1276))
    } else {
        f396(v1)
    }
}

pub fn f300(v0: Bytes) -> R<Bytes> {
    let a1282 = {
        let a1280 = Bytes::lit(&[0x22]);
        let a1281 = {
            let (_, h1277) = {
                let (h1278, _) = f397(Fn5::F398(v0.clone()), {
                    let a1279 = v0;
                    length_bytes(a1279)
                }, (0u64, Bytes::lit(&[])))?;
                h1278
            };
            h1277
        };
        append_bytes(a1280, a1281)
    };
    let a1283 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1282, a1283))
}

pub fn f301(v0: List<Bytes>) -> Bytes {
    let a1286 = {
        let a1284 = Bytes::lit(&[0x5b]);
        let a1285 = f123(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1284, a1285)
    };
    let a1287 = Bytes::lit(&[0x5d]);
    append_bytes(a1286, a1287)
}

pub fn f302(v0: Adt14) -> R<List<Bytes>> {
    let m1288 = v0;
    match m1288 {
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
        Adt14::C6(r1289, r1290) => {
            let v7 = (*r1289).clone();
            let v8 = (*r1290).clone();
            Ok(List::cons(f189(v7)?, f302(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f303(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f399(Fn6::F400, List::<(Bytes, Bytes)>::nil(), f401(v0)?);
    let v2: List<Bytes> = f402(Fn7::F403, List::<Bytes>::nil(), v1)?;
    let a1293 = {
        let a1291 = Bytes::lit(&[0x7b]);
        let a1292 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1291, a1292)
    };
    let a1294 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1293, a1294))
}

pub fn f304(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1295 = v0;
    match m1295 {
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
        Adt14::C7(v9, r1296, r1297) => {
            let v10 = (*r1296).clone();
            let v11 = (*r1297).clone();
            Ok(List::cons((v9, f189(v10)?), f304(v11)?))
        }
    }
}

pub fn f305() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f306(v0: Adt13) -> Bytes {
    let m1298 = v0;
    match m1298 {
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

pub fn f307() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f308(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f404(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f404(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f404(f8(Str::lit("name")), f405(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f404(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f404(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f404(f8(Str::lit("statement")), f406(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f404(f8(Str::lit("media_type")), f407(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f404(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f404(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f404(f8(Str::lit("url")), f192(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f309(v0: Bytes) -> R<u64> {
    let (h1299, _) = f312(Fn3::F408(v0.clone()), {
        let a1300 = v0.clone();
        length_bytes(a1300)
    }, {
        let a1301 = v0;
        length_bytes(a1301)
    })?;
    Ok(h1299)
}

pub fn f310(v0: Bytes, v1: u64) -> R<u64> {
    let (h1302, _) = f312(Fn3::F409(v1, v0), v1, 0u64)?;
    Ok(h1302)
}

pub fn f311(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f116(v0.clone(), {
        let a1303 = v1;
        let a1304 = 1u64;
        nat_add(a1303, a1304)?
    })?;
    let v4: Bytes = {
        let a1305 = v3;
        let a1306 = 117u64;
        if nat_eq(a1305, a1306) {
            f410(v0, v1)?
        } else {
            f411(v3)
        }
    };
    let a1307 = v4.clone();
    let a1308 = Bytes::lit(&[]);
    if equal(a1307, a1308) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1309 = v3;
            let a1310 = 117u64;
            if nat_eq(a1309, a1310) {
                let a1311 = v1;
                let a1312 = 6u64;
                nat_add(a1311, a1312)?
            } else {
                let a1313 = v1;
                let a1314 = 2u64;
                nat_add(a1313, a1314)?
            }
        }, ({
            let a1315 = v2;
            let a1316 = v4;
            append_bytes(a1315, a1316)
        }, 0u64))))
    }
}

pub fn f312(v0: Fn3, p1321: u64, p1322: u64) -> R<(u64, bool)> {
    let m1320 = (p1321, p1322);
    {
        let mut m1320 = m1320;
        loop {
            let step = {
                let (v1, v2) = m1320;
                let m1317 = v1;
                if m1317 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1317 - 1;
                    let m1319 = {
                        let c1318: Fn3 = v0.clone();
                        c1318.apply(v2)?
                    };
                    match m1319 {
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
                Ok(next) => { m1320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f412(v0, v1)
}

pub fn f314(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f413(v0, v1)
}

pub fn f315(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1323 = v0.clone();
        length_bytes(a1323)
    };
    let a1324 = 0u64;
    let a1325 = v1;
    if nat_lt(a1324, a1325) {
        let a1326 = f414(v0.clone())?;
        let a1327 = v1;
        if nat_eq(a1326, a1327) {
            let a1328 = 1u64;
            let a1329 = v1;
            if nat_lt(a1328, a1329) {
                let m1332 = {
                    let a1330 = v0;
                    let a1331 = 0u64;
                    index_bytes(a1330, a1331)
                };
                match m1332 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1333 = v2;
                        let a1334 = 48u8;
                        Ok(!equal(a1333, a1334))
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

pub fn f316(v0: Fn4, p1339: u64, p1340: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1338 = (p1339, p1340);
    {
        let mut m1338 = m1338;
        loop {
            let step = {
                let (v1, v2) = m1338;
                let m1335 = v1;
                if m1335 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1335 - 1;
                    let m1337 = {
                        let c1336: Fn4 = v0.clone();
                        c1336.apply(v2)?
                    };
                    match m1337 {
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
                Ok(next) => { m1338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f415(v0, v1)
}

pub fn f318(v0: Bytes) -> R<u64> {
    let (h1341, _) = f312(Fn3::F416(v0.clone()), {
        let a1343 = {
            let a1342 = v0;
            length_bytes(a1342)
        };
        let a1344 = 1u64;
        nat_add(a1343, a1344)?
    }, 0u64)?;
    Ok(h1341)
}

pub fn f319(v0: u64) -> Bytes {
    let m1345 = v0;
    if m1345 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1345 - 1;
        let a1346 = f8(Str::lit("0"));
        let a1347 = f319(v1);
        append_bytes(a1346, a1347)
    }
}

pub fn f320(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1348 = v0;
    match m1348.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1349 = f94(v1, f8(Str::lit("std")));
            match m1349 {
                None => {
                    f320(v2)
                }
                Some(v3) => {
                    if f220(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f320(v2))
                    } else {
                        f320(v2)
                    }
                }
            }
        }
    }
}

pub fn f321(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1350 = v1;
    match m1350.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt15::C0(h1351, _, _) => {
                    h1351
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1352, _) => {
                        h1352
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f321(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt15::C0(h1353, _, _) => {
                        h1353
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1354, _) => {
                            h1354
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f321(v0, v3))
                } else {
                    f321(v0, v3)
                }
            }
        }
    }
}

pub fn f322(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt18>> {
    let m1355 = v0;
    match m1355.uncons() {
        None => {
            Ok(List::<Adt18>::nil())
        }
        Some((v4, v5)) => {
            if if f417(v4.clone(), v2.clone()) {
                true
            } else {
                f417(v4.clone(), v3.clone())
            } {
                f322(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt18::C0({
                    let (h1356, _) = v4.clone();
                    h1356
                }, {
                    let a1359 = {
                        let a1358 = f146(f418({
                            let (h1357, _) = v4.clone();
                            h1357
                        }, v1.clone()))?;
                        length_list(a1358)
                    };
                    let a1360 = 1u64;
                    if nat_eq(a1359, a1360) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1361) = v4.clone();
                    h1361
                }), f322(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f323(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1362 = v1;
    match m1362.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1363, _) = v2.clone();
                h1363
            }, v0.clone()) {
                List::cons({
                    let (_, h1364) = v2;
                    h1364
                }, f323(v0, v3))
            } else {
                if f52({
                    let (_, h1365) = v2;
                    h1365
                }, v0.clone()) {
                    List::cons({
                        let (h1366, _) = v2;
                        h1366
                    }, f323(v0, v3))
                } else {
                    f323(v0, v3)
                }
            }
        }
    }
}

pub fn f324(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt15::C0(h1367, _, _) => {
            h1367
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1368, _) => {
                h1368
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt15::C0(_, _, h1369) => {
                    h1369
                }
            }) {
                true
            } else {
                if f220(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1370, _) => {
                            h1370
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f325(v0: Bytes) -> R<Option<Adt14>> {
    let m1371 = f43(v0)?;
    match m1371 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v1) => {
            Ok(f92(v1))
        }
    }
}

pub fn f326(v0: Adt14) -> bool {
    if f220(v0.clone(), f8(Str::lit("domain"))) {
        if f220(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f419()) {
                if f420(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f420(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f420(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f420(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f327(v0: Adt14) -> bool {
    if f220(v0.clone(), f8(Str::lit("subject"))) {
        if f220(v0.clone(), f8(Str::lit("object"))) {
            if f220(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f421())
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

pub fn f328(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1372 = f43(v1)?;
    match m1372 {
        None => {
            Ok(f226(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f279(f280(f275(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1373 = f381(f275(v4.clone(), f8(Str::lit("construct"))));
                match m1373 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1374 = v5;
            match m1374.uncons() {
                None => {
                    Ok(f226(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1375 = v9;
                    match m1375.uncons() {
                        None => {
                            Ok(f226(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f51(f32(v8, f8(Str::lit("std")))) {
                                Ok(f226(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f51(f52(v10.clone(), v2)) {
                                    Ok(f226(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f118(f51(f32(v7.clone(), f8(Str::lit("none")))), f51(f52(v7.clone(), v3))) {
                                        Ok(f226(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1376 = f381(f275(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1376 {
                                            None => {
                                                Ok(f226(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1377 = f275(v4, f8(Str::lit("value")));
                                                match m1377 {
                                                    None => {
                                                        Ok(f226(v0, f8(Str::lit("a row has no value"))))
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

pub fn f329(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1378 = v1;
    match m1378.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f422(v3.clone())
            } else {
                f51(f422(v3.clone()))
            } {
                List::cons(f239(v0.clone(), v3), f329(v0, v4, v2))
            } else {
                f329(v0, v4, v2)
            }
        }
    }
}

pub fn f330(p1381: List<Bytes>) -> R<Option<Bytes>> {
    let m1380 = p1381;
    {
        let mut m1380 = m1380;
        loop {
            let step = {
                let v0 = m1380;
                let m1379 = v0;
                match m1379.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f423(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f197(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1380 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1382 = v1;
    match m1382.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f126(f96(f64(v0.clone(), v2))?, f331(v0, v3)?))
        }
    }
}

pub fn f332(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1383 = v0.clone();
        length_bytes(a1383)
    };
    let v3: u64 = {
        let a1384 = v1.clone();
        length_bytes(a1384)
    };
    let v4: u64 = {
        let (h1385, _) = f312(Fn3::F424(v1.clone(), v3, v0.clone(), v2), {
            let a1386 = v2;
            let a1387 = 1u64;
            nat_add(a1386, a1387)?
        }, 0u64)?;
        h1385
    };
    if f425(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f333() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f334(v0: Adt16) -> R<Adt19> {
    Ok(Adt19::C0(f426(match v0.clone() {
        Adt16::C0(h1388, _, _, _, _) => {
            h1388
        }
    }, 0u64)?, f426(match v0.clone() {
        Adt16::C0(_, h1389, _, _, _) => {
            h1389
        }
    }, 0u64)?, f426(match v0.clone() {
        Adt16::C0(_, _, h1390, _, _) => {
            h1390
        }
    }, 0u64)?, f426(match v0 {
        Adt16::C0(_, _, _, h1391, _) => {
            h1391
        }
    }, 0u64)?))
}

pub fn f335(v0: Adt16) -> List<Bytes> {
    f126(match v0.clone() {
        Adt16::C0(h1392, _, _, _, _) => {
            h1392
        }
    }, f126(match v0.clone() {
        Adt16::C0(_, _, h1393, _, _) => {
            h1393
        }
    }, f126(match v0.clone() {
        Adt16::C0(_, _, _, h1394, _) => {
            h1394
        }
    }, match v0 {
        Adt16::C0(_, _, _, _, h1395) => {
            h1395
        }
    })))
}

pub fn f336(v0: Adt19, p1401: List<u64>, p1402: List<Adt21>) -> Option<u64> {
    let m1400 = (p1401, p1402);
    {
        let mut m1400 = m1400;
        loop {
            let step = {
                let (v1, v2) = m1400;
                let m1396 = v2;
                match m1396.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt21>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f427(v0.clone(), v1.clone(), match v3.clone() {
                            Adt21::C0(_, h1397) => {
                                h1397
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt21>), Option<u64>>((List::cons(match v3 {
                                Adt21::C0(h1398, _) => {
                                    h1398
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt21>), Option<u64>>(Some(match v3 {
                                Adt21::C0(h1399, _) => {
                                    h1399
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1400 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: Adt16, v1: List<Adt14>, v2: u64) -> R<List<Adt21>> {
    let m1403 = v1;
    match m1403.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt21::C0(v2, f428(v0.clone(), v3)?), f337(v0, v4, {
                let a1404 = v2;
                let a1405 = 1u64;
                nat_add(a1404, a1405)?
            })?))
        }
    }
}

pub fn f338(p1409: List<Bytes>, p1410: u64) -> Bytes {
    let m1408 = (p1409, p1410);
    {
        let mut m1408 = m1408;
        loop {
            let step = {
                let (v0, v1) = m1408;
                let m1406 = v0;
                match m1406.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1407 = v1;
                        if m1407 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1407 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
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

pub fn f339(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f429(v0.clone(), v1.clone()) {
        if f429(v1, v0) {
            f430(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f340(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1411 = v0;
    match m1411.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f431(v2, v1.clone())?, f340(v3, v1)?))
        }
    }
}

pub fn f341(v0: List<Adt14>) -> List<u64> {
    let m1412 = v0;
    match m1412.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f432(v1), f341(v2))
        }
    }
}

pub fn f342(p1415: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1414 = p1415;
    {
        let mut m1414 = m1414;
        loop {
            let step = {
                let v0 = m1414;
                let m1413 = v0;
                match m1413.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f433(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f343(p1418: List<Bytes>, p1419: List<Bytes>) -> Option<Bytes> {
    let m1417 = (p1418, p1419);
    {
        let mut m1417 = m1417;
        loop {
            let step = {
                let (v0, v1) = m1417;
                let m1416 = v0;
                match m1416.uncons() {
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
                Ok(next) => { m1417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f345(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1420 = v0.clone();
    match m1420.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1421 = v2;
            let a1422 = v1.clone();
            if equal(a1421, a1422) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f346(v0: List<(Bytes, ())>) -> List<Bytes> {
    f236(f434(Fn8::F435, List::<Bytes>::nil(), v0))
}

pub fn f347(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f436(f437(Fn9::F438, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1427 = {
        let (h1423, _) = f439(Fn10::F440, {
            let a1425 = {
                let a1424 = v0;
                length_list(a1424)
            };
            let a1426 = 1u64;
            nat_add(a1425, a1426)?
        }, v1)?;
        h1423
    };
    match m1427.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f348(v0: List<Bytes>) -> List<(Bytes, ())> {
    f441(f442(Fn11::F443, List::<(Bytes, ())>::nil(), v0))
}

pub fn f349(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1428 = f44(v1, 34u8)?;
    match m1428.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1429 = v3;
            match m1429.uncons() {
                None => {
                    Ok(f444(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f350(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f445(v0.clone(), 0u64, {
        let a1430 = v0;
        length_bytes(a1430)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f351(v0: Adt14, v1: Bytes) -> Bytes {
    if f32(f446(f94(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f352(v0: Bytes, v1: Bytes, p1443: u64, p1444: u64) -> R<bool> {
    let m1442 = (p1443, p1444);
    {
        let mut m1442 = m1442;
        loop {
            let step = {
                let (v2, v3) = m1442;
                let m1431 = v3;
                if m1431 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1431 - 1;
                    let m1434 = {
                        let a1432 = v1.clone();
                        let a1433 = v2;
                        index_bytes(a1432, a1433)
                    };
                    match m1434 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1437 = {
                                let a1435 = v0.clone();
                                let a1436 = v2;
                                index_bytes(a1435, a1436)
                            };
                            match m1437 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1438 = v5;
                                    let a1439 = v6;
                                    if equal(a1438, a1439) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1440 = v2;
                                            let a1441 = 1u64;
                                            nat_add(a1440, a1441)?
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
                Ok(next) => { m1442 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1445 = f91(v0, v1)?;
    match m1445 {
        None => {
            Ok(Adt2::C8)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f354(v0: Bytes) -> R<Bytes> {
    let a1446 = f8(Str::lit("Catalog."));
    let a1447 = f375(v0)?;
    Ok(append_bytes(a1446, a1447))
}

pub fn f355(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f375(f278(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f356(v0: List<Bytes>) -> List<Bytes> {
    let m1448 = v0;
    match m1448.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f239(f259(), v1), f356(v2))
            } else {
                f356(v2)
            }
        }
    }
}

pub fn f357(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1449 = v1;
    match m1449.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f126(f447(f448(f96(f64(v0.clone(), v2))?)?), f357(v0, v3)?))
        }
    }
}

pub fn f358(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1450 = v1;
    match m1450.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f449(v0.clone(), v3, v2.clone())?;
            let (h1451, _) = v5.clone();
            if h1451 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f358(v0, v4, v2)?;
                let (h1452, _) = v6.clone();
                if h1452 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1455 = {
                            let (_, h1453) = v5;
                            h1453
                        };
                        let a1456 = {
                            let (_, h1454) = v6;
                            h1454
                        };
                        append_bytes(a1455, a1456)
                    }))
                }
            }
        }
    }
}

pub fn f359(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1457 = v1;
    match m1457.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f220(v2.clone(), f8(Str::lit("subject"))) {
                if f221(v0.clone(), v2.clone()) {
                    f359(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f359(v0, v3))
                }
            } else {
                if f368(v0.clone(), v2.clone()) {
                    f359(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f369(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f359(v0, v3))
                }
            }
        }
    }
}

pub fn f360(v0: Adt1) -> R<Bytes> {
    let a1459 = {
        let a1458 = f129(f130(f252(f249(f250(f64(v0.clone(), f212()))?)), f131(f97(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f132(f97(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1458)
    };
    let a1460 = 0u64;
    if nat_eq(a1459, a1460) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f361(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1461 = v0;
    match m1461.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f123(f214(f187(f94(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f450(f96(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f361(v2)?))
        }
    }
}

pub fn f362(v0: Bytes, p1519: u64, p1520: u64, p1521: Adt20) -> R<Adt20> {
    let m1518 = (p1519, (p1520, p1521));
    {
        let mut m1518 = m1518;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1518;
                let m1462 = v2;
                if m1462 == 0 {
                    Err::<(u64, (u64, Adt20)), R<Adt20>>(Ok(v3))
                } else {
                    let v4 = m1462 - 1;
                    let v5: u64 = f451(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt20::C0(_, h1463, _, _, _) => {
                            h1463
                        }
                    } {
                        Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                            let a1464 = v1;
                            let a1465 = 1u64;
                            nat_add(a1464, a1465)?
                        }, (v4, Adt20::C0(true, false, match v3.clone() {
                            Adt20::C0(_, _, h1466, _, _) => {
                                h1466
                            }
                        }, match v3.clone() {
                            Adt20::C0(_, _, _, h1467, _) => {
                                h1467
                            }
                        }, match v3 {
                            Adt20::C0(_, _, _, _, h1468) => {
                                h1468
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt20::C0(h1469, _, _, _, _) => {
                                h1469
                            }
                        } {
                            let a1470 = v5;
                            let a1471 = 34u64;
                            if nat_eq(a1470, a1471) {
                                let a1474 = f451(v0.clone(), {
                                    let a1472 = v1;
                                    let a1473 = 1u64;
                                    nat_add(a1472, a1473)?
                                })?;
                                let a1475 = 34u64;
                                if nat_eq(a1474, a1475) {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1476 = v1;
                                        let a1477 = 1u64;
                                        nat_add(a1476, a1477)?
                                    }, (v4, Adt20::C0(true, true, f452(match v3.clone() {
                                        Adt20::C0(_, _, h1478, _, _) => {
                                            h1478
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt20::C0(_, _, _, h1479, _) => {
                                            h1479
                                        }
                                    }, match v3 {
                                        Adt20::C0(_, _, _, _, h1480) => {
                                            h1480
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1481 = v1;
                                        let a1482 = 1u64;
                                        nat_add(a1481, a1482)?
                                    }, (v4, Adt20::C0(false, false, match v3.clone() {
                                        Adt20::C0(_, _, h1483, _, _) => {
                                            h1483
                                        }
                                    }, match v3.clone() {
                                        Adt20::C0(_, _, _, h1484, _) => {
                                            h1484
                                        }
                                    }, match v3 {
                                        Adt20::C0(_, _, _, _, h1485) => {
                                            h1485
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                    let a1486 = v1;
                                    let a1487 = 1u64;
                                    nat_add(a1486, a1487)?
                                }, (v4, Adt20::C0(true, false, f452(match v3.clone() {
                                    Adt20::C0(_, _, h1488, _, _) => {
                                        h1488
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt20::C0(_, _, _, h1489, _) => {
                                        h1489
                                    }
                                }, match v3 {
                                    Adt20::C0(_, _, _, _, h1490) => {
                                        h1490
                                    }
                                }))))
                            }
                        } else {
                            let a1491 = v5;
                            let a1492 = 34u64;
                            if nat_eq(a1491, a1492) {
                                Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                    let a1493 = v1;
                                    let a1494 = 1u64;
                                    nat_add(a1493, a1494)?
                                }, (v4, Adt20::C0(true, false, match v3.clone() {
                                    Adt20::C0(_, _, h1495, _, _) => {
                                        h1495
                                    }
                                }, match v3.clone() {
                                    Adt20::C0(_, _, _, h1496, _) => {
                                        h1496
                                    }
                                }, match v3 {
                                    Adt20::C0(_, _, _, _, h1497) => {
                                        h1497
                                    }
                                }))))
                            } else {
                                let a1498 = v5;
                                let a1499 = 44u64;
                                if nat_eq(a1498, a1499) {
                                    Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                        let a1500 = v1;
                                        let a1501 = 1u64;
                                        nat_add(a1500, a1501)?
                                    }, (v4, Adt20::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt20::C0(_, _, h1502, _, _) => {
                                            h1502
                                        }
                                    }, match v3.clone() {
                                        Adt20::C0(_, _, _, h1503, _) => {
                                            h1503
                                        }
                                    }), match v3 {
                                        Adt20::C0(_, _, _, _, h1504) => {
                                            h1504
                                        }
                                    }))))
                                } else {
                                    let a1505 = v5;
                                    let a1506 = 13u64;
                                    if nat_eq(a1505, a1506) {
                                        Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                            let a1507 = v1;
                                            let a1508 = 1u64;
                                            nat_add(a1507, a1508)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1509 = v5;
                                        let a1510 = 10u64;
                                        if nat_eq(a1509, a1510) {
                                            Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                                let a1511 = v1;
                                                let a1512 = 1u64;
                                                nat_add(a1511, a1512)?
                                            }, (v4, f363(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt20)), R<Adt20>>(({
                                                let a1513 = v1;
                                                let a1514 = 1u64;
                                                nat_add(a1513, a1514)?
                                            }, (v4, Adt20::C0(false, false, f452(match v3.clone() {
                                                Adt20::C0(_, _, h1515, _, _) => {
                                                    h1515
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt20::C0(_, _, _, h1516, _) => {
                                                    h1516
                                                }
                                            }, match v3 {
                                                Adt20::C0(_, _, _, _, h1517) => {
                                                    h1517
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
                Ok(next) => { m1518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363(v0: Adt20) -> Adt20 {
    Adt20::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f453(List::cons(match v0.clone() {
        Adt20::C0(_, _, h1522, _, _) => {
            h1522
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1523, _) => {
            h1523
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt20::C0(_, _, _, _, h1524) => {
            h1524
        }
    }))
}

pub fn f364(p1527: List<List<Bytes>>, p1528: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1526 = (p1527, p1528);
    {
        let mut m1526 = m1526;
        loop {
            let step = {
                let (v0, v1) = m1526;
                let m1525 = v0;
                match m1525.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f365(v0: Bytes, v1: List<Adt18>) -> List<Bytes> {
    let m1529 = v1;
    match m1529.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt18::C0(h1530, _, _) => {
                    h1530
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt18::C0(_, h1531, _) => {
                        h1531
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt18::C0(_, _, h1532) => {
                        h1532
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f365(v0, v3))
            } else {
                f365(v0, v3)
            }
        }
    }
}

pub fn f366(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1533 = v1;
    match m1533.uncons() {
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

pub fn f367(p1538: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1537 = p1538;
    {
        let mut m1537 = m1537;
        loop {
            let step = {
                let v0 = m1537;
                let m1534 = v0;
                match m1534.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1535 = v2;
                        match m1535.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1536 = v5;
                                    match m1536.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f454(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1537 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(v0: Adt15, v1: Adt14) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1539, _, _) => {
            h1539
        }
    }) {
        f52(f369(v1), match v0 {
            Adt15::C0(_, h1540, _) => {
                h1540
            }
        })
    } else {
        false
    }
}

pub fn f369(v0: Adt14) -> Bytes {
    let m1541 = f94(v0, f8(Str::lit("evidence")));
    match m1541 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f370(v0: Adt14) -> Bytes {
    if f220(v0.clone(), f8(Str::lit("req"))) {
        let a1542 = f8(Str::lit("req "));
        let a1543 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1542, a1543)
    } else {
        let m1544 = f94(v0.clone(), f8(Str::lit("std")));
        match m1544 {
            None => {
                let m1547 = f94(v0, f8(Str::lit("der")));
                match m1547 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f123(f214(f187(f94(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f220(v1.clone(), f8(Str::lit("row"))) {
                    let a1545 = f8(Str::lit(" row "));
                    let a1546 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1545, a1546)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f371(v0: Bytes, v1: Bytes) -> bool {
    let a1548 = v0;
    let a1549 = f387();
    if equal(a1548, a1549) {
        let a1553 = {
            let a1551 = {
                let a1550 = v1;
                length_bytes(a1550)
            };
            let a1552 = 0u64;
            nat_eq(a1551, a1552)
        };
        bool_not(a1553)
    } else {
        false
    }
}

pub fn f372(v0: Bytes) -> Bytes {
    f7(List::cons(f145(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f373(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1555 = v0;
    let a1556 = {
        let a1554 = Str::lit("redistributable");
        utf8_encode(a1554)
    };
    if equal(a1555, a1556) {
        f455(v1)
    } else {
        Ok(false)
    }
}

pub fn f374(v0: Adt2) -> bool {
    if f456(v0.clone(), f8(Str::lit("name"))) {
        if f457(v0.clone(), f8(Str::lit("schema"))) {
            let m1557 = f378(f279(f280(f275(v0, f8(Str::lit("citation"))))));
            match m1557 {
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

pub fn f375(v0: Bytes) -> R<Bytes> {
    f458(v0.clone(), 0u64, {
        let a1558 = v0;
        length_bytes(a1558)
    }, true, Bytes::lit(&[]))
}

pub fn f376(v0: Adt14) -> Bytes {
    if f220(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f220(v0.clone(), f8(Str::lit("id"))) {
            if f220(v0.clone(), f8(Str::lit("statement"))) {
                if f220(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f393(v0, f8(Str::lit("evidence")))
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

pub fn f377(v0: List<Bytes>) -> Adt14 {
    let m1559 = v0;
    match m1559.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f377(v2)))
        }
    }
}

pub fn f378(v0: List<Bytes>) -> Option<Adt14> {
    let m1560 = v0;
    match m1560.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m1561 = v2;
                match m1561.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m1562 = v2;
                    match m1562.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1563 = v6;
                                match m1563.uncons() {
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
                        let m1564 = v2;
                        match m1564.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1565 = v10;
                                match m1565.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f377(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f379(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m1566 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1566.uncons() {
        None => {
            let m1567 = f378(v4);
            match m1567 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1568 = f8(Str::lit("Catalog."));
                        let a1569 = f375(v3.clone())?;
                        append_bytes(a1568, a1569)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f375(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1570 = f459(v2, v9.clone(), f375(v3.clone())?)?;
                    match m1570 {
                        Ok(v12) => {
                            Ok(Adt12::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f375(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f460(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f460(f97(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1574 = f461(f64(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v10.clone(), 4u64, {
                                    let a1572 = {
                                        let a1571 = v10;
                                        length_bytes(a1571)
                                    };
                                    let a1573 = 4u64;
                                    nat_sub(a1572, a1573)
                                }))?;
                                let a1575 = f8(Str::lit("\u{a}"));
                                append_bytes(a1574, a1575)
                            }), List::cons(f274(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Err(v11) => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v11, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f380(v0: Adt2) -> R<Bytes> {
    let m1576 = v0;
    match m1576 {
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
            f299(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f462(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f463(v9)
        }
        Adt2::C6(r1577) => {
            let v10 = (*r1577).clone();
            Ok(f301(f464(v10)?))
        }
        Adt2::C7(r1578) => {
            let v11 = (*r1578).clone();
            f465(f466(v11)?)
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

pub fn f381(v0: Option<Adt2>) -> Option<Bytes> {
    let m1579 = v0;
    match m1579 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1580 = v1;
            match m1580 {
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

pub fn f382(v0: Adt2) -> List<Adt2> {
    let m1581 = v0;
    match m1581 {
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
        Adt2::C6(r1582) => {
            let v3 = (*r1582).clone();
            f382(v3)
        }
        Adt2::C7(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C8 => {
            List::<Adt2>::nil()
        }
        Adt2::C9(r1583, r1584) => {
            let v1 = (*r1583).clone();
            let v2 = (*r1584).clone();
            List::cons(v1, f382(v2))
        }
        Adt2::C10(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f383(v0: Adt24) -> Bytes {
    let a1605 = f467(match v0.clone() {
        Adt24::C0(h1585, _, _, _, _, _, _, _) => {
            h1585
        }
    });
    let a1606 = {
        let a1603 = f467(match v0.clone() {
            Adt24::C0(_, h1586, _, _, _, _, _, _) => {
                h1586
            }
        });
        let a1604 = {
            let a1601 = f467(match v0.clone() {
                Adt24::C0(_, _, h1587, _, _, _, _, _) => {
                    h1587
                }
            });
            let a1602 = {
                let a1599 = f467(match v0.clone() {
                    Adt24::C0(_, _, _, h1588, _, _, _, _) => {
                        h1588
                    }
                });
                let a1600 = {
                    let a1597 = f467(match v0.clone() {
                        Adt24::C0(_, _, _, _, h1589, _, _, _) => {
                            h1589
                        }
                    });
                    let a1598 = {
                        let a1595 = f467(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h1590, _, _) => {
                                h1590
                            }
                        });
                        let a1596 = {
                            let a1593 = f467(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h1591, _) => {
                                    h1591
                                }
                            });
                            let a1594 = f467(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h1592) => {
                                    h1592
                                }
                            });
                            append_bytes(a1593, a1594)
                        };
                        append_bytes(a1595, a1596)
                    };
                    append_bytes(a1597, a1598)
                };
                append_bytes(a1599, a1600)
            };
            append_bytes(a1601, a1602)
        };
        append_bytes(a1603, a1604)
    };
    append_bytes(a1605, a1606)
}

pub fn f384(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a1607 = v0.clone();
        length_bytes(a1607)
    };
    let v2: u64 = {
        let a1613 = {
            let a1610 = {
                let a1608 = v1;
                let a1609 = 72u64;
                nat_add(a1608, a1609)?
            };
            let a1611 = 64u64;
            let a1612 = 0u64;
            nat_quot(a1610, a1611, a1612)
        };
        let a1614 = 64u64;
        nat_mul(a1613, a1614)?
    };
    let (_, h1615) = f468(Fn12::F469(v0, v1, v2), {
        let a1616 = v2;
        let a1617 = 64u64;
        let a1618 = 0u64;
        nat_quot(a1616, a1617, a1618)
    }, (0u64, f470()))?;
    Ok(h1615)
}

pub fn f385() -> Bytes {
    f8(Str::lit("target/release/verdant_readers"))
}

pub fn f386(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>) -> R<Adt12> {
    let m1619 = f91(v0.clone(), v2.clone())?;
    match m1619 {
        None => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
        }
        Some(v6) => {
            if f32(v4.clone(), f8(Str::lit("none"))) {
                f471(v0, v1, v2, v3, v4, v5, v6)
            } else {
                let v7: Bytes = {
                    let a1620 = f8(Str::lit("Catalog."));
                    let a1621 = v4.clone();
                    append_bytes(a1620, a1621)
                };
                if f51(f472(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, v7.clone(), v3.clone())) {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v3, List::cons(f8(Str::lit(" in model/admissions.json, so it cannot check the entries\u{a}")), List::<Bytes>::nil()))))))))
                } else {
                    let m1622 = f91(v0.clone(), f7(List::cons(f8(Str::lit("encodings/")), List::cons(v4.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))?;
                    match m1622 {
                        None => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: encodings/")), List::cons(v4, List::cons(f8(Str::lit(".schema.json is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v8) => {
                            let m1623 = f473(v8, v6.clone())?;
                            match m1623 {
                                None => {
                                    f471(v0, v1, v2, v3, {
                                        let a1624 = f8(Str::lit("Catalog."));
                                        let a1625 = v4;
                                        append_bytes(a1624, a1625)
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

pub fn f387() -> Bytes {
    let a1626 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1626)
}

pub fn f388(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f94(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f288(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f288(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f288(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f288(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f288(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f288(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f288(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f288(Str::lit("url"), f446(v2.clone(), f8(Str::lit("url"))))?, List::cons(f288(Str::lit("media_type"), f446(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f288(Str::lit("license"), f446(v2.clone(), f8(Str::lit("license"))))?, List::cons(f288(Str::lit("redistribution"), f446(v2, f8(Str::lit("redistribution"))))?, List::cons(f288(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1627 = v1;
        match m1627 {
            None => {
                f7(List::cons(f288(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f288(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f288(Str::lit("artifact"), {
                    let (h1628, _) = v3;
                    h1628
                })?, List::cons(f288(Str::lit("checksum"), {
                    let (_, h1629) = v3;
                    h1629
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f474(f214(f187(f94(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f389(p1632: List<List<Bytes>>, v1: Bytes, p1633: List<Bytes>) -> R<List<Bytes>> {
    let m1631 = (p1632, p1633);
    {
        let mut m1631 = m1631;
        loop {
            let step = {
                let (v0, v2) = m1631;
                let m1630 = v0;
                match m1630.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f475(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f391(f476(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1631 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f309(v0)?))
}

pub fn f391(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1634 = v0;
    match m1634.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1635 = v3.clone();
            match m1635.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1638 = v2;
                    let a1639 = {
                        let a1636 = v1.clone();
                        let a1637 = f391(v3, v1);
                        append_bytes(a1636, a1637)
                    };
                    append_bytes(a1638, a1639)
                }
            }
        }
    }
}

pub fn f392(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1640 = f94(v0, v1);
    match m1640 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f187(f94(v3, v2))
        }
    }
}

pub fn f393(v0: Adt14, v1: Bytes) -> bool {
    let m1641 = f94(v0, v1);
    match m1641 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f394(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a1643 = {
                let a1642 = v0;
                length_bytes(a1642)
            };
            let a1644 = 1u64;
            nat_sub(a1643, a1644)
        })
    } else {
        v0
    }
}

pub fn f395(v0: Adt14, v1: Bytes) -> Adt14 {
    let m1645 = v0;
    match m1645 {
        Adt14::C0(v3) => {
            Adt14::C0(v3)
        }
        Adt14::C1(v4, v5) => {
            Adt14::C1(v4, v5)
        }
        Adt14::C2(v2) => {
            Adt14::C2(f477(f60(), v2, v1))
        }
        Adt14::C3(r1646) => {
            let v6 = (*r1646).clone();
            Adt14::C3(std::rc::Rc::new(v6))
        }
        Adt14::C4(r1647) => {
            let v7 = (*r1647).clone();
            Adt14::C4(std::rc::Rc::new(v7))
        }
        Adt14::C5 => {
            Adt14::C5
        }
        Adt14::C6(r1648, r1649) => {
            let v8 = (*r1648).clone();
            let v9 = (*r1649).clone();
            Adt14::C6(std::rc::Rc::new(v8), std::rc::Rc::new(v9))
        }
        Adt14::C7(v10, r1650, r1651) => {
            let v11 = (*r1650).clone();
            let v12 = (*r1651).clone();
            Adt14::C7(v10, std::rc::Rc::new(v11), std::rc::Rc::new(v12))
        }
    }
}

pub fn f396(v0: u64) -> R<Bytes> {
    f478(f479(), 10u64, v0)
}

pub fn f397(v0: Fn5, p1656: u64, p1657: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1655 = (p1656, p1657);
    {
        let mut m1655 = m1655;
        loop {
            let step = {
                let (v1, v2) = m1655;
                let m1652 = v1;
                if m1652 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1652 - 1;
                    let m1654 = {
                        let c1653: Fn5 = v0.clone();
                        c1653.apply(v2.clone())?
                    };
                    match m1654 {
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
                Ok(next) => { m1655 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f480(v0, v1)
}

pub fn f399(v0: Fn6, p1661: List<(Bytes, Bytes)>, p1662: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1660 = (p1661, p1662);
    {
        let mut m1660 = m1660;
        loop {
            let step = {
                let (v1, v2) = m1660;
                let m1658 = v2;
                match m1658.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1659: Fn6 = v0.clone();
                            c1659.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f481(v0, v1)
}

pub fn f401(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f482(v0)
}

pub fn f402(v0: Fn7, p1666: List<Bytes>, p1667: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1665 = (p1666, p1667);
    {
        let mut m1665 = m1665;
        loop {
            let step = {
                let (v1, v2) = m1665;
                let m1663 = v2;
                match m1663.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1664: Fn7 = v0.clone();
                            c1664.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1665 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f483(v1)?, v0))
}

pub fn f404(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f300(v0)?, List::cons(f8(Str::lit(":")), List::cons(f300(v1)?, List::<Bytes>::nil())))))
}

pub fn f405(v0: Adt13) -> Bytes {
    let m1668 = v0;
    match m1668 {
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

pub fn f406(v0: Adt13) -> Bytes {
    let m1669 = v0;
    match m1669 {
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

pub fn f407(v0: Adt13) -> Bytes {
    let m1670 = v0;
    match m1670 {
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

pub fn f408(v0: Bytes, v1: u64) -> Option<u64> {
    let a1671 = 0u64;
    let a1672 = v1;
    if nat_lt(a1671, a1672) {
        if f484(v0, {
            let a1673 = v1;
            let a1674 = 1u64;
            nat_sub(a1673, a1674)
        }) {
            Some({
                let a1675 = v1;
                let a1676 = 1u64;
                nat_sub(a1675, a1676)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f409(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1677 = v2;
    let a1678 = v0;
    if nat_lt(a1677, a1678) {
        if f484(v1, v2) {
            Ok(Some({
                let a1679 = v2;
                let a1680 = 1u64;
                nat_add(a1679, a1680)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f410(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1683 = f46(v0.clone(), {
        let a1681 = v1;
        let a1682 = 2u64;
        nat_add(a1681, a1682)?
    }, 2u64);
    let a1684 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1683, a1684) {
        let m1687 = f485(f46(v0, {
            let a1685 = v1;
            let a1686 = 4u64;
            nat_add(a1685, a1686)?
        }, 2u64))?;
        match m1687 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f116(v2.clone(), 0u64)?;
                let a1688 = v3;
                let a1689 = 32u64;
                if nat_lt(a1688, a1689) {
                    if f486(v3) {
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

pub fn f411(v0: u64) -> Bytes {
    let a1690 = v0;
    let a1691 = 34u64;
    if nat_eq(a1690, a1691) {
        Bytes::lit(&[0x22])
    } else {
        let a1692 = v0;
        let a1693 = 92u64;
        if nat_eq(a1692, a1693) {
            Bytes::lit(&[0x5c])
        } else {
            let a1694 = v0;
            let a1695 = 98u64;
            if nat_eq(a1694, a1695) {
                Bytes::lit(&[0x08])
            } else {
                let a1696 = v0;
                let a1697 = 102u64;
                if nat_eq(a1696, a1697) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1698 = v0;
                    let a1699 = 110u64;
                    if nat_eq(a1698, a1699) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1700 = v0;
                        let a1701 = 114u64;
                        if nat_eq(a1700, a1701) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1702 = v0;
                            let a1703 = 116u64;
                            if nat_eq(a1702, a1703) {
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

pub fn f412(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f116(v0, v1)?;
    let a1704 = 47u64;
    let a1705 = v2;
    if if nat_lt(a1704, a1705) {
        let a1706 = v2;
        let a1707 = 58u64;
        nat_lt(a1706, a1707)
    } else {
        false
    } {
        Ok(Some({
            let a1708 = v1;
            let a1709 = 1u64;
            nat_add(a1708, a1709)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f413(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1710 = f116(v0, v1)?;
    let a1711 = 48u64;
    if nat_eq(a1710, a1711) {
        Ok(Some({
            let a1712 = v1;
            let a1713 = 1u64;
            nat_add(a1712, a1713)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f414(v0: Bytes) -> R<u64> {
    let (h1714, _) = f312(Fn3::F487(v0.clone()), {
        let a1715 = v0;
        length_bytes(a1715)
    }, 0u64)?;
    Ok(h1714)
}

pub fn f415(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1719 = {
        let a1717 = v0;
        let a1718 = {
            let (h1716, _) = v1;
            h1716
        };
        index_bytes(a1717, a1718)
    };
    match m1719 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1721 = {
                    let (h1720, _) = v1;
                    h1720
                };
                let a1722 = 1u64;
                nat_add(a1721, a1722)?
            }, {
                let a1726 = {
                    let a1724 = {
                        let (_, h1723) = v1;
                        h1723
                    };
                    let a1725 = 10u64;
                    nat_mul(a1724, a1725)?
                };
                let a1727 = f488(v2)?;
                nat_add(a1726, a1727)?
            })))
        }
    }
}

pub fn f416(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f489(v0, v1)
}

pub fn f417(v0: (Bytes, Bytes), p1730: List<(Bytes, Bytes)>) -> bool {
    let m1729 = p1730;
    {
        let mut m1729 = m1729;
        loop {
            let step = {
                let v1 = m1729;
                let m1728 = v1;
                match m1728.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f490(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1729 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f418(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1731 = v1;
    match m1731.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1732, _) = v2;
                h1732
            }, v0.clone()) {
                List::cons({
                    let (_, h1733) = v2;
                    h1733
                }, f418(v0, v3))
            } else {
                f418(v0, v3)
            }
        }
    }
}

pub fn f419() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f420(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1734 = f94(v0, v1);
    match m1734 {
        None => {
            false
        }
        Some(v3) => {
            f220(v3, v2)
        }
    }
}

pub fn f421() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f422(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f423(v0: Bytes) -> R<bool> {
    let v1: Bytes = f197(v0)?;
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
        let a1736 = {
            let a1735 = v1;
            length_bytes(a1735)
        };
        let a1737 = v2;
        nat_sub(a1736, a1737)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1738 = f332(v3.clone(), f8(Str::lit(")")))?;
        match m1738 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1739 = v4;
                    let a1740 = 1u64;
                    nat_add(a1739, a1740)?
                }, {
                    let a1744 = {
                        let a1741 = v3;
                        length_bytes(a1741)
                    };
                    let a1745 = {
                        let a1742 = v4;
                        let a1743 = 1u64;
                        nat_add(a1742, a1743)?
                    };
                    nat_sub(a1744, a1745)
                })
            }
        }
    } else {
        v3
    };
    let a1746 = 0u64;
    let a1747 = v2;
    if nat_lt(a1746, a1747) {
        f491(v5)
    } else {
        Ok(false)
    }
}

pub fn f424(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f425(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1750 = {
            let a1748 = v4;
            let a1749 = v1;
            nat_add(a1748, a1749)?
        };
        let a1751 = v3;
        if nat_lt(a1750, a1751) {
            Ok(Some({
                let a1752 = v4;
                let a1753 = 1u64;
                nat_add(a1752, a1753)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f425(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1758 = {
        let a1755 = v0;
        let a1756 = v2;
        let a1757 = {
            let a1754 = v1.clone();
            length_bytes(a1754)
        };
        slice_bytes(a1755, a1756, a1757)
    };
    match m1758 {
        None => {
            false
        }
        Some(v3) => {
            let a1759 = v3;
            let a1760 = v1;
            equal(a1759, a1760)
        }
    }
}

pub fn f426(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1761 = v0;
    match m1761.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f426(v3, {
                let a1762 = v1;
                let a1763 = 1u64;
                nat_add(a1762, a1763)?
            })?))
        }
    }
}

pub fn f427(v0: Adt19, v1: List<u64>, v2: Adt22) -> bool {
    let m1764 = v2;
    match m1764 {
        Adt22::C0(v3, v4) => {
            if f433(v3, match v0.clone() {
                Adt19::C0(h1765, _, _, _) => {
                    h1765
                }
            }) {
                f492(v0, v4)
            } else {
                false
            }
        }
        Adt22::C1(v5) => {
            f433(v5, match v0 {
                Adt19::C0(_, _, h1766, _) => {
                    h1766
                }
            })
        }
        Adt22::C2(_, v7, v8) => {
            if f429(v7, v1) {
                f433(v8, match v0 {
                    Adt19::C0(_, _, _, h1767) => {
                        h1767
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f428(v0: Adt16, v1: Adt14) -> R<Adt22> {
    let m1768 = f94(v1.clone(), f8(Str::lit("std")));
    match m1768 {
        None => {
            let m1773 = f93(f94(v1.clone(), f8(Str::lit("req"))));
            match m1773 {
                None => {
                    let m1775 = f94(v1, f8(Str::lit("der")));
                    match m1775 {
                        None => {
                            Ok(f493(v0))
                        }
                        Some(v5) => {
                            let m1776 = f494(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1776 {
                                None => {
                                    Ok(f493(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt22::C2(v6, f340(f214(f187(f94(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt16::C0(_, _, _, _, h1777) => {
                                            h1777
                                        }
                                    })?, f431(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt16::C0(_, _, _, h1778, _) => {
                                            h1778
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt22::C1(f431(v4, match v0 {
                        Adt16::C0(_, _, h1774, _, _) => {
                            h1774
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1769 = f93(f94(v2.clone(), f8(Str::lit("row"))));
            match m1769 {
                None => {
                    Ok(Adt22::C0(f431(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt16::C0(h1770, _, _, _, _) => {
                            h1770
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt22::C0(f431(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt16::C0(h1771, _, _, _, _) => {
                            h1771
                        }
                    })?, Some(f431(v3, match v0 {
                        Adt16::C0(_, h1772, _, _, _) => {
                            h1772
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f429(p1781: List<u64>, v1: List<u64>) -> bool {
    let m1780 = p1781;
    {
        let mut m1780 = m1780;
        loop {
            let step = {
                let v0 = m1780;
                let m1779 = v0;
                match m1779.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f433(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f430(p1786: List<u64>) -> bool {
    let m1785 = p1786;
    {
        let mut m1785 = m1785;
        loop {
            let step = {
                let v0 = m1785;
                let m1782 = v0;
                match m1782.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1783 = v1;
                        let a1784 = 7u64;
                        if nat_le(a1783, a1784) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f431(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1787 = f495(v0, v1.clone(), 0u64)?;
    match m1787 {
        None => {
            let a1788 = v1;
            Ok(length_list(a1788))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f432(v0: Adt14) -> u64 {
    let m1789 = f496(f94(v0, f8(Str::lit("stratum"))));
    match m1789 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f433(v0: u64, p1794: List<u64>) -> bool {
    let m1793 = p1794;
    {
        let mut m1793 = m1793;
        loop {
            let step = {
                let v1 = m1793;
                let m1790 = v1;
                match m1790.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1791 = v2;
                        let a1792 = v0;
                        if nat_eq(a1791, a1792) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1793 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Fn8, p1798: List<Bytes>, p1799: List<(Bytes, ())>) -> List<Bytes> {
    let m1797 = (p1798, p1799);
    {
        let mut m1797 = m1797;
        loop {
            let step = {
                let (v1, v2) = m1797;
                let m1795 = v2;
                match m1795.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1796: Fn8 = v0.clone();
                            c1796.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1797 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f435(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1800, _) = v1;
        h1800
    }, v0)
}

pub fn f436(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f497(Fn13::F498, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f437(v0: Fn9, p1804: List<List<(Bytes, ())>>, p1805: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1803 = (p1804, p1805);
    {
        let mut m1803 = m1803;
        loop {
            let step = {
                let (v1, v2) = m1803;
                let m1801 = v2;
                match m1801.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1802: Fn9 = v0.clone();
                            c1802.apply(v1, v3)
                        }, v4))
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

pub fn f438(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f439(v0: Fn10, p1810: u64, p1811: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1809 = (p1810, p1811);
    {
        let mut m1809 = m1809;
        loop {
            let step = {
                let (v1, v2) = m1809;
                let m1806 = v1;
                if m1806 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1806 - 1;
                    let m1808 = {
                        let c1807: Fn10 = v0.clone();
                        c1807.apply(v2.clone())?
                    };
                    match m1808 {
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
                Ok(next) => { m1809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f440(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f499(v0)
}

pub fn f441(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f500(Fn14::F501, List::<(Bytes, ())>::nil(), v0)
}

pub fn f442(v0: Fn11, p1815: List<(Bytes, ())>, p1816: List<Bytes>) -> List<(Bytes, ())> {
    let m1814 = (p1815, p1816);
    {
        let mut m1814 = m1814;
        loop {
            let step = {
                let (v1, v2) = m1814;
                let m1812 = v2;
                match m1812.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1813: Fn11 = v0.clone();
                            c1813.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f444(p1821: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1820 = p1821;
    {
        let mut m1820 = m1820;
        loop {
            let step = {
                let v0 = m1820;
                let m1817 = v0;
                match m1817.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1818, _) = v2;
                            h1818
                        }) {
                            let (_, h1819) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1819)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(v0: Bytes, p1847: u64, p1848: u64, p1849: Bytes) -> R<Bytes> {
    let m1846 = (p1847, (p1848, p1849));
    {
        let mut m1846 = m1846;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1846;
                let m1822 = v2;
                if m1822 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1822 - 1;
                    let m1825 = {
                        let a1823 = v0.clone();
                        let a1824 = v1;
                        index_bytes(a1823, a1824)
                    };
                    match m1825 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f202({
                                let a1826 = v6;
                                let a1827 = 34u64;
                                nat_eq(a1826, a1827)
                            }, {
                                let a1828 = v6;
                                let a1829 = 92u64;
                                nat_eq(a1828, a1829)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1830 = v1;
                                    let a1831 = 1u64;
                                    nat_add(a1830, a1831)?
                                }, (v4, {
                                    let a1834 = v3;
                                    let a1835 = {
                                        let a1832 = f8(Str::lit("\\"));
                                        let a1833 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1832, a1833)
                                    };
                                    append_bytes(a1834, a1835)
                                })))
                            } else {
                                let a1836 = v6;
                                let a1837 = 10u64;
                                if nat_eq(a1836, a1837) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1838 = v1;
                                        let a1839 = 1u64;
                                        nat_add(a1838, a1839)?
                                    }, (v4, {
                                        let a1840 = v3;
                                        let a1841 = f8(Str::lit("\\n"));
                                        append_bytes(a1840, a1841)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1842 = v1;
                                        let a1843 = 1u64;
                                        nat_add(a1842, a1843)?
                                    }, (v4, {
                                        let a1844 = v3;
                                        let a1845 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1844, a1845)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1846 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1850 = v0;
    match m1850 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f447(v0: List<Adt14>) -> List<Bytes> {
    let m1851 = v0;
    match m1851.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f502(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f281(f58(v1, f8(Str::lit("authority")))), f447(v2)))
        }
    }
}

pub fn f448(v0: List<Bytes>) -> R<List<Adt14>> {
    let m1852 = f291(v0.clone())?;
    match m1852 {
        None => {
            Ok(f268(f267(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt14>::nil())
        }
    }
}

pub fn f449(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f96(f64(v0.clone(), v1.clone()))?;
    let m1853 = f291(v3.clone())?;
    match m1853 {
        None => {
            let m1854 = f267(v3.clone())?;
            match m1854 {
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

pub fn f450(v0: List<Bytes>) -> List<Bytes> {
    let m1855 = v0;
    match m1855.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1857 = {
                    let a1856 = v1.clone();
                    length_bytes(a1856)
                };
                let a1858 = 0u64;
                if nat_eq(a1857, a1858) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f450(v2))
        }
    }
}

pub fn f451(v0: Bytes, v1: u64) -> R<u64> {
    let m1861 = {
        let a1859 = v0;
        let a1860 = v1;
        index_bytes(a1859, a1860)
    };
    match m1861 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f452(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1865 = {
        let a1862 = v1;
        let a1863 = v2;
        let a1864 = 1u64;
        slice_bytes(a1862, a1863, a1864)
    };
    match m1865 {
        None => {
            v0
        }
        Some(v3) => {
            let a1866 = v0;
            let a1867 = v3;
            append_bytes(a1866, a1867)
        }
    }
}

pub fn f453(p1870: List<Bytes>, p1871: List<Bytes>) -> List<Bytes> {
    let m1869 = (p1870, p1871);
    {
        let mut m1869 = m1869;
        loop {
            let step = {
                let (v0, v1) = m1869;
                let m1868 = v0;
                match m1868.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: List<Bytes>) -> Bytes {
    let m1872 = v0;
    match m1872.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f455(v0: Bytes) -> R<bool> {
    let m1874 = {
        let a1873 = v0.clone();
        length_bytes(a1873)
    };
    if m1874 == 0 {
        Ok(true)
    } else {
        let v1 = m1874 - 1;
        let m1877 = {
            let a1875 = v0;
            let a1876 = v1;
            index_bytes(a1875, a1876)
        };
        match m1877 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f506({
                    let v3: u8 = v2;
                    let a1922 = {
                        let a1898 = {
                            let a1886 = {
                                let a1880 = {
                                    let a1878 = v3;
                                    let a1879 = 128u8;
                                    fixed_u8::bit_and(a1878, a1879)
                                };
                                let a1881 = 128u8;
                                if equal(a1880, a1881) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1887 = {
                                let a1884 = {
                                    let a1882 = v3;
                                    let a1883 = 64u8;
                                    fixed_u8::bit_and(a1882, a1883)
                                };
                                let a1885 = 64u8;
                                if equal(a1884, a1885) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1886, a1887)?
                        };
                        let a1899 = {
                            let a1896 = {
                                let a1890 = {
                                    let a1888 = v3;
                                    let a1889 = 32u8;
                                    fixed_u8::bit_and(a1888, a1889)
                                };
                                let a1891 = 32u8;
                                if equal(a1890, a1891) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1897 = {
                                let a1894 = {
                                    let a1892 = v3;
                                    let a1893 = 16u8;
                                    fixed_u8::bit_and(a1892, a1893)
                                };
                                let a1895 = 16u8;
                                if equal(a1894, a1895) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1896, a1897)?
                        };
                        nat_add(a1898, a1899)?
                    };
                    let a1923 = {
                        let a1920 = {
                            let a1908 = {
                                let a1902 = {
                                    let a1900 = v3;
                                    let a1901 = 8u8;
                                    fixed_u8::bit_and(a1900, a1901)
                                };
                                let a1903 = 8u8;
                                if equal(a1902, a1903) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1909 = {
                                let a1906 = {
                                    let a1904 = v3;
                                    let a1905 = 4u8;
                                    fixed_u8::bit_and(a1904, a1905)
                                };
                                let a1907 = 4u8;
                                if equal(a1906, a1907) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1908, a1909)?
                        };
                        let a1921 = {
                            let a1918 = {
                                let a1912 = {
                                    let a1910 = v3;
                                    let a1911 = 2u8;
                                    fixed_u8::bit_and(a1910, a1911)
                                };
                                let a1913 = 2u8;
                                if equal(a1912, a1913) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1919 = {
                                let a1916 = {
                                    let a1914 = v3;
                                    let a1915 = 1u8;
                                    fixed_u8::bit_and(a1914, a1915)
                                };
                                let a1917 = 1u8;
                                if equal(a1916, a1917) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1918, a1919)?
                        };
                        nat_add(a1920, a1921)?
                    };
                    nat_add(a1922, a1923)?
                }))
            }
        }
    }
}

pub fn f456(v0: Adt2, v1: Bytes) -> bool {
    let a1925 = {
        let a1924 = f278(v0, v1);
        length_bytes(a1924)
    };
    let a1926 = 0u64;
    !nat_eq(a1925, a1926)
}

pub fn f457(v0: Adt2, v1: Bytes) -> bool {
    let m1927 = f275(v0, v1);
    match m1927 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f458(v0: Bytes, p1948: u64, p1949: u64, p1950: bool, p1951: Bytes) -> R<Bytes> {
    let m1947 = (p1948, (p1949, (p1950, p1951)));
    {
        let mut m1947 = m1947;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1947;
                let m1928 = v2;
                if m1928 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1928 - 1;
                    let m1931 = {
                        let a1929 = v0.clone();
                        let a1930 = v1;
                        index_bytes(a1929, a1930)
                    };
                    match m1931 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f507(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1932 = v1;
                                    let a1933 = 1u64;
                                    nat_add(a1932, a1933)?
                                }, (v5, (true, v4))))
                            } else {
                                if f118(f508(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1934 = v1;
                                        let a1935 = 1u64;
                                        nat_add(a1934, a1935)?
                                    }, (v5, (false, {
                                        let a1939 = v4;
                                        let a1940 = f46({
                                            let a1936 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1936)
                                        }, {
                                            let a1937 = v7;
                                            let a1938 = 97u64;
                                            nat_sub(a1937, a1938)
                                        }, 1u64);
                                        append_bytes(a1939, a1940)
                                    }))))
                                } else {
                                    if f202(f508(v7), f202(f509(v7), f510(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1941 = v1;
                                            let a1942 = 1u64;
                                            nat_add(a1941, a1942)?
                                        }, (v5, (false, {
                                            let a1943 = v4;
                                            let a1944 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1943, a1944)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1945 = v1;
                                            let a1946 = 1u64;
                                            nat_add(a1945, a1946)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1947 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f511(f512(f275(v0.clone(), f8(Str::lit("$defs")))), f512(f275(v0.clone(), f8(Str::lit("definitions")))));
    let m1952 = f513(4096u64, v3.clone(), List::<Bytes>::nil(), f514(v3)?)?;
    match m1952 {
        Ok(v5) => {
            let m1953 = f515(4096u64, f516(f517(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1953 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f391(f242(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f460(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1954 = v0;
    match m1954.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f32(f58(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f518(v5, v1, v2))
            } else {
                List::cons(v4, f460(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f461(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f391(f519(f96(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f462(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f396(v1)?;
    let v4: u64 = {
        let a1955 = v3.clone();
        length_bytes(a1955)
    };
    let v5: Bytes = {
        let a1956 = v2;
        let a1957 = v4;
        if nat_lt(a1956, a1957) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1958 = v4;
                let a1959 = v2;
                nat_sub(a1958, a1959)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1960 = v4;
                let a1961 = v2;
                nat_sub(a1960, a1961)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f319({
                let a1962 = v2;
                let a1963 = v4;
                nat_sub(a1962, a1963)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1964 = f8(Str::lit("-"));
        let a1965 = v5;
        Ok(append_bytes(a1964, a1965))
    } else {
        Ok(v5)
    }
}

pub fn f463(v0: Bytes) -> R<Bytes> {
    let a1970 = f199(v0.clone(), 0u64, {
        let a1967 = {
            let a1966 = v0.clone();
            length_bytes(a1966)
        };
        let a1968 = 1u64;
        nat_add(a1967, a1968)?
    })?;
    let a1971 = {
        let a1969 = v0.clone();
        length_bytes(a1969)
    };
    if nat_eq(a1970, a1971) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1977 = {
            let a1975 = Bytes::lit(&[0x22]);
            let a1976 = f520(v0.clone(), 0u64, {
                let a1973 = {
                    let a1972 = v0;
                    length_bytes(a1972)
                };
                let a1974 = 1u64;
                nat_add(a1973, a1974)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1975, a1976)
        };
        let a1978 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1977, a1978))
    }
}

pub fn f464(v0: Adt2) -> R<List<Bytes>> {
    let m1979 = v0;
    match m1979 {
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
        Adt2::C9(r1980, r1981) => {
            let v12 = (*r1980).clone();
            let v13 = (*r1981).clone();
            Ok(List::cons(f380(v12)?, f464(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f465(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f399(Fn6::F521, List::<(Bytes, Bytes)>::nil(), f401(v0)?);
    let v2: List<Bytes> = f402(Fn7::F522, List::<Bytes>::nil(), v1)?;
    let a1984 = {
        let a1982 = Bytes::lit(&[0x7b]);
        let a1983 = f123(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1982, a1983)
    };
    let a1985 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1984, a1985))
}

pub fn f466(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1986 = v0;
    match m1986 {
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
        Adt2::C10(v14, r1987, r1988) => {
            let v15 = (*r1987).clone();
            let v16 = (*r1988).clone();
            Ok(List::cons((v14, f380(v15)?), f466(v16)?))
        }
    }
}

pub fn f467(v0: u32) -> Bytes {
    let a2017 = f523({
        let a1989 = f524(v0, 28u32);
        let a1990 = 15u32;
        fixed_u32::bit_and(a1989, a1990)
    });
    let a2018 = {
        let a2015 = f523({
            let a1991 = f524(v0, 24u32);
            let a1992 = 15u32;
            fixed_u32::bit_and(a1991, a1992)
        });
        let a2016 = {
            let a2013 = f523({
                let a1993 = f524(v0, 20u32);
                let a1994 = 15u32;
                fixed_u32::bit_and(a1993, a1994)
            });
            let a2014 = {
                let a2011 = f523({
                    let a1995 = f524(v0, 16u32);
                    let a1996 = 15u32;
                    fixed_u32::bit_and(a1995, a1996)
                });
                let a2012 = {
                    let a2009 = f523({
                        let a1997 = f524(v0, 12u32);
                        let a1998 = 15u32;
                        fixed_u32::bit_and(a1997, a1998)
                    });
                    let a2010 = {
                        let a2007 = f523({
                            let a1999 = f524(v0, 8u32);
                            let a2000 = 15u32;
                            fixed_u32::bit_and(a1999, a2000)
                        });
                        let a2008 = {
                            let a2005 = f523({
                                let a2001 = f524(v0, 4u32);
                                let a2002 = 15u32;
                                fixed_u32::bit_and(a2001, a2002)
                            });
                            let a2006 = f523({
                                let a2003 = v0;
                                let a2004 = 15u32;
                                fixed_u32::bit_and(a2003, a2004)
                            });
                            append_bytes(a2005, a2006)
                        };
                        append_bytes(a2007, a2008)
                    };
                    append_bytes(a2009, a2010)
                };
                append_bytes(a2011, a2012)
            };
            append_bytes(a2013, a2014)
        };
        append_bytes(a2015, a2016)
    };
    append_bytes(a2017, a2018)
}

pub fn f468(v0: Fn12, p2022: u64, p2023: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m2021 = (p2022, p2023);
    {
        let mut m2021 = m2021;
        loop {
            let step = {
                let (v1, v2) = m2021;
                let m2019 = v1;
                if m2019 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m2019 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c2020: Fn12 = v0.clone();
                        c2020.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2021 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a2025 = {
            let (h2024, _) = v3.clone();
            h2024
        };
        let a2026 = 64u64;
        nat_add(a2025, a2026)?
    }, f525({
        let (_, h2027) = v3;
        h2027
    }, f526(v0, v1, v2, {
        let (h2028, _) = v3;
        h2028
    })?)?))
}

pub fn f470() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f471(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Adt2) -> R<Adt12> {
    let m2029 = f527(f528(v1.clone(), v3.clone(), v4, v6))?;
    match m2029 {
        Ok(v8) => {
            let v9: Bytes = f502(v1.clone());
            Ok(Adt12::C3(List::cons((v9.clone(), v8), List::cons((f8(Str::lit("model/admissions.json")), f101(Adt14::C3(std::rc::Rc::new(f102(f460(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("row")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5))))))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("translation")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("command")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f377(v5)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt14::C2(f282(f64(v0, v2))?)), std::rc::Rc::new(Adt14::C5))))))), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::<(Bytes, Bytes)>::nil())), f7(List::cons(f8(Str::lit("admitted ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        }
        Err(v7) => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(": ")), List::cons(v7, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
        }
    }
}

pub fn f472(p2033: List<Adt14>, v1: Bytes, v2: Bytes) -> bool {
    let m2032 = p2033;
    {
        let mut m2032 = m2032;
        loop {
            let step = {
                let v0 = m2032;
                let m2030 = v0;
                match m2030.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f118(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2031 = f94(v3, f8(Str::lit("std")));
                            match m2031 {
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
                Ok(next) => { m2032 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(v0: Adt2, v1: Adt2) -> R<Option<Bytes>> {
    f529({
        let a2040 = {
            let a2035 = {
                let a2034 = f530(v0.clone())?;
                length_bytes(a2034)
            };
            let a2036 = 2u64;
            nat_add(a2035, a2036)?
        };
        let a2041 = {
            let a2038 = {
                let a2037 = f530(v1.clone())?;
                length_bytes(a2037)
            };
            let a2039 = 2u64;
            nat_add(a2038, a2039)?
        };
        nat_mul(a2040, a2041)?
    }, v0.clone(), v0, v1, Bytes::lit(&[]))
}

pub fn f474(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f391(f531(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f475(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2042 = f44(v1, 34u8)?;
    match m2042.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2043 = v3;
            match m2043.uncons() {
                None => {
                    Ok(f532(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f476(v0: List<Bytes>) -> List<Bytes> {
    let m2044 = v0;
    match m2044.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f476(v2)
            } else {
                List::cons(v1, f476(v2))
            }
        }
    }
}

pub fn f477(p2047: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2046 = p2047;
    {
        let mut m2046 = m2046;
        loop {
            let step = {
                let v0 = m2046;
                let m2045 = v0;
                match m2045.uncons() {
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
                Ok(next) => { m2046 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2048 = v2;
    let a2049 = 0u64;
    if nat_eq(a2048, a2049) {
        Ok(f533(v0, 0u64))
    } else {
        let (_, h2050) = {
            let (h2051, _) = f397(Fn5::F534(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2051
        };
        Ok(h2050)
    }
}

pub fn f479() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f480(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2055 = {
        let a2053 = v0.clone();
        let a2054 = {
            let (h2052, _) = v1.clone();
            h2052
        };
        index_bytes(a2053, a2054)
    };
    match m2055 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2057 = {
                    let (h2056, _) = v1.clone();
                    h2056
                };
                let a2058 = 1u64;
                nat_add(a2057, a2058)?
            }, {
                let a2061 = {
                    let (_, h2059) = v1;
                    h2059
                };
                let a2062 = f535(v0, {
                    let (h2060, _) = v1;
                    h2060
                }, v2)?;
                append_bytes(a2061, a2062)
            })))
        }
    }
}

pub fn f481(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2063 = v0.clone();
    match m2063.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2066 = {
                let (h2064, _) = v2;
                h2064
            };
            let a2067 = {
                let (h2065, _) = v1.clone();
                h2065
            };
            if equal(a2066, a2067) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f482(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f536(f537(Fn15::F538, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2072 = {
        let (h2068, _) = f539(Fn16::F540, {
            let a2070 = {
                let a2069 = v0;
                length_list(a2069)
            };
            let a2071 = 1u64;
            nat_add(a2070, a2071)?
        }, v1)?;
        h2068
    };
    match m2072.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f483(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2077 = {
        let a2074 = f300({
            let (h2073, _) = v0;
            h2073
        })?;
        let a2075 = Bytes::lit(&[0x3a]);
        append_bytes(a2074, a2075)
    };
    let a2078 = {
        let (_, h2076) = v0;
        h2076
    };
    Ok(append_bytes(a2077, a2078))
}

pub fn f484(v0: Bytes, v1: u64) -> bool {
    let m2079 = f207(v0, v1);
    match m2079 {
        None => {
            false
        }
        Some(v2) => {
            f541(v2)
        }
    }
}

pub fn f485(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2080 = v0.clone();
        length_bytes(a2080)
    };
    let a2084 = {
        let a2081 = v1;
        let a2082 = 2u64;
        let a2083 = 0u64;
        nat_rem(a2081, a2082, a2083)
    };
    let a2085 = 0u64;
    if nat_eq(a2084, a2085) {
        let (_, h2086) = {
            let (h2087, _) = f542(Fn17::F543(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2087
        };
        Ok(h2086)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f486(v0: u64) -> bool {
    let a2088 = v0;
    let a2089 = 8u64;
    if nat_eq(a2088, a2089) {
        true
    } else {
        let a2090 = v0;
        let a2091 = 9u64;
        if nat_eq(a2090, a2091) {
            true
        } else {
            let a2092 = v0;
            let a2093 = 10u64;
            if nat_eq(a2092, a2093) {
                true
            } else {
                let a2094 = v0;
                let a2095 = 12u64;
                if nat_eq(a2094, a2095) {
                    true
                } else {
                    let a2096 = v0;
                    let a2097 = 13u64;
                    nat_eq(a2096, a2097)
                }
            }
        }
    }
}

pub fn f487(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2100 = {
        let a2098 = v0;
        let a2099 = v1;
        index_bytes(a2098, a2099)
    };
    match m2100 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f544(v2)? {
                Ok(Some({
                    let a2101 = v1;
                    let a2102 = 1u64;
                    nat_add(a2101, a2102)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f488(v0: u8) -> R<u64> {
    let a2149 = {
        let v1: u8 = v0;
        let a2147 = {
            let a2123 = {
                let a2111 = {
                    let a2105 = {
                        let a2103 = v1;
                        let a2104 = 128u8;
                        fixed_u8::bit_and(a2103, a2104)
                    };
                    let a2106 = 128u8;
                    if equal(a2105, a2106) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2112 = {
                    let a2109 = {
                        let a2107 = v1;
                        let a2108 = 64u8;
                        fixed_u8::bit_and(a2107, a2108)
                    };
                    let a2110 = 64u8;
                    if equal(a2109, a2110) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2111, a2112)?
            };
            let a2124 = {
                let a2121 = {
                    let a2115 = {
                        let a2113 = v1;
                        let a2114 = 32u8;
                        fixed_u8::bit_and(a2113, a2114)
                    };
                    let a2116 = 32u8;
                    if equal(a2115, a2116) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2122 = {
                    let a2119 = {
                        let a2117 = v1;
                        let a2118 = 16u8;
                        fixed_u8::bit_and(a2117, a2118)
                    };
                    let a2120 = 16u8;
                    if equal(a2119, a2120) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2121, a2122)?
            };
            nat_add(a2123, a2124)?
        };
        let a2148 = {
            let a2145 = {
                let a2133 = {
                    let a2127 = {
                        let a2125 = v1;
                        let a2126 = 8u8;
                        fixed_u8::bit_and(a2125, a2126)
                    };
                    let a2128 = 8u8;
                    if equal(a2127, a2128) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2134 = {
                    let a2131 = {
                        let a2129 = v1;
                        let a2130 = 4u8;
                        fixed_u8::bit_and(a2129, a2130)
                    };
                    let a2132 = 4u8;
                    if equal(a2131, a2132) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2133, a2134)?
            };
            let a2146 = {
                let a2143 = {
                    let a2137 = {
                        let a2135 = v1;
                        let a2136 = 2u8;
                        fixed_u8::bit_and(a2135, a2136)
                    };
                    let a2138 = 2u8;
                    if equal(a2137, a2138) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2144 = {
                    let a2141 = {
                        let a2139 = v1;
                        let a2140 = 1u8;
                        fixed_u8::bit_and(a2139, a2140)
                    };
                    let a2142 = 1u8;
                    if equal(a2141, a2142) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2143, a2144)?
            };
            nat_add(a2145, a2146)?
        };
        nat_add(a2147, a2148)?
    };
    let a2150 = 48u64;
    Ok(nat_sub(a2149, a2150))
}

pub fn f489(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2151 = v0.clone();
        length_bytes(a2151)
    };
    let a2152 = v1;
    let a2153 = v2;
    if if nat_lt(a2152, a2153) {
        let a2158 = f116(v0, {
            let a2156 = v2;
            let a2157 = {
                let a2154 = v1;
                let a2155 = 1u64;
                nat_add(a2154, a2155)?
            };
            nat_sub(a2156, a2157)
        })?;
        let a2159 = 48u64;
        nat_eq(a2158, a2159)
    } else {
        false
    } {
        Ok(Some({
            let a2160 = v1;
            let a2161 = 1u64;
            nat_add(a2160, a2161)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f490(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f118(f32({
        let (h2162, _) = v0;
        h2162
    }, {
        let (h2163, _) = v1;
        h2163
    }), f32({
        let (_, h2164) = v0;
        h2164
    }, {
        let (_, h2165) = v1;
        h2165
    }))
}

pub fn f491(v0: Bytes) -> R<bool> {
    let m2166 = f332(v0.clone(), f8(Str::lit("&&")))?;
    if match m2166 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2167 = f332(v0, f8(Str::lit("||")))?;
        match m2167 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f492(v0: Adt19, v1: Option<u64>) -> bool {
    let m2168 = v1;
    match m2168 {
        None => {
            true
        }
        Some(v2) => {
            f433(v2, match v0 {
                Adt19::C0(_, h2169, _, _) => {
                    h2169
                }
            })
        }
    }
}

pub fn f493(v0: Adt16) -> Adt22 {
    Adt22::C1({
        let a2171 = match v0 {
            Adt16::C0(_, _, h2170, _, _) => {
                h2170
            }
        };
        length_list(a2171)
    })
}

pub fn f494(v0: Bytes) -> Option<Adt23> {
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

pub fn f495(v0: Bytes, p2176: List<Bytes>, p2177: u64) -> R<Option<u64>> {
    let m2175 = (p2176, p2177);
    {
        let mut m2175 = m2175;
        loop {
            let step = {
                let (v1, v2) = m2175;
                let m2172 = v1;
                match m2172.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2173 = v2;
                                let a2174 = 1u64;
                                nat_add(a2173, a2174)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2175 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f496(v0: Option<Adt14>) -> Option<u64> {
    let m2178 = v0;
    match m2178 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2179 = v1;
            match m2179 {
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

pub fn f497(v0: Fn13, p2183: List<List<(Bytes, ())>>, p2184: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2182 = (p2183, p2184);
    {
        let mut m2182 = m2182;
        loop {
            let step = {
                let (v1, v2) = m2182;
                let m2180 = v2;
                match m2180.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2181: Fn13 = v0.clone();
                            c2181.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f499(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2186 = {
        let a2185 = v0.clone();
        length_list(a2185)
    };
    let a2187 = 1u64;
    if nat_le(a2186, a2187) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f545(v0)?))
    }
}

pub fn f500(v0: Fn14, p2191: List<(Bytes, ())>, p2192: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2190 = (p2191, p2192);
    {
        let mut m2190 = m2190;
        loop {
            let step = {
                let (v1, v2) = m2190;
                let m2188 = v2;
                match m2188.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2189: Fn14 = v0.clone();
                            c2189.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f502(v0: Bytes) -> Bytes {
    f7(List::cons(f140(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f503(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f143(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f143(f97(v0, f154())?, f8(Str::lit("at"))));
    let a2194 = {
        let a2193 = v4.clone();
        length_list(a2193)
    };
    let a2195 = 0u64;
    if nat_eq(a2194, a2195) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2198 = {
            let a2196 = v4;
            length_list(a2196)
        };
        let a2199 = {
            let a2197 = v3;
            length_list(a2197)
        };
        if nat_eq(a2198, a2199) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f546(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f504(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt14>) -> R<(bool, Bytes)> {
    let m2200 = v3;
    match m2200.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f202(f67(v0.clone(), f502(v6.clone())), f52(v6.clone(), v2.clone())) {
                f504(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f281(v6.clone()))) {
                    let v9: (bool, Bytes) = f504(v0, v1.clone(), v2, v5)?;
                    let (h2201, _) = v9.clone();
                    if h2201 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2203 = f546(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2204 = {
                                let (_, h2202) = v9;
                                h2202
                            };
                            append_bytes(a2203, a2204)
                        }))
                    }
                } else {
                    if f118(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f472(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2205 = f8(Str::lit("Catalog."));
                        let a2206 = v8.clone();
                        append_bytes(a2205, a2206)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f504(v0, v1.clone(), v2, v5)?;
                        let (h2207, _) = v10.clone();
                        if h2207 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2209 = f546(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2210 = {
                                    let (_, h2208) = v10;
                                    h2208
                                };
                                append_bytes(a2209, a2210)
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
    let v3: List<Adt14> = f268(f270(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f268(f270(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f126(f143(v3.clone(), f8(Str::lit("id"))), f143(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f143(f97(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f126(v6.clone(), f143(f97(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2212 = {
        let a2211 = v7.clone();
        length_list(a2211)
    };
    let a2213 = 0u64;
    if nat_eq(a2212, a2213) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2216 = {
                let a2214 = v7;
                length_list(a2214)
            };
            let a2217 = {
                let a2215 = v5;
                length_list(a2215)
            };
            nat_eq(a2216, a2217)
        }) {
            Ok((false, f546(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f146(f126(f62(f547(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f548(v4), f126(v6, f143(v3, f8(Str::lit("id")))))))?;
            let a2219 = {
                let a2218 = v8.clone();
                length_list(a2218)
            };
            let a2220 = 0u64;
            if nat_eq(a2219, a2220) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f546(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f123(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f506(v0: u64) -> bool {
    let a2221 = v0;
    let a2222 = 47u64;
    nat_eq(a2221, a2222)
}

pub fn f507(v0: u64) -> bool {
    f202({
        let a2223 = v0;
        let a2224 = 95u64;
        nat_eq(a2223, a2224)
    }, f202({
        let a2225 = v0;
        let a2226 = 45u64;
        nat_eq(a2225, a2226)
    }, f202({
        let a2227 = v0;
        let a2228 = 32u64;
        nat_eq(a2227, a2228)
    }, f202({
        let a2229 = v0;
        let a2230 = 46u64;
        nat_eq(a2229, a2230)
    }, {
        let a2231 = v0;
        let a2232 = 47u64;
        nat_eq(a2231, a2232)
    }))))
}

pub fn f508(v0: u64) -> bool {
    f118({
        let a2233 = 97u64;
        let a2234 = v0;
        nat_le(a2233, a2234)
    }, {
        let a2235 = v0;
        let a2236 = 122u64;
        nat_le(a2235, a2236)
    })
}

pub fn f509(v0: u64) -> bool {
    f118({
        let a2237 = 65u64;
        let a2238 = v0;
        nat_le(a2237, a2238)
    }, {
        let a2239 = v0;
        let a2240 = 90u64;
        nat_le(a2239, a2240)
    })
}

pub fn f510(v0: u64) -> bool {
    f118({
        let a2241 = 48u64;
        let a2242 = v0;
        nat_le(a2241, a2242)
    }, {
        let a2243 = v0;
        let a2244 = 57u64;
        nat_le(a2243, a2244)
    })
}

pub fn f511(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m2245 = v0;
    match m2245.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f511(v3, v1))
        }
    }
}

pub fn f512(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m2246 = v0;
    match m2246 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f549(v1)
        }
    }
}

pub fn f513(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m2247 = v0;
    if m2247 == 0 {
        let m2248 = v1;
        match m2248.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2247 - 1;
        let m2249 = v1.clone();
        match m2249.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m2250 = f550(v1.clone(), v2.clone(), v3.clone())?;
                match m2250 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f391(f514(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2253 = f513(v6, f551(v1, {
                            let (h2251, _) = v9.clone();
                            h2251
                        }), List::cons(f552({
                            let (h2252, _) = v9.clone();
                            h2252
                        })?, v2), v3)?;
                        match m2253 {
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

pub fn f514(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m2254 = v0;
    match m2254.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f552({
                let (h2255, _) = v1;
                h2255
            })?, f514(v2)?))
        }
    }
}

pub fn f515(p2261: u64, p2262: List<Adt25>, p2263: List<Bytes>, p2264: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2260 = (p2261, (p2262, (p2263, p2264)));
    {
        let mut m2260 = m2260;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2260;
                let m2256 = v0;
                if m2256 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2256 - 1;
                    let m2257 = v1;
                    match m2257.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2258 = v5;
                            match m2258 {
                                Adt25::C0(v8, v9) => {
                                    if f32(v8.clone(), f553()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f52(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2259 = f555(v8.clone(), v9)?;
                                            match m2259 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f516(f556(v11.clone()), List::cons(Adt25::C1(f557(v11)), v6)), (List::cons(v8, v2), v3))))
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
                                    if f52(f553(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f553(), v2), List::cons(f554(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2260 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2265 = v0;
    match m2265.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f516(v3, v1))
        }
    }
}

pub fn f517(v0: List<(Bytes, Adt2)>) -> R<List<Adt25>> {
    let m2266 = v0;
    match m2266.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f552({
                let (h2267, _) = v1;
                h2267
            })?, {
                let (_, h2268) = v1;
                h2268
            }), f517(v2)?))
        }
    }
}

pub fn f518(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2269 = v0;
    match m2269.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f32(f58(v3.clone(), v1.clone()), v2.clone()) {
                f518(v4, v1, v2)
            } else {
                List::cons(v3, f518(v4, v1, v2))
            }
        }
    }
}

pub fn f519(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2270 = v0;
    match m2270.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f85(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f391(f531(f146(List::cons(v1, f558(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f519(v3, v1)?))
            }
        }
    }
}

pub fn f520(v0: Bytes, p2289: u64, p2290: u64, p2291: Bytes) -> R<Bytes> {
    let m2288 = (p2289, (p2290, p2291));
    {
        let mut m2288 = m2288;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2288;
                let m2271 = v2;
                if m2271 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2271 - 1;
                    let v5: u64 = f199(v0.clone(), v1, {
                        let a2275 = {
                            let a2273 = {
                                let a2272 = v0.clone();
                                length_bytes(a2272)
                            };
                            let a2274 = v1;
                            nat_sub(a2273, a2274)
                        };
                        let a2276 = 1u64;
                        nat_add(a2275, a2276)?
                    })?;
                    let v6: Bytes = {
                        let a2279 = v3;
                        let a2280 = f46(v0.clone(), v1, {
                            let a2277 = v5;
                            let a2278 = v1;
                            nat_sub(a2277, a2278)
                        });
                        append_bytes(a2279, a2280)
                    };
                    let m2283 = {
                        let a2281 = v0.clone();
                        let a2282 = v5;
                        index_bytes(a2281, a2282)
                    };
                    match m2283 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2284 = v5;
                                let a2285 = 1u64;
                                nat_add(a2284, a2285)?
                            }, (v4, {
                                let a2286 = v6;
                                let a2287 = f535(v0.clone(), v5, v7)?;
                                append_bytes(a2286, a2287)
                            })))
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

pub fn f521(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f481(v0, v1)
}

pub fn f522(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f559(v1)?, v0))
}

pub fn f523(v0: u32) -> Bytes {
    let a2294 = {
        let a2292 = v0;
        let a2293 = 8u32;
        fixed_u32::bit_and(a2292, a2293)
    };
    let a2295 = 0u32;
    if equal(a2294, a2295) {
        let a2298 = {
            let a2296 = v0;
            let a2297 = 4u32;
            fixed_u32::bit_and(a2296, a2297)
        };
        let a2299 = 0u32;
        if equal(a2298, a2299) {
            let a2302 = {
                let a2300 = v0;
                let a2301 = 2u32;
                fixed_u32::bit_and(a2300, a2301)
            };
            let a2303 = 0u32;
            if equal(a2302, a2303) {
                let a2306 = {
                    let a2304 = v0;
                    let a2305 = 1u32;
                    fixed_u32::bit_and(a2304, a2305)
                };
                let a2307 = 0u32;
                if equal(a2306, a2307) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2310 = {
                    let a2308 = v0;
                    let a2309 = 1u32;
                    fixed_u32::bit_and(a2308, a2309)
                };
                let a2311 = 0u32;
                if equal(a2310, a2311) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2314 = {
                let a2312 = v0;
                let a2313 = 2u32;
                fixed_u32::bit_and(a2312, a2313)
            };
            let a2315 = 0u32;
            if equal(a2314, a2315) {
                let a2318 = {
                    let a2316 = v0;
                    let a2317 = 1u32;
                    fixed_u32::bit_and(a2316, a2317)
                };
                let a2319 = 0u32;
                if equal(a2318, a2319) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2322 = {
                    let a2320 = v0;
                    let a2321 = 1u32;
                    fixed_u32::bit_and(a2320, a2321)
                };
                let a2323 = 0u32;
                if equal(a2322, a2323) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2326 = {
            let a2324 = v0;
            let a2325 = 4u32;
            fixed_u32::bit_and(a2324, a2325)
        };
        let a2327 = 0u32;
        if equal(a2326, a2327) {
            let a2330 = {
                let a2328 = v0;
                let a2329 = 2u32;
                fixed_u32::bit_and(a2328, a2329)
            };
            let a2331 = 0u32;
            if equal(a2330, a2331) {
                let a2334 = {
                    let a2332 = v0;
                    let a2333 = 1u32;
                    fixed_u32::bit_and(a2332, a2333)
                };
                let a2335 = 0u32;
                if equal(a2334, a2335) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2338 = {
                    let a2336 = v0;
                    let a2337 = 1u32;
                    fixed_u32::bit_and(a2336, a2337)
                };
                let a2339 = 0u32;
                if equal(a2338, a2339) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2342 = {
                let a2340 = v0;
                let a2341 = 2u32;
                fixed_u32::bit_and(a2340, a2341)
            };
            let a2343 = 0u32;
            if equal(a2342, a2343) {
                let a2346 = {
                    let a2344 = v0;
                    let a2345 = 1u32;
                    fixed_u32::bit_and(a2344, a2345)
                };
                let a2347 = 0u32;
                if equal(a2346, a2347) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2350 = {
                    let a2348 = v0;
                    let a2349 = 1u32;
                    fixed_u32::bit_and(a2348, a2349)
                };
                let a2351 = 0u32;
                if equal(a2350, a2351) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f524(v0: u32, v1: u32) -> u32 {
    let m2354 = {
        let a2352 = v0;
        let a2353 = v1;
        fixed_u32::shift_right(a2352, a2353)
    };
    match m2354 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f525(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f560(Fn18::F561, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2355, _, _, _, _, _, _, _) => {
            h2355
        }
    }, match v0.clone() {
        Adt24::C0(_, h2356, _, _, _, _, _, _) => {
            h2356
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2357, _, _, _, _, _) => {
            h2357
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2358, _, _, _, _) => {
            h2358
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2359, _, _, _) => {
            h2359
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2360, _, _) => {
            h2360
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2361, _) => {
            h2361
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2362) => {
            h2362
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f562(match v0.clone() {
        Adt24::C0(h2363, _, _, _, _, _, _, _) => {
            h2363
        }
    }, match v2.clone() {
        Adt27::C0(h2364, _, _, _, _, _, _, _, _, _) => {
            h2364
        }
    }), f562(match v0.clone() {
        Adt24::C0(_, h2365, _, _, _, _, _, _) => {
            h2365
        }
    }, match v2.clone() {
        Adt27::C0(_, h2366, _, _, _, _, _, _, _, _) => {
            h2366
        }
    }), f562(match v0.clone() {
        Adt24::C0(_, _, h2367, _, _, _, _, _) => {
            h2367
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2368, _, _, _, _, _, _, _) => {
            h2368
        }
    }), f562(match v0.clone() {
        Adt24::C0(_, _, _, h2369, _, _, _, _) => {
            h2369
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2370, _, _, _, _, _, _) => {
            h2370
        }
    }), f562(match v0.clone() {
        Adt24::C0(_, _, _, _, h2371, _, _, _) => {
            h2371
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2372, _, _, _, _, _) => {
            h2372
        }
    }), f562(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2373, _, _) => {
            h2373
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2374, _, _, _, _) => {
            h2374
        }
    }), f562(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2375, _) => {
            h2375
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2376, _, _, _) => {
            h2376
        }
    }), f562(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2377) => {
            h2377
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2378, _, _) => {
            h2378
        }
    })))
}

pub fn f526(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f563(v0.clone(), v1, v2, v3)?, f563(v0.clone(), v1, v2, {
        let a2379 = v3;
        let a2380 = 4u64;
        nat_add(a2379, a2380)?
    })?, f563(v0.clone(), v1, v2, {
        let a2381 = v3;
        let a2382 = 8u64;
        nat_add(a2381, a2382)?
    })?, f563(v0.clone(), v1, v2, {
        let a2383 = v3;
        let a2384 = 12u64;
        nat_add(a2383, a2384)?
    })?, f563(v0.clone(), v1, v2, {
        let a2385 = v3;
        let a2386 = 16u64;
        nat_add(a2385, a2386)?
    })?, f563(v0.clone(), v1, v2, {
        let a2387 = v3;
        let a2388 = 20u64;
        nat_add(a2387, a2388)?
    })?, f563(v0.clone(), v1, v2, {
        let a2389 = v3;
        let a2390 = 24u64;
        nat_add(a2389, a2390)?
    })?, f563(v0.clone(), v1, v2, {
        let a2391 = v3;
        let a2392 = 28u64;
        nat_add(a2391, a2392)?
    })?, f563(v0.clone(), v1, v2, {
        let a2393 = v3;
        let a2394 = 32u64;
        nat_add(a2393, a2394)?
    })?, f563(v0.clone(), v1, v2, {
        let a2395 = v3;
        let a2396 = 36u64;
        nat_add(a2395, a2396)?
    })?, f563(v0.clone(), v1, v2, {
        let a2397 = v3;
        let a2398 = 40u64;
        nat_add(a2397, a2398)?
    })?, f563(v0.clone(), v1, v2, {
        let a2399 = v3;
        let a2400 = 44u64;
        nat_add(a2399, a2400)?
    })?, f563(v0.clone(), v1, v2, {
        let a2401 = v3;
        let a2402 = 48u64;
        nat_add(a2401, a2402)?
    })?, f563(v0.clone(), v1, v2, {
        let a2403 = v3;
        let a2404 = 52u64;
        nat_add(a2403, a2404)?
    })?, f563(v0.clone(), v1, v2, {
        let a2405 = v3;
        let a2406 = 56u64;
        nat_add(a2405, a2406)?
    })?, f563(v0, v1, v2, {
        let a2407 = v3;
        let a2408 = 60u64;
        nat_add(a2407, a2408)?
    })?))
}

pub fn f527(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m2409 = f564(v0.clone());
    match m2409 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f380(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f528(v0: Bytes, v1: Bytes, v2: Bytes, v3: Adt2) -> Adt2 {
    Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("citation")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f565(List::cons(f8(Str::lit("std")), List::cons(v0, List::cons(v1, List::<Bytes>::nil()))))))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C5(v2)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("pointer")), std::rc::Rc::new(Adt2::C5(Bytes::lit(&[]))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("value")), std::rc::Rc::new(v3), std::rc::Rc::new(Adt2::C8))))))))))
}

pub fn f529(v0: u64, v1: Adt2, v2: Adt2, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2410 = v0;
    if m2410 == 0 {
        Ok(Some(f7(List::cons(f8(Str::lit("the schema's references do not end before the instance at \"")), List::cons(v4, List::cons(f8(Str::lit("\" is checked (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
    } else {
        let v5 = m2410 - 1;
        let m2411 = v2.clone();
        match m2411 {
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
                    Ok(f566(v4, f8(Str::lit("is refused by the false schema (JSON Schema 2020-12 Core section 4.3.2)"))))
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
            Adt2::C7(r2425) => {
                let v7 = (*r2425).clone();
                let m2412 = f567(f549(v7));
                match m2412 {
                    None => {
                        Ok(f568(f569(v2.clone(), v3.clone(), v4.clone())?, f568({
                            let m2413 = f381(f275(v2.clone(), f8(Str::lit("$ref"))));
                            match m2413 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v9) => {
                                    let m2414 = f570(v1.clone(), v9)?;
                                    match m2414 {
                                        Ok(v11) => {
                                            f529(v5, v1.clone(), v11, v3.clone(), v4.clone())?
                                        }
                                        Err(v10) => {
                                            Some(v10)
                                        }
                                    }
                                }
                            }
                        }, f568(f571(v5, v1.clone(), f280(f275(v2.clone(), f8(Str::lit("allOf")))), v3.clone(), v4.clone())?, f568({
                            let m2415 = f275(v2.clone(), f8(Str::lit("anyOf")));
                            match m2415 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v12) => {
                                    let a2416 = 0u64;
                                    let a2417 = f572(v5, v1.clone(), f280(Some(v12)), v3.clone(), v4.clone())?;
                                    if nat_lt(a2416, a2417) {
                                        None::<Bytes>
                                    } else {
                                        f566(v4.clone(), f8(Str::lit("matches no anyOf branch (JSON Schema 2020-12 Core section 10.2.1.2)")))
                                    }
                                }
                            }
                        }, f568({
                            let m2418 = f275(v2.clone(), f8(Str::lit("oneOf")));
                            match m2418 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v13) => {
                                    let a2419 = f572(v5, v1.clone(), f280(Some(v13)), v3.clone(), v4.clone())?;
                                    let a2420 = 1u64;
                                    if nat_eq(a2419, a2420) {
                                        None::<Bytes>
                                    } else {
                                        f566(v4.clone(), f8(Str::lit("does not match exactly one oneOf branch (JSON Schema 2020-12 Core section 10.2.1.3)")))
                                    }
                                }
                            }
                        }, {
                            let m2421 = v3.clone();
                            match m2421 {
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
                                Adt2::C6(r2423) => {
                                    let v15 = (*r2423).clone();
                                    let m2422 = f275(v2, f8(Str::lit("items")));
                                    match m2422 {
                                        None => {
                                            None::<Bytes>
                                        }
                                        Some(v16) => {
                                            f575(v5, v1, v16, f382(v15), v4, 0u64)?
                                        }
                                    }
                                }
                                Adt2::C7(r2424) => {
                                    let v14 = (*r2424).clone();
                                    f568(f573(v5, v1.clone(), f512(f275(v2.clone(), f8(Str::lit("properties")))), v3, v4.clone())?, f574(v5, v1, v2, f549(v14), v4)?)
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

pub fn f530(v0: Adt2) -> R<Bytes> {
    f380(v0)
}

pub fn f531(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2426 = v0;
    match m2426.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f350(v1)?, f531(v2)?))
        }
    }
}

pub fn f532(p2429: List<Bytes>, v1: Bytes) -> bool {
    let m2428 = p2429;
    {
        let mut m2428 = m2428;
        loop {
            let step = {
                let v0 = m2428;
                let m2427 = v0;
                match m2427.uncons() {
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
                Ok(next) => { m2428 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f533(v0: Bytes, v1: u64) -> Bytes {
    let m2433 = {
        let a2430 = v0;
        let a2431 = v1;
        let a2432 = 1u64;
        slice_bytes(a2430, a2431, a2432)
    };
    match m2433 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f534(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f576(v1, v0, v2)
}

pub fn f535(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2478 = {
            let a2454 = {
                let a2442 = {
                    let a2436 = {
                        let a2434 = v3;
                        let a2435 = 128u8;
                        fixed_u8::bit_and(a2434, a2435)
                    };
                    let a2437 = 128u8;
                    if equal(a2436, a2437) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2443 = {
                    let a2440 = {
                        let a2438 = v3;
                        let a2439 = 64u8;
                        fixed_u8::bit_and(a2438, a2439)
                    };
                    let a2441 = 64u8;
                    if equal(a2440, a2441) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2442, a2443)?
            };
            let a2455 = {
                let a2452 = {
                    let a2446 = {
                        let a2444 = v3;
                        let a2445 = 32u8;
                        fixed_u8::bit_and(a2444, a2445)
                    };
                    let a2447 = 32u8;
                    if equal(a2446, a2447) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2453 = {
                    let a2450 = {
                        let a2448 = v3;
                        let a2449 = 16u8;
                        fixed_u8::bit_and(a2448, a2449)
                    };
                    let a2451 = 16u8;
                    if equal(a2450, a2451) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2452, a2453)?
            };
            nat_add(a2454, a2455)?
        };
        let a2479 = {
            let a2476 = {
                let a2464 = {
                    let a2458 = {
                        let a2456 = v3;
                        let a2457 = 8u8;
                        fixed_u8::bit_and(a2456, a2457)
                    };
                    let a2459 = 8u8;
                    if equal(a2458, a2459) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2465 = {
                    let a2462 = {
                        let a2460 = v3;
                        let a2461 = 4u8;
                        fixed_u8::bit_and(a2460, a2461)
                    };
                    let a2463 = 4u8;
                    if equal(a2462, a2463) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2464, a2465)?
            };
            let a2477 = {
                let a2474 = {
                    let a2468 = {
                        let a2466 = v3;
                        let a2467 = 2u8;
                        fixed_u8::bit_and(a2466, a2467)
                    };
                    let a2469 = 2u8;
                    if equal(a2468, a2469) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2475 = {
                    let a2472 = {
                        let a2470 = v3;
                        let a2471 = 1u8;
                        fixed_u8::bit_and(a2470, a2471)
                    };
                    let a2473 = 1u8;
                    if equal(a2472, a2473) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2474, a2475)?
            };
            nat_add(a2476, a2477)?
        };
        nat_add(a2478, a2479)?
    };
    let a2480 = v4;
    let a2481 = 34u64;
    if nat_eq(a2480, a2481) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2482 = v4;
        let a2483 = 92u64;
        if nat_eq(a2482, a2483) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2484 = v4;
            let a2485 = 8u64;
            if nat_eq(a2484, a2485) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2486 = v4;
                let a2487 = 12u64;
                if nat_eq(a2486, a2487) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2488 = v4;
                    let a2489 = 10u64;
                    if nat_eq(a2488, a2489) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2490 = v4;
                        let a2491 = 13u64;
                        if nat_eq(a2490, a2491) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2492 = v4;
                            let a2493 = 9u64;
                            if nat_eq(a2492, a2493) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2494 = v4;
                                let a2495 = 32u64;
                                if nat_lt(a2494, a2495) {
                                    let a2496 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2497 = f577(v2)?;
                                    Ok(append_bytes(a2496, a2497))
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

pub fn f536(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f578(Fn19::F579, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f537(v0: Fn15, p2501: List<List<(Bytes, Bytes)>>, p2502: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2500 = (p2501, p2502);
    {
        let mut m2500 = m2500;
        loop {
            let step = {
                let (v1, v2) = m2500;
                let m2498 = v2;
                match m2498.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2499: Fn15 = v0.clone();
                            c2499.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2500 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f539(v0: Fn16, p2507: u64, p2508: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2506 = (p2507, p2508);
    {
        let mut m2506 = m2506;
        loop {
            let step = {
                let (v1, v2) = m2506;
                let m2503 = v1;
                if m2503 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2503 - 1;
                    let m2505 = {
                        let c2504: Fn16 = v0.clone();
                        c2504.apply(v2.clone())?
                    };
                    match m2505 {
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
                Ok(next) => { m2506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f540(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f580(v0)
}

pub fn f541(v0: u8) -> bool {
    let a2509 = v0;
    let a2510 = 32u8;
    if if if equal(a2509, a2510) {
        true
    } else {
        let a2511 = v0;
        let a2512 = 9u8;
        equal(a2511, a2512)
    } {
        true
    } else {
        let a2513 = v0;
        let a2514 = 10u8;
        equal(a2513, a2514)
    } {
        true
    } else {
        let a2515 = v0;
        let a2516 = 12u8;
        if equal(a2515, a2516) {
            true
        } else {
            let a2517 = v0;
            let a2518 = 13u8;
            equal(a2517, a2518)
        }
    }
}

pub fn f542(v0: Fn17, p2523: u64, p2524: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2522 = (p2523, p2524);
    {
        let mut m2522 = m2522;
        loop {
            let step = {
                let (v1, v2) = m2522;
                let m2519 = v1;
                if m2519 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2519 - 1;
                    let m2521 = {
                        let c2520: Fn17 = v0.clone();
                        c2520.apply(v2.clone())?
                    };
                    match m2521 {
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
                Ok(next) => { m2522 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f543(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f581(v0, v1)
}

pub fn f544(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2569 = {
            let a2545 = {
                let a2533 = {
                    let a2527 = {
                        let a2525 = v1;
                        let a2526 = 128u8;
                        fixed_u8::bit_and(a2525, a2526)
                    };
                    let a2528 = 128u8;
                    if equal(a2527, a2528) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2534 = {
                    let a2531 = {
                        let a2529 = v1;
                        let a2530 = 64u8;
                        fixed_u8::bit_and(a2529, a2530)
                    };
                    let a2532 = 64u8;
                    if equal(a2531, a2532) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2533, a2534)?
            };
            let a2546 = {
                let a2543 = {
                    let a2537 = {
                        let a2535 = v1;
                        let a2536 = 32u8;
                        fixed_u8::bit_and(a2535, a2536)
                    };
                    let a2538 = 32u8;
                    if equal(a2537, a2538) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2544 = {
                    let a2541 = {
                        let a2539 = v1;
                        let a2540 = 16u8;
                        fixed_u8::bit_and(a2539, a2540)
                    };
                    let a2542 = 16u8;
                    if equal(a2541, a2542) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2543, a2544)?
            };
            nat_add(a2545, a2546)?
        };
        let a2570 = {
            let a2567 = {
                let a2555 = {
                    let a2549 = {
                        let a2547 = v1;
                        let a2548 = 8u8;
                        fixed_u8::bit_and(a2547, a2548)
                    };
                    let a2550 = 8u8;
                    if equal(a2549, a2550) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2556 = {
                    let a2553 = {
                        let a2551 = v1;
                        let a2552 = 4u8;
                        fixed_u8::bit_and(a2551, a2552)
                    };
                    let a2554 = 4u8;
                    if equal(a2553, a2554) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2555, a2556)?
            };
            let a2568 = {
                let a2565 = {
                    let a2559 = {
                        let a2557 = v1;
                        let a2558 = 2u8;
                        fixed_u8::bit_and(a2557, a2558)
                    };
                    let a2560 = 2u8;
                    if equal(a2559, a2560) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2566 = {
                    let a2563 = {
                        let a2561 = v1;
                        let a2562 = 1u8;
                        fixed_u8::bit_and(a2561, a2562)
                    };
                    let a2564 = 1u8;
                    if equal(a2563, a2564) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2565, a2566)?
            };
            nat_add(a2567, a2568)?
        };
        nat_add(a2569, a2570)?
    };
    let a2571 = 48u64;
    let a2572 = v2;
    if nat_le(a2571, a2572) {
        let a2573 = v2;
        let a2574 = 57u64;
        Ok(nat_le(a2573, a2574))
    } else {
        Ok(false)
    }
}

pub fn f545(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f436({
        let (_, h2575) = {
            let (h2576, _) = f582(Fn20::F583, {
                let a2578 = {
                    let a2577 = v0.clone();
                    length_list(a2577)
                };
                let a2579 = 1u64;
                nat_add(a2578, a2579)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2576
        };
        h2575
    }))
}

pub fn f546(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f547(v0: List<Adt14>) -> List<Bytes> {
    let m2580 = v0;
    match m2580.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2581 = f94(v1, f8(Str::lit("std")));
            match m2581 {
                None => {
                    f547(v2)
                }
                Some(v3) => {
                    List::cons(f58(v3, f8(Str::lit("authority"))), f547(v2))
                }
            }
        }
    }
}

pub fn f548(v0: List<Adt14>) -> List<Bytes> {
    let m2582 = v0;
    match m2582.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2583 = f93(f94(v1, f8(Str::lit("req"))));
            match m2583 {
                None => {
                    f548(v2)
                }
                Some(v3) => {
                    List::cons(v3, f548(v2))
                }
            }
        }
    }
}

pub fn f549(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m2584 = v0;
    match m2584 {
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
        Adt2::C7(r2585) => {
            let v4 = (*r2585).clone();
            f549(v4)
        }
        Adt2::C8 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C9(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C10(v1, r2586, r2587) => {
            let v2 = (*r2586).clone();
            let v3 = (*r2587).clone();
            List::cons((v1, v2), f549(v3))
        }
    }
}

pub fn f550(p2592: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m2591 = p2592;
    {
        let mut m2591 = m2591;
        loop {
            let step = {
                let v0 = m2591;
                let m2588 = v0;
                match m2588.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f584(f585({
                            let (_, h2589) = v3.clone();
                            h2589
                        }, List::<Bytes>::nil())?, f552({
                            let (h2590, _) = v3.clone();
                            h2590
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f551(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m2593 = v0;
    match m2593.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h2594, _) = v2.clone();
                h2594
            }, v1.clone()) {
                f551(v3, v1)
            } else {
                List::cons(v2, f551(v3, v1))
            }
        }
    }
}

pub fn f552(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f375(v0)?;
    if f52(v1.clone(), f586()) {
        f587(v1)
    } else {
        Ok(v1)
    }
}

pub fn f553() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f554() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f555(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    f588(v0, f589(v1))
}

pub fn f556(v0: Adt28) -> List<Adt25> {
    let m2595 = v0;
    match m2595 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f557(v0: Adt28) -> Bytes {
    let m2596 = v0;
    match m2596 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f558(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2597 = v0;
    match m2597.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f558(v3, false))
            } else {
                f558(v3, true)
            }
        }
    }
}

pub fn f559(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2602 = {
        let a2599 = f463({
            let (h2598, _) = v0;
            h2598
        })?;
        let a2600 = Bytes::lit(&[0x3a]);
        append_bytes(a2599, a2600)
    };
    let a2603 = {
        let (_, h2601) = v0;
        h2601
    };
    Ok(append_bytes(a2602, a2603))
}

pub fn f560(v0: Fn18, p2607: u64, p2608: Adt27) -> R<Adt27> {
    let m2606 = (p2607, p2608);
    {
        let mut m2606 = m2606;
        loop {
            let step = {
                let (v1, v2) = m2606;
                let m2604 = v1;
                if m2604 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2604 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2605: Fn18 = v0.clone();
                        c2605.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(v0: Adt27) -> R<Adt27> {
    f590(v0)
}

pub fn f562(v0: u32, v1: u32) -> u32 {
    let m2611 = {
        let a2609 = f591(v0);
        let a2610 = f591(v1);
        fixed_u64::checked_add(a2609, a2610)
    };
    match m2611 {
        None => {
            0u32
        }
        Some(v2) => {
            f592(v2)
        }
    }
}

pub fn f563(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2614 = {
        let a2612 = v3;
        let a2613 = 8u64;
        nat_add(a2612, a2613)?
    };
    let a2615 = v2;
    if nat_eq(a2614, a2615) {
        Ok(f593({
            let a2618 = {
                let a2616 = v1;
                let a2617 = 8u64;
                nat_mul(a2616, a2617)?
            };
            let a2619 = 4294967296u64;
            let a2620 = 0u64;
            nat_quot(a2618, a2619, a2620)
        }))
    } else {
        let a2623 = {
            let a2621 = v3;
            let a2622 = 4u64;
            nat_add(a2621, a2622)?
        };
        let a2624 = v2;
        if nat_eq(a2623, a2624) {
            Ok(f593({
                let a2627 = {
                    let a2625 = v1;
                    let a2626 = 8u64;
                    nat_mul(a2625, a2626)?
                };
                let a2628 = 4294967296u64;
                let a2629 = 0u64;
                nat_rem(a2627, a2628, a2629)
            }))
        } else {
            Ok(f594(f595(v0.clone(), v1, v3), f595(v0.clone(), v1, {
                let a2630 = v3;
                let a2631 = 1u64;
                nat_add(a2630, a2631)?
            }), f595(v0.clone(), v1, {
                let a2632 = v3;
                let a2633 = 2u64;
                nat_add(a2632, a2633)?
            }), f595(v0, v1, {
                let a2634 = v3;
                let a2635 = 3u64;
                nat_add(a2634, a2635)?
            })))
        }
    }
}

pub fn f564(v0: Adt2) -> Option<Bytes> {
    let m2636 = v0;
    match m2636 {
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
        Adt2::C6(r2639) => {
            let v3 = (*r2639).clone();
            f564(v3)
        }
        Adt2::C7(r2640) => {
            let v4 = (*r2640).clone();
            f564(v4)
        }
        Adt2::C8 => {
            None::<Bytes>
        }
        Adt2::C9(r2641, r2642) => {
            let v5 = (*r2641).clone();
            let v6 = (*r2642).clone();
            let m2637 = f564(v5);
            match m2637 {
                None => {
                    f564(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r2643, r2644) => {
            let v9 = (*r2643).clone();
            let v10 = (*r2644).clone();
            let m2638 = f564(v9);
            match m2638 {
                None => {
                    f564(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f565(v0: List<Bytes>) -> Adt2 {
    let m2645 = v0;
    match m2645.uncons() {
        None => {
            Adt2::C8
        }
        Some((v1, v2)) => {
            Adt2::C9(std::rc::Rc::new(Adt2::C5(v1)), std::rc::Rc::new(f565(v2)))
        }
    }
}

pub fn f566(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    Some(f7(List::cons(f8(Str::lit("the instance at \"")), List::cons(v0, List::cons(f8(Str::lit("\" ")), List::cons(v1, List::<Bytes>::nil()))))))
}

pub fn f567(p2650: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m2649 = p2650;
    {
        let mut m2649 = m2649;
        loop {
            let step = {
                let v0 = m2649;
                let m2646 = v0;
                match m2646.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h2647, _) = v1.clone();
                            h2647
                        }, f596()) {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h2648, _) = v1;
                                h2648
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f568(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m2651 = v0;
    match m2651 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f569(v0: Adt2, v1: Adt2, v2: Bytes) -> R<Option<Bytes>> {
    let v3: Bytes = f597(v1.clone());
    let v7: Option<Bytes> = {
        let m2652 = f275(v0.clone(), f8(Str::lit("type")));
        match m2652 {
            None => {
                None::<Bytes>
            }
            Some(v4) => {
                let v6: List<Bytes> = {
                    let m2653 = f381(Some(v4.clone()));
                    match m2653 {
                        None => {
                            f279(f280(Some(v4)))
                        }
                        Some(v5) => {
                            List::cons(v5, List::<Bytes>::nil())
                        }
                    }
                };
                if f598(v6.clone(), v1.clone()) {
                    None::<Bytes>
                } else {
                    f566(v2.clone(), f7(List::cons(f8(Str::lit("is ")), List::cons(v3, List::cons(f8(Str::lit(", not of the type ")), List::cons(f391(v6, f8(Str::lit(" or "))), List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.1.1)")), List::<Bytes>::nil())))))))
                }
            }
        }
    };
    let v9: Option<Bytes> = {
        let m2654 = f275(v0.clone(), f8(Str::lit("enum")));
        match m2654 {
            None => {
                None::<Bytes>
            }
            Some(v8) => {
                if f599(f280(Some(v8)), v1.clone())? {
                    None::<Bytes>
                } else {
                    f566(v2.clone(), f8(Str::lit("is none of the enum values (JSON Schema 2020-12 Validation section 6.1.2)")))
                }
            }
        }
    };
    let v11: Option<Bytes> = {
        let m2655 = f275(v0.clone(), f8(Str::lit("const")));
        match m2655 {
            None => {
                None::<Bytes>
            }
            Some(v10) => {
                if f600(false, v10, v1.clone())? {
                    None::<Bytes>
                } else {
                    f566(v2.clone(), f8(Str::lit("is not the const value (JSON Schema 2020-12 Validation section 6.1.3)")))
                }
            }
        }
    };
    let v41: Option<Bytes> = {
        let m2656 = v1.clone();
        match m2656 {
            Adt2::C0 => {
                None::<Bytes>
            }
            Adt2::C1(v28, _) => {
                f566(v2.clone(), f7(List::cons(f8(Str::lit("is tagged ")), List::cons(v28, List::cons(f8(Str::lit(" and has no JSON type (RFC 8259 section 3)")), List::<Bytes>::nil())))))
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
                let m2657 = f601(v12.clone())?;
                match m2657 {
                    None => {
                        f566(v2.clone(), f8(Str::lit("is not well-formed UTF-8 (RFC 3629 section 4)")))
                    }
                    Some(v13) => {
                        let v14: u64 = f602(v13)?;
                        f568({
                            let m2658 = f603(f275(v0.clone(), f8(Str::lit("minLength"))));
                            match m2658 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v15) => {
                                    let a2659 = v14;
                                    let a2660 = v15;
                                    if nat_lt(a2659, a2660) {
                                        f566(v2.clone(), f8(Str::lit("is shorter than minLength (JSON Schema 2020-12 Validation section 6.3.2)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, f568({
                            let m2661 = f603(f275(v0.clone(), f8(Str::lit("maxLength"))));
                            match m2661 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v16) => {
                                    let a2662 = v16;
                                    let a2663 = v14;
                                    if nat_lt(a2662, a2663) {
                                        f566(v2.clone(), f8(Str::lit("is longer than maxLength (JSON Schema 2020-12 Validation section 6.3.1)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, {
                            let m2664 = f381(f275(v0.clone(), f8(Str::lit("pattern"))));
                            match m2664 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v17) => {
                                    let m2665 = f604(v17.clone(), v12)?;
                                    match m2665 {
                                        Ok(v19) => {
                                            if v19 {
                                                None::<Bytes>
                                            } else {
                                                f566(v2.clone(), f7(List::cons(f8(Str::lit("does not match the pattern ")), List::cons(v17, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.3.3)")), List::<Bytes>::nil())))))
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
            Adt2::C6(r2673) => {
                let v20 = (*r2673).clone();
                let v21: u64 = f605(v20)?;
                f568({
                    let m2666 = f603(f275(v0.clone(), f8(Str::lit("minItems"))));
                    match m2666 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v22) => {
                            let a2667 = v21;
                            let a2668 = v22;
                            if nat_lt(a2667, a2668) {
                                f566(v2.clone(), f8(Str::lit("has fewer items than minItems (JSON Schema 2020-12 Validation section 6.4.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2669 = f603(f275(v0.clone(), f8(Str::lit("maxItems"))));
                    match m2669 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v23) => {
                            let a2670 = v23;
                            let a2671 = v21;
                            if nat_lt(a2670, a2671) {
                                f566(v2.clone(), f8(Str::lit("has more items than maxItems (JSON Schema 2020-12 Validation section 6.4.1)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
            Adt2::C7(r2674) => {
                let v24 = (*r2674).clone();
                let v25: List<Bytes> = f606(f279(f280(f275(v0.clone(), f8(Str::lit("required"))))), f607(f549(v24)));
                let m2672 = v25;
                match m2672.uncons() {
                    None => {
                        None::<Bytes>
                    }
                    Some((v26, _)) => {
                        f566(v2.clone(), f7(List::cons(f8(Str::lit("lacks the required member ")), List::cons(v26, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.5.3)")), List::<Bytes>::nil())))))
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
        let m2675 = f608(v1);
        match m2675 {
            None => {
                None::<Bytes>
            }
            Some(v42) => {
                f568({
                    let m2677 = {
                        let m2676 = f275(v0.clone(), f8(Str::lit("minimum")));
                        match m2676 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v43) => {
                                f608(v43)
                            }
                        }
                    };
                    match m2677 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v44) => {
                            if f609(v42, v44)? {
                                f566(v2.clone(), f8(Str::lit("is less than minimum (JSON Schema 2020-12 Validation section 6.2.4)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m2679 = {
                        let m2678 = f275(v0, f8(Str::lit("maximum")));
                        match m2678 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v45) => {
                                f608(v45)
                            }
                        }
                    };
                    match m2679 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v46) => {
                            if f609(v46, v42)? {
                                f566(v2, f8(Str::lit("is greater than maximum (JSON Schema 2020-12 Validation section 6.2.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
        }
    };
    Ok(f568(v7, f568(v9, f568(v11, f568(v41, v47)))))
}

pub fn f570(v0: Adt2, v1: Bytes) -> R<Result<Adt2, Bytes>> {
    let v2: u64 = {
        let a2680 = v1.clone();
        length_bytes(a2680)
    };
    if f32(v1.clone(), f8(Str::lit("#"))) {
        Ok(Ok::<Adt2, Bytes>(v0))
    } else {
        let m2685 = if f85(v1.clone(), f8(Str::lit("#/$defs/"))) {
            Some((f8(Str::lit("$defs")), f46(v1.clone(), 8u64, {
                let a2681 = v2;
                let a2682 = 8u64;
                nat_sub(a2681, a2682)
            })))
        } else {
            if f85(v1.clone(), f8(Str::lit("#/definitions/"))) {
                Some((f8(Str::lit("definitions")), f46(v1.clone(), 14u64, {
                    let a2683 = v2;
                    let a2684 = 14u64;
                    nat_sub(a2683, a2684)
                })))
            } else {
                None::<(Bytes, Bytes)>
            }
        };
        match m2685 {
            None => {
                Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" is not resolved: only #, #/$defs/NAME and #/definitions/NAME in the same document are (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
            }
            Some(v3) => {
                let v4: Bytes = f610({
                    let a2688 = {
                        let a2687 = {
                            let (_, h2686) = v3.clone();
                            h2686
                        };
                        length_bytes(a2687)
                    };
                    let a2689 = 1u64;
                    nat_add(a2688, a2689)?
                }, {
                    let (_, h2690) = v3;
                    h2690
                }, 0u64, Bytes::lit(&[]))?;
                let m2693 = {
                    let m2692 = f275(v0, {
                        let (h2691, _) = v3;
                        h2691
                    });
                    match m2692 {
                        None => {
                            None::<Adt2>
                        }
                        Some(v5) => {
                            f275(v5, v4)
                        }
                    }
                };
                match m2693 {
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

pub fn f571(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2694 = v0;
    if m2694 == 0 {
        Ok(Some(f8(Str::lit("the allOf branches do not end (JSON Schema 2020-12 Core section 10.2.1.1)"))))
    } else {
        let v5 = m2694 - 1;
        let m2695 = v2;
        match m2695.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f568(f529(v5, v1.clone(), v6, v3.clone(), v4.clone())?, f571(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f572(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<u64> {
    let m2696 = v0;
    if m2696 == 0 {
        Ok(0u64)
    } else {
        let v5 = m2696 - 1;
        let m2697 = v2;
        match m2697.uncons() {
            None => {
                Ok(0u64)
            }
            Some((v6, v7)) => {
                let a2699 = {
                    let m2698 = f529(v5, v1.clone(), v6, v3.clone(), v4.clone())?;
                    match m2698 {
                        None => {
                            1u64
                        }
                        Some(_) => {
                            0u64
                        }
                    }
                };
                let a2700 = f572(v5, v1, v7, v3, v4)?;
                nat_add(a2699, a2700)
            }
        }
    }
}

pub fn f573(v0: u64, v1: Adt2, v2: List<(Bytes, Adt2)>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m2701 = v0;
    if m2701 == 0 {
        Ok(Some(f8(Str::lit("the properties do not end (JSON Schema 2020-12 Core section 10.3.2.1)"))))
    } else {
        let v5 = m2701 - 1;
        let m2702 = v2;
        match m2702.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f568({
                    let m2704 = f275(v3.clone(), {
                        let (h2703, _) = v6.clone();
                        h2703
                    });
                    match m2704 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v8) => {
                            f529(v5, v1.clone(), {
                                let (_, h2705) = v6;
                                h2705
                            }, v8, f611(v4.clone(), {
                                let (h2706, _) = v6;
                                h2706
                            })?)?
                        }
                    }
                }, f573(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f574(v0: u64, v1: Adt2, v2: Adt2, v3: List<(Bytes, Adt2)>, v4: Bytes) -> R<Option<Bytes>> {
    let m2707 = v0;
    if m2707 == 0 {
        Ok(Some(f8(Str::lit("the members do not end (JSON Schema 2020-12 Core section 10.3.2.2)"))))
    } else {
        let v5 = m2707 - 1;
        let m2708 = v3;
        match m2708.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                let v8: Bytes = f611(v4.clone(), {
                    let (h2709, _) = v6.clone();
                    h2709
                })?;
                let m2711 = f612(f512(f275(v2.clone(), f8(Str::lit("patternProperties")))), {
                    let (h2710, _) = v6.clone();
                    h2710
                })?;
                match m2711 {
                    Ok(v10) => {
                        Ok(f568(f571(v5, v1.clone(), v10.clone(), {
                            let (_, h2712) = v6.clone();
                            h2712
                        }, v8.clone())?, f568({
                            let m2713 = v10;
                            match m2713.uncons() {
                                None => {
                                    if f52({
                                        let (h2714, _) = v6;
                                        h2714
                                    }, f607(f549({
                                        let m2715 = f275(v2.clone(), f8(Str::lit("properties")));
                                        match m2715 {
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
                                        let m2716 = f275(v2.clone(), f8(Str::lit("additionalProperties")));
                                        match m2716 {
                                            None => {
                                                None::<Bytes>
                                            }
                                            Some(v14) => {
                                                f529(v5, v1.clone(), v14, {
                                                    let (_, h2717) = v6;
                                                    h2717
                                                }, v8)?
                                            }
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    None::<Bytes>
                                }
                            }
                        }, f574(v5, v1, v2, v7, v4)?)))
                    }
                    Err(v9) => {
                        Ok(Some(v9))
                    }
                }
            }
        }
    }
}

pub fn f575(v0: u64, v1: Adt2, v2: Adt2, v3: List<Adt2>, v4: Bytes, v5: u64) -> R<Option<Bytes>> {
    let m2718 = v0;
    if m2718 == 0 {
        Ok(Some(f8(Str::lit("the items do not end (JSON Schema 2020-12 Core section 10.3.1.2)"))))
    } else {
        let v6 = m2718 - 1;
        let m2719 = v3;
        match m2719.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v7, v8)) => {
                Ok(f568(f529(v6, v1.clone(), v2.clone(), v7, f611(v4.clone(), f396(v5)?)?)?, f575(v6, v1, v2, v8, v4, {
                    let a2720 = v5;
                    let a2721 = 1u64;
                    nat_add(a2720, a2721)?
                })?))
            }
        }
    }
}

pub fn f576(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2722, _) = v2;
        h2722
    };
    let a2723 = v3;
    let a2724 = 0u64;
    if nat_eq(a2723, a2724) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2725 = v3;
            let a2726 = v1;
            let a2727 = 0u64;
            nat_quot(a2725, a2726, a2727)
        }, {
            let a2732 = f533(v0, {
                let a2728 = v3;
                let a2729 = v1;
                let a2730 = 0u64;
                nat_rem(a2728, a2729, a2730)
            });
            let a2733 = {
                let (_, h2731) = v2;
                h2731
            };
            append_bytes(a2732, a2733)
        }))
    }
}

pub fn f577(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2778 = {
            let a2754 = {
                let a2742 = {
                    let a2736 = {
                        let a2734 = v1;
                        let a2735 = 128u8;
                        fixed_u8::bit_and(a2734, a2735)
                    };
                    let a2737 = 128u8;
                    if equal(a2736, a2737) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2743 = {
                    let a2740 = {
                        let a2738 = v1;
                        let a2739 = 64u8;
                        fixed_u8::bit_and(a2738, a2739)
                    };
                    let a2741 = 64u8;
                    if equal(a2740, a2741) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2742, a2743)?
            };
            let a2755 = {
                let a2752 = {
                    let a2746 = {
                        let a2744 = v1;
                        let a2745 = 32u8;
                        fixed_u8::bit_and(a2744, a2745)
                    };
                    let a2747 = 32u8;
                    if equal(a2746, a2747) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2753 = {
                    let a2750 = {
                        let a2748 = v1;
                        let a2749 = 16u8;
                        fixed_u8::bit_and(a2748, a2749)
                    };
                    let a2751 = 16u8;
                    if equal(a2750, a2751) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2752, a2753)?
            };
            nat_add(a2754, a2755)?
        };
        let a2779 = {
            let a2776 = {
                let a2764 = {
                    let a2758 = {
                        let a2756 = v1;
                        let a2757 = 8u8;
                        fixed_u8::bit_and(a2756, a2757)
                    };
                    let a2759 = 8u8;
                    if equal(a2758, a2759) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2765 = {
                    let a2762 = {
                        let a2760 = v1;
                        let a2761 = 4u8;
                        fixed_u8::bit_and(a2760, a2761)
                    };
                    let a2763 = 4u8;
                    if equal(a2762, a2763) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2764, a2765)?
            };
            let a2777 = {
                let a2774 = {
                    let a2768 = {
                        let a2766 = v1;
                        let a2767 = 2u8;
                        fixed_u8::bit_and(a2766, a2767)
                    };
                    let a2769 = 2u8;
                    if equal(a2768, a2769) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2775 = {
                    let a2772 = {
                        let a2770 = v1;
                        let a2771 = 1u8;
                        fixed_u8::bit_and(a2770, a2771)
                    };
                    let a2773 = 1u8;
                    if equal(a2772, a2773) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2774, a2775)?
            };
            nat_add(a2776, a2777)?
        };
        nat_add(a2778, a2779)?
    };
    let a2786 = f533(f613(), {
        let a2780 = v2;
        let a2781 = 16u64;
        let a2782 = 0u64;
        nat_quot(a2780, a2781, a2782)
    });
    let a2787 = f533(f613(), {
        let a2783 = v2;
        let a2784 = 16u64;
        let a2785 = 0u64;
        nat_rem(a2783, a2784, a2785)
    });
    Ok(append_bytes(a2786, a2787))
}

pub fn f578(v0: Fn19, p2791: List<List<(Bytes, Bytes)>>, p2792: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2790 = (p2791, p2792);
    {
        let mut m2790 = m2790;
        loop {
            let step = {
                let (v1, v2) = m2790;
                let m2788 = v2;
                match m2788.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2789: Fn19 = v0.clone();
                            c2789.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2790 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f580(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2794 = {
        let a2793 = v0.clone();
        length_list(a2793)
    };
    let a2795 = 1u64;
    if nat_le(a2794, a2795) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f614(v0)?))
    }
}

pub fn f581(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2796, _) = v1;
        h2796
    };
    let m2798 = {
        let (_, h2797) = v1;
        h2797
    };
    match m2798 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2801 = {
                let a2799 = v0.clone();
                let a2800 = v2;
                index_bytes(a2799, a2800)
            };
            match m2801 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2806 = {
                        let a2804 = v0;
                        let a2805 = {
                            let a2802 = v2;
                            let a2803 = 1u64;
                            nat_add(a2802, a2803)?
                        };
                        index_bytes(a2804, a2805)
                    };
                    match m2806 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f615(v4)? {
                                f615(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2807 = v2;
                                    let a2808 = 2u64;
                                    nat_add(a2807, a2808)?
                                }, Some({
                                    let a2813 = v3;
                                    let a2814 = f533(f616(), {
                                        let a2811 = {
                                            let a2809 = f617(v4)?;
                                            let a2810 = 16u64;
                                            nat_mul(a2809, a2810)?
                                        };
                                        let a2812 = f617(v5)?;
                                        nat_add(a2811, a2812)?
                                    });
                                    append_bytes(a2813, a2814)
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

pub fn f582(v0: Fn20, p2819: u64, p2820: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2818 = (p2819, p2820);
    {
        let mut m2818 = m2818;
        loop {
            let step = {
                let (v1, v2) = m2818;
                let m2815 = v1;
                if m2815 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2815 - 1;
                    let m2817 = {
                        let c2816: Fn20 = v0.clone();
                        c2816.apply(v2.clone())?
                    };
                    match m2817 {
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
                Ok(next) => { m2818 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f618(v0)
}

pub fn f584(p2823: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m2822 = p2823;
    {
        let mut m2822 = m2822;
        loop {
            let step = {
                let v0 = m2822;
                let m2821 = v0;
                match m2821.uncons() {
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
                Ok(next) => { m2822 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f585(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2824 = v0;
    match m2824 {
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
        Adt2::C6(r2827) => {
            let v2 = (*r2827).clone();
            f585(v2, v1)
        }
        Adt2::C7(r2828) => {
            let v3 = (*r2828).clone();
            f585(v3, v1)
        }
        Adt2::C8 => {
            Ok(v1)
        }
        Adt2::C9(r2829, r2830) => {
            let v4 = (*r2829).clone();
            let v5 = (*r2830).clone();
            f585(v4, f585(v5, v1)?)
        }
        Adt2::C10(v6, r2831, r2832) => {
            let v7 = (*r2831).clone();
            let v8 = (*r2832).clone();
            let v9: List<Bytes> = f585(v7.clone(), f585(v8, v1)?)?;
            if f32(v6, f8(Str::lit("$ref"))) {
                let m2825 = f381(Some(v7));
                match m2825 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m2826 = f619(v10)?;
                        match m2826 {
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

pub fn f586() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f587(v0: Bytes) -> R<Bytes> {
    let m2835 = {
        let a2833 = v0.clone();
        let a2834 = 0u64;
        index_bytes(a2833, a2834)
    };
    match m2835 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2841 = {
                let a2836 = f8(Str::lit("u"));
                let a2837 = f620(8u64, f72(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2836, a2837)
            };
            let a2842 = f46(v0.clone(), 1u64, {
                let a2839 = {
                    let a2838 = v0;
                    length_bytes(a2838)
                };
                let a2840 = 1u64;
                nat_sub(a2839, a2840)
            });
            Ok(append_bytes(a2841, a2842))
        }
    }
}

pub fn f588(v0: Bytes, v1: Adt2) -> R<Result<Adt28, Bytes>> {
    let m2843 = f275(v1.clone(), f8(Str::lit("enum")));
    match m2843 {
        None => {
            let m2845 = f275(v1.clone(), f8(Str::lit("oneOf")));
            match m2845 {
                None => {
                    if f625(v1.clone(), v0.clone())? {
                        let m2846 = f626(v0.clone(), f512(f275(v1.clone(), f8(Str::lit("properties")))), f279(f280(f275(v1, f8(Str::lit("required"))))), true)?;
                        match m2846 {
                            Ok(v7) => {
                                Ok(f623(f627(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f391(f628(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f629(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt28, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2847 = f512(f275(v1.clone(), f8(Str::lit("properties"))));
                        match m2847.uncons() {
                            None => {
                                if f630(v1.clone()) {
                                    Ok(f623(f631(v0), List::<Adt25>::nil()))
                                } else {
                                    let m2850 = f632(v1, {
                                        let a2848 = v0.clone();
                                        let a2849 = f8(Str::lit("Value"));
                                        append_bytes(a2848, a2849)
                                    })?;
                                    match m2850 {
                                        Ok(v9) => {
                                            Ok(f623(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f557(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f556(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt28, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m2851 = f626(v0.clone(), f512(f275(v1.clone(), f8(Str::lit("properties")))), f279(f280(f275(v1, f8(Str::lit("required"))))), false)?;
                                match m2851 {
                                    Ok(v13) => {
                                        Ok(f623(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m2852 = f628(v13.clone());
                                            match m2852.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f391(f628(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f629(v13)))
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
                    f624(v0, f382(v5))
                }
            }
        }
        Some(v2) => {
            let m2844 = f621(f382(v2), List::<Bytes>::nil())?;
            match m2844 {
                Ok(v4) => {
                    Ok(f623(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f391(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f622(v3, v0))
                }
            }
        }
    }
}

pub fn f589(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m2853 = f275(v0.clone(), f8(Str::lit("anyOf")));
        match m2853 {
            None => {
                v0
            }
            Some(v1) => {
                if f633(f382(v1.clone())) {
                    f634(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f635(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f636(f382(v1.clone()))
                    } {
                        v0
                    } else {
                        f637(f634(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m2854 = f275(v2.clone(), f8(Str::lit("oneOf")));
        match m2854 {
            None => {
                v2
            }
            Some(v3) => {
                if f633(f382(v3)) {
                    f634(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m2855 = f275(v4.clone(), f8(Str::lit("allOf")));
    match m2855 {
        None => {
            v4
        }
        Some(v5) => {
            if f638(f382(v5)) {
                f634(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f590(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f639(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2856) => {
            h2856
        }
    });
    let v2: u32 = f562(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2857, _, _) => {
            h2857
        }
    }, f562(f640(match v0.clone() {
        Adt27::C0(_, _, _, _, h2858, _, _, _, _, _) => {
            h2858
        }
    }), f562(f641(match v0.clone() {
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
    }), f562(v1, {
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
    let v3: u32 = f562(f642(match v0.clone() {
        Adt27::C0(h2865, _, _, _, _, _, _, _, _, _) => {
            h2865
        }
    }), f643(match v0.clone() {
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
    Ok(Adt27::C0(f562(v2, v3), match v0.clone() {
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
    }, f562(match v0.clone() {
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
    }, f644(match v0.clone() {
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

pub fn f591(v0: u32) -> u64 {
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

pub fn f592(v0: u64) -> u32 {
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

pub fn f593(v0: u64) -> u32 {
    let (_, h2886) = {
        let (_, h2887) = f645(Fn21::F646, 32u64, (v0, (1u32, 0u32)));
        h2887
    };
    h2886
}

pub fn f594(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2892 = f647(v0, 24u32);
    let a2893 = {
        let a2890 = f647(v1, 16u32);
        let a2891 = {
            let a2888 = f647(v2, 8u32);
            let a2889 = v3;
            fixed_u32::bit_or(a2888, a2889)
        };
        fixed_u32::bit_or(a2890, a2891)
    };
    fixed_u32::bit_or(a2892, a2893)
}

pub fn f595(v0: Bytes, v1: u64, v2: u64) -> u32 {
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

pub fn f596() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("patternProperties")), List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("definitions")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("format")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))
}

pub fn f597(v0: Adt2) -> Bytes {
    let m2903 = v0;
    match m2903 {
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

pub fn f598(p2906: List<Bytes>, v1: Adt2) -> bool {
    let m2905 = p2906;
    {
        let mut m2905 = m2905;
        loop {
            let step = {
                let v0 = m2905;
                let m2904 = v0;
                match m2904.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f648(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2905 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f599(p2909: List<Adt2>, v1: Adt2) -> R<bool> {
    let m2908 = p2909;
    {
        let mut m2908 = m2908;
        loop {
            let step = {
                let v0 = m2908;
                let m2907 = v0;
                match m2907.uncons() {
                    None => {
                        Err::<List<Adt2>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f600(false, v2, v1.clone())? {
                            Err::<List<Adt2>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt2>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2908 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(v0: bool, v1: Adt2, v2: Adt2) -> R<bool> {
    let m2910 = v1;
    match m2910 {
        Adt2::C0 => {
            Ok(f32(f530(v2)?, f8(Str::lit("null"))))
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(v77) => {
            Ok(f32(f530(v2)?, f530(Adt2::C2(v77))?))
        }
        Adt2::C3(v78, v79) => {
            Ok(f32(f530(v2)?, f530(Adt2::C3(v78, v79))?))
        }
        Adt2::C4(v80, v81, v82) => {
            Ok(f32(f530(v2)?, f530(Adt2::C4(v80, v81, v82))?))
        }
        Adt2::C5(v83) => {
            let m2922 = v2;
            match m2922 {
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
        Adt2::C6(r2923) => {
            let v58 = (*r2923).clone();
            let m2920 = v2;
            match m2920 {
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
                Adt2::C6(r2921) => {
                    let v59 = (*r2921).clone();
                    f600(false, v58, v59)
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
        Adt2::C7(r2924) => {
            let v41 = (*r2924).clone();
            let m2916 = v2;
            match m2916 {
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
                Adt2::C7(r2919) => {
                    let v42 = (*r2919).clone();
                    if f118({
                        let a2917 = f605(v41.clone())?;
                        let a2918 = f605(v42.clone())?;
                        nat_eq(a2917, a2918)
                    }, true) {
                        f600(true, v41, v42)
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
                let m2912 = v2;
                match m2912 {
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
        Adt2::C9(r2925, r2926) => {
            let v23 = (*r2925).clone();
            let v24 = (*r2926).clone();
            let m2913 = v2;
            match m2913 {
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
                Adt2::C9(r2914, r2915) => {
                    let v25 = (*r2914).clone();
                    let v26 = (*r2915).clone();
                    if f600(false, v23, v25)? {
                        f600(false, v24, v26)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C10(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C10(v3, r2927, r2928) => {
            let v4 = (*r2927).clone();
            let v5 = (*r2928).clone();
            let m2911 = f275(v2.clone(), v3);
            if match m2911 {
                None => {
                    false
                }
                Some(v6) => {
                    f600(false, v4, v6)?
                }
            } {
                f600(true, v5, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f601(v0: Bytes) -> R<Option<List<u64>>> {
    f649({
        let a2930 = {
            let a2929 = v0.clone();
            length_bytes(a2929)
        };
        let a2931 = 1u64;
        nat_add(a2930, a2931)?
    }, v0, 0u64)
}

pub fn f602(v0: List<u64>) -> R<u64> {
    let m2932 = v0;
    match m2932.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a2933 = f602(v2)?;
            let a2934 = 1u64;
            nat_add(a2933, a2934)
        }
    }
}

pub fn f603(v0: Option<Adt2>) -> Option<u64> {
    let m2935 = v0;
    match m2935 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2936 = v1;
            match m2936 {
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

pub fn f604(v0: Bytes, v1: Bytes) -> R<Result<bool, Bytes>> {
    let m2937 = f650(v0)?;
    match m2937 {
        Ok(v3) => {
            let m2938 = f601(v1)?;
            match m2938 {
                None => {
                    Ok(Err::<bool, Bytes>(f8(Str::lit("a string is not well-formed UTF-8 (RFC 3629 section 4)"))))
                }
                Some(v4) => {
                    Ok(Ok::<bool, Bytes>({
                        let (h2939, _) = {
                            let (h2940, _) = v3.clone();
                            h2940
                        };
                        if h2939 {
                            f651({
                                let a2944 = {
                                    let a2942 = f652({
                                        let (_, h2941) = v3.clone();
                                        h2941
                                    })?;
                                    let a2943 = f602(v4.clone())?;
                                    nat_add(a2942, a2943)?
                                };
                                let a2945 = 2u64;
                                nat_add(a2944, a2945)?
                            }, {
                                let (_, h2946) = v3;
                                h2946
                            }, v4, {
                                let (_, h2947) = {
                                    let (h2948, _) = v3;
                                    h2948
                                };
                                h2947
                            })?
                        } else {
                            f653({
                                let (_, h2949) = v3;
                                h2949
                            }, v4, {
                                let (_, h2950) = {
                                    let (h2951, _) = v3;
                                    h2951
                                };
                                h2950
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

pub fn f605(v0: Adt2) -> R<u64> {
    let m2952 = v0;
    match m2952 {
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
        Adt2::C9(_, r2957) => {
            let v5 = (*r2957).clone();
            let a2955 = f605(v5)?;
            let a2956 = 1u64;
            nat_add(a2955, a2956)
        }
        Adt2::C10(_, _, r2958) => {
            let v3 = (*r2958).clone();
            let a2953 = f605(v3)?;
            let a2954 = 1u64;
            nat_add(a2953, a2954)
        }
    }
}

pub fn f606(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2959 = v0;
    match m2959.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52(v2.clone(), v1.clone()) {
                f606(v3, v1)
            } else {
                List::cons(v2, f606(v3, v1))
            }
        }
    }
}

pub fn f607(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m2960 = v0;
    match m2960.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2961, _) = v1;
                h2961
            }, f607(v2))
        }
    }
}

pub fn f608(v0: Adt2) -> Option<(bool, (u64, u64))> {
    let m2962 = v0;
    match m2962 {
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

pub fn f609(v0: (bool, (u64, u64)), v1: (bool, (u64, u64))) -> R<bool> {
    let v2: u64 = {
        let a2967 = {
            let (h2963, _) = {
                let (_, h2964) = v0;
                h2964
            };
            h2963
        };
        let a2968 = f654({
            let (_, h2965) = {
                let (_, h2966) = v1;
                h2966
            };
            h2965
        })?;
        nat_mul(a2967, a2968)?
    };
    let v3: u64 = {
        let a2973 = {
            let (h2969, _) = {
                let (_, h2970) = v1;
                h2970
            };
            h2969
        };
        let a2974 = f654({
            let (_, h2971) = {
                let (_, h2972) = v0;
                h2972
            };
            h2971
        })?;
        nat_mul(a2973, a2974)?
    };
    let (h2975, _) = v0;
    if h2975 {
        let (h2976, _) = v1;
        if h2976 {
            let a2977 = v3;
            let a2978 = v2;
            Ok(nat_lt(a2977, a2978))
        } else {
            Ok(true)
        }
    } else {
        let (h2979, _) = v1;
        if h2979 {
            Ok(false)
        } else {
            let a2980 = v2;
            let a2981 = v3;
            Ok(nat_lt(a2980, a2981))
        }
    }
}

pub fn f610(p3011: u64, v1: Bytes, p3012: u64, p3013: Bytes) -> R<Bytes> {
    let m3010 = (p3011, (p3012, p3013));
    {
        let mut m3010 = m3010;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3010;
                let m2982 = v0;
                if m2982 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2982 - 1;
                    let a2984 = {
                        let a2983 = v1.clone();
                        length_bytes(a2983)
                    };
                    let a2985 = v2;
                    if nat_le(a2984, a2985) {
                        Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                    } else {
                        if f118({
                            let a2986 = f116(v1.clone(), v2)?;
                            let a2987 = 126u64;
                            nat_eq(a2986, a2987)
                        }, {
                            let a2990 = f116(v1.clone(), {
                                let a2988 = v2;
                                let a2989 = 1u64;
                                nat_add(a2988, a2989)?
                            })?;
                            let a2991 = 48u64;
                            nat_eq(a2990, a2991)
                        }) {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a2992 = v2;
                                let a2993 = 2u64;
                                nat_add(a2992, a2993)?
                            }, {
                                let a2994 = v3;
                                let a2995 = f8(Str::lit("~"));
                                append_bytes(a2994, a2995)
                            })))
                        } else {
                            if f118({
                                let a2996 = f116(v1.clone(), v2)?;
                                let a2997 = 126u64;
                                nat_eq(a2996, a2997)
                            }, {
                                let a3000 = f116(v1.clone(), {
                                    let a2998 = v2;
                                    let a2999 = 1u64;
                                    nat_add(a2998, a2999)?
                                })?;
                                let a3001 = 49u64;
                                nat_eq(a3000, a3001)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a3002 = v2;
                                    let a3003 = 2u64;
                                    nat_add(a3002, a3003)?
                                }, {
                                    let a3004 = v3;
                                    let a3005 = f8(Str::lit("/"));
                                    append_bytes(a3004, a3005)
                                })))
                            } else {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a3006 = v2;
                                    let a3007 = 1u64;
                                    nat_add(a3006, a3007)?
                                }, {
                                    let a3008 = v3;
                                    let a3009 = f46(v1.clone(), v2, 1u64);
                                    append_bytes(a3008, a3009)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f611(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(f655({
        let a3015 = {
            let a3014 = v1.clone();
            length_bytes(a3014)
        };
        let a3016 = 1u64;
        nat_add(a3015, a3016)?
    }, v1, 0u64, Bytes::lit(&[]))?, List::<Bytes>::nil())))))
}

pub fn f612(v0: List<(Bytes, Adt2)>, v1: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let m3017 = v0;
    match m3017.uncons() {
        None => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let m3019 = f604({
                let (h3018, _) = v2;
                h3018
            }, v1.clone())?;
            match m3019 {
                Ok(v5) => {
                    let m3020 = f612(v3, v1)?;
                    match m3020 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt2>, Bytes>(if v5 {
                                List::cons({
                                    let (_, h3021) = v2;
                                    h3021
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

pub fn f613() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f614(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f536({
        let (_, h3022) = {
            let (h3023, _) = f656(Fn22::F657, {
                let a3025 = {
                    let a3024 = v0.clone();
                    length_list(a3024)
                };
                let a3026 = 1u64;
                nat_add(a3025, a3026)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3023
        };
        h3022
    }))
}

pub fn f615(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3071 = {
            let a3047 = {
                let a3035 = {
                    let a3029 = {
                        let a3027 = v1;
                        let a3028 = 128u8;
                        fixed_u8::bit_and(a3027, a3028)
                    };
                    let a3030 = 128u8;
                    if equal(a3029, a3030) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3036 = {
                    let a3033 = {
                        let a3031 = v1;
                        let a3032 = 64u8;
                        fixed_u8::bit_and(a3031, a3032)
                    };
                    let a3034 = 64u8;
                    if equal(a3033, a3034) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3035, a3036)?
            };
            let a3048 = {
                let a3045 = {
                    let a3039 = {
                        let a3037 = v1;
                        let a3038 = 32u8;
                        fixed_u8::bit_and(a3037, a3038)
                    };
                    let a3040 = 32u8;
                    if equal(a3039, a3040) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3046 = {
                    let a3043 = {
                        let a3041 = v1;
                        let a3042 = 16u8;
                        fixed_u8::bit_and(a3041, a3042)
                    };
                    let a3044 = 16u8;
                    if equal(a3043, a3044) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3045, a3046)?
            };
            nat_add(a3047, a3048)?
        };
        let a3072 = {
            let a3069 = {
                let a3057 = {
                    let a3051 = {
                        let a3049 = v1;
                        let a3050 = 8u8;
                        fixed_u8::bit_and(a3049, a3050)
                    };
                    let a3052 = 8u8;
                    if equal(a3051, a3052) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3058 = {
                    let a3055 = {
                        let a3053 = v1;
                        let a3054 = 4u8;
                        fixed_u8::bit_and(a3053, a3054)
                    };
                    let a3056 = 4u8;
                    if equal(a3055, a3056) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3057, a3058)?
            };
            let a3070 = {
                let a3067 = {
                    let a3061 = {
                        let a3059 = v1;
                        let a3060 = 2u8;
                        fixed_u8::bit_and(a3059, a3060)
                    };
                    let a3062 = 2u8;
                    if equal(a3061, a3062) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3068 = {
                    let a3065 = {
                        let a3063 = v1;
                        let a3064 = 1u8;
                        fixed_u8::bit_and(a3063, a3064)
                    };
                    let a3066 = 1u8;
                    if equal(a3065, a3066) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3067, a3068)?
            };
            nat_add(a3069, a3070)?
        };
        nat_add(a3071, a3072)?
    };
    let a3073 = 48u64;
    let a3074 = v2;
    if if nat_le(a3073, a3074) {
        let a3075 = v2;
        let a3076 = 57u64;
        nat_le(a3075, a3076)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3077 = 97u64;
        let a3078 = v2;
        if nat_le(a3077, a3078) {
            let a3079 = v2;
            let a3080 = 102u64;
            Ok(nat_le(a3079, a3080))
        } else {
            Ok(false)
        }
    }
}

pub fn f616() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f617(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3125 = {
            let a3101 = {
                let a3089 = {
                    let a3083 = {
                        let a3081 = v1;
                        let a3082 = 128u8;
                        fixed_u8::bit_and(a3081, a3082)
                    };
                    let a3084 = 128u8;
                    if equal(a3083, a3084) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3090 = {
                    let a3087 = {
                        let a3085 = v1;
                        let a3086 = 64u8;
                        fixed_u8::bit_and(a3085, a3086)
                    };
                    let a3088 = 64u8;
                    if equal(a3087, a3088) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3089, a3090)?
            };
            let a3102 = {
                let a3099 = {
                    let a3093 = {
                        let a3091 = v1;
                        let a3092 = 32u8;
                        fixed_u8::bit_and(a3091, a3092)
                    };
                    let a3094 = 32u8;
                    if equal(a3093, a3094) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3100 = {
                    let a3097 = {
                        let a3095 = v1;
                        let a3096 = 16u8;
                        fixed_u8::bit_and(a3095, a3096)
                    };
                    let a3098 = 16u8;
                    if equal(a3097, a3098) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3099, a3100)?
            };
            nat_add(a3101, a3102)?
        };
        let a3126 = {
            let a3123 = {
                let a3111 = {
                    let a3105 = {
                        let a3103 = v1;
                        let a3104 = 8u8;
                        fixed_u8::bit_and(a3103, a3104)
                    };
                    let a3106 = 8u8;
                    if equal(a3105, a3106) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3112 = {
                    let a3109 = {
                        let a3107 = v1;
                        let a3108 = 4u8;
                        fixed_u8::bit_and(a3107, a3108)
                    };
                    let a3110 = 4u8;
                    if equal(a3109, a3110) {
                        4u64
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
                        let a3114 = 2u8;
                        fixed_u8::bit_and(a3113, a3114)
                    };
                    let a3116 = 2u8;
                    if equal(a3115, a3116) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3122 = {
                    let a3119 = {
                        let a3117 = v1;
                        let a3118 = 1u8;
                        fixed_u8::bit_and(a3117, a3118)
                    };
                    let a3120 = 1u8;
                    if equal(a3119, a3120) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3121, a3122)?
            };
            nat_add(a3123, a3124)?
        };
        nat_add(a3125, a3126)?
    };
    let a3127 = v2;
    let a3128 = 57u64;
    if nat_le(a3127, a3128) {
        let a3129 = v2;
        let a3130 = 48u64;
        Ok(nat_sub(a3129, a3130))
    } else {
        let a3131 = v2;
        let a3132 = 87u64;
        Ok(nat_sub(a3131, a3132))
    }
}

pub fn f618(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3133, _) = v0;
        h3133
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3134) = v0;
        h3134
    };
    let m3135 = v1;
    match m3135.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3136 = v4;
            match m3136.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f658(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f619(v0: Bytes) -> R<Option<Bytes>> {
    if f85(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f552(f46(v0.clone(), 8u64, {
            let a3138 = {
                let a3137 = v0;
                length_bytes(a3137)
            };
            let a3139 = 8u64;
            nat_sub(a3138, a3139)
        }))?))
    } else {
        if f85(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f552(f46(v0.clone(), 14u64, {
                let a3141 = {
                    let a3140 = v0;
                    length_bytes(a3140)
                };
                let a3142 = 14u64;
                nat_sub(a3141, a3142)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f620(p3159: u64, p3160: u64, p3161: u64, p3162: Bytes) -> Bytes {
    let m3158 = (p3159, (p3160, (p3161, p3162)));
    {
        let mut m3158 = m3158;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3158;
                let m3143 = v0;
                if m3143 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3143 - 1;
                    if f118({
                        let a3144 = v1;
                        let a3145 = 0u64;
                        nat_eq(a3144, a3145)
                    }, {
                        let a3146 = v2;
                        let a3147 = 0u64;
                        nat_eq(a3146, a3147)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3148 = v1;
                            let a3149 = 16u64;
                            let a3150 = 0u64;
                            nat_quot(a3148, a3149, a3150)
                        }, ({
                            let a3151 = v2;
                            let a3152 = 1u64;
                            nat_sub(a3151, a3152)
                        }, {
                            let a3156 = f659({
                                let a3153 = v1;
                                let a3154 = 16u64;
                                let a3155 = 0u64;
                                nat_rem(a3153, a3154, a3155)
                            });
                            let a3157 = v3;
                            append_bytes(a3156, a3157)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f660(f661(f279(v0.clone()))?, v0, v1)
}

pub fn f622(v0: Bytes, v1: Bytes) -> Result<Adt28, Bytes> {
    Err::<Adt28, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f623(v0: Bytes, v1: List<Adt25>) -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(v0, v1))
}

pub fn f624(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt28, Bytes>> {
    let m3163 = v1.clone();
    match m3163.uncons() {
        None => {
            Ok(f622(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3164 = f662(f512(f275(v2, f8(Str::lit("properties")))), v1.clone());
            match m3164.uncons() {
                None => {
                    let m3165 = f663(f661(f664(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3165 {
                        Ok(v5) => {
                            Ok(f623(f627(v0, f628(v5.clone())), f629(v5)))
                        }
                        Err(v4) => {
                            Ok(f622(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3166 = f665(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3166 {
                        Ok(v9) => {
                            Ok(f623(f627(v0, f628(v9.clone())), f629(v9)))
                        }
                        Err(v8) => {
                            Ok(f622(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f625(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3167 = v0;
    match m3167 {
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
        Adt2::C6(r3168) => {
            let v2 = (*r3168).clone();
            f625(v2, v1)
        }
        Adt2::C7(r3169) => {
            let v3 = (*r3169).clone();
            f625(v3, v1)
        }
        Adt2::C8 => {
            Ok(false)
        }
        Adt2::C9(r3170, r3171) => {
            let v4 = (*r3170).clone();
            let v5 = (*r3171).clone();
            if f625(v4, v1.clone())? {
                Ok(true)
            } else {
                f625(v5, v1)
            }
        }
        Adt2::C10(v6, r3172, r3173) => {
            let v7 = (*r3172).clone();
            let v8 = (*r3173).clone();
            if if f32(v6, f8(Str::lit("$ref"))) {
                f666(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f625(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f625(v8, v1)
                }
            }
        }
    }
}

pub fn f626(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt29, Bytes>> {
    f667(f661(f607(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f627(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f391(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f628(v0: Adt29) -> List<Bytes> {
    let m3174 = v0;
    match m3174 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f629(v0: Adt29) -> List<Adt25> {
    let m3175 = v0;
    match m3175 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f630(v0: Adt2) -> bool {
    let m3176 = f512(f275(v0.clone(), f8(Str::lit("properties"))));
    match m3176.uncons() {
        None => {
            let m3177 = f668(v0);
            match m3177 {
                None => {
                    false
                }
                Some(v3) => {
                    f669(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f631(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f632(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3178 = f670(v0.clone());
    match m3178 {
        None => {
            if f32(f278(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3179 = f275(v0, f8(Str::lit("items")));
                match m3179 {
                    None => {
                        Ok(f671(f8(Str::lit("List")), f673()))
                    }
                    Some(v3) => {
                        Ok(f671(f8(Str::lit("List")), f672(v3, {
                            let a3180 = v1;
                            let a3181 = f8(Str::lit("Item"));
                            append_bytes(a3180, a3181)
                        })?))
                    }
                }
            } else {
                f672(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f671(f8(Str::lit("Option")), f672(v2, v1)?))
        }
    }
}

pub fn f633(p3184: List<Adt2>) -> bool {
    let m3183 = p3184;
    {
        let mut m3183 = m3183;
        loop {
            let step = {
                let v0 = m3183;
                let m3182 = v0;
                match m3182.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f674(f549(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3183 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f634(v0: Adt2, v1: Bytes) -> Adt2 {
    let m3185 = v0;
    match m3185 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v6, r3186) => {
            let v7 = (*r3186).clone();
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
        Adt2::C6(r3187) => {
            let v15 = (*r3187).clone();
            Adt2::C6(std::rc::Rc::new(v15))
        }
        Adt2::C7(r3188) => {
            let v5 = (*r3188).clone();
            Adt2::C7(std::rc::Rc::new(f634(v5, v1)))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r3189, r3190) => {
            let v16 = (*r3189).clone();
            let v17 = (*r3190).clone();
            Adt2::C9(std::rc::Rc::new(v16), std::rc::Rc::new(v17))
        }
        Adt2::C10(v2, r3191, r3192) => {
            let v3 = (*r3191).clone();
            let v4 = (*r3192).clone();
            if f32(v2.clone(), v1.clone()) {
                f634(v4, v1)
            } else {
                Adt2::C10(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f634(v4, v1)))
            }
        }
    }
}

pub fn f635(v0: Adt2, v1: Bytes) -> bool {
    let m3193 = f275(v0, v1);
    match m3193 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f636(v0: List<Adt2>) -> bool {
    let m3194 = v0.clone();
    match m3194.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3195 = f662(f512(f275(v1, f8(Str::lit("properties")))), v0.clone());
            match m3195.uncons() {
                None => {
                    f676(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f675(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f637(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m3196 = v0;
    match m3196 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v7, r3197) => {
            let v8 = (*r3197).clone();
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
        Adt2::C6(r3198) => {
            let v16 = (*r3198).clone();
            Adt2::C6(std::rc::Rc::new(v16))
        }
        Adt2::C7(r3199) => {
            let v3 = (*r3199).clone();
            Adt2::C7(std::rc::Rc::new(Adt2::C10(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C8 => {
            Adt2::C8
        }
        Adt2::C9(r3200, r3201) => {
            let v17 = (*r3200).clone();
            let v18 = (*r3201).clone();
            Adt2::C9(std::rc::Rc::new(v17), std::rc::Rc::new(v18))
        }
        Adt2::C10(v4, r3202, r3203) => {
            let v5 = (*r3202).clone();
            let v6 = (*r3203).clone();
            Adt2::C10(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f638(p3206: List<Adt2>) -> bool {
    let m3205 = p3206;
    {
        let mut m3205 = m3205;
        loop {
            let step = {
                let v0 = m3205;
                let m3204 = v0;
                match m3204.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f677(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3205 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f639(v0: u64) -> u32 {
    let a3207 = v0;
    let a3208 = 32u64;
    if nat_lt(a3207, a3208) {
        let a3209 = v0;
        let a3210 = 16u64;
        if nat_lt(a3209, a3210) {
            let a3211 = v0;
            let a3212 = 8u64;
            if nat_lt(a3211, a3212) {
                let a3213 = v0;
                let a3214 = 4u64;
                if nat_lt(a3213, a3214) {
                    let a3215 = v0;
                    let a3216 = 2u64;
                    if nat_lt(a3215, a3216) {
                        let a3217 = v0;
                        let a3218 = 1u64;
                        if nat_lt(a3217, a3218) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3219 = v0;
                        let a3220 = 3u64;
                        if nat_lt(a3219, a3220) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3221 = v0;
                    let a3222 = 6u64;
                    if nat_lt(a3221, a3222) {
                        let a3223 = v0;
                        let a3224 = 5u64;
                        if nat_lt(a3223, a3224) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3225 = v0;
                        let a3226 = 7u64;
                        if nat_lt(a3225, a3226) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3227 = v0;
                let a3228 = 12u64;
                if nat_lt(a3227, a3228) {
                    let a3229 = v0;
                    let a3230 = 10u64;
                    if nat_lt(a3229, a3230) {
                        let a3231 = v0;
                        let a3232 = 9u64;
                        if nat_lt(a3231, a3232) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3233 = v0;
                        let a3234 = 11u64;
                        if nat_lt(a3233, a3234) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3235 = v0;
                    let a3236 = 14u64;
                    if nat_lt(a3235, a3236) {
                        let a3237 = v0;
                        let a3238 = 13u64;
                        if nat_lt(a3237, a3238) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3239 = v0;
                        let a3240 = 15u64;
                        if nat_lt(a3239, a3240) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3241 = v0;
            let a3242 = 24u64;
            if nat_lt(a3241, a3242) {
                let a3243 = v0;
                let a3244 = 20u64;
                if nat_lt(a3243, a3244) {
                    let a3245 = v0;
                    let a3246 = 18u64;
                    if nat_lt(a3245, a3246) {
                        let a3247 = v0;
                        let a3248 = 17u64;
                        if nat_lt(a3247, a3248) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3249 = v0;
                        let a3250 = 19u64;
                        if nat_lt(a3249, a3250) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3251 = v0;
                    let a3252 = 22u64;
                    if nat_lt(a3251, a3252) {
                        let a3253 = v0;
                        let a3254 = 21u64;
                        if nat_lt(a3253, a3254) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3255 = v0;
                        let a3256 = 23u64;
                        if nat_lt(a3255, a3256) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3257 = v0;
                let a3258 = 28u64;
                if nat_lt(a3257, a3258) {
                    let a3259 = v0;
                    let a3260 = 26u64;
                    if nat_lt(a3259, a3260) {
                        let a3261 = v0;
                        let a3262 = 25u64;
                        if nat_lt(a3261, a3262) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3263 = v0;
                        let a3264 = 27u64;
                        if nat_lt(a3263, a3264) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3265 = v0;
                    let a3266 = 30u64;
                    if nat_lt(a3265, a3266) {
                        let a3267 = v0;
                        let a3268 = 29u64;
                        if nat_lt(a3267, a3268) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3269 = v0;
                        let a3270 = 31u64;
                        if nat_lt(a3269, a3270) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3271 = v0;
        let a3272 = 48u64;
        if nat_lt(a3271, a3272) {
            let a3273 = v0;
            let a3274 = 40u64;
            if nat_lt(a3273, a3274) {
                let a3275 = v0;
                let a3276 = 36u64;
                if nat_lt(a3275, a3276) {
                    let a3277 = v0;
                    let a3278 = 34u64;
                    if nat_lt(a3277, a3278) {
                        let a3279 = v0;
                        let a3280 = 33u64;
                        if nat_lt(a3279, a3280) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3281 = v0;
                        let a3282 = 35u64;
                        if nat_lt(a3281, a3282) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3283 = v0;
                    let a3284 = 38u64;
                    if nat_lt(a3283, a3284) {
                        let a3285 = v0;
                        let a3286 = 37u64;
                        if nat_lt(a3285, a3286) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3287 = v0;
                        let a3288 = 39u64;
                        if nat_lt(a3287, a3288) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3289 = v0;
                let a3290 = 44u64;
                if nat_lt(a3289, a3290) {
                    let a3291 = v0;
                    let a3292 = 42u64;
                    if nat_lt(a3291, a3292) {
                        let a3293 = v0;
                        let a3294 = 41u64;
                        if nat_lt(a3293, a3294) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3295 = v0;
                        let a3296 = 43u64;
                        if nat_lt(a3295, a3296) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3297 = v0;
                    let a3298 = 46u64;
                    if nat_lt(a3297, a3298) {
                        let a3299 = v0;
                        let a3300 = 45u64;
                        if nat_lt(a3299, a3300) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3301 = v0;
                        let a3302 = 47u64;
                        if nat_lt(a3301, a3302) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3303 = v0;
            let a3304 = 56u64;
            if nat_lt(a3303, a3304) {
                let a3305 = v0;
                let a3306 = 52u64;
                if nat_lt(a3305, a3306) {
                    let a3307 = v0;
                    let a3308 = 50u64;
                    if nat_lt(a3307, a3308) {
                        let a3309 = v0;
                        let a3310 = 49u64;
                        if nat_lt(a3309, a3310) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3311 = v0;
                        let a3312 = 51u64;
                        if nat_lt(a3311, a3312) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3313 = v0;
                    let a3314 = 54u64;
                    if nat_lt(a3313, a3314) {
                        let a3315 = v0;
                        let a3316 = 53u64;
                        if nat_lt(a3315, a3316) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3317 = v0;
                        let a3318 = 55u64;
                        if nat_lt(a3317, a3318) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3319 = v0;
                let a3320 = 60u64;
                if nat_lt(a3319, a3320) {
                    let a3321 = v0;
                    let a3322 = 58u64;
                    if nat_lt(a3321, a3322) {
                        let a3323 = v0;
                        let a3324 = 57u64;
                        if nat_lt(a3323, a3324) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3325 = v0;
                        let a3326 = 59u64;
                        if nat_lt(a3325, a3326) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3327 = v0;
                    let a3328 = 62u64;
                    if nat_lt(a3327, a3328) {
                        let a3329 = v0;
                        let a3330 = 61u64;
                        if nat_lt(a3329, a3330) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3331 = v0;
                        let a3332 = 63u64;
                        if nat_lt(a3331, a3332) {
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

pub fn f640(v0: u32) -> u32 {
    let a3335 = {
        let a3333 = f678(v0, 6u32, 26u32);
        let a3334 = f678(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3333, a3334)
    };
    let a3336 = f678(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3335, a3336)
}

pub fn f641(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3342 = {
        let a3337 = v0;
        let a3338 = v1;
        fixed_u32::bit_and(a3337, a3338)
    };
    let a3343 = {
        let a3340 = {
            let a3339 = v0;
            fixed_u32::bit_not(a3339)
        };
        let a3341 = v2;
        fixed_u32::bit_and(a3340, a3341)
    };
    fixed_u32::bit_xor(a3342, a3343)
}

pub fn f642(v0: u32) -> u32 {
    let a3346 = {
        let a3344 = f678(v0, 2u32, 30u32);
        let a3345 = f678(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3344, a3345)
    };
    let a3347 = f678(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3346, a3347)
}

pub fn f643(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3356 = {
        let a3352 = {
            let a3348 = v0;
            let a3349 = v1;
            fixed_u32::bit_and(a3348, a3349)
        };
        let a3353 = {
            let a3350 = v0;
            let a3351 = v2;
            fixed_u32::bit_and(a3350, a3351)
        };
        fixed_u32::bit_xor(a3352, a3353)
    };
    let a3357 = {
        let a3354 = v1;
        let a3355 = v2;
        fixed_u32::bit_and(a3354, a3355)
    };
    fixed_u32::bit_xor(a3356, a3357)
}

pub fn f644(v0: Adt26) -> Adt26 {
    Adt26::C0(match v0.clone() {
        Adt26::C0(_, h3358, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3358
        }
    }, match v0.clone() {
        Adt26::C0(_, _, h3359, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3359
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, h3360, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3360
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, h3361, _, _, _, _, _, _, _, _, _, _, _) => {
            h3361
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, h3362, _, _, _, _, _, _, _, _, _, _) => {
            h3362
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, h3363, _, _, _, _, _, _, _, _, _) => {
            h3363
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, h3364, _, _, _, _, _, _, _, _) => {
            h3364
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, h3365, _, _, _, _, _, _, _) => {
            h3365
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3366, _, _, _, _, _, _) => {
            h3366
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, h3367, _, _, _, _, _) => {
            h3367
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, h3368, _, _, _, _) => {
            h3368
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3369, _, _, _) => {
            h3369
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3370, _, _) => {
            h3370
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3371, _) => {
            h3371
        }
    }, match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3372) => {
            h3372
        }
    }, f562(f679(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3373, _) => {
            h3373
        }
    }), f562(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3374, _, _, _, _, _, _) => {
            h3374
        }
    }, f562(f680(match v0.clone() {
        Adt26::C0(_, h3375, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3375
        }
    }), match v0 {
        Adt26::C0(h3376, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3376
        }
    }))))
}

pub fn f645(v0: Fn21, p3380: u64, p3381: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3379 = (p3380, p3381);
    {
        let mut m3379 = m3379;
        loop {
            let step = {
                let (v1, v2) = m3379;
                let m3377 = v1;
                if m3377 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3377 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3378: Fn21 = v0.clone();
                        c3378.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3379 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f646(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3382, _) = v0;
        h3382
    };
    let v2: u32 = {
        let (h3383, _) = {
            let (_, h3384) = v0;
            h3384
        };
        h3383
    };
    let v3: u32 = {
        let (_, h3385) = {
            let (_, h3386) = v0;
            h3386
        };
        h3385
    };
    ({
        let a3387 = v1;
        let a3388 = 2u64;
        let a3389 = 0u64;
        nat_quot(a3387, a3388, a3389)
    }, (f647(v2, 1u32), {
        let a3393 = {
            let a3390 = v1;
            let a3391 = 2u64;
            let a3392 = 0u64;
            nat_rem(a3390, a3391, a3392)
        };
        let a3394 = 1u64;
        if nat_eq(a3393, a3394) {
            let a3395 = v3;
            let a3396 = v2;
            fixed_u32::bit_or(a3395, a3396)
        } else {
            v3
        }
    }))
}

pub fn f647(v0: u32, v1: u32) -> u32 {
    let m3399 = {
        let a3397 = v0;
        let a3398 = v1;
        fixed_u32::shift_left(a3397, a3398)
    };
    match m3399 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f648(v0: Bytes, v1: Adt2) -> bool {
    let v2: Bytes = f597(v1);
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

pub fn f649(v0: u64, v1: Bytes, v2: u64) -> R<Option<List<u64>>> {
    let m3400 = v0;
    if m3400 == 0 {
        Ok(Some(List::<u64>::nil()))
    } else {
        let v3 = m3400 - 1;
        let a3402 = {
            let a3401 = v1.clone();
            length_bytes(a3401)
        };
        let a3403 = v2;
        if nat_le(a3402, a3403) {
            Ok(Some(List::<u64>::nil()))
        } else {
            let m3404 = f681(v1.clone(), v2)?;
            match m3404 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v4) => {
                    let m3408 = f649(v3, v1, {
                        let a3406 = v2;
                        let a3407 = {
                            let (_, h3405) = v4;
                            h3405
                        };
                        nat_add(a3406, a3407)?
                    })?;
                    match m3408 {
                        None => {
                            Ok(None::<List<u64>>)
                        }
                        Some(v5) => {
                            Ok(Some(List::cons({
                                let (h3409, _) = v4;
                                h3409
                            }, v5)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f650(v0: Bytes) -> R<Result<((bool, bool), List<Adt30>), Bytes>> {
    let v1: u64 = {
        let a3410 = v0.clone();
        length_bytes(a3410)
    };
    let v2: bool = {
        let a3411 = 0u64;
        let a3412 = v1;
        if nat_lt(a3411, a3412) {
            let a3413 = f116(v0.clone(), 0u64)?;
            let a3414 = 94u64;
            nat_eq(a3413, a3414)
        } else {
            false
        }
    };
    let v3: bool = {
        let a3415 = 0u64;
        let a3416 = v1;
        if nat_lt(a3415, a3416) {
            f118({
                let a3419 = f116(v0.clone(), {
                    let a3417 = v1;
                    let a3418 = 1u64;
                    nat_sub(a3417, a3418)
                })?;
                let a3420 = 36u64;
                nat_eq(a3419, a3420)
            }, f682({
                let a3423 = f116(v0.clone(), {
                    let a3421 = v1;
                    let a3422 = 2u64;
                    nat_sub(a3421, a3422)
                })?;
                let a3424 = 92u64;
                nat_eq(a3423, a3424)
            }))
        } else {
            false
        }
    };
    let m3429 = f683({
        let a3425 = v1;
        let a3426 = 1u64;
        nat_add(a3425, a3426)?
    }, v0, if v2 {
        1u64
    } else {
        0u64
    }, if v3 {
        let a3427 = v1;
        let a3428 = 1u64;
        nat_sub(a3427, a3428)
    } else {
        v1
    })?;
    match m3429 {
        Ok(v5) => {
            Ok(Ok::<((bool, bool), List<Adt30>), Bytes>(((v2, v3), v5)))
        }
        Err(v4) => {
            Ok(Err::<((bool, bool), List<Adt30>), Bytes>(v4))
        }
    }
}

pub fn f651(v0: u64, v1: List<Adt30>, v2: List<u64>, v3: bool) -> R<bool> {
    let m3430 = v0;
    if m3430 == 0 {
        Ok(false)
    } else {
        let v4 = m3430 - 1;
        let m3431 = v1.clone();
        match m3431.uncons() {
            None => {
                if v3 {
                    let a3432 = f602(v2)?;
                    let a3433 = 0u64;
                    Ok(nat_eq(a3432, a3433))
                } else {
                    Ok(true)
                }
            }
            Some((v5, v6)) => {
                let m3434 = v5.clone();
                match m3434 {
                    Adt30::C0(v7, v8, v9) => {
                        let a3435 = v9;
                        let a3436 = 1u64;
                        if nat_eq(a3435, a3436) {
                            if f651(v4, v6, v2.clone(), v3)? {
                                Ok(true)
                            } else {
                                let m3437 = v2;
                                match m3437.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v10, v11)) => {
                                        if f684(v5, v10) {
                                            f651(v4, v1, v11, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a3438 = v9;
                            let a3439 = 2u64;
                            if nat_eq(a3438, a3439) {
                                let m3440 = v2;
                                match m3440.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v12, v13)) => {
                                        if f684(v5, v12) {
                                            f651(v4, List::cons(Adt30::C0(v7, v8, 1u64), v6), v13, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            } else {
                                let a3441 = v9;
                                let a3442 = 3u64;
                                if nat_eq(a3441, a3442) {
                                    if f651(v4, v6.clone(), v2.clone(), v3)? {
                                        Ok(true)
                                    } else {
                                        let m3443 = v2;
                                        match m3443.uncons() {
                                            None => {
                                                Ok(false)
                                            }
                                            Some((v14, v15)) => {
                                                if f684(v5, v14) {
                                                    f651(v4, v6, v15, v3)
                                                } else {
                                                    Ok(false)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let m3444 = v2;
                                    match m3444.uncons() {
                                        None => {
                                            Ok(false)
                                        }
                                        Some((v16, v17)) => {
                                            if f684(v5, v16) {
                                                f651(v4, v6, v17, v3)
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

pub fn f652(v0: List<Adt30>) -> R<u64> {
    let m3445 = v0;
    match m3445.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a3446 = f652(v2)?;
            let a3447 = 1u64;
            nat_add(a3446, a3447)
        }
    }
}

pub fn f653(v0: List<Adt30>, v1: List<u64>, v2: bool) -> R<bool> {
    if f651({
        let a3450 = {
            let a3448 = f652(v0.clone())?;
            let a3449 = f602(v1.clone())?;
            nat_add(a3448, a3449)?
        };
        let a3451 = 2u64;
        nat_add(a3450, a3451)?
    }, v0.clone(), v1.clone(), v2)? {
        Ok(true)
    } else {
        f685(v0, v1, v2)
    }
}

pub fn f654(v0: u64) -> R<u64> {
    let m3452 = v0;
    if m3452 == 0 {
        Ok(1u64)
    } else {
        let v1 = m3452 - 1;
        let a3453 = f654(v1)?;
        let a3454 = 10u64;
        nat_mul(a3453, a3454)
    }
}

pub fn f655(p3468: u64, v1: Bytes, p3469: u64, p3470: Bytes) -> R<Bytes> {
    let m3467 = (p3468, (p3469, p3470));
    {
        let mut m3467 = m3467;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3467;
                let m3455 = v0;
                if m3455 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3455 - 1;
                    let m3458 = {
                        let a3456 = v1.clone();
                        let a3457 = v2;
                        index_bytes(a3456, a3457)
                    };
                    match m3458 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a3459 = v2;
                                let a3460 = 1u64;
                                nat_add(a3459, a3460)?
                            }, {
                                let a3465 = v3;
                                let a3466 = {
                                    let a3461 = v6;
                                    let a3462 = 126u64;
                                    if nat_eq(a3461, a3462) {
                                        f8(Str::lit("~0"))
                                    } else {
                                        let a3463 = v6;
                                        let a3464 = 47u64;
                                        if nat_eq(a3463, a3464) {
                                            f8(Str::lit("~1"))
                                        } else {
                                            f46(v1.clone(), v2, 1u64)
                                        }
                                    }
                                };
                                append_bytes(a3465, a3466)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f656(v0: Fn22, p3475: u64, p3476: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3474 = (p3475, p3476);
    {
        let mut m3474 = m3474;
        loop {
            let step = {
                let (v1, v2) = m3474;
                let m3471 = v1;
                if m3471 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3471 - 1;
                    let m3473 = {
                        let c3472: Fn22 = v0.clone();
                        c3472.apply(v2.clone())?
                    };
                    match m3473 {
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
                Ok(next) => { m3474 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f657(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f686(v0)
}

pub fn f658(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3481 = {
            let a3479 = {
                let a3477 = v0.clone();
                length_list(a3477)
            };
            let a3480 = {
                let a3478 = v1.clone();
                length_list(a3478)
            };
            nat_add(a3479, a3480)?
        };
        let a3482 = 1u64;
        nat_add(a3481, a3482)?
    };
    Ok(f441({
        let (_, h3483) = {
            let (_, h3484) = {
                let (h3485, _) = f687(Fn23::F688, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3485
            };
            h3484
        };
        h3483
    }))
}

pub fn f659(v0: u64) -> Bytes {
    f46({
        let a3486 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3486)
    }, v0, 1u64)
}

pub fn f660(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3487 = v1;
    match m3487.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3488 = f689(v0, v3.clone())?;
            match m3488 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f530(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f52(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3489 = f660(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3489 {
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

pub fn f661(p3493: List<Bytes>) -> R<bool> {
    let m3492 = p3493;
    {
        let mut m3492 = m3492;
        loop {
            let step = {
                let v0 = m3492;
                let m3490 = v0;
                match m3490.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3491 = f690(v1)?;
                        match m3491 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f52(v3, f691(false, v2.clone())?) {
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
                Ok(next) => { m3492 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f662(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m3494 = v0;
    match m3494.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f692(v1.clone(), {
                let (h3495, _) = v2.clone();
                h3495
            }) {
                List::cons({
                    let (h3496, _) = v2;
                    h3496
                }, f662(v3, v1))
            } else {
                f662(v3, v1)
            }
        }
    }
}

pub fn f663(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3497 = v2;
    match m3497.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3498 = f693(v4.clone())?;
            match m3498 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3499 = f694(v0, v6.clone())?;
                    match m3499 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f52(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3502 = f695(v4, {
                                    let a3500 = v1.clone();
                                    let a3501 = f375(v6)?;
                                    append_bytes(a3500, a3501)
                                })?;
                                match m3502 {
                                    Ok(v9) => {
                                        let m3503 = f663(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3503 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f557(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f628(v11.clone())), f516(f556(v9), f629(v11)))))
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

pub fn f664(v0: List<Adt2>) -> R<List<Bytes>> {
    let m3504 = v0;
    match m3504.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3505 = f693(v1)?;
            match m3505 {
                None => {
                    f664(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f664(v2)?))
                }
            }
        }
    }
}

pub fn f665(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    f696(f661(f697(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f666(v0: Adt2, v1: Bytes) -> R<bool> {
    let m3506 = f381(Some(v0));
    match m3506 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3507 = f619(v2)?;
            match m3507 {
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

pub fn f667(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt29, Bytes>> {
    let m3508 = v2;
    match m3508.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3510 = f698(v0, {
                let (h3509, _) = v5.clone();
                h3509
            })?;
            match m3510 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3511, _) = v5;
                        h3511
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f52(v7.clone(), f699(v0, f607(v6.clone()))?) {
                        Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3516 = f632({
                            let (_, h3512) = v5.clone();
                            h3512
                        }, {
                            let a3514 = v1.clone();
                            let a3515 = f375({
                                let (h3513, _) = v5.clone();
                                h3513
                            })?;
                            append_bytes(a3514, a3515)
                        })?;
                        match m3516 {
                            Ok(v9) => {
                                let v10: Bytes = if f52({
                                    let (h3517, _) = v5;
                                    h3517
                                }, v3.clone()) {
                                    f557(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f557(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3518 = f667(v0, v1, v6, v3, v4)?;
                                match m3518 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f628(v12.clone())), f516(f556(v9), f629(v12)))))
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

pub fn f668(v0: Adt2) -> Option<Adt2> {
    let m3519 = f275(v0.clone(), f8(Str::lit("additionalProperties")));
    match m3519 {
        None => {
            f700(v0)
        }
        Some(v1) => {
            if f669(v1.clone()) {
                let m3520 = f700(v0);
                match m3520 {
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

pub fn f669(v0: Adt2) -> bool {
    let m3521 = v0;
    match m3521 {
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

pub fn f670(v0: Adt2) -> Option<Adt2> {
    let m3522 = f280(f275(v0, f8(Str::lit("oneOf"))));
    match m3522.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3523 = v2;
            match m3523.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m3524 = v4;
                    match m3524.uncons() {
                        None => {
                            if f701(v3.clone()) {
                                Some(v1)
                            } else {
                                if f701(v1) {
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

pub fn f671(v0: Bytes, v1: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3525 = v1;
    match m3525 {
        Ok(v3) => {
            f623(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f557(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f556(v3))
        }
        Err(v2) => {
            Err::<Adt28, Bytes>(v2)
        }
    }
}

pub fn f672(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3526 = f670(v0.clone());
    match m3526 {
        None => {
            if f32(f278(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3527 = f275(v0, f8(Str::lit("items")));
                match m3527 {
                    None => {
                        Ok(f671(f8(Str::lit("List")), f673()))
                    }
                    Some(v3) => {
                        Ok(f671(f8(Str::lit("List")), f695(v3, {
                            let a3528 = v1;
                            let a3529 = f8(Str::lit("Item"));
                            append_bytes(a3528, a3529)
                        })?))
                    }
                }
            } else {
                f695(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f671(f8(Str::lit("Option")), f695(v2, v1)?))
        }
    }
}

pub fn f673() -> Result<Adt28, Bytes> {
    Ok::<Adt28, Bytes>(Adt28::C0(f553(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f674(p3533: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m3532 = p3533;
    {
        let mut m3532 = m3532;
        loop {
            let step = {
                let v0 = m3532;
                let m3530 = v0;
                match m3530.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f52({
                            let (h3531, _) = v2;
                            h3531
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3532 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f675(p3537: List<Adt2>, v1: Bytes, p3538: List<Bytes>) -> bool {
    let m3536 = (p3537, p3538);
    {
        let mut m3536 = m3536;
        loop {
            let step = {
                let (v0, v2) = m3536;
                let m3534 = v0;
                match m3534.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3535 = f702(v3, v1.clone());
                        match m3535 {
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
                Ok(next) => { m3536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f676(p3541: List<Adt2>, p3542: List<Bytes>) -> bool {
    let m3540 = (p3541, p3542);
    {
        let mut m3540 = m3540;
        loop {
            let step = {
                let (v0, v1) = m3540;
                let m3539 = v0;
                match m3539.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f278(v2, f8(Str::lit("type")));
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
                Ok(next) => { m3540 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f677(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m3543 = f275(v0.clone(), f8(Str::lit("anyOf")));
        match m3543 {
            None => {
                Adt2::C8
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f674(f549(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f633(f382(v2))
    } else {
        false
    }
}

pub fn f678(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3544 = f524(v0, v1);
    let a3545 = f647(v0, v2);
    fixed_u32::bit_or(a3544, a3545)
}

pub fn f679(v0: u32) -> u32 {
    let a3548 = {
        let a3546 = f678(v0, 17u32, 15u32);
        let a3547 = f678(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3546, a3547)
    };
    let a3549 = f524(v0, 10u32);
    fixed_u32::bit_xor(a3548, a3549)
}

pub fn f680(v0: u32) -> u32 {
    let a3552 = {
        let a3550 = f678(v0, 7u32, 25u32);
        let a3551 = f678(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3550, a3551)
    };
    let a3553 = f524(v0, 3u32);
    fixed_u32::bit_xor(a3552, a3553)
}

pub fn f681(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m3556 = {
        let a3554 = v0.clone();
        let a3555 = v1;
        index_bytes(a3554, a3555)
    };
    match m3556 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            let a3557 = v3;
            let a3558 = 128u64;
            if nat_lt(a3557, a3558) {
                Ok(Some((v3, 1u64)))
            } else {
                if f118({
                    let a3559 = 194u64;
                    let a3560 = v3;
                    nat_le(a3559, a3560)
                }, {
                    let a3561 = v3;
                    let a3562 = 223u64;
                    nat_le(a3561, a3562)
                }) {
                    let m3565 = f703(v0, {
                        let a3563 = v1;
                        let a3564 = 1u64;
                        nat_add(a3563, a3564)?
                    })?;
                    match m3565 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f704({
                                let a3570 = {
                                    let a3568 = {
                                        let a3566 = v3;
                                        let a3567 = 192u64;
                                        nat_sub(a3566, a3567)
                                    };
                                    let a3569 = 64u64;
                                    nat_mul(a3568, a3569)?
                                };
                                let a3571 = v4;
                                nat_add(a3570, a3571)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f118({
                        let a3572 = 224u64;
                        let a3573 = v3;
                        nat_le(a3572, a3573)
                    }, {
                        let a3574 = v3;
                        let a3575 = 239u64;
                        nat_le(a3574, a3575)
                    }) {
                        let m3578 = f703(v0.clone(), {
                            let a3576 = v1;
                            let a3577 = 1u64;
                            nat_add(a3576, a3577)?
                        })?;
                        match m3578 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m3581 = f703(v0, {
                                    let a3579 = v1;
                                    let a3580 = 2u64;
                                    nat_add(a3579, a3580)?
                                })?;
                                match m3581 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f704({
                                            let a3590 = {
                                                let a3584 = {
                                                    let a3582 = v3;
                                                    let a3583 = 224u64;
                                                    nat_sub(a3582, a3583)
                                                };
                                                let a3585 = 4096u64;
                                                nat_mul(a3584, a3585)?
                                            };
                                            let a3591 = {
                                                let a3588 = {
                                                    let a3586 = v5;
                                                    let a3587 = 64u64;
                                                    nat_mul(a3586, a3587)?
                                                };
                                                let a3589 = v6;
                                                nat_add(a3588, a3589)?
                                            };
                                            nat_add(a3590, a3591)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f118({
                            let a3592 = 240u64;
                            let a3593 = v3;
                            nat_le(a3592, a3593)
                        }, {
                            let a3594 = v3;
                            let a3595 = 244u64;
                            nat_le(a3594, a3595)
                        }) {
                            let m3598 = f703(v0.clone(), {
                                let a3596 = v1;
                                let a3597 = 1u64;
                                nat_add(a3596, a3597)?
                            })?;
                            match m3598 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m3601 = f703(v0.clone(), {
                                        let a3599 = v1;
                                        let a3600 = 2u64;
                                        nat_add(a3599, a3600)?
                                    })?;
                                    match m3601 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m3604 = f703(v0, {
                                                let a3602 = v1;
                                                let a3603 = 3u64;
                                                nat_add(a3602, a3603)?
                                            })?;
                                            match m3604 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f704({
                                                        let a3617 = {
                                                            let a3607 = {
                                                                let a3605 = v3;
                                                                let a3606 = 240u64;
                                                                nat_sub(a3605, a3606)
                                                            };
                                                            let a3608 = 262144u64;
                                                            nat_mul(a3607, a3608)?
                                                        };
                                                        let a3618 = {
                                                            let a3615 = {
                                                                let a3609 = v7;
                                                                let a3610 = 4096u64;
                                                                nat_mul(a3609, a3610)?
                                                            };
                                                            let a3616 = {
                                                                let a3613 = {
                                                                    let a3611 = v8;
                                                                    let a3612 = 64u64;
                                                                    nat_mul(a3611, a3612)?
                                                                };
                                                                let a3614 = v9;
                                                                nat_add(a3613, a3614)?
                                                            };
                                                            nat_add(a3615, a3616)?
                                                        };
                                                        nat_add(a3617, a3618)?
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

pub fn f682(v0: bool) -> bool {
    !v0
}

pub fn f683(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Result<List<Adt30>, Bytes>> {
    let m3619 = v0;
    if m3619 == 0 {
        Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
    } else {
        let v4 = m3619 - 1;
        let a3620 = v3;
        let a3621 = v2;
        if nat_le(a3620, a3621) {
            Ok(Ok::<List<Adt30>, Bytes>(List::<Adt30>::nil()))
        } else {
            let v5: u64 = f116(v1.clone(), v2)?;
            let m3670 = {
                let a3622 = v5;
                let a3623 = 91u64;
                if nat_eq(a3622, a3623) {
                    let a3626 = f116(v1.clone(), {
                        let a3624 = v2;
                        let a3625 = 1u64;
                        nat_add(a3624, a3625)?
                    })?;
                    let a3627 = 94u64;
                    if nat_eq(a3626, a3627) {
                        let m3631 = f705({
                            let a3628 = v1.clone();
                            length_bytes(a3628)
                        }, v1.clone(), {
                            let a3629 = v2;
                            let a3630 = 2u64;
                            nat_add(a3629, a3630)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3631 {
                            Ok(v7) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, {
                                    let (h3632, _) = v7;
                                    h3632
                                }), {
                                    let (_, h3633) = v7;
                                    h3633
                                }))
                            }
                            Err(v6) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v6)
                            }
                        }
                    } else {
                        let m3637 = f705({
                            let a3634 = v1.clone();
                            length_bytes(a3634)
                        }, v1.clone(), {
                            let a3635 = v2;
                            let a3636 = 1u64;
                            nat_add(a3635, a3636)?
                        }, List::<(u64, u64)>::nil())?;
                        match m3637 {
                            Ok(v9) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, {
                                    let (h3638, _) = v9;
                                    h3638
                                }), {
                                    let (_, h3639) = v9;
                                    h3639
                                }))
                            }
                            Err(v8) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v8)
                            }
                        }
                    }
                } else {
                    let a3640 = v5;
                    let a3641 = 46u64;
                    if nat_eq(a3640, a3641) {
                        Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, List::cons((10u64, 10u64), List::cons((13u64, 13u64), List::cons((8232u64, 8233u64), List::<(u64, u64)>::nil())))), {
                            let a3642 = v2;
                            let a3643 = 1u64;
                            nat_add(a3642, a3643)?
                        }))
                    } else {
                        let a3644 = v5;
                        let a3645 = 40u64;
                        if if if if if if nat_eq(a3644, a3645) {
                            true
                        } else {
                            let a3646 = v5;
                            let a3647 = 41u64;
                            nat_eq(a3646, a3647)
                        } {
                            true
                        } else {
                            let a3648 = v5;
                            let a3649 = 124u64;
                            if nat_eq(a3648, a3649) {
                                true
                            } else {
                                let a3650 = v5;
                                let a3651 = 123u64;
                                nat_eq(a3650, a3651)
                            }
                        } {
                            true
                        } else {
                            let a3652 = v5;
                            let a3653 = 125u64;
                            if if nat_eq(a3652, a3653) {
                                true
                            } else {
                                let a3654 = v5;
                                let a3655 = 92u64;
                                nat_eq(a3654, a3655)
                            } {
                                true
                            } else {
                                let a3656 = v5;
                                let a3657 = 94u64;
                                if nat_eq(a3656, a3657) {
                                    true
                                } else {
                                    let a3658 = v5;
                                    let a3659 = 36u64;
                                    nat_eq(a3658, a3659)
                                }
                            }
                        } {
                            true
                        } else {
                            let a3660 = v5;
                            let a3661 = 42u64;
                            if if nat_eq(a3660, a3661) {
                                true
                            } else {
                                let a3662 = v5;
                                let a3663 = 43u64;
                                nat_eq(a3662, a3663)
                            } {
                                true
                            } else {
                                let a3664 = v5;
                                let a3665 = 63u64;
                                nat_eq(a3664, a3665)
                            }
                        } {
                            true
                        } else {
                            let a3666 = 127u64;
                            let a3667 = v5;
                            nat_lt(a3666, a3667)
                        } {
                            Err::<((bool, List<(u64, u64)>), u64), Bytes>(f7(List::cons(f8(Str::lit("the pattern construct ")), List::cons(f46(v1.clone(), v2, 1u64), List::cons(f8(Str::lit(" is not evaluated: only ^, $, ., character classes, literal ASCII characters and the quantifiers *, + and ? are (ECMA-262 section 22.2.1)")), List::<Bytes>::nil())))))
                        } else {
                            Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, List::cons((v5, v5), List::<(u64, u64)>::nil())), {
                                let a3668 = v2;
                                let a3669 = 1u64;
                                nat_add(a3668, a3669)?
                            }))
                        }
                    }
                }
            };
            match m3670 {
                Ok(v11) => {
                    let v12: u64 = {
                        let (_, h3671) = v11.clone();
                        h3671
                    };
                    let v13: u64 = {
                        let a3672 = v12;
                        let a3673 = v3;
                        if nat_lt(a3672, a3673) {
                            f706(f116(v1.clone(), v12)?)
                        } else {
                            0u64
                        }
                    };
                    let a3674 = v12;
                    let a3675 = v3;
                    if if nat_lt(a3674, a3675) {
                        let a3676 = f116(v1.clone(), v12)?;
                        let a3677 = 123u64;
                        nat_eq(a3676, a3677)
                    } else {
                        false
                    } {
                        Ok(Err::<List<Adt30>, Bytes>(f8(Str::lit("a {n,m} quantifier in a pattern is not evaluated (ECMA-262 section 22.2.1)"))))
                    } else {
                        let m3682 = f683(v4, v1, {
                            let a3678 = 0u64;
                            let a3679 = v13;
                            if nat_lt(a3678, a3679) {
                                let a3680 = v12;
                                let a3681 = 1u64;
                                nat_add(a3680, a3681)?
                            } else {
                                v12
                            }
                        }, v3)?;
                        match m3682 {
                            Ok(v15) => {
                                Ok(Ok::<List<Adt30>, Bytes>(List::cons(Adt30::C0({
                                    let (h3683, _) = {
                                        let (h3684, _) = v11.clone();
                                        h3684
                                    };
                                    h3683
                                }, {
                                    let (_, h3685) = {
                                        let (h3686, _) = v11;
                                        h3686
                                    };
                                    h3685
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

pub fn f684(v0: Adt30, v1: u64) -> bool {
    let m3687 = v0;
    match m3687 {
        Adt30::C0(v2, v3, _) => {
            if v2 {
                f682(f707(v3, v1))
            } else {
                f707(v3, v1)
            }
        }
    }
}

pub fn f685(v0: List<Adt30>, p3694: List<u64>, v2: bool) -> R<bool> {
    let m3693 = p3694;
    {
        let mut m3693 = m3693;
        loop {
            let step = {
                let v1 = m3693;
                let m3688 = v1;
                match m3688.uncons() {
                    None => {
                        Err::<List<u64>, R<bool>>(Ok(false))
                    }
                    Some((_, v4)) => {
                        if f651({
                            let a3691 = {
                                let a3689 = f652(v0.clone())?;
                                let a3690 = f602(v4.clone())?;
                                nat_add(a3689, a3690)?
                            };
                            let a3692 = 2u64;
                            nat_add(a3691, a3692)?
                        }, v0.clone(), v4.clone(), v2)? {
                            Err::<List<u64>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<u64>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3693 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3695, _) = v0;
        h3695
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3696) = v0;
        h3696
    };
    let m3697 = v1;
    match m3697.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3698 = v4;
            match m3698.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f708(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f687(v0: Fn23, p3703: u64, p3704: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3702 = (p3703, p3704);
    {
        let mut m3702 = m3702;
        loop {
            let step = {
                let (v1, v2) = m3702;
                let m3699 = v1;
                if m3699 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3699 - 1;
                    let m3701 = {
                        let c3700: Fn23 = v0.clone();
                        c3700.apply(v2.clone())
                    };
                    match m3701 {
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
                Ok(next) => { m3702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f688(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f709(v0)
}

pub fn f689(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3705 = v1;
    match m3705 {
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
                let a3716 = if f118(v7, {
                    let a3714 = 0u64;
                    let a3715 = v8;
                    nat_lt(a3714, a3715)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3717 = f299(false, v8)?;
                append_bytes(a3716, a3717)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3708 = f710({
                let a3706 = f8(Str::lit("n"));
                let a3707 = f462(false, v4, v5)?;
                append_bytes(a3706, a3707)
            })?;
            match m3708 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3712 = f8(Str::lit("nMinus"));
                        let a3713 = f46(v6.clone(), 1u64, {
                            let a3710 = {
                                let a3709 = v6;
                                length_bytes(a3709)
                            };
                            let a3711 = 1u64;
                            nat_sub(a3710, a3711)
                        });
                        append_bytes(a3712, a3713)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f694(v0, v2)
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

pub fn f690(v0: Bytes) -> R<Option<Bytes>> {
    let m3718 = f711(v0)?;
    match m3718 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f712(v1)?))
        }
    }
}

pub fn f691(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3719 = v1;
    match m3719.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3720 = f694(v0, v2)?;
            match m3720 {
                None => {
                    f691(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f691(v0, v3)?))
                }
            }
        }
    }
}

pub fn f692(p3724: List<Adt2>, v1: Bytes) -> bool {
    let m3723 = p3724;
    {
        let mut m3723 = m3723;
        loop {
            let step = {
                let v0 = m3723;
                let m3721 = v0;
                match m3721.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3722 = f702(v2, v1.clone());
                        if match m3722 {
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
                Ok(next) => { m3723 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f693(v0: Adt2) -> R<Option<Bytes>> {
    let m3725 = f381(f275(v0.clone(), f8(Str::lit("$ref"))));
    match m3725 {
        None => {
            Ok(f381(f275(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f619(v1)
        }
    }
}

pub fn f694(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f710(v1)
    } else {
        f690(v1)
    }
}

pub fn f695(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f713(f589(v0), v1)
}

pub fn f696(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt29, Bytes>> {
    let m3726 = v2;
    match m3726.uncons() {
        None => {
            Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3727 = f702(v5.clone(), v3.clone());
            match m3727 {
                None => {
                    Ok(Err::<Adt29, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3728 = f694(v0, v7.clone())?;
                    match m3728 {
                        None => {
                            Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f52(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt29, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3731 = f626({
                                    let a3729 = v1.clone();
                                    let a3730 = f375(v7)?;
                                    append_bytes(a3729, a3730)
                                }, f551(f512(f275(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f279(f280(f275(v5, f8(Str::lit("required"))))), true)?;
                                match m3731 {
                                    Ok(v10) => {
                                        let m3732 = f696(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3732 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f391(f628(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f628(v12.clone())), f516(f629(v10), f629(v12)))))
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

pub fn f697(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3733 = v0;
    match m3733.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3734 = f702(v2, v1.clone());
            match m3734 {
                None => {
                    f697(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f697(v3, v1))
                }
            }
        }
    }
}

pub fn f698(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3735 = f694(v0, v1)?;
    match m3735 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f32(v2.clone(), f8(Str::lit("mk"))) {
                f587(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f699(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3736 = v1;
    match m3736.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3737 = f698(v0, v2)?;
            match m3737 {
                None => {
                    f699(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f699(v0, v3)?))
                }
            }
        }
    }
}

pub fn f700(v0: Adt2) -> Option<Adt2> {
    let m3738 = f512(f275(v0, f8(Str::lit("patternProperties"))));
    match m3738.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3739 = v2;
            match m3739.uncons() {
                None => {
                    Some({
                        let (_, h3740) = v1;
                        h3740
                    })
                }
                Some((_, _)) => {
                    None::<Adt2>
                }
            }
        }
    }
}

pub fn f701(v0: Adt2) -> bool {
    f32(f278(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f702(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f52(v1.clone(), f279(f280(f275(v0.clone(), f8(Str::lit("required")))))) {
        let m3741 = f275(v0, f8(Str::lit("properties")));
        match m3741 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3742 = f275(v2, v1);
                match m3742 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f381(f275(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f703(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3745 = {
        let a3743 = v0;
        let a3744 = v1;
        index_bytes(a3743, a3744)
    };
    match m3745 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f118({
                let a3746 = 128u64;
                let a3747 = v3;
                nat_le(a3746, a3747)
            }, {
                let a3748 = v3;
                let a3749 = 191u64;
                nat_le(a3748, a3749)
            }) {
                Ok(Some({
                    let a3750 = v3;
                    let a3751 = 128u64;
                    nat_sub(a3750, a3751)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f704(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f118({
        let a3752 = v1;
        let a3753 = v0;
        nat_le(a3752, a3753)
    }, f118({
        let a3754 = v0;
        let a3755 = v2;
        nat_le(a3754, a3755)
    }, f202({
        let a3756 = v0;
        let a3757 = 55296u64;
        nat_lt(a3756, a3757)
    }, {
        let a3758 = 57343u64;
        let a3759 = v0;
        nat_lt(a3758, a3759)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f705(p3796: u64, v1: Bytes, p3797: u64, p3798: List<(u64, u64)>) -> R<Result<(List<(u64, u64)>, u64), Bytes>> {
    let m3795 = (p3796, (p3797, p3798));
    {
        let mut m3795 = m3795;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3795;
                let m3760 = v0;
                if m3760 == 0 {
                    Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                } else {
                    let v4 = m3760 - 1;
                    let v5: u64 = f116(v1.clone(), v2)?;
                    let a3762 = {
                        let a3761 = v1.clone();
                        length_bytes(a3761)
                    };
                    let a3763 = v2;
                    if nat_le(a3762, a3763) {
                        Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                    } else {
                        let a3764 = v5;
                        let a3765 = 93u64;
                        if nat_eq(a3764, a3765) {
                            Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Ok::<(List<(u64, u64)>, u64), Bytes>((v3, {
                                let a3766 = v2;
                                let a3767 = 1u64;
                                nat_add(a3766, a3767)?
                            }))))
                        } else {
                            let a3768 = v5;
                            let a3769 = 92u64;
                            if if nat_eq(a3768, a3769) {
                                true
                            } else {
                                let a3770 = 127u64;
                                let a3771 = v5;
                                nat_lt(a3770, a3771)
                            } {
                                Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("an escape or a non-ASCII character in a pattern's character class is not evaluated (ECMA-262 section 22.2.1)")))))
                            } else {
                                let a3774 = f116(v1.clone(), {
                                    let a3772 = v2;
                                    let a3773 = 1u64;
                                    nat_add(a3772, a3773)?
                                })?;
                                let a3775 = 45u64;
                                if if nat_eq(a3774, a3775) {
                                    f118({
                                        let a3779 = {
                                            let a3776 = v2;
                                            let a3777 = 2u64;
                                            nat_add(a3776, a3777)?
                                        };
                                        let a3780 = {
                                            let a3778 = v1.clone();
                                            length_bytes(a3778)
                                        };
                                        nat_lt(a3779, a3780)
                                    }, f118(f682({
                                        let a3783 = f116(v1.clone(), {
                                            let a3781 = v2;
                                            let a3782 = 2u64;
                                            nat_add(a3781, a3782)?
                                        })?;
                                        let a3784 = 93u64;
                                        nat_eq(a3783, a3784)
                                    }), f682({
                                        let a3787 = f116(v1.clone(), {
                                            let a3785 = v2;
                                            let a3786 = 2u64;
                                            nat_add(a3785, a3786)?
                                        })?;
                                        let a3788 = 92u64;
                                        nat_eq(a3787, a3788)
                                    })))
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3789 = v2;
                                        let a3790 = 3u64;
                                        nat_add(a3789, a3790)?
                                    }, List::cons((v5, f116(v1.clone(), {
                                        let a3791 = v2;
                                        let a3792 = 2u64;
                                        nat_add(a3791, a3792)?
                                    })?), v3))))
                                } else {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3793 = v2;
                                        let a3794 = 1u64;
                                        nat_add(a3793, a3794)?
                                    }, List::cons((v5, v5), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3795 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f706(v0: u64) -> u64 {
    let a3799 = v0;
    let a3800 = 42u64;
    if nat_eq(a3799, a3800) {
        1u64
    } else {
        let a3801 = v0;
        let a3802 = 43u64;
        if nat_eq(a3801, a3802) {
            2u64
        } else {
            let a3803 = v0;
            let a3804 = 63u64;
            if nat_eq(a3803, a3804) {
                3u64
            } else {
                0u64
            }
        }
    }
}

pub fn f707(p3813: List<(u64, u64)>, v1: u64) -> bool {
    let m3812 = p3813;
    {
        let mut m3812 = m3812;
        loop {
            let step = {
                let v0 = m3812;
                let m3805 = v0;
                match m3805.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3807 = {
                            let (h3806, _) = v2;
                            h3806
                        };
                        let a3808 = v1;
                        if if nat_le(a3807, a3808) {
                            let a3810 = v1;
                            let a3811 = {
                                let (_, h3809) = v2;
                                h3809
                            };
                            nat_le(a3810, a3811)
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
                Ok(next) => { m3812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f708(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3818 = {
            let a3816 = {
                let a3814 = v0.clone();
                length_list(a3814)
            };
            let a3817 = {
                let a3815 = v1.clone();
                length_list(a3815)
            };
            nat_add(a3816, a3817)?
        };
        let a3819 = 1u64;
        nat_add(a3818, a3819)?
    };
    Ok(f714({
        let (_, h3820) = {
            let (_, h3821) = {
                let (h3822, _) = f715(Fn24::F716, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3822
            };
            h3821
        };
        h3820
    }))
}

pub fn f709(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3823, _) = v0.clone();
        h3823
    };
    let v2: List<(Bytes, ())> = {
        let (h3824, _) = {
            let (_, h3825) = v0.clone();
            h3825
        };
        h3824
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3826) = {
            let (_, h3827) = v0;
            h3827
        };
        h3826
    };
    let m3828 = v1.clone();
    match m3828.uncons() {
        None => {
            let m3829 = v2;
            match m3829.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3830 = v2.clone();
            match m3830.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f717({
                        let (h3831, _) = v8.clone();
                        h3831
                    }, {
                        let (h3832, _) = v6.clone();
                        h3832
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

pub fn f710(v0: Bytes) -> R<Option<Bytes>> {
    let m3834 = f718({
        let a3833 = v0.clone();
        length_bytes(a3833)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3834 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3836 = {
                let a3835 = v1.clone();
                length_bytes(a3835)
            };
            let a3837 = 0u64;
            if nat_eq(a3836, a3837) {
                let a3839 = {
                    let a3838 = v0;
                    length_bytes(a3838)
                };
                let a3840 = 0u64;
                if nat_eq(a3839, a3840) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f712(v1)?))
            }
        }
    }
}

pub fn f711(v0: Bytes) -> R<Option<Bytes>> {
    if f719({
        let a3841 = v0.clone();
        length_bytes(a3841)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f720(v0)?;
        if f721(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3844 = {
                let a3842 = v1.clone();
                let a3843 = 0u64;
                index_bytes(a3842, a3843)
            };
            match m3844 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f510(f72(v2)?) {
                        Ok(Some({
                            let a3845 = f8(Str::lit("n"));
                            let a3846 = v1;
                            append_bytes(a3845, a3846)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f722({
            let a3847 = v0.clone();
            length_bytes(a3847)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f712(v0: Bytes) -> R<Bytes> {
    if f52(v0.clone(), f723()) {
        f587(v0)
    } else {
        Ok(v0)
    }
}

pub fn f713(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3848 = v0.clone();
    match m3848 {
        Adt2::C0 => {
            Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C1(_, _) => {
            Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(v4) => {
            if v4 {
                Ok(f673())
            } else {
                Ok(f622(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C3(_, _) => {
            Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_, _, _) => {
            Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_) => {
            Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C6(_) => {
            Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C7(_) => {
            let m3849 = f724(f549(v0.clone()));
            match m3849 {
                None => {
                    let m3850 = f381(f275(v0.clone(), f8(Str::lit("$ref"))));
                    match m3850 {
                        None => {
                            let m3852 = f275(v0.clone(), f8(Str::lit("allOf")));
                            match m3852 {
                                None => {
                                    let m3857 = f275(v0.clone(), f8(Str::lit("enum")));
                                    match m3857 {
                                        None => {
                                            let m3858 = f275(v0.clone(), f8(Str::lit("const")));
                                            match m3858 {
                                                None => {
                                                    let m3860 = f275(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3860 {
                                                        None => {
                                                            let v32: Bytes = f278(v0.clone(), f8(Str::lit("type")));
                                                            if if f32(v32.clone(), Bytes::lit(&[])) {
                                                                !f725(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f673())
                                                            } else {
                                                                if f32(v32.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f623(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f32(v32.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f623(f726(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f32(v32.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f623(f727(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f32(v32.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f623(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f32(v32.clone(), f8(Str::lit("object"))) {
                                                                                    let m3861 = f512(f275(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3861.uncons() {
                                                                                        None => {
                                                                                            if f630(v0.clone()) {
                                                                                                Ok(f623(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                            } else {
                                                                                                let m3862 = f668(v0);
                                                                                                match m3862 {
                                                                                                    None => {
                                                                                                        Ok(f671(f8(Str::lit("List")), f728(f673())))
                                                                                                    }
                                                                                                    Some(v35) => {
                                                                                                        Ok(f671(f8(Str::lit("List")), f728(f729(v35, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f623(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f622(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v32, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f623(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v29) => {
                                                    let m3859 = f381(Some(v29));
                                                    match m3859 {
                                                        None => {
                                                            Ok(f622(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f623(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f623(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v21) => {
                                    let m3853 = f382(v21);
                                    match m3853.uncons() {
                                        None => {
                                            Ok(f622(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v22, v23)) => {
                                            let m3854 = v23;
                                            match m3854.uncons() {
                                                None => {
                                                    let m3855 = f381(f275(v22, f8(Str::lit("$ref"))));
                                                    match m3855 {
                                                        None => {
                                                            Ok(f622(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v26) => {
                                                            let m3856 = f619(v26)?;
                                                            match m3856 {
                                                                None => {
                                                                    Ok(f622(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v27) => {
                                                                    Ok(f623(v27, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f622(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v19) => {
                            let m3851 = f619(v19)?;
                            match m3851 {
                                None => {
                                    Ok(f622(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v20) => {
                                    Ok(f623(v20, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v18) => {
                    Ok(f622(if f32(v18.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v18, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C8 => {
            Ok(f673())
        }
        Adt2::C9(_, _) => {
            Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C10(_, _, _) => {
            Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f714(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f399(Fn6::F730, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f715(v0: Fn24, p3867: u64, p3868: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3866 = (p3867, p3868);
    {
        let mut m3866 = m3866;
        loop {
            let step = {
                let (v1, v2) = m3866;
                let m3863 = v1;
                if m3863 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3863 - 1;
                    let m3865 = {
                        let c3864: Fn24 = v0.clone();
                        c3864.apply(v2.clone())
                    };
                    match m3865 {
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
                Ok(next) => { m3866 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f716(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f731(v0)
}

pub fn f717(v0: Bytes, v1: Bytes) -> bool {
    let a3873 = {
        let a3869 = v0;
        let a3870 = v1;
        compare_bytes(a3869, a3870)
    };
    let a3874 = {
        let a3871 = Bytes::lit(&[]);
        let a3872 = Bytes::lit(&[0x01]);
        compare_bytes(a3871, a3872)
    };
    equal(a3873, a3874)
}

pub fn f718(p3887: u64, v1: Bytes, p3888: u64, p3889: Bytes) -> R<Option<Bytes>> {
    let m3886 = (p3887, (p3888, p3889));
    {
        let mut m3886 = m3886;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3886;
                let m3875 = v0;
                if m3875 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3875 - 1;
                    let a3877 = {
                        let a3876 = v1.clone();
                        length_bytes(a3876)
                    };
                    let a3878 = v2;
                    if nat_le(a3877, a3878) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3879 = f681(v1.clone(), v2)?;
                        match m3879 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3881 = v2;
                                    let a3882 = {
                                        let (_, h3880) = v5;
                                        h3880
                                    };
                                    nat_add(a3881, a3882)?
                                }, {
                                    let a3884 = v3;
                                    let a3885 = f732(v1.clone(), v2, {
                                        let (h3883, _) = v5;
                                        h3883
                                    });
                                    append_bytes(a3884, a3885)
                                })))
                            }
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

pub fn f719(p3899: u64, v1: Bytes, p3900: u64) -> R<bool> {
    let m3898 = (p3899, p3900);
    {
        let mut m3898 = m3898;
        loop {
            let step = {
                let (v0, v2) = m3898;
                let m3890 = v0;
                if m3890 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3890 - 1;
                    let m3893 = {
                        let a3891 = v1.clone();
                        let a3892 = v2;
                        index_bytes(a3891, a3892)
                    };
                    match m3893 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3894 = f72(v4)?;
                            let a3895 = 128u64;
                            if nat_lt(a3894, a3895) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3896 = v2;
                                    let a3897 = 1u64;
                                    nat_add(a3896, a3897)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3898 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f720(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f458(v0.clone(), 0u64, {
        let a3901 = v0;
        length_bytes(a3901)
    }, false, Bytes::lit(&[]))?;
    let m3904 = {
        let a3902 = v1.clone();
        let a3903 = 0u64;
        index_bytes(a3902, a3903)
    };
    match m3904 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f72(v2)?;
            if f509(v3) {
                let a3911 = f46({
                    let a3905 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3905)
                }, {
                    let a3906 = v3;
                    let a3907 = 65u64;
                    nat_sub(a3906, a3907)
                }, 1u64);
                let a3912 = f46(v1.clone(), 1u64, {
                    let a3909 = {
                        let a3908 = v1;
                        length_bytes(a3908)
                    };
                    let a3910 = 1u64;
                    nat_sub(a3909, a3910)
                });
                Ok(append_bytes(a3911, a3912))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f721(v0: Bytes) -> R<bool> {
    let m3915 = {
        let a3913 = v0;
        let a3914 = 0u64;
        index_bytes(a3913, a3914)
    };
    match m3915 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f202(f508(f72(v1)?), f509(f72(v1)?)))
        }
    }
}

pub fn f722(p3928: u64, v1: Bytes, p3929: u64, p3930: Bytes) -> R<Option<Bytes>> {
    let m3927 = (p3928, (p3929, p3930));
    {
        let mut m3927 = m3927;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3927;
                let m3916 = v0;
                if m3916 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3916 - 1;
                    let a3918 = {
                        let a3917 = v1.clone();
                        length_bytes(a3917)
                    };
                    let a3919 = v2;
                    if nat_le(a3918, a3919) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3920 = f681(v1.clone(), v2)?;
                        match m3920 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3922 = v2;
                                    let a3923 = {
                                        let (_, h3921) = v5;
                                        h3921
                                    };
                                    nat_add(a3922, a3923)?
                                }, {
                                    let a3925 = v3;
                                    let a3926 = f733({
                                        let (h3924, _) = v5;
                                        h3924
                                    });
                                    append_bytes(a3925, a3926)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3927 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f723() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f724(p3935: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m3934 = p3935;
    {
        let mut m3934 = m3934;
        loop {
            let step = {
                let v0 = m3934;
                let m3931 = v0;
                match m3931.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f52({
                            let (h3932, _) = v1.clone();
                            h3932
                        }, f734()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h3933, _) = v1;
                                h3933
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3934 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f725(v0: Adt2) -> bool {
    if f635(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f635(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f635(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f635(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f635(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f635(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f726(v0: Adt2) -> Bytes {
    let m3936 = f603(f275(v0, f8(Str::lit("minimum"))));
    match m3936 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f727() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f728(v0: Result<Adt28, Bytes>) -> Result<Adt28, Bytes> {
    let m3937 = v0;
    match m3937 {
        Ok(v2) => {
            f623(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f557(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f556(v2))
        }
        Err(v1) => {
            Err::<Adt28, Bytes>(v1)
        }
    }
}

pub fn f729(v0: Adt2, v1: Bytes) -> R<Result<Adt28, Bytes>> {
    f735(16u64, v0, v1)
}

pub fn f730(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f731(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3938, _) = v0.clone();
        h3938
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3939, _) = {
            let (_, h3940) = v0.clone();
            h3940
        };
        h3939
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3941) = {
            let (_, h3942) = v0;
            h3942
        };
        h3941
    };
    let m3943 = v1.clone();
    match m3943.uncons() {
        None => {
            let m3944 = v2;
            match m3944.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3945 = v2.clone();
            match m3945.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f717({
                        let (h3946, _) = v8.clone();
                        h3946
                    }, {
                        let (h3947, _) = v6.clone();
                        h3947
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

pub fn f732(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f202(f118(f202(f508(v2), f509(v2)), f118({
        let a3948 = v2;
        let a3949 = 117u64;
        !nat_eq(a3948, a3949)
    }, {
        let a3950 = v2;
        let a3951 = 85u64;
        !nat_eq(a3950, a3951)
    })), f118(f510(v2), {
        let a3952 = 0u64;
        let a3953 = v1;
        nat_lt(a3952, a3953)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a3954 = v2;
        let a3955 = 65536u64;
        if nat_lt(a3954, a3955) {
            let a3956 = f8(Str::lit("u"));
            let a3957 = f620(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3956, a3957)
        } else {
            let a3958 = f8(Str::lit("U"));
            let a3959 = f620(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3958, a3959)
        }
    }
}

pub fn f733(v0: u64) -> Bytes {
    let a3960 = f8(Str::lit("u"));
    let a3961 = f620(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3960, a3961)
}

pub fn f734() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f735(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3962 = v0;
    if m3962 == 0 {
        Ok(f622(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3962 - 1;
        let m3963 = v1.clone();
        match m3963 {
            Adt2::C0 => {
                Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C1(_, _) => {
                Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(f673())
                } else {
                    Ok(f622(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C3(_, _) => {
                Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_, _, _) => {
                Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_) => {
                Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C6(_) => {
                Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C7(_) => {
                let m3964 = f381(f275(v1.clone(), f8(Str::lit("$ref"))));
                match m3964 {
                    None => {
                        if if f635(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f635(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f635(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f623(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v22: Bytes = f278(v1.clone(), f8(Str::lit("type")));
                            if if f32(v22.clone(), Bytes::lit(&[])) {
                                !f725(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f673())
                            } else {
                                if f32(v22.clone(), f8(Str::lit("string"))) {
                                    Ok(f623(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f32(v22.clone(), f8(Str::lit("integer"))) {
                                        Ok(f623(f726(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f32(v22.clone(), f8(Str::lit("number"))) {
                                            Ok(f623(f727(), List::<Adt25>::nil()))
                                        } else {
                                            if f32(v22.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f623(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f32(v22.clone(), f8(Str::lit("array"))) {
                                                    let m3966 = f275(v1, f8(Str::lit("items")));
                                                    match m3966 {
                                                        None => {
                                                            Ok(f671(f8(Str::lit("List")), f673()))
                                                        }
                                                        Some(v23) => {
                                                            Ok(f671(f8(Str::lit("List")), f735(v3, v23, {
                                                                let a3967 = v2;
                                                                let a3968 = f8(Str::lit("Item"));
                                                                append_bytes(a3967, a3968)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v22.clone(), f8(Str::lit("object"))) {
                                                        if f630(v1.clone()) {
                                                            Ok(f623(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                                                        } else {
                                                            let m3969 = f668(v1);
                                                            match m3969 {
                                                                None => {
                                                                    Ok(f671(f8(Str::lit("List")), f728(f673())))
                                                                }
                                                                Some(v24) => {
                                                                    Ok(f671(f8(Str::lit("List")), f728(f735(v3, v24, {
                                                                        let a3970 = v2;
                                                                        let a3971 = f8(Str::lit("Value"));
                                                                        append_bytes(a3970, a3971)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f622(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v22, List::<Bytes>::nil()))), v2))
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
                        let m3965 = f619(v20)?;
                        match m3965 {
                            None => {
                                Ok(f622(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v21) => {
                                Ok(f623(v21, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C8 => {
                Ok(f673())
            }
            Adt2::C9(_, _) => {
                Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C10(_, _, _) => {
                Ok(f622(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}
