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
    F121,
    F240,
    F353,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F121 => f121(p0, p1),
            Fn1::F240 => f240(p0, p1),
            Fn1::F353 => f353(p0, p1),
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
    F322(Bytes),
    F323(Bytes),
    F424(Bytes),
    F425(u64, Bytes),
    F432(Bytes),
    F440(Bytes, u64, Bytes, u64),
    F502(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F322(k0) => f322(k0.clone(), p0),
            Fn3::F323(k0) => f323(k0.clone(), p0),
            Fn3::F424(k0) => Ok(f424(k0.clone(), p0)),
            Fn3::F425(k0, k1) => f425(*k0, k1.clone(), p0),
            Fn3::F432(k0) => f432(k0.clone(), p0),
            Fn3::F440(k0, k1, k2, k3) => f440(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F502(k0) => f502(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F326(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn4::F326(k0) => f326(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F416,
    F607,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F416 => f416(p0, p1),
            Fn5::F607 => f607(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F419,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F419 => f419(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F451,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn7::F451 => f451(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F454,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn8::F454 => f454(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F456,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn9::F456 => f456(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F459,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn10::F459 => f459(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F482(Bytes, u64, u64),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Adt23)) -> R<(u64, Adt23)> {
        match self {
            Fn11::F482(k0, k1, k2) => f482(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F513,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F513 => f513(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F516,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F516 => f516(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F536(u64, Bytes),
    F547(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn14::F536(k0, k1) => Ok(f536(*k0, k1.clone(), p0)),
            Fn14::F547(k0) => f547(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F544,
}

impl Fn15 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn15::F544 => f544(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F546,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn16::F546 => f546(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F550(Bytes),
}

impl Fn17 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn17::F550(k0) => f550(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F559,
}

impl Fn18 {
    pub fn apply(&self, p0: Adt25) -> R<Adt25> {
        match self {
            Fn18::F559 => f559(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F566,
}

impl Fn19 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn19::F566 => f566(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F571,
}

impl Fn20 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn20::F571 => f571(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F590,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F590 => f590(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F593,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F593 => f593(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F600,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F600 => f600(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F605,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F605 => f605(p0),
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
                    Ok(Adt12::C3(f154(f155(v0.clone())?, List::cons((f156(), f157()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f158()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f159()), List::cons((f8(Str::lit(".github/scripts/export.sh")), f160()), List::cons((f8(Str::lit(".github/scripts/readmit.sh")), f161()), List::cons((f8(Str::lit(".github/scripts/merge-entries.py")), f162()), List::cons((f8(Str::lit(".github/scripts/prism-acceptance.py")), f163()), List::cons((f8(Str::lit(".github/scripts/prepare-change.py")), f164()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f165()), List::cons((f8(Str::lit(".github/lexlean.pin")), f166()), List::cons((f8(Str::lit("docs/catalog.md")), f167(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f168(v0.clone())?), List::cons((f8(Str::lit("app/src/Studio/Prism/Closure/Register.lex.tex")), f169(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f170(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f171()), List::<(Bytes, Bytes)>::nil())))))))))))))))), f8(Str::lit("published the workflows, the export, readmit, entry-merge, change-preparing and Prism acceptance scripts, CODEOWNERS, the lexlean pin, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md, docs/owner-decisions.md and the Prism closure register\u{a}"))))
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
        f172(v3.clone(), v1.clone(), f173(v3, v1, false)?)
    } else {
        if f32(v0.clone(), f8(Str::lit("stage-bindings"))) {
            f172(v3.clone(), v1.clone(), f174(v3, v1)?)
        } else {
            if f32(v0.clone(), f8(Str::lit("stage-decisions"))) {
                f172(v3.clone(), v1.clone(), f173(v3, v1, true)?)
            } else {
                if f32(v0.clone(), f8(Str::lit("stage-registries"))) {
                    f172(v3.clone(), v1.clone(), f175(v3, v1)?)
                } else {
                    if f32(v0.clone(), f8(Str::lit("stage-encodings"))) {
                        f172(v3.clone(), v1.clone(), f176(v3, v1)?)
                    } else {
                        if f32(v0.clone(), f8(Str::lit("stage-mappings"))) {
                            f172(v3.clone(), v1.clone(), f177(v3, v1, false)?)
                        } else {
                            if f32(v0, f8(Str::lit("stage-matrix"))) {
                                f172(v3.clone(), v1.clone(), f177(v3, v1, true)?)
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
            f178(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("ingest-index"))) {
                f179(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("merge-gate"))) {
                    f180(v3, v1)
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

pub fn f88(v0: Bytes) -> R<bool> {
    f183(v0.clone(), 0u64, {
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
    let m490 = f184(f185(v0.clone(), v1.clone()), v1.clone());
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
            let m500 = f186(v2, v1.clone())?;
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
        let (h502, _) = f76(Fn0::F187(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
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
    Ok(f188(f57(v0, v1)?))
}

pub fn f97(v0: List<Adt2>, v1: Bytes) -> List<Adt2> {
    let m511 = v0;
    match m511.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f189(v2, v1.clone()), f97(v3, v1))
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
    let a520 = f190(v0)?;
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
        let m527 = f191(v0.clone(), f60())?;
        match m527 {
            None => {
                if f51(f52(f194(), match v0.clone() {
                    Adt1::C0(_, _, h528, _) => {
                        h528
                    }
                })) {
                    Ok(Adt12::C2(f8(Str::lit("date")), List::cons(f8(Str::lit("-u")), List::cons(f7(List::cons(Bytes::lit(&[0x2b, 0x25, 0x46]), List::<Bytes>::nil())), List::<Bytes>::nil())), f194()))
                } else {
                    let m529 = f56(v0.clone(), List::cons(f195(), f196(f60())?));
                    match m529.uncons() {
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
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f199(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m530 - 1;
                    let m533 = {
                        let a531 = v0.clone();
                        let a532 = v1;
                        index_bytes(a531, a532)
                    };
                    match m533 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f199(v0.clone(), v5, v1, v6)))
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
                                    }, f199(v0.clone(), v5, v1, v6)))))))
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
    let v2: u64 = f200(v0.clone(), v1, {
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
        f201(v0.clone(), v1, {
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
    let v4: u64 = f202(v0.clone(), v3)?;
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
        f202(v0.clone(), v7)?
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
    let v12: bool = f117(v10, f203({
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
        f202(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a648 = v14;
        let a649 = v13;
        nat_sub(a648, a649)
    });
    let v16: u64 = f204(v15.clone())?;
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
                f203(if v10 {
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
                f205(v17)?
            }
        };
        match m669 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m674 = f206(f46(v0.clone(), v3, v5), f46(v0, v7, {
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
                            let m680 = f205({
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
                                    if f207(v21, v20) {
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
    let m692 = f208(v0.clone(), v3);
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
    let m708 = f209(Fn2::F210(v1), None::<Bytes>, v0);
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
    }, f211()) {
        if f138(match v0 {
            Adt1::C0(_, h710, _, _) => {
                h710
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
    Ok(f56(v0.clone(), List::cons(f213(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f136(v0)?)))))
}

pub fn f127(v0: Adt1) -> R<Adt14> {
    Ok(Adt14::C0(f214(v0.clone())?, f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f142(f96(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
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
            f125(f215(f188(f93(v1, f8(Str::lit("defers"))))), f130(v2))
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
    let v3: List<(Bytes, Bytes)> = f154(f216(v2.clone()), f217(f218(v0, v1.clone(), v2)?));
    f219(f145(f220(v1))?, v3.clone(), {
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
            if if f221(v2.clone(), f8(Str::lit("subject"))) {
                f222(v0.clone(), v2.clone())
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
            let m723 = f223(f95(f64(v0.clone(), v2.clone()))?, f85(v2, f212()))?;
            match m723 {
                None => {
                    f135(v0, v3)
                }
                Some(v4) => {
                    Ok(f224(v4, f135(v0, v3)?))
                }
            }
        }
    }
}

pub fn f136(v0: Adt1) -> R<List<Bytes>> {
    Ok(f125(f145(f225(f211(), f147(match v0.clone() {
        Adt1::C0(_, h724, _, _) => {
            h724
        }
    }, f211())))?, f145(f225(f212(), f147(match v0 {
        Adt1::C0(_, h725, _, _) => {
            h725
        }
    }, f212())))?))
}

pub fn f137(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("-c")), List::cons(f226(), List::cons(f8(Str::lit("prism-code")), List::cons(f8(Str::lit("UOR-Foundation/prismpm")), List::cons(f227(), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil())))))))))
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
    f145(f225(f139(), f147(match v0 {
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
                f228(v4.clone(), f95(f64(v0.clone(), v4))?, v2.clone(), v3.clone())?
            } else {
                f229(v4, f8(Str::lit("the file has no row in model/admissions.json, so admit-registry did not write it")))
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
    let m733 = f56(v0.clone(), f230());
    match m733.uncons() {
        None => {
            let m734 = f231(v0.clone(), f230())?;
            match m734.uncons() {
                None => {
                    let m737 = f232(match v0.clone() {
                        Adt1::C0(_, h735, _, _) => {
                            h735
                        }
                    }, f125(f233(), f234(match v0.clone() {
                        Adt1::C0(_, h736, _, _) => {
                            h736
                        }
                    }, false)));
                    match m737.uncons() {
                        None => {
                            let m739 = f56(v0.clone(), f234(match v0.clone() {
                                Adt1::C0(_, h738, _, _) => {
                                    h738
                                }
                            }, true));
                            match m739.uncons() {
                                None => {
                                    let m740 = f56(v0.clone(), f235());
                                    match m740.uncons() {
                                        None => {
                                            let m741 = f236(v0.clone())?;
                                            match m741 {
                                                None => {
                                                    let m743 = f237(v0.clone(), f234(match v0.clone() {
                                                        Adt1::C0(_, h742, _, _) => {
                                                            h742
                                                        }
                                                    }, true))?;
                                                    match m743 {
                                                        None => {
                                                            let v13: (bool, Bytes) = f238(Adt16::C0(Adt15::C0(f63(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f214(v0.clone())?, f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f63(f64(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f96(v0.clone(), f8(Str::lit("model/strata.json")))?, f215(f96(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f234(match v0 {
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
    Ok(f239(f120(Fn1::F240, List::<Bytes>::nil(), f241(v0)?)))
}

pub fn f146(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m748 = v1;
    match m748.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f242(v0.clone(), v2), f146(v0, v3))
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
                        if f243(v0.clone(), v1.clone(), v3.clone())? {
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
            Ok(List::cons((f244(v1.clone())?, v1), f149(v2)?))
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
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f245(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f32(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f245(v1, List::<Bytes>::nil()), v2))))
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
    }, f246())) {
        Ok(Adt12::C1(List::cons(f246(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f146(f246(), f147(match v0.clone() {
            Adt1::C0(_, h765, _, _) => {
                h765
            }
        }, f246())))?, v1.clone());
        let m766 = f56(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m766.uncons() {
            None => {
                let v5: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m767 = f56(v0.clone(), f247(v0.clone(), v2.clone(), v5.clone())?);
                match m767.uncons() {
                    None => {
                        let m768 = f248(v0.clone(), v2, v5)?;
                        match m768 {
                            None => {
                                f249(v0, v1)
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
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f250(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f156() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f157() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: |\u{a}            target/release/verdant_studio\u{a}            target/release/verdant_readers\u{a}            target/release/verdant_xml\u{a}            target/release/verdant_admit\u{a}          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      # Restored on every run, not only when certified/ changed: the end-to-end propose-command step below\u{a}      # exports with the attested lexlean binary, and a fresh cargo install does not reproduce its hash.\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}          if [ -d certified-readers ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}          fi\u{a}          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}          # Building may write only target/: a stray file in a bundle fails verify-artifact on the next run (LLV7102).\u{a}          if [ -n \"$(git status --porcelain -- certified certified-readers certified-xml certified-admit)\" ]; then\u{a}            echo \"::error::building the certified commands left files in certified/, certified-readers/, certified-xml/ or certified-admit/:\"\u{a}            git status --short -- certified certified-readers certified-xml certified-admit | head -20\u{a}            exit 1\u{a}          fi\u{a}      # verify-artifact replays a bundle but does not compare it with the sources. So each LexLean project's\u{a}      # semantic id must equal the one its bundle records. Lock-level ids move with every admission to app/,\u{a}      # which readers depends on without using it, so they are not compared.\u{a}      - name: the certified bundles were exported from these sources (semantic ids, no Lean)\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The lexlean cache is keyed by .github/lexlean.pin, so after the pin moves the replay above may have been skipped with no binary restored.\u{a}          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          for pair in studio:certified readers:certified-readers readers/xml:certified-xml admit:certified-admit; do\u{a}            project=$(cut -d : -f 1 <<< \"$pair\")\u{a}            bundle=$(cut -d : -f 2 <<< \"$pair\")\u{a}            if [ ! -f \"$project/lexlean.toml\" ]; then continue; fi\u{a}            if [ ! -f \"$bundle/result.json\" ]; then echo \"::warning::$project has no committed bundle $bundle, so nothing records what it was exported from\"; continue; fi\u{a}            now=$(cd \"$project\" && \"$ll\" lock >/dev/null && \"$ll\" check | sed -n 's/.*semantic \\([0-9a-f]*\\)).*/\\1/p')\u{a}            git checkout -q -- \"$project/lexlean.lock\"\u{a}            was=$(jq -r .source.semantic_id \"$bundle/result.json\")\u{a}            if [ \"$now\" != \"$was\" ]; then echo \"::error::$project now has semantic id $now, but $bundle was exported from $was: export it again through propose-command\"; exit 1; fi\u{a}            echo \"$project matches $bundle (semantic $now)\"\u{a}          done\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          # Each encoding admitted through admit-translated recorded the reader command that printed it: run it again and fail if the output differs.\u{a}          target/release/verdant_readers verify-translations model/admissions.json\u{a}          if [ -x target/release/verdant_xml ]; then target/release/verdant_xml verify-translations model/admissions.json; fi\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept the Prism of the seed Verdant Studio, every facet at its media type and its generator's fidelity\u{a}        run: python3 .github/scripts/prism-acceptance.py node target/prismpm/sdk/npm/prism-code/bin/prism-code.mjs\u{a}      - name: next answers within a second, because operate's advance runs it on every push to main\u{a}        run: |\u{a}          set -euo pipefail\u{a}          best=999999\u{a}          for attempt in 1 2 3; do\u{a}            start=$EPOCHREALTIME\u{a}            target/release/verdant_studio next > target/next.txt || true\u{a}            end=$EPOCHREALTIME\u{a}            took=$(( (${end/./} - ${start/./}) / 1000 ))\u{a}            if [ \"$took\" -lt \"$best\" ]; then best=$took; fi\u{a}          done\u{a}          echo \"next took ${best} ms, the best of 3 runs\"\u{a}          if [ \"$best\" -gt 1000 ]; then echo \"::error::next took ${best} ms, over its 1 s budget: make the reads it repeats cheaper before adding more\"; exit 1; fi\u{a}      - if: github.event_name == 'pull_request'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - name: run propose-command end to end when the pull request changes the API's own workflows\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          BASE: ${{ github.base_ref }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # The studio check proves the model and replays the certified bundle, but the API's own paths\u{a}          # run in these workflows. So a change to them must pass operate's propose-command step itself,\u{a}          # on a fixture change, and that export must reproduce certified/ byte for byte.\u{a}          git fetch -q --depth=1 origin \"$BASE\"\u{a}          if git diff --quiet FETCH_HEAD -- .github; then echo \"the workflows are main's, which main's own runs exercise\"; exit 0; fi\u{a}          mkdir -p target\u{a}          python3 -c 'import yaml; steps = yaml.safe_load(open(\".github/workflows/operate.yml\"))[\"jobs\"][\"operate\"][\"steps\"]; print([step[\"run\"] for step in steps if step.get(\"name\", \"\").startswith(\"apply a change\")][0])' > target/propose.sh\u{a}          d=dev\u{a}          none=null\u{a}          {\u{a}            echo \"Fixture change for the studio check's end-to-end run of propose-command.\"\u{a}            echo\u{a}            echo \"diff --git a/features/suites/e2e.fixture b/features/suites/e2e.fixture\"\u{a}            echo \"new file mode 100644\"\u{a}            echo \"--- /$d/$none\"\u{a}            echo \"+++ b/features/suites/e2e.fixture\"\u{a}            echo \"@@ -0,0 +1 @@\"\u{a}            echo \"+fixture\"\u{a}          } > target/fixture.patch\u{a}          INPUT=\"$(cat target/fixture.patch)\" bash target/propose.sh\u{a}          rm -f features/suites/e2e.fixture\u{a}          # certified-xml/ and certified-admit/ join the comparison once main holds them: the export that first writes it is the one that adds it.\u{a}          bundles=\"certified certified-readers\"\u{a}          if [ -n \"$(git ls-files certified-xml | head -n 1)\" ]; then bundles=\"$bundles certified-xml\"; fi\u{a}          if [ -n \"$(git ls-files certified-admit | head -n 1)\" ]; then bundles=\"$bundles certified-admit\"; fi\u{a}          if [ -n \"$(git status --porcelain -- $bundles)\" ]; then\u{a}            echo \"::error::operate's propose-command step ran, but its export did not reproduce certified/:\"\u{a}            git status --short -- $bundles | head -20\u{a}            attested=$(git show HEAD:certified/evidence/attestation.json | jq -c .lexlean); used=$(jq -c .lexlean certified/evidence/attestation.json)\u{a}            [ \"$attested\" = \"$used\" ] || echo \"::error::the export ran with lexlean $used, but certified/ attests lexlean $attested. lexlean is built from .github/lexlean.pin with rust 1.97.0 and its cargo home remapped, so a different executable means the toolchain or the runner image changed\"\u{a}            exit 1\u{a}          fi\u{a}          echo \"propose-command applied the fixture, exported again with Lean and reproduced certified/ byte for byte\"\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f158() -> Bytes {
    f7(List::cons(f251(), List::cons(f252(), List::cons(f253(), List::cons(f254(), List::cons(f255(), List::cons(f256(), List::<Bytes>::nil())))))))
}

pub fn f159() -> Bytes {
    f7(List::cons(f257(), List::cons(f258(), List::cons(f259(), List::cons(f260(), List::<Bytes>::nil())))))
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
    f7(List::cons(f261(), List::cons(f262(), List::<Bytes>::nil())))
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
    let v4: List<List<Bytes>> = f263(f264(f64(v0, f213()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f265(v4.clone(), f266(v4), f218(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f133(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f168(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f267(f127(v0.clone())?, f135(v0.clone(), f136(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f169(v0: Adt1) -> R<Bytes> {
    let v1: List<List<Bytes>> = f263(f264(f64(v0.clone(), f213()))?);
    let v2: List<(Bytes, Bytes)> = f268(f266(v1.clone()), f135(v0.clone(), f136(v0)?)?);
    let v3: Bytes = f7(f269(v1, 0u64, 26u64)?);
    let v4: Bytes = f7(f145(f270(v2.clone(), v2)?)?);
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Studio.Prism.Closure.Register}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}definition{\u{a}  name: cells,\u{a}  parameters: [],\u{a}  result: Bytes,\u{a}  body: 0x")), List::cons(f271(v3.clone(), 0u64, {
        let a771 = v3;
        length_bytes(a771)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\u{a}}\u{a}\u{a}definition{\u{a}  name: pairs,\u{a}  parameters: [],\u{a}  result: Bytes,\u{a}  body: 0x")), List::cons(f271(v4.clone(), 0u64, {
        let a772 = v4;
        length_bytes(a772)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\u{a}}\u{a}\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f170(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f272(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f273(f96(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f171() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f172(v0: Adt1, v1: Bytes, v2: Adt12) -> R<Adt12> {
    let m773 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m773.uncons() {
        None => {
            if f274(f95(f64(v0, v1))?)? {
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

pub fn f173(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m774 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m774.uncons() {
        None => {
            let v5: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f275(v5)?
                } else {
                    f276(v5)?
                } {
                    Ok(Adt12::C3(List::cons((f242(f277(), f278(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f242(f277(), f278(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt12::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f279()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f174(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m775 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m775.uncons() {
        None => {
            let m776 = f280(f95(f64(v0.clone(), v1.clone()))?)?;
            match m776 {
                None => {
                    Ok(Adt12::C4(f281()))
                }
                Some(v4) => {
                    let m777 = f282(f95(f64(v0, v1))?)?;
                    match m777 {
                        None => {
                            Ok(Adt12::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f122(f49(v4), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt12::C4(f283(v5)))
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
    let v2: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
        Ok(Adt12::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
    } else {
        if f117(f284(f285(v2.clone())?), f51({
            let a779 = {
                let a778 = f286(f285(v2)?);
                length_list(a778)
            };
            let a780 = 0u64;
            nat_eq(a779, a780)
        })) {
            Ok(Adt12::C3(List::cons((f242(f277(), f278(v1.clone())?), f64(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f242(f277(), f278(v1)?), List::cons(f8(Str::lit(": next derives admit-registry for each entry that has no register yet\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok(Adt12::C4(f8(Str::lit("every line must be a registry entry: authority, row, construct (a Catalog type, or none), reader (a verdant_readers read- command) and artifact\u{a}"))))
        }
    }
}

pub fn f176(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m781 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m781.uncons() {
        None => {
            let m782 = f287(f95(f64(v0, v1))?)?;
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

pub fn f177(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt12> {
    let m783 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m783.uncons() {
        None => {
            let v5: Bytes = f242(if v2 {
                f212()
            } else {
                f211()
            }, f278(v1.clone())?);
            if f51(f41(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt12::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m784 = f223(f95(f64(v0.clone(), v1.clone()))?, v2)?;
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

pub fn f178(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m788 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m788.uncons() {
        None => {
            let m789 = f57(v0.clone(), v1.clone())?;
            match m789 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f51(f32(f58(v4.clone(), f8(Str::lit("schema"))), f288())) {
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
                        let v9: Bytes = f242(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f242(v9.clone(), f8(Str::lit("binding.json"))), f64(v0.clone(), v1));
                        let m791 = v8;
                        match m791 {
                            None => {
                                f289(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f51(f32(f58(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f289(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f242(v9, f278(f58(v7.clone(), f8(Str::lit("url"))))?);
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
                                                if f32(f290(f64(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f289(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f179(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m794 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m794.uncons() {
        None => {
            let v4: Bytes = f64(v0.clone(), v1);
            let v5: List<List<Bytes>> = f264(v4.clone())?;
            if f51(f98(f291(v5.clone()), f292())) {
                Ok(Adt12::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f293(Str::lit("id"), v6.clone())?, List::cons(f293(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f293(Str::lit("artifact"), f213())?, List::cons(f293(Str::lit("checksum"), f290(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt12::C3(List::cons((f213(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f294(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f295(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f122(f266(f263(v5)), f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f180(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m795 = f56(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m795.uncons() {
        None => {
            let m796 = f57(v0, v1)?;
            match m796 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f296(f188(Some(v4))) {
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

pub fn f181(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m797 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f153(), List::<Bytes>::nil())));
    match m797.uncons() {
        None => {
            let m798 = f297(f95(f64(v0.clone(), v1))?)?;
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
                        Ok(Adt12::C3(List::cons(f298(f153(), f224(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f122(v6, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
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
                            let m809 = f56(v0.clone(), List::cons(v1.clone(), List::cons(f299(), List::<Bytes>::nil())));
                            match m809.uncons() {
                                None => {
                                    let v11: Bytes = f122(v6, f8(Str::lit(" ")));
                                    let v12: Bytes = f64(v0.clone(), v1);
                                    let v13: List<Adt2> = f96(v0, f299())?;
                                    if f300(v13.clone(), v11.clone(), v12.clone()) {
                                        Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("this refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" is already recorded in model/errors.json\u{a}")), List::<Bytes>::nil()))))))
                                    } else {
                                        Ok(Adt12::C3(List::cons(f298(f299(), f224(v13, List::cons(f301(v11.clone(), v12, v5, v3), List::<Adt2>::nil())))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the refusal of ")), List::cons(v11, List::cons(f8(Str::lit(" in model/errors.json as a Studio.Error.Observation (W3C-SOSA-2017)\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f183(v0: Bytes, p863: u64, p864: u64) -> R<bool> {
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
                            if f302({
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

pub fn f184(p869: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
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

pub fn f185(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
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

pub fn f186(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
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

pub fn f187(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f303(v0, v1)
}

pub fn f188(v0: Option<Adt2>) -> List<Adt2> {
    let m876 = v0;
    match m876 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f304(v1)
        }
    }
}

pub fn f189(v0: Adt2, v1: Bytes) -> Adt2 {
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
            Adt2::C8(std::rc::Rc::new(f189(v5, v1)))
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
                Adt2::C11(v2, std::rc::Rc::new(f305(v3, v1.clone())), std::rc::Rc::new(f189(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f189(v4, v1)))
            }
        }
    }
}

pub fn f190(v0: Adt2) -> R<Bytes> {
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
            f306(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f307(v6, v7, v8)
        }
        Adt2::C5(v9, v10) => {
            f308(v9, v10)
        }
        Adt2::C6(v11) => {
            f309(v11)
        }
        Adt2::C7(r886) => {
            let v12 = (*r886).clone();
            Ok(f310(f311(v12)?))
        }
        Adt2::C8(r887) => {
            let v13 = (*r887).clone();
            f312(f313(v13)?)
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

pub fn f191(v0: Adt1, p891: List<Adt13>) -> R<Option<Adt13>> {
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
                        if f52(f192(v2.clone())?, match v0.clone() {
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

pub fn f192(v0: Adt13) -> R<Bytes> {
    Ok(f242(f314(), f278(f315(v0))?))
}

pub fn f193(v0: Adt13, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("https://raw.githubusercontent.com/afflom/verdant-engine/")), List::cons(v1, List::cons(f8(Str::lit("/")), List::cons(f315(v0), List::<Bytes>::nil())))))
}

pub fn f194() -> Bytes {
    f8(Str::lit("target/engine/date"))
}

pub fn f195() -> Bytes {
    f316()
}

pub fn f196(v0: List<Adt13>) -> R<List<Bytes>> {
    let m892 = v0;
    match m892.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f192(v1)?, f196(v2)?))
        }
    }
}

pub fn f197(v0: Adt1, v1: List<Adt13>, v2: Bytes, v3: Bytes, v4: Bytes) -> R<List<(Bytes, Bytes)>> {
    let m893 = v1;
    match m893.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v5, v6)) => {
            Ok(List::cons((f68(f69(v5.clone(), f61(v2.clone()))), f317(v5.clone(), v2.clone(), v3.clone(), f290(f64(v0.clone(), f192(v5)?))?, v4.clone())?), f197(v0, v6, v2, v3, v4)?))
        }
    }
}

pub fn f198(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f318(v0.clone())?;
    let v2: u64 = f319(v0.clone(), v1)?;
    Ok(f46(v0, v2, {
        let a894 = v1;
        let a895 = v2;
        nat_sub(a894, a895)
    }))
}

pub fn f199(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a898 = v3;
    let a899 = f46(v0, v1, {
        let a896 = v2;
        let a897 = v1;
        nat_sub(a896, a897)
    });
    append_bytes(a898, a899)
}

pub fn f200(v0: Bytes, p912: u64, p913: u64) -> R<u64> {
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

pub fn f201(v0: Bytes, p939: u64, p940: u64, p941: Bytes) -> R<Option<(u64, Bytes)>> {
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
                    let v5: u64 = f200(v0.clone(), v1, {
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
                            let m930 = f320(v0.clone(), v5, v6)?;
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

pub fn f202(v0: Bytes, v1: u64) -> R<u64> {
    let (h942, _) = f321(Fn3::F322(v0.clone()), {
        let a944 = {
            let a943 = v0;
            length_bytes(a943)
        };
        let a945 = 1u64;
        nat_add(a944, a945)?
    }, v1)?;
    Ok(h942)
}

pub fn f203(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f204(v0: Bytes) -> R<u64> {
    let (h946, _) = f321(Fn3::F323(v0.clone()), {
        let a948 = {
            let a947 = v0;
            length_bytes(a947)
        };
        let a949 = 1u64;
        nat_add(a948, a949)?
    }, 0u64)?;
    Ok(h946)
}

pub fn f205(v0: Bytes) -> R<Option<u64>> {
    if f324(v0.clone())? {
        Ok(Some({
            let (_, h950) = {
                let (h951, _) = f325(Fn4::F326(v0.clone()), {
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

pub fn f206(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a954 = {
            let a953 = v1.clone();
            length_bytes(a953)
        };
        let a955 = f327(v1)?;
        nat_sub(a954, a955)
    });
    let v5: Bytes = {
        let a956 = v0;
        let a957 = v4.clone();
        append_bytes(a956, a957)
    };
    let v6: u64 = f204(v5.clone())?;
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
        if f203(v2, {
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
            let v11: u64 = f327(v7.clone())?;
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
                    let a990 = f328(v12);
                    append_bytes(a989, a990)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f207(v0: bool, v1: u64) -> bool {
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

pub fn f208(v0: Bytes, v1: u64) -> Option<u8> {
    let a997 = v0;
    let a998 = v1;
    index_bytes(a997, a998)
}

pub fn f209(v0: Fn2, p1002: Option<Bytes>, p1003: List<Bytes>) -> Option<Bytes> {
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

pub fn f210(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
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
    let m1009 = f264(f64(v0, f213()))?;
    match m1009.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f266(v2))
        }
    }
}

pub fn f215(v0: List<Adt2>) -> List<Bytes> {
    let m1010 = v0;
    match m1010.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1011 = f92(Some(v1));
            match m1011 {
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

pub fn f216(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1012 = v0;
    match m1012.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f58(v1.clone(), f8(Str::lit("subject"))), f58(v1, f8(Str::lit("object")))), f216(v2))
        }
    }
}

pub fn f217(v0: List<Adt17>) -> List<(Bytes, Bytes)> {
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
            }), f217(v2))
        }
    }
}

pub fn f218(v0: Adt14, v1: List<Adt2>, v2: List<Adt2>) -> R<List<Adt17>> {
    let v3: List<(Bytes, Bytes)> = f329(v1);
    let v4: List<(Bytes, Bytes)> = f330(v0, v2);
    f331(v3.clone(), f154(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f219(p1022: List<Bytes>, v1: List<(Bytes, Bytes)>, p1023: u64) -> R<List<Bytes>> {
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
                    let v4: List<Bytes> = f145(f125(v0.clone(), f332(v0.clone(), v1.clone())))?;
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

pub fn f220(v0: List<Adt2>) -> List<Bytes> {
    let m1024 = v0;
    match m1024.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1025 = f93(v1, f8(Str::lit("std")));
            match m1025 {
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

pub fn f221(v0: Adt2, v1: Bytes) -> bool {
    f51({
        let a1027 = {
            let a1026 = f58(v0, v1);
            length_bytes(a1026)
        };
        let a1028 = 0u64;
        nat_eq(a1027, a1028)
    })
}

pub fn f222(v0: Adt14, v1: Adt2) -> bool {
    if f333(v0.clone(), v1.clone(), f58(v1.clone(), f8(Str::lit("subject")))) {
        f333(v0, v1.clone(), f58(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f223(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt2>>> {
    let m1029 = v0;
    match m1029.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f198(v2)?;
            let a1031 = {
                let a1030 = v4.clone();
                length_bytes(a1030)
            };
            let a1032 = 0u64;
            if nat_eq(a1031, a1032) {
                f223(v3, v1)
            } else {
                let m1033 = f43(v4)?;
                match m1033 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f334(v5.clone())
                        } else {
                            f335(v5.clone())
                        } {
                            let m1034 = f223(v3, v1)?;
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

pub fn f224(v0: List<Adt2>, v1: List<Adt2>) -> List<Adt2> {
    let m1035 = v0;
    match m1035.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f224(v3, v1))
        }
    }
}

pub fn f225(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m1036 = v1;
    match m1036.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f41(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f242(v0.clone(), v2), f225(v0, v3))
            } else {
                f225(v0, v3)
            }
        }
    }
}

pub fn f226() -> Bytes {
    f8(Str::lit("set -euo pipefail; repo=$1; rev=$2; shift 2; d=target/prismpm; if [ \"$(git -C $d rev-parse HEAD 2>/dev/null)\" != \"$rev\" ]; then rm -rf $d; git init -q $d; git -C $d remote add origin \"https://github.com/$repo\"; git -C $d sparse-checkout set --no-cone sdk/npm/prism-code; git -C $d fetch -q --depth=1 --filter=blob:none origin \"$rev\"; git -C $d checkout -q FETCH_HEAD; fi; exec node $d/sdk/npm/prism-code/bin/prism-code.mjs \"$@\""))
}

pub fn f227() -> Bytes {
    f8(Str::lit("047c868227ca45c5837bf07c1e7ed3b762d0f490"))
}

pub fn f228(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1037 = v1;
    match m1037.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v4, v5)) => {
            Ok(f125({
                let a1039 = {
                    let a1038 = f198(v4.clone())?;
                    length_bytes(a1038)
                };
                let a1040 = 0u64;
                if nat_eq(a1039, a1040) {
                    List::<Bytes>::nil()
                } else {
                    f336(v0.clone(), v4, v2.clone(), v3.clone())?
                }
            }, f228(v0, v5, v2, v3)?))
        }
    }
}

pub fn f229(v0: Bytes, v1: Bytes) -> List<Bytes> {
    List::cons(f7(List::cons(f8(Str::lit("registry ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), List::<Bytes>::nil())
}

pub fn f230() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f213(), List::<Bytes>::nil())))))))
}

pub fn f231(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
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
                        Ok(List::cons(v2, f231(v0, v3)?))
                    }
                    Some(_) => {
                        f231(v0, v3)
                    }
                }
            } else {
                f231(v0, v3)
            }
        }
    }
}

pub fn f232(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m1043 = v1;
    match m1043.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f138(v0.clone(), v2.clone()) {
                f232(v0, v3)
            } else {
                List::cons(v2, f232(v0, v3))
            }
        }
    }
}

pub fn f233() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f234(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m1044 = v0;
    match m1044.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f125(f337({
                let (h1045, _) = v2;
                h1045
            }, {
                let (_, h1046) = v2;
                h1046
            }, v1), f234(v3, v1))
        }
    }
}

pub fn f235() -> List<Bytes> {
    List::cons(f8(Str::lit(".github/workflows/pages.yml")), List::cons(f8(Str::lit(".github/workflows/operate.yml")), List::cons(f8(Str::lit(".github/workflows/refresh.yml")), List::cons(f8(Str::lit(".github/scripts/export.sh")), List::cons(f8(Str::lit(".github/scripts/readmit.sh")), List::<Bytes>::nil())))))
}

pub fn f236(v0: Adt1) -> R<Option<Bytes>> {
    f338(f339(v0, f235())?)
}

pub fn f237(v0: Adt1, p1050: List<Bytes>) -> R<Option<Bytes>> {
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
                        let m1048 = f340(f64(v0.clone(), v2.clone()), f341())?;
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

pub fn f238(v0: Adt16) -> R<(bool, Bytes)> {
    let v1: Adt15 = match v0.clone() {
        Adt16::C0(h1051, _, _, _, _, _) => {
            h1051
        }
    };
    let v2: Adt18 = f342(v1.clone())?;
    let v3: List<Bytes> = f343(v1.clone());
    let v4: List<Bytes> = f142(match v0.clone() {
        Adt16::C0(_, _, _, h1052, _, _) => {
            h1052
        }
    }, f8(Str::lit("path")));
    let m1054 = f344(v2, List::<u64>::nil(), f345(v1.clone(), match v0.clone() {
        Adt16::C0(_, h1053, _, _, _, _) => {
            h1053
        }
    }, 0u64)?);
    match m1054 {
        None => {
            if f347(f348(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1056) => {
                    h1056
                }
            }, f125(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1057) => {
                    h1057
                }
            }, v4.clone()))?, f348(v4.clone(), f125(match v0.clone() {
                Adt16::C0(_, _, _, _, _, h1058) => {
                    h1058
                }
            }, v4.clone()))?, f349(match v0.clone() {
                Adt16::C0(_, _, _, h1059, _, _) => {
                    h1059
                }
            })) {
                let m1063 = f350(f348(match v0.clone() {
                    Adt16::C0(_, _, _, _, h1060, _) => {
                        h1060
                    }
                }, f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, _, h1061, _) => {
                        h1061
                    }
                }))?, f348(v3.clone(), f125(v3.clone(), match v0.clone() {
                    Adt16::C0(_, _, _, _, h1062, _) => {
                        h1062
                    }
                }))?);
                match m1063 {
                    None => {
                        let m1066 = f351(f142(match v0.clone() {
                            Adt16::C0(_, h1065, _, _, _, _) => {
                                h1065
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m1066 {
                            None => {
                                let m1067 = f351(v4, List::<Bytes>::nil());
                                match m1067 {
                                    None => {
                                        let m1069 = f352(v1, match v0 {
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f346(f125(v3, match v0 {
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
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f346(match v1 {
                Adt15::C0(_, _, _, _, h1055) => {
                    h1055
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f239(v0: List<Bytes>) -> List<Bytes> {
    f120(Fn1::F353, List::<Bytes>::nil(), v0)
}

pub fn f240(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f354(v0, v1)
}

pub fn f241(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f355(f356(f357(v0))?))
}

pub fn f242(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f243(v0: Adt1, v1: List<(List<Bytes>, List<Bytes>)>, v2: Bytes) -> R<bool> {
    let m1070 = f57(v0, v2)?;
    match m1070 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f358(v1, f58(v3.clone(), f8(Str::lit("id"))))?;
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
                            Ok(f51(f52(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f359(f360(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f244(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1074 = v0;
    match m1074.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            if f361(v1.clone(), f8(Str::lit("id = \"")), 0u64, 6u64)? {
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
                        f244(v2)
                    }
                    Some((v3, v4)) => {
                        let m1079 = v4;
                        match m1079.uncons() {
                            None => {
                                f244(v2)
                            }
                            Some((_, _)) => {
                                Ok(List::cons(v3, f244(v2)?))
                            }
                        }
                    }
                }
            } else {
                f244(v2)
            }
        }
    }
}

pub fn f245(p1082: List<Bytes>, p1083: List<Bytes>) -> List<Bytes> {
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

pub fn f246() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f247(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1084 = v1;
    match m1084.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt2 = f362(v0.clone(), v3)?;
            if f52(f363(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f364(v5)?, f247(v0, v4, v2)?))
            } else {
                f247(v0, v4, v2)
            }
        }
    }
}

pub fn f248(v0: Adt1, p1088: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
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
                        let v5: Adt2 = f362(v0.clone(), v3.clone())?;
                        if if f52(f363(f58(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
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
                            })?, f64(v0.clone(), f364(v5)?))
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

pub fn f249(v0: Adt1, v1: List<Bytes>) -> R<Adt12> {
    if f51(f138(match v0.clone() {
        Adt1::C0(_, h1089, _, _) => {
            h1089
        }
    }, f277())) {
        Ok(Adt12::C1(List::cons(f277(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f151(f145(f365(f147(match v0.clone() {
            Adt1::C0(_, h1090, _, _) => {
                h1090
            }
        }, f277())))?, v1.clone());
        let m1091 = f56(v0.clone(), List::cons(f153(), List::cons(f213(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m1091.uncons() {
            None => {
                let m1092 = f56(v0.clone(), f366(v0.clone(), v2.clone())?);
                match m1092.uncons() {
                    None => {
                        let v7: (bool, Bytes) = f367(v0.clone(), v2, v1)?;
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
                                            }, List::cons(f7(f145(f368(f127(v0.clone())?, f135(v0.clone(), f136(v0.clone())?)?))?), List::cons(f369(v0)?, List::<Bytes>::nil()))))))
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

pub fn f250(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f370(f96(v0, f153())?)?)))
}

pub fn f251() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# The one way an agent changes this repository: dispatch this workflow with an\u{a}")), List::cons(f8(Str::lit("# operation of the API. It runs the certified command, and opens a pull request\u{a}")), List::cons(f8(Str::lit("# as the Verdant Studio GitHub App, which merges it once the studio check passes.\u{a}")), List::cons(f8(Str::lit("name: operate\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}")), List::cons(f8(Str::lit("# that defers to an open advance pull request always has a successor when that merges.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("    inputs:\u{a}")), List::cons(f8(Str::lit("      args:\u{a}")), List::cons(f8(Str::lit("        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}")), List::cons(f8(Str::lit("        required: true\u{a}")), List::cons(f8(Str::lit("        default: 'advance'\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input:\u{a}")), List::cons(f8(Str::lit("        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("      input_name:\u{a}")), List::cons(f8(Str::lit("        description: 'File name for input'\u{a}")), List::cons(f8(Str::lit("        required: false\u{a}")), List::cons(f8(Str::lit("        type: string\u{a}")), List::cons(f8(Str::lit("        default: 'input'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# Only operations that write the registers share one group. GitHub keeps one\u{a}")), List::cons(f8(Str::lit("# pending run per group, so a staging or propose-command run gets its own.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  # The wait is its own job under its own limit, so time spent waiting never comes out of the 30 minutes the operation has.\u{a}  wait:\u{a}    if: ${{ !startsWith(inputs.args, 'revert-merge ') }}\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 12\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      # Main moves about every 10 minutes, and refresh's regeneration of an API change plus its check takes\u{a}")), List::cons(f8(Str::lit("      # about 12, so an operation merged in that window pushes the change behind main again (#309, #315).\u{a}")), List::cons(f8(Str::lit("      # refresh keeps a regenerated change ready with its auto-merge set, so this run waits for every such\u{a}")), List::cons(f8(Str::lit("      # change whose studio check has not failed to merge first, then starts from the new main. It waits at\u{a}")), List::cons(f8(Str::lit("      # most 10 minutes, so neither queue stalls. A new propose-command waits too, so it never merges ahead of an\u{a}")), List::cons(f8(Str::lit("      # older change that is being regenerated; revert-merge, the repair of a broken path, never waits.\u{a}")), List::cons(f8(Str::lit("      - name: wait for the API changes that are next to merge (every operation but revert-merge)\u{a}")), List::cons(f8(Str::lit("")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          for try in $(seq 1 20); do\u{a}")), List::cons(f8(Str::lit("            next=''\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio propose-command\")) and .auto_merge != null and (.draft | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then next=\"$next #${pr}\"; fi\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f252() -> Bytes {
    f7(List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -z \"$next\" ]; then exit 0; fi\u{a}")), List::cons(f8(Str::lit("            echo \"waiting for${next} to merge (check $try of 20)\"\u{a}")), List::cons(f8(Str::lit("            sleep 30\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          echo \"::warning::${next# } still open after 10 minutes, so this run goes on\"\u{a}  operate:\u{a}    needs: wait\u{a}    if: ${{ !cancelled() && (needs.wait.result == 'success' || needs.wait.result == 'skipped') }}\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}")), List::cons(f8(Str::lit("        if: inputs.args == 'propose-command'\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          mkdir -p target/input\u{a}")), List::cons(f8(Str::lit("          cat > target/input/change.patch <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}")), List::cons(f8(Str::lit("          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}")), List::cons(f8(Str::lit("          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' -e '^readers/' -e '^admit/' -e '^lexlean-workspace.toml$' -e '^studio/lexlean.toml$' -e '^studio/production-request.json$' -e '^app/lexlean.toml$' -e '^stdlib/lexlean.toml$' -e '^.github/lexlean.pin$' || true)\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites, README.md, readers, admit, lexlean-workspace.toml, the studio, app and stdlib lexlean.toml, studio/production-request.json and .github/lexlean.pin, not: $outside\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          # studio/lexlean.lock, readers/lexlean.lock, readers/xml/lexlean.lock and admit/lexlean.lock are generated, and the export below writes them again.\u{a}")), List::cons(f8(Str::lit("          # A VS ID this change adds that main already has becomes the next free one, and a pure append is\u{a}          # appended on main instead of patched (prepare-change.py, which refresh runs too).\u{a}          python3 .github/scripts/prepare-change.py prepare target/input/change.patch target/input/appends.json\u{a}          # git apply drops a file whose binary patch is corrupt, says so on one error line and still exits 0, so any error line stops the run here, before the export.\u{a}          # Contract and strata keep their entries on one line, so a change that only adds entries there is merged\u{a}          # into main by entry, and never conflicts with another change that adds others (merge-entries.py).\u{a}          python3 .github/scripts/merge-entries.py patch target/input/change.patch\u{a}          : > target/apply.err\u{a}          if grep -q '^diff --git' target/input/change.patch; then git apply --exclude=studio/lexlean.lock --exclude=readers/lexlean.lock --exclude=readers/xml/lexlean.lock --exclude=admit/lexlean.lock target/input/change.patch 2> target/apply.err || { sed 's/^/::error::/' target/apply.err; exit 1; }; fi\u{a}          if grep -q '^error' target/apply.err; then sed 's/^/::error::the patch did not apply whole: /' target/apply.err; exit 1; fi\u{a}          python3 .github/scripts/prepare-change.py append target/input/appends.json\u{a}")), List::cons(f8(Str::lit("          # The owner reads this pull request, so its body leads with the proposer's own account (the prose\u{a}")), List::cons(f8(Str::lit("          # before the first diff), then the definitions it changes. Checked before the export, which takes minutes.\u{a}")), List::cons(f8(Str::lit("          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}")), List::cons(f8(Str::lit("          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}")), List::cons(f8(Str::lit("            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The export refuses Lean output holding an absolute path (LLV7006) only at its end, so refuse here, in seconds,\u{a}")), List::cons(f8(Str::lit("          # an added string literal that LexLean's own test would refuse, read as Lean prints it (prepare-change.py lint).\u{a}")), List::cons(f8(Str::lit("          python3 .github/scripts/prepare-change.py lint target/input/change.patch\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          # The one export, shared with refresh and run end to end by the studio check.\u{a}")), List::cons(f8(Str::lit("          if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f371(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("            tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}")), List::cons(f8(Str::lit("          {\u{a}")), List::cons(f8(Str::lit("            cat target/summary.md\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"### Definitions this adds or changes\"\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f253() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            {\u{a}")), List::cons(f8(Str::lit("              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}")), List::cons(f8(Str::lit("              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio readers/src readers/xml/src admit/src | grep '^[+]  name: ' || true; } | cut -c2-\u{a}")), List::cons(f8(Str::lit("            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            slash=/\u{a}")), List::cons(f8(Str::lit("            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"Lean phases by wall time in seconds, longest first (the export fails at 240 of the 300 s limit):\"\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            head -n 8 target/export.log.phases || true\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml\u{a}")), List::cons(f8(Str::lit("            echo '```'\u{a}")), List::cons(f8(Str::lit("            echo\u{a}")), List::cons(f8(Str::lit("            echo \"<${slash}details>\"\u{a}")), List::cons(f8(Str::lit("          } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("      - id: command\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: |\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_readers\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_xml\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_admit\u{a}")), List::cons(f8(Str::lit("          key: verdant-studio-${{ hashFiles('certified/**', 'certified-readers/**', 'certified-xml/**', 'certified-admit/**') }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}")), List::cons(f8(Str::lit("        if: steps.command.outputs.cache-hit != 'true'\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          target/lexlean/bin/lexlean production verify-artifact certified\u{a}")), List::cons(f8(Str::lit("          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("          # The reader operations are a second LexLean project with its own bundle, so that each export stays under its limits.\u{a}")), List::cons(f8(Str::lit("          if [ -d certified-readers ]; then\u{a}")), List::cons(f8(Str::lit("            target/lexlean/bin/lexlean production verify-artifact certified-readers\u{a}")), List::cons(f8(Str::lit("            cargo build --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          # The XML readers are a third project with their own bundle, for the same reason.\u{a}          if [ -d certified-xml ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-xml\u{a}            cargo build --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock\u{a}          fi\u{a}          # The admission operations are a fourth project with their own bundle, for the same reason.\u{a}          if [ -d certified-admit ]; then\u{a}            target/lexlean/bin/lexlean production verify-artifact certified-admit\u{a}            cargo build --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock\u{a}          fi\u{a}")), List::cons(f8(Str::lit("      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("          INPUT: ${{ inputs.input }}\u{a}")), List::cons(f8(Str::lit("          INPUT_NAME: ${{ inputs.input_name }}\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail -f\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          # read-* operations run in their reader project's command when it is built (the XML ones in verdant_xml), the admission operations in verdant_admit, every other in the studio command.\u{a}")), List::cons(f8(Str::lit("          vs() { case \"$1\" in admit|admit-encoding|admit-registry|admit-translated|propose-encoding) if [ -x target/release/verdant_admit ]; then target/release/verdant_admit \"$@\"; return; fi;; read-xml|read-xsd|read-xsd-components) if [ -x target/release/verdant_xml ]; then target/release/verdant_xml \"$@\"; return; fi;; read-*) if [ -x target/release/verdant_readers ]; then target/release/verdant_readers \"$@\"; return; fi;; esac; target/release/verdant_studio \"$@\"; }\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f254() -> Bytes {
    f7(List::cons(f8(Str::lit("          # An App pull request whose files already hold on main has nothing left to merge,\u{a}")), List::cons(f8(Str::lit("          # so it is closed; the propose-command ones change the API and stay open until they merge.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}")), List::cons(f8(Str::lit("            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}")), List::cons(f8(Str::lit("              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}")), List::cons(f8(Str::lit("              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}")), List::cons(f8(Str::lit("              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}")), List::cons(f8(Str::lit("                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}")), List::cons(f8(Str::lit("                exit 0\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then\u{a}")), List::cons(f8(Str::lit("            # A change to a reader re-derives the encodings it printed before the studio check replays them.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then\u{a}")), List::cons(f8(Str::lit("              readmitted=$(bash .github/scripts/readmit.sh target/release/verdant_readers; if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi)\u{a}")), List::cons(f8(Str::lit("              if [ -n \"$readmitted\" ]; then\u{a}")), List::cons(f8(Str::lit("                { echo; echo \"### Encodings admitted again\"; echo; echo \"This change alters what a reader prints for these admitted encodings, so each was admitted again through read-admit with the command and citation its row records:\"; echo; sed 's/^/- /' <<< \"$readmitted\"; } >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            cat target/operation.txt\u{a}")), List::cons(f8(Str::lit("          elif [ \"$ARGS\" = advance ]; then\u{a}")), List::cons(f8(Str::lit("            : > target/operation.txt\u{a}")), List::cons(f8(Str::lit("            last=''\u{a}")), List::cons(f8(Str::lit("            skipped=''\u{a}")), List::cons(f8(Str::lit("            # next prints one operation it can dispatch, or else the lines that say why the\u{a}")), List::cons(f8(Str::lit("            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}")), List::cons(f8(Str::lit("            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}")), List::cons(f8(Str::lit("              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("              if vs $op > target/attempt.txt 2>&1; then\u{a}")), List::cons(f8(Str::lit("                tee -a target/operation.txt < target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                last=$op\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                cat target/attempt.txt\u{a}")), List::cons(f8(Str::lit("                binding=${op#* }\u{a}")), List::cons(f8(Str::lit("                echo \"::warning::$op was refused; the other bindings continue\"\u{a}")), List::cons(f8(Str::lit("                echo \"refused: $op\" >> target/operation.txt\u{a}")), List::cons(f8(Str::lit("                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}")), List::cons(f8(Str::lit("                skipped=\"$skipped $binding\"\u{a}")), List::cons(f8(Str::lit("                # The refusal becomes a Studio.Error.Observation in the model (REQ-SELF-HEALING).\u{a}")), List::cons(f8(Str::lit("                vs record-error target/attempt.txt \"$(date -u -Iseconds)\" \"${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}\" $op | tee -a target/operation.txt\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            t0=$EPOCHREALTIME\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}")), List::cons(f8(Str::lit("            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}")), List::cons(f8(Str::lit("            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}")), List::cons(f8(Str::lit("              echo \"waiting merge: #${pr} changes the API, and merges once the studio check passes\" >> target/waiting.txt\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}")), List::cons(f8(Str::lit("          elif [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f255() -> Bytes {
    f7(List::cons(f8(Str::lit("            # The repair of a merge that broke the API's own path. Dispatched from a branch at the last\u{a}")), List::cons(f8(Str::lit("            # good main commit, it reverts the merge on main as the App, and merges once the studio check passes.\u{a}")), List::cons(f8(Str::lit("            merge=${ARGS#revert-merge }\u{a}")), List::cons(f8(Str::lit("            case \"$merge\" in *[!0-9a-f]*|'') echo \"::error::revert-merge takes the full hex id of a merge commit on main\"; exit 1;; esac\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B revert origin/main\u{a}")), List::cons(f8(Str::lit("            if ! git merge-base --is-ancestor \"$merge\" origin/main; then echo \"::error::$merge is not on main\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("            git revert -m 1 --no-commit \"$merge\"\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            { echo \"Revert $merge, a merge that broke the API's own path\"; echo; echo \"**What changes and why.** This reverts the merge $merge on main $(git rev-parse --short origin/main), made by the App from the last good main commit, so the repair needs no bypass.\"; echo; echo '```'; git diff --cached --stat -- studio/src app/src model features README.md .github; echo '```'; } > target/operation.txt\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::cons(f8(Str::lit("            vs $ARGS | tee target/operation.txt\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("          target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("      - name: open a pull request as the Verdant Studio app\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          ARGS: ${{ inputs.args || 'advance' }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}")), List::cons(f8(Str::lit("          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}")), List::cons(f8(Str::lit("          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}")), List::cons(f8(Str::lit("          git add -A\u{a}")), List::cons(f8(Str::lit("          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}")), List::cons(f8(Str::lit("          verdant_studio $ARGS\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("          $(cat target/operation.txt)\u{a}")), List::cons(f8(Str::lit("          MSG\u{a}")), List::cons(f8(Str::lit("          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}")), List::cons(f8(Str::lit("          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}")), List::cons(f8(Str::lit("            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}")), List::cons(f8(Str::lit("            exit 1\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          title=\"verdant_studio $ARGS\"\u{a}")), List::cons(f8(Str::lit("          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}")), List::cons(f8(Str::lit("          if [ \"${ARGS#revert-merge }\" != \"$ARGS\" ]; then title=\"verdant_studio propose-command: Revert ${ARGS#revert-merge }, a merge that broke the API's own path\"; fi\u{a}")), List::cons(f8(Str::lit("          # REST, not GraphQL: run 45646779790 lost its pull request to four GraphQL errors.\u{a}")), List::cons(f8(Str::lit("          # Opening is idempotent: run 45649684941 opened #263 although its call reported an error, and the retry then failed on \"already exists\".\u{a}")), List::cons(f8(Str::lit("          # So each try first looks for an open pull request from this branch, and an existing one counts as opened.\u{a}")), List::cons(f8(Str::lit("          pr=\"\"\u{a}")), List::cons(f8(Str::lit("          for try in 1 2 3 4; do\u{a}")), List::cons(f8(Str::lit("            pr=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&head=${GITHUB_REPOSITORY_OWNER}:${branch}\" --jq '.[0].number // empty')\u{a}")), List::cons(f8(Str::lit("            if [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            if pr=$(command gh api \"repos/${GITHUB_REPOSITORY}/pulls\" -f head=\"$branch\" -f base=main -f title=\"$title\" -F body=@target/operation.txt --jq .number) && [ -n \"$pr\" ]; then break; fi\u{a}")), List::cons(f8(Str::lit("            pr=\"\"\u{a}")), List::cons(f8(Str::lit("            echo \"::warning::opening the pull request failed (try $try of 4)\"\u{a}")), List::cons(f8(Str::lit("            sleep $((2 ** try))\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -z \"$pr\" ]; then echo \"::error::no pull request could be opened for $branch\"; exit 1; fi\u{a}")), List::cons(f8(Str::lit("          echo \"opened #${pr}\"\u{a}")), List::cons(f8(Str::lit("          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}")), List::cons(f8(Str::lit("          # A change to the API itself (propose-command, revert-merge) merges as every App pull request does:\u{a}")), List::cons(f8(Str::lit("          # the owner automated these approvals (2026-11-26), so the studio check, which runs the API's own\u{a}")), List::cons(f8(Str::lit("          # end-to-end propose run, is its gate. refresh keeps its auto-merge while it regenerates it.\u{a}")), List::cons(f8(Str::lit("          # An API change merges before every change opened after it, however long each check takes: #478 and\u{a}")), List::cons(f8(Str::lit("          # #479 merged ahead of #474 after the wait above gave up, and each pushed it behind main again.\u{a}")), List::cons(f8(Str::lit("          # So while an older propose-command pull request keeps its auto-merge and its studio check has not\u{a}")), List::cons(f8(Str::lit("          # failed, this one is left ready without auto-merge, and refresh sets it once that one merges.\u{a}")), List::cons(f8(Str::lit("          # revert-merge, the repair of a broken path, never waits.\u{a}")), List::cons(f8(Str::lit("          ahead=''\u{a}")), List::cons(f8(Str::lit("          if [ \"${ARGS#revert-merge }\" = \"$ARGS\" ]; then\u{a}")), List::cons(f8(Str::lit("            for older in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select(.user.type == \\\"Bot\\\" and (.title | startswith(\\\"verdant_studio propose-command\\\")) and .auto_merge != null and .number < ${pr}) | .number\"); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${older}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then ahead=\"$ahead #${older}\"; fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}")), List::cons(f8(Str::lit("          if [ \"$gate\" = merge ] && [ -n \"$ahead\" ]; then\u{a}")), List::cons(f8(Str::lit("            echo \"::notice::#${pr} merges after${ahead}: refresh sets its auto-merge once they merge\"\u{a}")), List::cons(f8(Str::lit("          elif [ \"$gate\" = merge ]; then\u{a}")), List::cons(f8(Str::lit("            gh pr merge \"$pr\" --auto --squash --delete-branch\u{a}")), List::cons(f8(Str::lit("          else\u{a}")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f256() -> Bytes {
    f7(List::cons(f8(Str::lit("            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::<Bytes>::nil())))
}

pub fn f257() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# When main moves, every open propose-command pull request that no longer merges\u{a}")), List::cons(f8(Str::lit("# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}")), List::cons(f8(Str::lit("# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}")), List::cons(f8(Str::lit("# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}")), List::cons(f8(Str::lit("# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}")), List::cons(f8(Str::lit("# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}")), List::cons(f8(Str::lit("# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("# The exception is one whose auto-merge is set, the next change to merge: it stays ready and keeps its\u{a}")), List::cons(f8(Str::lit("# auto-merge, so it merges as soon as its regeneration passes the studio check. It is\u{a}")), List::cons(f8(Str::lit("# regenerated first, and only the oldest such one at a time, since two changes to the\u{a}")), List::cons(f8(Str::lit("# API conflict on their export (#309 and #315 were pushed behind main this way).\u{a}")), List::cons(f8(Str::lit("name: refresh\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A pull request that opens while main moves is caught too: this workflow also starts when an\u{a}")), List::cons(f8(Str::lit("# operate run completes, which needs no token that may dispatch workflows, so every\u{a}")), List::cons(f8(Str::lit("# regeneration takes this one path. A run that finds nothing to regenerate ends at once.\u{a}")), List::cons(f8(Str::lit("on:\u{a}")), List::cons(f8(Str::lit("  push:\u{a}")), List::cons(f8(Str::lit("    branches: [main]\u{a}")), List::cons(f8(Str::lit("  workflow_run:\u{a}")), List::cons(f8(Str::lit("    workflows: [operate]\u{a}")), List::cons(f8(Str::lit("    types: [completed]\u{a}")), List::cons(f8(Str::lit("  workflow_dispatch:\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("permissions:\u{a}")), List::cons(f8(Str::lit("  contents: read\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("# A running refresh always finishes, because an export takes minutes and main moves more\u{a}")), List::cons(f8(Str::lit("# often than that; GitHub keeps only the newest pending run behind it.\u{a}")), List::cons(f8(Str::lit("concurrency:\u{a}")), List::cons(f8(Str::lit("  group: refresh\u{a}")), List::cons(f8(Str::lit("  cancel-in-progress: false\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("jobs:\u{a}")), List::cons(f8(Str::lit("  find:\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 15\u{a}")), List::cons(f8(Str::lit("    outputs:\u{a}")), List::cons(f8(Str::lit("      stale: ${{ steps.stale.outputs.stale }}\u{a}")), List::cons(f8(Str::lit("      matrix: ${{ steps.stale.outputs.matrix }}\u{a}")), List::cons(f8(Str::lit("      auto: ${{ steps.stale.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - id: stale\u{a}")), List::cons(f8(Str::lit("        name: find the propose-command pull requests that conflict with main\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          stale=''\u{a}")), List::cons(f8(Str::lit("          auto=''\u{a}")), List::cons(f8(Str::lit("          # Oldest first, so the oldest one whose auto-merge is set is the one regenerated now.\u{a}")), List::cons(f8(Str::lit("          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}")), List::cons(f8(Str::lit("            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}")), List::cons(f8(Str::lit("            # which a cancelled run may have left behind.\u{a}")), List::cons(f8(Str::lit("            mergeable=null\u{a}")), List::cons(f8(Str::lit("            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}")), List::cons(f8(Str::lit("              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f258() -> Bytes {
    f7(List::cons(f8(Str::lit("              if [ \"$mergeable\" != null ]; then break; fi\u{a}")), List::cons(f8(Str::lit("              sleep 10\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            # One that changes .github is exported again by the studio check, byte for byte, so it also fails\u{a}")), List::cons(f8(Str::lit("            # when main changed its export inputs since it was exported, even without a conflict: readers\u{a}")), List::cons(f8(Str::lit("            # depends on app, so admissions merged during its export (#357 and #361 under #362) turn it red.\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = true ] && [ \"$(gh api --paginate \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files\" --jq '.[].filename' | grep -c '^\\.github/' || true)\" != 0 ]; then\u{a}")), List::cons(f8(Str::lit("              base=$(gh api \"repos/${GITHUB_REPOSITORY}/compare/main...$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\" --jq .merge_base_commit.sha)\u{a}")), List::cons(f8(Str::lit("              moved=$(gh api \"repos/${GITHUB_REPOSITORY}/compare/${base}...main\" --jq '[.files[]?.filename | select(test(\"^(app|stdlib|studio|readers)/\"))] | length')\u{a}")), List::cons(f8(Str::lit("              if [ \"$moved\" != 0 ]; then echo \"#${pr}: main changed ${moved} of its export inputs since it was exported\"; mergeable=false; fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            if [ \"$mergeable\" = false ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              # Drafting would drop its auto-merge, so it stays ready. A later one waits: it is\u{a}")), List::cons(f8(Str::lit("              # regenerated once this one merges, when it would conflict again anyway.\u{a}")), List::cons(f8(Str::lit("              if [ -z \"$auto\" ]; then auto=$pr; else echo \"#${pr} waits for #${auto} to merge\"; fi\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = false ]; then\u{a}")), List::cons(f8(Str::lit("              stale=\"$stale $pr\"\u{a}")), List::cons(f8(Str::lit("              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}")), List::cons(f8(Str::lit("              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}")), List::cons(f8(Str::lit("              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          # A change operate left without auto-merge, behind an older API change, gets it here once no\u{a}")), List::cons(f8(Str::lit("          # propose-command pull request keeps its auto-merge with a studio check that has not failed: oldest\u{a}")), List::cons(f8(Str::lit("          # first, up to and including the first API change, so API changes still merge one at a time in order.\u{a}")), List::cons(f8(Str::lit("          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > \"$RUNNER_TEMP/rules.json\"\u{a}")), List::cons(f8(Str::lit("          if [ \"$(jq '[.[] | select(.type == \"required_status_checks\") | .parameters.required_status_checks[]?.context] | index(\"studio\") != null' \"$RUNNER_TEMP/rules.json\")\" = true ]; then\u{a}")), List::cons(f8(Str::lit("            armed=''\u{a}")), List::cons(f8(Str::lit("            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select(.user.login == \\\"${SLUG}[bot]\\\" and (.title | startswith(\\\"verdant_studio propose-command\\\")) and .auto_merge != null) | .number\"); do\u{a}")), List::cons(f8(Str::lit("              sha=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.sha)\u{a}")), List::cons(f8(Str::lit("              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/commits/${sha}/check-runs?check_name=studio\" --jq '[.check_runs[] | select(.conclusion == \"failure\")] | length')\" = 0 ]; then armed=\"$armed #${pr}\"; fi\u{a}")), List::cons(f8(Str::lit("            done\u{a}")), List::cons(f8(Str::lit("            if [ -z \"$armed\" ]; then\u{a}")), List::cons(f8(Str::lit("              for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \"sort_by(.number) | .[] | select(.user.login == \\\"${SLUG}[bot]\\\" and .auto_merge == null and (.draft | not)) | .number\"); do\u{a}")), List::cons(f8(Str::lit("                if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\" = true ]; then\u{a}")), List::cons(f8(Str::lit("                  gh pr merge \"$pr\" --auto --squash --delete-branch --repo \"$GITHUB_REPOSITORY\" || true\u{a}")), List::cons(f8(Str::lit("                  echo \"#${pr}: auto-merge set\"\u{a}")), List::cons(f8(Str::lit("                  if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.title | startswith(\"verdant_studio propose-command\")')\" = true ]; then break; fi\u{a}")), List::cons(f8(Str::lit("                fi\u{a}")), List::cons(f8(Str::lit("              done\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$auto\" ]; then stale=\" $auto$stale\"; fi\u{a}")), List::cons(f8(Str::lit("          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"auto=$auto\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}")), List::cons(f8(Str::lit("          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("  regenerate:\u{a}")), List::cons(f8(Str::lit("    needs: find\u{a}")), List::cons(f8(Str::lit("    if: needs.find.outputs.stale != ''\u{a}")), List::cons(f8(Str::lit("    runs-on: ubuntu-24.04\u{a}")), List::cons(f8(Str::lit("    timeout-minutes: 60\u{a}")), List::cons(f8(Str::lit("    strategy:\u{a}")), List::cons(f8(Str::lit("      fail-fast: false\u{a}")), List::cons(f8(Str::lit("      matrix:\u{a}")), List::cons(f8(Str::lit("        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}")), List::cons(f8(Str::lit("    steps:\u{a}")), List::cons(f8(Str::lit("      - id: app\u{a}")), List::cons(f8(Str::lit("        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}")), List::cons(f8(Str::lit("          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}")), List::cons(f8(Str::lit("      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          token: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          fetch-depth: 0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          node-version: '24'\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.elan\u{a}")), List::cons(f8(Str::lit("          key: lean-4.32.2-elan-4.1.2\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: target/lexlean\u{a}")), List::cons(f8(Str::lit("          key: lexlean-${{ hashFiles('.github/lexlean.pin') }}-rust-1.97.0\u{a}")), List::cons(f8(Str::lit("      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}")), List::cons(f8(Str::lit("        with:\u{a}")), List::cons(f8(Str::lit("          path: ~/.npm\u{a}")), List::cons(f8(Str::lit("          key: npm-prism-code-0.11.1\u{a}")), List::cons(f8(Str::lit("      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}")), List::cons(f8(Str::lit("        env:\u{a}")), List::cons(f8(Str::lit("          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}")), List::cons(f8(Str::lit("          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}")), List::cons(f8(Str::lit("          STALE: ${{ matrix.pr }}\u{a}")), List::cons(f8(Str::lit("          AUTO: ${{ needs.find.outputs.auto }}\u{a}")), List::cons(f8(Str::lit("        run: |\u{a}")), List::cons(f8(Str::lit("          set -euo pipefail\u{a}")), List::cons(f8(Str::lit("          # A regeneration of the one whose auto-merge is set that fails leaves it a draft, which drops\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f259() -> Bytes {
    f7(List::cons(f8(Str::lit("          # its auto-merge, so it is no longer the next change to merge and nothing is handed over red.\u{a}")), List::cons(f8(Str::lit("          trap 'rc=$?; if [ \"$rc\" != 0 ] && [ \"$STALE\" = \"$AUTO\" ]; then gh pr ready --undo \"$STALE\" || true; fi' EXIT\u{a}")), List::cons(f8(Str::lit("          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}")), List::cons(f8(Str::lit("            mkdir -p target\u{a}")), List::cons(f8(Str::lit("            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}")), List::cons(f8(Str::lit("            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}")), List::cons(f8(Str::lit("            tar -xzf target/elan.tgz -C target\u{a}")), List::cons(f8(Str::lit("            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          fi\u{a}")), List::cons(f8(Str::lit("          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}")), List::cons(f8(Str::lit("          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}")), List::cons(f8(Str::lit("          cmp -s .github/lexlean.pin target/lexlean/pin || { rm -rf target/lexlean; rustup toolchain install 1.97.0 --profile minimal; RUSTFLAGS=\"--remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=cargo-home\" cargo +1.97.0 install --locked $(cat .github/lexlean.pin) lexlean --root target/lexlean; cp .github/lexlean.pin target/lexlean/pin; }\u{a}")), List::cons(f8(Str::lit("          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}")), List::cons(f8(Str::lit("          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}")), List::cons(f8(Str::lit("          failed=''\u{a}")), List::cons(f8(Str::lit("          for pr in $STALE; do\u{a}")), List::cons(f8(Str::lit("            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}")), List::cons(f8(Str::lit("            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}")), List::cons(f8(Str::lit("            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}")), List::cons(f8(Str::lit("            # The one whose auto-merge is set stays ready, so that it keeps its auto-merge.\u{a}")), List::cons(f8(Str::lit("            if [ \"$pr\" != \"$AUTO\" ]; then gh pr ready --undo \"$pr\" || true; fi\u{a}")), List::cons(f8(Str::lit("            git fetch -q origin \"$branch\" main\u{a}")), List::cons(f8(Str::lit("            git checkout -q -B refresh origin/main\u{a}")), List::cons(f8(Str::lit("            git clean -fdq\u{a}")), List::cons(f8(Str::lit("            # readers/lexlean.lock and readers/xml/lexlean.lock are generated: the export below writes them again, so they are left out of the\u{a}")), List::cons(f8(Str::lit("            # change, and two reader pull requests never conflict on it.\u{a}")), List::cons(f8(Str::lit("            git diff --full-index \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml ':!readers/lexlean.lock' ':!readers/xml/lexlean.lock' ':!admit/lexlean.lock' > target/change.patch\u{a}")), List::cons(f8(Str::lit("            # app/src/Contract.lex.tex and model/strata.json each keep their entries on one line, so two changes\u{a}")), List::cons(f8(Str::lit("            # that each add entries there always conflict. A conflict on those two files alone is merged by entry\u{a}")), List::cons(f8(Str::lit("            # (merge-entries.py): main's file, then each entry the pull request adds that main does not have.\u{a}")), List::cons(f8(Str::lit("            # git apply drops a file whose binary patch is corrupt and still exits 0, so that error counts as not applying.\u{a}            # The same path as propose-command: renumber colliding VS IDs, move pure appends aside, and merge\u{a}            # Contract's and strata's entries by entry, so a lane that loses an ID or entry race never dispatches again.\u{a}            python3 .github/scripts/prepare-change.py prepare target/change.patch target/appends.json\u{a}            python3 .github/scripts/merge-entries.py patch target/change.patch\u{a}            applied=yes\u{a}            : > target/apply.err\u{a}            if grep -q '^diff --git' target/change.patch; then git apply --3way target/change.patch 2> target/apply.err || applied=no; fi\u{a}            if grep -q '^error: corrupt' target/apply.err; then applied=no; fi\u{a}            if [ \"$applied\" = no ]; then\u{a}")), List::cons(f8(Str::lit("              merged=no\u{a}")), List::cons(f8(Str::lit("              if [ -n \"$(git diff --name-only --diff-filter=U)\" ] && [ -z \"$(git diff --name-only --diff-filter=U | grep -v -x -e app/src/Contract.lex.tex -e model/strata.json || true)\" ]; then\u{a}")), List::cons(f8(Str::lit("                merged=yes\u{a}")), List::cons(f8(Str::lit("                base=$(git merge-base origin/main \"origin/$branch\")\u{a}")), List::cons(f8(Str::lit("                for f in $(git diff --name-only --diff-filter=U); do\u{a}")), List::cons(f8(Str::lit("                  git show \"$base:$f\" > target/entries-base\u{a}")), List::cons(f8(Str::lit("                  git show \"origin/$branch:$f\" > target/entries-pr\u{a}")), List::cons(f8(Str::lit("                  git show \"origin/main:$f\" > target/entries-main\u{a}")), List::cons(f8(Str::lit("                  if python3 .github/scripts/merge-entries.py merge target/entries-base target/entries-pr target/entries-main \"$f\" > target/entries-merged; then cp target/entries-merged \"$f\"; git add \"$f\"; else merged=no; fi\u{a}")), List::cons(f8(Str::lit("                done\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("              if [ \"$merged\" = yes ]; then\u{a}")), List::cons(f8(Str::lit("                echo \"::notice::#${pr}: merged the entries it adds to Contract and strata into main's by entry\"\u{a}")), List::cons(f8(Str::lit("              else\u{a}")), List::cons(f8(Str::lit("                sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}")), List::cons(f8(Str::lit("                git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("                failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("                # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}")), List::cons(f8(Str::lit("                { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("                gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("                continue\u{a}")), List::cons(f8(Str::lit("              fi\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            python3 .github/scripts/prepare-change.py append target/appends.json\u{a}")), List::cons(f8(Str::lit("            if ! bash .github/scripts/export.sh \"$ll\" target/export.log; then\u{a}")), List::cons(f7(List::cons(f8(Str::lit("              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f371(), List::cons(f8(Str::lit("\"\u{a}")), List::<Bytes>::nil())))), List::cons(f8(Str::lit("              tail -n 40 target/export.log\u{a}")), List::cons(f8(Str::lit("              git reset -q --hard origin/main\u{a}")), List::cons(f8(Str::lit("              failed=\"$failed $pr\"\u{a}")), List::cons(f8(Str::lit("              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}")), List::cons(f8(Str::lit("              gh api --silent \"repos/${GITHUB_REPOSITORY}/issues/${pr}/comments\" -F body=@target/comment.md || true\u{a}")), List::cons(f8(Str::lit("              continue\u{a}")), List::cons(f8(Str::lit("            fi\u{a}")), List::cons(f8(Str::lit("            rm -rf studio/.lexlean/production readers/.lexlean/production readers/xml/.lexlean/production admit/.lexlean/production\u{a}")), List::cons(f8(Str::lit("            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-readers ]; then cargo build -q --release --manifest-path certified-readers/packages/rust-std/verdant_readers/Cargo.toml --target-dir target && rm -f certified-readers/packages/rust-std/verdant_readers/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-xml ]; then cargo build -q --release --manifest-path certified-xml/packages/rust-std/verdant_xml/Cargo.toml --target-dir target && rm -f certified-xml/packages/rust-std/verdant_xml/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            if [ -d certified-admit ]; then cargo build -q --release --manifest-path certified-admit/packages/rust-std/verdant_admit/Cargo.toml --target-dir target && rm -f certified-admit/packages/rust-std/verdant_admit/Cargo.lock; fi\u{a}")), List::cons(f8(Str::lit("            # The regenerated change re-derives the encodings whose reader output it changes, as propose-command does.\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_readers ]; then bash .github/scripts/readmit.sh target/release/verdant_readers; fi\u{a}")), List::cons(f8(Str::lit("            if [ -x target/release/verdant_xml ]; then bash .github/scripts/readmit.sh target/release/verdant_xml; fi\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio check\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio build\u{a}")), List::cons(f8(Str::lit("            target/release/verdant_studio publish\u{a}")), List::cons(f8(Str::lit("            git add -A\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f260() -> Bytes {
    f7(List::cons(f8(Str::lit("            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md readers admit lexlean-workspace.toml; } > target/message.txt\u{a}")), List::cons(f8(Str::lit("            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}")), List::cons(f8(Str::lit("            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}")), List::cons(f8(Str::lit("            gh pr ready \"$pr\" || true\u{a}")), List::cons(f8(Str::lit("            echo \"::notice::regenerated #${pr} on the new main\"\u{a}")), List::cons(f8(Str::lit("          done\u{a}")), List::cons(f8(Str::lit("          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))))))
}

pub fn f261() -> Bytes {
    f7(List::cons(f8(Str::lit("#!/usr/bin/env python3\u{a}")), List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}")), List::cons(f8(Str::lit("# The owner's acceptance test for the whole pipeline (REQ-SELF-HOSTED, REQ-PRISM-GENERATION): prompt Studio\u{a}")), List::cons(f8(Str::lit("# with the seed \"Verdant Studio\" and require every facet the Prism lists to have its media type at the\u{a}")), List::cons(f8(Str::lit("# fidelity its generator's theorems state. Run from the repository root after the build, so the app is not\u{a}")), List::cons(f8(Str::lit("# proved again: prism-acceptance.py COMMAND... runs COMMAND run app --input REQUEST for each request.\u{a}")), List::cons(f8(Str::lit("import math, struct, subprocess, sys, xml.etree.ElementTree as tree\u{a}")), List::cons(f8(Str::lit("SEED = 'Verdant Studio'\u{a}")), List::cons(f8(Str::lit("MIDI_HEADER = bytes.fromhex('4d54686400000006000000010060')\u{a}")), List::cons(f8(Str::lit("FMT = bytes.fromhex('0100010080bb00000077010002001000')\u{a}")), List::cons(f8(Str::lit("# SONY-ASWG-R001 (the AswgLoudnessTarget catalog module), the home-based platform target REQ-ENGINE-FIDELITY's high-end gaming\u{a}")), List::cons(f8(Str::lit("# systems name: -24 (+-2) LKFS integrated, at most -1 dBTP true peak, both measured as ITU-R-BS-1770-5 states.\u{a}")), List::cons(f8(Str::lit("TARGET, TOLERANCE, TRUE_PEAK = -24.0, 2.0, -1.0\u{a}")), List::cons(f8(Str::lit("# BS.1770-5 Annex 1 pre-filter and RLB filter at 48 kHz, as (b0, b1, b2, a1, a2).\u{a}")), List::cons(f8(Str::lit("K_WEIGHTING = ((1.53512485958697, -2.69169618940638, 1.19839281085285, -1.69065929318241, 0.73248077421585),\u{a}")), List::cons(f8(Str::lit("               (1.0, -2.0, 1.0, -1.99004745483398, 0.99007225036621))\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def loudness(x):\u{a}")), List::cons(f8(Str::lit("    # Annex 1: K-weighting, 400 ms blocks overlapping by three quarters, absolute gate -70 LKFS, relative gate -10 LU.\u{a}")), List::cons(f8(Str::lit("    for b0, b1, b2, a1, a2 in K_WEIGHTING:\u{a}")), List::cons(f8(Str::lit("        y, x1, x2, y1, y2 = [], 0.0, 0.0, 0.0, 0.0\u{a}")), List::cons(f8(Str::lit("        for v in list(x):\u{a}")), List::cons(f8(Str::lit("            o = b0 * v + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2\u{a}")), List::cons(f8(Str::lit("            x2, x1, y2, y1 = x1, v, y1, o\u{a}")), List::cons(f8(Str::lit("            y.append(o)\u{a}")), List::cons(f8(Str::lit("        x = y\u{a}")), List::cons(f8(Str::lit("    squares = [0.0]\u{a}")), List::cons(f8(Str::lit("    for v in list(x):\u{a}")), List::cons(f8(Str::lit("        squares.append(squares[-1] + v * v)\u{a}")), List::cons(f8(Str::lit("    size, hop = 19200, 4800\u{a}")), List::cons(f8(Str::lit("    blocks = [(squares[at + size] - squares[at]) * size ** -1 for at in range(0, len(x) - size + 1, hop)]\u{a}")), List::cons(f8(Str::lit("    level = lambda power: -0.691 + 10 * math.log10(power) if power > 0 else -math.inf\u{a}")), List::cons(f8(Str::lit("    gated = [power for power in blocks if level(power) > -70]\u{a}")), List::cons(f8(Str::lit("    if not gated:\u{a}")), List::cons(f8(Str::lit("        return -math.inf\u{a}")), List::cons(f8(Str::lit("    relative = level(sum(gated) * len(gated) ** -1) - 10\u{a}")), List::cons(f8(Str::lit("    gated = [power for power in gated if level(power) > relative]\u{a}")), List::cons(f8(Str::lit("    return level(sum(gated) * len(gated) ** -1)\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def true_peak(x):\u{a}")), List::cons(f8(Str::lit("    # Annex 2: oversample by 4 and take the greatest magnitude, here with a 48-tap Hann-windowed sinc interpolator.\u{a}")), List::cons(f8(Str::lit("    taps = [[(math.sin(math.pi * (k - 24 + phase * 0.25)) * (math.pi * (k - 24 + phase * 0.25)) ** -1 if k - 24 + phase * 0.25 else 1.0)\u{a}")), List::cons(f8(Str::lit("             * (0.5 - 0.5 * math.cos(2 * math.pi * (k + phase * 0.25) * 48 ** -1)) for k in range(48)] for phase in range(1, 4)]\u{a}")), List::cons(f8(Str::lit("    peak = max(map(abs, x), default=0.0)\u{a}")), List::cons(f8(Str::lit("    for at in range(len(x)):\u{a}")), List::cons(f8(Str::lit("        if abs(x[at]) < peak * 0.7:\u{a}")), List::cons(f8(Str::lit("            continue\u{a}")), List::cons(f8(Str::lit("        window = x[max(0, at - 23):at + 25]\u{a}")), List::cons(f8(Str::lit("        for row in taps:\u{a}")), List::cons(f8(Str::lit("            peak = max(peak, abs(sum(c * v for c, v in zip(row[23 - (at - max(0, at - 23)):], window))))\u{a}")), List::cons(f8(Str::lit("    return 20 * math.log10(peak) if peak > 0 else -math.inf\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("SVG_ROOT = '{http:' + chr(47) * 2 + 'www.w3.org' + chr(47) + '2000' + chr(47) + 'svg}svg'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def run(request):\u{a}")), List::cons(f8(Str::lit("    got = subprocess.run(sys.argv[1:] + ['run', 'app', '--input', request, '--rm'], capture_output=True)\u{a}")), List::cons(f8(Str::lit("    if got.returncode != 0:\u{a}")), List::cons(f8(Str::lit("        raise SystemExit('::error::' + repr(request) + ' failed: ' + got.stderr.decode(errors='replace')[-400:])\u{a}")), List::cons(f8(Str::lit("    return got.stdout\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f262() -> Bytes {
    f7(List::cons(f8(Str::lit("def midi(data):\u{a}")), List::cons(f8(Str::lit("    # Midi.lex.tex writes_a_format_0_file: format 0, one track, 96 ticks per quarter note.\u{a}")), List::cons(f8(Str::lit("    if not data.startswith(MIDI_HEADER) or data[14:18] != b'MTrk':\u{a}")), List::cons(f8(Str::lit("        return 'is no format 0 SMF with one track at 96 ticks per quarter note'\u{a}")), List::cons(f8(Str::lit("    if struct.unpack('>I', data[18:22])[0] != len(data) - 22:\u{a}")), List::cons(f8(Str::lit("        return 'has a track length that is not the rest of the file'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def wave(data):\u{a}")), List::cons(f8(Str::lit("    # Bwf.lex.tex format_is_pcm_mono_48000_16 and wave_orders_bext_fmt_data: bext (646 bytes), fmt, data.\u{a}")), List::cons(f8(Str::lit("    if data[:4] != b'RIFF' or data[8:12] != b'WAVE' or struct.unpack('<I', data[4:8])[0] != len(data) - 8:\u{a}")), List::cons(f8(Str::lit("        return 'is no RIFF WAVE file of its own length'\u{a}")), List::cons(f8(Str::lit("    chunks, at = [], 12\u{a}")), List::cons(f8(Str::lit("    while at + 8 <= len(data):\u{a}")), List::cons(f8(Str::lit("        name, size = data[at:at + 4], struct.unpack('<I', data[at + 4:at + 8])[0]\u{a}")), List::cons(f8(Str::lit("        chunks.append((name, data[at + 8:at + 8 + size]))\u{a}")), List::cons(f8(Str::lit("        at += 8 + size + (size & 1)\u{a}")), List::cons(f8(Str::lit("    if [name for name, body in chunks] != [b'bext', b'fmt ', b'data']:\u{a}")), List::cons(f8(Str::lit("        return 'has chunks ' + repr([name for name, body in chunks]) + ', not bext, fmt and data'\u{a}")), List::cons(f8(Str::lit("    if len(chunks[0][1]) != 646:\u{a}")), List::cons(f8(Str::lit("        return 'has a bext chunk of ' + str(len(chunks[0][1])) + ' bytes, not 646'\u{a}")), List::cons(f8(Str::lit("    if chunks[1][1] != FMT:\u{a}")), List::cons(f8(Str::lit("        return 'is not PCM, mono, 48000 Hz, 16-bit'\u{a}")), List::cons(f8(Str::lit("    if not chunks[2][1]:\u{a}")), List::cons(f8(Str::lit("        return 'has no samples'\u{a}")), List::cons(f8(Str::lit("    samples = [value * 2.0 ** -15 for value in struct.unpack('<' + str(len(chunks[2][1]) >> 1) + 'h', chunks[2][1])]\u{a}")), List::cons(f8(Str::lit("    measured, peak = loudness(samples), true_peak(samples)\u{a}")), List::cons(f8(Str::lit("    if abs(measured - TARGET) > TOLERANCE:\u{a}")), List::cons(f8(Str::lit("        return 'reads ' + format(measured, '.1f') + ' LKFS, outside the -24 (+-2) LKFS home-platform target of ASWG-R001'\u{a}")), List::cons(f8(Str::lit("    if peak > TRUE_PEAK:\u{a}")), List::cons(f8(Str::lit("        return 'peaks at ' + format(peak, '.2f') + ' dBTP, above the -1 dBTP of ASWG-R001'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def svg(data):\u{a}")), List::cons(f8(Str::lit("    try:\u{a}")), List::cons(f8(Str::lit("        root = tree.fromstring(data)\u{a}")), List::cons(f8(Str::lit("    except tree.ParseError as failure:\u{a}")), List::cons(f8(Str::lit("        return 'is not well-formed XML: ' + str(failure)\u{a}")), List::cons(f8(Str::lit("    if root.tag != SVG_ROOT:\u{a}")), List::cons(f8(Str::lit("        return 'has the root ' + root.tag + ', not svg in the SVG namespace'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("def bundle(data):\u{a}")), List::cons(f8(Str::lit("    if data[:4] != b'PK' + bytes([3, 4]) or b'bagit.txt' not in data:\u{a}")), List::cons(f8(Str::lit("        return 'is no ZIP holding a BagIt bag'\u{a}")), List::cons(f8(Str::lit("\u{a}")), List::cons(f8(Str::lit("RULES = {'audio/midi': midi, 'audio/wave': wave, 'image/svg+xml': svg, 'application/zip': bundle}\u{a}")), List::cons(f8(Str::lit("facets = [line for line in run('prism' + chr(9) + chr(9) + SEED).decode().split(chr(10)) if line]\u{a}")), List::cons(f8(Str::lit("if not facets:\u{a}")), List::cons(f8(Str::lit("    raise SystemExit('::error::the Prism of ' + SEED + ' lists no facet')\u{a}")), List::cons(f8(Str::lit("failed = False\u{a}")), List::cons(f8(Str::lit("for facet in facets:\u{a}")), List::cons(f8(Str::lit("    data = run('prism' + chr(9) + facet + chr(9) + SEED)\u{a}")), List::cons(f8(Str::lit("    rule = RULES.get(facet)\u{a}")), List::cons(f8(Str::lit("    problem = 'has no acceptance rule, so its fidelity is unchecked' if rule is None else rule(data)\u{a}")), List::cons(f8(Str::lit("    if problem:\u{a}")), List::cons(f8(Str::lit("        failed = True\u{a}")), List::cons(f8(Str::lit("        print('::error::the ' + facet + ' facet of ' + SEED + ' ' + problem)\u{a}")), List::cons(f8(Str::lit("    else:\u{a}")), List::cons(f8(Str::lit("        print('accepted: the ' + facet + ' facet of ' + SEED + ', ' + str(len(data)) + ' bytes')\u{a}")), List::cons(f8(Str::lit("sys.exit(1 if failed else 0)\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f263(v0: List<List<Bytes>>) -> List<List<Bytes>> {
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

pub fn f264(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt19 = f372(v0.clone(), 0u64, {
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
            f373(v1)
        }
    };
    Ok(f374(match v4 {
        Adt19::C0(_, _, _, _, h1106) => {
            h1106
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f265(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt17>, v3: List<Adt2>, v4: List<Bytes>) -> List<Bytes> {
    let m1107 = v1;
    match m1107.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f125(f375(v5.clone(), v2.clone()), f376(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f377(v0.clone(), v5.clone()), List::cons(if f52(v5, v4.clone()) {
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
            }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))), f265(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f266(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1111 = v0;
    match m1111.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1112 = v1;
            match m1112.uncons() {
                None => {
                    f266(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f266(v2))
                }
            }
        }
    }
}

pub fn f267(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1113 = v1;
    match m1113.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f221(v2.clone(), f8(Str::lit("domain"))) {
                f378(v0.clone(), v2.clone())
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
                }, List::cons(f8(Str::lit("** is ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f58(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f379(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
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
                }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))))))))))))), f267(v0, v3))
            } else {
                f267(v0, v3)
            }
        }
    }
}

pub fn f268(v0: List<Bytes>, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1117 = v1;
    match m1117.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            if f221(v2.clone(), f8(Str::lit("subject"))) {
                if f52(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                    List::cons(({
                        let a1118 = f8(Str::lit("a:"));
                        let a1119 = f58(v2.clone(), f8(Str::lit("object")));
                        append_bytes(a1118, a1119)
                    }, f46(f58(v2, f8(Str::lit("subject"))), 0u64, 1u64)), f268(v0, v3))
                } else {
                    f268(v0, v3)
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
                    }, f46(f58(v2, f8(Str::lit("row"))), 0u64, 1u64)), f268(v0, v3))
                } else {
                    f268(v0, v3)
                }
            }
        }
    }
}

pub fn f269(v0: List<List<Bytes>>, v1: u64, v2: u64) -> R<List<Bytes>> {
    let m1126 = v2;
    if m1126 == 0 {
        Ok(List::<Bytes>::nil())
    } else {
        let v3 = m1126 - 1;
        Ok(f125(f380(v0.clone(), f46(f381(), v1, 1u64), f382()?)?, f269(v0, {
            let a1127 = v1;
            let a1128 = 1u64;
            nat_add(a1127, a1128)?
        }, v3)?))
    }
}

pub fn f270(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1129 = v0;
    match m1129.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f383({
                let (h1130, _) = v2;
                h1130
            }, {
                let (_, h1131) = v2;
                h1131
            }, v1.clone())?, f270(v3, v1)?))
        }
    }
}

pub fn f271(v0: Bytes, p1149: u64, p1150: u64, p1151: Bytes) -> R<Bytes> {
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

pub fn f272(v0: List<Adt2>) -> List<Bytes> {
    let m1152 = v0;
    match m1152.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f58(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f58(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f58(v1, f8(Str::lit("statement"))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))))), f272(v2))
        }
    }
}

pub fn f273(v0: List<Adt2>) -> List<Bytes> {
    let m1153 = v0;
    match m1153.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f58(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f384(v1), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f273(v2))
        }
    }
}

pub fn f274(p1159: List<Bytes>) -> R<bool> {
    let m1158 = p1159;
    {
        let mut m1158 = m1158;
        loop {
            let step = {
                let v0 = m1158;
                let m1154 = v0;
                match m1154.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a1156 = {
                            let a1155 = f198(v1)?;
                            length_bytes(a1155)
                        };
                        let a1157 = 0u64;
                        if nat_eq(a1156, a1157) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
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

pub fn f275(v0: List<Bytes>) -> R<bool> {
    if f284(f297(v0.clone())?) {
        Ok(f51({
            let a1161 = {
                let a1160 = f286(f297(v0)?);
                length_list(a1160)
            };
            let a1162 = 0u64;
            nat_eq(a1161, a1162)
        }))
    } else {
        Ok(false)
    }
}

pub fn f276(v0: List<Bytes>) -> R<bool> {
    if f284(f385(v0.clone(), f8(Str::lit("requirement")))?) {
        if f284(f385(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f51({
                let a1164 = {
                    let a1163 = f224(f286(f385(v0.clone(), f8(Str::lit("requirement")))?), f286(f385(v0, f8(Str::lit("admission")))?));
                    length_list(a1163)
                };
                let a1165 = 0u64;
                nat_eq(a1164, a1165)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f277() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f278(v0: Bytes) -> R<Bytes> {
    Ok(f45(f44(v0, 47u8)?))
}

pub fn f279() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f280(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1166 = v0;
    match m1166.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1168 = {
                let a1167 = v3.clone();
                length_bytes(a1167)
            };
            let a1169 = 0u64;
            if nat_eq(a1168, a1169) {
                f280(v2)
            } else {
                let m1170 = f43(v3.clone())?;
                match m1170 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f58(v4.clone(), f8(Str::lit("id")));
                        if f386(f58(v4, f8(Str::lit("schema"))), v5.clone()) {
                            let m1171 = f280(v2)?;
                            match m1171 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f387(v5), {
                                        let a1172 = v3;
                                        let a1173 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1172, a1173)
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

pub fn f281() -> Bytes {
    let a1174 = Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}");
    utf8_encode(a1174)
}

pub fn f282(p1179: List<Bytes>) -> R<Option<Bytes>> {
    let m1178 = p1179;
    {
        let mut m1178 = m1178;
        loop {
            let step = {
                let v0 = m1178;
                let m1175 = v0;
                match m1175.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m1176 = f43(f198(v1)?)?;
                        match m1176 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m1177 = f93(v3.clone(), f8(Str::lit("source")));
                                match m1177 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if f388(f58(v4.clone(), f8(Str::lit("redistribution"))), f58(v4, f8(Str::lit("url"))))? {
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
                Ok(next) => { m1178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(v0: Bytes) -> Bytes {
    let a1184 = {
        let a1180 = Str::lit("binding ");
        utf8_encode(a1180)
    };
    let a1185 = {
        let a1182 = v0;
        let a1183 = {
            let a1181 = Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}");
            utf8_encode(a1181)
        };
        append_bytes(a1182, a1183)
    };
    append_bytes(a1184, a1185)
}

pub fn f284(v0: Option<List<Adt2>>) -> bool {
    let m1186 = v0;
    match m1186 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f285(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1187 = v0;
    match m1187.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1189 = {
                let a1188 = v3.clone();
                length_bytes(a1188)
            };
            let a1190 = 0u64;
            if nat_eq(a1189, a1190) {
                f285(v2)
            } else {
                let m1191 = f43(v3)?;
                match m1191 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v4) => {
                        if f117(f117(f221(v4.clone(), f8(Str::lit("authority"))), f221(v4.clone(), f8(Str::lit("row")))), f117(f117(f221(v4.clone(), f8(Str::lit("construct"))), f221(v4.clone(), f8(Str::lit("artifact")))), f389(f58(v4.clone(), f8(Str::lit("reader"))), f58(v4.clone(), f8(Str::lit("construct")))))) {
                            let m1192 = f285(v2)?;
                            match m1192 {
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

pub fn f286(v0: Option<List<Adt2>>) -> List<Adt2> {
    let m1193 = v0;
    match m1193 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f287(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m1194 = v0;
    match m1194.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1196 = {
                let a1195 = v3.clone();
                length_bytes(a1195)
            };
            let a1197 = 0u64;
            if nat_eq(a1196, a1197) {
                f287(v2)
            } else {
                let m1198 = f43(v3.clone())?;
                match m1198 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f390(v4.clone()) {
                            let m1199 = f287(v2)?;
                            match m1199 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f246(), List::cons(f8(Str::lit("/")), List::cons(f391(f58(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a1200 = v3;
                                        let a1201 = f8(Str::lit("\u{a}"));
                                        append_bytes(a1200, a1201)
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

pub fn f288() -> Bytes {
    f392()
}

pub fn f289(v0: Adt1, v1: Adt2, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt12> {
    Ok(Adt12::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f294(f64(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f393(v1, v4)?)?), List::cons(f295(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f290(v0: Bytes) -> R<Bytes> {
    Ok(f394(f395(v0)?))
}

pub fn f291(v0: List<List<Bytes>>) -> List<Bytes> {
    let m1202 = v0;
    match m1202.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f292() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f293(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f359(v1)?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f294(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f396(f150(f95(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f397(f398(v3, f8(Str::lit("\u{a}"))))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f295(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f100(Adt2::C7(std::rc::Rc::new(f399(f145(f125(v1, f215(f96(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f296(p1205: List<Adt2>) -> bool {
    let m1204 = p1205;
    {
        let mut m1204 = m1204;
        loop {
            let step = {
                let v0 = m1204;
                let m1203 = v0;
                match m1203.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f32(f58(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f52(f8(Str::lit("studio")), f142(f400(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m1204 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: List<Bytes>) -> R<Option<List<Adt2>>> {
    let m1206 = v0;
    match m1206.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f198(v1)?;
            let a1208 = {
                let a1207 = v3.clone();
                length_bytes(a1207)
            };
            let a1209 = 0u64;
            if nat_eq(a1208, a1209) {
                f297(v2)
            } else {
                let m1210 = f43(v3)?;
                match m1210 {
                    None => {
                        Ok(None::<List<Adt2>>)
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
                            let m1211 = f297(v2)?;
                            match m1211 {
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

pub fn f298(v0: Bytes, v1: List<Adt2>) -> R<(Bytes, Bytes)> {
    Ok((v0, f100(Adt2::C7(std::rc::Rc::new(f101(v1))))?))
}

pub fn f299() -> Bytes {
    f8(Str::lit("model/errors.json"))
}

pub fn f300(p1214: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m1213 = p1214;
    {
        let mut m1213 = m1213;
        loop {
            let step = {
                let v0 = m1213;
                let m1212 = v0;
                match m1212.uncons() {
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
                Ok(next) => { m1213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f301(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> Adt2 {
    Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(f8(Str::lit("Studio.Error.Observation")))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("hasSimpleResult")), std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("madeBySensor")), std::rc::Rc::new(Adt2::C6(v2)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("observedProperty")), std::rc::Rc::new(Adt2::C6(v0)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("resultTime")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9))))))))))))
}

pub fn f302(v0: u64) -> bool {
    let a1215 = v0;
    let a1216 = 127u64;
    nat_le(a1215, a1216)
}

pub fn f303(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1217, _) = v1.clone();
        h1217
    };
    let v3: u64 = {
        let (h1218, _) = {
            let (_, h1219) = v1.clone();
            h1219
        };
        h1218
    };
    let v4: List<Bytes> = {
        let (_, h1220) = {
            let (_, h1221) = v1;
            h1221
        };
        h1220
    };
    let m1222 = f208(v0.clone(), v2);
    match m1222 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1223 = v5;
            let a1224 = 10u8;
            if equal(a1223, a1224) {
                Ok(Some(({
                    let a1225 = v2;
                    let a1226 = 1u64;
                    nat_add(a1225, a1226)?
                }, ({
                    let a1227 = v2;
                    let a1228 = 1u64;
                    nat_add(a1227, a1228)?
                }, List::cons(f402(f46(v0, v3, {
                    let a1229 = v2;
                    let a1230 = v3;
                    nat_sub(a1229, a1230)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1231 = v2;
                    let a1232 = 1u64;
                    nat_add(a1231, a1232)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f304(v0: Adt2) -> List<Adt2> {
    let m1233 = v0;
    match m1233 {
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
        Adt2::C7(r1234) => {
            let v3 = (*r1234).clone();
            f304(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r1235, r1236) => {
            let v1 = (*r1235).clone();
            let v2 = (*r1236).clone();
            List::cons(v1, f304(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f305(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1237 = v0;
    match m1237 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v11, r1238) => {
            let v12 = (*r1238).clone();
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
        Adt2::C7(r1239) => {
            let v10 = (*r1239).clone();
            Adt2::C7(std::rc::Rc::new(v10))
        }
        Adt2::C8(r1240) => {
            let v5 = (*r1240).clone();
            Adt2::C8(std::rc::Rc::new(f305(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1241, r1242) => {
            let v18 = (*r1241).clone();
            let v19 = (*r1242).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r1243, r1244) => {
            let v3 = (*r1243).clone();
            let v4 = (*r1244).clone();
            if f32(v2.clone(), f8(Str::lit("authority"))) {
                Adt2::C11(v2, std::rc::Rc::new(f403(v3, v1.clone())), std::rc::Rc::new(f305(v4, v1)))
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f305(v4, v1)))
            }
        }
    }
}

pub fn f306(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1245 = 0u64;
        let a1246 = v1;
        nat_lt(a1245, a1246)
    } else {
        false
    } {
        let a1247 = Bytes::lit(&[0x2d]);
        let a1248 = f404(v1)?;
        Ok(append_bytes(a1247, a1248))
    } else {
        f404(v1)
    }
}

pub fn f307(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f404(v1)?;
    let v4: u64 = {
        let a1249 = v3.clone();
        length_bytes(a1249)
    };
    let v5: Bytes = {
        let a1250 = v2;
        let a1251 = v4;
        if nat_lt(a1250, a1251) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1252 = v4;
                let a1253 = v2;
                nat_sub(a1252, a1253)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1254 = v4;
                let a1255 = v2;
                nat_sub(a1254, a1255)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f328({
                let a1256 = v2;
                let a1257 = v4;
                nat_sub(a1256, a1257)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1258 = f8(Str::lit("-"));
        let a1259 = v5;
        Ok(append_bytes(a1258, a1259))
    } else {
        Ok(v5)
    }
}

pub fn f308(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f405(v0)?;
    let v3: bool = {
        let a1260 = f406(v0)?;
        let a1261 = v1;
        nat_le(a1260, a1261)
    };
    let v4: u64 = {
        let a1265 = {
            let a1262 = v1;
            let a1263 = v2;
            let a1264 = 0u64;
            nat_quot(a1262, a1263, a1264)
        };
        let a1266 = f407(f408(v0))?;
        let a1267 = 0u64;
        nat_rem(a1265, a1266, a1267)
    };
    let v5: u64 = {
        let a1268 = v1;
        let a1269 = v2;
        let a1270 = 0u64;
        nat_rem(a1268, a1269, a1270)
    };
    let a1271 = v4;
    let a1272 = f409(v0)?;
    if nat_eq(a1271, a1272) {
        let a1273 = v5;
        let a1274 = 0u64;
        if nat_eq(a1273, a1274) {
            Ok(f410(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f410(v3, {
                let a1278 = {
                    let a1275 = v2;
                    let a1276 = 2u64;
                    let a1277 = 0u64;
                    nat_quot(a1275, a1276, a1277)
                };
                let a1279 = v5;
                if nat_le(a1278, a1279) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f117({
            let a1280 = v4;
            let a1281 = 0u64;
            nat_eq(a1280, a1281)
        }, {
            let a1282 = v5;
            let a1283 = 0u64;
            nat_eq(a1282, a1283)
        }) {
            Ok(f410(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1284 = v4;
                let a1285 = 0u64;
                if nat_eq(a1284, a1285) {
                    v5
                } else {
                    let a1286 = v2;
                    let a1287 = v5;
                    nat_add(a1286, a1287)?
                }
            };
            let v7: u64 = {
                let a1288 = v4;
                let a1289 = 0u64;
                if nat_eq(a1288, a1289) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1296 = {
                    let a1292 = {
                        let a1290 = v7;
                        let a1291 = 2000u64;
                        nat_add(a1290, a1291)?
                    };
                    let a1293 = f411(v0);
                    nat_sub(a1292, a1293)
                };
                let a1297 = {
                    let a1294 = f412(v0);
                    let a1295 = 1u64;
                    nat_sub(a1294, a1295)
                };
                nat_sub(a1296, a1297)
            };
            Ok(f410(v3, {
                let a1298 = 2000u64;
                let a1299 = v8;
                if nat_le(a1298, a1299) {
                    f413(v6, true, {
                        let a1300 = v8;
                        let a1301 = 2000u64;
                        nat_sub(a1300, a1301)
                    })?
                } else {
                    f413(v6, false, {
                        let a1302 = 2000u64;
                        let a1303 = v8;
                        nat_sub(a1302, a1303)
                    })?
                }
            }))
        }
    }
}

pub fn f309(v0: Bytes) -> R<Bytes> {
    let a1308 = f200(v0.clone(), 0u64, {
        let a1305 = {
            let a1304 = v0.clone();
            length_bytes(a1304)
        };
        let a1306 = 1u64;
        nat_add(a1305, a1306)?
    })?;
    let a1309 = {
        let a1307 = v0.clone();
        length_bytes(a1307)
    };
    if nat_eq(a1308, a1309) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1315 = {
            let a1313 = Bytes::lit(&[0x22]);
            let a1314 = f414(v0.clone(), 0u64, {
                let a1311 = {
                    let a1310 = v0;
                    length_bytes(a1310)
                };
                let a1312 = 1u64;
                nat_add(a1311, a1312)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1313, a1314)
        };
        let a1316 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1315, a1316))
    }
}

pub fn f310(v0: List<Bytes>) -> Bytes {
    let a1319 = {
        let a1317 = Bytes::lit(&[0x5b]);
        let a1318 = f122(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1317, a1318)
    };
    let a1320 = Bytes::lit(&[0x5d]);
    append_bytes(a1319, a1320)
}

pub fn f311(v0: Adt2) -> R<List<Bytes>> {
    let m1321 = v0;
    match m1321 {
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
        Adt2::C10(r1322, r1323) => {
            let v14 = (*r1322).clone();
            let v15 = (*r1323).clone();
            Ok(List::cons(f190(v14)?, f311(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f312(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f415(Fn5::F416, List::<(Bytes, Bytes)>::nil(), f417(v0)?);
    let v2: List<Bytes> = f418(Fn6::F419, List::<Bytes>::nil(), v1)?;
    let a1326 = {
        let a1324 = Bytes::lit(&[0x7b]);
        let a1325 = f122(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1324, a1325)
    };
    let a1327 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1326, a1327))
}

pub fn f313(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1328 = v0;
    match m1328 {
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
        Adt2::C11(v16, r1329, r1330) => {
            let v17 = (*r1329).clone();
            let v18 = (*r1330).clone();
            Ok(List::cons((v16, f190(v17)?), f313(v18)?))
        }
    }
}

pub fn f314() -> Bytes {
    f8(Str::lit("target/engine"))
}

pub fn f315(v0: Adt13) -> Bytes {
    let m1331 = v0;
    match m1331 {
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

pub fn f316() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f317(v0: Adt13, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("{\"oracle_ids\":[],\"realized_by\":[],")), List::cons(f420(f8(Str::lit("retrieval_date")), v4)?, List::cons(f8(Str::lit(",\"schema\":\"prismpm/authority-binding/1\",\"signature_trust_root\":null,\"source_role\":\"normative\",\"supersession\":\"explicit-lock-update-only\",")), List::cons(f420(f8(Str::lit("id")), f69(v0.clone(), f61(v1.clone())))?, List::cons(f8(Str::lit(",")), List::cons(f420(f8(Str::lit("name")), f421(v0.clone()))?, List::cons(f8(Str::lit(",\"issuer\":\"afflom\",")), List::cons(f420(f8(Str::lit("edition")), v1.clone())?, List::cons(f8(Str::lit(",")), List::cons(f420(f8(Str::lit("canonical_id")), f7(List::cons(f8(Str::lit("https://github.com/afflom/verdant-engine/releases/tag/")), List::cons(v1, List::<Bytes>::nil()))))?, List::cons(f8(Str::lit(",")), List::cons(f420(f8(Str::lit("statement")), f422(v0.clone()))?, List::cons(f8(Str::lit(",\"source\":{\"license\":\"MIT OR Apache-2.0\",")), List::cons(f420(f8(Str::lit("media_type")), f423(v0.clone()))?, List::cons(f8(Str::lit(",\"redistribution\":\"redistributable\",")), List::cons(f420(f8(Str::lit("revision")), v2.clone())?, List::cons(f8(Str::lit(",")), List::cons(f420(f8(Str::lit("sha256")), v3)?, List::cons(f8(Str::lit(",\"signature\":\"none\",")), List::cons(f420(f8(Str::lit("url")), f193(v0, v2))?, List::cons(f8(Str::lit("}}\u{a}")), List::<Bytes>::nil())))))))))))))))))))))))
}

pub fn f318(v0: Bytes) -> R<u64> {
    let (h1332, _) = f321(Fn3::F424(v0.clone()), {
        let a1333 = v0.clone();
        length_bytes(a1333)
    }, {
        let a1334 = v0;
        length_bytes(a1334)
    })?;
    Ok(h1332)
}

pub fn f319(v0: Bytes, v1: u64) -> R<u64> {
    let (h1335, _) = f321(Fn3::F425(v1, v0), v1, 0u64)?;
    Ok(h1335)
}

pub fn f320(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f115(v0.clone(), {
        let a1336 = v1;
        let a1337 = 1u64;
        nat_add(a1336, a1337)?
    })?;
    let v4: Bytes = {
        let a1338 = v3;
        let a1339 = 117u64;
        if nat_eq(a1338, a1339) {
            f426(v0, v1)?
        } else {
            f427(v3)
        }
    };
    let a1340 = v4.clone();
    let a1341 = Bytes::lit(&[]);
    if equal(a1340, a1341) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1342 = v3;
            let a1343 = 117u64;
            if nat_eq(a1342, a1343) {
                let a1344 = v1;
                let a1345 = 6u64;
                nat_add(a1344, a1345)?
            } else {
                let a1346 = v1;
                let a1347 = 2u64;
                nat_add(a1346, a1347)?
            }
        }, ({
            let a1348 = v2;
            let a1349 = v4;
            append_bytes(a1348, a1349)
        }, 0u64))))
    }
}

pub fn f321(v0: Fn3, p1354: u64, p1355: u64) -> R<(u64, bool)> {
    let m1353 = (p1354, p1355);
    {
        let mut m1353 = m1353;
        loop {
            let step = {
                let (v1, v2) = m1353;
                let m1350 = v1;
                if m1350 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1350 - 1;
                    let m1352 = {
                        let c1351: Fn3 = v0.clone();
                        c1351.apply(v2)?
                    };
                    match m1352 {
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
                Ok(next) => { m1353 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f322(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f428(v0, v1)
}

pub fn f323(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f429(v0, v1)
}

pub fn f324(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1356 = v0.clone();
        length_bytes(a1356)
    };
    let a1357 = 0u64;
    let a1358 = v1;
    if nat_lt(a1357, a1358) {
        let a1359 = f430(v0.clone())?;
        let a1360 = v1;
        if nat_eq(a1359, a1360) {
            let a1361 = 1u64;
            let a1362 = v1;
            if nat_lt(a1361, a1362) {
                let m1365 = {
                    let a1363 = v0;
                    let a1364 = 0u64;
                    index_bytes(a1363, a1364)
                };
                match m1365 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1366 = v2;
                        let a1367 = 48u8;
                        Ok(!equal(a1366, a1367))
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

pub fn f325(v0: Fn4, p1372: u64, p1373: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1371 = (p1372, p1373);
    {
        let mut m1371 = m1371;
        loop {
            let step = {
                let (v1, v2) = m1371;
                let m1368 = v1;
                if m1368 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1368 - 1;
                    let m1370 = {
                        let c1369: Fn4 = v0.clone();
                        c1369.apply(v2)?
                    };
                    match m1370 {
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
                Ok(next) => { m1371 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f431(v0, v1)
}

pub fn f327(v0: Bytes) -> R<u64> {
    let (h1374, _) = f321(Fn3::F432(v0.clone()), {
        let a1376 = {
            let a1375 = v0;
            length_bytes(a1375)
        };
        let a1377 = 1u64;
        nat_add(a1376, a1377)?
    }, 0u64)?;
    Ok(h1374)
}

pub fn f328(v0: u64) -> Bytes {
    let m1378 = v0;
    if m1378 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1378 - 1;
        let a1379 = f8(Str::lit("0"));
        let a1380 = f328(v1);
        append_bytes(a1379, a1380)
    }
}

pub fn f329(v0: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1381 = v0;
    match m1381.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1382 = f93(v1, f8(Str::lit("std")));
            match m1382 {
                None => {
                    f329(v2)
                }
                Some(v3) => {
                    if f221(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f58(v3.clone(), f8(Str::lit("row"))), f58(v3, f8(Str::lit("authority")))), f329(v2))
                    } else {
                        f329(v2)
                    }
                }
            }
        }
    }
}

pub fn f330(v0: Adt14, v1: List<Adt2>) -> List<(Bytes, Bytes)> {
    let m1383 = v1;
    match m1383.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f58(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f58(v2, f8(Str::lit("object")));
            if if f52(v4.clone(), match v0.clone() {
                Adt14::C0(h1384, _, _) => {
                    h1384
                }
            }) {
                f52(v5.clone(), match v0.clone() {
                    Adt14::C0(_, h1385, _) => {
                        h1385
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f330(v0, v3))
            } else {
                if if f52(v5.clone(), match v0.clone() {
                    Adt14::C0(h1386, _, _) => {
                        h1386
                    }
                }) {
                    f52(v4.clone(), match v0.clone() {
                        Adt14::C0(_, h1387, _) => {
                            h1387
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f330(v0, v3))
                } else {
                    f330(v0, v3)
                }
            }
        }
    }
}

pub fn f331(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt17>> {
    let m1388 = v0;
    match m1388.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v4, v5)) => {
            if if f433(v4.clone(), v2.clone()) {
                true
            } else {
                f433(v4.clone(), v3.clone())
            } {
                f331(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt17::C0({
                    let (h1389, _) = v4.clone();
                    h1389
                }, {
                    let a1392 = {
                        let a1391 = f145(f434({
                            let (h1390, _) = v4.clone();
                            h1390
                        }, v1.clone()))?;
                        length_list(a1391)
                    };
                    let a1393 = 1u64;
                    if nat_eq(a1392, a1393) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1394) = v4.clone();
                    h1394
                }), f331(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f332(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1395 = v1;
    match m1395.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f52({
                let (h1396, _) = v2.clone();
                h1396
            }, v0.clone()) {
                List::cons({
                    let (_, h1397) = v2;
                    h1397
                }, f332(v0, v3))
            } else {
                if f52({
                    let (_, h1398) = v2;
                    h1398
                }, v0.clone()) {
                    List::cons({
                        let (h1399, _) = v2;
                        h1399
                    }, f332(v0, v3))
                } else {
                    f332(v0, v3)
                }
            }
        }
    }
}

pub fn f333(v0: Adt14, v1: Adt2, v2: Bytes) -> bool {
    if f52(v2.clone(), match v0.clone() {
        Adt14::C0(h1400, _, _) => {
            h1400
        }
    }) {
        true
    } else {
        if f52(v2.clone(), match v0.clone() {
            Adt14::C0(_, h1401, _) => {
                h1401
            }
        }) {
            true
        } else {
            if f52(v2, match v0.clone() {
                Adt14::C0(_, _, h1402) => {
                    h1402
                }
            }) {
                true
            } else {
                if f221(v1.clone(), f8(Str::lit("scheme"))) {
                    f52(f58(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt14::C0(_, h1403, _) => {
                            h1403
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f334(v0: Adt2) -> bool {
    if f221(v0.clone(), f8(Str::lit("domain"))) {
        if f221(v0.clone(), f8(Str::lit("row"))) {
            if f52(f58(v0.clone(), f8(Str::lit("relation"))), f435()) {
                if f436(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f436(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f436(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f436(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f335(v0: Adt2) -> bool {
    if f221(v0.clone(), f8(Str::lit("subject"))) {
        if f221(v0.clone(), f8(Str::lit("object"))) {
            if f221(v0.clone(), f8(Str::lit("evidence"))) {
                f52(f58(v0, f8(Str::lit("relation"))), f437())
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

pub fn f336(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> R<List<Bytes>> {
    let m1404 = f43(v1)?;
    match m1404 {
        None => {
            Ok(f229(v0, f8(Str::lit("a line is not one JSON text (RFC 8259 section 2)"))))
        }
        Some(v4) => {
            let v5: List<Bytes> = f215(f188(f93(v4.clone(), f8(Str::lit("citation")))));
            let v7: Bytes = {
                let m1405 = f92(f93(v4.clone(), f8(Str::lit("construct"))));
                match m1405 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v6) => {
                        v6
                    }
                }
            };
            let m1406 = v5;
            match m1406.uncons() {
                None => {
                    Ok(f229(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                }
                Some((v8, v9)) => {
                    let m1407 = v9;
                    match m1407.uncons() {
                        None => {
                            Ok(f229(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                        }
                        Some((v10, _)) => {
                            if f51(f32(v8, f8(Str::lit("std")))) {
                                Ok(f229(v0, f8(Str::lit("a row's citation is not [\"std\", AUTHORITY, ROW]"))))
                            } else {
                                if f51(f52(v10.clone(), v2)) {
                                    Ok(f229(v0, f7(List::cons(f8(Str::lit("the cited authority ")), List::cons(v10, List::cons(f8(Str::lit(" is not bound")), List::<Bytes>::nil()))))))
                                } else {
                                    if f117(f51(f32(v7.clone(), f8(Str::lit("none")))), f51(f52(v7.clone(), v3))) {
                                        Ok(f229(v0, f7(List::cons(f8(Str::lit("the construct ")), List::cons(v7, List::cons(f8(Str::lit(" is not admitted")), List::<Bytes>::nil()))))))
                                    } else {
                                        let m1408 = f92(f93(v4.clone(), f8(Str::lit("pointer"))));
                                        match m1408 {
                                            None => {
                                                Ok(f229(v0, f8(Str::lit("a row has no RFC 6901 pointer"))))
                                            }
                                            Some(_) => {
                                                let m1409 = f93(v4.clone(), f8(Str::lit("reader")));
                                                match m1409 {
                                                    None => {
                                                        let m1410 = f93(v4, f8(Str::lit("value")));
                                                        match m1410 {
                                                            None => {
                                                                Ok(f229(v0, f8(Str::lit("a row names neither the reader of its artifact nor a value"))))
                                                            }
                                                            Some(_) => {
                                                                Ok(List::<Bytes>::nil())
                                                            }
                                                        }
                                                    }
                                                    Some(_) => {
                                                        if f221(v4, f8(Str::lit("artifact"))) {
                                                            Ok(List::<Bytes>::nil())
                                                        } else {
                                                            Ok(f229(v0, f8(Str::lit("a row names its reader but not its artifact"))))
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

pub fn f337(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m1411 = v1;
    match m1411.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f438(v3.clone())
            } else {
                f51(f438(v3.clone()))
            } {
                List::cons(f242(v0.clone(), v3), f337(v0, v4, v2))
            } else {
                f337(v0, v4, v2)
            }
        }
    }
}

pub fn f338(p1414: List<Bytes>) -> R<Option<Bytes>> {
    let m1413 = p1414;
    {
        let mut m1413 = m1413;
        loop {
            let step = {
                let v0 = m1413;
                let m1412 = v0;
                match m1412.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f439(v1.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f198(v1)?)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
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

pub fn f339(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1415 = v1;
    match m1415.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f95(f64(v0.clone(), v2))?, f339(v0, v3)?))
        }
    }
}

pub fn f340(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1416 = v0.clone();
        length_bytes(a1416)
    };
    let v3: u64 = {
        let a1417 = v1.clone();
        length_bytes(a1417)
    };
    let v4: u64 = {
        let (h1418, _) = f321(Fn3::F440(v1.clone(), v3, v0.clone(), v2), {
            let a1419 = v2;
            let a1420 = 1u64;
            nat_add(a1419, a1420)?
        }, 0u64)?;
        h1418
    };
    if f441(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f341() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f342(v0: Adt15) -> R<Adt18> {
    Ok(Adt18::C0(f442(match v0.clone() {
        Adt15::C0(h1421, _, _, _, _) => {
            h1421
        }
    }, 0u64)?, f442(match v0.clone() {
        Adt15::C0(_, h1422, _, _, _) => {
            h1422
        }
    }, 0u64)?, f442(match v0.clone() {
        Adt15::C0(_, _, h1423, _, _) => {
            h1423
        }
    }, 0u64)?, f442(match v0 {
        Adt15::C0(_, _, _, h1424, _) => {
            h1424
        }
    }, 0u64)?))
}

pub fn f343(v0: Adt15) -> List<Bytes> {
    f125(match v0.clone() {
        Adt15::C0(h1425, _, _, _, _) => {
            h1425
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, h1426, _, _) => {
            h1426
        }
    }, f125(match v0.clone() {
        Adt15::C0(_, _, _, h1427, _) => {
            h1427
        }
    }, match v0 {
        Adt15::C0(_, _, _, _, h1428) => {
            h1428
        }
    })))
}

pub fn f344(v0: Adt18, p1434: List<u64>, p1435: List<Adt20>) -> Option<u64> {
    let m1433 = (p1434, p1435);
    {
        let mut m1433 = m1433;
        loop {
            let step = {
                let (v1, v2) = m1433;
                let m1429 = v2;
                match m1429.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f443(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h1430) => {
                                h1430
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h1431, _) => {
                                    h1431
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h1432, _) => {
                                    h1432
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: Adt15, v1: List<Adt2>, v2: u64) -> R<List<Adt20>> {
    let m1436 = v1;
    match m1436.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f444(v0.clone(), v3)?), f345(v0, v4, {
                let a1437 = v2;
                let a1438 = 1u64;
                nat_add(a1437, a1438)?
            })?))
        }
    }
}

pub fn f346(p1442: List<Bytes>, p1443: u64) -> Bytes {
    let m1441 = (p1442, p1443);
    {
        let mut m1441 = m1441;
        loop {
            let step = {
                let (v0, v1) = m1441;
                let m1439 = v0;
                match m1439.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1440 = v1;
                        if m1440 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m1440 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f445(v0.clone(), v1.clone()) {
        if f445(v1, v0) {
            f446(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f348(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m1444 = v0;
    match m1444.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f447(v2, v1.clone())?, f348(v3, v1)?))
        }
    }
}

pub fn f349(v0: List<Adt2>) -> List<u64> {
    let m1445 = v0;
    match m1445.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f448(v1), f349(v2))
        }
    }
}

pub fn f350(p1448: List<u64>, v1: List<u64>) -> Option<u64> {
    let m1447 = p1448;
    {
        let mut m1447 = m1447;
        loop {
            let step = {
                let v0 = m1447;
                let m1446 = v0;
                match m1446.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f449(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1447 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(p1451: List<Bytes>, p1452: List<Bytes>) -> Option<Bytes> {
    let m1450 = (p1451, p1452);
    {
        let mut m1450 = m1450;
        loop {
            let step = {
                let (v0, v1) = m1450;
                let m1449 = v0;
                match m1449.uncons() {
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
                Ok(next) => { m1450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(v0: Adt15, p1460: List<Adt2>) -> Option<Bytes> {
    let m1459 = p1460;
    {
        let mut m1459 = m1459;
        loop {
            let step = {
                let v1 = m1459;
                let m1453 = v1;
                match m1453.uncons() {
                    None => {
                        Err::<List<Adt2>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1454 = f93(v2.clone(), f8(Str::lit("evidence")));
                        match m1454 {
                            None => {
                                Ok::<List<Adt2>, Option<Bytes>>(v3)
                            }
                            Some(v4) => {
                                let m1455 = f93(v4, f8(Str::lit("constraint")));
                                match m1455 {
                                    None => {
                                        Ok::<List<Adt2>, Option<Bytes>>(v3)
                                    }
                                    Some(v5) => {
                                        if f52(f58(v5.clone(), f8(Str::lit("authority"))), match v0.clone() {
                                            Adt15::C0(h1456, _, _, _, _) => {
                                                h1456
                                            }
                                        }) {
                                            let m1457 = f92(f93(v5, f8(Str::lit("row"))));
                                            match m1457 {
                                                None => {
                                                    Ok::<List<Adt2>, Option<Bytes>>(v3)
                                                }
                                                Some(v6) => {
                                                    if f52(v6, match v0.clone() {
                                                        Adt15::C0(_, h1458, _, _, _) => {
                                                            h1458
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
                Ok(next) => { m1459 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f354(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1461 = v0.clone();
    match m1461.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1462 = v2;
            let a1463 = v1.clone();
            if equal(a1462, a1463) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f355(v0: List<(Bytes, ())>) -> List<Bytes> {
    f239(f450(Fn7::F451, List::<Bytes>::nil(), v0))
}

pub fn f356(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f452(f453(Fn8::F454, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1468 = {
        let (h1464, _) = f455(Fn9::F456, {
            let a1466 = {
                let a1465 = v0;
                length_list(a1465)
            };
            let a1467 = 1u64;
            nat_add(a1466, a1467)?
        }, v1)?;
        h1464
    };
    match m1468.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f357(v0: List<Bytes>) -> List<(Bytes, ())> {
    f457(f458(Fn10::F459, List::<(Bytes, ())>::nil(), v0))
}

pub fn f358(v0: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> R<List<Bytes>> {
    let m1469 = f44(v1, 34u8)?;
    match m1469.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1470 = v3;
            match m1470.uncons() {
                None => {
                    Ok(f460(v0, v2))
                }
                Some((_, _)) => {
                    Ok(List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f359(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f461(v0.clone(), 0u64, {
        let a1471 = v0;
        length_bytes(a1471)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f360(v0: Adt2, v1: Bytes) -> Bytes {
    if f32(f462(f93(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f361(v0: Bytes, v1: Bytes, p1484: u64, p1485: u64) -> R<bool> {
    let m1483 = (p1484, p1485);
    {
        let mut m1483 = m1483;
        loop {
            let step = {
                let (v2, v3) = m1483;
                let m1472 = v3;
                if m1472 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v4 = m1472 - 1;
                    let m1475 = {
                        let a1473 = v1.clone();
                        let a1474 = v2;
                        index_bytes(a1473, a1474)
                    };
                    match m1475 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v5) => {
                            let m1478 = {
                                let a1476 = v0.clone();
                                let a1477 = v2;
                                index_bytes(a1476, a1477)
                            };
                            match m1478 {
                                None => {
                                    Err::<(u64, u64), R<bool>>(Ok(false))
                                }
                                Some(v6) => {
                                    let a1479 = v5;
                                    let a1480 = v6;
                                    if equal(a1479, a1480) {
                                        Ok::<(u64, u64), R<bool>>(({
                                            let a1481 = v2;
                                            let a1482 = 1u64;
                                            nat_add(a1481, a1482)?
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
                Ok(next) => { m1483 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f362(v0: Adt1, v1: Bytes) -> R<Adt2> {
    let m1486 = f91(v0, v1)?;
    match m1486 {
        None => {
            Ok(Adt2::C9)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f363(v0: Bytes) -> R<Bytes> {
    let a1487 = f8(Str::lit("Catalog."));
    let a1488 = f391(v0)?;
    Ok(append_bytes(a1487, a1488))
}

pub fn f364(v0: Adt2) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f391(f58(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f365(v0: List<Bytes>) -> List<Bytes> {
    let m1489 = v0;
    match m1489.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f41(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f242(f277(), v1), f365(v2))
            } else {
                f365(v2)
            }
        }
    }
}

pub fn f366(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1490 = v1;
    match m1490.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            Ok(f125(f463(f464(f95(f64(v0.clone(), v2))?)?), f366(v0, v3)?))
        }
    }
}

pub fn f367(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let m1491 = v1;
    match m1491.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v3, v4)) => {
            let v5: (bool, Bytes) = f465(v0.clone(), v3, v2.clone())?;
            let (h1492, _) = v5.clone();
            if h1492 {
                Ok(v5)
            } else {
                let v6: (bool, Bytes) = f367(v0, v4, v2)?;
                let (h1493, _) = v6.clone();
                if h1493 {
                    Ok(v6)
                } else {
                    Ok((false, {
                        let a1496 = {
                            let (_, h1494) = v5;
                            h1494
                        };
                        let a1497 = {
                            let (_, h1495) = v6;
                            h1495
                        };
                        append_bytes(a1496, a1497)
                    }))
                }
            }
        }
    }
}

pub fn f368(v0: Adt14, v1: List<Adt2>) -> List<Bytes> {
    let m1498 = v1;
    match m1498.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f221(v2.clone(), f8(Str::lit("subject"))) {
                if f222(v0.clone(), v2.clone()) {
                    f368(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f58(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f368(v0, v3))
                }
            } else {
                if f378(v0.clone(), v2.clone()) {
                    f368(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f58(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f379(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f368(v0, v3))
                }
            }
        }
    }
}

pub fn f369(v0: Adt1) -> R<Bytes> {
    let a1500 = {
        let a1499 = f128(f129(f266(f263(f264(f64(v0.clone(), f213()))?)), f130(f96(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f131(f96(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1499)
    };
    let a1501 = 0u64;
    if nat_eq(a1500, a1501) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f370(v0: List<Adt2>) -> R<List<Bytes>> {
    let m1502 = v0;
    match m1502.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f122(f215(f188(f93(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f58(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f58(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f466(f95(f58(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f370(v2)?))
        }
    }
}

pub fn f371() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f372(v0: Bytes, p1560: u64, p1561: u64, p1562: Adt19) -> R<Adt19> {
    let m1559 = (p1560, (p1561, p1562));
    {
        let mut m1559 = m1559;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1559;
                let m1503 = v2;
                if m1503 == 0 {
                    Err::<(u64, (u64, Adt19)), R<Adt19>>(Ok(v3))
                } else {
                    let v4 = m1503 - 1;
                    let v5: u64 = f467(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt19::C0(_, h1504, _, _, _) => {
                            h1504
                        }
                    } {
                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                            let a1505 = v1;
                            let a1506 = 1u64;
                            nat_add(a1505, a1506)?
                        }, (v4, Adt19::C0(true, false, match v3.clone() {
                            Adt19::C0(_, _, h1507, _, _) => {
                                h1507
                            }
                        }, match v3.clone() {
                            Adt19::C0(_, _, _, h1508, _) => {
                                h1508
                            }
                        }, match v3 {
                            Adt19::C0(_, _, _, _, h1509) => {
                                h1509
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt19::C0(h1510, _, _, _, _) => {
                                h1510
                            }
                        } {
                            let a1511 = v5;
                            let a1512 = 34u64;
                            if nat_eq(a1511, a1512) {
                                let a1515 = f467(v0.clone(), {
                                    let a1513 = v1;
                                    let a1514 = 1u64;
                                    nat_add(a1513, a1514)?
                                })?;
                                let a1516 = 34u64;
                                if nat_eq(a1515, a1516) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1517 = v1;
                                        let a1518 = 1u64;
                                        nat_add(a1517, a1518)?
                                    }, (v4, Adt19::C0(true, true, f468(match v3.clone() {
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
                                } else {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1522 = v1;
                                        let a1523 = 1u64;
                                        nat_add(a1522, a1523)?
                                    }, (v4, Adt19::C0(false, false, match v3.clone() {
                                        Adt19::C0(_, _, h1524, _, _) => {
                                            h1524
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1525, _) => {
                                            h1525
                                        }
                                    }, match v3 {
                                        Adt19::C0(_, _, _, _, h1526) => {
                                            h1526
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1527 = v1;
                                    let a1528 = 1u64;
                                    nat_add(a1527, a1528)?
                                }, (v4, Adt19::C0(true, false, f468(match v3.clone() {
                                    Adt19::C0(_, _, h1529, _, _) => {
                                        h1529
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt19::C0(_, _, _, h1530, _) => {
                                        h1530
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1531) => {
                                        h1531
                                    }
                                }))))
                            }
                        } else {
                            let a1532 = v5;
                            let a1533 = 34u64;
                            if nat_eq(a1532, a1533) {
                                Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                    let a1534 = v1;
                                    let a1535 = 1u64;
                                    nat_add(a1534, a1535)?
                                }, (v4, Adt19::C0(true, false, match v3.clone() {
                                    Adt19::C0(_, _, h1536, _, _) => {
                                        h1536
                                    }
                                }, match v3.clone() {
                                    Adt19::C0(_, _, _, h1537, _) => {
                                        h1537
                                    }
                                }, match v3 {
                                    Adt19::C0(_, _, _, _, h1538) => {
                                        h1538
                                    }
                                }))))
                            } else {
                                let a1539 = v5;
                                let a1540 = 44u64;
                                if nat_eq(a1539, a1540) {
                                    Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                        let a1541 = v1;
                                        let a1542 = 1u64;
                                        nat_add(a1541, a1542)?
                                    }, (v4, Adt19::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt19::C0(_, _, h1543, _, _) => {
                                            h1543
                                        }
                                    }, match v3.clone() {
                                        Adt19::C0(_, _, _, h1544, _) => {
                                            h1544
                                        }
                                    }), match v3 {
                                        Adt19::C0(_, _, _, _, h1545) => {
                                            h1545
                                        }
                                    }))))
                                } else {
                                    let a1546 = v5;
                                    let a1547 = 13u64;
                                    if nat_eq(a1546, a1547) {
                                        Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                            let a1548 = v1;
                                            let a1549 = 1u64;
                                            nat_add(a1548, a1549)?
                                        }, (v4, v3)))
                                    } else {
                                        let a1550 = v5;
                                        let a1551 = 10u64;
                                        if nat_eq(a1550, a1551) {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1552 = v1;
                                                let a1553 = 1u64;
                                                nat_add(a1552, a1553)?
                                            }, (v4, f373(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt19)), R<Adt19>>(({
                                                let a1554 = v1;
                                                let a1555 = 1u64;
                                                nat_add(a1554, a1555)?
                                            }, (v4, Adt19::C0(false, false, f468(match v3.clone() {
                                                Adt19::C0(_, _, h1556, _, _) => {
                                                    h1556
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt19::C0(_, _, _, h1557, _) => {
                                                    h1557
                                                }
                                            }, match v3 {
                                                Adt19::C0(_, _, _, _, h1558) => {
                                                    h1558
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
                Ok(next) => { m1559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373(v0: Adt19) -> Adt19 {
    Adt19::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f469(List::cons(match v0.clone() {
        Adt19::C0(_, _, h1563, _, _) => {
            h1563
        }
    }, match v0.clone() {
        Adt19::C0(_, _, _, h1564, _) => {
            h1564
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt19::C0(_, _, _, _, h1565) => {
            h1565
        }
    }))
}

pub fn f374(p1568: List<List<Bytes>>, p1569: List<List<Bytes>>) -> List<List<Bytes>> {
    let m1567 = (p1568, p1569);
    {
        let mut m1567 = m1567;
        loop {
            let step = {
                let (v0, v1) = m1567;
                let m1566 = v0;
                match m1566.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f375(v0: Bytes, v1: List<Adt17>) -> List<Bytes> {
    let m1570 = v1;
    match m1570.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(match v2.clone() {
                Adt17::C0(h1571, _, _) => {
                    h1571
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt17::C0(_, h1572, _) => {
                        h1572
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt17::C0(_, _, h1573) => {
                        h1573
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f375(v0, v3))
            } else {
                f375(v0, v3)
            }
        }
    }
}

pub fn f376(v0: Bytes, v1: List<Adt2>) -> List<Bytes> {
    let m1574 = v1;
    match m1574.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32(f58(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f58(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f376(v0, v3))
            } else {
                if f32(f58(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f58(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f58(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f376(v0, v3))
                } else {
                    f376(v0, v3)
                }
            }
        }
    }
}

pub fn f377(p1579: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1578 = p1579;
    {
        let mut m1578 = m1578;
        loop {
            let step = {
                let v0 = m1578;
                let m1575 = v0;
                match m1575.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1576 = v2;
                        match m1576.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f32(v4, v1.clone()) {
                                    let m1577 = v5;
                                    match m1577.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f470(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1578 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(v0: Adt14, v1: Adt2) -> bool {
    if f52(f58(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt14::C0(h1580, _, _) => {
            h1580
        }
    }) {
        f52(f379(v1), match v0 {
            Adt14::C0(_, h1581, _) => {
                h1581
            }
        })
    } else {
        false
    }
}

pub fn f379(v0: Adt2) -> Bytes {
    let m1582 = f93(v0, f8(Str::lit("evidence")));
    match m1582 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f58(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f380(v0: List<List<Bytes>>, v1: Bytes, v2: List<Bytes>) -> R<List<Bytes>> {
    let m1583 = v2;
    match m1583.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            if f471(v0.clone(), v1.clone(), v3.clone())? {
                Ok(List::cons(f7(List::cons(v1.clone(), List::cons(f8(Str::lit("/")), List::cons(v3, List::cons(f8(Str::lit("\u{9}\u{a}")), List::<Bytes>::nil()))))), f380(v0, v1, v4)?))
            } else {
                f380(v0, v1, v4)
            }
        }
    }
}

pub fn f381() -> Bytes {
    f8(Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))
}

pub fn f382() -> R<List<Bytes>> {
    f44(f8(Str::lit("CON PRE PRO QA LCH OPS MNT EOL")), 32u8)
}

pub fn f383(v0: Bytes, v1: Bytes, v2: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1584 = v2;
    match m1584.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            if f117(f32({
                let (h1585, _) = v3.clone();
                h1585
            }, v0.clone()), {
                let a1587 = f472(v1.clone())?;
                let a1588 = f472({
                    let (_, h1586) = v3.clone();
                    h1586
                })?;
                nat_lt(a1587, a1588)
            }) {
                Ok(List::cons({
                    let a1590 = v1.clone();
                    let a1591 = {
                        let (_, h1589) = v3;
                        h1589
                    };
                    append_bytes(a1590, a1591)
                }, f383(v0, v1, v4)?))
            } else {
                f383(v0, v1, v4)
            }
        }
    }
}

pub fn f384(v0: Adt2) -> Bytes {
    if f221(v0.clone(), f8(Str::lit("req"))) {
        let a1592 = f8(Str::lit("req "));
        let a1593 = f58(v0, f8(Str::lit("req")));
        append_bytes(a1592, a1593)
    } else {
        let m1594 = f93(v0.clone(), f8(Str::lit("std")));
        match m1594 {
            None => {
                let m1597 = f93(v0, f8(Str::lit("der")));
                match m1597 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f58(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f122(f215(f188(f93(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f58(v1.clone(), f8(Str::lit("authority"))), List::cons(if f221(v1.clone(), f8(Str::lit("row"))) {
                    let a1595 = f8(Str::lit(" row "));
                    let a1596 = f58(v1, f8(Str::lit("row")));
                    append_bytes(a1595, a1596)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f385(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt2>>> {
    let m1598 = v0;
    match m1598.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f198(v2)?;
            let a1600 = {
                let a1599 = v4.clone();
                length_bytes(a1599)
            };
            let a1601 = 0u64;
            if nat_eq(a1600, a1601) {
                f385(v3, v1)
            } else {
                let m1602 = f43(v4)?;
                match m1602 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f473(v5.clone());
                        let a1604 = {
                            let a1603 = v6.clone();
                            length_bytes(a1603)
                        };
                        let a1605 = 0u64;
                        if nat_eq(a1604, a1605) {
                            Ok(None::<List<Adt2>>)
                        } else {
                            let m1606 = f385(v3, v1.clone())?;
                            match m1606 {
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

pub fn f386(v0: Bytes, v1: Bytes) -> bool {
    let a1607 = v0;
    let a1608 = f392();
    if equal(a1607, a1608) {
        let a1612 = {
            let a1610 = {
                let a1609 = v1;
                length_bytes(a1609)
            };
            let a1611 = 0u64;
            nat_eq(a1610, a1611)
        };
        bool_not(a1612)
    } else {
        false
    }
}

pub fn f387(v0: Bytes) -> Bytes {
    f7(List::cons(f144(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f388(v0: Bytes, v1: Bytes) -> R<bool> {
    let a1614 = v0;
    let a1615 = {
        let a1613 = Str::lit("redistributable");
        utf8_encode(a1613)
    };
    if equal(a1614, a1615) {
        f474(v1)
    } else {
        Ok(false)
    }
}

pub fn f389(v0: Bytes, v1: Bytes) -> bool {
    f203(f85(v0.clone(), f8(Str::lit("read-"))), f117(f32(v0, f8(Str::lit("check-yaml"))), f32(v1, f8(Str::lit("none")))))
}

pub fn f390(v0: Adt2) -> bool {
    if f475(v0.clone(), f8(Str::lit("name"))) {
        if f476(v0.clone(), f8(Str::lit("schema"))) {
            let m1616 = f477(f215(f188(f93(v0, f8(Str::lit("citation"))))));
            match m1616 {
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

pub fn f391(v0: Bytes) -> R<Bytes> {
    f478(v0.clone(), 0u64, {
        let a1617 = v0;
        length_bytes(a1617)
    }, true, Bytes::lit(&[]))
}

pub fn f392() -> Bytes {
    let a1618 = Str::lit("prismpm/authority-binding/1");
    utf8_encode(a1618)
}

pub fn f393(v0: Adt2, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt2> = f93(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f293(Str::lit("id"), f58(v0.clone(), f8(Str::lit("id"))))?, List::cons(f293(Str::lit("name"), f58(v0.clone(), f8(Str::lit("name"))))?, List::cons(f293(Str::lit("issuer"), f58(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f293(Str::lit("edition"), f58(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f293(Str::lit("canonical_id"), f58(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f293(Str::lit("source_role"), f58(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f293(Str::lit("retrieval_date"), f58(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f293(Str::lit("url"), f462(v2.clone(), f8(Str::lit("url"))))?, List::cons(f293(Str::lit("media_type"), f462(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f293(Str::lit("license"), f462(v2.clone(), f8(Str::lit("license"))))?, List::cons(f293(Str::lit("redistribution"), f462(v2, f8(Str::lit("redistribution"))))?, List::cons(f293(Str::lit("statement"), f58(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1619 = v1;
        match m1619 {
            None => {
                f7(List::cons(f293(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f293(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f293(Str::lit("artifact"), {
                    let (h1620, _) = v3;
                    h1620
                })?, List::cons(f293(Str::lit("checksum"), {
                    let (_, h1621) = v3;
                    h1621
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f479(f215(f188(f93(v0, f8(Str::lit("realized_by"))))))?, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f394(v0: Adt23) -> Bytes {
    let a1642 = f480(match v0.clone() {
        Adt23::C0(h1622, _, _, _, _, _, _, _) => {
            h1622
        }
    });
    let a1643 = {
        let a1640 = f480(match v0.clone() {
            Adt23::C0(_, h1623, _, _, _, _, _, _) => {
                h1623
            }
        });
        let a1641 = {
            let a1638 = f480(match v0.clone() {
                Adt23::C0(_, _, h1624, _, _, _, _, _) => {
                    h1624
                }
            });
            let a1639 = {
                let a1636 = f480(match v0.clone() {
                    Adt23::C0(_, _, _, h1625, _, _, _, _) => {
                        h1625
                    }
                });
                let a1637 = {
                    let a1634 = f480(match v0.clone() {
                        Adt23::C0(_, _, _, _, h1626, _, _, _) => {
                            h1626
                        }
                    });
                    let a1635 = {
                        let a1632 = f480(match v0.clone() {
                            Adt23::C0(_, _, _, _, _, h1627, _, _) => {
                                h1627
                            }
                        });
                        let a1633 = {
                            let a1630 = f480(match v0.clone() {
                                Adt23::C0(_, _, _, _, _, _, h1628, _) => {
                                    h1628
                                }
                            });
                            let a1631 = f480(match v0 {
                                Adt23::C0(_, _, _, _, _, _, _, h1629) => {
                                    h1629
                                }
                            });
                            append_bytes(a1630, a1631)
                        };
                        append_bytes(a1632, a1633)
                    };
                    append_bytes(a1634, a1635)
                };
                append_bytes(a1636, a1637)
            };
            append_bytes(a1638, a1639)
        };
        append_bytes(a1640, a1641)
    };
    append_bytes(a1642, a1643)
}

pub fn f395(v0: Bytes) -> R<Adt23> {
    let v1: u64 = {
        let a1644 = v0.clone();
        length_bytes(a1644)
    };
    let v2: u64 = {
        let a1650 = {
            let a1647 = {
                let a1645 = v1;
                let a1646 = 72u64;
                nat_add(a1645, a1646)?
            };
            let a1648 = 64u64;
            let a1649 = 0u64;
            nat_quot(a1647, a1648, a1649)
        };
        let a1651 = 64u64;
        nat_mul(a1650, a1651)?
    };
    let (_, h1652) = f481(Fn11::F482(v0, v1, v2), {
        let a1653 = v2;
        let a1654 = 64u64;
        let a1655 = 0u64;
        nat_quot(a1653, a1654, a1655)
    }, (0u64, f483()))?;
    Ok(h1652)
}

pub fn f396(p1658: List<List<Bytes>>, v1: Bytes, p1659: List<Bytes>) -> R<List<Bytes>> {
    let m1657 = (p1658, p1659);
    {
        let mut m1657 = m1657;
        loop {
            let step = {
                let (v0, v2) = m1657;
                let m1656 = v0;
                match m1656.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f484(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f398(f485(v3), f8(Str::lit("\u{a}"))), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1657 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: Bytes) -> R<Bytes> {
    Ok(f46(v0.clone(), 0u64, f318(v0)?))
}

pub fn f398(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1660 = v0;
    match m1660.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1661 = v3.clone();
            match m1661.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1664 = v2;
                    let a1665 = {
                        let a1662 = v1.clone();
                        let a1663 = f398(v3, v1);
                        append_bytes(a1662, a1663)
                    };
                    append_bytes(a1664, a1665)
                }
            }
        }
    }
}

pub fn f399(v0: List<Bytes>) -> Adt2 {
    let m1666 = v0;
    match m1666.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f399(v2)))
        }
    }
}

pub fn f400(v0: Adt2, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1667 = f93(v0, v1);
    match m1667 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v3) => {
            f188(f93(v3, v2))
        }
    }
}

pub fn f401(v0: Adt2, v1: Bytes) -> bool {
    let m1668 = f93(v0, v1);
    match m1668 {
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
            let a1670 = {
                let a1669 = v0;
                length_bytes(a1669)
            };
            let a1671 = 1u64;
            nat_sub(a1670, a1671)
        })
    } else {
        v0
    }
}

pub fn f403(v0: Adt2, v1: Bytes) -> Adt2 {
    let m1672 = v0;
    match m1672 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v8, r1673) => {
            let v9 = (*r1673).clone();
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
            Adt2::C6(f486(f60(), v2, v1))
        }
        Adt2::C7(r1674) => {
            let v6 = (*r1674).clone();
            Adt2::C7(std::rc::Rc::new(v6))
        }
        Adt2::C8(r1675) => {
            let v7 = (*r1675).clone();
            Adt2::C8(std::rc::Rc::new(v7))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r1676, r1677) => {
            let v15 = (*r1676).clone();
            let v16 = (*r1677).clone();
            Adt2::C10(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt2::C11(v17, r1678, r1679) => {
            let v18 = (*r1678).clone();
            let v19 = (*r1679).clone();
            Adt2::C11(v17, std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
    }
}

pub fn f404(v0: u64) -> R<Bytes> {
    f487(f488(), 10u64, v0)
}

pub fn f405(v0: bool) -> R<u64> {
    f407({
        let a1680 = f412(v0);
        let a1681 = 1u64;
        nat_sub(a1680, a1681)
    })
}

pub fn f406(v0: bool) -> R<u64> {
    f407(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f407(v0: u64) -> R<u64> {
    let m1682 = v0;
    if m1682 == 0 {
        Ok(1u64)
    } else {
        let v1 = m1682 - 1;
        let a1683 = 2u64;
        let a1684 = f407(v1)?;
        nat_mul(a1683, a1684)
    }
}

pub fn f408(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f409(v0: bool) -> R<u64> {
    let a1685 = f407(f408(v0))?;
    let a1686 = 1u64;
    Ok(nat_sub(a1685, a1686))
}

pub fn f410(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a1687 = Bytes::lit(&[0x2d]);
        let a1688 = v1;
        append_bytes(a1687, a1688)
    } else {
        v1
    }
}

pub fn f411(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f412(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f413(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        f489(f490(v2, f491(3u64, v0), 2u64)?)
    } else {
        let v3: Bytes = f489(f490(v2, f491(3u64, v0), 5u64)?)?;
        let v4: u64 = {
            let a1689 = v3.clone();
            length_bytes(a1689)
        };
        let v5: Bytes = {
            let a1690 = v2;
            let a1691 = v4;
            if nat_lt(a1690, a1691) {
                f46(v3.clone(), 0u64, {
                    let a1692 = v4;
                    let a1693 = v2;
                    nat_sub(a1692, a1693)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f492({
            let a1694 = v2;
            let a1695 = v4;
            if nat_lt(a1694, a1695) {
                f46(v3, {
                    let a1696 = v4;
                    let a1697 = v2;
                    nat_sub(a1696, a1697)
                }, v2)
            } else {
                let a1700 = f493({
                    let a1698 = v2;
                    let a1699 = v4;
                    nat_sub(a1698, a1699)
                });
                let a1701 = v3;
                append_bytes(a1700, a1701)
            }
        }, {
            let a1702 = v2;
            let a1703 = 1u64;
            nat_add(a1702, a1703)?
        })?;
        let a1705 = {
            let a1704 = v6.clone();
            length_bytes(a1704)
        };
        let a1706 = 0u64;
        if nat_eq(a1705, a1706) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f414(v0: Bytes, p1725: u64, p1726: u64, p1727: Bytes) -> R<Bytes> {
    let m1724 = (p1725, (p1726, p1727));
    {
        let mut m1724 = m1724;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1724;
                let m1707 = v2;
                if m1707 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1707 - 1;
                    let v5: u64 = f200(v0.clone(), v1, {
                        let a1711 = {
                            let a1709 = {
                                let a1708 = v0.clone();
                                length_bytes(a1708)
                            };
                            let a1710 = v1;
                            nat_sub(a1709, a1710)
                        };
                        let a1712 = 1u64;
                        nat_add(a1711, a1712)?
                    })?;
                    let v6: Bytes = {
                        let a1715 = v3;
                        let a1716 = f46(v0.clone(), v1, {
                            let a1713 = v5;
                            let a1714 = v1;
                            nat_sub(a1713, a1714)
                        });
                        append_bytes(a1715, a1716)
                    };
                    let m1719 = {
                        let a1717 = v0.clone();
                        let a1718 = v5;
                        index_bytes(a1717, a1718)
                    };
                    match m1719 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a1720 = v5;
                                let a1721 = 1u64;
                                nat_add(a1720, a1721)?
                            }, (v4, {
                                let a1722 = v6;
                                let a1723 = f494(v0.clone(), v5, v7)?;
                                append_bytes(a1722, a1723)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1724 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f415(v0: Fn5, p1731: List<(Bytes, Bytes)>, p1732: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1730 = (p1731, p1732);
    {
        let mut m1730 = m1730;
        loop {
            let step = {
                let (v1, v2) = m1730;
                let m1728 = v2;
                match m1728.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1729: Fn5 = v0.clone();
                            c1729.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1730 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f495(v0, v1)
}

pub fn f417(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f496(v0)
}

pub fn f418(v0: Fn6, p1736: List<Bytes>, p1737: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1735 = (p1736, p1737);
    {
        let mut m1735 = m1735;
        loop {
            let step = {
                let (v1, v2) = m1735;
                let m1733 = v2;
                match m1733.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1734: Fn6 = v0.clone();
                            c1734.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1735 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f419(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f497(v1)?, v0))
}

pub fn f420(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f498(v0)?, List::cons(f8(Str::lit(":")), List::cons(f498(v1)?, List::<Bytes>::nil())))))
}

pub fn f421(v0: Adt13) -> Bytes {
    let m1738 = v0;
    match m1738 {
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

pub fn f422(v0: Adt13) -> Bytes {
    let m1739 = v0;
    match m1739 {
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

pub fn f423(v0: Adt13) -> Bytes {
    let m1740 = v0;
    match m1740 {
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

pub fn f424(v0: Bytes, v1: u64) -> Option<u64> {
    let a1741 = 0u64;
    let a1742 = v1;
    if nat_lt(a1741, a1742) {
        if f499(v0, {
            let a1743 = v1;
            let a1744 = 1u64;
            nat_sub(a1743, a1744)
        }) {
            Some({
                let a1745 = v1;
                let a1746 = 1u64;
                nat_sub(a1745, a1746)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f425(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1747 = v2;
    let a1748 = v0;
    if nat_lt(a1747, a1748) {
        if f499(v1, v2) {
            Ok(Some({
                let a1749 = v2;
                let a1750 = 1u64;
                nat_add(a1749, a1750)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f426(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1753 = f46(v0.clone(), {
        let a1751 = v1;
        let a1752 = 2u64;
        nat_add(a1751, a1752)?
    }, 2u64);
    let a1754 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1753, a1754) {
        let m1757 = f500(f46(v0, {
            let a1755 = v1;
            let a1756 = 4u64;
            nat_add(a1755, a1756)?
        }, 2u64))?;
        match m1757 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f115(v2.clone(), 0u64)?;
                let a1758 = v3;
                let a1759 = 32u64;
                if nat_lt(a1758, a1759) {
                    if f501(v3) {
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

pub fn f427(v0: u64) -> Bytes {
    let a1760 = v0;
    let a1761 = 34u64;
    if nat_eq(a1760, a1761) {
        Bytes::lit(&[0x22])
    } else {
        let a1762 = v0;
        let a1763 = 92u64;
        if nat_eq(a1762, a1763) {
            Bytes::lit(&[0x5c])
        } else {
            let a1764 = v0;
            let a1765 = 98u64;
            if nat_eq(a1764, a1765) {
                Bytes::lit(&[0x08])
            } else {
                let a1766 = v0;
                let a1767 = 102u64;
                if nat_eq(a1766, a1767) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1768 = v0;
                    let a1769 = 110u64;
                    if nat_eq(a1768, a1769) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1770 = v0;
                        let a1771 = 114u64;
                        if nat_eq(a1770, a1771) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1772 = v0;
                            let a1773 = 116u64;
                            if nat_eq(a1772, a1773) {
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

pub fn f428(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f115(v0, v1)?;
    let a1774 = 47u64;
    let a1775 = v2;
    if if nat_lt(a1774, a1775) {
        let a1776 = v2;
        let a1777 = 58u64;
        nat_lt(a1776, a1777)
    } else {
        false
    } {
        Ok(Some({
            let a1778 = v1;
            let a1779 = 1u64;
            nat_add(a1778, a1779)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f429(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1780 = f115(v0, v1)?;
    let a1781 = 48u64;
    if nat_eq(a1780, a1781) {
        Ok(Some({
            let a1782 = v1;
            let a1783 = 1u64;
            nat_add(a1782, a1783)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f430(v0: Bytes) -> R<u64> {
    let (h1784, _) = f321(Fn3::F502(v0.clone()), {
        let a1785 = v0;
        length_bytes(a1785)
    }, 0u64)?;
    Ok(h1784)
}

pub fn f431(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1789 = {
        let a1787 = v0;
        let a1788 = {
            let (h1786, _) = v1;
            h1786
        };
        index_bytes(a1787, a1788)
    };
    match m1789 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1791 = {
                    let (h1790, _) = v1;
                    h1790
                };
                let a1792 = 1u64;
                nat_add(a1791, a1792)?
            }, {
                let a1796 = {
                    let a1794 = {
                        let (_, h1793) = v1;
                        h1793
                    };
                    let a1795 = 10u64;
                    nat_mul(a1794, a1795)?
                };
                let a1797 = f503(v2)?;
                nat_add(a1796, a1797)?
            })))
        }
    }
}

pub fn f432(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f504(v0, v1)
}

pub fn f433(v0: (Bytes, Bytes), p1800: List<(Bytes, Bytes)>) -> bool {
    let m1799 = p1800;
    {
        let mut m1799 = m1799;
        loop {
            let step = {
                let v1 = m1799;
                let m1798 = v1;
                match m1798.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f505(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
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

pub fn f434(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1801 = v1;
    match m1801.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f32({
                let (h1802, _) = v2;
                h1802
            }, v0.clone()) {
                List::cons({
                    let (_, h1803) = v2;
                    h1803
                }, f434(v0, v3))
            } else {
                f434(v0, v3)
            }
        }
    }
}

pub fn f435() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f436(v0: Adt2, v1: Bytes, v2: Bytes) -> bool {
    let m1804 = f93(v0, v1);
    match m1804 {
        None => {
            false
        }
        Some(v3) => {
            f221(v3, v2)
        }
    }
}

pub fn f437() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f438(v0: Bytes) -> bool {
    f41(v0, f8(Str::lit(".lex.tex")))
}

pub fn f439(v0: Bytes) -> R<bool> {
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
        let a1806 = {
            let a1805 = v1;
            length_bytes(a1805)
        };
        let a1807 = v2;
        nat_sub(a1806, a1807)
    });
    let v5: Bytes = if f85(v3.clone(), f8(Str::lit("("))) {
        let m1808 = f340(v3.clone(), f8(Str::lit(")")))?;
        match m1808 {
            None => {
                v3
            }
            Some(v4) => {
                f46(v3.clone(), {
                    let a1809 = v4;
                    let a1810 = 1u64;
                    nat_add(a1809, a1810)?
                }, {
                    let a1814 = {
                        let a1811 = v3;
                        length_bytes(a1811)
                    };
                    let a1815 = {
                        let a1812 = v4;
                        let a1813 = 1u64;
                        nat_add(a1812, a1813)?
                    };
                    nat_sub(a1814, a1815)
                })
            }
        }
    } else {
        v3
    };
    let a1816 = 0u64;
    let a1817 = v2;
    if nat_lt(a1816, a1817) {
        f506(v5)
    } else {
        Ok(false)
    }
}

pub fn f440(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f441(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1820 = {
            let a1818 = v4;
            let a1819 = v1;
            nat_add(a1818, a1819)?
        };
        let a1821 = v3;
        if nat_lt(a1820, a1821) {
            Ok(Some({
                let a1822 = v4;
                let a1823 = 1u64;
                nat_add(a1822, a1823)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f441(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1828 = {
        let a1825 = v0;
        let a1826 = v2;
        let a1827 = {
            let a1824 = v1.clone();
            length_bytes(a1824)
        };
        slice_bytes(a1825, a1826, a1827)
    };
    match m1828 {
        None => {
            false
        }
        Some(v3) => {
            let a1829 = v3;
            let a1830 = v1;
            equal(a1829, a1830)
        }
    }
}

pub fn f442(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1831 = v0;
    match m1831.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f442(v3, {
                let a1832 = v1;
                let a1833 = 1u64;
                nat_add(a1832, a1833)?
            })?))
        }
    }
}

pub fn f443(v0: Adt18, v1: List<u64>, v2: Adt21) -> bool {
    let m1834 = v2;
    match m1834 {
        Adt21::C0(v3, v4) => {
            if f449(v3, match v0.clone() {
                Adt18::C0(h1835, _, _, _) => {
                    h1835
                }
            }) {
                f507(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f449(v5, match v0 {
                Adt18::C0(_, _, h1836, _) => {
                    h1836
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f445(v7, v1) {
                f449(v8, match v0 {
                    Adt18::C0(_, _, _, h1837) => {
                        h1837
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f444(v0: Adt15, v1: Adt2) -> R<Adt21> {
    let m1838 = f93(v1.clone(), f8(Str::lit("std")));
    match m1838 {
        None => {
            let m1843 = f92(f93(v1.clone(), f8(Str::lit("req"))));
            match m1843 {
                None => {
                    let m1845 = f93(v1, f8(Str::lit("der")));
                    match m1845 {
                        None => {
                            Ok(f508(v0))
                        }
                        Some(v5) => {
                            let m1846 = f509(f58(v5.clone(), f8(Str::lit("rule"))));
                            match m1846 {
                                None => {
                                    Ok(f508(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f348(f215(f188(f93(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt15::C0(_, _, _, _, h1847) => {
                                            h1847
                                        }
                                    })?, f447(f58(v5, f8(Str::lit("check"))), match v0 {
                                        Adt15::C0(_, _, _, h1848, _) => {
                                            h1848
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f447(v4, match v0 {
                        Adt15::C0(_, _, h1844, _, _) => {
                            h1844
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1839 = f92(f93(v2.clone(), f8(Str::lit("row"))));
            match m1839 {
                None => {
                    Ok(Adt21::C0(f447(f58(v2, f8(Str::lit("authority"))), match v0 {
                        Adt15::C0(h1840, _, _, _, _) => {
                            h1840
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f447(f58(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt15::C0(h1841, _, _, _, _) => {
                            h1841
                        }
                    })?, Some(f447(v3, match v0 {
                        Adt15::C0(_, h1842, _, _, _) => {
                            h1842
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f445(p1851: List<u64>, v1: List<u64>) -> bool {
    let m1850 = p1851;
    {
        let mut m1850 = m1850;
        loop {
            let step = {
                let v0 = m1850;
                let m1849 = v0;
                match m1849.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f449(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1850 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f446(p1856: List<u64>) -> bool {
    let m1855 = p1856;
    {
        let mut m1855 = m1855;
        loop {
            let step = {
                let v0 = m1855;
                let m1852 = v0;
                match m1852.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1853 = v1;
                        let a1854 = 7u64;
                        if nat_le(a1853, a1854) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1855 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1857 = f510(v0, v1.clone(), 0u64)?;
    match m1857 {
        None => {
            let a1858 = v1;
            Ok(length_list(a1858))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f448(v0: Adt2) -> u64 {
    let m1859 = f511(f93(v0, f8(Str::lit("stratum"))));
    match m1859 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f449(v0: u64, p1864: List<u64>) -> bool {
    let m1863 = p1864;
    {
        let mut m1863 = m1863;
        loop {
            let step = {
                let v1 = m1863;
                let m1860 = v1;
                match m1860.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1861 = v2;
                        let a1862 = v0;
                        if nat_eq(a1861, a1862) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(v0: Fn7, p1868: List<Bytes>, p1869: List<(Bytes, ())>) -> List<Bytes> {
    let m1867 = (p1868, p1869);
    {
        let mut m1867 = m1867;
        loop {
            let step = {
                let (v1, v2) = m1867;
                let m1865 = v2;
                match m1865.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1866: Fn7 = v0.clone();
                            c1866.apply(v1, v3)
                        }, v4))
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

pub fn f451(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1870, _) = v1;
        h1870
    }, v0)
}

pub fn f452(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f512(Fn12::F513, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f453(v0: Fn8, p1874: List<List<(Bytes, ())>>, p1875: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1873 = (p1874, p1875);
    {
        let mut m1873 = m1873;
        loop {
            let step = {
                let (v1, v2) = m1873;
                let m1871 = v2;
                match m1871.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1872: Fn8 = v0.clone();
                            c1872.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1873 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f455(v0: Fn9, p1880: u64, p1881: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1879 = (p1880, p1881);
    {
        let mut m1879 = m1879;
        loop {
            let step = {
                let (v1, v2) = m1879;
                let m1876 = v1;
                if m1876 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1876 - 1;
                    let m1878 = {
                        let c1877: Fn9 = v0.clone();
                        c1877.apply(v2.clone())?
                    };
                    match m1878 {
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
                Ok(next) => { m1879 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f514(v0)
}

pub fn f457(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f515(Fn13::F516, List::<(Bytes, ())>::nil(), v0)
}

pub fn f458(v0: Fn10, p1885: List<(Bytes, ())>, p1886: List<Bytes>) -> List<(Bytes, ())> {
    let m1884 = (p1885, p1886);
    {
        let mut m1884 = m1884;
        loop {
            let step = {
                let (v1, v2) = m1884;
                let m1882 = v2;
                match m1882.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1883: Fn10 = v0.clone();
                            c1883.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1884 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f460(p1891: List<(List<Bytes>, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m1890 = p1891;
    {
        let mut m1890 = m1890;
        loop {
            let step = {
                let v0 = m1890;
                let m1887 = v0;
                match m1887.uncons() {
                    None => {
                        Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f52(v1.clone(), {
                            let (h1888, _) = v2;
                            h1888
                        }) {
                            let (_, h1889) = v2;
                            Err::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(h1889)
                        } else {
                            Ok::<List<(List<Bytes>, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1890 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: Bytes, p1917: u64, p1918: u64, p1919: Bytes) -> R<Bytes> {
    let m1916 = (p1917, (p1918, p1919));
    {
        let mut m1916 = m1916;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1916;
                let m1892 = v2;
                if m1892 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1892 - 1;
                    let m1895 = {
                        let a1893 = v0.clone();
                        let a1894 = v1;
                        index_bytes(a1893, a1894)
                    };
                    match m1895 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f72(v5)?;
                            if f203({
                                let a1896 = v6;
                                let a1897 = 34u64;
                                nat_eq(a1896, a1897)
                            }, {
                                let a1898 = v6;
                                let a1899 = 92u64;
                                nat_eq(a1898, a1899)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1900 = v1;
                                    let a1901 = 1u64;
                                    nat_add(a1900, a1901)?
                                }, (v4, {
                                    let a1904 = v3;
                                    let a1905 = {
                                        let a1902 = f8(Str::lit("\\"));
                                        let a1903 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1902, a1903)
                                    };
                                    append_bytes(a1904, a1905)
                                })))
                            } else {
                                let a1906 = v6;
                                let a1907 = 10u64;
                                if nat_eq(a1906, a1907) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1908 = v1;
                                        let a1909 = 1u64;
                                        nat_add(a1908, a1909)?
                                    }, (v4, {
                                        let a1910 = v3;
                                        let a1911 = f8(Str::lit("\\n"));
                                        append_bytes(a1910, a1911)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1912 = v1;
                                        let a1913 = 1u64;
                                        nat_add(a1912, a1913)?
                                    }, (v4, {
                                        let a1914 = v3;
                                        let a1915 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a1914, a1915)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Option<Adt2>, v1: Bytes) -> Bytes {
    let m1920 = v0;
    match m1920 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f58(v2, v1)
        }
    }
}

pub fn f463(v0: List<Adt2>) -> List<Bytes> {
    let m1921 = v0;
    match m1921.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f517(f58(v1.clone(), f8(Str::lit("authority")))), List::cons(f518(f58(v1, f8(Str::lit("authority")))), f463(v2)))
        }
    }
}

pub fn f464(v0: List<Bytes>) -> R<List<Adt2>> {
    let m1922 = f297(v0.clone())?;
    match m1922 {
        None => {
            Ok(f286(f285(v0)?))
        }
        Some(_) => {
            Ok(List::<Adt2>::nil())
        }
    }
}

pub fn f465(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f95(f64(v0.clone(), v1.clone()))?;
    let m1923 = f297(v3.clone())?;
    match m1923 {
        None => {
            let m1924 = f285(v3.clone())?;
            match m1924 {
                None => {
                    f521(v0, v1, v3)
                }
                Some(v5) => {
                    f520(v0, v1, v2, v5)
                }
            }
        }
        Some(v4) => {
            f519(v0, v1, v4)
        }
    }
}

pub fn f466(v0: List<Bytes>) -> List<Bytes> {
    let m1925 = v0;
    match m1925.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1927 = {
                    let a1926 = v1.clone();
                    length_bytes(a1926)
                };
                let a1928 = 0u64;
                if nat_eq(a1927, a1928) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
                }
            }, f466(v2))
        }
    }
}

pub fn f467(v0: Bytes, v1: u64) -> R<u64> {
    let m1931 = {
        let a1929 = v0;
        let a1930 = v1;
        index_bytes(a1929, a1930)
    };
    match m1931 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            f72(v2)
        }
    }
}

pub fn f468(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1935 = {
        let a1932 = v1;
        let a1933 = v2;
        let a1934 = 1u64;
        slice_bytes(a1932, a1933, a1934)
    };
    match m1935 {
        None => {
            v0
        }
        Some(v3) => {
            let a1936 = v0;
            let a1937 = v3;
            append_bytes(a1936, a1937)
        }
    }
}

pub fn f469(p1940: List<Bytes>, p1941: List<Bytes>) -> List<Bytes> {
    let m1939 = (p1940, p1941);
    {
        let mut m1939 = m1939;
        loop {
            let step = {
                let (v0, v1) = m1939;
                let m1938 = v0;
                match m1938.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1939 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f470(v0: List<Bytes>) -> Bytes {
    let m1942 = v0;
    match m1942.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f471(p1945: List<List<Bytes>>, v1: Bytes, v2: Bytes) -> R<bool> {
    let m1944 = p1945;
    {
        let mut m1944 = m1944;
        loop {
            let step = {
                let v0 = m1944;
                let m1943 = v0;
                match m1943.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        if f117(f85(f522(v3.clone(), 0u64), v1.clone()), f52(v2.clone(), f44(f522(v3, 4u64), 32u8)?)) {
                            Err::<List<List<Bytes>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<Bytes>>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1944 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: Bytes) -> R<u64> {
    let m1948 = {
        let a1946 = v0;
        let a1947 = 0u64;
        index_bytes(a1946, a1947)
    };
    match m1948 {
        None => {
            Ok(256u64)
        }
        Some(v1) => {
            f72(v1)
        }
    }
}

pub fn f473(v0: Adt2) -> Bytes {
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

pub fn f474(v0: Bytes) -> R<bool> {
    let m1950 = {
        let a1949 = v0.clone();
        length_bytes(a1949)
    };
    if m1950 == 0 {
        Ok(true)
    } else {
        let v1 = m1950 - 1;
        let m1953 = {
            let a1951 = v0;
            let a1952 = v1;
            index_bytes(a1951, a1952)
        };
        match m1953 {
            None => {
                Ok(true)
            }
            Some(v2) => {
                Ok(f523({
                    let v3: u8 = v2;
                    let a1998 = {
                        let a1974 = {
                            let a1962 = {
                                let a1956 = {
                                    let a1954 = v3;
                                    let a1955 = 128u8;
                                    fixed_u8::bit_and(a1954, a1955)
                                };
                                let a1957 = 128u8;
                                if equal(a1956, a1957) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1963 = {
                                let a1960 = {
                                    let a1958 = v3;
                                    let a1959 = 64u8;
                                    fixed_u8::bit_and(a1958, a1959)
                                };
                                let a1961 = 64u8;
                                if equal(a1960, a1961) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1962, a1963)?
                        };
                        let a1975 = {
                            let a1972 = {
                                let a1966 = {
                                    let a1964 = v3;
                                    let a1965 = 32u8;
                                    fixed_u8::bit_and(a1964, a1965)
                                };
                                let a1967 = 32u8;
                                if equal(a1966, a1967) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1973 = {
                                let a1970 = {
                                    let a1968 = v3;
                                    let a1969 = 16u8;
                                    fixed_u8::bit_and(a1968, a1969)
                                };
                                let a1971 = 16u8;
                                if equal(a1970, a1971) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1972, a1973)?
                        };
                        nat_add(a1974, a1975)?
                    };
                    let a1999 = {
                        let a1996 = {
                            let a1984 = {
                                let a1978 = {
                                    let a1976 = v3;
                                    let a1977 = 8u8;
                                    fixed_u8::bit_and(a1976, a1977)
                                };
                                let a1979 = 8u8;
                                if equal(a1978, a1979) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1985 = {
                                let a1982 = {
                                    let a1980 = v3;
                                    let a1981 = 4u8;
                                    fixed_u8::bit_and(a1980, a1981)
                                };
                                let a1983 = 4u8;
                                if equal(a1982, a1983) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1984, a1985)?
                        };
                        let a1997 = {
                            let a1994 = {
                                let a1988 = {
                                    let a1986 = v3;
                                    let a1987 = 2u8;
                                    fixed_u8::bit_and(a1986, a1987)
                                };
                                let a1989 = 2u8;
                                if equal(a1988, a1989) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1995 = {
                                let a1992 = {
                                    let a1990 = v3;
                                    let a1991 = 1u8;
                                    fixed_u8::bit_and(a1990, a1991)
                                };
                                let a1993 = 1u8;
                                if equal(a1992, a1993) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1994, a1995)?
                        };
                        nat_add(a1996, a1997)?
                    };
                    nat_add(a1998, a1999)?
                }))
            }
        }
    }
}

pub fn f475(v0: Adt2, v1: Bytes) -> bool {
    let a2001 = {
        let a2000 = f58(v0, v1);
        length_bytes(a2000)
    };
    let a2002 = 0u64;
    !nat_eq(a2001, a2002)
}

pub fn f476(v0: Adt2, v1: Bytes) -> bool {
    let m2003 = f93(v0, v1);
    match m2003 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f477(v0: List<Bytes>) -> Option<Adt2> {
    let m2004 = v0;
    match m2004.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m2005 = v2;
                match m2005.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m2006 = v2;
                    match m2006.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m2007 = v6;
                                match m2007.uncons() {
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
                        let m2008 = v2;
                        match m2008.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m2009 = v10;
                                match m2009.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C11(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C6(v11)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f399(v12)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C6(v9)), std::rc::Rc::new(Adt2::C9))))))))), std::rc::Rc::new(Adt2::C9)))
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

pub fn f478(v0: Bytes, p2030: u64, p2031: u64, p2032: bool, p2033: Bytes) -> R<Bytes> {
    let m2029 = (p2030, (p2031, (p2032, p2033)));
    {
        let mut m2029 = m2029;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m2029;
                let m2010 = v2;
                if m2010 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m2010 - 1;
                    let m2013 = {
                        let a2011 = v0.clone();
                        let a2012 = v1;
                        index_bytes(a2011, a2012)
                    };
                    match m2013 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f72(v6)?;
                            if f524(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a2014 = v1;
                                    let a2015 = 1u64;
                                    nat_add(a2014, a2015)?
                                }, (v5, (true, v4))))
                            } else {
                                if f117(f525(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a2016 = v1;
                                        let a2017 = 1u64;
                                        nat_add(a2016, a2017)?
                                    }, (v5, (false, {
                                        let a2021 = v4;
                                        let a2022 = f46({
                                            let a2018 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a2018)
                                        }, {
                                            let a2019 = v7;
                                            let a2020 = 97u64;
                                            nat_sub(a2019, a2020)
                                        }, 1u64);
                                        append_bytes(a2021, a2022)
                                    }))))
                                } else {
                                    if f203(f525(v7), f203(f526(v7), f527(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2023 = v1;
                                            let a2024 = 1u64;
                                            nat_add(a2023, a2024)?
                                        }, (v5, (false, {
                                            let a2025 = v4;
                                            let a2026 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a2025, a2026)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a2027 = v1;
                                            let a2028 = 1u64;
                                            nat_add(a2027, a2028)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2029 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f398(f528(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f480(v0: u32) -> Bytes {
    let a2062 = f529({
        let a2034 = f530(v0, 28u32);
        let a2035 = 15u32;
        fixed_u32::bit_and(a2034, a2035)
    });
    let a2063 = {
        let a2060 = f529({
            let a2036 = f530(v0, 24u32);
            let a2037 = 15u32;
            fixed_u32::bit_and(a2036, a2037)
        });
        let a2061 = {
            let a2058 = f529({
                let a2038 = f530(v0, 20u32);
                let a2039 = 15u32;
                fixed_u32::bit_and(a2038, a2039)
            });
            let a2059 = {
                let a2056 = f529({
                    let a2040 = f530(v0, 16u32);
                    let a2041 = 15u32;
                    fixed_u32::bit_and(a2040, a2041)
                });
                let a2057 = {
                    let a2054 = f529({
                        let a2042 = f530(v0, 12u32);
                        let a2043 = 15u32;
                        fixed_u32::bit_and(a2042, a2043)
                    });
                    let a2055 = {
                        let a2052 = f529({
                            let a2044 = f530(v0, 8u32);
                            let a2045 = 15u32;
                            fixed_u32::bit_and(a2044, a2045)
                        });
                        let a2053 = {
                            let a2050 = f529({
                                let a2046 = f530(v0, 4u32);
                                let a2047 = 15u32;
                                fixed_u32::bit_and(a2046, a2047)
                            });
                            let a2051 = f529({
                                let a2048 = v0;
                                let a2049 = 15u32;
                                fixed_u32::bit_and(a2048, a2049)
                            });
                            append_bytes(a2050, a2051)
                        };
                        append_bytes(a2052, a2053)
                    };
                    append_bytes(a2054, a2055)
                };
                append_bytes(a2056, a2057)
            };
            append_bytes(a2058, a2059)
        };
        append_bytes(a2060, a2061)
    };
    append_bytes(a2062, a2063)
}

pub fn f481(v0: Fn11, p2067: u64, p2068: (u64, Adt23)) -> R<(u64, Adt23)> {
    let m2066 = (p2067, p2068);
    {
        let mut m2066 = m2066;
        loop {
            let step = {
                let (v1, v2) = m2066;
                let m2064 = v1;
                if m2064 == 0 {
                    Err::<(u64, (u64, Adt23)), R<(u64, Adt23)>>(Ok(v2))
                } else {
                    let v3 = m2064 - 1;
                    Ok::<(u64, (u64, Adt23)), R<(u64, Adt23)>>((v3, {
                        let c2065: Fn11 = v0.clone();
                        c2065.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f482(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt23)) -> R<(u64, Adt23)> {
    Ok(({
        let a2070 = {
            let (h2069, _) = v3.clone();
            h2069
        };
        let a2071 = 64u64;
        nat_add(a2070, a2071)?
    }, f531({
        let (_, h2072) = v3;
        h2072
    }, f532(v0, v1, v2, {
        let (h2073, _) = v3;
        h2073
    })?)?))
}

pub fn f483() -> Adt23 {
    Adt23::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f484(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m2074 = f44(v1, 34u8)?;
    match m2074.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m2075 = v3;
            match m2075.uncons() {
                None => {
                    Ok(f533(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f485(v0: List<Bytes>) -> List<Bytes> {
    let m2076 = v0;
    match m2076.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("authority = []"))) {
                f485(v2)
            } else {
                List::cons(v1, f485(v2))
            }
        }
    }
}

pub fn f486(p2079: List<Adt13>, v1: Bytes, v2: Bytes) -> Bytes {
    let m2078 = p2079;
    {
        let mut m2078 = m2078;
        loop {
            let step = {
                let v0 = m2078;
                let m2077 = v0;
                match m2077.uncons() {
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
                Ok(next) => { m2078 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f487(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a2080 = v2;
    let a2081 = 0u64;
    if nat_eq(a2080, a2081) {
        Ok(f534(v0, 0u64))
    } else {
        let (_, h2082) = {
            let (h2083, _) = f535(Fn14::F536(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h2083
        };
        Ok(h2082)
    }
}

pub fn f488() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f489(v0: List<u64>) -> R<Bytes> {
    let m2084 = f537(v0, List::<u64>::nil());
    match m2084.uncons() {
        None => {
            Ok(Bytes::lit(&[0x30]))
        }
        Some((v1, v2)) => {
            let a2085 = f404(v1)?;
            let a2086 = f538(v2)?;
            Ok(append_bytes(a2085, a2086))
        }
    }
}

pub fn f490(p2089: u64, p2090: List<u64>, v2: u64) -> R<List<u64>> {
    let m2088 = (p2089, p2090);
    {
        let mut m2088 = m2088;
        loop {
            let step = {
                let (v0, v1) = m2088;
                let m2087 = v0;
                if m2087 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m2087 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f539(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m2088 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f491(v0: u64, v1: u64) -> List<u64> {
    let m2091 = v0;
    if m2091 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m2091 - 1;
        let a2092 = v1;
        let a2093 = 0u64;
        if nat_eq(a2092, a2093) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a2094 = v1;
                let a2095 = f540();
                let a2096 = 0u64;
                nat_rem(a2094, a2095, a2096)
            }, f491(v2, {
                let a2097 = v1;
                let a2098 = f540();
                let a2099 = 0u64;
                nat_quot(a2097, a2098, a2099)
            }))
        }
    }
}

pub fn f492(p2110: Bytes, p2111: u64) -> R<Bytes> {
    let m2109 = (p2110, p2111);
    {
        let mut m2109 = m2109;
        loop {
            let step = {
                let (v0, v1) = m2109;
                let m2100 = v1;
                if m2100 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m2100 - 1;
                    let a2104 = f115(v0.clone(), {
                        let a2102 = {
                            let a2101 = v0.clone();
                            length_bytes(a2101)
                        };
                        let a2103 = 1u64;
                        nat_sub(a2102, a2103)
                    })?;
                    let a2105 = 48u64;
                    if nat_eq(a2104, a2105) {
                        Ok::<(Bytes, u64), R<Bytes>>((f46(v0.clone(), 0u64, {
                            let a2107 = {
                                let a2106 = v0;
                                length_bytes(a2106)
                            };
                            let a2108 = 1u64;
                            nat_sub(a2107, a2108)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m2109 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(v0: u64) -> Bytes {
    let m2112 = v0;
    if m2112 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2112 - 1;
        let a2113 = Bytes::lit(&[0x30]);
        let a2114 = f493(v1);
        append_bytes(a2113, a2114)
    }
}

pub fn f494(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2159 = {
            let a2135 = {
                let a2123 = {
                    let a2117 = {
                        let a2115 = v3;
                        let a2116 = 128u8;
                        fixed_u8::bit_and(a2115, a2116)
                    };
                    let a2118 = 128u8;
                    if equal(a2117, a2118) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2124 = {
                    let a2121 = {
                        let a2119 = v3;
                        let a2120 = 64u8;
                        fixed_u8::bit_and(a2119, a2120)
                    };
                    let a2122 = 64u8;
                    if equal(a2121, a2122) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2123, a2124)?
            };
            let a2136 = {
                let a2133 = {
                    let a2127 = {
                        let a2125 = v3;
                        let a2126 = 32u8;
                        fixed_u8::bit_and(a2125, a2126)
                    };
                    let a2128 = 32u8;
                    if equal(a2127, a2128) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2134 = {
                    let a2131 = {
                        let a2129 = v3;
                        let a2130 = 16u8;
                        fixed_u8::bit_and(a2129, a2130)
                    };
                    let a2132 = 16u8;
                    if equal(a2131, a2132) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2133, a2134)?
            };
            nat_add(a2135, a2136)?
        };
        let a2160 = {
            let a2157 = {
                let a2145 = {
                    let a2139 = {
                        let a2137 = v3;
                        let a2138 = 8u8;
                        fixed_u8::bit_and(a2137, a2138)
                    };
                    let a2140 = 8u8;
                    if equal(a2139, a2140) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2146 = {
                    let a2143 = {
                        let a2141 = v3;
                        let a2142 = 4u8;
                        fixed_u8::bit_and(a2141, a2142)
                    };
                    let a2144 = 4u8;
                    if equal(a2143, a2144) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2145, a2146)?
            };
            let a2158 = {
                let a2155 = {
                    let a2149 = {
                        let a2147 = v3;
                        let a2148 = 2u8;
                        fixed_u8::bit_and(a2147, a2148)
                    };
                    let a2150 = 2u8;
                    if equal(a2149, a2150) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2156 = {
                    let a2153 = {
                        let a2151 = v3;
                        let a2152 = 1u8;
                        fixed_u8::bit_and(a2151, a2152)
                    };
                    let a2154 = 1u8;
                    if equal(a2153, a2154) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2155, a2156)?
            };
            nat_add(a2157, a2158)?
        };
        nat_add(a2159, a2160)?
    };
    let a2161 = v4;
    let a2162 = 34u64;
    if nat_eq(a2161, a2162) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2163 = v4;
        let a2164 = 92u64;
        if nat_eq(a2163, a2164) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2165 = v4;
            let a2166 = 8u64;
            if nat_eq(a2165, a2166) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2167 = v4;
                let a2168 = 12u64;
                if nat_eq(a2167, a2168) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2169 = v4;
                    let a2170 = 10u64;
                    if nat_eq(a2169, a2170) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2171 = v4;
                        let a2172 = 13u64;
                        if nat_eq(a2171, a2172) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2173 = v4;
                            let a2174 = 9u64;
                            if nat_eq(a2173, a2174) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2175 = v4;
                                let a2176 = 32u64;
                                if nat_lt(a2175, a2176) {
                                    let a2177 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2178 = f541(v2)?;
                                    Ok(append_bytes(a2177, a2178))
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

pub fn f495(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2179 = v0.clone();
    match m2179.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2182 = {
                let (h2180, _) = v2;
                h2180
            };
            let a2183 = {
                let (h2181, _) = v1.clone();
                h2181
            };
            if equal(a2182, a2183) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f496(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f542(f543(Fn15::F544, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2188 = {
        let (h2184, _) = f545(Fn16::F546, {
            let a2186 = {
                let a2185 = v0;
                length_list(a2185)
            };
            let a2187 = 1u64;
            nat_add(a2186, a2187)?
        }, v1)?;
        h2184
    };
    match m2188.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f497(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2193 = {
        let a2190 = f309({
            let (h2189, _) = v0;
            h2189
        })?;
        let a2191 = Bytes::lit(&[0x3a]);
        append_bytes(a2190, a2191)
    };
    let a2194 = {
        let (_, h2192) = v0;
        h2192
    };
    Ok(append_bytes(a2193, a2194))
}

pub fn f498(v0: Bytes) -> R<Bytes> {
    let a2200 = {
        let a2198 = Bytes::lit(&[0x22]);
        let a2199 = {
            let (_, h2195) = {
                let (h2196, _) = f535(Fn14::F547(v0.clone()), {
                    let a2197 = v0;
                    length_bytes(a2197)
                }, (0u64, Bytes::lit(&[])))?;
                h2196
            };
            h2195
        };
        append_bytes(a2198, a2199)
    };
    let a2201 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2200, a2201))
}

pub fn f499(v0: Bytes, v1: u64) -> bool {
    let m2202 = f208(v0, v1);
    match m2202 {
        None => {
            false
        }
        Some(v2) => {
            f548(v2)
        }
    }
}

pub fn f500(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2203 = v0.clone();
        length_bytes(a2203)
    };
    let a2207 = {
        let a2204 = v1;
        let a2205 = 2u64;
        let a2206 = 0u64;
        nat_rem(a2204, a2205, a2206)
    };
    let a2208 = 0u64;
    if nat_eq(a2207, a2208) {
        let (_, h2209) = {
            let (h2210, _) = f549(Fn17::F550(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2210
        };
        Ok(h2209)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f501(v0: u64) -> bool {
    let a2211 = v0;
    let a2212 = 8u64;
    if nat_eq(a2211, a2212) {
        true
    } else {
        let a2213 = v0;
        let a2214 = 9u64;
        if nat_eq(a2213, a2214) {
            true
        } else {
            let a2215 = v0;
            let a2216 = 10u64;
            if nat_eq(a2215, a2216) {
                true
            } else {
                let a2217 = v0;
                let a2218 = 12u64;
                if nat_eq(a2217, a2218) {
                    true
                } else {
                    let a2219 = v0;
                    let a2220 = 13u64;
                    nat_eq(a2219, a2220)
                }
            }
        }
    }
}

pub fn f502(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2223 = {
        let a2221 = v0;
        let a2222 = v1;
        index_bytes(a2221, a2222)
    };
    match m2223 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f551(v2)? {
                Ok(Some({
                    let a2224 = v1;
                    let a2225 = 1u64;
                    nat_add(a2224, a2225)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f503(v0: u8) -> R<u64> {
    let a2272 = {
        let v1: u8 = v0;
        let a2270 = {
            let a2246 = {
                let a2234 = {
                    let a2228 = {
                        let a2226 = v1;
                        let a2227 = 128u8;
                        fixed_u8::bit_and(a2226, a2227)
                    };
                    let a2229 = 128u8;
                    if equal(a2228, a2229) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2235 = {
                    let a2232 = {
                        let a2230 = v1;
                        let a2231 = 64u8;
                        fixed_u8::bit_and(a2230, a2231)
                    };
                    let a2233 = 64u8;
                    if equal(a2232, a2233) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2234, a2235)?
            };
            let a2247 = {
                let a2244 = {
                    let a2238 = {
                        let a2236 = v1;
                        let a2237 = 32u8;
                        fixed_u8::bit_and(a2236, a2237)
                    };
                    let a2239 = 32u8;
                    if equal(a2238, a2239) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2245 = {
                    let a2242 = {
                        let a2240 = v1;
                        let a2241 = 16u8;
                        fixed_u8::bit_and(a2240, a2241)
                    };
                    let a2243 = 16u8;
                    if equal(a2242, a2243) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2244, a2245)?
            };
            nat_add(a2246, a2247)?
        };
        let a2271 = {
            let a2268 = {
                let a2256 = {
                    let a2250 = {
                        let a2248 = v1;
                        let a2249 = 8u8;
                        fixed_u8::bit_and(a2248, a2249)
                    };
                    let a2251 = 8u8;
                    if equal(a2250, a2251) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2257 = {
                    let a2254 = {
                        let a2252 = v1;
                        let a2253 = 4u8;
                        fixed_u8::bit_and(a2252, a2253)
                    };
                    let a2255 = 4u8;
                    if equal(a2254, a2255) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2256, a2257)?
            };
            let a2269 = {
                let a2266 = {
                    let a2260 = {
                        let a2258 = v1;
                        let a2259 = 2u8;
                        fixed_u8::bit_and(a2258, a2259)
                    };
                    let a2261 = 2u8;
                    if equal(a2260, a2261) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2267 = {
                    let a2264 = {
                        let a2262 = v1;
                        let a2263 = 1u8;
                        fixed_u8::bit_and(a2262, a2263)
                    };
                    let a2265 = 1u8;
                    if equal(a2264, a2265) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2266, a2267)?
            };
            nat_add(a2268, a2269)?
        };
        nat_add(a2270, a2271)?
    };
    let a2273 = 48u64;
    Ok(nat_sub(a2272, a2273))
}

pub fn f504(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2274 = v0.clone();
        length_bytes(a2274)
    };
    let a2275 = v1;
    let a2276 = v2;
    if if nat_lt(a2275, a2276) {
        let a2281 = f115(v0, {
            let a2279 = v2;
            let a2280 = {
                let a2277 = v1;
                let a2278 = 1u64;
                nat_add(a2277, a2278)?
            };
            nat_sub(a2279, a2280)
        })?;
        let a2282 = 48u64;
        nat_eq(a2281, a2282)
    } else {
        false
    } {
        Ok(Some({
            let a2283 = v1;
            let a2284 = 1u64;
            nat_add(a2283, a2284)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f505(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    f117(f32({
        let (h2285, _) = v0;
        h2285
    }, {
        let (h2286, _) = v1;
        h2286
    }), f32({
        let (_, h2287) = v0;
        h2287
    }, {
        let (_, h2288) = v1;
        h2288
    }))
}

pub fn f506(v0: Bytes) -> R<bool> {
    let m2289 = f340(v0.clone(), f8(Str::lit("&&")))?;
    if match m2289 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        Ok(true)
    } else {
        let m2290 = f340(v0, f8(Str::lit("||")))?;
        match m2290 {
            None => {
                Ok(false)
            }
            Some(_) => {
                Ok(true)
            }
        }
    }
}

pub fn f507(v0: Adt18, v1: Option<u64>) -> bool {
    let m2291 = v1;
    match m2291 {
        None => {
            true
        }
        Some(v2) => {
            f449(v2, match v0 {
                Adt18::C0(_, h2292, _, _) => {
                    h2292
                }
            })
        }
    }
}

pub fn f508(v0: Adt15) -> Adt21 {
    Adt21::C1({
        let a2294 = match v0 {
            Adt15::C0(_, _, h2293, _, _) => {
                h2293
            }
        };
        length_list(a2294)
    })
}

pub fn f509(v0: Bytes) -> Option<Adt22> {
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

pub fn f510(v0: Bytes, p2299: List<Bytes>, p2300: u64) -> R<Option<u64>> {
    let m2298 = (p2299, p2300);
    {
        let mut m2298 = m2298;
        loop {
            let step = {
                let (v1, v2) = m2298;
                let m2295 = v1;
                match m2295.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a2296 = v2;
                                let a2297 = 1u64;
                                nat_add(a2296, a2297)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2298 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f511(v0: Option<Adt2>) -> Option<u64> {
    let m2301 = v0;
    match m2301 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2302 = v1;
            match m2302 {
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

pub fn f512(v0: Fn12, p2306: List<List<(Bytes, ())>>, p2307: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2305 = (p2306, p2307);
    {
        let mut m2305 = m2305;
        loop {
            let step = {
                let (v1, v2) = m2305;
                let m2303 = v2;
                match m2303.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2304: Fn12 = v0.clone();
                            c2304.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2305 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f514(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2309 = {
        let a2308 = v0.clone();
        length_list(a2308)
    };
    let a2310 = 1u64;
    if nat_le(a2309, a2310) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f552(v0)?))
    }
}

pub fn f515(v0: Fn13, p2314: List<(Bytes, ())>, p2315: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2313 = (p2314, p2315);
    {
        let mut m2313 = m2313;
        loop {
            let step = {
                let (v1, v2) = m2313;
                let m2311 = v2;
                match m2311.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2312: Fn13 = v0.clone();
                            c2312.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f517(v0: Bytes) -> Bytes {
    f7(List::cons(f139(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f518(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f519(v0: Adt1, v1: Bytes, v2: List<Adt2>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f142(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f62(v3.clone(), f142(f96(v0, f153())?, f8(Str::lit("at"))));
    let a2317 = {
        let a2316 = v4.clone();
        length_list(a2316)
    };
    let a2318 = 0u64;
    if nat_eq(a2317, a2318) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2321 = {
            let a2319 = v4;
            length_list(a2319)
        };
        let a2322 = {
            let a2320 = v3;
            length_list(a2320)
        };
        if nat_eq(a2321, a2322) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f553(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f520(v0: Adt1, v1: Bytes, v2: List<Bytes>, v3: List<Adt2>) -> R<(bool, Bytes)> {
    let m2323 = v3;
    match m2323.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v4, v5)) => {
            let v6: Bytes = f58(v4.clone(), f8(Str::lit("authority")));
            let v7: Bytes = f58(v4.clone(), f8(Str::lit("row")));
            let v8: Bytes = f58(v4.clone(), f8(Str::lit("construct")));
            if f203(f554(v0.clone(), v6.clone())?, f52(v6.clone(), v2.clone())) {
                f520(v0, v1, v2, v5)
            } else {
                if f51(f67(v0.clone(), f518(v6.clone()))) {
                    let v9: (bool, Bytes) = f520(v0, v1.clone(), v2, v5)?;
                    let (h2324, _) = v9.clone();
                    if h2324 {
                        Ok(v9)
                    } else {
                        Ok((false, {
                            let a2326 = f553(v1, f7(List::cons(v6, List::cons(f8(Str::lit(" is not bound yet")), List::<Bytes>::nil()))));
                            let a2327 = {
                                let (_, h2325) = v9;
                                h2325
                            };
                            append_bytes(a2326, a2327)
                        }))
                    }
                } else {
                    if f117(f51(f32(v8.clone(), f8(Str::lit("none")))), f51(f555(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, {
                        let a2328 = f8(Str::lit("Catalog."));
                        let a2329 = v8.clone();
                        append_bytes(a2328, a2329)
                    }, v7.clone()))) {
                        let v10: (bool, Bytes) = f520(v0, v1.clone(), v2, v5)?;
                        let (h2330, _) = v10.clone();
                        if h2330 {
                            Ok(v10)
                        } else {
                            Ok((false, {
                                let a2332 = f553(v1, f7(List::cons(f8(Str::lit("Catalog.")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v7, List::cons(f8(Str::lit(" yet")), List::<Bytes>::nil())))))));
                                let a2333 = {
                                    let (_, h2331) = v10;
                                    h2331
                                };
                                append_bytes(a2332, a2333)
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

pub fn f521(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt2> = f286(f385(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt2> = f286(f385(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f125(f142(v3.clone(), f8(Str::lit("id"))), f142(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f142(f96(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f62(v5.clone(), f125(v6.clone(), f142(f96(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2335 = {
        let a2334 = v7.clone();
        length_list(a2334)
    };
    let a2336 = 0u64;
    if nat_eq(a2335, a2336) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f51({
            let a2339 = {
                let a2337 = v7;
                length_list(a2337)
            };
            let a2340 = {
                let a2338 = v5;
                length_list(a2338)
            };
            nat_eq(a2339, a2340)
        }) {
            Ok((false, f553(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f145(f125(f62(f556(v4.clone()), f63(f64(v0, f8(Str::lit("model/authorities.toml"))))?), f62(f557(v4), f125(v6, f142(v3, f8(Str::lit("id")))))))?;
            let a2342 = {
                let a2341 = v8.clone();
                length_list(a2341)
            };
            let a2343 = 0u64;
            if nat_eq(a2342, a2343) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f553(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f122(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f522(p2350: List<Bytes>, p2351: u64) -> Bytes {
    let m2349 = (p2350, p2351);
    {
        let mut m2349 = m2349;
        loop {
            let step = {
                let (v0, v1) = m2349;
                let m2344 = v0;
                match m2344.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a2345 = v1;
                        let a2346 = 0u64;
                        if nat_eq(a2345, a2346) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a2347 = v1;
                                let a2348 = 1u64;
                                nat_sub(a2347, a2348)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(v0: u64) -> bool {
    let a2352 = v0;
    let a2353 = 47u64;
    nat_eq(a2352, a2353)
}

pub fn f524(v0: u64) -> bool {
    f203({
        let a2354 = v0;
        let a2355 = 95u64;
        nat_eq(a2354, a2355)
    }, f203({
        let a2356 = v0;
        let a2357 = 45u64;
        nat_eq(a2356, a2357)
    }, f203({
        let a2358 = v0;
        let a2359 = 32u64;
        nat_eq(a2358, a2359)
    }, f203({
        let a2360 = v0;
        let a2361 = 46u64;
        nat_eq(a2360, a2361)
    }, {
        let a2362 = v0;
        let a2363 = 47u64;
        nat_eq(a2362, a2363)
    }))))
}

pub fn f525(v0: u64) -> bool {
    f117({
        let a2364 = 97u64;
        let a2365 = v0;
        nat_le(a2364, a2365)
    }, {
        let a2366 = v0;
        let a2367 = 122u64;
        nat_le(a2366, a2367)
    })
}

pub fn f526(v0: u64) -> bool {
    f117({
        let a2368 = 65u64;
        let a2369 = v0;
        nat_le(a2368, a2369)
    }, {
        let a2370 = v0;
        let a2371 = 90u64;
        nat_le(a2370, a2371)
    })
}

pub fn f527(v0: u64) -> bool {
    f117({
        let a2372 = 48u64;
        let a2373 = v0;
        nat_le(a2372, a2373)
    }, {
        let a2374 = v0;
        let a2375 = 57u64;
        nat_le(a2374, a2375)
    })
}

pub fn f528(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2376 = v0;
    match m2376.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f359(v1)?, f528(v2)?))
        }
    }
}

pub fn f529(v0: u32) -> Bytes {
    let a2379 = {
        let a2377 = v0;
        let a2378 = 8u32;
        fixed_u32::bit_and(a2377, a2378)
    };
    let a2380 = 0u32;
    if equal(a2379, a2380) {
        let a2383 = {
            let a2381 = v0;
            let a2382 = 4u32;
            fixed_u32::bit_and(a2381, a2382)
        };
        let a2384 = 0u32;
        if equal(a2383, a2384) {
            let a2387 = {
                let a2385 = v0;
                let a2386 = 2u32;
                fixed_u32::bit_and(a2385, a2386)
            };
            let a2388 = 0u32;
            if equal(a2387, a2388) {
                let a2391 = {
                    let a2389 = v0;
                    let a2390 = 1u32;
                    fixed_u32::bit_and(a2389, a2390)
                };
                let a2392 = 0u32;
                if equal(a2391, a2392) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a2395 = {
                    let a2393 = v0;
                    let a2394 = 1u32;
                    fixed_u32::bit_and(a2393, a2394)
                };
                let a2396 = 0u32;
                if equal(a2395, a2396) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a2399 = {
                let a2397 = v0;
                let a2398 = 2u32;
                fixed_u32::bit_and(a2397, a2398)
            };
            let a2400 = 0u32;
            if equal(a2399, a2400) {
                let a2403 = {
                    let a2401 = v0;
                    let a2402 = 1u32;
                    fixed_u32::bit_and(a2401, a2402)
                };
                let a2404 = 0u32;
                if equal(a2403, a2404) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a2407 = {
                    let a2405 = v0;
                    let a2406 = 1u32;
                    fixed_u32::bit_and(a2405, a2406)
                };
                let a2408 = 0u32;
                if equal(a2407, a2408) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a2411 = {
            let a2409 = v0;
            let a2410 = 4u32;
            fixed_u32::bit_and(a2409, a2410)
        };
        let a2412 = 0u32;
        if equal(a2411, a2412) {
            let a2415 = {
                let a2413 = v0;
                let a2414 = 2u32;
                fixed_u32::bit_and(a2413, a2414)
            };
            let a2416 = 0u32;
            if equal(a2415, a2416) {
                let a2419 = {
                    let a2417 = v0;
                    let a2418 = 1u32;
                    fixed_u32::bit_and(a2417, a2418)
                };
                let a2420 = 0u32;
                if equal(a2419, a2420) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a2423 = {
                    let a2421 = v0;
                    let a2422 = 1u32;
                    fixed_u32::bit_and(a2421, a2422)
                };
                let a2424 = 0u32;
                if equal(a2423, a2424) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a2427 = {
                let a2425 = v0;
                let a2426 = 2u32;
                fixed_u32::bit_and(a2425, a2426)
            };
            let a2428 = 0u32;
            if equal(a2427, a2428) {
                let a2431 = {
                    let a2429 = v0;
                    let a2430 = 1u32;
                    fixed_u32::bit_and(a2429, a2430)
                };
                let a2432 = 0u32;
                if equal(a2431, a2432) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2435 = {
                    let a2433 = v0;
                    let a2434 = 1u32;
                    fixed_u32::bit_and(a2433, a2434)
                };
                let a2436 = 0u32;
                if equal(a2435, a2436) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f530(v0: u32, v1: u32) -> u32 {
    let m2439 = {
        let a2437 = v0;
        let a2438 = v1;
        fixed_u32::shift_right(a2437, a2438)
    };
    match m2439 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f531(v0: Adt23, v1: Adt24) -> R<Adt23> {
    let v2: Adt25 = f558(Fn18::F559, 64u64, Adt25::C0(match v0.clone() {
        Adt23::C0(h2440, _, _, _, _, _, _, _) => {
            h2440
        }
    }, match v0.clone() {
        Adt23::C0(_, h2441, _, _, _, _, _, _) => {
            h2441
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h2442, _, _, _, _, _) => {
            h2442
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, h2443, _, _, _, _) => {
            h2443
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, h2444, _, _, _) => {
            h2444
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2445, _, _) => {
            h2445
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2446, _) => {
            h2446
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h2447) => {
            h2447
        }
    }, v1, 0u64))?;
    Ok(Adt23::C0(f560(match v0.clone() {
        Adt23::C0(h2448, _, _, _, _, _, _, _) => {
            h2448
        }
    }, match v2.clone() {
        Adt25::C0(h2449, _, _, _, _, _, _, _, _, _) => {
            h2449
        }
    }), f560(match v0.clone() {
        Adt23::C0(_, h2450, _, _, _, _, _, _) => {
            h2450
        }
    }, match v2.clone() {
        Adt25::C0(_, h2451, _, _, _, _, _, _, _, _) => {
            h2451
        }
    }), f560(match v0.clone() {
        Adt23::C0(_, _, h2452, _, _, _, _, _) => {
            h2452
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h2453, _, _, _, _, _, _, _) => {
            h2453
        }
    }), f560(match v0.clone() {
        Adt23::C0(_, _, _, h2454, _, _, _, _) => {
            h2454
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, h2455, _, _, _, _, _, _) => {
            h2455
        }
    }), f560(match v0.clone() {
        Adt23::C0(_, _, _, _, h2456, _, _, _) => {
            h2456
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, h2457, _, _, _, _, _) => {
            h2457
        }
    }), f560(match v0.clone() {
        Adt23::C0(_, _, _, _, _, h2458, _, _) => {
            h2458
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, h2459, _, _, _, _) => {
            h2459
        }
    }), f560(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h2460, _) => {
            h2460
        }
    }, match v2.clone() {
        Adt25::C0(_, _, _, _, _, _, h2461, _, _, _) => {
            h2461
        }
    }), f560(match v0 {
        Adt23::C0(_, _, _, _, _, _, _, h2462) => {
            h2462
        }
    }, match v2 {
        Adt25::C0(_, _, _, _, _, _, _, h2463, _, _) => {
            h2463
        }
    })))
}

pub fn f532(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt24> {
    Ok(Adt24::C0(f561(v0.clone(), v1, v2, v3)?, f561(v0.clone(), v1, v2, {
        let a2464 = v3;
        let a2465 = 4u64;
        nat_add(a2464, a2465)?
    })?, f561(v0.clone(), v1, v2, {
        let a2466 = v3;
        let a2467 = 8u64;
        nat_add(a2466, a2467)?
    })?, f561(v0.clone(), v1, v2, {
        let a2468 = v3;
        let a2469 = 12u64;
        nat_add(a2468, a2469)?
    })?, f561(v0.clone(), v1, v2, {
        let a2470 = v3;
        let a2471 = 16u64;
        nat_add(a2470, a2471)?
    })?, f561(v0.clone(), v1, v2, {
        let a2472 = v3;
        let a2473 = 20u64;
        nat_add(a2472, a2473)?
    })?, f561(v0.clone(), v1, v2, {
        let a2474 = v3;
        let a2475 = 24u64;
        nat_add(a2474, a2475)?
    })?, f561(v0.clone(), v1, v2, {
        let a2476 = v3;
        let a2477 = 28u64;
        nat_add(a2476, a2477)?
    })?, f561(v0.clone(), v1, v2, {
        let a2478 = v3;
        let a2479 = 32u64;
        nat_add(a2478, a2479)?
    })?, f561(v0.clone(), v1, v2, {
        let a2480 = v3;
        let a2481 = 36u64;
        nat_add(a2480, a2481)?
    })?, f561(v0.clone(), v1, v2, {
        let a2482 = v3;
        let a2483 = 40u64;
        nat_add(a2482, a2483)?
    })?, f561(v0.clone(), v1, v2, {
        let a2484 = v3;
        let a2485 = 44u64;
        nat_add(a2484, a2485)?
    })?, f561(v0.clone(), v1, v2, {
        let a2486 = v3;
        let a2487 = 48u64;
        nat_add(a2486, a2487)?
    })?, f561(v0.clone(), v1, v2, {
        let a2488 = v3;
        let a2489 = 52u64;
        nat_add(a2488, a2489)?
    })?, f561(v0.clone(), v1, v2, {
        let a2490 = v3;
        let a2491 = 56u64;
        nat_add(a2490, a2491)?
    })?, f561(v0, v1, v2, {
        let a2492 = v3;
        let a2493 = 60u64;
        nat_add(a2492, a2493)?
    })?))
}

pub fn f533(p2496: List<Bytes>, v1: Bytes) -> bool {
    let m2495 = p2496;
    {
        let mut m2495 = m2495;
        loop {
            let step = {
                let v0 = m2495;
                let m2494 = v0;
                match m2494.uncons() {
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
                Ok(next) => { m2495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f534(v0: Bytes, v1: u64) -> Bytes {
    let m2500 = {
        let a2497 = v0;
        let a2498 = v1;
        let a2499 = 1u64;
        slice_bytes(a2497, a2498, a2499)
    };
    match m2500 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f535(v0: Fn14, p2505: u64, p2506: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m2504 = (p2505, p2506);
    {
        let mut m2504 = m2504;
        loop {
            let step = {
                let (v1, v2) = m2504;
                let m2501 = v1;
                if m2501 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2501 - 1;
                    let m2503 = {
                        let c2502: Fn14 = v0.clone();
                        c2502.apply(v2.clone())?
                    };
                    match m2503 {
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
                Ok(next) => { m2504 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f536(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f562(v1, v0, v2)
}

pub fn f537(p2509: List<u64>, p2510: List<u64>) -> List<u64> {
    let m2508 = (p2509, p2510);
    {
        let mut m2508 = m2508;
        loop {
            let step = {
                let (v0, v1) = m2508;
                let m2507 = v0;
                match m2507.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f538(v0: List<u64>) -> R<Bytes> {
    let m2511 = v0;
    match m2511.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            let a2512 = f563(v1)?;
            let a2513 = f538(v2)?;
            Ok(append_bytes(a2512, a2513))
        }
    }
}

pub fn f539(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m2514 = v0;
    match m2514.uncons() {
        None => {
            let a2515 = v2;
            let a2516 = 0u64;
            if nat_eq(a2515, a2516) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a2519 = {
                    let a2517 = v3;
                    let a2518 = v1;
                    nat_mul(a2517, a2518)?
                };
                let a2520 = v2;
                nat_add(a2519, a2520)?
            };
            Ok(List::cons({
                let a2521 = v5;
                let a2522 = f540();
                let a2523 = 0u64;
                nat_rem(a2521, a2522, a2523)
            }, f539(v4, v1, {
                let a2524 = v5;
                let a2525 = f540();
                let a2526 = 0u64;
                nat_quot(a2524, a2525, a2526)
            })?))
        }
    }
}

pub fn f540() -> u64 {
    1000000000u64
}

pub fn f541(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2571 = {
            let a2547 = {
                let a2535 = {
                    let a2529 = {
                        let a2527 = v1;
                        let a2528 = 128u8;
                        fixed_u8::bit_and(a2527, a2528)
                    };
                    let a2530 = 128u8;
                    if equal(a2529, a2530) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2536 = {
                    let a2533 = {
                        let a2531 = v1;
                        let a2532 = 64u8;
                        fixed_u8::bit_and(a2531, a2532)
                    };
                    let a2534 = 64u8;
                    if equal(a2533, a2534) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2535, a2536)?
            };
            let a2548 = {
                let a2545 = {
                    let a2539 = {
                        let a2537 = v1;
                        let a2538 = 32u8;
                        fixed_u8::bit_and(a2537, a2538)
                    };
                    let a2540 = 32u8;
                    if equal(a2539, a2540) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2546 = {
                    let a2543 = {
                        let a2541 = v1;
                        let a2542 = 16u8;
                        fixed_u8::bit_and(a2541, a2542)
                    };
                    let a2544 = 16u8;
                    if equal(a2543, a2544) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2545, a2546)?
            };
            nat_add(a2547, a2548)?
        };
        let a2572 = {
            let a2569 = {
                let a2557 = {
                    let a2551 = {
                        let a2549 = v1;
                        let a2550 = 8u8;
                        fixed_u8::bit_and(a2549, a2550)
                    };
                    let a2552 = 8u8;
                    if equal(a2551, a2552) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2558 = {
                    let a2555 = {
                        let a2553 = v1;
                        let a2554 = 4u8;
                        fixed_u8::bit_and(a2553, a2554)
                    };
                    let a2556 = 4u8;
                    if equal(a2555, a2556) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2557, a2558)?
            };
            let a2570 = {
                let a2567 = {
                    let a2561 = {
                        let a2559 = v1;
                        let a2560 = 2u8;
                        fixed_u8::bit_and(a2559, a2560)
                    };
                    let a2562 = 2u8;
                    if equal(a2561, a2562) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2568 = {
                    let a2565 = {
                        let a2563 = v1;
                        let a2564 = 1u8;
                        fixed_u8::bit_and(a2563, a2564)
                    };
                    let a2566 = 1u8;
                    if equal(a2565, a2566) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2567, a2568)?
            };
            nat_add(a2569, a2570)?
        };
        nat_add(a2571, a2572)?
    };
    let a2579 = f534(f564(), {
        let a2573 = v2;
        let a2574 = 16u64;
        let a2575 = 0u64;
        nat_quot(a2573, a2574, a2575)
    });
    let a2580 = f534(f564(), {
        let a2576 = v2;
        let a2577 = 16u64;
        let a2578 = 0u64;
        nat_rem(a2576, a2577, a2578)
    });
    Ok(append_bytes(a2579, a2580))
}

pub fn f542(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f565(Fn19::F566, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f543(v0: Fn15, p2584: List<List<(Bytes, Bytes)>>, p2585: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2583 = (p2584, p2585);
    {
        let mut m2583 = m2583;
        loop {
            let step = {
                let (v1, v2) = m2583;
                let m2581 = v2;
                match m2581.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2582: Fn15 = v0.clone();
                            c2582.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f544(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f545(v0: Fn16, p2590: u64, p2591: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2589 = (p2590, p2591);
    {
        let mut m2589 = m2589;
        loop {
            let step = {
                let (v1, v2) = m2589;
                let m2586 = v1;
                if m2586 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2586 - 1;
                    let m2588 = {
                        let c2587: Fn16 = v0.clone();
                        c2587.apply(v2.clone())?
                    };
                    match m2588 {
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
                Ok(next) => { m2589 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f546(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f567(v0)
}

pub fn f547(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f568(v0, v1)
}

pub fn f548(v0: u8) -> bool {
    let a2592 = v0;
    let a2593 = 32u8;
    if if if equal(a2592, a2593) {
        true
    } else {
        let a2594 = v0;
        let a2595 = 9u8;
        equal(a2594, a2595)
    } {
        true
    } else {
        let a2596 = v0;
        let a2597 = 10u8;
        equal(a2596, a2597)
    } {
        true
    } else {
        let a2598 = v0;
        let a2599 = 12u8;
        if equal(a2598, a2599) {
            true
        } else {
            let a2600 = v0;
            let a2601 = 13u8;
            equal(a2600, a2601)
        }
    }
}

pub fn f549(v0: Fn17, p2606: u64, p2607: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2605 = (p2606, p2607);
    {
        let mut m2605 = m2605;
        loop {
            let step = {
                let (v1, v2) = m2605;
                let m2602 = v1;
                if m2602 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2602 - 1;
                    let m2604 = {
                        let c2603: Fn17 = v0.clone();
                        c2603.apply(v2.clone())?
                    };
                    match m2604 {
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
                Ok(next) => { m2605 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f569(v0, v1)
}

pub fn f551(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2652 = {
            let a2628 = {
                let a2616 = {
                    let a2610 = {
                        let a2608 = v1;
                        let a2609 = 128u8;
                        fixed_u8::bit_and(a2608, a2609)
                    };
                    let a2611 = 128u8;
                    if equal(a2610, a2611) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2617 = {
                    let a2614 = {
                        let a2612 = v1;
                        let a2613 = 64u8;
                        fixed_u8::bit_and(a2612, a2613)
                    };
                    let a2615 = 64u8;
                    if equal(a2614, a2615) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2616, a2617)?
            };
            let a2629 = {
                let a2626 = {
                    let a2620 = {
                        let a2618 = v1;
                        let a2619 = 32u8;
                        fixed_u8::bit_and(a2618, a2619)
                    };
                    let a2621 = 32u8;
                    if equal(a2620, a2621) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2627 = {
                    let a2624 = {
                        let a2622 = v1;
                        let a2623 = 16u8;
                        fixed_u8::bit_and(a2622, a2623)
                    };
                    let a2625 = 16u8;
                    if equal(a2624, a2625) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2626, a2627)?
            };
            nat_add(a2628, a2629)?
        };
        let a2653 = {
            let a2650 = {
                let a2638 = {
                    let a2632 = {
                        let a2630 = v1;
                        let a2631 = 8u8;
                        fixed_u8::bit_and(a2630, a2631)
                    };
                    let a2633 = 8u8;
                    if equal(a2632, a2633) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2639 = {
                    let a2636 = {
                        let a2634 = v1;
                        let a2635 = 4u8;
                        fixed_u8::bit_and(a2634, a2635)
                    };
                    let a2637 = 4u8;
                    if equal(a2636, a2637) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2638, a2639)?
            };
            let a2651 = {
                let a2648 = {
                    let a2642 = {
                        let a2640 = v1;
                        let a2641 = 2u8;
                        fixed_u8::bit_and(a2640, a2641)
                    };
                    let a2643 = 2u8;
                    if equal(a2642, a2643) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2649 = {
                    let a2646 = {
                        let a2644 = v1;
                        let a2645 = 1u8;
                        fixed_u8::bit_and(a2644, a2645)
                    };
                    let a2647 = 1u8;
                    if equal(a2646, a2647) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2648, a2649)?
            };
            nat_add(a2650, a2651)?
        };
        nat_add(a2652, a2653)?
    };
    let a2654 = 48u64;
    let a2655 = v2;
    if nat_le(a2654, a2655) {
        let a2656 = v2;
        let a2657 = 57u64;
        Ok(nat_le(a2656, a2657))
    } else {
        Ok(false)
    }
}

pub fn f552(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f452({
        let (_, h2658) = {
            let (h2659, _) = f570(Fn20::F571, {
                let a2661 = {
                    let a2660 = v0.clone();
                    length_list(a2660)
                };
                let a2662 = 1u64;
                nat_add(a2661, a2662)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2659
        };
        h2658
    }))
}

pub fn f553(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f554(v0: Adt1, v1: Bytes) -> R<bool> {
    if f67(v0.clone(), f517(v1.clone())) {
        let m2663 = f95(f64(v0, f517(v1)))?;
        match m2663.uncons() {
            None => {
                Ok(false)
            }
            Some((v2, _)) => {
                let m2664 = f43(v2)?;
                match m2664 {
                    None => {
                        Ok(false)
                    }
                    Some(v4) => {
                        let m2665 = f93(v4, f8(Str::lit("reader")));
                        match m2665 {
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

pub fn f555(p2669: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m2668 = p2669;
    {
        let mut m2668 = m2668;
        loop {
            let step = {
                let v0 = m2668;
                let m2666 = v0;
                match m2666.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f117(f32(f58(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m2667 = f93(v3, f8(Str::lit("std")));
                            match m2667 {
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
                Ok(next) => { m2668 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f556(v0: List<Adt2>) -> List<Bytes> {
    let m2670 = v0;
    match m2670.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2671 = f93(v1, f8(Str::lit("std")));
            match m2671 {
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

pub fn f557(v0: List<Adt2>) -> List<Bytes> {
    let m2672 = v0;
    match m2672.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2673 = f92(f93(v1, f8(Str::lit("req"))));
            match m2673 {
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

pub fn f558(v0: Fn18, p2677: u64, p2678: Adt25) -> R<Adt25> {
    let m2676 = (p2677, p2678);
    {
        let mut m2676 = m2676;
        loop {
            let step = {
                let (v1, v2) = m2676;
                let m2674 = v1;
                if m2674 == 0 {
                    Err::<(u64, Adt25), R<Adt25>>(Ok(v2))
                } else {
                    let v3 = m2674 - 1;
                    Ok::<(u64, Adt25), R<Adt25>>((v3, {
                        let c2675: Fn18 = v0.clone();
                        c2675.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2676 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f559(v0: Adt25) -> R<Adt25> {
    f572(v0)
}

pub fn f560(v0: u32, v1: u32) -> u32 {
    let m2681 = {
        let a2679 = f573(v0);
        let a2680 = f573(v1);
        fixed_u64::checked_add(a2679, a2680)
    };
    match m2681 {
        None => {
            0u32
        }
        Some(v2) => {
            f574(v2)
        }
    }
}

pub fn f561(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2684 = {
        let a2682 = v3;
        let a2683 = 8u64;
        nat_add(a2682, a2683)?
    };
    let a2685 = v2;
    if nat_eq(a2684, a2685) {
        Ok(f575({
            let a2688 = {
                let a2686 = v1;
                let a2687 = 8u64;
                nat_mul(a2686, a2687)?
            };
            let a2689 = 4294967296u64;
            let a2690 = 0u64;
            nat_quot(a2688, a2689, a2690)
        }))
    } else {
        let a2693 = {
            let a2691 = v3;
            let a2692 = 4u64;
            nat_add(a2691, a2692)?
        };
        let a2694 = v2;
        if nat_eq(a2693, a2694) {
            Ok(f575({
                let a2697 = {
                    let a2695 = v1;
                    let a2696 = 8u64;
                    nat_mul(a2695, a2696)?
                };
                let a2698 = 4294967296u64;
                let a2699 = 0u64;
                nat_rem(a2697, a2698, a2699)
            }))
        } else {
            Ok(f576(f577(v0.clone(), v1, v3), f577(v0.clone(), v1, {
                let a2700 = v3;
                let a2701 = 1u64;
                nat_add(a2700, a2701)?
            }), f577(v0.clone(), v1, {
                let a2702 = v3;
                let a2703 = 2u64;
                nat_add(a2702, a2703)?
            }), f577(v0, v1, {
                let a2704 = v3;
                let a2705 = 3u64;
                nat_add(a2704, a2705)?
            })))
        }
    }
}

pub fn f562(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2706, _) = v2;
        h2706
    };
    let a2707 = v3;
    let a2708 = 0u64;
    if nat_eq(a2707, a2708) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2709 = v3;
            let a2710 = v1;
            let a2711 = 0u64;
            nat_quot(a2709, a2710, a2711)
        }, {
            let a2716 = f534(v0, {
                let a2712 = v3;
                let a2713 = v1;
                let a2714 = 0u64;
                nat_rem(a2712, a2713, a2714)
            });
            let a2717 = {
                let (_, h2715) = v2;
                h2715
            };
            append_bytes(a2716, a2717)
        }))
    }
}

pub fn f563(v0: u64) -> R<Bytes> {
    let v1: Bytes = f404(v0)?;
    let a2721 = f493({
        let a2719 = 9u64;
        let a2720 = {
            let a2718 = v1.clone();
            length_bytes(a2718)
        };
        nat_sub(a2719, a2720)
    });
    let a2722 = v1;
    Ok(append_bytes(a2721, a2722))
}

pub fn f564() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f565(v0: Fn19, p2726: List<List<(Bytes, Bytes)>>, p2727: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2725 = (p2726, p2727);
    {
        let mut m2725 = m2725;
        loop {
            let step = {
                let (v1, v2) = m2725;
                let m2723 = v2;
                match m2723.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2724: Fn19 = v0.clone();
                            c2724.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f566(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f567(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2729 = {
        let a2728 = v0.clone();
        length_list(a2728)
    };
    let a2730 = 1u64;
    if nat_le(a2729, a2730) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f578(v0)?))
    }
}

pub fn f568(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2734 = {
        let a2732 = v0.clone();
        let a2733 = {
            let (h2731, _) = v1.clone();
            h2731
        };
        index_bytes(a2732, a2733)
    };
    match m2734 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2736 = {
                    let (h2735, _) = v1.clone();
                    h2735
                };
                let a2737 = 1u64;
                nat_add(a2736, a2737)?
            }, {
                let a2740 = {
                    let (_, h2738) = v1;
                    h2738
                };
                let a2741 = f494(v0, {
                    let (h2739, _) = v1;
                    h2739
                }, v2)?;
                append_bytes(a2740, a2741)
            })))
        }
    }
}

pub fn f569(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2742, _) = v1;
        h2742
    };
    let m2744 = {
        let (_, h2743) = v1;
        h2743
    };
    match m2744 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2747 = {
                let a2745 = v0.clone();
                let a2746 = v2;
                index_bytes(a2745, a2746)
            };
            match m2747 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2752 = {
                        let a2750 = v0;
                        let a2751 = {
                            let a2748 = v2;
                            let a2749 = 1u64;
                            nat_add(a2748, a2749)?
                        };
                        index_bytes(a2750, a2751)
                    };
                    match m2752 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f579(v4)? {
                                f579(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2753 = v2;
                                    let a2754 = 2u64;
                                    nat_add(a2753, a2754)?
                                }, Some({
                                    let a2759 = v3;
                                    let a2760 = f534(f580(), {
                                        let a2757 = {
                                            let a2755 = f581(v4)?;
                                            let a2756 = 16u64;
                                            nat_mul(a2755, a2756)?
                                        };
                                        let a2758 = f581(v5)?;
                                        nat_add(a2757, a2758)?
                                    });
                                    append_bytes(a2759, a2760)
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

pub fn f570(v0: Fn20, p2765: u64, p2766: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2764 = (p2765, p2766);
    {
        let mut m2764 = m2764;
        loop {
            let step = {
                let (v1, v2) = m2764;
                let m2761 = v1;
                if m2761 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2761 - 1;
                    let m2763 = {
                        let c2762: Fn20 = v0.clone();
                        c2762.apply(v2.clone())?
                    };
                    match m2763 {
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
                Ok(next) => { m2764 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f571(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f582(v0)
}

pub fn f572(v0: Adt25) -> R<Adt25> {
    let v1: u32 = f583(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, _, h2767) => {
            h2767
        }
    });
    let v2: u32 = f560(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, h2768, _, _) => {
            h2768
        }
    }, f560(f584(match v0.clone() {
        Adt25::C0(_, _, _, _, h2769, _, _, _, _, _) => {
            h2769
        }
    }), f560(f585(match v0.clone() {
        Adt25::C0(_, _, _, _, h2770, _, _, _, _, _) => {
            h2770
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2771, _, _, _, _) => {
            h2771
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2772, _, _, _) => {
            h2772
        }
    }), f560(v1, {
        let m2775 = match v0.clone() {
            Adt25::C0(_, _, _, _, _, _, _, _, h2774, _) => {
                h2774
            }
        };
        match m2775 {
            Adt24::C0(h2773, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2773
            }
        }
    }))));
    let v3: u32 = f560(f586(match v0.clone() {
        Adt25::C0(h2776, _, _, _, _, _, _, _, _, _) => {
            h2776
        }
    }), f587(match v0.clone() {
        Adt25::C0(h2777, _, _, _, _, _, _, _, _, _) => {
            h2777
        }
    }, match v0.clone() {
        Adt25::C0(_, h2778, _, _, _, _, _, _, _, _) => {
            h2778
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2779, _, _, _, _, _, _, _) => {
            h2779
        }
    }));
    Ok(Adt25::C0(f560(v2, v3), match v0.clone() {
        Adt25::C0(h2780, _, _, _, _, _, _, _, _, _) => {
            h2780
        }
    }, match v0.clone() {
        Adt25::C0(_, h2781, _, _, _, _, _, _, _, _) => {
            h2781
        }
    }, match v0.clone() {
        Adt25::C0(_, _, h2782, _, _, _, _, _, _, _) => {
            h2782
        }
    }, f560(match v0.clone() {
        Adt25::C0(_, _, _, h2783, _, _, _, _, _, _) => {
            h2783
        }
    }, v2), match v0.clone() {
        Adt25::C0(_, _, _, _, h2784, _, _, _, _, _) => {
            h2784
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, h2785, _, _, _, _) => {
            h2785
        }
    }, match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, h2786, _, _, _) => {
            h2786
        }
    }, f588(match v0.clone() {
        Adt25::C0(_, _, _, _, _, _, _, _, h2787, _) => {
            h2787
        }
    }), {
        let a2789 = match v0 {
            Adt25::C0(_, _, _, _, _, _, _, _, _, h2788) => {
                h2788
            }
        };
        let a2790 = 1u64;
        nat_add(a2789, a2790)?
    }))
}

pub fn f573(v0: u32) -> u64 {
    let m2792 = {
        let a2791 = v0;
        fixed_u64::convert(i128::from(a2791))
    };
    match m2792 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f574(v0: u64) -> u32 {
    let m2796 = {
        let a2795 = {
            let a2793 = v0;
            let a2794 = 4294967295u64;
            fixed_u64::bit_and(a2793, a2794)
        };
        fixed_u32::convert(i128::from(a2795))
    };
    match m2796 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f575(v0: u64) -> u32 {
    let (_, h2797) = {
        let (_, h2798) = f589(Fn21::F590, 32u64, (v0, (1u32, 0u32)));
        h2798
    };
    h2797
}

pub fn f576(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2803 = f591(v0, 24u32);
    let a2804 = {
        let a2801 = f591(v1, 16u32);
        let a2802 = {
            let a2799 = f591(v2, 8u32);
            let a2800 = v3;
            fixed_u32::bit_or(a2799, a2800)
        };
        fixed_u32::bit_or(a2801, a2802)
    };
    fixed_u32::bit_or(a2803, a2804)
}

pub fn f577(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2805 = v2;
    let a2806 = v1;
    if nat_lt(a2805, a2806) {
        let m2809 = {
            let a2807 = v0;
            let a2808 = v2;
            index_bytes(a2807, a2808)
        };
        match m2809 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2811 = {
                    let a2810 = v3;
                    fixed_u32::convert(i128::from(a2810))
                };
                match m2811 {
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
        let a2812 = v2;
        let a2813 = v1;
        if nat_eq(a2812, a2813) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f578(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f542({
        let (_, h2814) = {
            let (h2815, _) = f592(Fn22::F593, {
                let a2817 = {
                    let a2816 = v0.clone();
                    length_list(a2816)
                };
                let a2818 = 1u64;
                nat_add(a2817, a2818)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2815
        };
        h2814
    }))
}

pub fn f579(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2863 = {
            let a2839 = {
                let a2827 = {
                    let a2821 = {
                        let a2819 = v1;
                        let a2820 = 128u8;
                        fixed_u8::bit_and(a2819, a2820)
                    };
                    let a2822 = 128u8;
                    if equal(a2821, a2822) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2828 = {
                    let a2825 = {
                        let a2823 = v1;
                        let a2824 = 64u8;
                        fixed_u8::bit_and(a2823, a2824)
                    };
                    let a2826 = 64u8;
                    if equal(a2825, a2826) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2827, a2828)?
            };
            let a2840 = {
                let a2837 = {
                    let a2831 = {
                        let a2829 = v1;
                        let a2830 = 32u8;
                        fixed_u8::bit_and(a2829, a2830)
                    };
                    let a2832 = 32u8;
                    if equal(a2831, a2832) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2838 = {
                    let a2835 = {
                        let a2833 = v1;
                        let a2834 = 16u8;
                        fixed_u8::bit_and(a2833, a2834)
                    };
                    let a2836 = 16u8;
                    if equal(a2835, a2836) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2837, a2838)?
            };
            nat_add(a2839, a2840)?
        };
        let a2864 = {
            let a2861 = {
                let a2849 = {
                    let a2843 = {
                        let a2841 = v1;
                        let a2842 = 8u8;
                        fixed_u8::bit_and(a2841, a2842)
                    };
                    let a2844 = 8u8;
                    if equal(a2843, a2844) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2850 = {
                    let a2847 = {
                        let a2845 = v1;
                        let a2846 = 4u8;
                        fixed_u8::bit_and(a2845, a2846)
                    };
                    let a2848 = 4u8;
                    if equal(a2847, a2848) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2849, a2850)?
            };
            let a2862 = {
                let a2859 = {
                    let a2853 = {
                        let a2851 = v1;
                        let a2852 = 2u8;
                        fixed_u8::bit_and(a2851, a2852)
                    };
                    let a2854 = 2u8;
                    if equal(a2853, a2854) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2860 = {
                    let a2857 = {
                        let a2855 = v1;
                        let a2856 = 1u8;
                        fixed_u8::bit_and(a2855, a2856)
                    };
                    let a2858 = 1u8;
                    if equal(a2857, a2858) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2859, a2860)?
            };
            nat_add(a2861, a2862)?
        };
        nat_add(a2863, a2864)?
    };
    let a2865 = 48u64;
    let a2866 = v2;
    if if nat_le(a2865, a2866) {
        let a2867 = v2;
        let a2868 = 57u64;
        nat_le(a2867, a2868)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2869 = 97u64;
        let a2870 = v2;
        if nat_le(a2869, a2870) {
            let a2871 = v2;
            let a2872 = 102u64;
            Ok(nat_le(a2871, a2872))
        } else {
            Ok(false)
        }
    }
}

pub fn f580() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f581(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2917 = {
            let a2893 = {
                let a2881 = {
                    let a2875 = {
                        let a2873 = v1;
                        let a2874 = 128u8;
                        fixed_u8::bit_and(a2873, a2874)
                    };
                    let a2876 = 128u8;
                    if equal(a2875, a2876) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2882 = {
                    let a2879 = {
                        let a2877 = v1;
                        let a2878 = 64u8;
                        fixed_u8::bit_and(a2877, a2878)
                    };
                    let a2880 = 64u8;
                    if equal(a2879, a2880) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2881, a2882)?
            };
            let a2894 = {
                let a2891 = {
                    let a2885 = {
                        let a2883 = v1;
                        let a2884 = 32u8;
                        fixed_u8::bit_and(a2883, a2884)
                    };
                    let a2886 = 32u8;
                    if equal(a2885, a2886) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2892 = {
                    let a2889 = {
                        let a2887 = v1;
                        let a2888 = 16u8;
                        fixed_u8::bit_and(a2887, a2888)
                    };
                    let a2890 = 16u8;
                    if equal(a2889, a2890) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2891, a2892)?
            };
            nat_add(a2893, a2894)?
        };
        let a2918 = {
            let a2915 = {
                let a2903 = {
                    let a2897 = {
                        let a2895 = v1;
                        let a2896 = 8u8;
                        fixed_u8::bit_and(a2895, a2896)
                    };
                    let a2898 = 8u8;
                    if equal(a2897, a2898) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2904 = {
                    let a2901 = {
                        let a2899 = v1;
                        let a2900 = 4u8;
                        fixed_u8::bit_and(a2899, a2900)
                    };
                    let a2902 = 4u8;
                    if equal(a2901, a2902) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2903, a2904)?
            };
            let a2916 = {
                let a2913 = {
                    let a2907 = {
                        let a2905 = v1;
                        let a2906 = 2u8;
                        fixed_u8::bit_and(a2905, a2906)
                    };
                    let a2908 = 2u8;
                    if equal(a2907, a2908) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2914 = {
                    let a2911 = {
                        let a2909 = v1;
                        let a2910 = 1u8;
                        fixed_u8::bit_and(a2909, a2910)
                    };
                    let a2912 = 1u8;
                    if equal(a2911, a2912) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2913, a2914)?
            };
            nat_add(a2915, a2916)?
        };
        nat_add(a2917, a2918)?
    };
    let a2919 = v2;
    let a2920 = 57u64;
    if nat_le(a2919, a2920) {
        let a2921 = v2;
        let a2922 = 48u64;
        Ok(nat_sub(a2921, a2922))
    } else {
        let a2923 = v2;
        let a2924 = 87u64;
        Ok(nat_sub(a2923, a2924))
    }
}

pub fn f582(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2925, _) = v0;
        h2925
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2926) = v0;
        h2926
    };
    let m2927 = v1;
    match m2927.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2928 = v4;
            match m2928.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f594(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f583(v0: u64) -> u32 {
    let a2929 = v0;
    let a2930 = 32u64;
    if nat_lt(a2929, a2930) {
        let a2931 = v0;
        let a2932 = 16u64;
        if nat_lt(a2931, a2932) {
            let a2933 = v0;
            let a2934 = 8u64;
            if nat_lt(a2933, a2934) {
                let a2935 = v0;
                let a2936 = 4u64;
                if nat_lt(a2935, a2936) {
                    let a2937 = v0;
                    let a2938 = 2u64;
                    if nat_lt(a2937, a2938) {
                        let a2939 = v0;
                        let a2940 = 1u64;
                        if nat_lt(a2939, a2940) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2941 = v0;
                        let a2942 = 3u64;
                        if nat_lt(a2941, a2942) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2943 = v0;
                    let a2944 = 6u64;
                    if nat_lt(a2943, a2944) {
                        let a2945 = v0;
                        let a2946 = 5u64;
                        if nat_lt(a2945, a2946) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2947 = v0;
                        let a2948 = 7u64;
                        if nat_lt(a2947, a2948) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2949 = v0;
                let a2950 = 12u64;
                if nat_lt(a2949, a2950) {
                    let a2951 = v0;
                    let a2952 = 10u64;
                    if nat_lt(a2951, a2952) {
                        let a2953 = v0;
                        let a2954 = 9u64;
                        if nat_lt(a2953, a2954) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2955 = v0;
                        let a2956 = 11u64;
                        if nat_lt(a2955, a2956) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2957 = v0;
                    let a2958 = 14u64;
                    if nat_lt(a2957, a2958) {
                        let a2959 = v0;
                        let a2960 = 13u64;
                        if nat_lt(a2959, a2960) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2961 = v0;
                        let a2962 = 15u64;
                        if nat_lt(a2961, a2962) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2963 = v0;
            let a2964 = 24u64;
            if nat_lt(a2963, a2964) {
                let a2965 = v0;
                let a2966 = 20u64;
                if nat_lt(a2965, a2966) {
                    let a2967 = v0;
                    let a2968 = 18u64;
                    if nat_lt(a2967, a2968) {
                        let a2969 = v0;
                        let a2970 = 17u64;
                        if nat_lt(a2969, a2970) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2971 = v0;
                        let a2972 = 19u64;
                        if nat_lt(a2971, a2972) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2973 = v0;
                    let a2974 = 22u64;
                    if nat_lt(a2973, a2974) {
                        let a2975 = v0;
                        let a2976 = 21u64;
                        if nat_lt(a2975, a2976) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2977 = v0;
                        let a2978 = 23u64;
                        if nat_lt(a2977, a2978) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2979 = v0;
                let a2980 = 28u64;
                if nat_lt(a2979, a2980) {
                    let a2981 = v0;
                    let a2982 = 26u64;
                    if nat_lt(a2981, a2982) {
                        let a2983 = v0;
                        let a2984 = 25u64;
                        if nat_lt(a2983, a2984) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2985 = v0;
                        let a2986 = 27u64;
                        if nat_lt(a2985, a2986) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2987 = v0;
                    let a2988 = 30u64;
                    if nat_lt(a2987, a2988) {
                        let a2989 = v0;
                        let a2990 = 29u64;
                        if nat_lt(a2989, a2990) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2991 = v0;
                        let a2992 = 31u64;
                        if nat_lt(a2991, a2992) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2993 = v0;
        let a2994 = 48u64;
        if nat_lt(a2993, a2994) {
            let a2995 = v0;
            let a2996 = 40u64;
            if nat_lt(a2995, a2996) {
                let a2997 = v0;
                let a2998 = 36u64;
                if nat_lt(a2997, a2998) {
                    let a2999 = v0;
                    let a3000 = 34u64;
                    if nat_lt(a2999, a3000) {
                        let a3001 = v0;
                        let a3002 = 33u64;
                        if nat_lt(a3001, a3002) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3003 = v0;
                        let a3004 = 35u64;
                        if nat_lt(a3003, a3004) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3005 = v0;
                    let a3006 = 38u64;
                    if nat_lt(a3005, a3006) {
                        let a3007 = v0;
                        let a3008 = 37u64;
                        if nat_lt(a3007, a3008) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3009 = v0;
                        let a3010 = 39u64;
                        if nat_lt(a3009, a3010) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3011 = v0;
                let a3012 = 44u64;
                if nat_lt(a3011, a3012) {
                    let a3013 = v0;
                    let a3014 = 42u64;
                    if nat_lt(a3013, a3014) {
                        let a3015 = v0;
                        let a3016 = 41u64;
                        if nat_lt(a3015, a3016) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3017 = v0;
                        let a3018 = 43u64;
                        if nat_lt(a3017, a3018) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3019 = v0;
                    let a3020 = 46u64;
                    if nat_lt(a3019, a3020) {
                        let a3021 = v0;
                        let a3022 = 45u64;
                        if nat_lt(a3021, a3022) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3023 = v0;
                        let a3024 = 47u64;
                        if nat_lt(a3023, a3024) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3025 = v0;
            let a3026 = 56u64;
            if nat_lt(a3025, a3026) {
                let a3027 = v0;
                let a3028 = 52u64;
                if nat_lt(a3027, a3028) {
                    let a3029 = v0;
                    let a3030 = 50u64;
                    if nat_lt(a3029, a3030) {
                        let a3031 = v0;
                        let a3032 = 49u64;
                        if nat_lt(a3031, a3032) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3033 = v0;
                        let a3034 = 51u64;
                        if nat_lt(a3033, a3034) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3035 = v0;
                    let a3036 = 54u64;
                    if nat_lt(a3035, a3036) {
                        let a3037 = v0;
                        let a3038 = 53u64;
                        if nat_lt(a3037, a3038) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3039 = v0;
                        let a3040 = 55u64;
                        if nat_lt(a3039, a3040) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3041 = v0;
                let a3042 = 60u64;
                if nat_lt(a3041, a3042) {
                    let a3043 = v0;
                    let a3044 = 58u64;
                    if nat_lt(a3043, a3044) {
                        let a3045 = v0;
                        let a3046 = 57u64;
                        if nat_lt(a3045, a3046) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3047 = v0;
                        let a3048 = 59u64;
                        if nat_lt(a3047, a3048) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3049 = v0;
                    let a3050 = 62u64;
                    if nat_lt(a3049, a3050) {
                        let a3051 = v0;
                        let a3052 = 61u64;
                        if nat_lt(a3051, a3052) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3053 = v0;
                        let a3054 = 63u64;
                        if nat_lt(a3053, a3054) {
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

pub fn f584(v0: u32) -> u32 {
    let a3057 = {
        let a3055 = f595(v0, 6u32, 26u32);
        let a3056 = f595(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3055, a3056)
    };
    let a3058 = f595(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3057, a3058)
}

pub fn f585(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3064 = {
        let a3059 = v0;
        let a3060 = v1;
        fixed_u32::bit_and(a3059, a3060)
    };
    let a3065 = {
        let a3062 = {
            let a3061 = v0;
            fixed_u32::bit_not(a3061)
        };
        let a3063 = v2;
        fixed_u32::bit_and(a3062, a3063)
    };
    fixed_u32::bit_xor(a3064, a3065)
}

pub fn f586(v0: u32) -> u32 {
    let a3068 = {
        let a3066 = f595(v0, 2u32, 30u32);
        let a3067 = f595(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3066, a3067)
    };
    let a3069 = f595(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3068, a3069)
}

pub fn f587(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3078 = {
        let a3074 = {
            let a3070 = v0;
            let a3071 = v1;
            fixed_u32::bit_and(a3070, a3071)
        };
        let a3075 = {
            let a3072 = v0;
            let a3073 = v2;
            fixed_u32::bit_and(a3072, a3073)
        };
        fixed_u32::bit_xor(a3074, a3075)
    };
    let a3079 = {
        let a3076 = v1;
        let a3077 = v2;
        fixed_u32::bit_and(a3076, a3077)
    };
    fixed_u32::bit_xor(a3078, a3079)
}

pub fn f588(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(_, h3080, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3080
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h3081, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3081
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h3082, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3082
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h3083, _, _, _, _, _, _, _, _, _, _, _) => {
            h3083
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h3084, _, _, _, _, _, _, _, _, _, _) => {
            h3084
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h3085, _, _, _, _, _, _, _, _, _) => {
            h3085
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h3086, _, _, _, _, _, _, _, _) => {
            h3086
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, h3087, _, _, _, _, _, _, _) => {
            h3087
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h3088, _, _, _, _, _, _) => {
            h3088
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, h3089, _, _, _, _, _) => {
            h3089
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, h3090, _, _, _, _) => {
            h3090
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3091, _, _, _) => {
            h3091
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3092, _, _) => {
            h3092
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3093, _) => {
            h3093
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3094) => {
            h3094
        }
    }, f560(f596(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3095, _) => {
            h3095
        }
    }), f560(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, _, _, h3096, _, _, _, _, _, _) => {
            h3096
        }
    }, f560(f597(match v0.clone() {
        Adt24::C0(_, h3097, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3097
        }
    }), match v0 {
        Adt24::C0(h3098, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3098
        }
    }))))
}

pub fn f589(v0: Fn21, p3102: u64, p3103: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3101 = (p3102, p3103);
    {
        let mut m3101 = m3101;
        loop {
            let step = {
                let (v1, v2) = m3101;
                let m3099 = v1;
                if m3099 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3099 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3100: Fn21 = v0.clone();
                        c3100.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3101 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f590(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3104, _) = v0;
        h3104
    };
    let v2: u32 = {
        let (h3105, _) = {
            let (_, h3106) = v0;
            h3106
        };
        h3105
    };
    let v3: u32 = {
        let (_, h3107) = {
            let (_, h3108) = v0;
            h3108
        };
        h3107
    };
    ({
        let a3109 = v1;
        let a3110 = 2u64;
        let a3111 = 0u64;
        nat_quot(a3109, a3110, a3111)
    }, (f591(v2, 1u32), {
        let a3115 = {
            let a3112 = v1;
            let a3113 = 2u64;
            let a3114 = 0u64;
            nat_rem(a3112, a3113, a3114)
        };
        let a3116 = 1u64;
        if nat_eq(a3115, a3116) {
            let a3117 = v3;
            let a3118 = v2;
            fixed_u32::bit_or(a3117, a3118)
        } else {
            v3
        }
    }))
}

pub fn f591(v0: u32, v1: u32) -> u32 {
    let m3121 = {
        let a3119 = v0;
        let a3120 = v1;
        fixed_u32::shift_left(a3119, a3120)
    };
    match m3121 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f592(v0: Fn22, p3126: u64, p3127: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3125 = (p3126, p3127);
    {
        let mut m3125 = m3125;
        loop {
            let step = {
                let (v1, v2) = m3125;
                let m3122 = v1;
                if m3122 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3122 - 1;
                    let m3124 = {
                        let c3123: Fn22 = v0.clone();
                        c3123.apply(v2.clone())?
                    };
                    match m3124 {
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
                Ok(next) => { m3125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f598(v0)
}

pub fn f594(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3132 = {
            let a3130 = {
                let a3128 = v0.clone();
                length_list(a3128)
            };
            let a3131 = {
                let a3129 = v1.clone();
                length_list(a3129)
            };
            nat_add(a3130, a3131)?
        };
        let a3133 = 1u64;
        nat_add(a3132, a3133)?
    };
    Ok(f457({
        let (_, h3134) = {
            let (_, h3135) = {
                let (h3136, _) = f599(Fn23::F600, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3136
            };
            h3135
        };
        h3134
    }))
}

pub fn f595(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3137 = f530(v0, v1);
    let a3138 = f591(v0, v2);
    fixed_u32::bit_or(a3137, a3138)
}

pub fn f596(v0: u32) -> u32 {
    let a3141 = {
        let a3139 = f595(v0, 17u32, 15u32);
        let a3140 = f595(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3139, a3140)
    };
    let a3142 = f530(v0, 10u32);
    fixed_u32::bit_xor(a3141, a3142)
}

pub fn f597(v0: u32) -> u32 {
    let a3145 = {
        let a3143 = f595(v0, 7u32, 25u32);
        let a3144 = f595(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3143, a3144)
    };
    let a3146 = f530(v0, 3u32);
    fixed_u32::bit_xor(a3145, a3146)
}

pub fn f598(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3147, _) = v0;
        h3147
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3148) = v0;
        h3148
    };
    let m3149 = v1;
    match m3149.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3150 = v4;
            match m3150.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f601(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f599(v0: Fn23, p3155: u64, p3156: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3154 = (p3155, p3156);
    {
        let mut m3154 = m3154;
        loop {
            let step = {
                let (v1, v2) = m3154;
                let m3151 = v1;
                if m3151 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3151 - 1;
                    let m3153 = {
                        let c3152: Fn23 = v0.clone();
                        c3152.apply(v2.clone())
                    };
                    match m3153 {
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
                Ok(next) => { m3154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f602(v0)
}

pub fn f601(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3161 = {
            let a3159 = {
                let a3157 = v0.clone();
                length_list(a3157)
            };
            let a3160 = {
                let a3158 = v1.clone();
                length_list(a3158)
            };
            nat_add(a3159, a3160)?
        };
        let a3162 = 1u64;
        nat_add(a3161, a3162)?
    };
    Ok(f603({
        let (_, h3163) = {
            let (_, h3164) = {
                let (h3165, _) = f604(Fn24::F605, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3165
            };
            h3164
        };
        h3163
    }))
}

pub fn f602(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3166, _) = v0.clone();
        h3166
    };
    let v2: List<(Bytes, ())> = {
        let (h3167, _) = {
            let (_, h3168) = v0.clone();
            h3168
        };
        h3167
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3169) = {
            let (_, h3170) = v0;
            h3170
        };
        h3169
    };
    let m3171 = v1.clone();
    match m3171.uncons() {
        None => {
            let m3172 = v2;
            match m3172.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3173 = v2.clone();
            match m3173.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f606({
                        let (h3174, _) = v8.clone();
                        h3174
                    }, {
                        let (h3175, _) = v6.clone();
                        h3175
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

pub fn f603(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f415(Fn5::F607, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f604(v0: Fn24, p3180: u64, p3181: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3179 = (p3180, p3181);
    {
        let mut m3179 = m3179;
        loop {
            let step = {
                let (v1, v2) = m3179;
                let m3176 = v1;
                if m3176 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3176 - 1;
                    let m3178 = {
                        let c3177: Fn24 = v0.clone();
                        c3177.apply(v2.clone())
                    };
                    match m3178 {
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
                Ok(next) => { m3179 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f605(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f608(v0)
}

pub fn f606(v0: Bytes, v1: Bytes) -> bool {
    let a3186 = {
        let a3182 = v0;
        let a3183 = v1;
        compare_bytes(a3182, a3183)
    };
    let a3187 = {
        let a3184 = Bytes::lit(&[]);
        let a3185 = Bytes::lit(&[0x01]);
        compare_bytes(a3184, a3185)
    };
    equal(a3186, a3187)
}

pub fn f607(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f608(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3188, _) = v0.clone();
        h3188
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3189, _) = {
            let (_, h3190) = v0.clone();
            h3190
        };
        h3189
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3191) = {
            let (_, h3192) = v0;
            h3192
        };
        h3191
    };
    let m3193 = v1.clone();
    match m3193.uncons() {
        None => {
            let m3194 = v2;
            match m3194.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3195 = v2.clone();
            match m3195.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f606({
                        let (h3196, _) = v8.clone();
                        h3196
                    }, {
                        let (h3197, _) = v6.clone();
                        h3197
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
