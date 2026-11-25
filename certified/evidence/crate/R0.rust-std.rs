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
    C0(Bytes, std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C1(Bytes, Bytes, std::rc::Rc<Adt18>),
    C2(Bytes),
    C3(std::rc::Rc<Adt18>, std::rc::Rc<Adt18>),
    C4,
}

#[derive(Clone)]
pub enum Adt19 {
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt19>),
    C5(std::rc::Rc<Adt19>),
    C6,
    C7(std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
    C8(Bytes, std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
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
    C0(Bytes, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Bytes, Adt19),
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
pub enum Fn0 {
    F57(u8, Bytes),
    F187(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F57(k0, k1) => f57(*k0, k1.clone(), p0),
            Fn0::F187(k0) => f187(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F165,
    F183,
    F265,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F165 => f165(p0, p1),
            Fn1::F183 => f183(p0, p1),
            Fn1::F265 => f265(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F219(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F219(k0) => f219(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F255(Bytes, u64, Bytes, u64),
    F306(Bytes),
    F367(Bytes),
    F372(Bytes, Bytes),
    F373(Bytes),
    F374(Bytes, u64),
    F379(u64, Bytes),
    F425(bool, u64, Bytes),
    F466(Bytes),
    F467(Bytes),
    F501(Bytes),
    F543(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn3::F255(k0, k1, k2, k3) => f255(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn3::F306(k0) => f306(k0.clone(), p0),
            Fn3::F367(k0) => Ok(f367(k0.clone(), p0)),
            Fn3::F372(k0, k1) => f372(k0.clone(), k1.clone(), p0),
            Fn3::F373(k0) => f373(k0.clone(), p0),
            Fn3::F374(k0, k1) => f374(k0.clone(), *k1, p0),
            Fn3::F379(k0, k1) => f379(*k0, k1.clone(), p0),
            Fn3::F425(k0, k1, k2) => f425(*k0, *k1, k2.clone(), p0),
            Fn3::F466(k0) => f466(k0.clone(), p0),
            Fn3::F467(k0) => f467(k0.clone(), p0),
            Fn3::F501(k0) => f501(k0.clone(), p0),
            Fn3::F543(k0) => f543(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F298(Bytes, u64, u64),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Adt24)) -> R<(u64, Adt24)> {
        match self {
            Fn4::F298(k0, k1, k2) => f298(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F305(Bytes),
    F318(u64, Bytes),
    F398(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn5::F305(k0) => f305(k0.clone(), p0),
            Fn5::F318(k0, k1) => Ok(f318(*k0, k1.clone(), p0)),
            Fn5::F398(k0) => f398(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F347,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn6::F347 => f347(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F350,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn7::F350 => f350(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F352,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn8::F352 => f352(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F355,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn9::F355 => f355(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F400,
    F618,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F400 => f400(p0, p1),
            Fn10::F618 => f618(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F403,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F403 => f403(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F406,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F406 => f406(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F409,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F409 => f409(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F414,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt27) -> R<Adt27> {
        match self {
            Fn14::F414 => f414(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F427(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F427(k0) => f427(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F462(bool, Bytes),
    F469(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F462(k0, k1) => f462(*k0, k1.clone(), p0),
            Fn16::F469(k0) => f469(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F478,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F478 => f478(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F480,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F480 => f480(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F482,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F482 => f482(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F490,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F490 => f490(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F519,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F519 => f519(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F581,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F581 => f581(p0),
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
    F585(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F585(k0) => f585(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F616,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F616 => f616(p0),
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
                                            if f36(v2.clone(), f8(Str::lit("read-xml"))) {
                                                f43(v1, v4)
                                            } else {
                                                if f36(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                    f44(v1.clone(), v4.clone(), f45(v1, v4)?)
                                                } else {
                                                    if f36(v2.clone(), f8(Str::lit("stage-mappings"))) {
                                                        f44(v1.clone(), v4.clone(), f46(v1, v4, false)?)
                                                    } else {
                                                        if f36(v2.clone(), f8(Str::lit("stage-matrix"))) {
                                                            f44(v1.clone(), v4.clone(), f46(v1, v4, true)?)
                                                        } else {
                                                            if f36(v2.clone(), f8(Str::lit("stage-admissions"))) {
                                                                f44(v1.clone(), v4.clone(), f47(v1, v4, false)?)
                                                            } else {
                                                                if f36(v2.clone(), f8(Str::lit("stage-decisions"))) {
                                                                    f44(v1.clone(), v4.clone(), f47(v1, v4, true)?)
                                                                } else {
                                                                    if f36(v2.clone(), f8(Str::lit("admit"))) {
                                                                        f48(v1, v4)
                                                                    } else {
                                                                        if f36(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                                            f44(v1.clone(), v4.clone(), f49(v1, v4)?)
                                                                        } else {
                                                                            if f36(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                                                f50(v1, v4)
                                                                            } else {
                                                                                if f36(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                                    f51(v1, v4)
                                                                                } else {
                                                                                    if f36(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                                        f52(v1, v4)
                                                                                    } else {
                                                                                        if f36(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                                            f53(v1, v4)
                                                                                        } else {
                                                                                            if f36(v2, f8(Str::lit("propose-encoding"))) {
                                                                                                let m179 = v5;
                                                                                                match m179.uncons() {
                                                                                                    None => {
                                                                                                        Ok(Adt11::C4(f35()))
                                                                                                    }
                                                                                                    Some((v6, v7)) => {
                                                                                                        f54(v1, v4, v6, v7)
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
}

pub fn f29(v0: u8) -> R<Str> {
    let m228 = {
        let a226 = f55();
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
        let (h230, _) = f56(Fn0::F57(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h230
    };
    let v4: u64 = {
        let (h231, _) = {
            let (_, h232) = v3.clone();
            h232
        };
        h231
    };
    Ok(f58(List::cons(f32(v0, v4, {
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
                        if f59(v1.clone(), 0u64, {
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
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | stage-admissions JSONL | stage-decisions JSONL | stage-mappings JSONL | stage-matrix JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | read-xml FILE | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f36(v0: Bytes, v1: Bytes) -> bool {
    let a251 = v0;
    let a252 = v1;
    equal(a251, a252)
}

pub fn f37(v0: Adt1) -> R<Adt11> {
    let m253 = f60(v0.clone(), f61());
    match m253.uncons() {
        None => {
            let m254 = f62(v0.clone(), f61())?;
            match m254.uncons() {
                None => {
                    let m257 = f63(match v0.clone() {
                        Adt1::C0(_, h255, _) => {
                            h255
                        }
                    }, f64(f65(), f66(match v0.clone() {
                        Adt1::C0(_, h256, _) => {
                            h256
                        }
                    }, false)));
                    match m257.uncons() {
                        None => {
                            let m259 = f60(v0.clone(), f66(match v0.clone() {
                                Adt1::C0(_, h258, _) => {
                                    h258
                                }
                            }, true));
                            match m259.uncons() {
                                None => {
                                    let m261 = f67(v0.clone(), f66(match v0.clone() {
                                        Adt1::C0(_, h260, _) => {
                                            h260
                                        }
                                    }, true))?;
                                    match m261 {
                                        None => {
                                            let v10: (bool, Bytes) = f68(Adt13::C0(Adt12::C0(f69(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f71(v0.clone())?, f72(f73(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f69(f70(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f72(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f73(v0.clone(), f8(Str::lit("model/strata.json")))?, f74(f73(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f66(match v0 {
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
    if f75(f76(match v0.clone() {
        Adt1::C0(_, h266, _) => {
            h266
        }
    }, f77())) {
        Ok(Adt11::C1(List::cons(f77(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f78(f79(f77(), f80(match v0.clone() {
            Adt1::C0(_, h267, _) => {
                h267
            }
        }, f77())))?;
        let m268 = f60(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m268.uncons() {
            None => {
                let m269 = f81(v0.clone(), f82(f83(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f84(v2, v1.clone()))?;
                match m269 {
                    None => {
                        f85(v0, v1)
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
    if f75(f86(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h270) => {
            h270
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f87(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f75(f86(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h271) => {
                h271
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f87(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f40(v0: Adt1) -> R<Adt11> {
    let m272 = f88(v0.clone());
    match m272.uncons() {
        None => {
            let m273 = f64(f60(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f89(), List::<Bytes>::nil())))), f90(v0.clone())?);
            match m273.uncons() {
                None => {
                    Ok(Adt11::C3(f91(f92(v0.clone())?, List::cons((f93(), f94()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f95()), List::cons((f8(Str::lit(".github/workflows/refresh.yml")), f96()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f97()), List::cons((f8(Str::lit("docs/catalog.md")), f98(v0.clone())?), List::cons((f8(Str::lit("docs/matrix.md")), f99(v0.clone())?), List::cons((f8(Str::lit("docs/model.md")), f100(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f101()), List::<(Bytes, Bytes)>::nil()))))))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md, docs/catalog.md, docs/matrix.md and docs/owner-decisions.md\u{a}"))))
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
    let m274 = f88(v0.clone());
    match m274.uncons() {
        None => {
            let m275 = f64(f60(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil())), f90(v0.clone())?);
            match m275.uncons() {
                None => {
                    let v5: Adt15 = f102(v0.clone())?;
                    let v6: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f64(f103(f104(match v5.clone() {
                        Adt15::C0(h276, _, _) => {
                            h276
                        }
                    }, f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f106(v6.clone())), f107(f104(match v5.clone() {
                        Adt15::C0(h277, _, _) => {
                            h277
                        }
                    }, f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f108(v5.clone(), v6, f109(v5, f110(v0.clone(), f111(v0)?)?))?)))))
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
    let m278 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m278.uncons() {
        None => {
            let v4: Bytes = f70(v0.clone(), v1);
            let v5: List<List<Bytes>> = f112(v4.clone())?;
            if f75(f113(f114(v5.clone()), f115())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f116(Str::lit("id"), v6.clone())?, List::cons(f116(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f116(Str::lit("artifact"), f117())?, List::cons(f116(Str::lit("checksum"), f118(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f117(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f119(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f120(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f121(f122(f123(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f43(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m279 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m279.uncons() {
        None => {
            let m280 = f124(f70(v0, v1))?;
            match m280 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f125(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f126(f127(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
                }
                Err(v4) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v4, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f44(v0: Adt1, v1: Bytes, v2: Adt11) -> R<Adt11> {
    let m281 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m281.uncons() {
        None => {
            if f128(f83(f70(v0, v1))?)? {
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

pub fn f45(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m282 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m282.uncons() {
        None => {
            let m283 = f129(f83(f70(v0.clone(), v1.clone()))?)?;
            match m283 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    let m284 = f130(f83(f70(v0, v1))?)?;
                    match m284 {
                        None => {
                            Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f121(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                        }
                        Some(v5) => {
                            Ok(Adt11::C4(f7(List::cons(f8(Str::lit("binding ")), List::cons(v5, List::cons(f8(Str::lit(": a redistributable source.url must name a file, because ingest-authority saves the download under the last segment of its path, and this URL ends in /\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f46(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m285 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m285.uncons() {
        None => {
            let v5: Bytes = f131(if v2 {
                f132()
            } else {
                f133()
            }, f134(v1.clone())?);
            if f75(f135(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a staged file is named NAME.jsonl\u{a}"))))
            } else {
                let m286 = f136(f83(f70(v0.clone(), v1.clone()))?, v2)?;
                match m286 {
                    None => {
                        Ok(Adt11::C4(if v2 {
                            f8(Str::lit("every line must be a stakeholder edge: domain, row, relation (PROV-O producer, consumer, attributed or onBehalfOf, SOSA observation, or ESCO esco:hasEssentialSkill or esco:hasOptionalSkill), role.term, evidence.authority, evidence.quote and origin.kind\u{a}"))
                        } else {
                            f8(Str::lit("every line must be a SKOS mapping: subject, relation (exactMatch, closeMatch, broadMatch, narrowMatch or relatedMatch), object and evidence, with scheme naming the authority of a concept URI\u{a}"))
                        }))
                    }
                    Some(v6) => {
                        let a288 = {
                            let a287 = v6;
                            length_list(a287)
                        };
                        let a289 = 0u64;
                        if nat_eq(a288, a289) {
                            Ok(Adt11::C4(f8(Str::lit("the file has no lines\u{a}"))))
                        } else {
                            Ok(Adt11::C3(List::cons((v5.clone(), f70(v0, v1)), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("staged ")), List::cons(v5, List::cons(f8(Str::lit(": its ends resolve against the index, the authorities, the constructs, or a concept of a named scheme\u{a}")), List::<Bytes>::nil()))))))
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

pub fn f47(v0: Adt1, v1: Bytes, v2: bool) -> R<Adt11> {
    let m290 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m290.uncons() {
        None => {
            let v5: List<Bytes> = f83(f70(v0.clone(), v1.clone()))?;
            if f75(f135(v1.clone(), f8(Str::lit(".jsonl")))) {
                Ok(Adt11::C4(f8(Str::lit("a queued file is named NAME.jsonl\u{a}"))))
            } else {
                if if v2 {
                    f137(v5)?
                } else {
                    f138(v5)?
                } {
                    Ok(Adt11::C3(List::cons((f131(f139(), f134(v1.clone())?), f70(v0, v1.clone())), List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("queued ")), List::cons(f131(f139(), f134(v1)?), List::cons(f8(Str::lit(": next applies it once every citation in it resolves\u{a}")), List::<Bytes>::nil()))))))
                } else {
                    Ok(Adt11::C4(if v2 {
                        f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))
                    } else {
                        f140()
                    }))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f48(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m291 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m291.uncons() {
        None => {
            let v4: List<Bytes> = f83(f70(v0.clone(), v1))?;
            let m292 = f141(v4.clone(), f8(Str::lit("requirement")))?;
            match m292 {
                None => {
                    Ok(Adt11::C4(f140()))
                }
                Some(v5) => {
                    let m293 = f141(v4, f8(Str::lit("admission")))?;
                    match m293 {
                        None => {
                            Ok(Adt11::C4(f140()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f64(f72(v5.clone(), f8(Str::lit("id"))), f72(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f78(f64(f72(v7.clone(), f8(Str::lit("id"))), f72(v8.clone(), f8(Str::lit("construct")))))?;
                            let m294 = f142(f106(v6.clone()), f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m294 {
                                None => {
                                    let a296 = {
                                        let a295 = v9.clone();
                                        length_list(a295)
                                    };
                                    let a297 = 0u64;
                                    if nat_eq(a296, a297) {
                                        Ok(Adt11::C4(f140()))
                                    } else {
                                        if f75({
                                            let a300 = {
                                                let a298 = f78(f64(v9.clone(), v10.clone()))?;
                                                length_list(a298)
                                            };
                                            let a301 = {
                                                let a299 = f64(v9.clone(), v10);
                                                length_list(a299)
                                            };
                                            nat_eq(a300, a301)
                                        }) {
                                            Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt11::C3(List::cons(f144(f8(Str::lit("model/requirements.json")), f145(v7, v5))?, List::cons(f144(f8(Str::lit("model/admissions.json")), f145(v8, v6))?, List::cons(f120(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f121(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f143(v11))
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

pub fn f49(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m302 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m302.uncons() {
        None => {
            let m303 = f146(f83(f70(v0, v1))?)?;
            match m303 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f121(f34(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f50(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m304 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m304.uncons() {
        None => {
            let m305 = f147(v0.clone(), v1.clone())?;
            match m305 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m306 = f148(v4.clone(), f8(Str::lit("schema")));
                    match m306 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f149(v0, f150(v5.clone())?, v5, f151(v4.clone(), f8(Str::lit("name"))), f152(f153(f148(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f51(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m307 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f89(), List::<Bytes>::nil())));
    match m307.uncons() {
        None => {
            let m308 = f154(f83(f70(v0.clone(), v1))?)?;
            match m308 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f73(v0, f89())?;
                    let v6: List<Bytes> = f72(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f78(f72(v5.clone(), f8(Str::lit("at"))))?;
                    let a310 = {
                        let a309 = v6.clone();
                        length_list(a309)
                    };
                    let a311 = 0u64;
                    if if nat_eq(a310, a311) {
                        true
                    } else {
                        f75({
                            let a314 = {
                                let a312 = f78(f64(v6.clone(), v7.clone()))?;
                                length_list(a312)
                            };
                            let a315 = {
                                let a313 = f64(v6.clone(), v7);
                                length_list(a313)
                            };
                            nat_eq(a314, a315)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f144(f89(), f145(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f121(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
    let m316 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m316.uncons() {
        None => {
            let m317 = f155(v0, v1)?;
            match m317 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f156(f157(Some(v4))) {
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

pub fn f53(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m318 = f60(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m318.uncons() {
        None => {
            let m319 = f155(v0.clone(), v1.clone())?;
            match m319 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f75(f36(f158(v4.clone(), f8(Str::lit("schema"))), f159())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f158(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m320 = f160(v4.clone(), f8(Str::lit("source")));
                            match m320 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f161(f160(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f131(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f131(v9.clone(), f8(Str::lit("binding.json"))), f70(v0.clone(), v1));
                        let m321 = v8;
                        match m321 {
                            None => {
                                f162(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f75(f36(f158(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f162(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f131(v9, f134(f158(v7.clone(), f8(Str::lit("url"))))?);
                                    if f75(f86(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h322) => {
                                            h322
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f158(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m323 = f60(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m323.uncons() {
                                            None => {
                                                if f36(f118(f70(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f162(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f54(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m324 = f60(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m324.uncons() {
        None => {
            let m325 = f147(v0.clone(), v1.clone())?;
            match m325 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f149(v0.clone(), f70(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f55() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f56(v0: Fn0, p330: u64, p331: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m329 = (p330, p331);
    {
        let mut m329 = m329;
        loop {
            let step = {
                let (v1, v2) = m329;
                let m326 = v1;
                if m326 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m326 - 1;
                    let m328 = {
                        let c327: Fn0 = v0.clone();
                        c327.apply(v2.clone())?
                    };
                    match m328 {
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
                Ok(next) => { m329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f163(v1, v0, v2)
}

pub fn f58(v0: List<Bytes>) -> List<Bytes> {
    f164(Fn1::F165, List::<Bytes>::nil(), v0)
}

pub fn f59(v0: Bytes, p387: u64, p388: u64) -> R<bool> {
    let m386 = (p387, p388);
    {
        let mut m386 = m386;
        loop {
            let step = {
                let (v1, v2) = m386;
                let m332 = v2;
                if m332 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m332 - 1;
                    let m335 = {
                        let a333 = v0.clone();
                        let a334 = v1;
                        index_bytes(a333, a334)
                    };
                    match m335 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a382 = {
                                let v5: u8 = v4;
                                let a380 = {
                                    let a356 = {
                                        let a344 = {
                                            let a338 = {
                                                let a336 = v5;
                                                let a337 = 128u8;
                                                fixed_u8::bit_and(a336, a337)
                                            };
                                            let a339 = 128u8;
                                            if equal(a338, a339) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a345 = {
                                            let a342 = {
                                                let a340 = v5;
                                                let a341 = 64u8;
                                                fixed_u8::bit_and(a340, a341)
                                            };
                                            let a343 = 64u8;
                                            if equal(a342, a343) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a344, a345)?
                                    };
                                    let a357 = {
                                        let a354 = {
                                            let a348 = {
                                                let a346 = v5;
                                                let a347 = 32u8;
                                                fixed_u8::bit_and(a346, a347)
                                            };
                                            let a349 = 32u8;
                                            if equal(a348, a349) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a355 = {
                                            let a352 = {
                                                let a350 = v5;
                                                let a351 = 16u8;
                                                fixed_u8::bit_and(a350, a351)
                                            };
                                            let a353 = 16u8;
                                            if equal(a352, a353) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a354, a355)?
                                    };
                                    nat_add(a356, a357)?
                                };
                                let a381 = {
                                    let a378 = {
                                        let a366 = {
                                            let a360 = {
                                                let a358 = v5;
                                                let a359 = 8u8;
                                                fixed_u8::bit_and(a358, a359)
                                            };
                                            let a361 = 8u8;
                                            if equal(a360, a361) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a367 = {
                                            let a364 = {
                                                let a362 = v5;
                                                let a363 = 4u8;
                                                fixed_u8::bit_and(a362, a363)
                                            };
                                            let a365 = 4u8;
                                            if equal(a364, a365) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a366, a367)?
                                    };
                                    let a379 = {
                                        let a376 = {
                                            let a370 = {
                                                let a368 = v5;
                                                let a369 = 2u8;
                                                fixed_u8::bit_and(a368, a369)
                                            };
                                            let a371 = 2u8;
                                            if equal(a370, a371) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a377 = {
                                            let a374 = {
                                                let a372 = v5;
                                                let a373 = 1u8;
                                                fixed_u8::bit_and(a372, a373)
                                            };
                                            let a375 = 1u8;
                                            if equal(a374, a375) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a376, a377)?
                                    };
                                    nat_add(a378, a379)?
                                };
                                nat_add(a380, a381)?
                            };
                            let a383 = 127u64;
                            if nat_le(a382, a383) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a384 = v1;
                                    let a385 = 1u64;
                                    nat_add(a384, a385)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m386 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m389 = v1;
    match m389.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m391 = f166(match v0.clone() {
                Adt1::C0(h390, _, _) => {
                    h390
                }
            }, v2.clone());
            match m391 {
                None => {
                    List::cons(v2, f60(v0, v3))
                }
                Some(_) => {
                    f60(v0, v3)
                }
            }
        }
    }
}

pub fn f61() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f117(), List::<Bytes>::nil())))))))
}

pub fn f62(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m392 = v1;
    match m392.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f167(v0.clone(), v2.clone()) {
                f135(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m393 = f155(v0.clone(), v2.clone())?;
                match m393 {
                    None => {
                        Ok(List::cons(v2, f62(v0, v3)?))
                    }
                    Some(_) => {
                        f62(v0, v3)
                    }
                }
            } else {
                f62(v0, v3)
            }
        }
    }
}

pub fn f63(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m394 = v1;
    match m394.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f76(v0.clone(), v2.clone()) {
                f63(v0, v3)
            } else {
                List::cons(v2, f63(v0, v3))
            }
        }
    }
}

pub fn f64(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m395 = v0;
    match m395.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f64(v3, v1))
        }
    }
}

pub fn f65() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f66(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m396 = v0;
    match m396.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f64(f168({
                let (h397, _) = v2;
                h397
            }, {
                let (_, h398) = v2;
                h398
            }, v1), f66(v3, v1))
        }
    }
}

pub fn f67(v0: Adt1, p402: List<Bytes>) -> R<Option<Bytes>> {
    let m401 = p402;
    {
        let mut m401 = m401;
        loop {
            let step = {
                let v1 = m401;
                let m399 = v1;
                match m399.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m400 = f169(f70(v0.clone(), v2.clone()), f170())?;
                        match m400 {
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
                Ok(next) => { m401 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h403, _, _, _, _) => {
            h403
        }
    };
    let v2: Adt16 = f171(v1.clone())?;
    let v3: List<Bytes> = f172(v1.clone());
    let v4: List<Bytes> = f72(match v0.clone() {
        Adt13::C0(_, _, h404, _, _) => {
            h404
        }
    }, f8(Str::lit("path")));
    let m406 = f173(v2, List::<u64>::nil(), f174(v1.clone(), match v0.clone() {
        Adt13::C0(_, h405, _, _, _) => {
            h405
        }
    }, 0u64)?);
    match m406 {
        None => {
            if f176(f177(match v0.clone() {
                Adt13::C0(_, _, _, _, h408) => {
                    h408
                }
            }, f64(match v0.clone() {
                Adt13::C0(_, _, _, _, h409) => {
                    h409
                }
            }, v4.clone()))?, f177(v4.clone(), f64(match v0.clone() {
                Adt13::C0(_, _, _, _, h410) => {
                    h410
                }
            }, v4.clone()))?, f178(match v0.clone() {
                Adt13::C0(_, _, h411, _, _) => {
                    h411
                }
            })) {
                let m415 = f179(f177(match v0.clone() {
                    Adt13::C0(_, _, _, h412, _) => {
                        h412
                    }
                }, f64(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h413, _) => {
                        h413
                    }
                }))?, f177(v3.clone(), f64(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h414, _) => {
                        h414
                    }
                }))?);
                match m415 {
                    None => {
                        let m418 = f180(f72(match v0 {
                            Adt13::C0(_, h417, _, _, _) => {
                                h417
                            }
                        }, f8(Str::lit("construct"))), List::<Bytes>::nil());
                        match m418 {
                            None => {
                                let m419 = f180(v4, List::<Bytes>::nil());
                                match m419 {
                                    None => {
                                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f121(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f175(f64(v3, match v0 {
                            Adt13::C0(_, _, _, h416, _) => {
                                h416
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f175(match v1 {
                Adt12::C0(_, _, _, _, h407) => {
                    h407
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f69(v0: Bytes) -> R<List<Bytes>> {
    f181(f83(v0)?, f8(Str::lit("id = \"")))
}

pub fn f70(v0: Adt1, v1: Bytes) -> Bytes {
    let m421 = f166(match v0 {
        Adt1::C0(h420, _, _) => {
            h420
        }
    }, v1);
    match m421 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m422 = v2;
            match m422 {
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

pub fn f71(v0: Adt1) -> R<List<Bytes>> {
    let m423 = f112(f70(v0, f117()))?;
    match m423.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f122(v2))
        }
    }
}

pub fn f72(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m424 = v0;
    match m424.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f158(v2, v1.clone()), f72(v3, v1))
        }
    }
}

pub fn f73(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f157(f155(v0, v1)?))
}

pub fn f74(v0: List<Adt14>) -> List<Bytes> {
    let m425 = v0;
    match m425.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m426 = f161(Some(v1));
            match m426 {
                None => {
                    f74(v2)
                }
                Some(v3) => {
                    List::cons(v3, f74(v2))
                }
            }
        }
    }
}

pub fn f75(v0: bool) -> bool {
    !v0
}

pub fn f76(p430: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m429 = p430;
    {
        let mut m429 = m429;
        loop {
            let step = {
                let v0 = m429;
                let m427 = v0;
                match m427.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h428, _) = v2;
                            h428
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f78(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f182(f164(Fn1::F183, List::<Bytes>::nil(), f184(v0)?)))
}

pub fn f79(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m431 = v1;
    match m431.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f135(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f131(v0.clone(), v2), f79(v0, v3))
            } else {
                f79(v0, v3)
            }
        }
    }
}

pub fn f80(p436: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m435 = p436;
    {
        let mut m435 = m435;
        loop {
            let step = {
                let v0 = m435;
                let m432 = v0;
                match m432.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h433, _) = v2;
                            h433
                        }, v1.clone()) {
                            let (_, h434) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h434)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
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

pub fn f81(v0: Adt1, v1: List<List<Bytes>>, p439: List<Bytes>) -> R<Option<Bytes>> {
    let m438 = p439;
    {
        let mut m438 = m438;
        loop {
            let step = {
                let v2 = m438;
                let m437 = v2;
                match m437.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f185(v0.clone(), v1.clone(), v3.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v4)
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

pub fn f82(p442: List<Bytes>, p443: List<Bytes>, p444: List<List<Bytes>>) -> List<List<Bytes>> {
    let m441 = (p442, (p443, p444));
    {
        let mut m441 = m441;
        loop {
            let step = {
                let (v0, (v1, v2)) = m441;
                let m440 = v0;
                match m440.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f186(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f36(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f186(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a445 = v0.clone();
        length_bytes(a445)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h446, _) = f56(Fn0::F187(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h446
    };
    let v3: u64 = {
        let (h447, _) = {
            let (_, h448) = v2.clone();
            h448
        };
        h447
    };
    let v4: List<Bytes> = {
        let (_, h449) = {
            let (_, h450) = v2;
            h450
        };
        h449
    };
    let a451 = v3;
    let a452 = v1;
    if nat_lt(a451, a452) {
        Ok(f58(List::cons(f32(v0, v3, {
            let a453 = v1;
            let a454 = v3;
            nat_sub(a453, a454)
        }), v4)))
    } else {
        Ok(f58(v4))
    }
}

pub fn f84(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m455 = v0;
    match m455.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f84(v3, v1)
            } else {
                List::cons(v2, f84(v3, v1))
            }
        }
    }
}

pub fn f85(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f75(f76(match v0.clone() {
        Adt1::C0(_, h456, _) => {
            h456
        }
    }, f188())) {
        Ok(Adt11::C1(List::cons(f188(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f84(f78(f79(f188(), f80(match v0.clone() {
            Adt1::C0(_, h457, _) => {
                h457
            }
        }, f188())))?, v1.clone());
        let m458 = f60(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m458.uncons() {
            None => {
                let v5: List<Bytes> = f72(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")));
                let m459 = f60(v0.clone(), f189(v0.clone(), v2.clone(), v5.clone())?);
                match m459.uncons() {
                    None => {
                        let m460 = f190(v0.clone(), v2, v5)?;
                        match m460 {
                            None => {
                                f191(v0, v1)
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

pub fn f86(v0: Bytes, p463: List<Bytes>) -> bool {
    let m462 = p463;
    {
        let mut m462 = m462;
        loop {
            let step = {
                let v1 = m462;
                let m461 = v1;
                match m461.uncons() {
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
                Ok(next) => { m462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f88(v0: Adt1) -> List<Bytes> {
    if f76(match v0.clone() {
        Adt1::C0(_, h464, _) => {
            h464
        }
    }, f133()) {
        if f76(match v0 {
            Adt1::C0(_, h465, _) => {
                h465
            }
        }, f132()) {
            List::<Bytes>::nil()
        } else {
            List::cons(f132(), List::<Bytes>::nil())
        }
    } else {
        List::cons(f133(), List::<Bytes>::nil())
    }
}

pub fn f89() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f90(v0: Adt1) -> R<List<Bytes>> {
    Ok(f60(v0.clone(), List::cons(f117(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/admissions.json")), f111(v0)?)))))
}

pub fn f91(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m466 = v0;
    match m466.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f91(v3, v1))
        }
    }
}

pub fn f92(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m467 = f73(v0.clone(), f89())?;
    match m467.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f192(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f93() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f94() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f95() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}# Every push to main runs advance. next (under a second) is the only gate, so a run\u{a}# that defers to an open advance pull request always has a successor when that merges.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          # A patch larger than a dispatch input can carry arrives gzipped and base64-encoded.\u{a}          if head -c 4 target/input/change.patch | grep -q '^H4sI'; then base64 -d < target/input/change.patch | gunzip > target/input/change.diff; mv target/input/change.diff target/input/change.patch; fi\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^app/src/Contract.lex.tex$' -e '^app/src/Studio/' -e '^model/ids.toml$' -e '^model/strata.json$' -e '^features/suites/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src, app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json, features/suites and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/' || echo \"::error::")), List::cons(f193(), List::cons(f8(Str::lit("\"\u{a}            tail -n 40 target/export.log\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          # The owner reviews this pull request, so its body leads with the proposer's own\u{a}          # account (the prose before the first diff), then the definitions it changes.\u{a}          awk 'index($0, \"diff --git\") == 1 { exit } { print }' target/input/change.patch > target/summary.md\u{a}          if ! grep -q '[[:alnum:]]' target/summary.md; then\u{a}            echo \"::error::propose-command needs a plain-language account before the patch: what changes and why, the requirement or decision it serves, what it unblocks, and the risk and how it was verified\"\u{a}            exit 1\u{a}          fi\u{a}          git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | awk 'index($0, \"+++ b\") == 1 { file = substr($0, 7) } index($0, \"@@\") == 1 { parts = split($3, at, \",\"); start = substr(at[1], 2); count = (parts > 1) ? at[2] : 1; print file \" \" ((count > 0) ? start + count - 1 : start) }' > target/hunks.txt\u{a}          {\u{a}            cat target/summary.md\u{a}            echo\u{a}            echo \"### Definitions this adds or changes\"\u{a}            echo\u{a}            {\u{a}              while read -r file line; do head -n \"$line\" \"$file\" | { grep '^  name: ' || true; } | tail -n 1; done < target/hunks.txt\u{a}              { git diff -U0 -- studio/src app/src/Contract.lex.tex app/src/Studio | grep '^[+]  name: ' || true; } | cut -c2-\u{a}            } | awk '{ sub(\"^  name: \", \"- `\"); sub(\",$\", \"`\"); print }' | sort -u\u{a}            echo\u{a}            slash=/\u{a}            echo \"<details><summary>Export and files<${slash}summary>\"\u{a}            echo\u{a}            echo \"Exported again with Lean as $(basename studio/.lexlean/production/*/). The studio check replays it, then runs check, build and publish.\"\u{a}            echo\u{a}            echo '```'\u{a}            git diff --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md\u{a}            echo '```'\u{a}            echo\u{a}            echo \"<${slash}details>\"\u{a}          } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          # An App pull request whose files already hold on main has nothing left to merge,\u{a}          # so it is closed; the propose-command ones are the owner's to review and stay open.\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.user.type == \"Bot\" and (.title | startswith(\"verdant_studio \")) and (.title | startswith(\"verdant_studio propose-command\") | not)) | .number'); do\u{a}            files=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}/files?per_page=100\" --jq '.[].filename')\u{a}            if [ -n \"$files\" ] && git fetch -q --depth=1 origin \"pull/${pr}/head\" && git diff --quiet HEAD FETCH_HEAD -- $files; then\u{a}              gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}              echo \"::notice::closed #${pr}: every file it changes already holds on main\"\u{a}            fi\u{a}          done\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}            gate=$(target/release/verdant_studio merge-gate target/rules.json)\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              # Where main lets an App pull request merge itself, one without auto-merge was left by a\u{a}              # run that failed after opening it, so nothing would ever merge it: this run supersedes it.\u{a}              elif [ \"$gate\" = merge ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.auto_merge != null')\" = false ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: auto-merge was never enabled on it, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            # next prints one operation it can dispatch, or else the lines that say why the\u{a}            # queue waits: each begins with \"waiting\", and none is dispatched.\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ] && [ \"${op#waiting}\" = \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}            t0=$EPOCHREALTIME\u{a}            target/release/verdant_studio next $skipped > target/next.txt || true\u{a}            echo \"next: $(echo \"$EPOCHREALTIME - $t0\" | bc) s for one call over $(find model -name '*.json*' | wc -l) model files\" | tee -a \"$GITHUB_STEP_SUMMARY\"\u{a}            if [ \"${op#waiting}\" != \"$op\" ]; then echo \"$op\" > target/waiting.txt; else : > target/waiting.txt; fi\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title | startswith(\"verdant_studio propose-command\")) | .number'); do\u{a}              echo \"waiting owner review: #${pr} changes the API, and only the owner merges it\" >> target/waiting.txt\u{a}            done\u{a}            if [ -s target/waiting.txt ]; then tee -a target/operation.txt < target/waiting.txt >> \"$GITHUB_STEP_SUMMARY\"; fi\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          # A GitHub API call that fails (a 5xx answer or a dropped connection) is tried again,\u{a}          # at most 4 tries, 2, 4 and 8 seconds apart, so a transient error never stops the queue.\u{a}          gh() { local try out; for try in 1 2 3 4; do if out=$(command gh \"$@\"); then if [ -n \"$out\" ]; then echo \"$out\"; fi; return 0; fi; if [ \"$try\" -lt 4 ]; then echo \"::warning::gh $1 failed (try $try of 4)\" >&2; sleep $((2 ** try)); fi; done; return 1; }\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          title=\"verdant_studio $ARGS\"\u{a}          if [ \"$ARGS\" = propose-command ]; then title=\"verdant_studio propose-command: $(grep -m 1 '[[:alnum:]]' target/summary.md | awk '{ sub(\"^[# ]*\", \"\"); print }')\"; fi\u{a}          gh pr create --head \"$branch\" --base main --title \"$title\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f96() -> Bytes {
    f7(List::cons(f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# When main moves, every open propose-command pull request that no longer merges\u{a}# is regenerated by the Verdant Studio GitHub App: its change to studio/src,\u{a}# app/src/Contract.lex.tex, app/src/Studio, model/ids.toml, model/strata.json,\u{a}# features/suites and README.md is applied to the new main and exported again with Lean, then pushed\u{a}# to the same pull request, so the owner never reviews a conflicting one. Every\u{a}# conflicting pull request is marked a draft as soon as it is found, and each one is\u{a}# regenerated in its own job, in parallel, and marked ready again only once it is pushed.\u{a}name: refresh\u{a}\u{a}# A pull request that opens while main moves is caught when it opens, too.\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}  pull_request:\u{a}    types: [opened]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# A running refresh always finishes, because an export takes minutes and main moves more\u{a}# often than that; GitHub keeps only the newest pending run behind it.\u{a}concurrency:\u{a}  group: refresh\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  find:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 15\u{a}    outputs:\u{a}      stale: ${{ steps.stale.outputs.stale }}\u{a}      matrix: ${{ steps.stale.outputs.matrix }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - id: stale\u{a}        name: find the propose-command pull requests that conflict with main\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          stale=''\u{a}          for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq \".[] | select((.title | startswith(\\\"verdant_studio propose-command\\\")) and .user.login == \\\"${SLUG}[bot]\\\") | .number\"); do\u{a}            # mergeable is null while GitHub computes it, and it stays meaningful for a draft,\u{a}            # which a cancelled run may have left behind.\u{a}            mergeable=null\u{a}            for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do\u{a}              mergeable=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq '.mergeable | tostring')\u{a}              if [ \"$mergeable\" != null ]; then break; fi\u{a}              sleep 10\u{a}            done\u{a}            if [ \"$mergeable\" = false ]; then\u{a}              stale=\"$stale $pr\"\u{a}              # A draft until its regeneration is pushed, so the owner never sees it conflicting.\u{a}              gh pr ready --undo \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            elif [ \"$mergeable\" = true ] && [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .draft)\" = true ]; then\u{a}              gh pr ready \"$pr\" --repo \"$GITHUB_REPOSITORY\" || true\u{a}            fi\u{a}          done\u{a}          echo \"stale=$stale\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"matrix=$(jq -cn --arg s \"$stale\" '$s | split(\" \") | map(select(. != \"\") | tonumber)')\" >> \"$GITHUB_OUTPUT\"\u{a}          echo \"propose-command pull requests to regenerate:${stale:- none}\"\u{a}\u{a}  regenerate:\u{a}    needs: find\u{a}    if: needs.find.outputs.stale != ''\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 60\u{a}    strategy:\u{a}      fail-fast: false\u{a}      matrix:\u{a}        pr: ${{ fromJSON(needs.find.outputs.matrix) }}\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}          fetch-depth: 0\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - name: apply the change to the new main, export it again, and push it to its pull request\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          STALE: ${{ matrix.pr }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            mkdir -p target\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          failed=''\u{a}          for pr in $STALE; do\u{a}            branch=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .head.ref)\u{a}            # While it is regenerated (an export takes minutes), the pull request is a draft, so\u{a}            # the owner never sees it as conflicting; it is marked ready again only once it is pushed.\u{a}            gh pr ready --undo \"$pr\" || true\u{a}            git fetch -q origin \"$branch\" main\u{a}            git checkout -q -B refresh origin/main\u{a}            git clean -fdq\u{a}            git diff \"$(git merge-base origin/main \"origin/$branch\")\" \"origin/$branch\" -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md > target/change.patch\u{a}            if ! git apply --3way target/change.patch 2> target/apply.err; then\u{a}              sed 's/^/::error::#'\"$pr\"' does not apply to main: /' target/apply.err | head -20\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              # A failed regeneration stays a draft, so the owner is never handed a red pull request.\u{a}              { echo \"This change no longer applies to main $(git rev-parse --short origin/main), so it stays a draft until it is proposed again:\"; echo; echo '```'; head -20 target/apply.err; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}              grep -m 20 'error' target/export.log | sed 's/^/::error::#'\"$pr\"': /' || echo \"::error::#${pr}: ")), List::cons(f193(), List::cons(f8(Str::lit("\"\u{a}              tail -n 40 target/export.log\u{a}              git reset -q --hard origin/main\u{a}              failed=\"$failed $pr\"\u{a}              { echo \"The Lean export failed on main $(git rev-parse --short origin/main), so this stays a draft until it is fixed:\"; echo; echo '```'; grep -m 20 'error' target/export.log || tail -n 20 target/export.log; echo '```'; } > target/comment.md\u{a}              gh pr comment \"$pr\" --body-file target/comment.md || true\u{a}              continue\u{a}            fi\u{a}            rm -rf certified\u{a}            cp -r studio/.lexlean/production/*/ certified\u{a}            rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}            rm -rf studio/.lexlean/production\u{a}            \"$ll\" production verify-artifact certified\u{a}            cargo build -q --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}            target/release/verdant_studio check\u{a}            target/release/verdant_studio build\u{a}            target/release/verdant_studio publish\u{a}            git add -A\u{a}            { echo \"verdant_studio propose-command\"; echo; echo \"regenerated on main $(git rev-parse --short origin/main)\"; git diff --cached --stat -- studio/src app/src/Contract.lex.tex app/src/Studio model/ids.toml model/strata.json features/suites README.md; } > target/message.txt\u{a}            git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F target/message.txt\u{a}            git push -q --force-with-lease=\"${branch}:origin/${branch}\" origin \"HEAD:refs/heads/${branch}\"\u{a}            gh pr ready \"$pr\" || true\u{a}            echo \"::notice::regenerated #${pr} on the new main\"\u{a}          done\u{a}          if [ -n \"$failed\" ]; then echo \"::error::these pull requests could not be regenerated:$failed\"; exit 1; fi\u{a}")), List::<Bytes>::nil()))))
}

pub fn f97() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f98(v0: Adt1) -> R<Bytes> {
    let v1: Adt15 = f102(v0.clone())?;
    let v2: List<Adt14> = f73(v0.clone(), f8(Str::lit("model/admissions.json")))?;
    let v3: List<Adt14> = f109(v1.clone(), f110(v0.clone(), f111(v0.clone())?)?);
    let v4: List<List<Bytes>> = f123(f112(f70(v0, f117()))?);
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio catalog\u{a}\u{a}Generated by `verdant_studio publish` from catalog/standards-index.csv, model/admissions.json and model/mappings/. Do not edit.\u{a}Each index row with its SKOS mappings. A mapping derived from a citation is exactMatch only when the row cites one authority in every source, and narrowMatch otherwise; a staged mapping overrides it. A row is reached when the mappings connect it to an origin, an authority or row that an admission cites.\u{a}\u{a}")), List::cons(f7(f194(v4.clone(), f122(v4), f195(v1.clone(), v2.clone(), v3.clone())?, v3.clone(), f108(v1, v2, v3)?)), List::<Bytes>::nil()))))
}

pub fn f99(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio stakeholder matrix\u{a}\u{a}Generated by `verdant_studio publish` from model/matrix/. Do not edit.\u{a}Each PROV-O edge: producer is prov:wasGeneratedBy with prov:wasAttributedTo, consumer is prov:used, attributed is prov:wasAttributedTo, onBehalfOf is prov:actedOnBehalfOf, observation is a SOSA sosa:Observation, and the esco: relations are ESCO occupation-skill relations. Every edge quotes its pinned source, and only edges whose row and quoted authority are in the model are shown.\u{a}\u{a}")), List::cons(f7(f196(f102(v0.clone())?, f110(v0.clone(), f111(v0)?)?)), List::<Bytes>::nil()))))
}

pub fn f100(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f197(f73(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f198(f73(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f101() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f102(v0: Adt1) -> R<Adt15> {
    Ok(Adt15::C0(f71(v0.clone())?, f69(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f72(f73(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))))
}

pub fn f103(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m468 = v0;
    match m468.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f103(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f103(v3, v1))
            }
        }
    }
}

pub fn f104(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m469 = v0;
    match m469.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f104(v3, v1)
            } else {
                List::cons(v2, f104(v3, v1))
            }
        }
    }
}

pub fn f105(v0: List<Adt14>) -> List<Bytes> {
    let m470 = v0;
    match m470.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f64(f74(f157(f160(v1, f8(Str::lit("defers"))))), f105(v2))
        }
    }
}

pub fn f106(v0: List<Adt14>) -> List<Bytes> {
    let m471 = v0;
    match m471.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m472 = f160(v1, f8(Str::lit("std")));
            match m472 {
                None => {
                    f106(v2)
                }
                Some(v3) => {
                    List::cons(f158(v3, f8(Str::lit("row"))), f106(v2))
                }
            }
        }
    }
}

pub fn f107(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m473 = v0;
    match m473.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f107(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("unreached-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f107(v3, v1))
            }
        }
    }
}

pub fn f108(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Bytes>> {
    let v3: List<(Bytes, Bytes)> = f91(f199(v2.clone()), f200(f195(v0, v1.clone(), v2)?));
    f201(f78(f202(v1))?, v3.clone(), {
        let a475 = {
            let a474 = v3;
            length_list(a474)
        };
        let a476 = 1u64;
        nat_add(a475, a476)?
    })
}

pub fn f109(v0: Adt15, v1: List<Adt14>) -> List<Adt14> {
    let m477 = v1;
    match m477.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v2, v3)) => {
            if if f203(v2.clone(), f8(Str::lit("subject"))) {
                f204(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(v2, f109(v0, v3))
            } else {
                f109(v0, v3)
            }
        }
    }
}

pub fn f110(v0: Adt1, v1: List<Bytes>) -> R<List<Adt14>> {
    let m478 = v1;
    match m478.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let m479 = f136(f83(f70(v0.clone(), v2.clone()))?, f205(v2, f132()))?;
            match m479 {
                None => {
                    f110(v0, v3)
                }
                Some(v4) => {
                    Ok(f145(v4, f110(v0, v3)?))
                }
            }
        }
    }
}

pub fn f111(v0: Adt1) -> R<List<Bytes>> {
    Ok(f64(f78(f206(f133(), f80(match v0.clone() {
        Adt1::C0(_, h480, _) => {
            h480
        }
    }, f133())))?, f78(f206(f132(), f80(match v0 {
        Adt1::C0(_, h481, _) => {
            h481
        }
    }, f132())))?))
}

pub fn f112(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt17 = f207(v0.clone(), 0u64, {
        let a482 = v0;
        length_bytes(a482)
    }, Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt17 = {
        let a485 = {
            let a484 = match v1.clone() {
                Adt17::C0(_, _, h483, _, _) => {
                    h483
                }
            };
            length_bytes(a484)
        };
        let a486 = 0u64;
        if if nat_eq(a485, a486) {
            let m488 = match v1.clone() {
                Adt17::C0(_, _, _, h487, _) => {
                    h487
                }
            };
            match m488.uncons() {
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
            f208(v1)
        }
    };
    Ok(f209(match v4 {
        Adt17::C0(_, _, _, _, h489) => {
            h489
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f113(p494: List<Bytes>, p495: List<Bytes>) -> bool {
    let m493 = (p494, p495);
    {
        let mut m493 = m493;
        loop {
            let step = {
                let (v0, v1) = m493;
                let m490 = v0;
                match m490.uncons() {
                    None => {
                        let m491 = v1;
                        match m491.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m492 = v1;
                        match m492.uncons() {
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
                Ok(next) => { m493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: List<List<Bytes>>) -> List<Bytes> {
    let m496 = v0;
    match m496.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f115() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f116(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f210(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f117() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f118(v0: Bytes) -> R<Bytes> {
    Ok(f211(f212(v0)?))
}

pub fn f119(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f213(f82(f83(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f214(f215(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f120(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f216(Adt14::C3(std::rc::Rc::new(f217(f78(f64(v1, f74(f73(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f121(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m497 = f218(Fn2::F219(v1), None::<Bytes>, v0);
    match m497 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f122(v0: List<List<Bytes>>) -> List<Bytes> {
    let m498 = v0;
    match m498.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m499 = v1;
            match m499.uncons() {
                None => {
                    f122(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f122(v2))
                }
            }
        }
    }
}

pub fn f123(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m500 = v0;
    match m500.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f124(v0: Bytes) -> R<Result<Adt18, Bytes>> {
    let v1: Bytes = if f220(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f32(v0.clone(), 3u64, {
            let a502 = {
                let a501 = v0;
                length_bytes(a501)
            };
            let a503 = 3u64;
            nat_sub(a502, a503)
        })
    } else {
        v0
    };
    let v2: Bytes = f221(v1)?;
    let v3: u64 = {
        let a504 = v2.clone();
        length_bytes(a504)
    };
    let v4: u64 = f222(v2.clone())?;
    let a505 = v4;
    let a506 = v3;
    if nat_lt(a505, a506) {
        Ok(Err::<Adt18, Bytes>(f223(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m507 = f224(v2.clone())?;
        match m507 {
            Ok(v6) => {
                let m510 = f225(v2.clone(), v6, {
                    let a508 = v3;
                    let a509 = 2u64;
                    nat_add(a508, a509)?
                })?;
                match m510 {
                    Ok(v8) => {
                        if f226(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(Err::<Adt18, Bytes>(f223(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a511 = f227(v2.clone(), v8)?;
                            let a512 = 60u64;
                            if !nat_eq(a511, a512) {
                                Ok(Err::<Adt18, Bytes>(f223(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            } else {
                                let m515 = f228(v2.clone(), v8, {
                                    let a513 = v3;
                                    let a514 = 2u64;
                                    nat_add(a513, a514)?
                                }, 0u64)?;
                                match m515 {
                                    Ok(v10) => {
                                        let m519 = f225(v2, {
                                            let (_, h516) = v10;
                                            h516
                                        }, {
                                            let a517 = v3;
                                            let a518 = 2u64;
                                            nat_add(a517, a518)?
                                        })?;
                                        match m519 {
                                            Ok(v12) => {
                                                let a520 = v12;
                                                let a521 = v3;
                                                if nat_eq(a520, a521) {
                                                    Ok(Ok::<Adt18, Bytes>({
                                                        let (h522, _) = v10;
                                                        h522
                                                    }))
                                                } else {
                                                    Ok(Err::<Adt18, Bytes>(f223(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt18, Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<Adt18, Bytes>(v9))
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt18, Bytes>(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(Err::<Adt18, Bytes>(v5))
            }
        }
    }
}

pub fn f125(v0: Adt18) -> Bytes {
    let m523 = v0;
    match m523 {
        Adt18::C0(v1, _, _) => {
            v1
        }
        Adt18::C1(_, _, _) => {
            Bytes::lit(&[])
        }
        Adt18::C2(_) => {
            Bytes::lit(&[])
        }
        Adt18::C3(_, _) => {
            Bytes::lit(&[])
        }
        Adt18::C4 => {
            Bytes::lit(&[])
        }
    }
}

pub fn f126(v0: u64) -> R<Bytes> {
    f229(f230(), 10u64, v0)
}

pub fn f127(v0: Adt18) -> R<u64> {
    let m524 = v0;
    match m524 {
        Adt18::C0(_, _, r529) => {
            let v3 = (*r529).clone();
            let a525 = 1u64;
            let a526 = f127(v3)?;
            nat_add(a525, a526)
        }
        Adt18::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt18::C2(_) => {
            Ok(0u64)
        }
        Adt18::C3(r530, r531) => {
            let v8 = (*r530).clone();
            let v9 = (*r531).clone();
            let a527 = f127(v8)?;
            let a528 = f127(v9)?;
            nat_add(a527, a528)
        }
        Adt18::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f128(p537: List<Bytes>) -> R<bool> {
    let m536 = p537;
    {
        let mut m536 = m536;
        loop {
            let step = {
                let v0 = m536;
                let m532 = v0;
                match m532.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let a534 = {
                            let a533 = f231(v1)?;
                            length_bytes(a533)
                        };
                        let a535 = 0u64;
                        if nat_eq(a534, a535) {
                            Err::<List<Bytes>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Bytes>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m538 = v0;
    match m538.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f231(v1)?;
            let a540 = {
                let a539 = v3.clone();
                length_bytes(a539)
            };
            let a541 = 0u64;
            if nat_eq(a540, a541) {
                f129(v2)
            } else {
                let m542 = f232(v3.clone())?;
                match m542 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f158(v4.clone(), f8(Str::lit("id")));
                        if if f36(f158(v4, f8(Str::lit("schema"))), f159()) {
                            f75({
                                let a544 = {
                                    let a543 = v5.clone();
                                    length_bytes(a543)
                                };
                                let a545 = 0u64;
                                nat_eq(a544, a545)
                            })
                        } else {
                            false
                        } {
                            let m546 = f129(v2)?;
                            match m546 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f233(v5), {
                                        let a547 = v3;
                                        let a548 = Bytes::lit(&[0x0a]);
                                        append_bytes(a547, a548)
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

pub fn f130(p556: List<Bytes>) -> R<Option<Bytes>> {
    let m555 = p556;
    {
        let mut m555 = m555;
        loop {
            let step = {
                let v0 = m555;
                let m549 = v0;
                match m549.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        let m550 = f232(f231(v1)?)?;
                        match m550 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                            }
                            Some(v3) => {
                                let m551 = f160(v3.clone(), f8(Str::lit("source")));
                                match m551 {
                                    None => {
                                        Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                                    }
                                    Some(v4) => {
                                        if if f36(f158(v4.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
                                            let a553 = {
                                                let a552 = f134(f158(v4, f8(Str::lit("url"))))?;
                                                length_bytes(a552)
                                            };
                                            let a554 = 0u64;
                                            nat_eq(a553, a554)
                                        } else {
                                            false
                                        } {
                                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(f158(v3, f8(Str::lit("id"))))))
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
                Ok(next) => { m555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f132() -> Bytes {
    f8(Str::lit("model/matrix"))
}

pub fn f133() -> Bytes {
    f8(Str::lit("model/mappings"))
}

pub fn f134(v0: Bytes) -> R<Bytes> {
    Ok(f31(f30(v0, 47u8)?))
}

pub fn f135(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a557 = v0.clone();
        length_bytes(a557)
    };
    let v3: u64 = {
        let a558 = v1.clone();
        length_bytes(a558)
    };
    let a559 = v3;
    let a560 = v2;
    if nat_le(a559, a560) {
        let m566 = {
            let a563 = v0;
            let a564 = {
                let a561 = v2;
                let a562 = v3;
                nat_sub(a561, a562)
            };
            let a565 = v3;
            slice_bytes(a563, a564, a565)
        };
        match m566 {
            None => {
                false
            }
            Some(v4) => {
                let a567 = v4;
                let a568 = v1;
                equal(a567, a568)
            }
        }
    } else {
        false
    }
}

pub fn f136(v0: List<Bytes>, v1: bool) -> R<Option<List<Adt14>>> {
    let m569 = v0;
    match m569.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f231(v2)?;
            let a571 = {
                let a570 = v4.clone();
                length_bytes(a570)
            };
            let a572 = 0u64;
            if nat_eq(a571, a572) {
                f136(v3, v1)
            } else {
                let m573 = f232(v4)?;
                match m573 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        if if v1 {
                            f234(v5.clone())
                        } else {
                            f235(v5.clone())
                        } {
                            let m574 = f136(v3, v1)?;
                            match m574 {
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

pub fn f137(v0: List<Bytes>) -> R<bool> {
    if f236(f154(v0.clone())?) {
        Ok(f75({
            let a576 = {
                let a575 = f237(f154(v0)?);
                length_list(a575)
            };
            let a577 = 0u64;
            nat_eq(a576, a577)
        }))
    } else {
        Ok(false)
    }
}

pub fn f138(v0: List<Bytes>) -> R<bool> {
    if f236(f141(v0.clone(), f8(Str::lit("requirement")))?) {
        if f236(f141(v0.clone(), f8(Str::lit("admission")))?) {
            Ok(f75({
                let a579 = {
                    let a578 = f145(f237(f141(v0.clone(), f8(Str::lit("requirement")))?), f237(f141(v0, f8(Str::lit("admission")))?));
                    length_list(a578)
                };
                let a580 = 0u64;
                nat_eq(a579, a580)
            }))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f139() -> Bytes {
    f8(Str::lit("model/queue"))
}

pub fn f140() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f141(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m581 = v0;
    match m581.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f231(v2)?;
            let a583 = {
                let a582 = v4.clone();
                length_bytes(a582)
            };
            let a584 = 0u64;
            if nat_eq(a583, a584) {
                f141(v3, v1)
            } else {
                let m585 = f232(v4)?;
                match m585 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f238(v5.clone());
                        let a587 = {
                            let a586 = v6.clone();
                            length_bytes(a586)
                        };
                        let a588 = 0u64;
                        if nat_eq(a587, a588) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m589 = f141(v3, v1.clone())?;
                            match m589 {
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

pub fn f142(p592: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m591 = p592;
    {
        let mut m591 = m591;
        loop {
            let step = {
                let v0 = m591;
                let m590 = v0;
                match m590.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f86(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Bytes) -> Adt11 {
    Adt11::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f144(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f216(Adt14::C3(std::rc::Rc::new(f239(v1))))?))
}

pub fn f145(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m593 = v0;
    match m593.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f145(v3, v1))
        }
    }
}

pub fn f146(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m594 = v0;
    match m594.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f231(v1)?;
            let a596 = {
                let a595 = v3.clone();
                length_bytes(a595)
            };
            let a597 = 0u64;
            if nat_eq(a596, a597) {
                f146(v2)
            } else {
                let m598 = f240(v3.clone())?;
                match m598 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f241(v4.clone()) {
                            let m599 = f146(v2)?;
                            match m599 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f188(), List::cons(f8(Str::lit("/")), List::cons(f242(f151(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a600 = v3;
                                        let a601 = Bytes::lit(&[0x0a]);
                                        append_bytes(a600, a601)
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

pub fn f147(v0: Adt1, v1: Bytes) -> R<Option<Adt19>> {
    f240(f70(v0, v1))
}

pub fn f148(p607: Adt19, v1: Bytes) -> Option<Adt19> {
    let m606 = p607;
    {
        let mut m606 = m606;
        loop {
            let step = {
                let v0 = m606;
                let m602 = v0;
                match m602 {
                    Adt19::C0(_) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C1(_, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C2(_, _, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C3(_) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C4(_) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C5(r603) => {
                        let v5 = (*r603).clone();
                        Ok::<Adt19, Option<Adt19>>(v5)
                    }
                    Adt19::C6 => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C7(_, _) => {
                        Err::<Adt19, Option<Adt19>>(None::<Adt19>)
                    }
                    Adt19::C8(v2, r604, r605) => {
                        let v3 = (*r604).clone();
                        let v4 = (*r605).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt19, Option<Adt19>>(Some(v3))
                        } else {
                            Ok::<Adt19, Option<Adt19>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m608 = f60(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m608.uncons() {
        None => {
            let m610 = f142(f106(List::cons(Adt14::C4(std::rc::Rc::new({
                let m609 = f243(v4.clone());
                match m609 {
                    None => {
                        Adt14::C5
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt14>::nil())), f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m610 {
                None => {
                    f244(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f143(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt11::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f150(v0: Adt19) -> R<Bytes> {
    let a611 = f245(v0)?;
    let a612 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a611, a612))
}

pub fn f151(v0: Adt19, v1: Bytes) -> Bytes {
    let m613 = f246(f148(v0, v1));
    match m613 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f152(v0: List<Adt19>) -> List<Bytes> {
    let m614 = v0;
    match m614.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m615 = f246(Some(v1));
            match m615 {
                None => {
                    f152(v2)
                }
                Some(v3) => {
                    List::cons(v3, f152(v2))
                }
            }
        }
    }
}

pub fn f153(v0: Option<Adt19>) -> List<Adt19> {
    let m616 = v0;
    match m616 {
        None => {
            List::<Adt19>::nil()
        }
        Some(v1) => {
            f247(v1)
        }
    }
}

pub fn f154(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m617 = v0;
    match m617.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f231(v1)?;
            let a619 = {
                let a618 = v3.clone();
                length_bytes(a618)
            };
            let a620 = 0u64;
            if nat_eq(a619, a620) {
                f154(v2)
            } else {
                let m621 = f232(v3)?;
                match m621 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f203(v4.clone(), f8(Str::lit("agent"))) {
                            if f203(v4.clone(), f8(Str::lit("at"))) {
                                if f203(v4.clone(), f8(Str::lit("quote"))) {
                                    f248(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m622 = f154(v2)?;
                            match m622 {
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

pub fn f155(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f232(f70(v0, v1))
}

pub fn f156(p625: List<Adt14>) -> bool {
    let m624 = p625;
    {
        let mut m624 = m624;
        loop {
            let step = {
                let v0 = m624;
                let m623 = v0;
                match m623.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f36(f158(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f86(f8(Str::lit("studio")), f72(f249(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m624 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: Option<Adt14>) -> List<Adt14> {
    let m626 = v0;
    match m626 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f250(v1)
        }
    }
}

pub fn f158(v0: Adt14, v1: Bytes) -> Bytes {
    let m627 = f161(f160(v0, v1));
    match m627 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f159() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f160(p633: Adt14, v1: Bytes) -> Option<Adt14> {
    let m632 = p633;
    {
        let mut m632 = m632;
        loop {
            let step = {
                let v0 = m632;
                let m628 = v0;
                match m628 {
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
                    Adt14::C4(r629) => {
                        let v5 = (*r629).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r630, r631) => {
                        let v3 = (*r630).clone();
                        let v4 = (*r631).clone();
                        if f36(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
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

pub fn f161(v0: Option<Adt14>) -> Option<Bytes> {
    let m634 = v0;
    match m634 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m635 = v1;
            match m635 {
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

pub fn f162(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f119(f70(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f251(v1, v4)?)?), List::cons(f120(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f163(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h636, _) = v2.clone();
        h636
    };
    let v4: u64 = {
        let (h637, _) = {
            let (_, h638) = v2.clone();
            h638
        };
        h637
    };
    let v5: List<Bytes> = {
        let (_, h639) = {
            let (_, h640) = v2;
            h640
        };
        h639
    };
    let m641 = f252(v0.clone(), v3);
    match m641 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a642 = v6;
            let a643 = v1;
            if equal(a642, a643) {
                Ok(Some(({
                    let a644 = v3;
                    let a645 = 1u64;
                    nat_add(a644, a645)?
                }, ({
                    let a646 = v3;
                    let a647 = 1u64;
                    nat_add(a646, a647)?
                }, List::cons(f32(v0, v4, {
                    let a648 = v3;
                    let a649 = v4;
                    nat_sub(a648, a649)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a650 = v3;
                    let a651 = 1u64;
                    nat_add(a650, a651)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f164(v0: Fn1, p655: List<Bytes>, p656: List<Bytes>) -> List<Bytes> {
    let m654 = (p655, p656);
    {
        let mut m654 = m654;
        loop {
            let step = {
                let (v1, v2) = m654;
                let m652 = v2;
                match m652.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c653: Fn1 = v0.clone();
                            c653.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m654 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f166(p661: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m660 = p661;
    {
        let mut m660 = m660;
        loop {
            let step = {
                let v0 = m660;
                let m657 = v0;
                match m657.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f36({
                            let (h658, _) = v2;
                            h658
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h659) = v2;
                                h659
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Adt1, v1: Bytes) -> bool {
    let m663 = f166(match v0 {
        Adt1::C0(h662, _, _) => {
            h662
        }
    }, v1);
    match m663 {
        None => {
            false
        }
        Some(v2) => {
            let m664 = v2;
            match m664 {
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

pub fn f168(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m665 = v1;
    match m665.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f253(v3.clone())
            } else {
                f75(f253(v3.clone()))
            } {
                List::cons(f131(v0.clone(), v3), f168(v0, v4, v2))
            } else {
                f168(v0, v4, v2)
            }
        }
    }
}

pub fn f169(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a666 = v0.clone();
        length_bytes(a666)
    };
    let v3: u64 = {
        let a667 = v1.clone();
        length_bytes(a667)
    };
    let v4: u64 = {
        let (h668, _) = f254(Fn3::F255(v1.clone(), v3, v0.clone(), v2), {
            let a669 = v2;
            let a670 = 1u64;
            nat_add(a669, a670)?
        }, 0u64)?;
        h668
    };
    if f220(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f170() -> Bytes {
    f7(List::cons(f8(Str::lit("Classical")), List::cons(f8(Str::lit(".choice")), List::<Bytes>::nil())))
}

pub fn f171(v0: Adt12) -> R<Adt16> {
    Ok(Adt16::C0(f256(match v0.clone() {
        Adt12::C0(h671, _, _, _, _) => {
            h671
        }
    }, 0u64)?, f256(match v0.clone() {
        Adt12::C0(_, h672, _, _, _) => {
            h672
        }
    }, 0u64)?, f256(match v0.clone() {
        Adt12::C0(_, _, h673, _, _) => {
            h673
        }
    }, 0u64)?, f256(match v0 {
        Adt12::C0(_, _, _, h674, _) => {
            h674
        }
    }, 0u64)?))
}

pub fn f172(v0: Adt12) -> List<Bytes> {
    f64(match v0.clone() {
        Adt12::C0(h675, _, _, _, _) => {
            h675
        }
    }, f64(match v0.clone() {
        Adt12::C0(_, _, h676, _, _) => {
            h676
        }
    }, f64(match v0.clone() {
        Adt12::C0(_, _, _, h677, _) => {
            h677
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h678) => {
            h678
        }
    })))
}

pub fn f173(v0: Adt16, p684: List<u64>, p685: List<Adt20>) -> Option<u64> {
    let m683 = (p684, p685);
    {
        let mut m683 = m683;
        loop {
            let step = {
                let (v1, v2) = m683;
                let m679 = v2;
                match m679.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt20>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f257(v0.clone(), v1.clone(), match v3.clone() {
                            Adt20::C0(_, h680) => {
                                h680
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt20>), Option<u64>>((List::cons(match v3 {
                                Adt20::C0(h681, _) => {
                                    h681
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt20>), Option<u64>>(Some(match v3 {
                                Adt20::C0(h682, _) => {
                                    h682
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m683 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f174(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt20>> {
    let m686 = v1;
    match m686.uncons() {
        None => {
            Ok(List::<Adt20>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt20::C0(v2, f258(v0.clone(), v3)?), f174(v0, v4, {
                let a687 = v2;
                let a688 = 1u64;
                nat_add(a687, a688)?
            })?))
        }
    }
}

pub fn f175(p692: List<Bytes>, p693: u64) -> Bytes {
    let m691 = (p692, p693);
    {
        let mut m691 = m691;
        loop {
            let step = {
                let (v0, v1) = m691;
                let m689 = v0;
                match m689.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m690 = v1;
                        if m690 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m690 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f176(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f259(v0.clone(), v1.clone()) {
        if f259(v1, v0) {
            f260(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f177(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m694 = v0;
    match m694.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f261(v2, v1.clone())?, f177(v3, v1)?))
        }
    }
}

pub fn f178(v0: List<Adt14>) -> List<u64> {
    let m695 = v0;
    match m695.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f262(v1), f178(v2))
        }
    }
}

pub fn f179(p698: List<u64>, v1: List<u64>) -> Option<u64> {
    let m697 = p698;
    {
        let mut m697 = m697;
        loop {
            let step = {
                let v0 = m697;
                let m696 = v0;
                match m696.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f263(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m697 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(p701: List<Bytes>, p702: List<Bytes>) -> Option<Bytes> {
    let m700 = (p701, p702);
    {
        let mut m700 = m700;
        loop {
            let step = {
                let (v0, v1) = m700;
                let m699 = v0;
                match m699.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f86(v2.clone(), v1.clone()) {
                            Err::<(List<Bytes>, List<Bytes>), Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<(List<Bytes>, List<Bytes>), Option<Bytes>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m703 = v0;
    match m703.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m704 = f264(v2, v1.clone())?;
            match m704 {
                None => {
                    f181(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f181(v3, v1)?))
                }
            }
        }
    }
}

pub fn f182(v0: List<Bytes>) -> List<Bytes> {
    f164(Fn1::F265, List::<Bytes>::nil(), v0)
}

pub fn f183(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f266(v0, v1)
}

pub fn f184(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f267(f268(f269(v0))?))
}

pub fn f185(v0: Adt1, v1: List<List<Bytes>>, v2: Bytes) -> R<bool> {
    let m705 = f155(v0, v2)?;
    match m705 {
        None => {
            Ok(false)
        }
        Some(v3) => {
            let v4: List<Bytes> = f270(v1, f158(v3.clone(), f8(Str::lit("id"))))?;
            let m706 = v4.clone();
            match m706.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m708 = f161(f160({
                        let m707 = f160(v3.clone(), f8(Str::lit("source")));
                        match m707 {
                            None => {
                                Adt14::C5
                            }
                            Some(v7) => {
                                v7
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m708 {
                        None => {
                            Ok(false)
                        }
                        Some(v8) => {
                            Ok(f75(f86(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f210(f271(v3, v8))?, List::<Bytes>::nil()))), v4)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f186(p711: List<Bytes>, p712: List<Bytes>) -> List<Bytes> {
    let m710 = (p711, p712);
    {
        let mut m710 = m710;
        loop {
            let step = {
                let (v0, v1) = m710;
                let m709 = v0;
                match m709.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m710 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f272(v0, v1)
}

pub fn f188() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f189(v0: Adt1, v1: List<Bytes>, v2: List<Bytes>) -> R<List<Bytes>> {
    let m713 = v1;
    match m713.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let v5: Adt19 = f273(v0.clone(), v3)?;
            if f86(f274(f151(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                Ok(List::cons(f275(v5)?, f189(v0, v4, v2)?))
            } else {
                f189(v0, v4, v2)
            }
        }
    }
}

pub fn f190(v0: Adt1, p717: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m716 = p717;
    {
        let mut m716 = m716;
        loop {
            let step = {
                let v1 = m716;
                let m714 = v1;
                match m714.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt19 = f273(v0.clone(), v3.clone())?;
                        if if f86(f274(f151(v5.clone(), f8(Str::lit("name"))))?, v2.clone()) {
                            f36(f150({
                                let m715 = f148(v5.clone(), f8(Str::lit("schema")));
                                match m715 {
                                    None => {
                                        Adt19::C6
                                    }
                                    Some(v6) => {
                                        v6
                                    }
                                }
                            })?, f70(v0.clone(), f275(v5)?))
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
                Ok(next) => { m716 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f75(f76(match v0.clone() {
        Adt1::C0(_, h718, _) => {
            h718
        }
    }, f139())) {
        Ok(Adt11::C1(List::cons(f139(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f84(f78(f276(f80(match v0.clone() {
            Adt1::C0(_, h719, _) => {
                h719
            }
        }, f139())))?, v1);
        let m720 = f60(v0.clone(), List::cons(f89(), List::cons(f117(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()))))));
        match m720.uncons() {
            None => {
                let v5: (bool, Bytes) = f277(v0.clone(), v2)?;
                let (h721, _) = v5;
                if h721 {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let (_, h722) = v5;
                        h722
                    }))
                } else {
                    let m723 = f88(v0.clone());
                    match m723.uncons() {
                        None => {
                            let m724 = f90(v0.clone())?;
                            match m724.uncons() {
                                None => {
                                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons({
                                        let (_, h725) = v5;
                                        h725
                                    }, List::cons(f7(f78(f278(f102(v0.clone())?, f110(v0.clone(), f111(v0.clone())?)?))?), List::cons(f279(v0)?, List::<Bytes>::nil()))))))
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

pub fn f192(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f280(f73(v0, f89())?)?)))
}

pub fn f193() -> Bytes {
    f8(Str::lit("the Lean export failed without an error line; its last lines follow"))
}

pub fn f194(v0: List<List<Bytes>>, v1: List<Bytes>, v2: List<Adt23>, v3: List<Adt14>, v4: List<Bytes>) -> List<Bytes> {
    let m726 = v1;
    match m726.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v5, v6)) => {
            let v7: List<Bytes> = f64(f281(v5.clone(), v2.clone()), f282(v5.clone(), v3.clone()));
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(v5.clone(), List::cons(f8(Str::lit("** ")), List::cons(f283(v0.clone(), v5.clone()), List::cons(if f86(v5, v4.clone()) {
                Bytes::lit(&[])
            } else {
                f8(Str::lit(", not reached from any origin"))
            }, List::cons(f8(Str::lit(": ")), List::cons({
                let a728 = {
                    let a727 = v7.clone();
                    length_list(a727)
                };
                let a729 = 0u64;
                if nat_eq(a728, a729) {
                    f8(Str::lit("no mapping yet"))
                } else {
                    f121(v7, f8(Str::lit("; ")))
                }
            }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))))), f194(v0, v6, v2, v3, v4))
        }
    }
}

pub fn f195(v0: Adt15, v1: List<Adt14>, v2: List<Adt14>) -> R<List<Adt23>> {
    let v3: List<(Bytes, Bytes)> = f284(v1);
    let v4: List<(Bytes, Bytes)> = f285(v0, v2);
    f286(v3.clone(), f91(v3, v4.clone()), v4, List::<(Bytes, Bytes)>::nil())
}

pub fn f196(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m730 = v1;
    match m730.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if if f203(v2.clone(), f8(Str::lit("domain"))) {
                f287(v0.clone(), v2.clone())
            } else {
                false
            } {
                List::cons(f7(List::cons(f8(Str::lit("- ")), List::cons(f158(v2.clone(), f8(Str::lit("domain"))), List::cons(f8(Str::lit(", row ")), List::cons(f158(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(": **")), List::cons({
                    let m731 = f160(v2.clone(), f8(Str::lit("role")));
                    match m731 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v4) => {
                            f158(v4, f8(Str::lit("term")))
                        }
                    }
                }, List::cons(f8(Str::lit("** is ")), List::cons(f158(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" of ")), List::cons(f158(v2.clone(), f8(Str::lit("artifact"))), List::cons(f8(Str::lit(" under ")), List::cons(f288(v2.clone()), List::cons(f8(Str::lit(" (")), List::cons({
                    let m732 = f160(v2.clone(), f8(Str::lit("origin")));
                    match m732 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v5) => {
                            f158(v5, f8(Str::lit("kind")))
                        }
                    }
                }, List::cons(f8(Str::lit(")\u{a}  > ")), List::cons({
                    let m733 = f160(v2, f8(Str::lit("evidence")));
                    match m733 {
                        None => {
                            Bytes::lit(&[])
                        }
                        Some(v6) => {
                            f158(v6, f8(Str::lit("quote")))
                        }
                    }
                }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))))))))))))), f196(v0, v3))
            } else {
                f196(v0, v3)
            }
        }
    }
}

pub fn f197(v0: List<Adt14>) -> List<Bytes> {
    let m734 = v0;
    match m734.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f158(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f158(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f158(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f197(v2))
        }
    }
}

pub fn f198(v0: List<Adt14>) -> List<Bytes> {
    let m735 = v0;
    match m735.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f158(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f289(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f198(v2))
        }
    }
}

pub fn f199(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m736 = v0;
    match m736.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((f158(v1.clone(), f8(Str::lit("subject"))), f158(v1, f8(Str::lit("object")))), f199(v2))
        }
    }
}

pub fn f200(v0: List<Adt23>) -> List<(Bytes, Bytes)> {
    let m737 = v0;
    match m737.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            List::cons((match v1.clone() {
                Adt23::C0(h738, _, _) => {
                    h738
                }
            }, match v1 {
                Adt23::C0(_, _, h739) => {
                    h739
                }
            }), f200(v2))
        }
    }
}

pub fn f201(p746: List<Bytes>, v1: List<(Bytes, Bytes)>, p747: u64) -> R<List<Bytes>> {
    let m745 = (p746, p747);
    {
        let mut m745 = m745;
        loop {
            let step = {
                let (v0, v2) = m745;
                let m740 = v2;
                if m740 == 0 {
                    Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                } else {
                    let v3 = m740 - 1;
                    let v4: List<Bytes> = f78(f64(v0.clone(), f290(v0.clone(), v1.clone())))?;
                    let a743 = {
                        let a741 = v4.clone();
                        length_list(a741)
                    };
                    let a744 = {
                        let a742 = v0.clone();
                        length_list(a742)
                    };
                    if nat_eq(a743, a744) {
                        Err::<(List<Bytes>, u64), R<List<Bytes>>>(Ok(v0))
                    } else {
                        Ok::<(List<Bytes>, u64), R<List<Bytes>>>((v4, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m745 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: List<Adt14>) -> List<Bytes> {
    let m748 = v0;
    match m748.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m749 = f160(v1, f8(Str::lit("std")));
            match m749 {
                None => {
                    f202(v2)
                }
                Some(v3) => {
                    List::cons(f158(v3.clone(), f8(Str::lit("authority"))), if f203(v3.clone(), f8(Str::lit("row"))) {
                        List::cons(f158(v3, f8(Str::lit("row"))), f202(v2))
                    } else {
                        f202(v2)
                    })
                }
            }
        }
    }
}

pub fn f203(v0: Adt14, v1: Bytes) -> bool {
    f75({
        let a751 = {
            let a750 = f158(v0, v1);
            length_bytes(a750)
        };
        let a752 = 0u64;
        nat_eq(a751, a752)
    })
}

pub fn f204(v0: Adt15, v1: Adt14) -> bool {
    if f291(v0.clone(), v1.clone(), f158(v1.clone(), f8(Str::lit("subject")))) {
        f291(v0, v1.clone(), f158(v1, f8(Str::lit("object"))))
    } else {
        false
    }
}

pub fn f205(v0: Bytes, v1: Bytes) -> bool {
    let m757 = {
        let a754 = v0;
        let a755 = 0u64;
        let a756 = {
            let a753 = v1.clone();
            length_bytes(a753)
        };
        slice_bytes(a754, a755, a756)
    };
    match m757 {
        None => {
            false
        }
        Some(v2) => {
            let a758 = v2;
            let a759 = v1;
            equal(a758, a759)
        }
    }
}

pub fn f206(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m760 = v1;
    match m760.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f135(v2.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f131(v0.clone(), v2), f206(v0, v3))
            } else {
                f206(v0, v3)
            }
        }
    }
}

pub fn f207(v0: Bytes, p818: u64, p819: u64, p820: Adt17) -> R<Adt17> {
    let m817 = (p818, (p819, p820));
    {
        let mut m817 = m817;
        loop {
            let step = {
                let (v1, (v2, v3)) = m817;
                let m761 = v2;
                if m761 == 0 {
                    Err::<(u64, (u64, Adt17)), R<Adt17>>(Ok(v3))
                } else {
                    let v4 = m761 - 1;
                    let v5: u64 = f292(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt17::C0(_, h762, _, _, _) => {
                            h762
                        }
                    } {
                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                            let a763 = v1;
                            let a764 = 1u64;
                            nat_add(a763, a764)?
                        }, (v4, Adt17::C0(true, false, match v3.clone() {
                            Adt17::C0(_, _, h765, _, _) => {
                                h765
                            }
                        }, match v3.clone() {
                            Adt17::C0(_, _, _, h766, _) => {
                                h766
                            }
                        }, match v3 {
                            Adt17::C0(_, _, _, _, h767) => {
                                h767
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt17::C0(h768, _, _, _, _) => {
                                h768
                            }
                        } {
                            let a769 = v5;
                            let a770 = 34u64;
                            if nat_eq(a769, a770) {
                                let a773 = f292(v0.clone(), {
                                    let a771 = v1;
                                    let a772 = 1u64;
                                    nat_add(a771, a772)?
                                })?;
                                let a774 = 34u64;
                                if nat_eq(a773, a774) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a775 = v1;
                                        let a776 = 1u64;
                                        nat_add(a775, a776)?
                                    }, (v4, Adt17::C0(true, true, f293(match v3.clone() {
                                        Adt17::C0(_, _, h777, _, _) => {
                                            h777
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt17::C0(_, _, _, h778, _) => {
                                            h778
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h779) => {
                                            h779
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a780 = v1;
                                        let a781 = 1u64;
                                        nat_add(a780, a781)?
                                    }, (v4, Adt17::C0(false, false, match v3.clone() {
                                        Adt17::C0(_, _, h782, _, _) => {
                                            h782
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h783, _) => {
                                            h783
                                        }
                                    }, match v3 {
                                        Adt17::C0(_, _, _, _, h784) => {
                                            h784
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a785 = v1;
                                    let a786 = 1u64;
                                    nat_add(a785, a786)?
                                }, (v4, Adt17::C0(true, false, f293(match v3.clone() {
                                    Adt17::C0(_, _, h787, _, _) => {
                                        h787
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt17::C0(_, _, _, h788, _) => {
                                        h788
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h789) => {
                                        h789
                                    }
                                }))))
                            }
                        } else {
                            let a790 = v5;
                            let a791 = 34u64;
                            if nat_eq(a790, a791) {
                                Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                    let a792 = v1;
                                    let a793 = 1u64;
                                    nat_add(a792, a793)?
                                }, (v4, Adt17::C0(true, false, match v3.clone() {
                                    Adt17::C0(_, _, h794, _, _) => {
                                        h794
                                    }
                                }, match v3.clone() {
                                    Adt17::C0(_, _, _, h795, _) => {
                                        h795
                                    }
                                }, match v3 {
                                    Adt17::C0(_, _, _, _, h796) => {
                                        h796
                                    }
                                }))))
                            } else {
                                let a797 = v5;
                                let a798 = 44u64;
                                if nat_eq(a797, a798) {
                                    Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                        let a799 = v1;
                                        let a800 = 1u64;
                                        nat_add(a799, a800)?
                                    }, (v4, Adt17::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt17::C0(_, _, h801, _, _) => {
                                            h801
                                        }
                                    }, match v3.clone() {
                                        Adt17::C0(_, _, _, h802, _) => {
                                            h802
                                        }
                                    }), match v3 {
                                        Adt17::C0(_, _, _, _, h803) => {
                                            h803
                                        }
                                    }))))
                                } else {
                                    let a804 = v5;
                                    let a805 = 13u64;
                                    if nat_eq(a804, a805) {
                                        Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                            let a806 = v1;
                                            let a807 = 1u64;
                                            nat_add(a806, a807)?
                                        }, (v4, v3)))
                                    } else {
                                        let a808 = v5;
                                        let a809 = 10u64;
                                        if nat_eq(a808, a809) {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a810 = v1;
                                                let a811 = 1u64;
                                                nat_add(a810, a811)?
                                            }, (v4, f208(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt17)), R<Adt17>>(({
                                                let a812 = v1;
                                                let a813 = 1u64;
                                                nat_add(a812, a813)?
                                            }, (v4, Adt17::C0(false, false, f293(match v3.clone() {
                                                Adt17::C0(_, _, h814, _, _) => {
                                                    h814
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt17::C0(_, _, _, h815, _) => {
                                                    h815
                                                }
                                            }, match v3 {
                                                Adt17::C0(_, _, _, _, h816) => {
                                                    h816
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
                Ok(next) => { m817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Adt17) -> Adt17 {
    Adt17::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f294(List::cons(match v0.clone() {
        Adt17::C0(_, _, h821, _, _) => {
            h821
        }
    }, match v0.clone() {
        Adt17::C0(_, _, _, h822, _) => {
            h822
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt17::C0(_, _, _, _, h823) => {
            h823
        }
    }))
}

pub fn f209(p826: List<List<Bytes>>, p827: List<List<Bytes>>) -> List<List<Bytes>> {
    let m825 = (p826, p827);
    {
        let mut m825 = m825;
        loop {
            let step = {
                let (v0, v1) = m825;
                let m824 = v0;
                match m824.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m825 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f210(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f295(v0.clone(), 0u64, {
        let a828 = v0;
        length_bytes(a828)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f211(v0: Adt24) -> Bytes {
    let a849 = f296(match v0.clone() {
        Adt24::C0(h829, _, _, _, _, _, _, _) => {
            h829
        }
    });
    let a850 = {
        let a847 = f296(match v0.clone() {
            Adt24::C0(_, h830, _, _, _, _, _, _) => {
                h830
            }
        });
        let a848 = {
            let a845 = f296(match v0.clone() {
                Adt24::C0(_, _, h831, _, _, _, _, _) => {
                    h831
                }
            });
            let a846 = {
                let a843 = f296(match v0.clone() {
                    Adt24::C0(_, _, _, h832, _, _, _, _) => {
                        h832
                    }
                });
                let a844 = {
                    let a841 = f296(match v0.clone() {
                        Adt24::C0(_, _, _, _, h833, _, _, _) => {
                            h833
                        }
                    });
                    let a842 = {
                        let a839 = f296(match v0.clone() {
                            Adt24::C0(_, _, _, _, _, h834, _, _) => {
                                h834
                            }
                        });
                        let a840 = {
                            let a837 = f296(match v0.clone() {
                                Adt24::C0(_, _, _, _, _, _, h835, _) => {
                                    h835
                                }
                            });
                            let a838 = f296(match v0 {
                                Adt24::C0(_, _, _, _, _, _, _, h836) => {
                                    h836
                                }
                            });
                            append_bytes(a837, a838)
                        };
                        append_bytes(a839, a840)
                    };
                    append_bytes(a841, a842)
                };
                append_bytes(a843, a844)
            };
            append_bytes(a845, a846)
        };
        append_bytes(a847, a848)
    };
    append_bytes(a849, a850)
}

pub fn f212(v0: Bytes) -> R<Adt24> {
    let v1: u64 = {
        let a851 = v0.clone();
        length_bytes(a851)
    };
    let v2: u64 = {
        let a857 = {
            let a854 = {
                let a852 = v1;
                let a853 = 72u64;
                nat_add(a852, a853)?
            };
            let a855 = 64u64;
            let a856 = 0u64;
            nat_quot(a854, a855, a856)
        };
        let a858 = 64u64;
        nat_mul(a857, a858)?
    };
    let (_, h859) = f297(Fn4::F298(v0, v1, v2), {
        let a860 = v2;
        let a861 = 64u64;
        let a862 = 0u64;
        nat_quot(a860, a861, a862)
    }, (0u64, f299()))?;
    Ok(h859)
}

pub fn f213(p865: List<List<Bytes>>, v1: Bytes, p866: List<Bytes>) -> R<List<Bytes>> {
    let m864 = (p865, p866);
    {
        let mut m864 = m864;
        loop {
            let step = {
                let (v0, v2) = m864;
                let m863 = v0;
                match m863.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f300(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f215(f301(v3), Bytes::lit(&[0x0a])), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m864 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f214(v0: Bytes) -> R<Bytes> {
    Ok(f32(v0.clone(), 0u64, f302(v0)?))
}

pub fn f215(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m867 = v0;
    match m867.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m868 = v3.clone();
            match m868.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a871 = v2;
                    let a872 = {
                        let a869 = v1.clone();
                        let a870 = f215(v3, v1);
                        append_bytes(a869, a870)
                    };
                    append_bytes(a871, a872)
                }
            }
        }
    }
}

pub fn f216(v0: Adt14) -> R<Bytes> {
    let a873 = f303(v0)?;
    let a874 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a873, a874))
}

pub fn f217(v0: List<Bytes>) -> Adt14 {
    let m875 = v0;
    match m875.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f217(v2)))
        }
    }
}

pub fn f218(v0: Fn2, p879: Option<Bytes>, p880: List<Bytes>) -> Option<Bytes> {
    let m878 = (p879, p880);
    {
        let mut m878 = m878;
        loop {
            let step = {
                let (v1, v2) = m878;
                let m876 = v2;
                match m876.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c877: Fn2 = v0.clone();
                            c877.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m878 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m881 = v1;
    match m881 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a884 = {
                    let a882 = v3;
                    let a883 = v0;
                    append_bytes(a882, a883)
                };
                let a885 = v2;
                append_bytes(a884, a885)
            })
        }
    }
}

pub fn f220(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m890 = {
        let a887 = v0;
        let a888 = v2;
        let a889 = {
            let a886 = v1.clone();
            length_bytes(a886)
        };
        slice_bytes(a887, a888, a889)
    };
    match m890 {
        None => {
            false
        }
        Some(v3) => {
            let a891 = v3;
            let a892 = v1;
            equal(a891, a892)
        }
    }
}

pub fn f221(v0: Bytes) -> R<Bytes> {
    let (_, h893) = {
        let (h894, _) = f304(Fn5::F305(v0.clone()), {
            let a896 = {
                let a895 = v0;
                length_bytes(a895)
            };
            let a897 = 1u64;
            nat_add(a896, a897)?
        }, (0u64, Bytes::lit(&[])))?;
        h894
    };
    Ok(h893)
}

pub fn f222(v0: Bytes) -> R<u64> {
    let (h898, _) = f254(Fn3::F306(v0.clone()), {
        let a900 = {
            let a899 = v0;
            length_bytes(a899)
        };
        let a901 = 1u64;
        nat_add(a900, a901)?
    }, 0u64)?;
    Ok(h898)
}

pub fn f223(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a902 = v0;
        utf8_encode(a902)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f126(v1)?, List::<Bytes>::nil())))))
}

pub fn f224(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f226(v0.clone(), 0u64, Str::lit("<?xml")) {
        f307(f227(v0.clone(), 5u64)?)
    } else {
        false
    } {
        let m904 = f308(v0.clone(), {
            let a903 = Str::lit("?>");
            utf8_encode(a903)
        }, 5u64)?;
        match m904 {
            None => {
                Ok(Err::<u64, Bytes>(f223(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m906 = f308(f32(v0.clone(), 0u64, v1), {
                    let a905 = Str::lit("encoding");
                    utf8_encode(a905)
                }, 5u64)?;
                match m906 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a907 = v1;
                            let a908 = 2u64;
                            nat_add(a907, a908)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f309(v0.clone(), {
                            let a909 = v2;
                            let a910 = 8u64;
                            nat_add(a909, a910)?
                        })?;
                        let v4: u64 = f309(v0.clone(), {
                            let a911 = v3;
                            let a912 = 1u64;
                            nat_add(a911, a912)?
                        })?;
                        let _: u64 = f227(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m915 = f308(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a913 = v4;
                                let a914 = 1u64;
                                nat_add(a913, a914)?
                            })?;
                            match m915 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f310(f32(v0, {
                            let a916 = v4;
                            let a917 = 1u64;
                            nat_add(a916, a917)?
                        }, {
                            let a920 = v7;
                            let a921 = {
                                let a918 = v4;
                                let a919 = 1u64;
                                nat_add(a918, a919)?
                            };
                            nat_sub(a920, a921)
                        }))? {
                            Ok(Ok::<u64, Bytes>({
                                let a922 = v1;
                                let a923 = 2u64;
                                nat_add(a922, a923)?
                            }))
                        } else {
                            Ok(Err::<u64, Bytes>(f223(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f225(v0: Bytes, p928: u64, p929: u64) -> R<Result<u64, Bytes>> {
    let m927 = (p928, p929);
    {
        let mut m927 = m927;
        loop {
            let step = {
                let (v1, v2) = m927;
                let m924 = v2;
                if m924 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(f223(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m924 - 1;
                    let v4: u64 = f309(v0.clone(), v1)?;
                    if f226(v0.clone(), v4, Str::lit("<!--")) {
                        let m925 = f311(v0.clone(), v4)?;
                        match m925 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v5)))
                            }
                        }
                    } else {
                        if f226(v0.clone(), v4, Str::lit("<?")) {
                            let m926 = f312(v0.clone(), v4)?;
                            match m926 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m927 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(v0: Bytes, v1: u64, v2: Str) -> bool {
    f220(v0, {
        let a930 = v2;
        utf8_encode(a930)
    }, v1)
}

pub fn f227(v0: Bytes, v1: u64) -> R<u64> {
    f313(v0, v1)
}

pub fn f228(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt18, u64), Bytes>> {
    let m931 = v2;
    if m931 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m931 - 1;
        let a932 = v3;
        let a933 = 0u64;
        if nat_eq(a932, a933) {
            let v5: u64 = f314(v0.clone(), {
                let a934 = v1;
                let a935 = 1u64;
                nat_add(a934, a935)?
            })?;
            let a938 = v5;
            let a939 = {
                let a936 = v1;
                let a937 = 1u64;
                nat_add(a936, a937)?
            };
            if nat_eq(a938, a939) {
                Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a940 = v1;
                    let a941 = 1u64;
                    nat_add(a940, a941)?
                }, {
                    let a944 = v5;
                    let a945 = {
                        let a942 = v1;
                        let a943 = 1u64;
                        nat_add(a942, a943)?
                    };
                    nat_sub(a944, a945)
                });
                let m946 = f315(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m946 {
                    Ok(v8) => {
                        if f226(v0.clone(), {
                            let (_, h947) = v8.clone();
                            h947
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                let (h948, _) = v8;
                                h948
                            }), std::rc::Rc::new(Adt18::C4)), {
                                let a950 = {
                                    let (_, h949) = v8;
                                    h949
                                };
                                let a951 = 2u64;
                                nat_add(a950, a951)?
                            })))
                        } else {
                            let m955 = f228(v0.clone(), {
                                let a953 = {
                                    let (_, h952) = v8;
                                    h952
                                };
                                let a954 = 1u64;
                                nat_add(a953, a954)?
                            }, v4, 1u64)?;
                            match m955 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a957 = {
                                            let (_, h956) = v10.clone();
                                            h956
                                        };
                                        let a958 = 2u64;
                                        nat_add(a957, a958)?
                                    };
                                    let v12: u64 = f314(v0.clone(), v11)?;
                                    if !f36(f32(v0.clone(), v11, {
                                        let a959 = v12;
                                        let a960 = v11;
                                        nat_sub(a959, a960)
                                    }), v6.clone()) {
                                        Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h961) = v10;
                                            h961
                                        })?))
                                    } else {
                                        let v13: u64 = f309(v0.clone(), v12)?;
                                        let a962 = f227(v0, v13)?;
                                        let a963 = 62u64;
                                        if nat_eq(a962, a963) {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C0(v6, std::rc::Rc::new({
                                                let (h964, _) = v8;
                                                h964
                                            }), std::rc::Rc::new({
                                                let (h965, _) = v10;
                                                h965
                                            })), {
                                                let a966 = v13;
                                                let a967 = 1u64;
                                                nat_add(a966, a967)?
                                            })))
                                        } else {
                                            Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<(Adt18, u64), Bytes>(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<(Adt18, u64), Bytes>(v7))
                    }
                }
            }
        } else {
            let a968 = f227(v0.clone(), v1)?;
            let a969 = 256u64;
            if nat_eq(a968, a969) {
                Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f226(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v1)))
                } else {
                    if f226(v0.clone(), v1, Str::lit("<!--")) {
                        let m970 = f311(v0.clone(), v1)?;
                        match m970 {
                            Ok(v15) => {
                                f228(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(Err::<(Adt18, u64), Bytes>(v14))
                            }
                        }
                    } else {
                        if f226(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m974 = f308(v0.clone(), {
                                let a971 = Str::lit("]]>");
                                utf8_encode(a971)
                            }, {
                                let a972 = v1;
                                let a973 = 9u64;
                                nat_add(a972, a973)?
                            })?;
                            match m974 {
                                None => {
                                    Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m977 = f228(v0.clone(), {
                                        let a975 = v16;
                                        let a976 = 3u64;
                                        nat_add(a975, a976)?
                                    }, v4, 1u64)?;
                                    match m977 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2(f32(v0, {
                                                let a978 = v1;
                                                let a979 = 9u64;
                                                nat_add(a978, a979)?
                                            }, {
                                                let a982 = v16;
                                                let a983 = {
                                                    let a980 = v1;
                                                    let a981 = 9u64;
                                                    nat_add(a980, a981)?
                                                };
                                                nat_sub(a982, a983)
                                            }))), std::rc::Rc::new({
                                                let (h984, _) = v18;
                                                h984
                                            })), {
                                                let (_, h985) = v18;
                                                h985
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(Err::<(Adt18, u64), Bytes>(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f226(v0.clone(), v1, Str::lit("<?")) {
                                let m986 = f312(v0.clone(), v1)?;
                                match m986 {
                                    Ok(v20) => {
                                        f228(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v19))
                                    }
                                }
                            } else {
                                if f226(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a987 = f227(v0.clone(), v1)?;
                                    let a988 = 60u64;
                                    if nat_eq(a987, a988) {
                                        let m989 = f228(v0.clone(), v1, v4, 0u64)?;
                                        match m989 {
                                            Ok(v22) => {
                                                let m991 = f228(v0, {
                                                    let (_, h990) = v22;
                                                    h990
                                                }, v4, 1u64)?;
                                                match m991 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new({
                                                            let (h992, _) = v22;
                                                            h992
                                                        }), std::rc::Rc::new({
                                                            let (h993, _) = v24;
                                                            h993
                                                        })), {
                                                            let (_, h994) = v24;
                                                            h994
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<(Adt18, u64), Bytes>(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(Err::<(Adt18, u64), Bytes>(v21))
                                            }
                                        }
                                    } else {
                                        let m995 = f316(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m995 {
                                            Ok(v26) => {
                                                let m997 = f228(v0, {
                                                    let (_, h996) = v26;
                                                    h996
                                                }, v4, 1u64)?;
                                                match m997 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C3(std::rc::Rc::new(Adt18::C2({
                                                            let (h998, _) = v26;
                                                            h998
                                                        })), std::rc::Rc::new({
                                                            let (h999, _) = v28;
                                                            h999
                                                        })), {
                                                            let (_, h1000) = v28;
                                                            h1000
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(Err::<(Adt18, u64), Bytes>(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(Err::<(Adt18, u64), Bytes>(v25))
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

pub fn f229(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1001 = v2;
    let a1002 = 0u64;
    if nat_eq(a1001, a1002) {
        Ok(f317(v0, 0u64))
    } else {
        let (_, h1003) = {
            let (h1004, _) = f304(Fn5::F318(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1004
        };
        Ok(h1003)
    }
}

pub fn f230() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f231(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f302(v0.clone())?;
    let v2: u64 = f319(v0.clone(), v1)?;
    Ok(f32(v0, v2, {
        let a1005 = v1;
        let a1006 = v2;
        nat_sub(a1005, a1006)
    }))
}

pub fn f232(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f320(v0)?;
    let m1010 = f321(v1.clone(), 0u64, {
        let a1008 = {
            let a1007 = v1.clone();
            length_bytes(a1007)
        };
        let a1009 = 2u64;
        nat_add(a1008, a1009)?
    }, 0u64)?;
    match m1010 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a1013 = {
                let (_, h1011) = v2;
                h1011
            };
            let a1014 = {
                let a1012 = v1;
                length_bytes(a1012)
            };
            if nat_eq(a1013, a1014) {
                Ok(Some({
                    let (h1015, _) = v2;
                    h1015
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f233(v0: Bytes) -> Bytes {
    f7(List::cons(f77(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f234(v0: Adt14) -> bool {
    if f203(v0.clone(), f8(Str::lit("domain"))) {
        if f203(v0.clone(), f8(Str::lit("row"))) {
            if f86(f158(v0.clone(), f8(Str::lit("relation"))), f322()) {
                if f323(v0.clone(), f8(Str::lit("role")), f8(Str::lit("term"))) {
                    if f323(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("authority"))) {
                        if f323(v0.clone(), f8(Str::lit("evidence")), f8(Str::lit("quote"))) {
                            f323(v0, f8(Str::lit("origin")), f8(Str::lit("kind")))
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

pub fn f235(v0: Adt14) -> bool {
    if f203(v0.clone(), f8(Str::lit("subject"))) {
        if f203(v0.clone(), f8(Str::lit("object"))) {
            if f203(v0.clone(), f8(Str::lit("evidence"))) {
                f86(f158(v0, f8(Str::lit("relation"))), f324())
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

pub fn f236(v0: Option<List<Adt14>>) -> bool {
    let m1016 = v0;
    match m1016 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f237(v0: Option<List<Adt14>>) -> List<Adt14> {
    let m1017 = v0;
    match m1017 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f238(v0: Adt14) -> Bytes {
    if f203(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f203(v0.clone(), f8(Str::lit("id"))) {
            if f203(v0.clone(), f8(Str::lit("statement"))) {
                if f203(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f248(v0, f8(Str::lit("evidence")))
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

pub fn f239(v0: List<Adt14>) -> Adt14 {
    let m1018 = v0;
    match m1018.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f239(v2)))
        }
    }
}

pub fn f240(v0: Bytes) -> R<Option<Adt19>> {
    let v1: Bytes = f320(v0)?;
    let m1022 = f325(v1.clone(), 0u64, {
        let a1020 = {
            let a1019 = v1.clone();
            length_bytes(a1019)
        };
        let a1021 = 2u64;
        nat_add(a1020, a1021)?
    }, 0u64)?;
    match m1022 {
        None => {
            Ok(None::<Adt19>)
        }
        Some(v2) => {
            let a1025 = {
                let (_, h1023) = v2;
                h1023
            };
            let a1026 = {
                let a1024 = v1;
                length_bytes(a1024)
            };
            if nat_eq(a1025, a1026) {
                Ok(Some({
                    let (h1027, _) = v2;
                    h1027
                }))
            } else {
                Ok(None::<Adt19>)
            }
        }
    }
}

pub fn f241(v0: Adt19) -> bool {
    if f326(v0.clone(), f8(Str::lit("name"))) {
        if f327(v0.clone(), f8(Str::lit("schema"))) {
            let m1028 = f243(f152(f153(f148(v0, f8(Str::lit("citation"))))));
            match m1028 {
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

pub fn f242(v0: Bytes) -> R<Bytes> {
    f328(v0.clone(), 0u64, {
        let a1029 = v0;
        length_bytes(a1029)
    }, true, Bytes::lit(&[]))
}

pub fn f243(v0: List<Bytes>) -> Option<Adt14> {
    let m1030 = v0;
    match m1030.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("req"))) {
                let m1031 = v2;
                match m1031.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f36(v1.clone(), f8(Str::lit("std"))) {
                    let m1032 = v2;
                    match m1032.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m1033 = v6;
                                match m1033.uncons() {
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
                        let m1034 = v2;
                        match m1034.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m1035 = v10;
                                match m1035.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f217(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f244(v0: Adt1, v1: Bytes, v2: Adt19, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m1036 = f60(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m1036.uncons() {
        None => {
            let m1037 = f243(v4);
            match m1037 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a1038 = f8(Str::lit("Catalog."));
                        let a1039 = f242(v3.clone())?;
                        append_bytes(a1038, a1039)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f242(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m1040 = f329(v2, v9.clone(), f242(v3.clone())?)?;
                    match m1040 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f242(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f216(Adt14::C3(std::rc::Rc::new(f239(f330(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f216(Adt14::C3(std::rc::Rc::new(f239(f330(f73(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v10.clone(), Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a1044 = f331(f70(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f32(v10.clone(), 4u64, {
                                    let a1042 = {
                                        let a1041 = v10;
                                        length_bytes(a1041)
                                    };
                                    let a1043 = 4u64;
                                    nat_sub(a1042, a1043)
                                }))?;
                                let a1045 = Bytes::lit(&[0x0a]);
                                append_bytes(a1044, a1045)
                            }), List::cons(f120(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f245(v0: Adt19) -> R<Bytes> {
    let m1046 = v0;
    match m1046 {
        Adt19::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt19::C1(v2, v3) => {
            f332(v2, v3)
        }
        Adt19::C2(v4, v5, v6) => {
            f333(v4, v5, v6)
        }
        Adt19::C3(v7) => {
            f334(v7)
        }
        Adt19::C4(r1047) => {
            let v8 = (*r1047).clone();
            Ok(f335(f336(v8)?))
        }
        Adt19::C5(r1048) => {
            let v9 = (*r1048).clone();
            f337(f338(v9)?)
        }
        Adt19::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f246(v0: Option<Adt19>) -> Option<Bytes> {
    let m1049 = v0;
    match m1049 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1050 = v1;
            match m1050 {
                Adt19::C0(_) => {
                    None::<Bytes>
                }
                Adt19::C1(_, _) => {
                    None::<Bytes>
                }
                Adt19::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt19::C3(v2) => {
                    Some(v2)
                }
                Adt19::C4(_) => {
                    None::<Bytes>
                }
                Adt19::C5(_) => {
                    None::<Bytes>
                }
                Adt19::C6 => {
                    None::<Bytes>
                }
                Adt19::C7(_, _) => {
                    None::<Bytes>
                }
                Adt19::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f247(v0: Adt19) -> List<Adt19> {
    let m1051 = v0;
    match m1051 {
        Adt19::C0(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C1(_, _) => {
            List::<Adt19>::nil()
        }
        Adt19::C2(_, _, _) => {
            List::<Adt19>::nil()
        }
        Adt19::C3(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C4(r1052) => {
            let v3 = (*r1052).clone();
            f247(v3)
        }
        Adt19::C5(_) => {
            List::<Adt19>::nil()
        }
        Adt19::C6 => {
            List::<Adt19>::nil()
        }
        Adt19::C7(r1053, r1054) => {
            let v1 = (*r1053).clone();
            let v2 = (*r1054).clone();
            List::cons(v1, f247(v2))
        }
        Adt19::C8(_, _, _) => {
            List::<Adt19>::nil()
        }
    }
}

pub fn f248(v0: Adt14, v1: Bytes) -> bool {
    let m1055 = f160(v0, v1);
    match m1055 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f249(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m1056 = f160(v0, v1);
    match m1056 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f157(f160(v3, v2))
        }
    }
}

pub fn f250(v0: Adt14) -> List<Adt14> {
    let m1057 = v0;
    match m1057 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r1058) => {
            let v3 = (*r1058).clone();
            f250(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r1059, r1060) => {
            let v1 = (*r1059).clone();
            let v2 = (*r1060).clone();
            List::cons(v1, f250(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f251(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f160(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f116(Str::lit("id"), f158(v0.clone(), f8(Str::lit("id"))))?, List::cons(f116(Str::lit("name"), f158(v0.clone(), f8(Str::lit("name"))))?, List::cons(f116(Str::lit("issuer"), f158(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f116(Str::lit("edition"), f158(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f116(Str::lit("canonical_id"), f158(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f116(Str::lit("source_role"), f158(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f116(Str::lit("retrieval_date"), f158(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f116(Str::lit("url"), f339(v2.clone(), f8(Str::lit("url"))))?, List::cons(f116(Str::lit("media_type"), f339(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f116(Str::lit("license"), f339(v2.clone(), f8(Str::lit("license"))))?, List::cons(f116(Str::lit("redistribution"), f339(v2, f8(Str::lit("redistribution"))))?, List::cons(f116(Str::lit("statement"), f158(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m1061 = v1;
        match m1061 {
            None => {
                f7(List::cons(f116(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f116(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f116(Str::lit("artifact"), {
                    let (h1062, _) = v3;
                    h1062
                })?, List::cons(f116(Str::lit("checksum"), {
                    let (_, h1063) = v3;
                    h1063
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f340(f74(f157(f160(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f252(v0: Bytes, v1: u64) -> Option<u8> {
    let a1064 = v0;
    let a1065 = v1;
    index_bytes(a1064, a1065)
}

pub fn f253(v0: Bytes) -> bool {
    f135(v0, f8(Str::lit(".lex.tex")))
}

pub fn f254(v0: Fn3, p1070: u64, p1071: u64) -> R<(u64, bool)> {
    let m1069 = (p1070, p1071);
    {
        let mut m1069 = m1069;
        loop {
            let step = {
                let (v1, v2) = m1069;
                let m1066 = v1;
                if m1066 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1066 - 1;
                    let m1068 = {
                        let c1067: Fn3 = v0.clone();
                        c1067.apply(v2)?
                    };
                    match m1068 {
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
                Ok(next) => { m1069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f220(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1074 = {
            let a1072 = v4;
            let a1073 = v1;
            nat_add(a1072, a1073)?
        };
        let a1075 = v3;
        if nat_lt(a1074, a1075) {
            Ok(Some({
                let a1076 = v4;
                let a1077 = 1u64;
                nat_add(a1076, a1077)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f256(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m1078 = v0;
    match m1078.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f256(v3, {
                let a1079 = v1;
                let a1080 = 1u64;
                nat_add(a1079, a1080)?
            })?))
        }
    }
}

pub fn f257(v0: Adt16, v1: List<u64>, v2: Adt21) -> bool {
    let m1081 = v2;
    match m1081 {
        Adt21::C0(v3, v4) => {
            if f263(v3, match v0.clone() {
                Adt16::C0(h1082, _, _, _) => {
                    h1082
                }
            }) {
                f341(v0, v4)
            } else {
                false
            }
        }
        Adt21::C1(v5) => {
            f263(v5, match v0 {
                Adt16::C0(_, _, h1083, _) => {
                    h1083
                }
            })
        }
        Adt21::C2(_, v7, v8) => {
            if f259(v7, v1) {
                f263(v8, match v0 {
                    Adt16::C0(_, _, _, h1084) => {
                        h1084
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f258(v0: Adt12, v1: Adt14) -> R<Adt21> {
    let m1085 = f160(v1.clone(), f8(Str::lit("std")));
    match m1085 {
        None => {
            let m1090 = f161(f160(v1.clone(), f8(Str::lit("req"))));
            match m1090 {
                None => {
                    let m1092 = f160(v1, f8(Str::lit("der")));
                    match m1092 {
                        None => {
                            Ok(f342(v0))
                        }
                        Some(v5) => {
                            let m1093 = f343(f158(v5.clone(), f8(Str::lit("rule"))));
                            match m1093 {
                                None => {
                                    Ok(f342(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt21::C2(v6, f177(f74(f157(f160(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h1094) => {
                                            h1094
                                        }
                                    })?, f261(f158(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h1095, _) => {
                                            h1095
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt21::C1(f261(v4, match v0 {
                        Adt12::C0(_, _, h1091, _, _) => {
                            h1091
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m1086 = f161(f160(v2.clone(), f8(Str::lit("row"))));
            match m1086 {
                None => {
                    Ok(Adt21::C0(f261(f158(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h1087, _, _, _, _) => {
                            h1087
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt21::C0(f261(f158(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h1088, _, _, _, _) => {
                            h1088
                        }
                    })?, Some(f261(v3, match v0 {
                        Adt12::C0(_, h1089, _, _, _) => {
                            h1089
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f259(p1098: List<u64>, v1: List<u64>) -> bool {
    let m1097 = p1098;
    {
        let mut m1097 = m1097;
        loop {
            let step = {
                let v0 = m1097;
                let m1096 = v0;
                match m1096.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f263(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1097 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f260(p1103: List<u64>) -> bool {
    let m1102 = p1103;
    {
        let mut m1102 = m1102;
        loop {
            let step = {
                let v0 = m1102;
                let m1099 = v0;
                match m1099.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1100 = v1;
                        let a1101 = 7u64;
                        if nat_le(a1100, a1101) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1102 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m1104 = f344(v0, v1.clone(), 0u64)?;
    match m1104 {
        None => {
            let a1105 = v1;
            Ok(length_list(a1105))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f262(v0: Adt14) -> u64 {
    let m1106 = f345(f160(v0, f8(Str::lit("stratum"))));
    match m1106 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f263(v0: u64, p1111: List<u64>) -> bool {
    let m1110 = p1111;
    {
        let mut m1110 = m1110;
        loop {
            let step = {
                let v1 = m1110;
                let m1107 = v1;
                match m1107.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1108 = v2;
                        let a1109 = v0;
                        if nat_eq(a1108, a1109) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f264(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f205(v0.clone(), v1) {
        let m1112 = f30(v0, 34u8)?;
        match m1112.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1113 = v3;
                match m1113.uncons() {
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

pub fn f265(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f266(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m1114 = v0.clone();
    match m1114.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a1115 = v2;
            let a1116 = v1.clone();
            if equal(a1115, a1116) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f267(v0: List<(Bytes, ())>) -> List<Bytes> {
    f182(f346(Fn6::F347, List::<Bytes>::nil(), v0))
}

pub fn f268(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f348(f349(Fn7::F350, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m1121 = {
        let (h1117, _) = f351(Fn8::F352, {
            let a1119 = {
                let a1118 = v0;
                length_list(a1118)
            };
            let a1120 = 1u64;
            nat_add(a1119, a1120)?
        }, v1)?;
        h1117
    };
    match m1121.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f269(v0: List<Bytes>) -> List<(Bytes, ())> {
    f353(f354(Fn9::F355, List::<(Bytes, ())>::nil(), v0))
}

pub fn f270(p1124: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m1123 = p1124;
    {
        let mut m1123 = m1123;
        loop {
            let step = {
                let v0 = m1123;
                let m1122 = v0;
                match m1122.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f300(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
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

pub fn f271(v0: Adt14, v1: Bytes) -> Bytes {
    if f36(f339(f160(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f272(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1125, _) = v1.clone();
        h1125
    };
    let v3: u64 = {
        let (h1126, _) = {
            let (_, h1127) = v1.clone();
            h1127
        };
        h1126
    };
    let v4: List<Bytes> = {
        let (_, h1128) = {
            let (_, h1129) = v1;
            h1129
        };
        h1128
    };
    let m1130 = f252(v0.clone(), v2);
    match m1130 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1131 = v5;
            let a1132 = 10u8;
            if equal(a1131, a1132) {
                Ok(Some(({
                    let a1133 = v2;
                    let a1134 = 1u64;
                    nat_add(a1133, a1134)?
                }, ({
                    let a1135 = v2;
                    let a1136 = 1u64;
                    nat_add(a1135, a1136)?
                }, List::cons(f356(f32(v0, v3, {
                    let a1137 = v2;
                    let a1138 = v3;
                    nat_sub(a1137, a1138)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1139 = v2;
                    let a1140 = 1u64;
                    nat_add(a1139, a1140)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f273(v0: Adt1, v1: Bytes) -> R<Adt19> {
    let m1141 = f147(v0, v1)?;
    match m1141 {
        None => {
            Ok(Adt19::C6)
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f274(v0: Bytes) -> R<Bytes> {
    let a1142 = f8(Str::lit("Catalog."));
    let a1143 = f242(v0)?;
    Ok(append_bytes(a1142, a1143))
}

pub fn f275(v0: Adt19) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("encodings/")), List::cons(f242(f151(v0, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))
}

pub fn f276(v0: List<Bytes>) -> List<Bytes> {
    let m1144 = v0;
    match m1144.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f135(v1.clone(), f8(Str::lit(".jsonl"))) {
                List::cons(f131(f139(), v1), f276(v2))
            } else {
                f276(v2)
            }
        }
    }
}

pub fn f277(v0: Adt1, v1: List<Bytes>) -> R<(bool, Bytes)> {
    let m1145 = v1;
    match m1145.uncons() {
        None => {
            Ok((false, Bytes::lit(&[])))
        }
        Some((v2, v3)) => {
            let v4: (bool, Bytes) = f357(v0.clone(), v2)?;
            let (h1146, _) = v4.clone();
            if h1146 {
                Ok(v4)
            } else {
                let v5: (bool, Bytes) = f277(v0, v3)?;
                let (h1147, _) = v5.clone();
                if h1147 {
                    Ok(v5)
                } else {
                    Ok((false, {
                        let a1150 = {
                            let (_, h1148) = v4;
                            h1148
                        };
                        let a1151 = {
                            let (_, h1149) = v5;
                            h1149
                        };
                        append_bytes(a1150, a1151)
                    }))
                }
            }
        }
    }
}

pub fn f278(v0: Adt15, v1: List<Adt14>) -> List<Bytes> {
    let m1152 = v1;
    match m1152.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f203(v2.clone(), f8(Str::lit("subject"))) {
                if f204(v0.clone(), v2.clone()) {
                    f278(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting mapping: ")), List::cons(f158(v2.clone(), f8(Str::lit("subject"))), List::cons(f8(Str::lit(" ")), List::cons(f158(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f158(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" has an end that is not in the model yet\u{a}")), List::<Bytes>::nil())))))))), f278(v0, v3))
                }
            } else {
                if f287(v0.clone(), v2.clone()) {
                    f278(v0, v3)
                } else {
                    List::cons(f7(List::cons(f8(Str::lit("waiting matrix edge: row ")), List::cons(f158(v2.clone(), f8(Str::lit("row"))), List::cons(f8(Str::lit(" of ")), List::cons(f288(v2), List::cons(f8(Str::lit(" is not in the model yet\u{a}")), List::<Bytes>::nil())))))), f278(v0, v3))
                }
            }
        }
    }
}

pub fn f279(v0: Adt1) -> R<Bytes> {
    let a1154 = {
        let a1153 = f103(f104(f122(f123(f112(f70(v0.clone(), f117()))?)), f105(f73(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?)), f106(f73(v0, f8(Str::lit("model/admissions.json")))?));
        length_list(a1153)
    };
    let a1155 = 0u64;
    if nat_eq(a1154, a1155) {
        Ok(Bytes::lit(&[]))
    } else {
        Ok(f8(Str::lit("waiting inference: catalog rows that no authority encodes yet, which verdant_studio agenda lists; each needs an encoding staged with stage-encodings\u{a}")))
    }
}

pub fn f280(v0: List<Adt14>) -> R<List<Bytes>> {
    let m1156 = v0;
    match m1156.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f121(f74(f157(f160(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f158(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f158(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f358(f83(f158(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f280(v2)?))
        }
    }
}

pub fn f281(v0: Bytes, v1: List<Adt23>) -> List<Bytes> {
    let m1157 = v1;
    match m1157.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(match v2.clone() {
                Adt23::C0(h1158, _, _) => {
                    h1158
                }
            }, v0.clone()) {
                List::cons(f7(List::cons(match v2.clone() {
                    Adt23::C0(_, h1159, _) => {
                        h1159
                    }
                }, List::cons(f8(Str::lit(" ")), List::cons(match v2 {
                    Adt23::C0(_, _, h1160) => {
                        h1160
                    }
                }, List::cons(f8(Str::lit(" (derived from its citation)")), List::<Bytes>::nil()))))), f281(v0, v3))
            } else {
                f281(v0, v3)
            }
        }
    }
}

pub fn f282(v0: Bytes, v1: List<Adt14>) -> List<Bytes> {
    let m1161 = v1;
    match m1161.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36(f158(v2.clone(), f8(Str::lit("subject"))), v0.clone()) {
                List::cons(f7(List::cons(f158(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" ")), List::cons(f158(v2, f8(Str::lit("object"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil()))))), f282(v0, v3))
            } else {
                if f36(f158(v2.clone(), f8(Str::lit("object"))), v0.clone()) {
                    List::cons(f7(List::cons(f8(Str::lit("inverse of ")), List::cons(f158(v2.clone(), f8(Str::lit("relation"))), List::cons(f8(Str::lit(" from ")), List::cons(f158(v2, f8(Str::lit("subject"))), List::cons(f8(Str::lit(" (staged)")), List::<Bytes>::nil())))))), f282(v0, v3))
                } else {
                    f282(v0, v3)
                }
            }
        }
    }
}

pub fn f283(p1166: List<List<Bytes>>, v1: Bytes) -> Bytes {
    let m1165 = p1166;
    {
        let mut m1165 = m1165;
        loop {
            let step = {
                let v0 = m1165;
                let m1162 = v0;
                match m1162.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1163 = v2;
                        match m1163.uncons() {
                            None => {
                                Ok::<List<List<Bytes>>, Bytes>(v3)
                            }
                            Some((v4, v5)) => {
                                if f36(v4, v1.clone()) {
                                    let m1164 = v5;
                                    match m1164.uncons() {
                                        None => {
                                            Err::<List<List<Bytes>>, Bytes>(Bytes::lit(&[]))
                                        }
                                        Some((v6, v7)) => {
                                            Err::<List<List<Bytes>>, Bytes>(f7(List::cons(f359(v7), List::cons(f8(Str::lit(" (")), List::cons(v6, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))))
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
                Ok(next) => { m1165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284(v0: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1167 = v0;
    match m1167.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v1, v2)) => {
            let m1168 = f160(v1, f8(Str::lit("std")));
            match m1168 {
                None => {
                    f284(v2)
                }
                Some(v3) => {
                    if f203(v3.clone(), f8(Str::lit("row"))) {
                        List::cons((f158(v3.clone(), f8(Str::lit("row"))), f158(v3, f8(Str::lit("authority")))), f284(v2))
                    } else {
                        f284(v2)
                    }
                }
            }
        }
    }
}

pub fn f285(v0: Adt15, v1: List<Adt14>) -> List<(Bytes, Bytes)> {
    let m1169 = v1;
    match m1169.uncons() {
        None => {
            List::<(Bytes, Bytes)>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = f158(v2.clone(), f8(Str::lit("subject")));
            let v5: Bytes = f158(v2, f8(Str::lit("object")));
            if if f86(v4.clone(), match v0.clone() {
                Adt15::C0(h1170, _, _) => {
                    h1170
                }
            }) {
                f86(v5.clone(), match v0.clone() {
                    Adt15::C0(_, h1171, _) => {
                        h1171
                    }
                })
            } else {
                false
            } {
                List::cons((v4, v5), f285(v0, v3))
            } else {
                if if f86(v5.clone(), match v0.clone() {
                    Adt15::C0(h1172, _, _) => {
                        h1172
                    }
                }) {
                    f86(v4.clone(), match v0.clone() {
                        Adt15::C0(_, h1173, _) => {
                            h1173
                        }
                    })
                } else {
                    false
                } {
                    List::cons((v5, v4), f285(v0, v3))
                } else {
                    f285(v0, v3)
                }
            }
        }
    }
}

pub fn f286(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>, v2: List<(Bytes, Bytes)>, v3: List<(Bytes, Bytes)>) -> R<List<Adt23>> {
    let m1174 = v0;
    match m1174.uncons() {
        None => {
            Ok(List::<Adt23>::nil())
        }
        Some((v4, v5)) => {
            if if f360(v4.clone(), v2.clone()) {
                true
            } else {
                f360(v4.clone(), v3.clone())
            } {
                f286(v5, v1, v2, v3)
            } else {
                Ok(List::cons(Adt23::C0({
                    let (h1175, _) = v4.clone();
                    h1175
                }, {
                    let a1178 = {
                        let a1177 = f78(f361({
                            let (h1176, _) = v4.clone();
                            h1176
                        }, v1.clone()))?;
                        length_list(a1177)
                    };
                    let a1179 = 1u64;
                    if nat_eq(a1178, a1179) {
                        f8(Str::lit("exactMatch"))
                    } else {
                        f8(Str::lit("narrowMatch"))
                    }
                }, {
                    let (_, h1180) = v4.clone();
                    h1180
                }), f286(v5, v1, v2, List::cons(v4, v3))?))
            }
        }
    }
}

pub fn f287(v0: Adt15, v1: Adt14) -> bool {
    if f86(f158(v1.clone(), f8(Str::lit("row"))), match v0.clone() {
        Adt15::C0(h1181, _, _) => {
            h1181
        }
    }) {
        f86(f288(v1), match v0 {
            Adt15::C0(_, h1182, _) => {
                h1182
            }
        })
    } else {
        false
    }
}

pub fn f288(v0: Adt14) -> Bytes {
    let m1183 = f160(v0, f8(Str::lit("evidence")));
    match m1183 {
        None => {
            Bytes::lit(&[])
        }
        Some(v1) => {
            f158(v1, f8(Str::lit("authority")))
        }
    }
}

pub fn f289(v0: Adt14) -> Bytes {
    if f203(v0.clone(), f8(Str::lit("req"))) {
        let a1184 = f8(Str::lit("req "));
        let a1185 = f158(v0, f8(Str::lit("req")));
        append_bytes(a1184, a1185)
    } else {
        let m1186 = f160(v0.clone(), f8(Str::lit("std")));
        match m1186 {
            None => {
                let m1189 = f160(v0, f8(Str::lit("der")));
                match m1189 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f158(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f121(f74(f157(f160(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f158(v1.clone(), f8(Str::lit("authority"))), List::cons(if f203(v1.clone(), f8(Str::lit("row"))) {
                    let a1187 = f8(Str::lit(" row "));
                    let a1188 = f158(v1, f8(Str::lit("row")));
                    append_bytes(a1187, a1188)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f290(v0: List<Bytes>, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1190 = v1;
    match m1190.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86({
                let (h1191, _) = v2.clone();
                h1191
            }, v0.clone()) {
                List::cons({
                    let (_, h1192) = v2;
                    h1192
                }, f290(v0, v3))
            } else {
                if f86({
                    let (_, h1193) = v2;
                    h1193
                }, v0.clone()) {
                    List::cons({
                        let (h1194, _) = v2;
                        h1194
                    }, f290(v0, v3))
                } else {
                    f290(v0, v3)
                }
            }
        }
    }
}

pub fn f291(v0: Adt15, v1: Adt14, v2: Bytes) -> bool {
    if f86(v2.clone(), match v0.clone() {
        Adt15::C0(h1195, _, _) => {
            h1195
        }
    }) {
        true
    } else {
        if f86(v2.clone(), match v0.clone() {
            Adt15::C0(_, h1196, _) => {
                h1196
            }
        }) {
            true
        } else {
            if f86(v2, match v0.clone() {
                Adt15::C0(_, _, h1197) => {
                    h1197
                }
            }) {
                true
            } else {
                if f203(v1.clone(), f8(Str::lit("scheme"))) {
                    f86(f158(v1, f8(Str::lit("scheme"))), match v0 {
                        Adt15::C0(_, h1198, _) => {
                            h1198
                        }
                    })
                } else {
                    false
                }
            }
        }
    }
}

pub fn f292(v0: Bytes, v1: u64) -> R<u64> {
    let m1201 = {
        let a1199 = v0;
        let a1200 = v1;
        index_bytes(a1199, a1200)
    };
    match m1201 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1246 = {
                let a1222 = {
                    let a1210 = {
                        let a1204 = {
                            let a1202 = v3;
                            let a1203 = 128u8;
                            fixed_u8::bit_and(a1202, a1203)
                        };
                        let a1205 = 128u8;
                        if equal(a1204, a1205) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1211 = {
                        let a1208 = {
                            let a1206 = v3;
                            let a1207 = 64u8;
                            fixed_u8::bit_and(a1206, a1207)
                        };
                        let a1209 = 64u8;
                        if equal(a1208, a1209) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1210, a1211)?
                };
                let a1223 = {
                    let a1220 = {
                        let a1214 = {
                            let a1212 = v3;
                            let a1213 = 32u8;
                            fixed_u8::bit_and(a1212, a1213)
                        };
                        let a1215 = 32u8;
                        if equal(a1214, a1215) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1221 = {
                        let a1218 = {
                            let a1216 = v3;
                            let a1217 = 16u8;
                            fixed_u8::bit_and(a1216, a1217)
                        };
                        let a1219 = 16u8;
                        if equal(a1218, a1219) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1220, a1221)?
                };
                nat_add(a1222, a1223)?
            };
            let a1247 = {
                let a1244 = {
                    let a1232 = {
                        let a1226 = {
                            let a1224 = v3;
                            let a1225 = 8u8;
                            fixed_u8::bit_and(a1224, a1225)
                        };
                        let a1227 = 8u8;
                        if equal(a1226, a1227) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1233 = {
                        let a1230 = {
                            let a1228 = v3;
                            let a1229 = 4u8;
                            fixed_u8::bit_and(a1228, a1229)
                        };
                        let a1231 = 4u8;
                        if equal(a1230, a1231) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1232, a1233)?
                };
                let a1245 = {
                    let a1242 = {
                        let a1236 = {
                            let a1234 = v3;
                            let a1235 = 2u8;
                            fixed_u8::bit_and(a1234, a1235)
                        };
                        let a1237 = 2u8;
                        if equal(a1236, a1237) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1243 = {
                        let a1240 = {
                            let a1238 = v3;
                            let a1239 = 1u8;
                            fixed_u8::bit_and(a1238, a1239)
                        };
                        let a1241 = 1u8;
                        if equal(a1240, a1241) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1242, a1243)?
                };
                nat_add(a1244, a1245)?
            };
            nat_add(a1246, a1247)
        }
    }
}

pub fn f293(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m1251 = {
        let a1248 = v1;
        let a1249 = v2;
        let a1250 = 1u64;
        slice_bytes(a1248, a1249, a1250)
    };
    match m1251 {
        None => {
            v0
        }
        Some(v3) => {
            let a1252 = v0;
            let a1253 = v3;
            append_bytes(a1252, a1253)
        }
    }
}

pub fn f294(p1256: List<Bytes>, p1257: List<Bytes>) -> List<Bytes> {
    let m1255 = (p1256, p1257);
    {
        let mut m1255 = m1255;
        loop {
            let step = {
                let (v0, v1) = m1255;
                let m1254 = v0;
                match m1254.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: Bytes, p1329: u64, p1330: u64, p1331: Bytes) -> R<Bytes> {
    let m1328 = (p1329, (p1330, p1331));
    {
        let mut m1328 = m1328;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1328;
                let m1258 = v2;
                if m1258 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1258 - 1;
                    let m1261 = {
                        let a1259 = v0.clone();
                        let a1260 = v1;
                        index_bytes(a1259, a1260)
                    };
                    match m1261 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a1306 = {
                                    let a1282 = {
                                        let a1270 = {
                                            let a1264 = {
                                                let a1262 = v6;
                                                let a1263 = 128u8;
                                                fixed_u8::bit_and(a1262, a1263)
                                            };
                                            let a1265 = 128u8;
                                            if equal(a1264, a1265) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1271 = {
                                            let a1268 = {
                                                let a1266 = v6;
                                                let a1267 = 64u8;
                                                fixed_u8::bit_and(a1266, a1267)
                                            };
                                            let a1269 = 64u8;
                                            if equal(a1268, a1269) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1270, a1271)?
                                    };
                                    let a1283 = {
                                        let a1280 = {
                                            let a1274 = {
                                                let a1272 = v6;
                                                let a1273 = 32u8;
                                                fixed_u8::bit_and(a1272, a1273)
                                            };
                                            let a1275 = 32u8;
                                            if equal(a1274, a1275) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1281 = {
                                            let a1278 = {
                                                let a1276 = v6;
                                                let a1277 = 16u8;
                                                fixed_u8::bit_and(a1276, a1277)
                                            };
                                            let a1279 = 16u8;
                                            if equal(a1278, a1279) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1280, a1281)?
                                    };
                                    nat_add(a1282, a1283)?
                                };
                                let a1307 = {
                                    let a1304 = {
                                        let a1292 = {
                                            let a1286 = {
                                                let a1284 = v6;
                                                let a1285 = 8u8;
                                                fixed_u8::bit_and(a1284, a1285)
                                            };
                                            let a1287 = 8u8;
                                            if equal(a1286, a1287) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1293 = {
                                            let a1290 = {
                                                let a1288 = v6;
                                                let a1289 = 4u8;
                                                fixed_u8::bit_and(a1288, a1289)
                                            };
                                            let a1291 = 4u8;
                                            if equal(a1290, a1291) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1292, a1293)?
                                    };
                                    let a1305 = {
                                        let a1302 = {
                                            let a1296 = {
                                                let a1294 = v6;
                                                let a1295 = 2u8;
                                                fixed_u8::bit_and(a1294, a1295)
                                            };
                                            let a1297 = 2u8;
                                            if equal(a1296, a1297) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1303 = {
                                            let a1300 = {
                                                let a1298 = v6;
                                                let a1299 = 1u8;
                                                fixed_u8::bit_and(a1298, a1299)
                                            };
                                            let a1301 = 1u8;
                                            if equal(a1300, a1301) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1302, a1303)?
                                    };
                                    nat_add(a1304, a1305)?
                                };
                                nat_add(a1306, a1307)?
                            };
                            let a1308 = v7;
                            let a1309 = 34u64;
                            if if nat_eq(a1308, a1309) {
                                true
                            } else {
                                let a1310 = v7;
                                let a1311 = 92u64;
                                nat_eq(a1310, a1311)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a1312 = v1;
                                    let a1313 = 1u64;
                                    nat_add(a1312, a1313)?
                                }, (v4, {
                                    let a1316 = v3;
                                    let a1317 = {
                                        let a1314 = Bytes::lit(&[0x5c]);
                                        let a1315 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1314, a1315)
                                    };
                                    append_bytes(a1316, a1317)
                                })))
                            } else {
                                let a1318 = v7;
                                let a1319 = 10u64;
                                if nat_eq(a1318, a1319) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1320 = v1;
                                        let a1321 = 1u64;
                                        nat_add(a1320, a1321)?
                                    }, (v4, {
                                        let a1322 = v3;
                                        let a1323 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a1322, a1323)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a1324 = v1;
                                        let a1325 = 1u64;
                                        nat_add(a1324, a1325)?
                                    }, (v4, {
                                        let a1326 = v3;
                                        let a1327 = f32(v0.clone(), v1, 1u64);
                                        append_bytes(a1326, a1327)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: u32) -> Bytes {
    let a1360 = f362({
        let a1332 = f363(v0, 28u32);
        let a1333 = 15u32;
        fixed_u32::bit_and(a1332, a1333)
    });
    let a1361 = {
        let a1358 = f362({
            let a1334 = f363(v0, 24u32);
            let a1335 = 15u32;
            fixed_u32::bit_and(a1334, a1335)
        });
        let a1359 = {
            let a1356 = f362({
                let a1336 = f363(v0, 20u32);
                let a1337 = 15u32;
                fixed_u32::bit_and(a1336, a1337)
            });
            let a1357 = {
                let a1354 = f362({
                    let a1338 = f363(v0, 16u32);
                    let a1339 = 15u32;
                    fixed_u32::bit_and(a1338, a1339)
                });
                let a1355 = {
                    let a1352 = f362({
                        let a1340 = f363(v0, 12u32);
                        let a1341 = 15u32;
                        fixed_u32::bit_and(a1340, a1341)
                    });
                    let a1353 = {
                        let a1350 = f362({
                            let a1342 = f363(v0, 8u32);
                            let a1343 = 15u32;
                            fixed_u32::bit_and(a1342, a1343)
                        });
                        let a1351 = {
                            let a1348 = f362({
                                let a1344 = f363(v0, 4u32);
                                let a1345 = 15u32;
                                fixed_u32::bit_and(a1344, a1345)
                            });
                            let a1349 = f362({
                                let a1346 = v0;
                                let a1347 = 15u32;
                                fixed_u32::bit_and(a1346, a1347)
                            });
                            append_bytes(a1348, a1349)
                        };
                        append_bytes(a1350, a1351)
                    };
                    append_bytes(a1352, a1353)
                };
                append_bytes(a1354, a1355)
            };
            append_bytes(a1356, a1357)
        };
        append_bytes(a1358, a1359)
    };
    append_bytes(a1360, a1361)
}

pub fn f297(v0: Fn4, p1365: u64, p1366: (u64, Adt24)) -> R<(u64, Adt24)> {
    let m1364 = (p1365, p1366);
    {
        let mut m1364 = m1364;
        loop {
            let step = {
                let (v1, v2) = m1364;
                let m1362 = v1;
                if m1362 == 0 {
                    Err::<(u64, (u64, Adt24)), R<(u64, Adt24)>>(Ok(v2))
                } else {
                    let v3 = m1362 - 1;
                    Ok::<(u64, (u64, Adt24)), R<(u64, Adt24)>>((v3, {
                        let c1363: Fn4 = v0.clone();
                        c1363.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1364 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f298(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt24)) -> R<(u64, Adt24)> {
    Ok(({
        let a1368 = {
            let (h1367, _) = v3.clone();
            h1367
        };
        let a1369 = 64u64;
        nat_add(a1368, a1369)?
    }, f364({
        let (_, h1370) = v3;
        h1370
    }, f365(v0, v1, v2, {
        let (h1371, _) = v3;
        h1371
    })?)?))
}

pub fn f299() -> Adt24 {
    Adt24::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f300(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    let m1372 = f30(v1, 34u8)?;
    match m1372.uncons() {
        None => {
            Ok(false)
        }
        Some((v2, v3)) => {
            let m1373 = v3;
            match m1373.uncons() {
                None => {
                    Ok(f366(v0, f7(List::cons(f8(Str::lit("id = \"")), List::cons(v2, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil()))))))
                }
                Some((_, _)) => {
                    Ok(false)
                }
            }
        }
    }
}

pub fn f301(v0: List<Bytes>) -> List<Bytes> {
    let m1374 = v0;
    match m1374.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f36(v1.clone(), f8(Str::lit("authority = []"))) {
                f301(v2)
            } else {
                List::cons(v1, f301(v2))
            }
        }
    }
}

pub fn f302(v0: Bytes) -> R<u64> {
    let (h1375, _) = f254(Fn3::F367(v0.clone()), {
        let a1376 = v0.clone();
        length_bytes(a1376)
    }, {
        let a1377 = v0;
        length_bytes(a1377)
    })?;
    Ok(h1375)
}

pub fn f303(v0: Adt14) -> R<Bytes> {
    let m1378 = v0;
    match m1378 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f332(v2, v3)
        }
        Adt14::C2(v4) => {
            f334(v4)
        }
        Adt14::C3(r1379) => {
            let v5 = (*r1379).clone();
            Ok(f335(f368(v5)?))
        }
        Adt14::C4(r1380) => {
            let v6 = (*r1380).clone();
            f337(f369(v6)?)
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

pub fn f304(v0: Fn5, p1385: u64, p1386: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1384 = (p1385, p1386);
    {
        let mut m1384 = m1384;
        loop {
            let step = {
                let (v1, v2) = m1384;
                let m1381 = v1;
                if m1381 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1381 - 1;
                    let m1383 = {
                        let c1382: Fn5 = v0.clone();
                        c1382.apply(v2.clone())?
                    };
                    match m1383 {
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
                Ok(next) => { m1384 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f305(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f370(v0, v1)
}

pub fn f306(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f371(v0, v1)
}

pub fn f307(v0: u64) -> bool {
    let a1387 = v0;
    let a1388 = 32u64;
    if if nat_eq(a1387, a1388) {
        true
    } else {
        let a1389 = v0;
        let a1390 = 9u64;
        nat_eq(a1389, a1390)
    } {
        true
    } else {
        let a1391 = v0;
        let a1392 = 10u64;
        if nat_eq(a1391, a1392) {
            true
        } else {
            let a1393 = v0;
            let a1394 = 13u64;
            nat_eq(a1393, a1394)
        }
    }
}

pub fn f308(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1395, _) = f254(Fn3::F372(v1.clone(), v0.clone()), {
            let a1397 = {
                let a1396 = v0.clone();
                length_bytes(a1396)
            };
            let a1398 = 1u64;
            nat_add(a1397, a1398)?
        }, v2)?;
        h1395
    };
    if f220(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f309(v0: Bytes, v1: u64) -> R<u64> {
    let (h1399, _) = f254(Fn3::F373(v0.clone()), {
        let a1401 = {
            let a1400 = v0;
            length_bytes(a1400)
        };
        let a1402 = 1u64;
        nat_add(a1401, a1402)?
    }, v1)?;
    Ok(h1399)
}

pub fn f310(v0: Bytes) -> R<bool> {
    let a1404 = {
        let a1403 = v0.clone();
        length_bytes(a1403)
    };
    let a1405 = 5u64;
    if nat_eq(a1404, a1405) {
        let a1406 = f313(v0.clone(), 0u64)?;
        let a1407 = 85u64;
        if if nat_eq(a1406, a1407) {
            true
        } else {
            let a1408 = f313(v0.clone(), 0u64)?;
            let a1409 = 117u64;
            nat_eq(a1408, a1409)
        } {
            let a1410 = f313(v0.clone(), 1u64)?;
            let a1411 = 84u64;
            if if nat_eq(a1410, a1411) {
                true
            } else {
                let a1412 = f313(v0.clone(), 1u64)?;
                let a1413 = 116u64;
                nat_eq(a1412, a1413)
            } {
                let a1414 = f313(v0.clone(), 2u64)?;
                let a1415 = 70u64;
                if if nat_eq(a1414, a1415) {
                    true
                } else {
                    let a1416 = f313(v0.clone(), 2u64)?;
                    let a1417 = 102u64;
                    nat_eq(a1416, a1417)
                } {
                    let a1418 = f313(v0.clone(), 3u64)?;
                    let a1419 = 45u64;
                    if nat_eq(a1418, a1419) {
                        let a1420 = f313(v0, 4u64)?;
                        let a1421 = 56u64;
                        Ok(nat_eq(a1420, a1421))
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f311(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1425 = f308(v0.clone(), {
        let a1422 = Str::lit("--");
        utf8_encode(a1422)
    }, {
        let a1423 = v1;
        let a1424 = 4u64;
        nat_add(a1423, a1424)?
    })?;
    match m1425 {
        None => {
            Ok(Err::<u64, Bytes>(f223(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1428 = f227(v0, {
                let a1426 = v2;
                let a1427 = 2u64;
                nat_add(a1426, a1427)?
            })?;
            let a1429 = 62u64;
            if nat_eq(a1428, a1429) {
                Ok(Ok::<u64, Bytes>({
                    let a1430 = v2;
                    let a1431 = 3u64;
                    nat_add(a1430, a1431)?
                }))
            } else {
                Ok(Err::<u64, Bytes>(f223(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f312(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f314(v0.clone(), {
        let a1432 = v1;
        let a1433 = 2u64;
        nat_add(a1432, a1433)?
    })?;
    let a1436 = v2;
    let a1437 = {
        let a1434 = v1;
        let a1435 = 2u64;
        nat_add(a1434, a1435)?
    };
    if nat_eq(a1436, a1437) {
        Ok(Err::<u64, Bytes>(f223(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1440 = v2;
        let a1441 = {
            let a1438 = v1;
            let a1439 = 5u64;
            nat_add(a1438, a1439)?
        };
        if if nat_eq(a1440, a1441) {
            let a1444 = f227(v0.clone(), {
                let a1442 = v1;
                let a1443 = 2u64;
                nat_add(a1442, a1443)?
            })?;
            let a1445 = 120u64;
            if if nat_eq(a1444, a1445) {
                true
            } else {
                let a1448 = f227(v0.clone(), {
                    let a1446 = v1;
                    let a1447 = 2u64;
                    nat_add(a1446, a1447)?
                })?;
                let a1449 = 88u64;
                nat_eq(a1448, a1449)
            } {
                let a1452 = f227(v0.clone(), {
                    let a1450 = v1;
                    let a1451 = 3u64;
                    nat_add(a1450, a1451)?
                })?;
                let a1453 = 109u64;
                if if nat_eq(a1452, a1453) {
                    true
                } else {
                    let a1456 = f227(v0.clone(), {
                        let a1454 = v1;
                        let a1455 = 3u64;
                        nat_add(a1454, a1455)?
                    })?;
                    let a1457 = 77u64;
                    nat_eq(a1456, a1457)
                } {
                    let a1460 = f227(v0.clone(), {
                        let a1458 = v1;
                        let a1459 = 4u64;
                        nat_add(a1458, a1459)?
                    })?;
                    let a1461 = 108u64;
                    if nat_eq(a1460, a1461) {
                        true
                    } else {
                        let a1464 = f227(v0.clone(), {
                            let a1462 = v1;
                            let a1463 = 4u64;
                            nat_add(a1462, a1463)?
                        })?;
                        let a1465 = 76u64;
                        nat_eq(a1464, a1465)
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f223(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1467 = f308(v0, {
                let a1466 = Str::lit("?>");
                utf8_encode(a1466)
            }, v2)?;
            match m1467 {
                None => {
                    Ok(Err::<u64, Bytes>(f223(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1468 = v3;
                        let a1469 = 2u64;
                        nat_add(a1468, a1469)?
                    }))
                }
            }
        }
    }
}

pub fn f313(v0: Bytes, v1: u64) -> R<u64> {
    let m1472 = {
        let a1470 = v0;
        let a1471 = v1;
        index_bytes(a1470, a1471)
    };
    match m1472 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1517 = {
                let a1493 = {
                    let a1481 = {
                        let a1475 = {
                            let a1473 = v3;
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
                            let a1477 = v3;
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
                            let a1483 = v3;
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
                            let a1487 = v3;
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
                            let a1495 = v3;
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
                            let a1499 = v3;
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
                            let a1505 = v3;
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
                            let a1509 = v3;
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
            nat_add(a1517, a1518)
        }
    }
}

pub fn f314(v0: Bytes, v1: u64) -> R<u64> {
    let (h1519, _) = f254(Fn3::F374(v0.clone(), v1), {
        let a1521 = {
            let a1520 = v0;
            length_bytes(a1520)
        };
        let a1522 = 1u64;
        nat_add(a1521, a1522)?
    }, v1)?;
    Ok(h1519)
}

pub fn f315(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt18, u64), Bytes>> {
    let m1523 = v2;
    if m1523 == 0 {
        Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1523 - 1;
        let v5: u64 = f309(v0.clone(), v1)?;
        let a1524 = f227(v0.clone(), v5)?;
        let a1525 = 62u64;
        if if nat_eq(a1524, a1525) {
            true
        } else {
            f226(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C4, v5)))
        } else {
            let a1526 = v5;
            let a1527 = v1;
            if nat_eq(a1526, a1527) {
                Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f314(v0.clone(), v5)?;
                let a1528 = v6;
                let a1529 = v5;
                if nat_eq(a1528, a1529) {
                    Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a1530 = v6;
                        let a1531 = v5;
                        nat_sub(a1530, a1531)
                    });
                    if f375(v7.clone(), v3.clone()) {
                        Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f309(v0.clone(), v6)?;
                        let a1532 = f227(v0.clone(), v8)?;
                        let a1533 = 61u64;
                        if !nat_eq(a1532, a1533) {
                            Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        } else {
                            let v9: u64 = f309(v0.clone(), {
                                let a1534 = v8;
                                let a1535 = 1u64;
                                nat_add(a1534, a1535)?
                            })?;
                            let v10: u64 = f227(v0.clone(), v9)?;
                            let a1536 = v10;
                            let a1537 = 34u64;
                            if !if nat_eq(a1536, a1537) {
                                true
                            } else {
                                let a1538 = v10;
                                let a1539 = 39u64;
                                nat_eq(a1538, a1539)
                            } {
                                Ok(Err::<(Adt18, u64), Bytes>(f223(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            } else {
                                let m1542 = f316(v0.clone(), {
                                    let a1540 = v9;
                                    let a1541 = 1u64;
                                    nat_add(a1540, a1541)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1542 {
                                    Ok(v12) => {
                                        let m1544 = f315(v0, {
                                            let (_, h1543) = v12;
                                            h1543
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1544 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt18, u64), Bytes>((Adt18::C1(v7, {
                                                    let (h1545, _) = v12;
                                                    h1545
                                                }, std::rc::Rc::new({
                                                    let (h1546, _) = v14;
                                                    h1546
                                                })), {
                                                    let (_, h1547) = v14;
                                                    h1547
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt18, u64), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt18, u64), Bytes>(v11))
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

pub fn f316(v0: Bytes, p1587: u64, p1588: u64, p1589: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1586 = (p1587, (p1588, p1589));
    {
        let mut m1586 = m1586;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1586;
                let m1548 = v2;
                if m1548 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f223(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1548 - 1;
                    let v7: u64 = f227(v0.clone(), v1)?;
                    if if v4 {
                        let a1549 = v7;
                        let a1550 = v5;
                        nat_eq(a1549, a1550)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1551 = v1;
                            let a1552 = 1u64;
                            nat_add(a1551, a1552)?
                        }))))
                    } else {
                        let a1553 = v7;
                        let a1554 = 256u64;
                        if nat_eq(a1553, a1554) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f223(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1555 = v7;
                            let a1556 = 60u64;
                            if nat_eq(a1555, a1556) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f223(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1557 = v7;
                                let a1558 = 38u64;
                                if nat_eq(a1557, a1558) {
                                    let m1559 = f376(v0.clone(), v1)?;
                                    match m1559 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1560) = v9;
                                                h1560
                                            }, (v6, {
                                                let a1562 = v3;
                                                let a1563 = {
                                                    let (h1561, _) = v9;
                                                    h1561
                                                };
                                                append_bytes(a1562, a1563)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1564 = v7;
                                        let a1565 = 9u64;
                                        if if nat_eq(a1564, a1565) {
                                            true
                                        } else {
                                            let a1566 = v7;
                                            let a1567 = 10u64;
                                            nat_eq(a1566, a1567)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1568 = v1;
                                                let a1569 = 1u64;
                                                nat_add(a1568, a1569)?
                                            }, (v6, {
                                                let a1570 = v3;
                                                let a1571 = Bytes::lit(&[0x20]);
                                                append_bytes(a1570, a1571)
                                            })))
                                        } else {
                                            let v10: u64 = f377(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1574 = v3;
                                                let a1575 = f32(v0.clone(), v1, {
                                                    let a1572 = v10;
                                                    let a1573 = v1;
                                                    nat_sub(a1572, a1573)
                                                });
                                                append_bytes(a1574, a1575)
                                            })))
                                        }
                                    } else {
                                        let a1576 = v7;
                                        let a1577 = 93u64;
                                        if nat_eq(a1576, a1577) {
                                            if f226(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f223(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1578 = v1;
                                                    let a1579 = 1u64;
                                                    nat_add(a1578, a1579)?
                                                }, (v6, {
                                                    let a1580 = v3;
                                                    let a1581 = Bytes::lit(&[0x5d]);
                                                    append_bytes(a1580, a1581)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f377(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1584 = v3;
                                                let a1585 = f32(v0.clone(), v1, {
                                                    let a1582 = v11;
                                                    let a1583 = v1;
                                                    nat_sub(a1582, a1583)
                                                });
                                                append_bytes(a1584, a1585)
                                            })))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1586 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: Bytes, v1: u64) -> Bytes {
    let m1593 = {
        let a1590 = v0;
        let a1591 = v1;
        let a1592 = 1u64;
        slice_bytes(a1590, a1591, a1592)
    };
    match m1593 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f318(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f378(v1, v0, v2)
}

pub fn f319(v0: Bytes, v1: u64) -> R<u64> {
    let (h1594, _) = f254(Fn3::F379(v1, v0), v1, 0u64)?;
    Ok(h1594)
}

pub fn f320(v0: Bytes) -> R<Bytes> {
    f380(v0.clone(), 0u64, {
        let a1595 = v0;
        length_bytes(a1595)
    }, false, false, Bytes::lit(&[]))
}

pub fn f321(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m1596 = v2;
    if m1596 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m1596 - 1;
        let v5: u64 = f313(v0.clone(), v1)?;
        let a1597 = v3;
        let a1598 = 1u64;
        if nat_eq(a1597, a1598) {
            let m1599 = f321(v0.clone(), v1, v4, 0u64)?;
            match m1599 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f313(v0.clone(), {
                        let (_, h1600) = v6.clone();
                        h1600
                    })?;
                    let a1601 = v7;
                    let a1602 = 44u64;
                    if nat_eq(a1601, a1602) {
                        let m1606 = f321(v0, {
                            let a1604 = {
                                let (_, h1603) = v6;
                                h1603
                            };
                            let a1605 = 1u64;
                            nat_add(a1604, a1605)?
                        }, v4, 1u64)?;
                        match m1606 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h1607, _) = v6;
                                    h1607
                                }), std::rc::Rc::new({
                                    let (h1608, _) = v8;
                                    h1608
                                })), {
                                    let (_, h1609) = v8;
                                    h1609
                                })))
                            }
                        }
                    } else {
                        let a1610 = v7;
                        let a1611 = 93u64;
                        if nat_eq(a1610, a1611) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h1612, _) = v6;
                                h1612
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a1614 = {
                                    let (_, h1613) = v6;
                                    h1613
                                };
                                let a1615 = 1u64;
                                nat_add(a1614, a1615)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1616 = v3;
            let a1617 = 2u64;
            if nat_eq(a1616, a1617) {
                let m1620 = f381(v0.clone(), {
                    let a1618 = v1;
                    let a1619 = 1u64;
                    nat_add(a1618, a1619)?
                })?;
                match m1620 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a1621 = v5;
                        let a1622 = 34u64;
                        if if nat_eq(a1621, a1622) {
                            let a1624 = f313(v0.clone(), {
                                let (h1623, _) = v9.clone();
                                h1623
                            })?;
                            let a1625 = 58u64;
                            nat_eq(a1624, a1625)
                        } else {
                            false
                        } {
                            let m1629 = f321(v0.clone(), {
                                let a1627 = {
                                    let (h1626, _) = v9;
                                    h1626
                                };
                                let a1628 = 1u64;
                                nat_add(a1627, a1628)?
                            }, v4, 0u64)?;
                            match m1629 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f313(v0.clone(), {
                                        let (_, h1630) = v10.clone();
                                        h1630
                                    })?;
                                    let a1631 = v11;
                                    let a1632 = 44u64;
                                    if nat_eq(a1631, a1632) {
                                        let m1636 = f321(v0, {
                                            let a1634 = {
                                                let (_, h1633) = v10;
                                                h1633
                                            };
                                            let a1635 = 1u64;
                                            nat_add(a1634, a1635)?
                                        }, v4, 2u64)?;
                                        match m1636 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h1637) = v9;
                                                    h1637
                                                }, std::rc::Rc::new({
                                                    let (h1638, _) = v10;
                                                    h1638
                                                }), std::rc::Rc::new({
                                                    let (h1639, _) = v12;
                                                    h1639
                                                })), {
                                                    let (_, h1640) = v12;
                                                    h1640
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1641 = v11;
                                        let a1642 = 125u64;
                                        if nat_eq(a1641, a1642) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h1643) = v9;
                                                h1643
                                            }, std::rc::Rc::new({
                                                let (h1644, _) = v10;
                                                h1644
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a1646 = {
                                                    let (_, h1645) = v10;
                                                    h1645
                                                };
                                                let a1647 = 1u64;
                                                nat_add(a1646, a1647)?
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
                let a1648 = v5;
                let a1649 = 34u64;
                if nat_eq(a1648, a1649) {
                    let m1652 = f381(v0, {
                        let a1650 = v1;
                        let a1651 = 1u64;
                        nat_add(a1650, a1651)?
                    })?;
                    match m1652 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h1653) = v13;
                                h1653
                            }), {
                                let (h1654, _) = v13;
                                h1654
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a1655 = v1;
                            let a1656 = 4u64;
                            nat_add(a1655, a1656)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a1657 = v1;
                                let a1658 = 5u64;
                                nat_add(a1657, a1658)?
                            })))
                        } else {
                            let a1659 = v5;
                            let a1660 = 91u64;
                            if nat_eq(a1659, a1660) {
                                let a1663 = f313(v0.clone(), {
                                    let a1661 = v1;
                                    let a1662 = 1u64;
                                    nat_add(a1661, a1662)?
                                })?;
                                let a1664 = 93u64;
                                if nat_eq(a1663, a1664) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a1665 = v1;
                                        let a1666 = 2u64;
                                        nat_add(a1665, a1666)?
                                    })))
                                } else {
                                    let m1669 = f321(v0, {
                                        let a1667 = v1;
                                        let a1668 = 1u64;
                                        nat_add(a1667, a1668)?
                                    }, v4, 1u64)?;
                                    match m1669 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h1670, _) = v14;
                                                h1670
                                            })), {
                                                let (_, h1671) = v14;
                                                h1671
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1672 = v5;
                                let a1673 = 123u64;
                                if nat_eq(a1672, a1673) {
                                    let a1676 = f313(v0.clone(), {
                                        let a1674 = v1;
                                        let a1675 = 1u64;
                                        nat_add(a1674, a1675)?
                                    })?;
                                    let a1677 = 125u64;
                                    if nat_eq(a1676, a1677) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a1678 = v1;
                                            let a1679 = 2u64;
                                            nat_add(a1678, a1679)?
                                        })))
                                    } else {
                                        let m1682 = f321(v0, {
                                            let a1680 = v1;
                                            let a1681 = 1u64;
                                            nat_add(a1680, a1681)?
                                        }, v4, 2u64)?;
                                        match m1682 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h1683, _) = v15;
                                                    h1683
                                                })), {
                                                    let (_, h1684) = v15;
                                                    h1684
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1685 = f382(v0, v1)?;
                                    match m1685 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1686) = v16;
                                                h1686
                                            }, {
                                                let (h1687, _) = v16;
                                                h1687
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

pub fn f322() -> List<Bytes> {
    List::cons(f8(Str::lit("producer")), List::cons(f8(Str::lit("consumer")), List::cons(f8(Str::lit("attributed")), List::cons(f8(Str::lit("onBehalfOf")), List::cons(f8(Str::lit("observation")), List::cons(f8(Str::lit("esco:hasEssentialSkill")), List::cons(f8(Str::lit("esco:hasOptionalSkill")), List::<Bytes>::nil())))))))
}

pub fn f323(v0: Adt14, v1: Bytes, v2: Bytes) -> bool {
    let m1688 = f160(v0, v1);
    match m1688 {
        None => {
            false
        }
        Some(v3) => {
            f203(v3, v2)
        }
    }
}

pub fn f324() -> List<Bytes> {
    List::cons(f8(Str::lit("exactMatch")), List::cons(f8(Str::lit("closeMatch")), List::cons(f8(Str::lit("broadMatch")), List::cons(f8(Str::lit("narrowMatch")), List::cons(f8(Str::lit("relatedMatch")), List::<Bytes>::nil())))))
}

pub fn f325(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt19, u64)>> {
    let m1689 = v2;
    if m1689 == 0 {
        Ok(None::<(Adt19, u64)>)
    } else {
        let v4 = m1689 - 1;
        let v5: u64 = f313(v0.clone(), v1)?;
        let a1690 = v3;
        let a1691 = 1u64;
        if nat_eq(a1690, a1691) {
            let m1692 = f325(v0.clone(), v1, v4, 0u64)?;
            match m1692 {
                None => {
                    Ok(None::<(Adt19, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f313(v0.clone(), {
                        let (_, h1693) = v6.clone();
                        h1693
                    })?;
                    let a1694 = v7;
                    let a1695 = 44u64;
                    if nat_eq(a1694, a1695) {
                        let m1699 = f325(v0, {
                            let a1697 = {
                                let (_, h1696) = v6;
                                h1696
                            };
                            let a1698 = 1u64;
                            nat_add(a1697, a1698)?
                        }, v4, 1u64)?;
                        match m1699 {
                            None => {
                                Ok(None::<(Adt19, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt19::C7(std::rc::Rc::new({
                                    let (h1700, _) = v6;
                                    h1700
                                }), std::rc::Rc::new({
                                    let (h1701, _) = v8;
                                    h1701
                                })), {
                                    let (_, h1702) = v8;
                                    h1702
                                })))
                            }
                        }
                    } else {
                        let a1703 = v7;
                        let a1704 = 93u64;
                        if nat_eq(a1703, a1704) {
                            Ok(Some((Adt19::C7(std::rc::Rc::new({
                                let (h1705, _) = v6;
                                h1705
                            }), std::rc::Rc::new(Adt19::C6)), {
                                let a1707 = {
                                    let (_, h1706) = v6;
                                    h1706
                                };
                                let a1708 = 1u64;
                                nat_add(a1707, a1708)?
                            })))
                        } else {
                            Ok(None::<(Adt19, u64)>)
                        }
                    }
                }
            }
        } else {
            let a1709 = v3;
            let a1710 = 2u64;
            if nat_eq(a1709, a1710) {
                let m1713 = f381(v0.clone(), {
                    let a1711 = v1;
                    let a1712 = 1u64;
                    nat_add(a1711, a1712)?
                })?;
                match m1713 {
                    None => {
                        Ok(None::<(Adt19, u64)>)
                    }
                    Some(v9) => {
                        let a1714 = v5;
                        let a1715 = 34u64;
                        if if nat_eq(a1714, a1715) {
                            let a1717 = f313(v0.clone(), {
                                let (h1716, _) = v9.clone();
                                h1716
                            })?;
                            let a1718 = 58u64;
                            nat_eq(a1717, a1718)
                        } else {
                            false
                        } {
                            let m1722 = f325(v0.clone(), {
                                let a1720 = {
                                    let (h1719, _) = v9;
                                    h1719
                                };
                                let a1721 = 1u64;
                                nat_add(a1720, a1721)?
                            }, v4, 0u64)?;
                            match m1722 {
                                None => {
                                    Ok(None::<(Adt19, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f313(v0.clone(), {
                                        let (_, h1723) = v10.clone();
                                        h1723
                                    })?;
                                    let a1724 = v11;
                                    let a1725 = 44u64;
                                    if nat_eq(a1724, a1725) {
                                        let m1729 = f325(v0, {
                                            let a1727 = {
                                                let (_, h1726) = v10;
                                                h1726
                                            };
                                            let a1728 = 1u64;
                                            nat_add(a1727, a1728)?
                                        }, v4, 2u64)?;
                                        match m1729 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt19::C8({
                                                    let (_, h1730) = v9;
                                                    h1730
                                                }, std::rc::Rc::new({
                                                    let (h1731, _) = v10;
                                                    h1731
                                                }), std::rc::Rc::new({
                                                    let (h1732, _) = v12;
                                                    h1732
                                                })), {
                                                    let (_, h1733) = v12;
                                                    h1733
                                                })))
                                            }
                                        }
                                    } else {
                                        let a1734 = v11;
                                        let a1735 = 125u64;
                                        if nat_eq(a1734, a1735) {
                                            Ok(Some((Adt19::C8({
                                                let (_, h1736) = v9;
                                                h1736
                                            }, std::rc::Rc::new({
                                                let (h1737, _) = v10;
                                                h1737
                                            }), std::rc::Rc::new(Adt19::C6)), {
                                                let a1739 = {
                                                    let (_, h1738) = v10;
                                                    h1738
                                                };
                                                let a1740 = 1u64;
                                                nat_add(a1739, a1740)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt19, u64)>)
                        }
                    }
                }
            } else {
                let a1741 = v5;
                let a1742 = 34u64;
                if nat_eq(a1741, a1742) {
                    let m1745 = f381(v0, {
                        let a1743 = v1;
                        let a1744 = 1u64;
                        nat_add(a1743, a1744)?
                    })?;
                    match m1745 {
                        None => {
                            Ok(None::<(Adt19, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt19::C3({
                                let (_, h1746) = v13;
                                h1746
                            }), {
                                let (h1747, _) = v13;
                                h1747
                            })))
                        }
                    }
                } else {
                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt19::C0(true), {
                            let a1748 = v1;
                            let a1749 = 4u64;
                            nat_add(a1748, a1749)?
                        })))
                    } else {
                        if f36(f32(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt19::C0(false), {
                                let a1750 = v1;
                                let a1751 = 5u64;
                                nat_add(a1750, a1751)?
                            })))
                        } else {
                            let a1752 = v5;
                            let a1753 = 91u64;
                            if nat_eq(a1752, a1753) {
                                let a1756 = f313(v0.clone(), {
                                    let a1754 = v1;
                                    let a1755 = 1u64;
                                    nat_add(a1754, a1755)?
                                })?;
                                let a1757 = 93u64;
                                if nat_eq(a1756, a1757) {
                                    Ok(Some((Adt19::C4(std::rc::Rc::new(Adt19::C6)), {
                                        let a1758 = v1;
                                        let a1759 = 2u64;
                                        nat_add(a1758, a1759)?
                                    })))
                                } else {
                                    let m1762 = f325(v0, {
                                        let a1760 = v1;
                                        let a1761 = 1u64;
                                        nat_add(a1760, a1761)?
                                    }, v4, 1u64)?;
                                    match m1762 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt19::C4(std::rc::Rc::new({
                                                let (h1763, _) = v14;
                                                h1763
                                            })), {
                                                let (_, h1764) = v14;
                                                h1764
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a1765 = v5;
                                let a1766 = 123u64;
                                if nat_eq(a1765, a1766) {
                                    let a1769 = f313(v0.clone(), {
                                        let a1767 = v1;
                                        let a1768 = 1u64;
                                        nat_add(a1767, a1768)?
                                    })?;
                                    let a1770 = 125u64;
                                    if nat_eq(a1769, a1770) {
                                        Ok(Some((Adt19::C5(std::rc::Rc::new(Adt19::C6)), {
                                            let a1771 = v1;
                                            let a1772 = 2u64;
                                            nat_add(a1771, a1772)?
                                        })))
                                    } else {
                                        let m1775 = f325(v0, {
                                            let a1773 = v1;
                                            let a1774 = 1u64;
                                            nat_add(a1773, a1774)?
                                        }, v4, 2u64)?;
                                        match m1775 {
                                            None => {
                                                Ok(None::<(Adt19, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt19::C5(std::rc::Rc::new({
                                                    let (h1776, _) = v15;
                                                    h1776
                                                })), {
                                                    let (_, h1777) = v15;
                                                    h1777
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m1778 = f383(v0, v1)?;
                                    match m1778 {
                                        None => {
                                            Ok(None::<(Adt19, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h1779) = v16;
                                                h1779
                                            }, {
                                                let (h1780, _) = v16;
                                                h1780
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

pub fn f326(v0: Adt19, v1: Bytes) -> bool {
    let a1782 = {
        let a1781 = f151(v0, v1);
        length_bytes(a1781)
    };
    let a1783 = 0u64;
    !nat_eq(a1782, a1783)
}

pub fn f327(v0: Adt19, v1: Bytes) -> bool {
    let m1784 = f148(v0, v1);
    match m1784 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f328(v0: Bytes, p1851: u64, p1852: u64, p1853: bool, p1854: Bytes) -> R<Bytes> {
    let m1850 = (p1851, (p1852, (p1853, p1854)));
    {
        let mut m1850 = m1850;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1850;
                let m1785 = v2;
                if m1785 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1785 - 1;
                    let m1788 = {
                        let a1786 = v0.clone();
                        let a1787 = v1;
                        index_bytes(a1786, a1787)
                    };
                    match m1788 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1833 = {
                                    let a1809 = {
                                        let a1797 = {
                                            let a1791 = {
                                                let a1789 = v7;
                                                let a1790 = 128u8;
                                                fixed_u8::bit_and(a1789, a1790)
                                            };
                                            let a1792 = 128u8;
                                            if equal(a1791, a1792) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1798 = {
                                            let a1795 = {
                                                let a1793 = v7;
                                                let a1794 = 64u8;
                                                fixed_u8::bit_and(a1793, a1794)
                                            };
                                            let a1796 = 64u8;
                                            if equal(a1795, a1796) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1797, a1798)?
                                    };
                                    let a1810 = {
                                        let a1807 = {
                                            let a1801 = {
                                                let a1799 = v7;
                                                let a1800 = 32u8;
                                                fixed_u8::bit_and(a1799, a1800)
                                            };
                                            let a1802 = 32u8;
                                            if equal(a1801, a1802) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1808 = {
                                            let a1805 = {
                                                let a1803 = v7;
                                                let a1804 = 16u8;
                                                fixed_u8::bit_and(a1803, a1804)
                                            };
                                            let a1806 = 16u8;
                                            if equal(a1805, a1806) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1807, a1808)?
                                    };
                                    nat_add(a1809, a1810)?
                                };
                                let a1834 = {
                                    let a1831 = {
                                        let a1819 = {
                                            let a1813 = {
                                                let a1811 = v7;
                                                let a1812 = 8u8;
                                                fixed_u8::bit_and(a1811, a1812)
                                            };
                                            let a1814 = 8u8;
                                            if equal(a1813, a1814) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1820 = {
                                            let a1817 = {
                                                let a1815 = v7;
                                                let a1816 = 4u8;
                                                fixed_u8::bit_and(a1815, a1816)
                                            };
                                            let a1818 = 4u8;
                                            if equal(a1817, a1818) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1819, a1820)?
                                    };
                                    let a1832 = {
                                        let a1829 = {
                                            let a1823 = {
                                                let a1821 = v7;
                                                let a1822 = 2u8;
                                                fixed_u8::bit_and(a1821, a1822)
                                            };
                                            let a1824 = 2u8;
                                            if equal(a1823, a1824) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1830 = {
                                            let a1827 = {
                                                let a1825 = v7;
                                                let a1826 = 1u8;
                                                fixed_u8::bit_and(a1825, a1826)
                                            };
                                            let a1828 = 1u8;
                                            if equal(a1827, a1828) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1829, a1830)?
                                    };
                                    nat_add(a1831, a1832)?
                                };
                                nat_add(a1833, a1834)?
                            };
                            if f384(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1835 = v1;
                                    let a1836 = 1u64;
                                    nat_add(a1835, a1836)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f385(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1837 = v1;
                                        let a1838 = 1u64;
                                        nat_add(a1837, a1838)?
                                    }, (v5, (false, {
                                        let a1842 = v4;
                                        let a1843 = f32({
                                            let a1839 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1839)
                                        }, {
                                            let a1840 = v8;
                                            let a1841 = 97u64;
                                            nat_sub(a1840, a1841)
                                        }, 1u64);
                                        append_bytes(a1842, a1843)
                                    }))))
                                } else {
                                    if if f385(v8) {
                                        true
                                    } else {
                                        if f386(v8) {
                                            true
                                        } else {
                                            f387(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1844 = v1;
                                            let a1845 = 1u64;
                                            nat_add(a1844, a1845)?
                                        }, (v5, (false, {
                                            let a1846 = v4;
                                            let a1847 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1846, a1847)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1848 = v1;
                                            let a1849 = 1u64;
                                            nat_add(a1848, a1849)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
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

pub fn f329(v0: Adt19, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt19)> = f388(f389(f148(v0.clone(), f8(Str::lit("$defs")))), f389(f148(v0.clone(), f8(Str::lit("definitions")))));
    let m1855 = f390(4096u64, v3.clone(), List::<Bytes>::nil(), f391(v3)?)?;
    match m1855 {
        Ok(v5) => {
            let m1856 = f392(4096u64, f393(f394(v5)?, List::cons(Adt25::C0(v2, v0), List::<Adt25>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m1856 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f215(f186(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f330(v0: List<Adt14>, v1: Bytes, v2: Bytes, v3: Adt14) -> List<Adt14> {
    let m1857 = v0;
    match m1857.uncons() {
        None => {
            List::cons(v3, List::<Adt14>::nil())
        }
        Some((v4, v5)) => {
            if f36(f158(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f395(v5, v1, v2))
            } else {
                List::cons(v4, f330(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f331(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f215(f396(f83(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f332(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1858 = 0u64;
        let a1859 = v1;
        nat_lt(a1858, a1859)
    } else {
        false
    } {
        let a1860 = Bytes::lit(&[0x2d]);
        let a1861 = f126(v1)?;
        Ok(append_bytes(a1860, a1861))
    } else {
        f126(v1)
    }
}

pub fn f333(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f126(v1)?;
    let v4: u64 = {
        let a1862 = v3.clone();
        length_bytes(a1862)
    };
    let v5: Bytes = {
        let a1863 = v2;
        let a1864 = v4;
        if nat_lt(a1863, a1864) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1865 = v4;
                let a1866 = v2;
                nat_sub(a1865, a1866)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1867 = v4;
                let a1868 = v2;
                nat_sub(a1867, a1868)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f397({
                let a1869 = v2;
                let a1870 = v4;
                nat_sub(a1869, a1870)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1871 = f8(Str::lit("-"));
        let a1872 = v5;
        Ok(append_bytes(a1871, a1872))
    } else {
        Ok(v5)
    }
}

pub fn f334(v0: Bytes) -> R<Bytes> {
    let a1878 = {
        let a1876 = Bytes::lit(&[0x22]);
        let a1877 = {
            let (_, h1873) = {
                let (h1874, _) = f304(Fn5::F398(v0.clone()), {
                    let a1875 = v0;
                    length_bytes(a1875)
                }, (0u64, Bytes::lit(&[])))?;
                h1874
            };
            h1873
        };
        append_bytes(a1876, a1877)
    };
    let a1879 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1878, a1879))
}

pub fn f335(v0: List<Bytes>) -> Bytes {
    let a1882 = {
        let a1880 = Bytes::lit(&[0x5b]);
        let a1881 = f121(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1880, a1881)
    };
    let a1883 = Bytes::lit(&[0x5d]);
    append_bytes(a1882, a1883)
}

pub fn f336(v0: Adt19) -> R<List<Bytes>> {
    let m1884 = v0;
    match m1884 {
        Adt19::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C7(r1885, r1886) => {
            let v10 = (*r1885).clone();
            let v11 = (*r1886).clone();
            Ok(List::cons(f245(v10)?, f336(v11)?))
        }
        Adt19::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f337(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f399(Fn10::F400, List::<(Bytes, Bytes)>::nil(), f401(v0)?);
    let v2: List<Bytes> = f402(Fn11::F403, List::<Bytes>::nil(), v1)?;
    let a1889 = {
        let a1887 = Bytes::lit(&[0x7b]);
        let a1888 = f121(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1887, a1888)
    };
    let a1890 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1889, a1890))
}

pub fn f338(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m1891 = v0;
    match m1891 {
        Adt19::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C8(v12, r1892, r1893) => {
            let v13 = (*r1892).clone();
            let v14 = (*r1893).clone();
            Ok(List::cons((v12, f245(v13)?), f338(v14)?))
        }
    }
}

pub fn f339(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1894 = v0;
    match m1894 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f158(v2, v1)
        }
    }
}

pub fn f340(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f215(f404(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f341(v0: Adt16, v1: Option<u64>) -> bool {
    let m1895 = v1;
    match m1895 {
        None => {
            true
        }
        Some(v2) => {
            f263(v2, match v0 {
                Adt16::C0(_, h1896, _, _) => {
                    h1896
                }
            })
        }
    }
}

pub fn f342(v0: Adt12) -> Adt21 {
    Adt21::C1({
        let a1898 = match v0 {
            Adt12::C0(_, _, h1897, _, _) => {
                h1897
            }
        };
        length_list(a1898)
    })
}

pub fn f343(v0: Bytes) -> Option<Adt22> {
    if f36(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt22::C0)
    } else {
        if f36(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt22::C1)
        } else {
            if f36(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt22::C2)
            } else {
                None::<Adt22>
            }
        }
    }
}

pub fn f344(v0: Bytes, p1903: List<Bytes>, p1904: u64) -> R<Option<u64>> {
    let m1902 = (p1903, p1904);
    {
        let mut m1902 = m1902;
        loop {
            let step = {
                let (v1, v2) = m1902;
                let m1899 = v1;
                match m1899.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f36(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1900 = v2;
                                let a1901 = 1u64;
                                nat_add(a1900, a1901)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1902 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: Option<Adt14>) -> Option<u64> {
    let m1905 = v0;
    match m1905 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1906 = v1;
            match m1906 {
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

pub fn f346(v0: Fn6, p1910: List<Bytes>, p1911: List<(Bytes, ())>) -> List<Bytes> {
    let m1909 = (p1910, p1911);
    {
        let mut m1909 = m1909;
        loop {
            let step = {
                let (v1, v2) = m1909;
                let m1907 = v2;
                match m1907.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1908: Fn6 = v0.clone();
                            c1908.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1912, _) = v1;
        h1912
    }, v0)
}

pub fn f348(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f405(Fn12::F406, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f349(v0: Fn7, p1916: List<List<(Bytes, ())>>, p1917: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1915 = (p1916, p1917);
    {
        let mut m1915 = m1915;
        loop {
            let step = {
                let (v1, v2) = m1915;
                let m1913 = v2;
                match m1913.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1914: Fn7 = v0.clone();
                            c1914.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f351(v0: Fn8, p1922: u64, p1923: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1921 = (p1922, p1923);
    {
        let mut m1921 = m1921;
        loop {
            let step = {
                let (v1, v2) = m1921;
                let m1918 = v1;
                if m1918 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1918 - 1;
                    let m1920 = {
                        let c1919: Fn8 = v0.clone();
                        c1919.apply(v2.clone())?
                    };
                    match m1920 {
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
                Ok(next) => { m1921 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f407(v0)
}

pub fn f353(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f408(Fn13::F409, List::<(Bytes, ())>::nil(), v0)
}

pub fn f354(v0: Fn9, p1927: List<(Bytes, ())>, p1928: List<Bytes>) -> List<(Bytes, ())> {
    let m1926 = (p1927, p1928);
    {
        let mut m1926 = m1926;
        loop {
            let step = {
                let (v1, v2) = m1926;
                let m1924 = v2;
                match m1924.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1925: Fn9 = v0.clone();
                            c1925.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1926 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f356(v0: Bytes) -> Bytes {
    if f135(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a1930 = {
                let a1929 = v0;
                length_bytes(a1929)
            };
            let a1931 = 1u64;
            nat_sub(a1930, a1931)
        })
    } else {
        v0
    }
}

pub fn f357(v0: Adt1, v1: Bytes) -> R<(bool, Bytes)> {
    let v2: List<Bytes> = f83(f70(v0.clone(), v1.clone()))?;
    let m1932 = f154(v2.clone())?;
    match m1932 {
        None => {
            f411(v0, v1, v2)
        }
        Some(v3) => {
            f410(v0, v1, v3)
        }
    }
}

pub fn f358(v0: List<Bytes>) -> List<Bytes> {
    let m1933 = v0;
    match m1933.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1935 = {
                    let a1934 = v1.clone();
                    length_bytes(a1934)
                };
                let a1936 = 0u64;
                if nat_eq(a1935, a1936) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f358(v2))
        }
    }
}

pub fn f359(v0: List<Bytes>) -> Bytes {
    let m1937 = v0;
    match m1937.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f360(v0: (Bytes, Bytes), p1940: List<(Bytes, Bytes)>) -> bool {
    let m1939 = p1940;
    {
        let mut m1939 = m1939;
        loop {
            let step = {
                let v1 = m1939;
                let m1938 = v1;
                match m1938.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f412(v0.clone(), v2) {
                            Err::<List<(Bytes, Bytes)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, Bytes)>, bool>(v3)
                        }
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

pub fn f361(v0: Bytes, v1: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m1941 = v1;
    match m1941.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h1942, _) = v2;
                h1942
            }, v0.clone()) {
                List::cons({
                    let (_, h1943) = v2;
                    h1943
                }, f361(v0, v3))
            } else {
                f361(v0, v3)
            }
        }
    }
}

pub fn f362(v0: u32) -> Bytes {
    let a1946 = {
        let a1944 = v0;
        let a1945 = 8u32;
        fixed_u32::bit_and(a1944, a1945)
    };
    let a1947 = 0u32;
    if equal(a1946, a1947) {
        let a1950 = {
            let a1948 = v0;
            let a1949 = 4u32;
            fixed_u32::bit_and(a1948, a1949)
        };
        let a1951 = 0u32;
        if equal(a1950, a1951) {
            let a1954 = {
                let a1952 = v0;
                let a1953 = 2u32;
                fixed_u32::bit_and(a1952, a1953)
            };
            let a1955 = 0u32;
            if equal(a1954, a1955) {
                let a1958 = {
                    let a1956 = v0;
                    let a1957 = 1u32;
                    fixed_u32::bit_and(a1956, a1957)
                };
                let a1959 = 0u32;
                if equal(a1958, a1959) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1962 = {
                    let a1960 = v0;
                    let a1961 = 1u32;
                    fixed_u32::bit_and(a1960, a1961)
                };
                let a1963 = 0u32;
                if equal(a1962, a1963) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1966 = {
                let a1964 = v0;
                let a1965 = 2u32;
                fixed_u32::bit_and(a1964, a1965)
            };
            let a1967 = 0u32;
            if equal(a1966, a1967) {
                let a1970 = {
                    let a1968 = v0;
                    let a1969 = 1u32;
                    fixed_u32::bit_and(a1968, a1969)
                };
                let a1971 = 0u32;
                if equal(a1970, a1971) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1974 = {
                    let a1972 = v0;
                    let a1973 = 1u32;
                    fixed_u32::bit_and(a1972, a1973)
                };
                let a1975 = 0u32;
                if equal(a1974, a1975) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1978 = {
            let a1976 = v0;
            let a1977 = 4u32;
            fixed_u32::bit_and(a1976, a1977)
        };
        let a1979 = 0u32;
        if equal(a1978, a1979) {
            let a1982 = {
                let a1980 = v0;
                let a1981 = 2u32;
                fixed_u32::bit_and(a1980, a1981)
            };
            let a1983 = 0u32;
            if equal(a1982, a1983) {
                let a1986 = {
                    let a1984 = v0;
                    let a1985 = 1u32;
                    fixed_u32::bit_and(a1984, a1985)
                };
                let a1987 = 0u32;
                if equal(a1986, a1987) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1990 = {
                    let a1988 = v0;
                    let a1989 = 1u32;
                    fixed_u32::bit_and(a1988, a1989)
                };
                let a1991 = 0u32;
                if equal(a1990, a1991) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1994 = {
                let a1992 = v0;
                let a1993 = 2u32;
                fixed_u32::bit_and(a1992, a1993)
            };
            let a1995 = 0u32;
            if equal(a1994, a1995) {
                let a1998 = {
                    let a1996 = v0;
                    let a1997 = 1u32;
                    fixed_u32::bit_and(a1996, a1997)
                };
                let a1999 = 0u32;
                if equal(a1998, a1999) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a2002 = {
                    let a2000 = v0;
                    let a2001 = 1u32;
                    fixed_u32::bit_and(a2000, a2001)
                };
                let a2003 = 0u32;
                if equal(a2002, a2003) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f363(v0: u32, v1: u32) -> u32 {
    let m2006 = {
        let a2004 = v0;
        let a2005 = v1;
        fixed_u32::shift_right(a2004, a2005)
    };
    match m2006 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f364(v0: Adt24, v1: Adt26) -> R<Adt24> {
    let v2: Adt27 = f413(Fn14::F414, 64u64, Adt27::C0(match v0.clone() {
        Adt24::C0(h2007, _, _, _, _, _, _, _) => {
            h2007
        }
    }, match v0.clone() {
        Adt24::C0(_, h2008, _, _, _, _, _, _) => {
            h2008
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h2009, _, _, _, _, _) => {
            h2009
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h2010, _, _, _, _) => {
            h2010
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, h2011, _, _, _) => {
            h2011
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2012, _, _) => {
            h2012
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2013, _) => {
            h2013
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, _, h2014) => {
            h2014
        }
    }, v1, 0u64))?;
    Ok(Adt24::C0(f415(match v0.clone() {
        Adt24::C0(h2015, _, _, _, _, _, _, _) => {
            h2015
        }
    }, match v2.clone() {
        Adt27::C0(h2016, _, _, _, _, _, _, _, _, _) => {
            h2016
        }
    }), f415(match v0.clone() {
        Adt24::C0(_, h2017, _, _, _, _, _, _) => {
            h2017
        }
    }, match v2.clone() {
        Adt27::C0(_, h2018, _, _, _, _, _, _, _, _) => {
            h2018
        }
    }), f415(match v0.clone() {
        Adt24::C0(_, _, h2019, _, _, _, _, _) => {
            h2019
        }
    }, match v2.clone() {
        Adt27::C0(_, _, h2020, _, _, _, _, _, _, _) => {
            h2020
        }
    }), f415(match v0.clone() {
        Adt24::C0(_, _, _, h2021, _, _, _, _) => {
            h2021
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, h2022, _, _, _, _, _, _) => {
            h2022
        }
    }), f415(match v0.clone() {
        Adt24::C0(_, _, _, _, h2023, _, _, _) => {
            h2023
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, h2024, _, _, _, _, _) => {
            h2024
        }
    }), f415(match v0.clone() {
        Adt24::C0(_, _, _, _, _, h2025, _, _) => {
            h2025
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, h2026, _, _, _, _) => {
            h2026
        }
    }), f415(match v0.clone() {
        Adt24::C0(_, _, _, _, _, _, h2027, _) => {
            h2027
        }
    }, match v2.clone() {
        Adt27::C0(_, _, _, _, _, _, h2028, _, _, _) => {
            h2028
        }
    }), f415(match v0 {
        Adt24::C0(_, _, _, _, _, _, _, h2029) => {
            h2029
        }
    }, match v2 {
        Adt27::C0(_, _, _, _, _, _, _, h2030, _, _) => {
            h2030
        }
    })))
}

pub fn f365(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt26> {
    Ok(Adt26::C0(f416(v0.clone(), v1, v2, v3)?, f416(v0.clone(), v1, v2, {
        let a2031 = v3;
        let a2032 = 4u64;
        nat_add(a2031, a2032)?
    })?, f416(v0.clone(), v1, v2, {
        let a2033 = v3;
        let a2034 = 8u64;
        nat_add(a2033, a2034)?
    })?, f416(v0.clone(), v1, v2, {
        let a2035 = v3;
        let a2036 = 12u64;
        nat_add(a2035, a2036)?
    })?, f416(v0.clone(), v1, v2, {
        let a2037 = v3;
        let a2038 = 16u64;
        nat_add(a2037, a2038)?
    })?, f416(v0.clone(), v1, v2, {
        let a2039 = v3;
        let a2040 = 20u64;
        nat_add(a2039, a2040)?
    })?, f416(v0.clone(), v1, v2, {
        let a2041 = v3;
        let a2042 = 24u64;
        nat_add(a2041, a2042)?
    })?, f416(v0.clone(), v1, v2, {
        let a2043 = v3;
        let a2044 = 28u64;
        nat_add(a2043, a2044)?
    })?, f416(v0.clone(), v1, v2, {
        let a2045 = v3;
        let a2046 = 32u64;
        nat_add(a2045, a2046)?
    })?, f416(v0.clone(), v1, v2, {
        let a2047 = v3;
        let a2048 = 36u64;
        nat_add(a2047, a2048)?
    })?, f416(v0.clone(), v1, v2, {
        let a2049 = v3;
        let a2050 = 40u64;
        nat_add(a2049, a2050)?
    })?, f416(v0.clone(), v1, v2, {
        let a2051 = v3;
        let a2052 = 44u64;
        nat_add(a2051, a2052)?
    })?, f416(v0.clone(), v1, v2, {
        let a2053 = v3;
        let a2054 = 48u64;
        nat_add(a2053, a2054)?
    })?, f416(v0.clone(), v1, v2, {
        let a2055 = v3;
        let a2056 = 52u64;
        nat_add(a2055, a2056)?
    })?, f416(v0.clone(), v1, v2, {
        let a2057 = v3;
        let a2058 = 56u64;
        nat_add(a2057, a2058)?
    })?, f416(v0, v1, v2, {
        let a2059 = v3;
        let a2060 = 60u64;
        nat_add(a2059, a2060)?
    })?))
}

pub fn f366(p2063: List<Bytes>, v1: Bytes) -> bool {
    let m2062 = p2063;
    {
        let mut m2062 = m2062;
        loop {
            let step = {
                let v0 = m2062;
                let m2061 = v0;
                match m2061.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f205(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2062 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f367(v0: Bytes, v1: u64) -> Option<u64> {
    let a2064 = 0u64;
    let a2065 = v1;
    if nat_lt(a2064, a2065) {
        if f417(v0, {
            let a2066 = v1;
            let a2067 = 1u64;
            nat_sub(a2066, a2067)
        }) {
            Some({
                let a2068 = v1;
                let a2069 = 1u64;
                nat_sub(a2068, a2069)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f368(v0: Adt14) -> R<List<Bytes>> {
    let m2070 = v0;
    match m2070 {
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
        Adt14::C6(r2071, r2072) => {
            let v7 = (*r2071).clone();
            let v8 = (*r2072).clone();
            Ok(List::cons(f303(v7)?, f368(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f369(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m2073 = v0;
    match m2073 {
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
        Adt14::C7(v9, r2074, r2075) => {
            let v10 = (*r2074).clone();
            let v11 = (*r2075).clone();
            Ok(List::cons((v9, f303(v10)?), f369(v11)?))
        }
    }
}

pub fn f370(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h2076, _) = v1;
        h2076
    };
    let a2078 = {
        let a2077 = v0.clone();
        length_bytes(a2077)
    };
    let a2079 = v2;
    if nat_le(a2078, a2079) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a2080 = f227(v0.clone(), v2)?;
        let a2081 = 13u64;
        if nat_eq(a2080, a2081) {
            Ok(Some(({
                let a2084 = f227(v0, {
                    let a2082 = v2;
                    let a2083 = 1u64;
                    nat_add(a2082, a2083)?
                })?;
                let a2085 = 10u64;
                if nat_eq(a2084, a2085) {
                    let a2086 = v2;
                    let a2087 = 2u64;
                    nat_add(a2086, a2087)?
                } else {
                    let a2088 = v2;
                    let a2089 = 1u64;
                    nat_add(a2088, a2089)?
                }
            }, {
                let a2091 = {
                    let (_, h2090) = v1;
                    h2090
                };
                let a2092 = Bytes::lit(&[0x0a]);
                append_bytes(a2091, a2092)
            })))
        } else {
            let v4: u64 = {
                let m2093 = f308(v0.clone(), Bytes::lit(&[0x0d]), v2)?;
                match m2093 {
                    None => {
                        let a2094 = v0.clone();
                        length_bytes(a2094)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2098 = {
                    let (_, h2095) = v1;
                    h2095
                };
                let a2099 = f32(v0, v2, {
                    let a2096 = v4;
                    let a2097 = v2;
                    nat_sub(a2096, a2097)
                });
                append_bytes(a2098, a2099)
            })))
        }
    }
}

pub fn f371(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2100 = f418(v0, v1)?;
    match m2100 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f419({
                let (h2101, _) = v2;
                h2101
            }) {
                Ok(Some({
                    let a2103 = v1;
                    let a2104 = {
                        let (_, h2102) = v2;
                        h2102
                    };
                    nat_add(a2103, a2104)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f372(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f420(v1, v0, v2)
}

pub fn f373(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f421(v0, v1)
}

pub fn f374(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f422(v0, v1, v2)
}

pub fn f375(v0: Bytes, p2107: List<Bytes>) -> bool {
    let m2106 = p2107;
    {
        let mut m2106 = m2106;
        loop {
            let step = {
                let v1 = m2106;
                let m2105 = v1;
                match m2105.uncons() {
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
                Ok(next) => { m2106 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f226(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2110 = f227(v0.clone(), {
                let a2108 = v1;
                let a2109 = 2u64;
                nat_add(a2108, a2109)?
            })?;
            let a2111 = 120u64;
            nat_eq(a2110, a2111)
        };
        let v3: u64 = if v2 {
            let a2112 = v1;
            let a2113 = 3u64;
            nat_add(a2112, a2113)?
        } else {
            let a2114 = v1;
            let a2115 = 2u64;
            nat_add(a2114, a2115)?
        };
        let v4: (u64, u64) = f423(v0.clone(), v2, v3)?;
        let a2117 = {
            let (h2116, _) = v4;
            h2116
        };
        let a2118 = v3;
        if if nat_eq(a2117, a2118) {
            true
        } else {
            let a2120 = f227(v0, {
                let (h2119, _) = v4;
                h2119
            })?;
            let a2121 = 59u64;
            !nat_eq(a2120, a2121)
        } {
            Ok(Err::<(Bytes, u64), Bytes>(f223(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f419({
                let (_, h2122) = v4;
                h2122
            }) {
                Ok(Ok::<(Bytes, u64), Bytes>((f424({
                    let (_, h2123) = v4;
                    h2123
                })?, {
                    let a2125 = {
                        let (h2124, _) = v4;
                        h2124
                    };
                    let a2126 = 1u64;
                    nat_add(a2125, a2126)?
                })))
            } else {
                Ok(Err::<(Bytes, u64), Bytes>(f223(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f226(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3c]), {
                let a2127 = v1;
                let a2128 = 4u64;
                nat_add(a2127, a2128)?
            })))
        } else {
            if f226(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x3e]), {
                    let a2129 = v1;
                    let a2130 = 4u64;
                    nat_add(a2129, a2130)?
                })))
            } else {
                if f226(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x26]), {
                        let a2131 = v1;
                        let a2132 = 5u64;
                        nat_add(a2131, a2132)?
                    })))
                } else {
                    if f226(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x27]), {
                            let a2133 = v1;
                            let a2134 = 6u64;
                            nat_add(a2133, a2134)?
                        })))
                    } else {
                        if f226(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((Bytes::lit(&[0x22]), {
                                let a2135 = v1;
                                let a2136 = 6u64;
                                nat_add(a2135, a2136)?
                            })))
                        } else {
                            Ok(Err::<(Bytes, u64), Bytes>(f223(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f377(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2137, _) = f254(Fn3::F425(v1, v2, v0.clone()), {
        let a2139 = {
            let a2138 = v0;
            length_bytes(a2138)
        };
        let a2140 = 1u64;
        nat_add(a2139, a2140)?
    }, v3)?;
    Ok(h2137)
}

pub fn f378(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2141, _) = v2;
        h2141
    };
    let a2142 = v3;
    let a2143 = 0u64;
    if nat_eq(a2142, a2143) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2144 = v3;
            let a2145 = v1;
            let a2146 = 0u64;
            nat_quot(a2144, a2145, a2146)
        }, {
            let a2151 = f317(v0, {
                let a2147 = v3;
                let a2148 = v1;
                let a2149 = 0u64;
                nat_rem(a2147, a2148, a2149)
            });
            let a2152 = {
                let (_, h2150) = v2;
                h2150
            };
            append_bytes(a2151, a2152)
        }))
    }
}

pub fn f379(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a2153 = v2;
    let a2154 = v0;
    if nat_lt(a2153, a2154) {
        if f417(v1, v2) {
            Ok(Some({
                let a2155 = v2;
                let a2156 = 1u64;
                nat_add(a2155, a2156)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f380(v0: Bytes, p2236: u64, p2237: u64, p2238: bool, p2239: bool, p2240: Bytes) -> R<Bytes> {
    let m2235 = (p2236, (p2237, (p2238, (p2239, p2240))));
    {
        let mut m2235 = m2235;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m2235;
                let m2157 = v2;
                if m2157 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m2157 - 1;
                    let m2160 = {
                        let a2158 = v0.clone();
                        let a2159 = v1;
                        index_bytes(a2158, a2159)
                    };
                    match m2160 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a2205 = {
                                    let a2181 = {
                                        let a2169 = {
                                            let a2163 = {
                                                let a2161 = v8;
                                                let a2162 = 128u8;
                                                fixed_u8::bit_and(a2161, a2162)
                                            };
                                            let a2164 = 128u8;
                                            if equal(a2163, a2164) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2170 = {
                                            let a2167 = {
                                                let a2165 = v8;
                                                let a2166 = 64u8;
                                                fixed_u8::bit_and(a2165, a2166)
                                            };
                                            let a2168 = 64u8;
                                            if equal(a2167, a2168) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2169, a2170)?
                                    };
                                    let a2182 = {
                                        let a2179 = {
                                            let a2173 = {
                                                let a2171 = v8;
                                                let a2172 = 32u8;
                                                fixed_u8::bit_and(a2171, a2172)
                                            };
                                            let a2174 = 32u8;
                                            if equal(a2173, a2174) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2180 = {
                                            let a2177 = {
                                                let a2175 = v8;
                                                let a2176 = 16u8;
                                                fixed_u8::bit_and(a2175, a2176)
                                            };
                                            let a2178 = 16u8;
                                            if equal(a2177, a2178) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2179, a2180)?
                                    };
                                    nat_add(a2181, a2182)?
                                };
                                let a2206 = {
                                    let a2203 = {
                                        let a2191 = {
                                            let a2185 = {
                                                let a2183 = v8;
                                                let a2184 = 8u8;
                                                fixed_u8::bit_and(a2183, a2184)
                                            };
                                            let a2186 = 8u8;
                                            if equal(a2185, a2186) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2192 = {
                                            let a2189 = {
                                                let a2187 = v8;
                                                let a2188 = 4u8;
                                                fixed_u8::bit_and(a2187, a2188)
                                            };
                                            let a2190 = 4u8;
                                            if equal(a2189, a2190) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2191, a2192)?
                                    };
                                    let a2204 = {
                                        let a2201 = {
                                            let a2195 = {
                                                let a2193 = v8;
                                                let a2194 = 2u8;
                                                fixed_u8::bit_and(a2193, a2194)
                                            };
                                            let a2196 = 2u8;
                                            if equal(a2195, a2196) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a2202 = {
                                            let a2199 = {
                                                let a2197 = v8;
                                                let a2198 = 1u8;
                                                fixed_u8::bit_and(a2197, a2198)
                                            };
                                            let a2200 = 1u8;
                                            if equal(a2199, a2200) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a2201, a2202)?
                                    };
                                    nat_add(a2203, a2204)?
                                };
                                nat_add(a2205, a2206)?
                            };
                            let v10: Bytes = f32(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a2207 = v1;
                                    let a2208 = 1u64;
                                    nat_add(a2207, a2208)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a2209 = v9;
                                    let a2210 = 34u64;
                                    !nat_eq(a2209, a2210)
                                }, (if v4 {
                                    false
                                } else {
                                    let a2211 = v9;
                                    let a2212 = 92u64;
                                    nat_eq(a2211, a2212)
                                }, {
                                    let a2213 = v5;
                                    let a2214 = v10;
                                    append_bytes(a2213, a2214)
                                })))))
                            } else {
                                let a2215 = v9;
                                let a2216 = 32u64;
                                if if if if nat_eq(a2215, a2216) {
                                    true
                                } else {
                                    let a2217 = v9;
                                    let a2218 = 9u64;
                                    nat_eq(a2217, a2218)
                                } {
                                    true
                                } else {
                                    let a2219 = v9;
                                    let a2220 = 10u64;
                                    nat_eq(a2219, a2220)
                                } {
                                    true
                                } else {
                                    let a2221 = v9;
                                    let a2222 = 13u64;
                                    nat_eq(a2221, a2222)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a2223 = v1;
                                        let a2224 = 1u64;
                                        nat_add(a2223, a2224)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f36(f32(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2225 = v1;
                                            let a2226 = 4u64;
                                            nat_add(a2225, a2226)?
                                        }, (v6, (false, (false, {
                                            let a2227 = v5;
                                            let a2228 = f8(Str::lit("[]"));
                                            append_bytes(a2227, a2228)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a2229 = v1;
                                            let a2230 = 1u64;
                                            nat_add(a2229, a2230)?
                                        }, (v6, ({
                                            let a2231 = v9;
                                            let a2232 = 34u64;
                                            nat_eq(a2231, a2232)
                                        }, (false, {
                                            let a2233 = v5;
                                            let a2234 = v10;
                                            append_bytes(a2233, a2234)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2235 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f381(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f426(Fn15::F427(v0.clone()), {
        let a2242 = {
            let a2241 = v0;
            length_bytes(a2241)
        };
        let a2243 = 1u64;
        nat_add(a2242, a2243)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a2247 = {
        let (_, h2244) = {
            let (_, h2245) = {
                let (h2246, _) = v2.clone();
                h2246
            };
            h2245
        };
        h2244
    };
    let a2248 = 1u64;
    if nat_eq(a2247, a2248) {
        Ok(Some(({
            let (h2249, _) = {
                let (h2250, _) = v2.clone();
                h2250
            };
            h2249
        }, {
            let (h2251, _) = {
                let (_, h2252) = {
                    let (h2253, _) = v2;
                    h2253
                };
                h2252
            };
            h2251
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f382(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2254 = f313(v0.clone(), v1)?;
        let a2255 = 45u64;
        nat_eq(a2254, a2255)
    };
    let v3: u64 = if v2 {
        let a2256 = v1;
        let a2257 = 1u64;
        nat_add(a2256, a2257)?
    } else {
        v1
    };
    let v4: u64 = f428(v0.clone(), v3)?;
    let v5: u64 = {
        let a2258 = v4;
        let a2259 = v3;
        nat_sub(a2258, a2259)
    };
    let v6: bool = {
        let a2260 = f313(v0.clone(), v4)?;
        let a2261 = 46u64;
        nat_eq(a2260, a2261)
    };
    let v7: u64 = if v6 {
        let a2262 = v4;
        let a2263 = 1u64;
        nat_add(a2262, a2263)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f428(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f313(v0.clone(), v8)?;
    let v10: bool = {
        let a2264 = v9;
        let a2265 = 101u64;
        if nat_eq(a2264, a2265) {
            true
        } else {
            let a2266 = v9;
            let a2267 = 69u64;
            nat_eq(a2266, a2267)
        }
    };
    let v11: u64 = f313(v0.clone(), {
        let a2268 = v8;
        let a2269 = 1u64;
        nat_add(a2268, a2269)?
    })?;
    let v12: bool = if v10 {
        let a2270 = v11;
        let a2271 = 43u64;
        if nat_eq(a2270, a2271) {
            true
        } else {
            let a2272 = v11;
            let a2273 = 45u64;
            nat_eq(a2272, a2273)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2274 = v8;
            let a2275 = 2u64;
            nat_add(a2274, a2275)?
        } else {
            let a2276 = v8;
            let a2277 = 1u64;
            nat_add(a2276, a2277)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f428(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2278 = v14;
        let a2279 = v13;
        nat_sub(a2278, a2279)
    });
    let v16: u64 = f429(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2281 = {
            let a2280 = v15;
            length_bytes(a2280)
        };
        let a2282 = v16;
        nat_sub(a2281, a2282)
    });
    let a2283 = v5;
    let a2284 = 0u64;
    if if nat_eq(a2283, a2284) {
        true
    } else {
        let a2285 = 1u64;
        let a2286 = v5;
        if if nat_lt(a2285, a2286) {
            let a2287 = f313(v0.clone(), v3)?;
            let a2288 = 48u64;
            nat_eq(a2287, a2288)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2289 = v8;
                let a2290 = v7;
                nat_eq(a2289, a2290)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2291 = v14;
                    let a2292 = v13;
                    nat_eq(a2291, a2292)
                } else {
                    false
                } {
                    true
                } else {
                    let a2294 = 18u64;
                    let a2295 = {
                        let a2293 = v17.clone();
                        length_bytes(a2293)
                    };
                    nat_lt(a2294, a2295)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2299 = {
            let a2297 = {
                let a2296 = v17.clone();
                length_bytes(a2296)
            };
            let a2298 = 0u64;
            if nat_eq(a2297, a2298) {
                Some(0u64)
            } else {
                f430(v17)?
            }
        };
        match m2299 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2304 = f431(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2300 = v8;
                    let a2301 = v7;
                    nat_sub(a2300, a2301)
                }), if v12 {
                    let a2302 = v11;
                    let a2303 = 45u64;
                    nat_eq(a2302, a2303)
                } else {
                    false
                }, v18)?;
                match m2304 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2306 = 19u64;
                        let a2307 = {
                            let a2305 = v19.clone();
                            length_bytes(a2305)
                        };
                        if nat_lt(a2306, a2307) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2308 = f430(v19)?;
                            match m2308 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2309 = 0u64;
                                        let a2310 = v20;
                                        nat_lt(a2309, a2310)
                                    } else {
                                        false
                                    };
                                    if f432(v21, v20) {
                                        Ok(Some((v14, Adt14::C1(v21, v20))))
                                    } else {
                                        Ok(None::<(u64, Adt14)>)
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

pub fn f383(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2311 = f313(v0.clone(), v1)?;
        let a2312 = 45u64;
        nat_eq(a2311, a2312)
    };
    let v3: u64 = if v2 {
        let a2313 = v1;
        let a2314 = 1u64;
        nat_add(a2313, a2314)?
    } else {
        v1
    };
    let v4: u64 = f428(v0.clone(), v3)?;
    let v5: u64 = {
        let a2315 = v4;
        let a2316 = v3;
        nat_sub(a2315, a2316)
    };
    let v6: bool = {
        let a2317 = f313(v0.clone(), v4)?;
        let a2318 = 46u64;
        nat_eq(a2317, a2318)
    };
    let v7: u64 = if v6 {
        let a2319 = v4;
        let a2320 = 1u64;
        nat_add(a2319, a2320)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f428(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f313(v0.clone(), v8)?;
    let v10: bool = {
        let a2321 = v9;
        let a2322 = 101u64;
        if nat_eq(a2321, a2322) {
            true
        } else {
            let a2323 = v9;
            let a2324 = 69u64;
            nat_eq(a2323, a2324)
        }
    };
    let v11: u64 = f313(v0.clone(), {
        let a2325 = v8;
        let a2326 = 1u64;
        nat_add(a2325, a2326)?
    })?;
    let v12: bool = if v10 {
        let a2327 = v11;
        let a2328 = 43u64;
        if nat_eq(a2327, a2328) {
            true
        } else {
            let a2329 = v11;
            let a2330 = 45u64;
            nat_eq(a2329, a2330)
        }
    } else {
        false
    };
    let v13: u64 = if v10 {
        if v12 {
            let a2331 = v8;
            let a2332 = 2u64;
            nat_add(a2331, a2332)?
        } else {
            let a2333 = v8;
            let a2334 = 1u64;
            nat_add(a2333, a2334)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f428(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2335 = v14;
        let a2336 = v13;
        nat_sub(a2335, a2336)
    });
    let v16: u64 = f429(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2338 = {
            let a2337 = v15;
            length_bytes(a2337)
        };
        let a2339 = v16;
        nat_sub(a2338, a2339)
    });
    let a2340 = v5;
    let a2341 = 0u64;
    if if nat_eq(a2340, a2341) {
        true
    } else {
        let a2342 = 1u64;
        let a2343 = v5;
        if if nat_lt(a2342, a2343) {
            let a2344 = f313(v0.clone(), v3)?;
            let a2345 = 48u64;
            nat_eq(a2344, a2345)
        } else {
            false
        } {
            true
        } else {
            if if v6 {
                let a2346 = v8;
                let a2347 = v7;
                nat_eq(a2346, a2347)
            } else {
                false
            } {
                true
            } else {
                if if v10 {
                    let a2348 = v14;
                    let a2349 = v13;
                    nat_eq(a2348, a2349)
                } else {
                    false
                } {
                    true
                } else {
                    let a2351 = 18u64;
                    let a2352 = {
                        let a2350 = v17.clone();
                        length_bytes(a2350)
                    };
                    nat_lt(a2351, a2352)
                }
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2356 = {
            let a2354 = {
                let a2353 = v17.clone();
                length_bytes(a2353)
            };
            let a2355 = 0u64;
            if nat_eq(a2354, a2355) {
                Some(0u64)
            } else {
                f430(v17)?
            }
        };
        match m2356 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2361 = f433(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2357 = v8;
                    let a2358 = v7;
                    nat_sub(a2357, a2358)
                }), if v12 {
                    let a2359 = v11;
                    let a2360 = 45u64;
                    nat_eq(a2359, a2360)
                } else {
                    false
                }, v18)?;
                match m2361 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2364 = 19u64;
                        let a2365 = {
                            let a2363 = {
                                let (h2362, _) = v19.clone();
                                h2362
                            };
                            length_bytes(a2363)
                        };
                        if nat_lt(a2364, a2365) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2367 = f430({
                                let (h2366, _) = v19.clone();
                                h2366
                            })?;
                            match m2367 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = if v2 {
                                        let a2368 = 0u64;
                                        let a2369 = v20;
                                        nat_lt(a2368, a2369)
                                    } else {
                                        false
                                    };
                                    if f432(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2371 = {
                                                let (_, h2370) = v19.clone();
                                                h2370
                                            };
                                            let a2372 = 0u64;
                                            if nat_eq(a2371, a2372) {
                                                Adt19::C1(v21, v20)
                                            } else {
                                                Adt19::C2(v21, v20, {
                                                    let (_, h2373) = v19;
                                                    h2373
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt19)>)
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

pub fn f384(v0: u64) -> bool {
    let a2374 = v0;
    let a2375 = 95u64;
    if nat_eq(a2374, a2375) {
        true
    } else {
        let a2376 = v0;
        let a2377 = 45u64;
        if nat_eq(a2376, a2377) {
            true
        } else {
            let a2378 = v0;
            let a2379 = 32u64;
            if nat_eq(a2378, a2379) {
                true
            } else {
                let a2380 = v0;
                let a2381 = 46u64;
                if nat_eq(a2380, a2381) {
                    true
                } else {
                    let a2382 = v0;
                    let a2383 = 47u64;
                    nat_eq(a2382, a2383)
                }
            }
        }
    }
}

pub fn f385(v0: u64) -> bool {
    let a2384 = 97u64;
    let a2385 = v0;
    if nat_le(a2384, a2385) {
        let a2386 = v0;
        let a2387 = 122u64;
        nat_le(a2386, a2387)
    } else {
        false
    }
}

pub fn f386(v0: u64) -> bool {
    let a2388 = 65u64;
    let a2389 = v0;
    if nat_le(a2388, a2389) {
        let a2390 = v0;
        let a2391 = 90u64;
        nat_le(a2390, a2391)
    } else {
        false
    }
}

pub fn f387(v0: u64) -> bool {
    let a2392 = 48u64;
    let a2393 = v0;
    if nat_le(a2392, a2393) {
        let a2394 = v0;
        let a2395 = 57u64;
        nat_le(a2394, a2395)
    } else {
        false
    }
}

pub fn f388(v0: List<(Bytes, Adt19)>, v1: List<(Bytes, Adt19)>) -> List<(Bytes, Adt19)> {
    let m2396 = v0;
    match m2396.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f388(v3, v1))
        }
    }
}

pub fn f389(v0: Option<Adt19>) -> List<(Bytes, Adt19)> {
    let m2397 = v0;
    match m2397 {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some(v1) => {
            f434(v1)
        }
    }
}

pub fn f390(v0: u64, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt19)>, Bytes>> {
    let m2398 = v0;
    if m2398 == 0 {
        let m2399 = v1;
        match m2399.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m2398 - 1;
        let m2400 = v1.clone();
        match m2400.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::<(Bytes, Adt19)>::nil()))
            }
            Some((_, _)) => {
                let m2401 = f435(v1.clone(), v2.clone(), v3.clone())?;
                match m2401 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt19)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f215(f391(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m2404 = f390(v6, f436(v1, {
                            let (h2402, _) = v9.clone();
                            h2402
                        }), List::cons(f242({
                            let (h2403, _) = v9.clone();
                            h2403
                        })?, v2), v3)?;
                        match m2404 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt19)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt19)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f391(v0: List<(Bytes, Adt19)>) -> R<List<Bytes>> {
    let m2405 = v0;
    match m2405.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f242({
                let (h2406, _) = v1;
                h2406
            })?, f391(v2)?))
        }
    }
}

pub fn f392(p2414: u64, p2415: List<Adt25>, p2416: List<Bytes>, p2417: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2413 = (p2414, (p2415, (p2416, p2417)));
    {
        let mut m2413 = m2413;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2413;
                let m2407 = v0;
                if m2407 == 0 {
                    Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m2407 - 1;
                    let m2408 = v1;
                    match m2408.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m2409 = v5;
                            match m2409 {
                                Adt25::C0(v8, v9) => {
                                    if f36(v8.clone(), f437()) {
                                        Err::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f86(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m2410 = f439(v8.clone(), v9)?;
                                            match m2410 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f393({
                                                        let (_, h2411) = v11;
                                                        h2411
                                                    }, List::cons(Adt25::C1({
                                                        let (h2412, _) = v11;
                                                        h2412
                                                    }), v6)), (List::cons(v8, v2), v3))))
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
                                    if f86(f437(), v2.clone()) {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt25>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f437(), v2), List::cons(f438(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2413 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2418 = v0;
    match m2418.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f393(v3, v1))
        }
    }
}

pub fn f394(v0: List<(Bytes, Adt19)>) -> R<List<Adt25>> {
    let m2419 = v0;
    match m2419.uncons() {
        None => {
            Ok(List::<Adt25>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt25::C0(f242({
                let (h2420, _) = v1;
                h2420
            })?, {
                let (_, h2421) = v1;
                h2421
            }), f394(v2)?))
        }
    }
}

pub fn f395(v0: List<Adt14>, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m2422 = v0;
    match m2422.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            if f36(f158(v3.clone(), v1.clone()), v2.clone()) {
                f395(v4, v1, v2)
            } else {
                List::cons(v3, f395(v4, v1, v2))
            }
        }
    }
}

pub fn f396(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m2423 = v0;
    match m2423.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f205(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f215(f404(f78(List::cons(v1, f440(f30(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f396(v3, v1)?))
            }
        }
    }
}

pub fn f397(v0: u64) -> Bytes {
    let m2424 = v0;
    if m2424 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2424 - 1;
        let a2425 = f8(Str::lit("0"));
        let a2426 = f397(v1);
        append_bytes(a2425, a2426)
    }
}

pub fn f398(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f441(v0, v1)
}

pub fn f399(v0: Fn10, p2430: List<(Bytes, Bytes)>, p2431: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2429 = (p2430, p2431);
    {
        let mut m2429 = m2429;
        loop {
            let step = {
                let (v1, v2) = m2429;
                let m2427 = v2;
                match m2427.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2428: Fn10 = v0.clone();
                            c2428.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f442(v0, v1)
}

pub fn f401(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f443(v0)
}

pub fn f402(v0: Fn11, p2435: List<Bytes>, p2436: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2434 = (p2435, p2436);
    {
        let mut m2434 = m2434;
        loop {
            let step = {
                let (v1, v2) = m2434;
                let m2432 = v2;
                match m2432.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2433: Fn11 = v0.clone();
                            c2433.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f444(v1)?, v0))
}

pub fn f404(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2437 = v0;
    match m2437.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f210(v1)?, f404(v2)?))
        }
    }
}

pub fn f405(v0: Fn12, p2441: List<List<(Bytes, ())>>, p2442: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m2440 = (p2441, p2442);
    {
        let mut m2440 = m2440;
        loop {
            let step = {
                let (v1, v2) = m2440;
                let m2438 = v2;
                match m2438.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c2439: Fn12 = v0.clone();
                            c2439.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f407(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a2444 = {
        let a2443 = v0.clone();
        length_list(a2443)
    };
    let a2445 = 1u64;
    if nat_le(a2444, a2445) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f445(v0)?))
    }
}

pub fn f408(v0: Fn13, p2449: List<(Bytes, ())>, p2450: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m2448 = (p2449, p2450);
    {
        let mut m2448 = m2448;
        loop {
            let step = {
                let (v1, v2) = m2448;
                let m2446 = v2;
                match m2446.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c2447: Fn13 = v0.clone();
                            c2447.apply(v1, v3)
                        }, v4))
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

pub fn f409(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f410(v0: Adt1, v1: Bytes, v2: List<Adt14>) -> R<(bool, Bytes)> {
    let v3: List<Bytes> = f72(v2, f8(Str::lit("at")));
    let v4: List<Bytes> = f446(v3.clone(), f72(f73(v0, f89())?, f8(Str::lit("at"))));
    let a2452 = {
        let a2451 = v4.clone();
        length_list(a2451)
    };
    let a2453 = 0u64;
    if nat_eq(a2452, a2453) {
        Ok((false, Bytes::lit(&[])))
    } else {
        let a2456 = {
            let a2454 = v4;
            length_list(a2454)
        };
        let a2457 = {
            let a2455 = v3;
            length_list(a2455)
        };
        if nat_eq(a2456, a2457) {
            Ok((true, f7(List::cons(f8(Str::lit("record-decision ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        } else {
            Ok((false, f447(v1, f8(Str::lit("some of its decisions are recorded already, so it needs a corrected file")))))
        }
    }
}

pub fn f411(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<(bool, Bytes)> {
    let v3: List<Adt14> = f237(f141(v2.clone(), f8(Str::lit("requirement")))?);
    let v4: List<Adt14> = f237(f141(v2, f8(Str::lit("admission")))?);
    let v5: List<Bytes> = f64(f72(v3.clone(), f8(Str::lit("id"))), f72(v4.clone(), f8(Str::lit("construct"))));
    let v6: List<Bytes> = f72(f73(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id")));
    let v7: List<Bytes> = f446(v5.clone(), f64(v6.clone(), f72(f73(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))));
    let a2459 = {
        let a2458 = v7.clone();
        length_list(a2458)
    };
    let a2460 = 0u64;
    if nat_eq(a2459, a2460) {
        Ok((false, Bytes::lit(&[])))
    } else {
        if f75({
            let a2463 = {
                let a2461 = v7;
                length_list(a2461)
            };
            let a2464 = {
                let a2462 = v5;
                length_list(a2462)
            };
            nat_eq(a2463, a2464)
        }) {
            Ok((false, f447(v1, f8(Str::lit("some of its names are in the model already, so it needs a corrected file")))))
        } else {
            let v8: List<Bytes> = f78(f64(f446(f448(v4.clone()), f69(f70(v0, f8(Str::lit("model/authorities.toml"))))?), f446(f449(v4), f64(v6, f72(v3, f8(Str::lit("id")))))))?;
            let a2466 = {
                let a2465 = v8.clone();
                length_list(a2465)
            };
            let a2467 = 0u64;
            if nat_eq(a2466, a2467) {
                Ok((true, f7(List::cons(f8(Str::lit("admit ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            } else {
                Ok((false, f447(v1, f7(List::cons(f8(Str::lit("cites ")), List::cons(f121(v8, f8(Str::lit(" "))), List::cons(f8(Str::lit(", which are not in the model yet")), List::<Bytes>::nil())))))))
            }
        }
    }
}

pub fn f412(v0: (Bytes, Bytes), v1: (Bytes, Bytes)) -> bool {
    if f36({
        let (h2468, _) = v0;
        h2468
    }, {
        let (h2469, _) = v1;
        h2469
    }) {
        f36({
            let (_, h2470) = v0;
            h2470
        }, {
            let (_, h2471) = v1;
            h2471
        })
    } else {
        false
    }
}

pub fn f413(v0: Fn14, p2475: u64, p2476: Adt27) -> R<Adt27> {
    let m2474 = (p2475, p2476);
    {
        let mut m2474 = m2474;
        loop {
            let step = {
                let (v1, v2) = m2474;
                let m2472 = v1;
                if m2472 == 0 {
                    Err::<(u64, Adt27), R<Adt27>>(Ok(v2))
                } else {
                    let v3 = m2472 - 1;
                    Ok::<(u64, Adt27), R<Adt27>>((v3, {
                        let c2473: Fn14 = v0.clone();
                        c2473.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2474 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f414(v0: Adt27) -> R<Adt27> {
    f450(v0)
}

pub fn f415(v0: u32, v1: u32) -> u32 {
    let m2479 = {
        let a2477 = f451(v0);
        let a2478 = f451(v1);
        fixed_u64::checked_add(a2477, a2478)
    };
    match m2479 {
        None => {
            0u32
        }
        Some(v2) => {
            f452(v2)
        }
    }
}

pub fn f416(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2482 = {
        let a2480 = v3;
        let a2481 = 8u64;
        nat_add(a2480, a2481)?
    };
    let a2483 = v2;
    if nat_eq(a2482, a2483) {
        Ok(f453({
            let a2486 = {
                let a2484 = v1;
                let a2485 = 8u64;
                nat_mul(a2484, a2485)?
            };
            let a2487 = 4294967296u64;
            let a2488 = 0u64;
            nat_quot(a2486, a2487, a2488)
        }))
    } else {
        let a2491 = {
            let a2489 = v3;
            let a2490 = 4u64;
            nat_add(a2489, a2490)?
        };
        let a2492 = v2;
        if nat_eq(a2491, a2492) {
            Ok(f453({
                let a2495 = {
                    let a2493 = v1;
                    let a2494 = 8u64;
                    nat_mul(a2493, a2494)?
                };
                let a2496 = 4294967296u64;
                let a2497 = 0u64;
                nat_rem(a2495, a2496, a2497)
            }))
        } else {
            Ok(f454(f455(v0.clone(), v1, v3), f455(v0.clone(), v1, {
                let a2498 = v3;
                let a2499 = 1u64;
                nat_add(a2498, a2499)?
            }), f455(v0.clone(), v1, {
                let a2500 = v3;
                let a2501 = 2u64;
                nat_add(a2500, a2501)?
            }), f455(v0, v1, {
                let a2502 = v3;
                let a2503 = 3u64;
                nat_add(a2502, a2503)?
            })))
        }
    }
}

pub fn f417(v0: Bytes, v1: u64) -> bool {
    let m2504 = f252(v0, v1);
    match m2504 {
        None => {
            false
        }
        Some(v2) => {
            f456(v2)
        }
    }
}

pub fn f418(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2507 = {
        let a2505 = v0.clone();
        let a2506 = v1;
        index_bytes(a2505, a2506)
    };
    match m2507 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2552 = {
                    let a2528 = {
                        let a2516 = {
                            let a2510 = {
                                let a2508 = v3;
                                let a2509 = 128u8;
                                fixed_u8::bit_and(a2508, a2509)
                            };
                            let a2511 = 128u8;
                            if equal(a2510, a2511) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2517 = {
                            let a2514 = {
                                let a2512 = v3;
                                let a2513 = 64u8;
                                fixed_u8::bit_and(a2512, a2513)
                            };
                            let a2515 = 64u8;
                            if equal(a2514, a2515) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2516, a2517)?
                    };
                    let a2529 = {
                        let a2526 = {
                            let a2520 = {
                                let a2518 = v3;
                                let a2519 = 32u8;
                                fixed_u8::bit_and(a2518, a2519)
                            };
                            let a2521 = 32u8;
                            if equal(a2520, a2521) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2527 = {
                            let a2524 = {
                                let a2522 = v3;
                                let a2523 = 16u8;
                                fixed_u8::bit_and(a2522, a2523)
                            };
                            let a2525 = 16u8;
                            if equal(a2524, a2525) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2526, a2527)?
                    };
                    nat_add(a2528, a2529)?
                };
                let a2553 = {
                    let a2550 = {
                        let a2538 = {
                            let a2532 = {
                                let a2530 = v3;
                                let a2531 = 8u8;
                                fixed_u8::bit_and(a2530, a2531)
                            };
                            let a2533 = 8u8;
                            if equal(a2532, a2533) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2539 = {
                            let a2536 = {
                                let a2534 = v3;
                                let a2535 = 4u8;
                                fixed_u8::bit_and(a2534, a2535)
                            };
                            let a2537 = 4u8;
                            if equal(a2536, a2537) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2538, a2539)?
                    };
                    let a2551 = {
                        let a2548 = {
                            let a2542 = {
                                let a2540 = v3;
                                let a2541 = 2u8;
                                fixed_u8::bit_and(a2540, a2541)
                            };
                            let a2543 = 2u8;
                            if equal(a2542, a2543) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2549 = {
                            let a2546 = {
                                let a2544 = v3;
                                let a2545 = 1u8;
                                fixed_u8::bit_and(a2544, a2545)
                            };
                            let a2547 = 1u8;
                            if equal(a2546, a2547) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2548, a2549)?
                    };
                    nat_add(a2550, a2551)?
                };
                nat_add(a2552, a2553)?
            };
            let a2554 = v4;
            let a2555 = 128u64;
            if nat_lt(a2554, a2555) {
                Ok(Some((v4, 1u64)))
            } else {
                let a2556 = 194u64;
                let a2557 = v4;
                if if nat_le(a2556, a2557) {
                    let a2558 = v4;
                    let a2559 = 223u64;
                    nat_le(a2558, a2559)
                } else {
                    false
                } {
                    let m2562 = f457(v0, {
                        let a2560 = v1;
                        let a2561 = 1u64;
                        nat_add(a2560, a2561)?
                    })?;
                    match m2562 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v5) => {
                            Ok(f458({
                                let a2567 = {
                                    let a2565 = {
                                        let a2563 = v4;
                                        let a2564 = 192u64;
                                        nat_sub(a2563, a2564)
                                    };
                                    let a2566 = 64u64;
                                    nat_mul(a2565, a2566)?
                                };
                                let a2568 = v5;
                                nat_add(a2567, a2568)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    let a2569 = 224u64;
                    let a2570 = v4;
                    if if nat_le(a2569, a2570) {
                        let a2571 = v4;
                        let a2572 = 239u64;
                        nat_le(a2571, a2572)
                    } else {
                        false
                    } {
                        let m2575 = f457(v0.clone(), {
                            let a2573 = v1;
                            let a2574 = 1u64;
                            nat_add(a2573, a2574)?
                        })?;
                        match m2575 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v6) => {
                                let m2578 = f457(v0, {
                                    let a2576 = v1;
                                    let a2577 = 2u64;
                                    nat_add(a2576, a2577)?
                                })?;
                                match m2578 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v7) => {
                                        Ok(f458({
                                            let a2587 = {
                                                let a2581 = {
                                                    let a2579 = v4;
                                                    let a2580 = 224u64;
                                                    nat_sub(a2579, a2580)
                                                };
                                                let a2582 = 4096u64;
                                                nat_mul(a2581, a2582)?
                                            };
                                            let a2588 = {
                                                let a2585 = {
                                                    let a2583 = v6;
                                                    let a2584 = 64u64;
                                                    nat_mul(a2583, a2584)?
                                                };
                                                let a2586 = v7;
                                                nat_add(a2585, a2586)?
                                            };
                                            nat_add(a2587, a2588)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        let a2589 = 240u64;
                        let a2590 = v4;
                        if if nat_le(a2589, a2590) {
                            let a2591 = v4;
                            let a2592 = 244u64;
                            nat_le(a2591, a2592)
                        } else {
                            false
                        } {
                            let m2595 = f457(v0.clone(), {
                                let a2593 = v1;
                                let a2594 = 1u64;
                                nat_add(a2593, a2594)?
                            })?;
                            match m2595 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v8) => {
                                    let m2598 = f457(v0.clone(), {
                                        let a2596 = v1;
                                        let a2597 = 2u64;
                                        nat_add(a2596, a2597)?
                                    })?;
                                    match m2598 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v9) => {
                                            let m2601 = f457(v0, {
                                                let a2599 = v1;
                                                let a2600 = 3u64;
                                                nat_add(a2599, a2600)?
                                            })?;
                                            match m2601 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v10) => {
                                                    Ok(f458({
                                                        let a2614 = {
                                                            let a2604 = {
                                                                let a2602 = v4;
                                                                let a2603 = 240u64;
                                                                nat_sub(a2602, a2603)
                                                            };
                                                            let a2605 = 262144u64;
                                                            nat_mul(a2604, a2605)?
                                                        };
                                                        let a2615 = {
                                                            let a2612 = {
                                                                let a2606 = v8;
                                                                let a2607 = 4096u64;
                                                                nat_mul(a2606, a2607)?
                                                            };
                                                            let a2613 = {
                                                                let a2610 = {
                                                                    let a2608 = v9;
                                                                    let a2609 = 64u64;
                                                                    nat_mul(a2608, a2609)?
                                                                };
                                                                let a2611 = v10;
                                                                nat_add(a2610, a2611)?
                                                            };
                                                            nat_add(a2612, a2613)?
                                                        };
                                                        nat_add(a2614, a2615)?
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

pub fn f419(v0: u64) -> bool {
    let a2616 = v0;
    let a2617 = 9u64;
    if if if nat_eq(a2616, a2617) {
        true
    } else {
        let a2618 = v0;
        let a2619 = 10u64;
        nat_eq(a2618, a2619)
    } {
        true
    } else {
        let a2620 = v0;
        let a2621 = 13u64;
        nat_eq(a2620, a2621)
    } {
        true
    } else {
        let a2622 = 32u64;
        let a2623 = v0;
        if if if nat_le(a2622, a2623) {
            let a2624 = v0;
            let a2625 = 55295u64;
            nat_le(a2624, a2625)
        } else {
            false
        } {
            true
        } else {
            let a2626 = 57344u64;
            let a2627 = v0;
            if nat_le(a2626, a2627) {
                let a2628 = v0;
                let a2629 = 65533u64;
                nat_le(a2628, a2629)
            } else {
                false
            }
        } {
            true
        } else {
            let a2630 = 65536u64;
            let a2631 = v0;
            if nat_le(a2630, a2631) {
                let a2632 = v0;
                let a2633 = 1114111u64;
                nat_le(a2632, a2633)
            } else {
                false
            }
        }
    }
}

pub fn f420(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f220(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2635 = v2;
        let a2636 = {
            let a2634 = v0;
            length_bytes(a2634)
        };
        if nat_lt(a2635, a2636) {
            Ok(Some({
                let a2637 = v2;
                let a2638 = 1u64;
                nat_add(a2637, a2638)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f421(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f307(f227(v0, v1)?) {
        Ok(Some({
            let a2639 = v1;
            let a2640 = 1u64;
            nat_add(a2639, a2640)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f422(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2641 = f418(v0, v2)?;
    match m2641 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2642 = v2;
            let a2643 = v1;
            if if nat_eq(a2642, a2643) {
                f459({
                    let (h2644, _) = v3;
                    h2644
                })
            } else {
                f460({
                    let (h2645, _) = v3;
                    h2645
                })
            } {
                Ok(Some({
                    let a2647 = v2;
                    let a2648 = {
                        let (_, h2646) = v3;
                        h2646
                    };
                    nat_add(a2647, a2648)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f423(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2649, _) = f461(Fn16::F462(v1, v0.clone()), {
        let a2651 = {
            let a2650 = v0;
            length_bytes(a2650)
        };
        let a2652 = 1u64;
        nat_add(a2651, a2652)?
    }, (v2, 0u64))?;
    Ok(h2649)
}

pub fn f424(v0: u64) -> R<Bytes> {
    let a2653 = v0;
    let a2654 = 128u64;
    if nat_lt(a2653, a2654) {
        Ok(f463(v0))
    } else {
        let a2655 = v0;
        let a2656 = 2048u64;
        if nat_lt(a2655, a2656) {
            let a2667 = f463({
                let a2660 = 192u64;
                let a2661 = {
                    let a2657 = v0;
                    let a2658 = 64u64;
                    let a2659 = 0u64;
                    nat_quot(a2657, a2658, a2659)
                };
                nat_add(a2660, a2661)?
            });
            let a2668 = f463({
                let a2665 = 128u64;
                let a2666 = {
                    let a2662 = v0;
                    let a2663 = 64u64;
                    let a2664 = 0u64;
                    nat_rem(a2662, a2663, a2664)
                };
                nat_add(a2665, a2666)?
            });
            Ok(append_bytes(a2667, a2668))
        } else {
            let a2669 = v0;
            let a2670 = 65536u64;
            if nat_lt(a2669, a2670) {
                let a2691 = f463({
                    let a2674 = 224u64;
                    let a2675 = {
                        let a2671 = v0;
                        let a2672 = 4096u64;
                        let a2673 = 0u64;
                        nat_quot(a2671, a2672, a2673)
                    };
                    nat_add(a2674, a2675)?
                });
                let a2692 = {
                    let a2689 = f463({
                        let a2682 = 128u64;
                        let a2683 = {
                            let a2679 = {
                                let a2676 = v0;
                                let a2677 = 64u64;
                                let a2678 = 0u64;
                                nat_quot(a2676, a2677, a2678)
                            };
                            let a2680 = 64u64;
                            let a2681 = 0u64;
                            nat_rem(a2679, a2680, a2681)
                        };
                        nat_add(a2682, a2683)?
                    });
                    let a2690 = f463({
                        let a2687 = 128u64;
                        let a2688 = {
                            let a2684 = v0;
                            let a2685 = 64u64;
                            let a2686 = 0u64;
                            nat_rem(a2684, a2685, a2686)
                        };
                        nat_add(a2687, a2688)?
                    });
                    append_bytes(a2689, a2690)
                };
                Ok(append_bytes(a2691, a2692))
            } else {
                let a2723 = f463({
                    let a2696 = 240u64;
                    let a2697 = {
                        let a2693 = v0;
                        let a2694 = 262144u64;
                        let a2695 = 0u64;
                        nat_quot(a2693, a2694, a2695)
                    };
                    nat_add(a2696, a2697)?
                });
                let a2724 = {
                    let a2721 = f463({
                        let a2704 = 128u64;
                        let a2705 = {
                            let a2701 = {
                                let a2698 = v0;
                                let a2699 = 4096u64;
                                let a2700 = 0u64;
                                nat_quot(a2698, a2699, a2700)
                            };
                            let a2702 = 64u64;
                            let a2703 = 0u64;
                            nat_rem(a2701, a2702, a2703)
                        };
                        nat_add(a2704, a2705)?
                    });
                    let a2722 = {
                        let a2719 = f463({
                            let a2712 = 128u64;
                            let a2713 = {
                                let a2709 = {
                                    let a2706 = v0;
                                    let a2707 = 64u64;
                                    let a2708 = 0u64;
                                    nat_quot(a2706, a2707, a2708)
                                };
                                let a2710 = 64u64;
                                let a2711 = 0u64;
                                nat_rem(a2709, a2710, a2711)
                            };
                            nat_add(a2712, a2713)?
                        });
                        let a2720 = f463({
                            let a2717 = 128u64;
                            let a2718 = {
                                let a2714 = v0;
                                let a2715 = 64u64;
                                let a2716 = 0u64;
                                nat_rem(a2714, a2715, a2716)
                            };
                            nat_add(a2717, a2718)?
                        });
                        append_bytes(a2719, a2720)
                    };
                    append_bytes(a2721, a2722)
                };
                Ok(append_bytes(a2723, a2724))
            }
        }
    }
}

pub fn f425(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f464(v2, v0, v1, v3)
}

pub fn f426(v0: Fn15, p2729: u64, p2730: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m2728 = (p2729, p2730);
    {
        let mut m2728 = m2728;
        loop {
            let step = {
                let (v1, v2) = m2728;
                let m2725 = v1;
                if m2725 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2725 - 1;
                    let m2727 = {
                        let c2726: Fn15 = v0.clone();
                        c2726.apply(v2.clone())?
                    };
                    match m2727 {
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
                Ok(next) => { m2728 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f427(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f465(v0, v1)
}

pub fn f428(v0: Bytes, v1: u64) -> R<u64> {
    let (h2731, _) = f254(Fn3::F466(v0.clone()), {
        let a2733 = {
            let a2732 = v0;
            length_bytes(a2732)
        };
        let a2734 = 1u64;
        nat_add(a2733, a2734)?
    }, v1)?;
    Ok(h2731)
}

pub fn f429(v0: Bytes) -> R<u64> {
    let (h2735, _) = f254(Fn3::F467(v0.clone()), {
        let a2737 = {
            let a2736 = v0;
            length_bytes(a2736)
        };
        let a2738 = 1u64;
        nat_add(a2737, a2738)?
    }, 0u64)?;
    Ok(h2735)
}

pub fn f430(v0: Bytes) -> R<Option<u64>> {
    if f468(v0.clone())? {
        Ok(Some({
            let (_, h2739) = {
                let (h2740, _) = f461(Fn16::F469(v0.clone()), {
                    let a2741 = v0;
                    length_bytes(a2741)
                }, (0u64, 0u64))?;
                h2740
            };
            h2739
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f431(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<Bytes>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2743 = {
            let a2742 = v1.clone();
            length_bytes(a2742)
        };
        let a2744 = f470(v1)?;
        nat_sub(a2743, a2744)
    });
    let v5: Bytes = {
        let a2745 = v0;
        let a2746 = v4.clone();
        append_bytes(a2745, a2746)
    };
    let v6: u64 = f429(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2748 = {
            let a2747 = v5;
            length_bytes(a2747)
        };
        let a2749 = v6;
        nat_sub(a2748, a2749)
    });
    let v8: u64 = {
        let a2750 = v4;
        length_bytes(a2750)
    };
    let v9: u64 = {
        let a2751 = v7.clone();
        length_bytes(a2751)
    };
    let a2752 = v9;
    let a2753 = 0u64;
    if nat_eq(a2752, a2753) {
        Ok(Some(f8(Str::lit("0"))))
    } else {
        if if v2 {
            true
        } else {
            let a2754 = v3;
            let a2755 = v8;
            nat_lt(a2754, a2755)
        } {
            let v10: u64 = if v2 {
                let a2756 = v3;
                let a2757 = v8;
                nat_add(a2756, a2757)?
            } else {
                let a2758 = v8;
                let a2759 = v3;
                nat_sub(a2758, a2759)
            };
            let a2760 = v10;
            let a2761 = f470(v7.clone())?;
            if nat_le(a2760, a2761) {
                Ok(Some(f32(v7, 0u64, {
                    let a2762 = v9;
                    let a2763 = v10;
                    nat_sub(a2762, a2763)
                })))
            } else {
                Ok(None::<Bytes>)
            }
        } else {
            let v11: u64 = {
                let a2764 = v3;
                let a2765 = v8;
                nat_sub(a2764, a2765)
            };
            let a2768 = {
                let a2766 = v9;
                let a2767 = v11;
                nat_add(a2766, a2767)?
            };
            let a2769 = 19u64;
            if nat_le(a2768, a2769) {
                Ok(Some({
                    let a2770 = v7;
                    let a2771 = f397(v11);
                    append_bytes(a2770, a2771)
                }))
            } else {
                Ok(None::<Bytes>)
            }
        }
    }
}

pub fn f432(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2772 = 0u64;
        let a2773 = v1;
        if nat_lt(a2772, a2773) {
            let a2774 = v1;
            let a2775 = 9223372036854775808u64;
            nat_le(a2774, a2775)
        } else {
            false
        }
    } else {
        let a2776 = v1;
        let a2777 = 9223372036854775807u64;
        nat_le(a2776, a2777)
    }
}

pub fn f433(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2779 = {
            let a2778 = v1.clone();
            length_bytes(a2778)
        };
        let a2780 = f470(v1)?;
        nat_sub(a2779, a2780)
    });
    let v5: Bytes = {
        let a2781 = v0;
        let a2782 = v4.clone();
        append_bytes(a2781, a2782)
    };
    let v6: u64 = f429(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2784 = {
            let a2783 = v5;
            length_bytes(a2783)
        };
        let a2785 = v6;
        nat_sub(a2784, a2785)
    });
    let v8: u64 = {
        let a2786 = v4;
        length_bytes(a2786)
    };
    let v9: u64 = {
        let a2787 = v7.clone();
        length_bytes(a2787)
    };
    let a2788 = v9;
    let a2789 = 0u64;
    if nat_eq(a2788, a2789) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if if v2 {
            true
        } else {
            let a2790 = v3;
            let a2791 = v8;
            nat_lt(a2790, a2791)
        } {
            let v10: u64 = if v2 {
                let a2792 = v3;
                let a2793 = v8;
                nat_add(a2792, a2793)?
            } else {
                let a2794 = v8;
                let a2795 = v3;
                nat_sub(a2794, a2795)
            };
            let v11: u64 = f470(v7.clone())?;
            let a2796 = v10;
            let a2797 = v11;
            if nat_le(a2796, a2797) {
                Ok(Some((f32(v7, 0u64, {
                    let a2798 = v9;
                    let a2799 = v10;
                    nat_sub(a2798, a2799)
                }), 0u64)))
            } else {
                let a2802 = {
                    let a2800 = v10;
                    let a2801 = v11;
                    nat_sub(a2800, a2801)
                };
                let a2803 = 19u64;
                if nat_le(a2802, a2803) {
                    Ok(Some((f32(v7, 0u64, {
                        let a2804 = v9;
                        let a2805 = v11;
                        nat_sub(a2804, a2805)
                    }), {
                        let a2806 = v10;
                        let a2807 = v11;
                        nat_sub(a2806, a2807)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a2808 = v3;
                let a2809 = v8;
                nat_sub(a2808, a2809)
            };
            let a2812 = {
                let a2810 = v9;
                let a2811 = v12;
                nat_add(a2810, a2811)?
            };
            let a2813 = 19u64;
            if nat_le(a2812, a2813) {
                Ok(Some(({
                    let a2814 = v7;
                    let a2815 = f397(v12);
                    append_bytes(a2814, a2815)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f434(v0: Adt19) -> List<(Bytes, Adt19)> {
    let m2816 = v0;
    match m2816 {
        Adt19::C0(_) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C1(_, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C2(_, _, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C3(_) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C4(_) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C5(r2817) => {
            let v4 = (*r2817).clone();
            f434(v4)
        }
        Adt19::C6 => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C7(_, _) => {
            List::<(Bytes, Adt19)>::nil()
        }
        Adt19::C8(v1, r2818, r2819) => {
            let v2 = (*r2818).clone();
            let v3 = (*r2819).clone();
            List::cons((v1, v2), f434(v3))
        }
    }
}

pub fn f435(p2824: List<(Bytes, Adt19)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt19)>> {
    let m2823 = p2824;
    {
        let mut m2823 = m2823;
        loop {
            let step = {
                let v0 = m2823;
                let m2820 = v0;
                match m2820.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(None::<(Bytes, Adt19)>))
                    }
                    Some((v3, v4)) => {
                        if f471(f472({
                            let (_, h2821) = v3.clone();
                            h2821
                        }, List::<Bytes>::nil())?, f242({
                            let (h2822, _) = v3.clone();
                            h2822
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, R<Option<(Bytes, Adt19)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2823 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: List<(Bytes, Adt19)>, v1: Bytes) -> List<(Bytes, Adt19)> {
    let m2825 = v0;
    match m2825.uncons() {
        None => {
            List::<(Bytes, Adt19)>::nil()
        }
        Some((v2, v3)) => {
            if f36({
                let (h2826, _) = v2.clone();
                h2826
            }, v1.clone()) {
                f436(v3, v1)
            } else {
                List::cons(v2, f436(v3, v1))
            }
        }
    }
}

pub fn f437() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f438() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f439(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f473(v0, f474(v1))
}

pub fn f440(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m2827 = v0;
    match m2827.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f440(v3, false))
            } else {
                f440(v3, true)
            }
        }
    }
}

pub fn f441(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2831 = {
        let a2829 = v0.clone();
        let a2830 = {
            let (h2828, _) = v1.clone();
            h2828
        };
        index_bytes(a2829, a2830)
    };
    match m2831 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2833 = {
                    let (h2832, _) = v1.clone();
                    h2832
                };
                let a2834 = 1u64;
                nat_add(a2833, a2834)?
            }, {
                let a2837 = {
                    let (_, h2835) = v1;
                    h2835
                };
                let a2838 = f475(v0, {
                    let (h2836, _) = v1;
                    h2836
                }, v2)?;
                append_bytes(a2837, a2838)
            })))
        }
    }
}

pub fn f442(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2839 = v0.clone();
    match m2839.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2842 = {
                let (h2840, _) = v2;
                h2840
            };
            let a2843 = {
                let (h2841, _) = v1.clone();
                h2841
            };
            if equal(a2842, a2843) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f443(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f476(f477(Fn17::F478, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2848 = {
        let (h2844, _) = f479(Fn18::F480, {
            let a2846 = {
                let a2845 = v0;
                length_list(a2845)
            };
            let a2847 = 1u64;
            nat_add(a2846, a2847)?
        }, v1)?;
        h2844
    };
    match m2848.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f444(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2853 = {
        let a2850 = f334({
            let (h2849, _) = v0;
            h2849
        })?;
        let a2851 = Bytes::lit(&[0x3a]);
        append_bytes(a2850, a2851)
    };
    let a2854 = {
        let (_, h2852) = v0;
        h2852
    };
    Ok(append_bytes(a2853, a2854))
}

pub fn f445(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f348({
        let (_, h2855) = {
            let (h2856, _) = f481(Fn19::F482, {
                let a2858 = {
                    let a2857 = v0.clone();
                    length_list(a2857)
                };
                let a2859 = 1u64;
                nat_add(a2858, a2859)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h2856
        };
        h2855
    }))
}

pub fn f446(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2860 = v0;
    match m2860.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f86(v2.clone(), v1.clone()) {
                f446(v3, v1)
            } else {
                List::cons(v2, f446(v3, v1))
            }
        }
    }
}

pub fn f447(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("waiting ")), List::cons(v0, List::cons(f8(Str::lit(": ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f448(v0: List<Adt14>) -> List<Bytes> {
    let m2861 = v0;
    match m2861.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2862 = f160(v1, f8(Str::lit("std")));
            match m2862 {
                None => {
                    f448(v2)
                }
                Some(v3) => {
                    List::cons(f158(v3, f8(Str::lit("authority"))), f448(v2))
                }
            }
        }
    }
}

pub fn f449(v0: List<Adt14>) -> List<Bytes> {
    let m2863 = v0;
    match m2863.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m2864 = f161(f160(v1, f8(Str::lit("req"))));
            match m2864 {
                None => {
                    f449(v2)
                }
                Some(v3) => {
                    List::cons(v3, f449(v2))
                }
            }
        }
    }
}

pub fn f450(v0: Adt27) -> R<Adt27> {
    let v1: u32 = f483(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, _, h2865) => {
            h2865
        }
    });
    let v2: u32 = f415(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, h2866, _, _) => {
            h2866
        }
    }, f415(f484(match v0.clone() {
        Adt27::C0(_, _, _, _, h2867, _, _, _, _, _) => {
            h2867
        }
    }), f415(f485(match v0.clone() {
        Adt27::C0(_, _, _, _, h2868, _, _, _, _, _) => {
            h2868
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2869, _, _, _, _) => {
            h2869
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2870, _, _, _) => {
            h2870
        }
    }), f415(v1, {
        let m2873 = match v0.clone() {
            Adt27::C0(_, _, _, _, _, _, _, _, h2872, _) => {
                h2872
            }
        };
        match m2873 {
            Adt26::C0(h2871, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2871
            }
        }
    }))));
    let v3: u32 = f415(f486(match v0.clone() {
        Adt27::C0(h2874, _, _, _, _, _, _, _, _, _) => {
            h2874
        }
    }), f487(match v0.clone() {
        Adt27::C0(h2875, _, _, _, _, _, _, _, _, _) => {
            h2875
        }
    }, match v0.clone() {
        Adt27::C0(_, h2876, _, _, _, _, _, _, _, _) => {
            h2876
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2877, _, _, _, _, _, _, _) => {
            h2877
        }
    }));
    Ok(Adt27::C0(f415(v2, v3), match v0.clone() {
        Adt27::C0(h2878, _, _, _, _, _, _, _, _, _) => {
            h2878
        }
    }, match v0.clone() {
        Adt27::C0(_, h2879, _, _, _, _, _, _, _, _) => {
            h2879
        }
    }, match v0.clone() {
        Adt27::C0(_, _, h2880, _, _, _, _, _, _, _) => {
            h2880
        }
    }, f415(match v0.clone() {
        Adt27::C0(_, _, _, h2881, _, _, _, _, _, _) => {
            h2881
        }
    }, v2), match v0.clone() {
        Adt27::C0(_, _, _, _, h2882, _, _, _, _, _) => {
            h2882
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, h2883, _, _, _, _) => {
            h2883
        }
    }, match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, h2884, _, _, _) => {
            h2884
        }
    }, f488(match v0.clone() {
        Adt27::C0(_, _, _, _, _, _, _, _, h2885, _) => {
            h2885
        }
    }), {
        let a2887 = match v0 {
            Adt27::C0(_, _, _, _, _, _, _, _, _, h2886) => {
                h2886
            }
        };
        let a2888 = 1u64;
        nat_add(a2887, a2888)?
    }))
}

pub fn f451(v0: u32) -> u64 {
    let m2890 = {
        let a2889 = v0;
        fixed_u64::convert(i128::from(a2889))
    };
    match m2890 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f452(v0: u64) -> u32 {
    let m2894 = {
        let a2893 = {
            let a2891 = v0;
            let a2892 = 4294967295u64;
            fixed_u64::bit_and(a2891, a2892)
        };
        fixed_u32::convert(i128::from(a2893))
    };
    match m2894 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f453(v0: u64) -> u32 {
    let (_, h2895) = {
        let (_, h2896) = f489(Fn20::F490, 32u64, (v0, (1u32, 0u32)));
        h2896
    };
    h2895
}

pub fn f454(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2901 = f491(v0, 24u32);
    let a2902 = {
        let a2899 = f491(v1, 16u32);
        let a2900 = {
            let a2897 = f491(v2, 8u32);
            let a2898 = v3;
            fixed_u32::bit_or(a2897, a2898)
        };
        fixed_u32::bit_or(a2899, a2900)
    };
    fixed_u32::bit_or(a2901, a2902)
}

pub fn f455(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2903 = v2;
    let a2904 = v1;
    if nat_lt(a2903, a2904) {
        let m2907 = {
            let a2905 = v0;
            let a2906 = v2;
            index_bytes(a2905, a2906)
        };
        match m2907 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2909 = {
                    let a2908 = v3;
                    fixed_u32::convert(i128::from(a2908))
                };
                match m2909 {
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
        let a2910 = v2;
        let a2911 = v1;
        if nat_eq(a2910, a2911) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f456(v0: u8) -> bool {
    let a2912 = v0;
    let a2913 = 32u8;
    if if if equal(a2912, a2913) {
        true
    } else {
        let a2914 = v0;
        let a2915 = 9u8;
        equal(a2914, a2915)
    } {
        true
    } else {
        let a2916 = v0;
        let a2917 = 10u8;
        equal(a2916, a2917)
    } {
        true
    } else {
        let a2918 = v0;
        let a2919 = 12u8;
        if equal(a2918, a2919) {
            true
        } else {
            let a2920 = v0;
            let a2921 = 13u8;
            equal(a2920, a2921)
        }
    }
}

pub fn f457(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2924 = {
        let a2922 = v0;
        let a2923 = v1;
        index_bytes(a2922, a2923)
    };
    match m2924 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a2969 = {
                    let a2945 = {
                        let a2933 = {
                            let a2927 = {
                                let a2925 = v3;
                                let a2926 = 128u8;
                                fixed_u8::bit_and(a2925, a2926)
                            };
                            let a2928 = 128u8;
                            if equal(a2927, a2928) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a2934 = {
                            let a2931 = {
                                let a2929 = v3;
                                let a2930 = 64u8;
                                fixed_u8::bit_and(a2929, a2930)
                            };
                            let a2932 = 64u8;
                            if equal(a2931, a2932) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2933, a2934)?
                    };
                    let a2946 = {
                        let a2943 = {
                            let a2937 = {
                                let a2935 = v3;
                                let a2936 = 32u8;
                                fixed_u8::bit_and(a2935, a2936)
                            };
                            let a2938 = 32u8;
                            if equal(a2937, a2938) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a2944 = {
                            let a2941 = {
                                let a2939 = v3;
                                let a2940 = 16u8;
                                fixed_u8::bit_and(a2939, a2940)
                            };
                            let a2942 = 16u8;
                            if equal(a2941, a2942) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2943, a2944)?
                    };
                    nat_add(a2945, a2946)?
                };
                let a2970 = {
                    let a2967 = {
                        let a2955 = {
                            let a2949 = {
                                let a2947 = v3;
                                let a2948 = 8u8;
                                fixed_u8::bit_and(a2947, a2948)
                            };
                            let a2950 = 8u8;
                            if equal(a2949, a2950) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a2956 = {
                            let a2953 = {
                                let a2951 = v3;
                                let a2952 = 4u8;
                                fixed_u8::bit_and(a2951, a2952)
                            };
                            let a2954 = 4u8;
                            if equal(a2953, a2954) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2955, a2956)?
                    };
                    let a2968 = {
                        let a2965 = {
                            let a2959 = {
                                let a2957 = v3;
                                let a2958 = 2u8;
                                fixed_u8::bit_and(a2957, a2958)
                            };
                            let a2960 = 2u8;
                            if equal(a2959, a2960) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a2966 = {
                            let a2963 = {
                                let a2961 = v3;
                                let a2962 = 1u8;
                                fixed_u8::bit_and(a2961, a2962)
                            };
                            let a2964 = 1u8;
                            if equal(a2963, a2964) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a2965, a2966)?
                    };
                    nat_add(a2967, a2968)?
                };
                nat_add(a2969, a2970)?
            };
            let a2971 = 128u64;
            let a2972 = v4;
            if if nat_le(a2971, a2972) {
                let a2973 = v4;
                let a2974 = 191u64;
                nat_le(a2973, a2974)
            } else {
                false
            } {
                Ok(Some({
                    let a2975 = v4;
                    let a2976 = 128u64;
                    nat_sub(a2975, a2976)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f458(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    let a2977 = v1;
    let a2978 = v0;
    if if nat_le(a2977, a2978) {
        let a2979 = v0;
        let a2980 = v2;
        if nat_le(a2979, a2980) {
            let a2981 = v0;
            let a2982 = 55296u64;
            if nat_lt(a2981, a2982) {
                true
            } else {
                let a2983 = 57343u64;
                let a2984 = v0;
                nat_lt(a2983, a2984)
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

pub fn f459(v0: u64) -> bool {
    let a2985 = v0;
    let a2986 = 58u64;
    if if if nat_eq(a2985, a2986) {
        true
    } else {
        let a2987 = v0;
        let a2988 = 95u64;
        nat_eq(a2987, a2988)
    } {
        true
    } else {
        let a2989 = 65u64;
        let a2990 = v0;
        if if nat_le(a2989, a2990) {
            let a2991 = v0;
            let a2992 = 90u64;
            nat_le(a2991, a2992)
        } else {
            false
        } {
            true
        } else {
            let a2993 = 97u64;
            let a2994 = v0;
            if nat_le(a2993, a2994) {
                let a2995 = v0;
                let a2996 = 122u64;
                nat_le(a2995, a2996)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2997 = 192u64;
        let a2998 = v0;
        if if if if nat_le(a2997, a2998) {
            let a2999 = v0;
            let a3000 = 214u64;
            nat_le(a2999, a3000)
        } else {
            false
        } {
            true
        } else {
            let a3001 = 216u64;
            let a3002 = v0;
            if nat_le(a3001, a3002) {
                let a3003 = v0;
                let a3004 = 246u64;
                nat_le(a3003, a3004)
            } else {
                false
            }
        } {
            true
        } else {
            let a3005 = 248u64;
            let a3006 = v0;
            if if nat_le(a3005, a3006) {
                let a3007 = v0;
                let a3008 = 767u64;
                nat_le(a3007, a3008)
            } else {
                false
            } {
                true
            } else {
                let a3009 = 880u64;
                let a3010 = v0;
                if nat_le(a3009, a3010) {
                    let a3011 = v0;
                    let a3012 = 893u64;
                    nat_le(a3011, a3012)
                } else {
                    false
                }
            }
        } {
            true
        } else {
            let a3013 = 895u64;
            let a3014 = v0;
            if if if if nat_le(a3013, a3014) {
                let a3015 = v0;
                let a3016 = 8191u64;
                nat_le(a3015, a3016)
            } else {
                false
            } {
                true
            } else {
                let a3017 = 8204u64;
                let a3018 = v0;
                if nat_le(a3017, a3018) {
                    let a3019 = v0;
                    let a3020 = 8205u64;
                    nat_le(a3019, a3020)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3021 = 8304u64;
                let a3022 = v0;
                if if nat_le(a3021, a3022) {
                    let a3023 = v0;
                    let a3024 = 8591u64;
                    nat_le(a3023, a3024)
                } else {
                    false
                } {
                    true
                } else {
                    let a3025 = 11264u64;
                    let a3026 = v0;
                    if nat_le(a3025, a3026) {
                        let a3027 = v0;
                        let a3028 = 12271u64;
                        nat_le(a3027, a3028)
                    } else {
                        false
                    }
                }
            } {
                true
            } else {
                let a3029 = 12289u64;
                let a3030 = v0;
                if if if nat_le(a3029, a3030) {
                    let a3031 = v0;
                    let a3032 = 55295u64;
                    nat_le(a3031, a3032)
                } else {
                    false
                } {
                    true
                } else {
                    let a3033 = 63744u64;
                    let a3034 = v0;
                    if nat_le(a3033, a3034) {
                        let a3035 = v0;
                        let a3036 = 64975u64;
                        nat_le(a3035, a3036)
                    } else {
                        false
                    }
                } {
                    true
                } else {
                    let a3037 = 65008u64;
                    let a3038 = v0;
                    if if nat_le(a3037, a3038) {
                        let a3039 = v0;
                        let a3040 = 65533u64;
                        nat_le(a3039, a3040)
                    } else {
                        false
                    } {
                        true
                    } else {
                        let a3041 = 65536u64;
                        let a3042 = v0;
                        if nat_le(a3041, a3042) {
                            let a3043 = v0;
                            let a3044 = 983039u64;
                            nat_le(a3043, a3044)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

pub fn f460(v0: u64) -> bool {
    if f459(v0) {
        true
    } else {
        let a3045 = v0;
        let a3046 = 45u64;
        if if if nat_eq(a3045, a3046) {
            true
        } else {
            let a3047 = v0;
            let a3048 = 46u64;
            nat_eq(a3047, a3048)
        } {
            true
        } else {
            let a3049 = 48u64;
            let a3050 = v0;
            if nat_le(a3049, a3050) {
                let a3051 = v0;
                let a3052 = 57u64;
                nat_le(a3051, a3052)
            } else {
                false
            }
        } {
            true
        } else {
            let a3053 = v0;
            let a3054 = 183u64;
            if if nat_eq(a3053, a3054) {
                true
            } else {
                let a3055 = 768u64;
                let a3056 = v0;
                if nat_le(a3055, a3056) {
                    let a3057 = v0;
                    let a3058 = 879u64;
                    nat_le(a3057, a3058)
                } else {
                    false
                }
            } {
                true
            } else {
                let a3059 = 8255u64;
                let a3060 = v0;
                if nat_le(a3059, a3060) {
                    let a3061 = v0;
                    let a3062 = 8256u64;
                    nat_le(a3061, a3062)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f461(v0: Fn16, p3067: u64, p3068: (u64, u64)) -> R<((u64, u64), bool)> {
    let m3066 = (p3067, p3068);
    {
        let mut m3066 = m3066;
        loop {
            let step = {
                let (v1, v2) = m3066;
                let m3063 = v1;
                if m3063 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3063 - 1;
                    let m3065 = {
                        let c3064: Fn16 = v0.clone();
                        c3064.apply(v2)?
                    };
                    match m3065 {
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
                Ok(next) => { m3066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f492(v1, v0, v2)
}

pub fn f463(v0: u64) -> Bytes {
    f493(f494({
        let a3069 = v0;
        let a3070 = 16u64;
        let a3071 = 0u64;
        nat_quot(a3069, a3070, a3071)
    }), {
        let a3072 = v0;
        let a3073 = 16u64;
        let a3074 = 0u64;
        nat_rem(a3072, a3073, a3074)
    })
}

pub fn f464(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f495(f227(v0, v3)?, v1, v2) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3075 = v3;
            let a3076 = 1u64;
            nat_add(a3075, a3076)?
        }))
    }
}

pub fn f465(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h3077, _) = v1.clone();
        h3077
    };
    let v3: Bytes = {
        let (h3078, _) = {
            let (_, h3079) = v1.clone();
            h3079
        };
        h3078
    };
    let a3082 = {
        let (_, h3080) = {
            let (_, h3081) = v1;
            h3081
        };
        h3080
    };
    let a3083 = 0u64;
    if nat_eq(a3082, a3083) {
        let v4: u64 = f313(v0.clone(), v2)?;
        let a3084 = v4;
        let a3085 = 34u64;
        if nat_eq(a3084, a3085) {
            Ok(Some(({
                let a3086 = v2;
                let a3087 = 1u64;
                nat_add(a3086, a3087)?
            }, (v3, 1u64))))
        } else {
            let a3088 = v4;
            let a3089 = 92u64;
            if nat_eq(a3088, a3089) {
                f496(v0, v2, v3)
            } else {
                let a3090 = v4;
                let a3091 = 32u64;
                if if nat_lt(a3090, a3091) {
                    true
                } else {
                    let a3092 = v4;
                    let a3093 = 256u64;
                    nat_eq(a3092, a3093)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a3094 = v2;
                        let a3095 = 1u64;
                        nat_add(a3094, a3095)?
                    }, ({
                        let a3096 = v3;
                        let a3097 = f32(v0, v2, 1u64);
                        append_bytes(a3096, a3097)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f466(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f497(v0, v1)
}

pub fn f467(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f498(v0, v1)
}

pub fn f468(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3098 = v0.clone();
        length_bytes(a3098)
    };
    let a3099 = 0u64;
    let a3100 = v1;
    if nat_lt(a3099, a3100) {
        let a3101 = f499(v0.clone())?;
        let a3102 = v1;
        if nat_eq(a3101, a3102) {
            let a3103 = 1u64;
            let a3104 = v1;
            if nat_lt(a3103, a3104) {
                let m3107 = {
                    let a3105 = v0;
                    let a3106 = 0u64;
                    index_bytes(a3105, a3106)
                };
                match m3107 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3108 = v2;
                        let a3109 = 48u8;
                        Ok(!equal(a3108, a3109))
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

pub fn f469(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f500(v0, v1)
}

pub fn f470(v0: Bytes) -> R<u64> {
    let (h3110, _) = f254(Fn3::F501(v0.clone()), {
        let a3112 = {
            let a3111 = v0;
            length_bytes(a3111)
        };
        let a3113 = 1u64;
        nat_add(a3112, a3113)?
    }, 0u64)?;
    Ok(h3110)
}

pub fn f471(p3116: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m3115 = p3116;
    {
        let mut m3115 = m3115;
        loop {
            let step = {
                let v0 = m3115;
                let m3114 = v0;
                match m3114.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f36(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f86(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f86(v4, v3.clone())
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
                Ok(next) => { m3115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: Adt19, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3117 = v0;
    match m3117 {
        Adt19::C0(_) => {
            Ok(v1)
        }
        Adt19::C1(_, _) => {
            Ok(v1)
        }
        Adt19::C2(_, _, _) => {
            Ok(v1)
        }
        Adt19::C3(_) => {
            Ok(v1)
        }
        Adt19::C4(r3120) => {
            let v2 = (*r3120).clone();
            f472(v2, v1)
        }
        Adt19::C5(r3121) => {
            let v3 = (*r3121).clone();
            f472(v3, v1)
        }
        Adt19::C6 => {
            Ok(v1)
        }
        Adt19::C7(r3122, r3123) => {
            let v4 = (*r3122).clone();
            let v5 = (*r3123).clone();
            f472(v4, f472(v5, v1)?)
        }
        Adt19::C8(v6, r3124, r3125) => {
            let v7 = (*r3124).clone();
            let v8 = (*r3125).clone();
            let v9: List<Bytes> = f472(v7.clone(), f472(v8, v1)?)?;
            if f36(v6, f8(Str::lit("$ref"))) {
                let m3118 = f246(Some(v7));
                match m3118 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m3119 = f502(v10)?;
                        match m3119 {
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

pub fn f473(v0: Bytes, v1: Adt19) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3126 = f148(v1.clone(), f8(Str::lit("enum")));
    match m3126 {
        None => {
            let m3128 = f148(v1.clone(), f8(Str::lit("oneOf")));
            match m3128 {
                None => {
                    if f507(v1.clone(), v0.clone())? {
                        let m3129 = f508(v0.clone(), f389(f148(v1.clone(), f8(Str::lit("properties")))), f152(f153(f148(v1, f8(Str::lit("required"))))), true)?;
                        match m3129 {
                            Ok(v7) => {
                                Ok(f505(f509(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f215({
                                    let (h3130, _) = v7;
                                    h3130
                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), {
                                    let (_, h3131) = v7;
                                    h3131
                                }))
                            }
                            Err(v6) => {
                                Ok(Err::<(Bytes, List<Adt25>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m3132 = f389(f148(v1.clone(), f8(Str::lit("properties"))));
                        match m3132.uncons() {
                            None => {
                                let m3135 = f510(v1, {
                                    let a3133 = v0.clone();
                                    let a3134 = f8(Str::lit("Value"));
                                    append_bytes(a3133, a3134)
                                })?;
                                match m3135 {
                                    Ok(v9) => {
                                        Ok(f505(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons({
                                            let (h3136, _) = v9;
                                            h3136
                                        }, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3137) = v9;
                                            h3137
                                        }))
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(Bytes, List<Adt25>), Bytes>(v8))
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m3138 = f508(v0.clone(), f389(f148(v1.clone(), f8(Str::lit("properties")))), f152(f153(f148(v1, f8(Str::lit("required"))))), false)?;
                                match m3138 {
                                    Ok(v13) => {
                                        Ok(f505(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m3140 = {
                                                let (h3139, _) = v13.clone();
                                                h3139
                                            };
                                            match m3140.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f215({
                                                        let (h3141, _) = v13;
                                                        h3141
                                                    }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                                            let (_, h3142) = v13;
                                            h3142
                                        }))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<(Bytes, List<Adt25>), Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f506(v0, f247(v5))
                }
            }
        }
        Some(v2) => {
            let m3127 = f503(f247(v2), List::<Bytes>::nil())?;
            match m3127 {
                Ok(v4) => {
                    Ok(f505(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f215(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt25>::nil()))
                }
                Err(v3) => {
                    Ok(f504(v3, v0))
                }
            }
        }
    }
}

pub fn f474(v0: Adt19) -> Adt19 {
    let v2: Adt19 = {
        let m3143 = f148(v0.clone(), f8(Str::lit("anyOf")));
        match m3143 {
            None => {
                v0
            }
            Some(v1) => {
                if f511(f247(v1.clone())) {
                    f512(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f513(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f514(f247(v1.clone()))
                    } {
                        v0
                    } else {
                        f515(f512(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt19 = {
        let m3144 = f148(v2.clone(), f8(Str::lit("oneOf")));
        match m3144 {
            None => {
                v2
            }
            Some(v3) => {
                if f511(f247(v3)) {
                    f512(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m3145 = f148(v4.clone(), f8(Str::lit("allOf")));
    match m3145 {
        None => {
            v4
        }
        Some(v5) => {
            if f516(f247(v5)) {
                f512(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f475(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3190 = {
            let a3166 = {
                let a3154 = {
                    let a3148 = {
                        let a3146 = v3;
                        let a3147 = 128u8;
                        fixed_u8::bit_and(a3146, a3147)
                    };
                    let a3149 = 128u8;
                    if equal(a3148, a3149) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3155 = {
                    let a3152 = {
                        let a3150 = v3;
                        let a3151 = 64u8;
                        fixed_u8::bit_and(a3150, a3151)
                    };
                    let a3153 = 64u8;
                    if equal(a3152, a3153) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3154, a3155)?
            };
            let a3167 = {
                let a3164 = {
                    let a3158 = {
                        let a3156 = v3;
                        let a3157 = 32u8;
                        fixed_u8::bit_and(a3156, a3157)
                    };
                    let a3159 = 32u8;
                    if equal(a3158, a3159) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3165 = {
                    let a3162 = {
                        let a3160 = v3;
                        let a3161 = 16u8;
                        fixed_u8::bit_and(a3160, a3161)
                    };
                    let a3163 = 16u8;
                    if equal(a3162, a3163) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3164, a3165)?
            };
            nat_add(a3166, a3167)?
        };
        let a3191 = {
            let a3188 = {
                let a3176 = {
                    let a3170 = {
                        let a3168 = v3;
                        let a3169 = 8u8;
                        fixed_u8::bit_and(a3168, a3169)
                    };
                    let a3171 = 8u8;
                    if equal(a3170, a3171) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3177 = {
                    let a3174 = {
                        let a3172 = v3;
                        let a3173 = 4u8;
                        fixed_u8::bit_and(a3172, a3173)
                    };
                    let a3175 = 4u8;
                    if equal(a3174, a3175) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3176, a3177)?
            };
            let a3189 = {
                let a3186 = {
                    let a3180 = {
                        let a3178 = v3;
                        let a3179 = 2u8;
                        fixed_u8::bit_and(a3178, a3179)
                    };
                    let a3181 = 2u8;
                    if equal(a3180, a3181) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3187 = {
                    let a3184 = {
                        let a3182 = v3;
                        let a3183 = 1u8;
                        fixed_u8::bit_and(a3182, a3183)
                    };
                    let a3185 = 1u8;
                    if equal(a3184, a3185) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3186, a3187)?
            };
            nat_add(a3188, a3189)?
        };
        nat_add(a3190, a3191)?
    };
    let a3192 = v4;
    let a3193 = 34u64;
    if nat_eq(a3192, a3193) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3194 = v4;
        let a3195 = 92u64;
        if nat_eq(a3194, a3195) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3196 = v4;
            let a3197 = 8u64;
            if nat_eq(a3196, a3197) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3198 = v4;
                let a3199 = 12u64;
                if nat_eq(a3198, a3199) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3200 = v4;
                    let a3201 = 10u64;
                    if nat_eq(a3200, a3201) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3202 = v4;
                        let a3203 = 13u64;
                        if nat_eq(a3202, a3203) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3204 = v4;
                            let a3205 = 9u64;
                            if nat_eq(a3204, a3205) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3206 = v4;
                                let a3207 = 32u64;
                                if nat_lt(a3206, a3207) {
                                    let a3208 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3209 = f517(v2)?;
                                    Ok(append_bytes(a3208, a3209))
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

pub fn f476(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f518(Fn21::F519, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f477(v0: Fn17, p3213: List<List<(Bytes, Bytes)>>, p3214: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3212 = (p3213, p3214);
    {
        let mut m3212 = m3212;
        loop {
            let step = {
                let (v1, v2) = m3212;
                let m3210 = v2;
                match m3210.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3211: Fn17 = v0.clone();
                            c3211.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3212 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f479(v0: Fn18, p3219: u64, p3220: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3218 = (p3219, p3220);
    {
        let mut m3218 = m3218;
        loop {
            let step = {
                let (v1, v2) = m3218;
                let m3215 = v1;
                if m3215 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3215 - 1;
                    let m3217 = {
                        let c3216: Fn18 = v0.clone();
                        c3216.apply(v2.clone())?
                    };
                    match m3217 {
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
                Ok(next) => { m3218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f520(v0)
}

pub fn f481(v0: Fn19, p3225: u64, p3226: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m3224 = (p3225, p3226);
    {
        let mut m3224 = m3224;
        loop {
            let step = {
                let (v1, v2) = m3224;
                let m3221 = v1;
                if m3221 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3221 - 1;
                    let m3223 = {
                        let c3222: Fn19 = v0.clone();
                        c3222.apply(v2.clone())?
                    };
                    match m3223 {
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
                Ok(next) => { m3224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f482(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f521(v0)
}

pub fn f483(v0: u64) -> u32 {
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

pub fn f484(v0: u32) -> u32 {
    let a3355 = {
        let a3353 = f522(v0, 6u32, 26u32);
        let a3354 = f522(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3353, a3354)
    };
    let a3356 = f522(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3355, a3356)
}

pub fn f485(v0: u32, v1: u32, v2: u32) -> u32 {
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

pub fn f486(v0: u32) -> u32 {
    let a3366 = {
        let a3364 = f522(v0, 2u32, 30u32);
        let a3365 = f522(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3364, a3365)
    };
    let a3367 = f522(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3366, a3367)
}

pub fn f487(v0: u32, v1: u32, v2: u32) -> u32 {
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

pub fn f488(v0: Adt26) -> Adt26 {
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
    }, f415(f523(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3393, _) => {
            h3393
        }
    }), f415(match v0.clone() {
        Adt26::C0(_, _, _, _, _, _, _, _, _, h3394, _, _, _, _, _, _) => {
            h3394
        }
    }, f415(f524(match v0.clone() {
        Adt26::C0(_, h3395, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3395
        }
    }), match v0 {
        Adt26::C0(h3396, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3396
        }
    }))))
}

pub fn f489(v0: Fn20, p3400: u64, p3401: (u64, (u32, u32))) -> (u64, (u32, u32)) {
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
                        let c3398: Fn20 = v0.clone();
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

pub fn f490(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
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
    }, (f491(v2, 1u32), {
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

pub fn f491(v0: u32, v1: u32) -> u32 {
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

pub fn f492(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f227(v0, {
        let (h3420, _) = v2;
        h3420
    })?;
    let a3422 = 1114111u64;
    let a3423 = {
        let (_, h3421) = v2;
        h3421
    };
    if nat_lt(a3422, a3423) {
        Ok(None::<(u64, u64)>)
    } else {
        let a3424 = 48u64;
        let a3425 = v3;
        if if nat_le(a3424, a3425) {
            let a3426 = v3;
            let a3427 = 57u64;
            nat_le(a3426, a3427)
        } else {
            false
        } {
            Ok(Some(({
                let a3429 = {
                    let (h3428, _) = v2;
                    h3428
                };
                let a3430 = 1u64;
                nat_add(a3429, a3430)?
            }, {
                let a3436 = {
                    let a3432 = {
                        let (_, h3431) = v2;
                        h3431
                    };
                    let a3433 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3432, a3433)?
                };
                let a3437 = {
                    let a3434 = v3;
                    let a3435 = 48u64;
                    nat_sub(a3434, a3435)
                };
                nat_add(a3436, a3437)?
            })))
        } else {
            if if v1 {
                let a3438 = 97u64;
                let a3439 = v3;
                if nat_le(a3438, a3439) {
                    let a3440 = v3;
                    let a3441 = 102u64;
                    nat_le(a3440, a3441)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some(({
                    let a3443 = {
                        let (h3442, _) = v2;
                        h3442
                    };
                    let a3444 = 1u64;
                    nat_add(a3443, a3444)?
                }, {
                    let a3450 = {
                        let a3446 = {
                            let (_, h3445) = v2;
                            h3445
                        };
                        let a3447 = 16u64;
                        nat_mul(a3446, a3447)?
                    };
                    let a3451 = {
                        let a3448 = v3;
                        let a3449 = 87u64;
                        nat_sub(a3448, a3449)
                    };
                    nat_add(a3450, a3451)?
                })))
            } else {
                if if v1 {
                    let a3452 = 65u64;
                    let a3453 = v3;
                    if nat_le(a3452, a3453) {
                        let a3454 = v3;
                        let a3455 = 70u64;
                        nat_le(a3454, a3455)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some(({
                        let a3457 = {
                            let (h3456, _) = v2;
                            h3456
                        };
                        let a3458 = 1u64;
                        nat_add(a3457, a3458)?
                    }, {
                        let a3464 = {
                            let a3460 = {
                                let (_, h3459) = v2;
                                h3459
                            };
                            let a3461 = 16u64;
                            nat_mul(a3460, a3461)?
                        };
                        let a3465 = {
                            let a3462 = v3;
                            let a3463 = 55u64;
                            nat_sub(a3462, a3463)
                        };
                        nat_add(a3464, a3465)?
                    })))
                } else {
                    Ok(None::<(u64, u64)>)
                }
            }
        }
    }
}

pub fn f493(p3472: List<Bytes>, p3473: u64) -> Bytes {
    let m3471 = (p3472, p3473);
    {
        let mut m3471 = m3471;
        loop {
            let step = {
                let (v0, v1) = m3471;
                let m3466 = v0;
                match m3466.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let a3467 = v1;
                        let a3468 = 0u64;
                        if nat_eq(a3467, a3468) {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            Ok::<(List<Bytes>, u64), Bytes>((v3, {
                                let a3469 = v1;
                                let a3470 = 1u64;
                                nat_sub(a3469, a3470)
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3471 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: u64) -> List<Bytes> {
    let a3474 = v0;
    let a3475 = 0u64;
    if nat_eq(a3474, a3475) {
        f525()
    } else {
        let a3476 = v0;
        let a3477 = 1u64;
        if nat_eq(a3476, a3477) {
            f526()
        } else {
            let a3478 = v0;
            let a3479 = 2u64;
            if nat_eq(a3478, a3479) {
                f527()
            } else {
                let a3480 = v0;
                let a3481 = 3u64;
                if nat_eq(a3480, a3481) {
                    f528()
                } else {
                    let a3482 = v0;
                    let a3483 = 4u64;
                    if nat_eq(a3482, a3483) {
                        f529()
                    } else {
                        let a3484 = v0;
                        let a3485 = 5u64;
                        if nat_eq(a3484, a3485) {
                            f530()
                        } else {
                            let a3486 = v0;
                            let a3487 = 6u64;
                            if nat_eq(a3486, a3487) {
                                f531()
                            } else {
                                let a3488 = v0;
                                let a3489 = 7u64;
                                if nat_eq(a3488, a3489) {
                                    f532()
                                } else {
                                    let a3490 = v0;
                                    let a3491 = 8u64;
                                    if nat_eq(a3490, a3491) {
                                        f533()
                                    } else {
                                        let a3492 = v0;
                                        let a3493 = 9u64;
                                        if nat_eq(a3492, a3493) {
                                            f534()
                                        } else {
                                            let a3494 = v0;
                                            let a3495 = 10u64;
                                            if nat_eq(a3494, a3495) {
                                                f535()
                                            } else {
                                                let a3496 = v0;
                                                let a3497 = 11u64;
                                                if nat_eq(a3496, a3497) {
                                                    f536()
                                                } else {
                                                    let a3498 = v0;
                                                    let a3499 = 12u64;
                                                    if nat_eq(a3498, a3499) {
                                                        f537()
                                                    } else {
                                                        let a3500 = v0;
                                                        let a3501 = 13u64;
                                                        if nat_eq(a3500, a3501) {
                                                            f538()
                                                        } else {
                                                            let a3502 = v0;
                                                            let a3503 = 14u64;
                                                            if nat_eq(a3502, a3503) {
                                                                f539()
                                                            } else {
                                                                f540()
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

pub fn f495(v0: u64, v1: bool, v2: u64) -> bool {
    if v1 {
        let a3504 = v0;
        let a3505 = v2;
        if if if nat_eq(a3504, a3505) {
            true
        } else {
            let a3506 = v0;
            let a3507 = 60u64;
            nat_eq(a3506, a3507)
        } {
            true
        } else {
            let a3508 = v0;
            let a3509 = 38u64;
            if nat_eq(a3508, a3509) {
                true
            } else {
                let a3510 = v0;
                let a3511 = 256u64;
                nat_eq(a3510, a3511)
            }
        } {
            true
        } else {
            let a3512 = v0;
            let a3513 = 9u64;
            if nat_eq(a3512, a3513) {
                true
            } else {
                let a3514 = v0;
                let a3515 = 10u64;
                nat_eq(a3514, a3515)
            }
        }
    } else {
        let a3516 = v0;
        let a3517 = 60u64;
        if if nat_eq(a3516, a3517) {
            true
        } else {
            let a3518 = v0;
            let a3519 = 38u64;
            nat_eq(a3518, a3519)
        } {
            true
        } else {
            let a3520 = v0;
            let a3521 = 93u64;
            if nat_eq(a3520, a3521) {
                true
            } else {
                let a3522 = v0;
                let a3523 = 256u64;
                nat_eq(a3522, a3523)
            }
        }
    }
}

pub fn f496(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f313(v0.clone(), {
        let a3524 = v1;
        let a3525 = 1u64;
        nat_add(a3524, a3525)?
    })?;
    let v4: Bytes = {
        let a3526 = v3;
        let a3527 = 117u64;
        if nat_eq(a3526, a3527) {
            f541(v0, v1)?
        } else {
            f542(v3)
        }
    };
    let a3528 = v4.clone();
    let a3529 = Bytes::lit(&[]);
    if equal(a3528, a3529) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a3530 = v3;
            let a3531 = 117u64;
            if nat_eq(a3530, a3531) {
                let a3532 = v1;
                let a3533 = 6u64;
                nat_add(a3532, a3533)?
            } else {
                let a3534 = v1;
                let a3535 = 2u64;
                nat_add(a3534, a3535)?
            }
        }, ({
            let a3536 = v2;
            let a3537 = v4;
            append_bytes(a3536, a3537)
        }, 0u64))))
    }
}

pub fn f497(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f313(v0, v1)?;
    let a3538 = 47u64;
    let a3539 = v2;
    if if nat_lt(a3538, a3539) {
        let a3540 = v2;
        let a3541 = 58u64;
        nat_lt(a3540, a3541)
    } else {
        false
    } {
        Ok(Some({
            let a3542 = v1;
            let a3543 = 1u64;
            nat_add(a3542, a3543)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f498(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3544 = f313(v0, v1)?;
    let a3545 = 48u64;
    if nat_eq(a3544, a3545) {
        Ok(Some({
            let a3546 = v1;
            let a3547 = 1u64;
            nat_add(a3546, a3547)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f499(v0: Bytes) -> R<u64> {
    let (h3548, _) = f254(Fn3::F543(v0.clone()), {
        let a3549 = v0;
        length_bytes(a3549)
    }, 0u64)?;
    Ok(h3548)
}

pub fn f500(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3553 = {
        let a3551 = v0;
        let a3552 = {
            let (h3550, _) = v1;
            h3550
        };
        index_bytes(a3551, a3552)
    };
    match m3553 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3555 = {
                    let (h3554, _) = v1;
                    h3554
                };
                let a3556 = 1u64;
                nat_add(a3555, a3556)?
            }, {
                let a3560 = {
                    let a3558 = {
                        let (_, h3557) = v1;
                        h3557
                    };
                    let a3559 = 10u64;
                    nat_mul(a3558, a3559)?
                };
                let a3561 = f544(v2)?;
                nat_add(a3560, a3561)?
            })))
        }
    }
}

pub fn f501(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f545(v0, v1)
}

pub fn f502(v0: Bytes) -> R<Option<Bytes>> {
    if f205(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f242(f32(v0.clone(), 8u64, {
            let a3563 = {
                let a3562 = v0;
                length_bytes(a3562)
            };
            let a3564 = 8u64;
            nat_sub(a3563, a3564)
        }))?))
    } else {
        if f205(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f242(f32(v0.clone(), 14u64, {
                let a3566 = {
                    let a3565 = v0;
                    length_bytes(a3565)
                };
                let a3567 = 14u64;
                nat_sub(a3566, a3567)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f503(v0: List<Adt19>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f546(f547(f152(v0.clone()))?, v0, v1)
}

pub fn f504(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt25>), Bytes> {
    Err::<(Bytes, List<Adt25>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f505(v0: Bytes, v1: List<Adt25>) -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((v0, v1))
}

pub fn f506(v0: Bytes, v1: List<Adt19>) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3568 = v1.clone();
    match m3568.uncons() {
        None => {
            Ok(f504(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m3569 = f548(f389(f148(v2, f8(Str::lit("properties")))), v1.clone());
            match m3569.uncons() {
                None => {
                    let m3570 = f549(f547(f550(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m3570 {
                        Ok(v5) => {
                            Ok(f505(f509(v0, {
                                let (h3571, _) = v5;
                                h3571
                            }), {
                                let (_, h3572) = v5;
                                h3572
                            }))
                        }
                        Err(v4) => {
                            Ok(f504(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m3573 = f551(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m3573 {
                        Ok(v9) => {
                            Ok(f505(f509(v0, {
                                let (h3574, _) = v9;
                                h3574
                            }), {
                                let (_, h3575) = v9;
                                h3575
                            }))
                        }
                        Err(v8) => {
                            Ok(f504(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f507(v0: Adt19, v1: Bytes) -> R<bool> {
    let m3576 = v0;
    match m3576 {
        Adt19::C0(_) => {
            Ok(false)
        }
        Adt19::C1(_, _) => {
            Ok(false)
        }
        Adt19::C2(_, _, _) => {
            Ok(false)
        }
        Adt19::C3(_) => {
            Ok(false)
        }
        Adt19::C4(r3577) => {
            let v2 = (*r3577).clone();
            f507(v2, v1)
        }
        Adt19::C5(r3578) => {
            let v3 = (*r3578).clone();
            f507(v3, v1)
        }
        Adt19::C6 => {
            Ok(false)
        }
        Adt19::C7(r3579, r3580) => {
            let v4 = (*r3579).clone();
            let v5 = (*r3580).clone();
            if f507(v4, v1.clone())? {
                Ok(true)
            } else {
                f507(v5, v1)
            }
        }
        Adt19::C8(v6, r3581, r3582) => {
            let v7 = (*r3581).clone();
            let v8 = (*r3582).clone();
            if if f36(v6, f8(Str::lit("$ref"))) {
                f552(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f507(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f507(v8, v1)
                }
            }
        }
    }
}

pub fn f508(v0: Bytes, v1: List<(Bytes, Adt19)>, v2: List<Bytes>, v3: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f553(f547(f554(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f509(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f215(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f510(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3583 = f555(v0.clone());
    match m3583 {
        None => {
            if f36(f151(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3584 = f148(v0, f8(Str::lit("items")));
                match m3584 {
                    None => {
                        Ok(f556(f8(Str::lit("List")), f558()))
                    }
                    Some(v3) => {
                        Ok(f556(f8(Str::lit("List")), f557(v3, {
                            let a3585 = v1;
                            let a3586 = f8(Str::lit("Item"));
                            append_bytes(a3585, a3586)
                        })?))
                    }
                }
            } else {
                f557(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f556(f8(Str::lit("Option")), f557(v2, v1)?))
        }
    }
}

pub fn f511(p3589: List<Adt19>) -> bool {
    let m3588 = p3589;
    {
        let mut m3588 = m3588;
        loop {
            let step = {
                let v0 = m3588;
                let m3587 = v0;
                match m3587.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f559(f434(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt19>, bool>(v2)
                        } else {
                            Err::<List<Adt19>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3588 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(v0: Adt19, v1: Bytes) -> Adt19 {
    let m3590 = v0;
    match m3590 {
        Adt19::C0(v6) => {
            Adt19::C0(v6)
        }
        Adt19::C1(v10, v11) => {
            Adt19::C1(v10, v11)
        }
        Adt19::C2(v7, v8, v9) => {
            Adt19::C2(v7, v8, v9)
        }
        Adt19::C3(v12) => {
            Adt19::C3(v12)
        }
        Adt19::C4(r3591) => {
            let v13 = (*r3591).clone();
            Adt19::C4(std::rc::Rc::new(v13))
        }
        Adt19::C5(r3592) => {
            let v5 = (*r3592).clone();
            Adt19::C5(std::rc::Rc::new(f512(v5, v1)))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r3593, r3594) => {
            let v14 = (*r3593).clone();
            let v15 = (*r3594).clone();
            Adt19::C7(std::rc::Rc::new(v14), std::rc::Rc::new(v15))
        }
        Adt19::C8(v2, r3595, r3596) => {
            let v3 = (*r3595).clone();
            let v4 = (*r3596).clone();
            if f36(v2.clone(), v1.clone()) {
                f512(v4, v1)
            } else {
                Adt19::C8(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f512(v4, v1)))
            }
        }
    }
}

pub fn f513(v0: Adt19, v1: Bytes) -> bool {
    let m3597 = f148(v0, v1);
    match m3597 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f514(v0: List<Adt19>) -> bool {
    let m3598 = v0.clone();
    match m3598.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m3599 = f548(f389(f148(v1, f8(Str::lit("properties")))), v0.clone());
            match m3599.uncons() {
                None => {
                    f561(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f560(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f515(v0: Adt19, v1: Bytes, v2: Adt19) -> Adt19 {
    let m3600 = v0;
    match m3600 {
        Adt19::C0(v7) => {
            Adt19::C0(v7)
        }
        Adt19::C1(v11, v12) => {
            Adt19::C1(v11, v12)
        }
        Adt19::C2(v8, v9, v10) => {
            Adt19::C2(v8, v9, v10)
        }
        Adt19::C3(v13) => {
            Adt19::C3(v13)
        }
        Adt19::C4(r3601) => {
            let v14 = (*r3601).clone();
            Adt19::C4(std::rc::Rc::new(v14))
        }
        Adt19::C5(r3602) => {
            let v3 = (*r3602).clone();
            Adt19::C5(std::rc::Rc::new(Adt19::C8(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt19::C6 => {
            Adt19::C6
        }
        Adt19::C7(r3603, r3604) => {
            let v15 = (*r3603).clone();
            let v16 = (*r3604).clone();
            Adt19::C7(std::rc::Rc::new(v15), std::rc::Rc::new(v16))
        }
        Adt19::C8(v4, r3605, r3606) => {
            let v5 = (*r3605).clone();
            let v6 = (*r3606).clone();
            Adt19::C8(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f516(p3610: List<Adt19>) -> bool {
    let m3609 = p3610;
    {
        let mut m3609 = m3609;
        loop {
            let step = {
                let v0 = m3609;
                let m3607 = v0;
                match m3607.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f559(f434(v1.clone()), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
                            if f511(f247({
                                let m3608 = f148(v1, f8(Str::lit("anyOf")));
                                match m3608 {
                                    None => {
                                        Adt19::C6
                                    }
                                    Some(v3) => {
                                        v3
                                    }
                                }
                            })) {
                                Ok::<List<Adt19>, bool>(v2)
                            } else {
                                Err::<List<Adt19>, bool>(false)
                            }
                        } else {
                            Err::<List<Adt19>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3609 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3655 = {
            let a3631 = {
                let a3619 = {
                    let a3613 = {
                        let a3611 = v1;
                        let a3612 = 128u8;
                        fixed_u8::bit_and(a3611, a3612)
                    };
                    let a3614 = 128u8;
                    if equal(a3613, a3614) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3620 = {
                    let a3617 = {
                        let a3615 = v1;
                        let a3616 = 64u8;
                        fixed_u8::bit_and(a3615, a3616)
                    };
                    let a3618 = 64u8;
                    if equal(a3617, a3618) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3619, a3620)?
            };
            let a3632 = {
                let a3629 = {
                    let a3623 = {
                        let a3621 = v1;
                        let a3622 = 32u8;
                        fixed_u8::bit_and(a3621, a3622)
                    };
                    let a3624 = 32u8;
                    if equal(a3623, a3624) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3630 = {
                    let a3627 = {
                        let a3625 = v1;
                        let a3626 = 16u8;
                        fixed_u8::bit_and(a3625, a3626)
                    };
                    let a3628 = 16u8;
                    if equal(a3627, a3628) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3629, a3630)?
            };
            nat_add(a3631, a3632)?
        };
        let a3656 = {
            let a3653 = {
                let a3641 = {
                    let a3635 = {
                        let a3633 = v1;
                        let a3634 = 8u8;
                        fixed_u8::bit_and(a3633, a3634)
                    };
                    let a3636 = 8u8;
                    if equal(a3635, a3636) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3642 = {
                    let a3639 = {
                        let a3637 = v1;
                        let a3638 = 4u8;
                        fixed_u8::bit_and(a3637, a3638)
                    };
                    let a3640 = 4u8;
                    if equal(a3639, a3640) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3641, a3642)?
            };
            let a3654 = {
                let a3651 = {
                    let a3645 = {
                        let a3643 = v1;
                        let a3644 = 2u8;
                        fixed_u8::bit_and(a3643, a3644)
                    };
                    let a3646 = 2u8;
                    if equal(a3645, a3646) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3652 = {
                    let a3649 = {
                        let a3647 = v1;
                        let a3648 = 1u8;
                        fixed_u8::bit_and(a3647, a3648)
                    };
                    let a3650 = 1u8;
                    if equal(a3649, a3650) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3651, a3652)?
            };
            nat_add(a3653, a3654)?
        };
        nat_add(a3655, a3656)?
    };
    let a3663 = f317(f562(), {
        let a3657 = v2;
        let a3658 = 16u64;
        let a3659 = 0u64;
        nat_quot(a3657, a3658, a3659)
    });
    let a3664 = f317(f562(), {
        let a3660 = v2;
        let a3661 = 16u64;
        let a3662 = 0u64;
        nat_rem(a3660, a3661, a3662)
    });
    Ok(append_bytes(a3663, a3664))
}

pub fn f518(v0: Fn21, p3668: List<List<(Bytes, Bytes)>>, p3669: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3667 = (p3668, p3669);
    {
        let mut m3667 = m3667;
        loop {
            let step = {
                let (v1, v2) = m3667;
                let m3665 = v2;
                match m3665.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3666: Fn21 = v0.clone();
                            c3666.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3667 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f519(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f520(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3671 = {
        let a3670 = v0.clone();
        length_list(a3670)
    };
    let a3672 = 1u64;
    if nat_le(a3671, a3672) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f563(v0)?))
    }
}

pub fn f521(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h3673, _) = v0;
        h3673
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h3674) = v0;
        h3674
    };
    let m3675 = v1;
    match m3675.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m3676 = v4;
            match m3676.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f564(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f522(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3677 = f363(v0, v1);
    let a3678 = f491(v0, v2);
    fixed_u32::bit_or(a3677, a3678)
}

pub fn f523(v0: u32) -> u32 {
    let a3681 = {
        let a3679 = f522(v0, 17u32, 15u32);
        let a3680 = f522(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3679, a3680)
    };
    let a3682 = f363(v0, 10u32);
    fixed_u32::bit_xor(a3681, a3682)
}

pub fn f524(v0: u32) -> u32 {
    let a3685 = {
        let a3683 = f522(v0, 7u32, 25u32);
        let a3684 = f522(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3683, a3684)
    };
    let a3686 = f363(v0, 3u32);
    fixed_u32::bit_xor(a3685, a3686)
}

pub fn f525() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x00]), List::cons(Bytes::lit(&[0x01]), List::cons(Bytes::lit(&[0x02]), List::cons(Bytes::lit(&[0x03]), List::cons(Bytes::lit(&[0x04]), List::cons(Bytes::lit(&[0x05]), List::cons(Bytes::lit(&[0x06]), List::cons(Bytes::lit(&[0x07]), List::cons(Bytes::lit(&[0x08]), List::cons(Bytes::lit(&[0x09]), List::cons(Bytes::lit(&[0x0a]), List::cons(Bytes::lit(&[0x0b]), List::cons(Bytes::lit(&[0x0c]), List::cons(Bytes::lit(&[0x0d]), List::cons(Bytes::lit(&[0x0e]), List::cons(Bytes::lit(&[0x0f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f526() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x10]), List::cons(Bytes::lit(&[0x11]), List::cons(Bytes::lit(&[0x12]), List::cons(Bytes::lit(&[0x13]), List::cons(Bytes::lit(&[0x14]), List::cons(Bytes::lit(&[0x15]), List::cons(Bytes::lit(&[0x16]), List::cons(Bytes::lit(&[0x17]), List::cons(Bytes::lit(&[0x18]), List::cons(Bytes::lit(&[0x19]), List::cons(Bytes::lit(&[0x1a]), List::cons(Bytes::lit(&[0x1b]), List::cons(Bytes::lit(&[0x1c]), List::cons(Bytes::lit(&[0x1d]), List::cons(Bytes::lit(&[0x1e]), List::cons(Bytes::lit(&[0x1f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f527() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x20]), List::cons(Bytes::lit(&[0x21]), List::cons(Bytes::lit(&[0x22]), List::cons(Bytes::lit(&[0x23]), List::cons(Bytes::lit(&[0x24]), List::cons(Bytes::lit(&[0x25]), List::cons(Bytes::lit(&[0x26]), List::cons(Bytes::lit(&[0x27]), List::cons(Bytes::lit(&[0x28]), List::cons(Bytes::lit(&[0x29]), List::cons(Bytes::lit(&[0x2a]), List::cons(Bytes::lit(&[0x2b]), List::cons(Bytes::lit(&[0x2c]), List::cons(Bytes::lit(&[0x2d]), List::cons(Bytes::lit(&[0x2e]), List::cons(Bytes::lit(&[0x2f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f528() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x30]), List::cons(Bytes::lit(&[0x31]), List::cons(Bytes::lit(&[0x32]), List::cons(Bytes::lit(&[0x33]), List::cons(Bytes::lit(&[0x34]), List::cons(Bytes::lit(&[0x35]), List::cons(Bytes::lit(&[0x36]), List::cons(Bytes::lit(&[0x37]), List::cons(Bytes::lit(&[0x38]), List::cons(Bytes::lit(&[0x39]), List::cons(Bytes::lit(&[0x3a]), List::cons(Bytes::lit(&[0x3b]), List::cons(Bytes::lit(&[0x3c]), List::cons(Bytes::lit(&[0x3d]), List::cons(Bytes::lit(&[0x3e]), List::cons(Bytes::lit(&[0x3f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f529() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x40]), List::cons(Bytes::lit(&[0x41]), List::cons(Bytes::lit(&[0x42]), List::cons(Bytes::lit(&[0x43]), List::cons(Bytes::lit(&[0x44]), List::cons(Bytes::lit(&[0x45]), List::cons(Bytes::lit(&[0x46]), List::cons(Bytes::lit(&[0x47]), List::cons(Bytes::lit(&[0x48]), List::cons(Bytes::lit(&[0x49]), List::cons(Bytes::lit(&[0x4a]), List::cons(Bytes::lit(&[0x4b]), List::cons(Bytes::lit(&[0x4c]), List::cons(Bytes::lit(&[0x4d]), List::cons(Bytes::lit(&[0x4e]), List::cons(Bytes::lit(&[0x4f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f530() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x50]), List::cons(Bytes::lit(&[0x51]), List::cons(Bytes::lit(&[0x52]), List::cons(Bytes::lit(&[0x53]), List::cons(Bytes::lit(&[0x54]), List::cons(Bytes::lit(&[0x55]), List::cons(Bytes::lit(&[0x56]), List::cons(Bytes::lit(&[0x57]), List::cons(Bytes::lit(&[0x58]), List::cons(Bytes::lit(&[0x59]), List::cons(Bytes::lit(&[0x5a]), List::cons(Bytes::lit(&[0x5b]), List::cons(Bytes::lit(&[0x5c]), List::cons(Bytes::lit(&[0x5d]), List::cons(Bytes::lit(&[0x5e]), List::cons(Bytes::lit(&[0x5f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f531() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x60]), List::cons(Bytes::lit(&[0x61]), List::cons(Bytes::lit(&[0x62]), List::cons(Bytes::lit(&[0x63]), List::cons(Bytes::lit(&[0x64]), List::cons(Bytes::lit(&[0x65]), List::cons(Bytes::lit(&[0x66]), List::cons(Bytes::lit(&[0x67]), List::cons(Bytes::lit(&[0x68]), List::cons(Bytes::lit(&[0x69]), List::cons(Bytes::lit(&[0x6a]), List::cons(Bytes::lit(&[0x6b]), List::cons(Bytes::lit(&[0x6c]), List::cons(Bytes::lit(&[0x6d]), List::cons(Bytes::lit(&[0x6e]), List::cons(Bytes::lit(&[0x6f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f532() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x70]), List::cons(Bytes::lit(&[0x71]), List::cons(Bytes::lit(&[0x72]), List::cons(Bytes::lit(&[0x73]), List::cons(Bytes::lit(&[0x74]), List::cons(Bytes::lit(&[0x75]), List::cons(Bytes::lit(&[0x76]), List::cons(Bytes::lit(&[0x77]), List::cons(Bytes::lit(&[0x78]), List::cons(Bytes::lit(&[0x79]), List::cons(Bytes::lit(&[0x7a]), List::cons(Bytes::lit(&[0x7b]), List::cons(Bytes::lit(&[0x7c]), List::cons(Bytes::lit(&[0x7d]), List::cons(Bytes::lit(&[0x7e]), List::cons(Bytes::lit(&[0x7f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f533() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x80]), List::cons(Bytes::lit(&[0x81]), List::cons(Bytes::lit(&[0x82]), List::cons(Bytes::lit(&[0x83]), List::cons(Bytes::lit(&[0x84]), List::cons(Bytes::lit(&[0x85]), List::cons(Bytes::lit(&[0x86]), List::cons(Bytes::lit(&[0x87]), List::cons(Bytes::lit(&[0x88]), List::cons(Bytes::lit(&[0x89]), List::cons(Bytes::lit(&[0x8a]), List::cons(Bytes::lit(&[0x8b]), List::cons(Bytes::lit(&[0x8c]), List::cons(Bytes::lit(&[0x8d]), List::cons(Bytes::lit(&[0x8e]), List::cons(Bytes::lit(&[0x8f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f534() -> List<Bytes> {
    List::cons(Bytes::lit(&[0x90]), List::cons(Bytes::lit(&[0x91]), List::cons(Bytes::lit(&[0x92]), List::cons(Bytes::lit(&[0x93]), List::cons(Bytes::lit(&[0x94]), List::cons(Bytes::lit(&[0x95]), List::cons(Bytes::lit(&[0x96]), List::cons(Bytes::lit(&[0x97]), List::cons(Bytes::lit(&[0x98]), List::cons(Bytes::lit(&[0x99]), List::cons(Bytes::lit(&[0x9a]), List::cons(Bytes::lit(&[0x9b]), List::cons(Bytes::lit(&[0x9c]), List::cons(Bytes::lit(&[0x9d]), List::cons(Bytes::lit(&[0x9e]), List::cons(Bytes::lit(&[0x9f]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f535() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xa0]), List::cons(Bytes::lit(&[0xa1]), List::cons(Bytes::lit(&[0xa2]), List::cons(Bytes::lit(&[0xa3]), List::cons(Bytes::lit(&[0xa4]), List::cons(Bytes::lit(&[0xa5]), List::cons(Bytes::lit(&[0xa6]), List::cons(Bytes::lit(&[0xa7]), List::cons(Bytes::lit(&[0xa8]), List::cons(Bytes::lit(&[0xa9]), List::cons(Bytes::lit(&[0xaa]), List::cons(Bytes::lit(&[0xab]), List::cons(Bytes::lit(&[0xac]), List::cons(Bytes::lit(&[0xad]), List::cons(Bytes::lit(&[0xae]), List::cons(Bytes::lit(&[0xaf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f536() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xb0]), List::cons(Bytes::lit(&[0xb1]), List::cons(Bytes::lit(&[0xb2]), List::cons(Bytes::lit(&[0xb3]), List::cons(Bytes::lit(&[0xb4]), List::cons(Bytes::lit(&[0xb5]), List::cons(Bytes::lit(&[0xb6]), List::cons(Bytes::lit(&[0xb7]), List::cons(Bytes::lit(&[0xb8]), List::cons(Bytes::lit(&[0xb9]), List::cons(Bytes::lit(&[0xba]), List::cons(Bytes::lit(&[0xbb]), List::cons(Bytes::lit(&[0xbc]), List::cons(Bytes::lit(&[0xbd]), List::cons(Bytes::lit(&[0xbe]), List::cons(Bytes::lit(&[0xbf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f537() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xc0]), List::cons(Bytes::lit(&[0xc1]), List::cons(Bytes::lit(&[0xc2]), List::cons(Bytes::lit(&[0xc3]), List::cons(Bytes::lit(&[0xc4]), List::cons(Bytes::lit(&[0xc5]), List::cons(Bytes::lit(&[0xc6]), List::cons(Bytes::lit(&[0xc7]), List::cons(Bytes::lit(&[0xc8]), List::cons(Bytes::lit(&[0xc9]), List::cons(Bytes::lit(&[0xca]), List::cons(Bytes::lit(&[0xcb]), List::cons(Bytes::lit(&[0xcc]), List::cons(Bytes::lit(&[0xcd]), List::cons(Bytes::lit(&[0xce]), List::cons(Bytes::lit(&[0xcf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f538() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xd0]), List::cons(Bytes::lit(&[0xd1]), List::cons(Bytes::lit(&[0xd2]), List::cons(Bytes::lit(&[0xd3]), List::cons(Bytes::lit(&[0xd4]), List::cons(Bytes::lit(&[0xd5]), List::cons(Bytes::lit(&[0xd6]), List::cons(Bytes::lit(&[0xd7]), List::cons(Bytes::lit(&[0xd8]), List::cons(Bytes::lit(&[0xd9]), List::cons(Bytes::lit(&[0xda]), List::cons(Bytes::lit(&[0xdb]), List::cons(Bytes::lit(&[0xdc]), List::cons(Bytes::lit(&[0xdd]), List::cons(Bytes::lit(&[0xde]), List::cons(Bytes::lit(&[0xdf]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f539() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xe0]), List::cons(Bytes::lit(&[0xe1]), List::cons(Bytes::lit(&[0xe2]), List::cons(Bytes::lit(&[0xe3]), List::cons(Bytes::lit(&[0xe4]), List::cons(Bytes::lit(&[0xe5]), List::cons(Bytes::lit(&[0xe6]), List::cons(Bytes::lit(&[0xe7]), List::cons(Bytes::lit(&[0xe8]), List::cons(Bytes::lit(&[0xe9]), List::cons(Bytes::lit(&[0xea]), List::cons(Bytes::lit(&[0xeb]), List::cons(Bytes::lit(&[0xec]), List::cons(Bytes::lit(&[0xed]), List::cons(Bytes::lit(&[0xee]), List::cons(Bytes::lit(&[0xef]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f540() -> List<Bytes> {
    List::cons(Bytes::lit(&[0xf0]), List::cons(Bytes::lit(&[0xf1]), List::cons(Bytes::lit(&[0xf2]), List::cons(Bytes::lit(&[0xf3]), List::cons(Bytes::lit(&[0xf4]), List::cons(Bytes::lit(&[0xf5]), List::cons(Bytes::lit(&[0xf6]), List::cons(Bytes::lit(&[0xf7]), List::cons(Bytes::lit(&[0xf8]), List::cons(Bytes::lit(&[0xf9]), List::cons(Bytes::lit(&[0xfa]), List::cons(Bytes::lit(&[0xfb]), List::cons(Bytes::lit(&[0xfc]), List::cons(Bytes::lit(&[0xfd]), List::cons(Bytes::lit(&[0xfe]), List::cons(Bytes::lit(&[0xff]), List::<Bytes>::nil()))))))))))))))))
}

pub fn f541(v0: Bytes, v1: u64) -> R<Bytes> {
    let a3689 = f32(v0.clone(), {
        let a3687 = v1;
        let a3688 = 2u64;
        nat_add(a3687, a3688)?
    }, 2u64);
    let a3690 = Bytes::lit(&[0x30, 0x30]);
    if equal(a3689, a3690) {
        let m3693 = f565(f32(v0, {
            let a3691 = v1;
            let a3692 = 4u64;
            nat_add(a3691, a3692)?
        }, 2u64))?;
        match m3693 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f313(v2.clone(), 0u64)?;
                let a3694 = v3;
                let a3695 = 32u64;
                if nat_lt(a3694, a3695) {
                    if f566(v3) {
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

pub fn f542(v0: u64) -> Bytes {
    let a3696 = v0;
    let a3697 = 34u64;
    if nat_eq(a3696, a3697) {
        Bytes::lit(&[0x22])
    } else {
        let a3698 = v0;
        let a3699 = 92u64;
        if nat_eq(a3698, a3699) {
            Bytes::lit(&[0x5c])
        } else {
            let a3700 = v0;
            let a3701 = 98u64;
            if nat_eq(a3700, a3701) {
                Bytes::lit(&[0x08])
            } else {
                let a3702 = v0;
                let a3703 = 102u64;
                if nat_eq(a3702, a3703) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a3704 = v0;
                    let a3705 = 110u64;
                    if nat_eq(a3704, a3705) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a3706 = v0;
                        let a3707 = 114u64;
                        if nat_eq(a3706, a3707) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a3708 = v0;
                            let a3709 = 116u64;
                            if nat_eq(a3708, a3709) {
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

pub fn f543(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3712 = {
        let a3710 = v0;
        let a3711 = v1;
        index_bytes(a3710, a3711)
    };
    match m3712 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f567(v2)? {
                Ok(Some({
                    let a3713 = v1;
                    let a3714 = 1u64;
                    nat_add(a3713, a3714)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f544(v0: u8) -> R<u64> {
    let a3761 = {
        let v1: u8 = v0;
        let a3759 = {
            let a3735 = {
                let a3723 = {
                    let a3717 = {
                        let a3715 = v1;
                        let a3716 = 128u8;
                        fixed_u8::bit_and(a3715, a3716)
                    };
                    let a3718 = 128u8;
                    if equal(a3717, a3718) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3724 = {
                    let a3721 = {
                        let a3719 = v1;
                        let a3720 = 64u8;
                        fixed_u8::bit_and(a3719, a3720)
                    };
                    let a3722 = 64u8;
                    if equal(a3721, a3722) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3723, a3724)?
            };
            let a3736 = {
                let a3733 = {
                    let a3727 = {
                        let a3725 = v1;
                        let a3726 = 32u8;
                        fixed_u8::bit_and(a3725, a3726)
                    };
                    let a3728 = 32u8;
                    if equal(a3727, a3728) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3734 = {
                    let a3731 = {
                        let a3729 = v1;
                        let a3730 = 16u8;
                        fixed_u8::bit_and(a3729, a3730)
                    };
                    let a3732 = 16u8;
                    if equal(a3731, a3732) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3733, a3734)?
            };
            nat_add(a3735, a3736)?
        };
        let a3760 = {
            let a3757 = {
                let a3745 = {
                    let a3739 = {
                        let a3737 = v1;
                        let a3738 = 8u8;
                        fixed_u8::bit_and(a3737, a3738)
                    };
                    let a3740 = 8u8;
                    if equal(a3739, a3740) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3746 = {
                    let a3743 = {
                        let a3741 = v1;
                        let a3742 = 4u8;
                        fixed_u8::bit_and(a3741, a3742)
                    };
                    let a3744 = 4u8;
                    if equal(a3743, a3744) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3745, a3746)?
            };
            let a3758 = {
                let a3755 = {
                    let a3749 = {
                        let a3747 = v1;
                        let a3748 = 2u8;
                        fixed_u8::bit_and(a3747, a3748)
                    };
                    let a3750 = 2u8;
                    if equal(a3749, a3750) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3756 = {
                    let a3753 = {
                        let a3751 = v1;
                        let a3752 = 1u8;
                        fixed_u8::bit_and(a3751, a3752)
                    };
                    let a3754 = 1u8;
                    if equal(a3753, a3754) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3755, a3756)?
            };
            nat_add(a3757, a3758)?
        };
        nat_add(a3759, a3760)?
    };
    let a3762 = 48u64;
    Ok(nat_sub(a3761, a3762))
}

pub fn f545(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3763 = v0.clone();
        length_bytes(a3763)
    };
    let a3764 = v1;
    let a3765 = v2;
    if if nat_lt(a3764, a3765) {
        let a3770 = f313(v0, {
            let a3768 = v2;
            let a3769 = {
                let a3766 = v1;
                let a3767 = 1u64;
                nat_add(a3766, a3767)?
            };
            nat_sub(a3768, a3769)
        })?;
        let a3771 = 48u64;
        nat_eq(a3770, a3771)
    } else {
        false
    } {
        Ok(Some({
            let a3772 = v1;
            let a3773 = 1u64;
            nat_add(a3772, a3773)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f546(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3774 = v1;
    match m3774.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3775 = f568(v0, v3.clone())?;
            match m3775 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f569(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f86(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3776 = f546(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3776 {
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

pub fn f547(p3780: List<Bytes>) -> R<bool> {
    let m3779 = p3780;
    {
        let mut m3779 = m3779;
        loop {
            let step = {
                let v0 = m3779;
                let m3777 = v0;
                match m3777.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m3778 = f570(v1)?;
                        match m3778 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f86(v3, f571(false, v2.clone())?) {
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
                Ok(next) => { m3779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f548(v0: List<(Bytes, Adt19)>, v1: List<Adt19>) -> List<Bytes> {
    let m3781 = v0;
    match m3781.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f572(v1.clone(), {
                let (h3782, _) = v2.clone();
                h3782
            }) {
                List::cons({
                    let (h3783, _) = v2;
                    h3783
                }, f548(v3, v1))
            } else {
                f548(v3, v1)
            }
        }
    }
}

pub fn f549(v0: bool, v1: Bytes, v2: List<Adt19>, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m3784 = v2;
    match m3784.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v4, v5)) => {
            let m3785 = f573(v4.clone())?;
            match m3785 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m3786 = f574(v0, v6.clone())?;
                    match m3786 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f86(v7.clone(), v3.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3789 = f575(v4, {
                                    let a3787 = v1.clone();
                                    let a3788 = f242(v6)?;
                                    append_bytes(a3787, a3788)
                                })?;
                                match m3789 {
                                    Ok(v9) => {
                                        let m3790 = f549(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m3790 {
                                            Ok(v11) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons({
                                                    let (h3791, _) = v9;
                                                    h3791
                                                }, List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h3792, _) = v11;
                                                    h3792
                                                }), f393({
                                                    let (_, h3793) = v9;
                                                    h3793
                                                }, {
                                                    let (_, h3794) = v11;
                                                    h3794
                                                }))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v8))
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

pub fn f550(v0: List<Adt19>) -> R<List<Bytes>> {
    let m3795 = v0;
    match m3795.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m3796 = f573(v1)?;
            match m3796 {
                None => {
                    f550(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f550(v2)?))
                }
            }
        }
    }
}

pub fn f551(v0: Bytes, v1: List<Adt19>, v2: Bytes, v3: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    f576(f547(f577(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f552(v0: Adt19, v1: Bytes) -> R<bool> {
    let m3797 = f246(Some(v0));
    match m3797 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3798 = f502(v2)?;
            match m3798 {
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

pub fn f553(v0: bool, v1: Bytes, v2: List<(Bytes, Adt19)>, v3: List<Bytes>, v4: bool) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m3799 = v2;
    match m3799.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3801 = f574(v0, {
                let (h3800, _) = v5.clone();
                h3800
            })?;
            match m3801 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h3802, _) = v5;
                        h3802
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f86(v7.clone(), f571(v0, f554(v6.clone()))?) {
                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m3807 = f510({
                            let (_, h3803) = v5.clone();
                            h3803
                        }, {
                            let a3805 = v1.clone();
                            let a3806 = f242({
                                let (h3804, _) = v5.clone();
                                h3804
                            })?;
                            append_bytes(a3805, a3806)
                        })?;
                        match m3807 {
                            Ok(v9) => {
                                let v10: Bytes = if f86({
                                    let (h3808, _) = v5;
                                    h3808
                                }, v3.clone()) {
                                    let (h3809, _) = v9;
                                    h3809
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                        let (h3810, _) = v9;
                                        h3810
                                    }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m3811 = f553(v0, v1, v6, v3, v4)?;
                                match m3811 {
                                    Ok(v12) => {
                                        Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, {
                                            let (h3812, _) = v12;
                                            h3812
                                        }), f393({
                                            let (_, h3813) = v9;
                                            h3813
                                        }, {
                                            let (_, h3814) = v12;
                                            h3814
                                        }))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f554(v0: List<(Bytes, Adt19)>) -> List<Bytes> {
    let m3815 = v0;
    match m3815.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h3816, _) = v1;
                h3816
            }, f554(v2))
        }
    }
}

pub fn f555(v0: Adt19) -> Option<Adt19> {
    let m3817 = f153(f148(v0, f8(Str::lit("oneOf"))));
    match m3817.uncons() {
        None => {
            None::<Adt19>
        }
        Some((v1, v2)) => {
            let m3818 = v2;
            match m3818.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v3, v4)) => {
                    let m3819 = v4;
                    match m3819.uncons() {
                        None => {
                            if f578(v3.clone()) {
                                Some(v1)
                            } else {
                                if f578(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt19>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt19>
                        }
                    }
                }
            }
        }
    }
}

pub fn f556(v0: Bytes, v1: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m3820 = v1;
    match m3820 {
        Ok(v3) => {
            f505(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h3821, _) = v3;
                h3821
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h3822) = v3;
                h3822
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v2)
        }
    }
}

pub fn f557(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m3823 = f555(v0.clone());
    match m3823 {
        None => {
            if f36(f151(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m3824 = f148(v0, f8(Str::lit("items")));
                match m3824 {
                    None => {
                        Ok(f556(f8(Str::lit("List")), f558()))
                    }
                    Some(v3) => {
                        Ok(f556(f8(Str::lit("List")), f575(v3, {
                            let a3825 = v1;
                            let a3826 = f8(Str::lit("Item"));
                            append_bytes(a3825, a3826)
                        })?))
                    }
                }
            } else {
                f575(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f556(f8(Str::lit("Option")), f575(v2, v1)?))
        }
    }
}

pub fn f558() -> Result<(Bytes, List<Adt25>), Bytes> {
    Ok::<(Bytes, List<Adt25>), Bytes>((f437(), List::cons(Adt25::C2, List::<Adt25>::nil())))
}

pub fn f559(p3830: List<(Bytes, Adt19)>, v1: List<Bytes>) -> bool {
    let m3829 = p3830;
    {
        let mut m3829 = m3829;
        loop {
            let step = {
                let v0 = m3829;
                let m3827 = v0;
                match m3827.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f86({
                            let (h3828, _) = v2;
                            h3828
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt19)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt19)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3829 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f560(p3834: List<Adt19>, v1: Bytes, p3835: List<Bytes>) -> bool {
    let m3833 = (p3834, p3835);
    {
        let mut m3833 = m3833;
        loop {
            let step = {
                let (v0, v2) = m3833;
                let m3831 = v0;
                match m3831.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m3832 = f579(v3, v1.clone());
                        match m3832 {
                            None => {
                                Err::<(List<Adt19>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f86(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt19>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt19>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f561(p3838: List<Adt19>, p3839: List<Bytes>) -> bool {
    let m3837 = (p3838, p3839);
    {
        let mut m3837 = m3837;
        loop {
            let step = {
                let (v0, v1) = m3837;
                let m3836 = v0;
                match m3836.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f151(v2, f8(Str::lit("type")));
                        if !f36(v4.clone(), Bytes::lit(&[])) {
                            if !f86(v4.clone(), v1.clone()) {
                                if !if if f36(v4.clone(), f8(Str::lit("integer"))) {
                                    f86(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f36(v4.clone(), f8(Str::lit("number"))) {
                                        f86(f8(Str::lit("integer")), v1.clone())
                                    } else {
                                        false
                                    }
                                } {
                                    Ok::<(List<Adt19>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt19>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt19>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt19>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3837 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f562() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f563(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f476({
        let (_, h3840) = {
            let (h3841, _) = f580(Fn22::F581, {
                let a3843 = {
                    let a3842 = v0.clone();
                    length_list(a3842)
                };
                let a3844 = 1u64;
                nat_add(a3843, a3844)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3841
        };
        h3840
    }))
}

pub fn f564(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3849 = {
            let a3847 = {
                let a3845 = v0.clone();
                length_list(a3845)
            };
            let a3848 = {
                let a3846 = v1.clone();
                length_list(a3846)
            };
            nat_add(a3847, a3848)?
        };
        let a3850 = 1u64;
        nat_add(a3849, a3850)?
    };
    Ok(f353({
        let (_, h3851) = {
            let (_, h3852) = {
                let (h3853, _) = f582(Fn23::F583, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3853
            };
            h3852
        };
        h3851
    }))
}

pub fn f565(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3854 = v0.clone();
        length_bytes(a3854)
    };
    let a3858 = {
        let a3855 = v1;
        let a3856 = 2u64;
        let a3857 = 0u64;
        nat_rem(a3855, a3856, a3857)
    };
    let a3859 = 0u64;
    if nat_eq(a3858, a3859) {
        let (_, h3860) = {
            let (h3861, _) = f584(Fn24::F585(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3861
        };
        Ok(h3860)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f566(v0: u64) -> bool {
    let a3862 = v0;
    let a3863 = 8u64;
    if nat_eq(a3862, a3863) {
        true
    } else {
        let a3864 = v0;
        let a3865 = 9u64;
        if nat_eq(a3864, a3865) {
            true
        } else {
            let a3866 = v0;
            let a3867 = 10u64;
            if nat_eq(a3866, a3867) {
                true
            } else {
                let a3868 = v0;
                let a3869 = 12u64;
                if nat_eq(a3868, a3869) {
                    true
                } else {
                    let a3870 = v0;
                    let a3871 = 13u64;
                    nat_eq(a3870, a3871)
                }
            }
        }
    }
}

pub fn f567(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3916 = {
            let a3892 = {
                let a3880 = {
                    let a3874 = {
                        let a3872 = v1;
                        let a3873 = 128u8;
                        fixed_u8::bit_and(a3872, a3873)
                    };
                    let a3875 = 128u8;
                    if equal(a3874, a3875) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3881 = {
                    let a3878 = {
                        let a3876 = v1;
                        let a3877 = 64u8;
                        fixed_u8::bit_and(a3876, a3877)
                    };
                    let a3879 = 64u8;
                    if equal(a3878, a3879) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3880, a3881)?
            };
            let a3893 = {
                let a3890 = {
                    let a3884 = {
                        let a3882 = v1;
                        let a3883 = 32u8;
                        fixed_u8::bit_and(a3882, a3883)
                    };
                    let a3885 = 32u8;
                    if equal(a3884, a3885) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3891 = {
                    let a3888 = {
                        let a3886 = v1;
                        let a3887 = 16u8;
                        fixed_u8::bit_and(a3886, a3887)
                    };
                    let a3889 = 16u8;
                    if equal(a3888, a3889) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3890, a3891)?
            };
            nat_add(a3892, a3893)?
        };
        let a3917 = {
            let a3914 = {
                let a3902 = {
                    let a3896 = {
                        let a3894 = v1;
                        let a3895 = 8u8;
                        fixed_u8::bit_and(a3894, a3895)
                    };
                    let a3897 = 8u8;
                    if equal(a3896, a3897) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3903 = {
                    let a3900 = {
                        let a3898 = v1;
                        let a3899 = 4u8;
                        fixed_u8::bit_and(a3898, a3899)
                    };
                    let a3901 = 4u8;
                    if equal(a3900, a3901) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3902, a3903)?
            };
            let a3915 = {
                let a3912 = {
                    let a3906 = {
                        let a3904 = v1;
                        let a3905 = 2u8;
                        fixed_u8::bit_and(a3904, a3905)
                    };
                    let a3907 = 2u8;
                    if equal(a3906, a3907) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3913 = {
                    let a3910 = {
                        let a3908 = v1;
                        let a3909 = 1u8;
                        fixed_u8::bit_and(a3908, a3909)
                    };
                    let a3911 = 1u8;
                    if equal(a3910, a3911) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3912, a3913)?
            };
            nat_add(a3914, a3915)?
        };
        nat_add(a3916, a3917)?
    };
    let a3918 = 48u64;
    let a3919 = v2;
    if nat_le(a3918, a3919) {
        let a3920 = v2;
        let a3921 = 57u64;
        Ok(nat_le(a3920, a3921))
    } else {
        Ok(false)
    }
}

pub fn f568(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m3922 = v1;
    match m3922 {
        Adt19::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C1(v7, v8) => {
            Ok(Some({
                let a3933 = if if v7 {
                    let a3931 = 0u64;
                    let a3932 = v8;
                    nat_lt(a3931, a3932)
                } else {
                    false
                } {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3934 = f332(false, v8)?;
                append_bytes(a3933, a3934)
            }))
        }
        Adt19::C2(v3, v4, v5) => {
            let m3925 = f586({
                let a3923 = f8(Str::lit("n"));
                let a3924 = f333(false, v4, v5)?;
                append_bytes(a3923, a3924)
            })?;
            match m3925 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3929 = f8(Str::lit("nMinus"));
                        let a3930 = f32(v6.clone(), 1u64, {
                            let a3927 = {
                                let a3926 = v6;
                                length_bytes(a3926)
                            };
                            let a3928 = 1u64;
                            nat_sub(a3927, a3928)
                        });
                        append_bytes(a3929, a3930)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt19::C3(v2) => {
            f574(v0, v2)
        }
        Adt19::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C6 => {
            Ok(None::<Bytes>)
        }
        Adt19::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt19::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f569(v0: Adt19) -> R<Bytes> {
    f245(v0)
}

pub fn f570(v0: Bytes) -> R<Option<Bytes>> {
    if f587({
        let a3935 = v0.clone();
        length_bytes(a3935)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f588(v0)?;
        if f589(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3938 = {
                let a3936 = v1.clone();
                let a3937 = 0u64;
                index_bytes(a3936, a3937)
            };
            match m3938 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f387({
                        let v3: u8 = v2;
                        let a3983 = {
                            let a3959 = {
                                let a3947 = {
                                    let a3941 = {
                                        let a3939 = v3;
                                        let a3940 = 128u8;
                                        fixed_u8::bit_and(a3939, a3940)
                                    };
                                    let a3942 = 128u8;
                                    if equal(a3941, a3942) {
                                        128u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3948 = {
                                    let a3945 = {
                                        let a3943 = v3;
                                        let a3944 = 64u8;
                                        fixed_u8::bit_and(a3943, a3944)
                                    };
                                    let a3946 = 64u8;
                                    if equal(a3945, a3946) {
                                        64u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3947, a3948)?
                            };
                            let a3960 = {
                                let a3957 = {
                                    let a3951 = {
                                        let a3949 = v3;
                                        let a3950 = 32u8;
                                        fixed_u8::bit_and(a3949, a3950)
                                    };
                                    let a3952 = 32u8;
                                    if equal(a3951, a3952) {
                                        32u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3958 = {
                                    let a3955 = {
                                        let a3953 = v3;
                                        let a3954 = 16u8;
                                        fixed_u8::bit_and(a3953, a3954)
                                    };
                                    let a3956 = 16u8;
                                    if equal(a3955, a3956) {
                                        16u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3957, a3958)?
                            };
                            nat_add(a3959, a3960)?
                        };
                        let a3984 = {
                            let a3981 = {
                                let a3969 = {
                                    let a3963 = {
                                        let a3961 = v3;
                                        let a3962 = 8u8;
                                        fixed_u8::bit_and(a3961, a3962)
                                    };
                                    let a3964 = 8u8;
                                    if equal(a3963, a3964) {
                                        8u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3970 = {
                                    let a3967 = {
                                        let a3965 = v3;
                                        let a3966 = 4u8;
                                        fixed_u8::bit_and(a3965, a3966)
                                    };
                                    let a3968 = 4u8;
                                    if equal(a3967, a3968) {
                                        4u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3969, a3970)?
                            };
                            let a3982 = {
                                let a3979 = {
                                    let a3973 = {
                                        let a3971 = v3;
                                        let a3972 = 2u8;
                                        fixed_u8::bit_and(a3971, a3972)
                                    };
                                    let a3974 = 2u8;
                                    if equal(a3973, a3974) {
                                        2u64
                                    } else {
                                        0u64
                                    }
                                };
                                let a3980 = {
                                    let a3977 = {
                                        let a3975 = v3;
                                        let a3976 = 1u8;
                                        fixed_u8::bit_and(a3975, a3976)
                                    };
                                    let a3978 = 1u8;
                                    if equal(a3977, a3978) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                };
                                nat_add(a3979, a3980)?
                            };
                            nat_add(a3981, a3982)?
                        };
                        nat_add(a3983, a3984)?
                    }) {
                        Ok(Some({
                            let a3985 = f8(Str::lit("n"));
                            let a3986 = v1;
                            append_bytes(a3985, a3986)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f590({
            let a3987 = v0.clone();
            length_bytes(a3987)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f571(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3988 = v1;
    match m3988.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3989 = f574(v0, v2)?;
            match m3989 {
                None => {
                    f571(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f571(v0, v3)?))
                }
            }
        }
    }
}

pub fn f572(p3993: List<Adt19>, v1: Bytes) -> bool {
    let m3992 = p3993;
    {
        let mut m3992 = m3992;
        loop {
            let step = {
                let v0 = m3992;
                let m3990 = v0;
                match m3990.uncons() {
                    None => {
                        Err::<List<Adt19>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3991 = f579(v2, v1.clone());
                        if match m3991 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt19>, bool>(v3)
                        } else {
                            Err::<List<Adt19>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3992 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(v0: Adt19) -> R<Option<Bytes>> {
    let m3994 = f246(f148(v0.clone(), f8(Str::lit("$ref"))));
    match m3994 {
        None => {
            Ok(f246(f148(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f502(v1)
        }
    }
}

pub fn f574(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f586(v1)
    } else {
        f570(v1)
    }
}

pub fn f575(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f591(f474(v0), v1)
}

pub fn f576(v0: bool, v1: Bytes, v2: List<Adt19>, v3: Bytes, v4: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt25>), Bytes>> {
    let m3995 = v2;
    match m3995.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::<Bytes>::nil(), List::<Adt25>::nil())))
        }
        Some((v5, v6)) => {
            let m3996 = f579(v5.clone(), v3.clone());
            match m3996 {
                None => {
                    Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3997 = f574(v0, v7.clone())?;
                    match m3997 {
                        None => {
                            Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f86(v8.clone(), v4.clone()) {
                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m4000 = f508({
                                    let a3998 = v1.clone();
                                    let a3999 = f242(v7)?;
                                    append_bytes(a3998, a3999)
                                }, f436(f389(f148(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f152(f153(f148(v5, f8(Str::lit("required"))))), true)?;
                                match m4000 {
                                    Ok(v10) => {
                                        let m4001 = f576(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m4001 {
                                            Ok(v12) => {
                                                Ok(Ok::<(List<Bytes>, List<Adt25>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f215({
                                                    let (h4002, _) = v10;
                                                    h4002
                                                }, f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), {
                                                    let (h4003, _) = v12;
                                                    h4003
                                                }), f393({
                                                    let (_, h4004) = v10;
                                                    h4004
                                                }, {
                                                    let (_, h4005) = v12;
                                                    h4005
                                                }))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(List<Bytes>, List<Adt25>), Bytes>(v9))
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

pub fn f577(v0: List<Adt19>, v1: Bytes) -> List<Bytes> {
    let m4006 = v0;
    match m4006.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4007 = f579(v2, v1.clone());
            match m4007 {
                None => {
                    f577(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f577(v3, v1))
                }
            }
        }
    }
}

pub fn f578(v0: Adt19) -> bool {
    f36(f151(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f579(v0: Adt19, v1: Bytes) -> Option<Bytes> {
    if f86(v1.clone(), f152(f153(f148(v0.clone(), f8(Str::lit("required")))))) {
        let m4008 = f148(v0, f8(Str::lit("properties")));
        match m4008 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m4009 = f148(v2, v1);
                match m4009 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f246(f148(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f580(v0: Fn22, p4014: u64, p4015: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4013 = (p4014, p4015);
    {
        let mut m4013 = m4013;
        loop {
            let step = {
                let (v1, v2) = m4013;
                let m4010 = v1;
                if m4010 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4010 - 1;
                    let m4012 = {
                        let c4011: Fn22 = v0.clone();
                        c4011.apply(v2.clone())?
                    };
                    match m4012 {
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
                Ok(next) => { m4013 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f592(v0)
}

pub fn f582(v0: Fn23, p4020: u64, p4021: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m4019 = (p4020, p4021);
    {
        let mut m4019 = m4019;
        loop {
            let step = {
                let (v1, v2) = m4019;
                let m4016 = v1;
                if m4016 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m4016 - 1;
                    let m4018 = {
                        let c4017: Fn23 = v0.clone();
                        c4017.apply(v2.clone())
                    };
                    match m4018 {
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
                Ok(next) => { m4019 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f593(v0)
}

pub fn f584(v0: Fn24, p4026: u64, p4027: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m4025 = (p4026, p4027);
    {
        let mut m4025 = m4025;
        loop {
            let step = {
                let (v1, v2) = m4025;
                let m4022 = v1;
                if m4022 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4022 - 1;
                    let m4024 = {
                        let c4023: Fn24 = v0.clone();
                        c4023.apply(v2.clone())?
                    };
                    match m4024 {
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
                Ok(next) => { m4025 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f585(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f594(v0, v1)
}

pub fn f586(v0: Bytes) -> R<Option<Bytes>> {
    let m4029 = f595({
        let a4028 = v0.clone();
        length_bytes(a4028)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m4029 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a4031 = {
                let a4030 = v1.clone();
                length_bytes(a4030)
            };
            let a4032 = 0u64;
            if nat_eq(a4031, a4032) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f587(p4088: u64, v1: Bytes, p4089: u64) -> R<bool> {
    let m4087 = (p4088, p4089);
    {
        let mut m4087 = m4087;
        loop {
            let step = {
                let (v0, v2) = m4087;
                let m4033 = v0;
                if m4033 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m4033 - 1;
                    let m4036 = {
                        let a4034 = v1.clone();
                        let a4035 = v2;
                        index_bytes(a4034, a4035)
                    };
                    match m4036 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a4083 = {
                                let v5: u8 = v4;
                                let a4081 = {
                                    let a4057 = {
                                        let a4045 = {
                                            let a4039 = {
                                                let a4037 = v5;
                                                let a4038 = 128u8;
                                                fixed_u8::bit_and(a4037, a4038)
                                            };
                                            let a4040 = 128u8;
                                            if equal(a4039, a4040) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4046 = {
                                            let a4043 = {
                                                let a4041 = v5;
                                                let a4042 = 64u8;
                                                fixed_u8::bit_and(a4041, a4042)
                                            };
                                            let a4044 = 64u8;
                                            if equal(a4043, a4044) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4045, a4046)?
                                    };
                                    let a4058 = {
                                        let a4055 = {
                                            let a4049 = {
                                                let a4047 = v5;
                                                let a4048 = 32u8;
                                                fixed_u8::bit_and(a4047, a4048)
                                            };
                                            let a4050 = 32u8;
                                            if equal(a4049, a4050) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4056 = {
                                            let a4053 = {
                                                let a4051 = v5;
                                                let a4052 = 16u8;
                                                fixed_u8::bit_and(a4051, a4052)
                                            };
                                            let a4054 = 16u8;
                                            if equal(a4053, a4054) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4055, a4056)?
                                    };
                                    nat_add(a4057, a4058)?
                                };
                                let a4082 = {
                                    let a4079 = {
                                        let a4067 = {
                                            let a4061 = {
                                                let a4059 = v5;
                                                let a4060 = 8u8;
                                                fixed_u8::bit_and(a4059, a4060)
                                            };
                                            let a4062 = 8u8;
                                            if equal(a4061, a4062) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4068 = {
                                            let a4065 = {
                                                let a4063 = v5;
                                                let a4064 = 4u8;
                                                fixed_u8::bit_and(a4063, a4064)
                                            };
                                            let a4066 = 4u8;
                                            if equal(a4065, a4066) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4067, a4068)?
                                    };
                                    let a4080 = {
                                        let a4077 = {
                                            let a4071 = {
                                                let a4069 = v5;
                                                let a4070 = 2u8;
                                                fixed_u8::bit_and(a4069, a4070)
                                            };
                                            let a4072 = 2u8;
                                            if equal(a4071, a4072) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a4078 = {
                                            let a4075 = {
                                                let a4073 = v5;
                                                let a4074 = 1u8;
                                                fixed_u8::bit_and(a4073, a4074)
                                            };
                                            let a4076 = 1u8;
                                            if equal(a4075, a4076) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a4077, a4078)?
                                    };
                                    nat_add(a4079, a4080)?
                                };
                                nat_add(a4081, a4082)?
                            };
                            let a4084 = 128u64;
                            if nat_lt(a4083, a4084) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a4085 = v2;
                                    let a4086 = 1u64;
                                    nat_add(a4085, a4086)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4087 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f328(v0.clone(), 0u64, {
        let a4090 = v0;
        length_bytes(a4090)
    }, false, Bytes::lit(&[]))?;
    let m4093 = {
        let a4091 = v1.clone();
        let a4092 = 0u64;
        index_bytes(a4091, a4092)
    };
    match m4093 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a4138 = {
                    let a4114 = {
                        let a4102 = {
                            let a4096 = {
                                let a4094 = v3;
                                let a4095 = 128u8;
                                fixed_u8::bit_and(a4094, a4095)
                            };
                            let a4097 = 128u8;
                            if equal(a4096, a4097) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4103 = {
                            let a4100 = {
                                let a4098 = v3;
                                let a4099 = 64u8;
                                fixed_u8::bit_and(a4098, a4099)
                            };
                            let a4101 = 64u8;
                            if equal(a4100, a4101) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4102, a4103)?
                    };
                    let a4115 = {
                        let a4112 = {
                            let a4106 = {
                                let a4104 = v3;
                                let a4105 = 32u8;
                                fixed_u8::bit_and(a4104, a4105)
                            };
                            let a4107 = 32u8;
                            if equal(a4106, a4107) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4113 = {
                            let a4110 = {
                                let a4108 = v3;
                                let a4109 = 16u8;
                                fixed_u8::bit_and(a4108, a4109)
                            };
                            let a4111 = 16u8;
                            if equal(a4110, a4111) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4112, a4113)?
                    };
                    nat_add(a4114, a4115)?
                };
                let a4139 = {
                    let a4136 = {
                        let a4124 = {
                            let a4118 = {
                                let a4116 = v3;
                                let a4117 = 8u8;
                                fixed_u8::bit_and(a4116, a4117)
                            };
                            let a4119 = 8u8;
                            if equal(a4118, a4119) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4125 = {
                            let a4122 = {
                                let a4120 = v3;
                                let a4121 = 4u8;
                                fixed_u8::bit_and(a4120, a4121)
                            };
                            let a4123 = 4u8;
                            if equal(a4122, a4123) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4124, a4125)?
                    };
                    let a4137 = {
                        let a4134 = {
                            let a4128 = {
                                let a4126 = v3;
                                let a4127 = 2u8;
                                fixed_u8::bit_and(a4126, a4127)
                            };
                            let a4129 = 2u8;
                            if equal(a4128, a4129) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4135 = {
                            let a4132 = {
                                let a4130 = v3;
                                let a4131 = 1u8;
                                fixed_u8::bit_and(a4130, a4131)
                            };
                            let a4133 = 1u8;
                            if equal(a4132, a4133) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4134, a4135)?
                    };
                    nat_add(a4136, a4137)?
                };
                nat_add(a4138, a4139)?
            };
            if f386(v4) {
                let a4146 = f32({
                    let a4140 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a4140)
                }, {
                    let a4141 = v4;
                    let a4142 = 65u64;
                    nat_sub(a4141, a4142)
                }, 1u64);
                let a4147 = f32(v1.clone(), 1u64, {
                    let a4144 = {
                        let a4143 = v1;
                        length_bytes(a4143)
                    };
                    let a4145 = 1u64;
                    nat_sub(a4144, a4145)
                });
                Ok(append_bytes(a4146, a4147))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f589(v0: Bytes) -> R<bool> {
    let m4150 = {
        let a4148 = v0;
        let a4149 = 0u64;
        index_bytes(a4148, a4149)
    };
    match m4150 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f385({
                let v2: u8 = v1;
                let a4195 = {
                    let a4171 = {
                        let a4159 = {
                            let a4153 = {
                                let a4151 = v2;
                                let a4152 = 128u8;
                                fixed_u8::bit_and(a4151, a4152)
                            };
                            let a4154 = 128u8;
                            if equal(a4153, a4154) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4160 = {
                            let a4157 = {
                                let a4155 = v2;
                                let a4156 = 64u8;
                                fixed_u8::bit_and(a4155, a4156)
                            };
                            let a4158 = 64u8;
                            if equal(a4157, a4158) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4159, a4160)?
                    };
                    let a4172 = {
                        let a4169 = {
                            let a4163 = {
                                let a4161 = v2;
                                let a4162 = 32u8;
                                fixed_u8::bit_and(a4161, a4162)
                            };
                            let a4164 = 32u8;
                            if equal(a4163, a4164) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4170 = {
                            let a4167 = {
                                let a4165 = v2;
                                let a4166 = 16u8;
                                fixed_u8::bit_and(a4165, a4166)
                            };
                            let a4168 = 16u8;
                            if equal(a4167, a4168) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4169, a4170)?
                    };
                    nat_add(a4171, a4172)?
                };
                let a4196 = {
                    let a4193 = {
                        let a4181 = {
                            let a4175 = {
                                let a4173 = v2;
                                let a4174 = 8u8;
                                fixed_u8::bit_and(a4173, a4174)
                            };
                            let a4176 = 8u8;
                            if equal(a4175, a4176) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4182 = {
                            let a4179 = {
                                let a4177 = v2;
                                let a4178 = 4u8;
                                fixed_u8::bit_and(a4177, a4178)
                            };
                            let a4180 = 4u8;
                            if equal(a4179, a4180) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4181, a4182)?
                    };
                    let a4194 = {
                        let a4191 = {
                            let a4185 = {
                                let a4183 = v2;
                                let a4184 = 2u8;
                                fixed_u8::bit_and(a4183, a4184)
                            };
                            let a4186 = 2u8;
                            if equal(a4185, a4186) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4192 = {
                            let a4189 = {
                                let a4187 = v2;
                                let a4188 = 1u8;
                                fixed_u8::bit_and(a4187, a4188)
                            };
                            let a4190 = 1u8;
                            if equal(a4189, a4190) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4191, a4192)?
                    };
                    nat_add(a4193, a4194)?
                };
                nat_add(a4195, a4196)?
            }) {
                Ok(true)
            } else {
                Ok(f386({
                    let v3: u8 = v1;
                    let a4241 = {
                        let a4217 = {
                            let a4205 = {
                                let a4199 = {
                                    let a4197 = v3;
                                    let a4198 = 128u8;
                                    fixed_u8::bit_and(a4197, a4198)
                                };
                                let a4200 = 128u8;
                                if equal(a4199, a4200) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a4206 = {
                                let a4203 = {
                                    let a4201 = v3;
                                    let a4202 = 64u8;
                                    fixed_u8::bit_and(a4201, a4202)
                                };
                                let a4204 = 64u8;
                                if equal(a4203, a4204) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4205, a4206)?
                        };
                        let a4218 = {
                            let a4215 = {
                                let a4209 = {
                                    let a4207 = v3;
                                    let a4208 = 32u8;
                                    fixed_u8::bit_and(a4207, a4208)
                                };
                                let a4210 = 32u8;
                                if equal(a4209, a4210) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a4216 = {
                                let a4213 = {
                                    let a4211 = v3;
                                    let a4212 = 16u8;
                                    fixed_u8::bit_and(a4211, a4212)
                                };
                                let a4214 = 16u8;
                                if equal(a4213, a4214) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4215, a4216)?
                        };
                        nat_add(a4217, a4218)?
                    };
                    let a4242 = {
                        let a4239 = {
                            let a4227 = {
                                let a4221 = {
                                    let a4219 = v3;
                                    let a4220 = 8u8;
                                    fixed_u8::bit_and(a4219, a4220)
                                };
                                let a4222 = 8u8;
                                if equal(a4221, a4222) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a4228 = {
                                let a4225 = {
                                    let a4223 = v3;
                                    let a4224 = 4u8;
                                    fixed_u8::bit_and(a4223, a4224)
                                };
                                let a4226 = 4u8;
                                if equal(a4225, a4226) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4227, a4228)?
                        };
                        let a4240 = {
                            let a4237 = {
                                let a4231 = {
                                    let a4229 = v3;
                                    let a4230 = 2u8;
                                    fixed_u8::bit_and(a4229, a4230)
                                };
                                let a4232 = 2u8;
                                if equal(a4231, a4232) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a4238 = {
                                let a4235 = {
                                    let a4233 = v3;
                                    let a4234 = 1u8;
                                    fixed_u8::bit_and(a4233, a4234)
                                };
                                let a4236 = 1u8;
                                if equal(a4235, a4236) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a4237, a4238)?
                        };
                        nat_add(a4239, a4240)?
                    };
                    nat_add(a4241, a4242)?
                }))
            }
        }
    }
}

pub fn f590(p4255: u64, v1: Bytes, p4256: u64, p4257: Bytes) -> R<Option<Bytes>> {
    let m4254 = (p4255, (p4256, p4257));
    {
        let mut m4254 = m4254;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4254;
                let m4243 = v0;
                if m4243 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4243 - 1;
                    let a4245 = {
                        let a4244 = v1.clone();
                        length_bytes(a4244)
                    };
                    let a4246 = v2;
                    if nat_le(a4245, a4246) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4247 = f418(v1.clone(), v2)?;
                        match m4247 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4249 = v2;
                                    let a4250 = {
                                        let (_, h4248) = v5;
                                        h4248
                                    };
                                    nat_add(a4249, a4250)?
                                }, {
                                    let a4252 = v3;
                                    let a4253 = f596({
                                        let (h4251, _) = v5;
                                        h4251
                                    });
                                    append_bytes(a4252, a4253)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4254 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f591(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4258 = v0.clone();
    match m4258 {
        Adt19::C0(v2) => {
            if v2 {
                Ok(f558())
            } else {
                Ok(f504(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt19::C1(_, _) => {
            Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C2(_, _, _) => {
            Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C3(_) => {
            Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C4(_) => {
            Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C5(_) => {
            let m4259 = f597(f434(v0.clone()));
            match m4259 {
                None => {
                    let m4260 = f246(f148(v0.clone(), f8(Str::lit("$ref"))));
                    match m4260 {
                        None => {
                            let m4262 = f148(v0.clone(), f8(Str::lit("allOf")));
                            match m4262 {
                                None => {
                                    let m4267 = f148(v0.clone(), f8(Str::lit("enum")));
                                    match m4267 {
                                        None => {
                                            let m4268 = f148(v0.clone(), f8(Str::lit("const")));
                                            match m4268 {
                                                None => {
                                                    let m4270 = f148(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m4270 {
                                                        None => {
                                                            let v30: Bytes = f151(v0.clone(), f8(Str::lit("type")));
                                                            if if f36(v30.clone(), Bytes::lit(&[])) {
                                                                !f598(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f558())
                                                            } else {
                                                                if f36(v30.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f505(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                                } else {
                                                                    if f36(v30.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f505(f599(v0), List::<Adt25>::nil()))
                                                                    } else {
                                                                        if f36(v30.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f505(f600(), List::<Adt25>::nil()))
                                                                        } else {
                                                                            if f36(v30.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f505(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                                                            } else {
                                                                                if f36(v30.clone(), f8(Str::lit("object"))) {
                                                                                    let m4271 = f389(f148(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m4271.uncons() {
                                                                                        None => {
                                                                                            let m4272 = f601(v0);
                                                                                            match m4272 {
                                                                                                None => {
                                                                                                    Ok(f556(f8(Str::lit("List")), f602(f558())))
                                                                                                }
                                                                                                Some(v33) => {
                                                                                                    Ok(f556(f8(Str::lit("List")), f602(f603(v33, v1)?)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f505(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f504(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v30, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f505(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v27) => {
                                                    let m4269 = f246(Some(v27));
                                                    match m4269 {
                                                        None => {
                                                            Ok(f504(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f505(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f505(v1.clone(), List::cons(Adt25::C0(v1, v0), List::<Adt25>::nil())))
                                        }
                                    }
                                }
                                Some(v19) => {
                                    let m4263 = f247(v19);
                                    match m4263.uncons() {
                                        None => {
                                            Ok(f504(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v20, v21)) => {
                                            let m4264 = v21;
                                            match m4264.uncons() {
                                                None => {
                                                    let m4265 = f246(f148(v20, f8(Str::lit("$ref"))));
                                                    match m4265 {
                                                        None => {
                                                            Ok(f504(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v24) => {
                                                            let m4266 = f502(v24)?;
                                                            match m4266 {
                                                                None => {
                                                                    Ok(f504(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v25) => {
                                                                    Ok(f505(v25, List::<Adt25>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f504(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v17) => {
                            let m4261 = f502(v17)?;
                            match m4261 {
                                None => {
                                    Ok(f504(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v18) => {
                                    Ok(f505(v18, List::<Adt25>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v16) => {
                    Ok(f504(if f36(v16.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v16, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt19::C6 => {
            Ok(f558())
        }
        Adt19::C7(_, _) => {
            Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt19::C8(_, _, _) => {
            Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f592(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4273, _) = v0;
        h4273
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4274) = v0;
        h4274
    };
    let m4275 = v1;
    match m4275.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4276 = v4;
            match m4276.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f604(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f593(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h4277, _) = v0.clone();
        h4277
    };
    let v2: List<(Bytes, ())> = {
        let (h4278, _) = {
            let (_, h4279) = v0.clone();
            h4279
        };
        h4278
    };
    let v3: List<(Bytes, ())> = {
        let (_, h4280) = {
            let (_, h4281) = v0;
            h4281
        };
        h4280
    };
    let m4282 = v1.clone();
    match m4282.uncons() {
        None => {
            let m4283 = v2;
            match m4283.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4284 = v2.clone();
            match m4284.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f605({
                        let (h4285, _) = v8.clone();
                        h4285
                    }, {
                        let (h4286, _) = v6.clone();
                        h4286
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

pub fn f594(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4287, _) = v1;
        h4287
    };
    let m4289 = {
        let (_, h4288) = v1;
        h4288
    };
    match m4289 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4292 = {
                let a4290 = v0.clone();
                let a4291 = v2;
                index_bytes(a4290, a4291)
            };
            match m4292 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4297 = {
                        let a4295 = v0;
                        let a4296 = {
                            let a4293 = v2;
                            let a4294 = 1u64;
                            nat_add(a4293, a4294)?
                        };
                        index_bytes(a4295, a4296)
                    };
                    match m4297 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f606(v4)? {
                                f606(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4298 = v2;
                                    let a4299 = 2u64;
                                    nat_add(a4298, a4299)?
                                }, Some({
                                    let a4304 = v3;
                                    let a4305 = f317(f607(), {
                                        let a4302 = {
                                            let a4300 = f608(v4)?;
                                            let a4301 = 16u64;
                                            nat_mul(a4300, a4301)?
                                        };
                                        let a4303 = f608(v5)?;
                                        nat_add(a4302, a4303)?
                                    });
                                    append_bytes(a4304, a4305)
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

pub fn f595(p4318: u64, v1: Bytes, p4319: u64, p4320: Bytes) -> R<Option<Bytes>> {
    let m4317 = (p4318, (p4319, p4320));
    {
        let mut m4317 = m4317;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4317;
                let m4306 = v0;
                if m4306 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4306 - 1;
                    let a4308 = {
                        let a4307 = v1.clone();
                        length_bytes(a4307)
                    };
                    let a4309 = v2;
                    if nat_le(a4308, a4309) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4310 = f418(v1.clone(), v2)?;
                        match m4310 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4312 = v2;
                                    let a4313 = {
                                        let (_, h4311) = v5;
                                        h4311
                                    };
                                    nat_add(a4312, a4313)?
                                }, {
                                    let a4315 = v3;
                                    let a4316 = f609(v1.clone(), v2, {
                                        let (h4314, _) = v5;
                                        h4314
                                    });
                                    append_bytes(a4315, a4316)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(v0: u64) -> Bytes {
    let a4321 = f8(Str::lit("u"));
    let a4322 = f610(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4321, a4322)
}

pub fn f597(p4327: List<(Bytes, Adt19)>) -> Option<Bytes> {
    let m4326 = p4327;
    {
        let mut m4326 = m4326;
        loop {
            let step = {
                let v0 = m4326;
                let m4323 = v0;
                match m4323.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt19)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f86({
                            let (h4324, _) = v1.clone();
                            h4324
                        }, f611()) {
                            Err::<List<(Bytes, Adt19)>, Option<Bytes>>(Some({
                                let (h4325, _) = v1;
                                h4325
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt19)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4326 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598(v0: Adt19) -> bool {
    if f513(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f513(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f513(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f513(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f513(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f513(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f599(v0: Adt19) -> Bytes {
    let m4328 = f612(f148(v0, f8(Str::lit("minimum"))));
    match m4328 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f600() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f601(v0: Adt19) -> Option<Adt19> {
    let m4329 = f148(v0.clone(), f8(Str::lit("additionalProperties")));
    match m4329 {
        None => {
            let m4330 = f389(f148(v0, f8(Str::lit("patternProperties"))));
            match m4330.uncons() {
                None => {
                    None::<Adt19>
                }
                Some((v2, v3)) => {
                    let m4331 = v3;
                    match m4331.uncons() {
                        None => {
                            Some({
                                let (_, h4332) = v2;
                                h4332
                            })
                        }
                        Some((_, _)) => {
                            None::<Adt19>
                        }
                    }
                }
            }
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f602(v0: Result<(Bytes, List<Adt25>), Bytes>) -> Result<(Bytes, List<Adt25>), Bytes> {
    let m4333 = v0;
    match m4333 {
        Ok(v2) => {
            f505(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons({
                let (h4334, _) = v2;
                h4334
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), {
                let (_, h4335) = v2;
                h4335
            })
        }
        Err(v1) => {
            Err::<(Bytes, List<Adt25>), Bytes>(v1)
        }
    }
}

pub fn f603(v0: Adt19, v1: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    f613(16u64, v0, v1)
}

pub fn f604(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4340 = {
            let a4338 = {
                let a4336 = v0.clone();
                length_list(a4336)
            };
            let a4339 = {
                let a4337 = v1.clone();
                length_list(a4337)
            };
            nat_add(a4338, a4339)?
        };
        let a4341 = 1u64;
        nat_add(a4340, a4341)?
    };
    Ok(f614({
        let (_, h4342) = {
            let (_, h4343) = {
                let (h4344, _) = f615(Fn25::F616, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4344
            };
            h4343
        };
        h4342
    }))
}

pub fn f605(v0: Bytes, v1: Bytes) -> bool {
    let a4349 = {
        let a4345 = v0;
        let a4346 = v1;
        compare_bytes(a4345, a4346)
    };
    let a4350 = {
        let a4347 = Bytes::lit(&[]);
        let a4348 = Bytes::lit(&[0x01]);
        compare_bytes(a4347, a4348)
    };
    equal(a4349, a4350)
}

pub fn f606(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4395 = {
            let a4371 = {
                let a4359 = {
                    let a4353 = {
                        let a4351 = v1;
                        let a4352 = 128u8;
                        fixed_u8::bit_and(a4351, a4352)
                    };
                    let a4354 = 128u8;
                    if equal(a4353, a4354) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4360 = {
                    let a4357 = {
                        let a4355 = v1;
                        let a4356 = 64u8;
                        fixed_u8::bit_and(a4355, a4356)
                    };
                    let a4358 = 64u8;
                    if equal(a4357, a4358) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4359, a4360)?
            };
            let a4372 = {
                let a4369 = {
                    let a4363 = {
                        let a4361 = v1;
                        let a4362 = 32u8;
                        fixed_u8::bit_and(a4361, a4362)
                    };
                    let a4364 = 32u8;
                    if equal(a4363, a4364) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4370 = {
                    let a4367 = {
                        let a4365 = v1;
                        let a4366 = 16u8;
                        fixed_u8::bit_and(a4365, a4366)
                    };
                    let a4368 = 16u8;
                    if equal(a4367, a4368) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4369, a4370)?
            };
            nat_add(a4371, a4372)?
        };
        let a4396 = {
            let a4393 = {
                let a4381 = {
                    let a4375 = {
                        let a4373 = v1;
                        let a4374 = 8u8;
                        fixed_u8::bit_and(a4373, a4374)
                    };
                    let a4376 = 8u8;
                    if equal(a4375, a4376) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4382 = {
                    let a4379 = {
                        let a4377 = v1;
                        let a4378 = 4u8;
                        fixed_u8::bit_and(a4377, a4378)
                    };
                    let a4380 = 4u8;
                    if equal(a4379, a4380) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4381, a4382)?
            };
            let a4394 = {
                let a4391 = {
                    let a4385 = {
                        let a4383 = v1;
                        let a4384 = 2u8;
                        fixed_u8::bit_and(a4383, a4384)
                    };
                    let a4386 = 2u8;
                    if equal(a4385, a4386) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4392 = {
                    let a4389 = {
                        let a4387 = v1;
                        let a4388 = 1u8;
                        fixed_u8::bit_and(a4387, a4388)
                    };
                    let a4390 = 1u8;
                    if equal(a4389, a4390) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4391, a4392)?
            };
            nat_add(a4393, a4394)?
        };
        nat_add(a4395, a4396)?
    };
    let a4397 = 48u64;
    let a4398 = v2;
    if if nat_le(a4397, a4398) {
        let a4399 = v2;
        let a4400 = 57u64;
        nat_le(a4399, a4400)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4401 = 97u64;
        let a4402 = v2;
        if nat_le(a4401, a4402) {
            let a4403 = v2;
            let a4404 = 102u64;
            Ok(nat_le(a4403, a4404))
        } else {
            Ok(false)
        }
    }
}

pub fn f607() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f608(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4449 = {
            let a4425 = {
                let a4413 = {
                    let a4407 = {
                        let a4405 = v1;
                        let a4406 = 128u8;
                        fixed_u8::bit_and(a4405, a4406)
                    };
                    let a4408 = 128u8;
                    if equal(a4407, a4408) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4414 = {
                    let a4411 = {
                        let a4409 = v1;
                        let a4410 = 64u8;
                        fixed_u8::bit_and(a4409, a4410)
                    };
                    let a4412 = 64u8;
                    if equal(a4411, a4412) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4413, a4414)?
            };
            let a4426 = {
                let a4423 = {
                    let a4417 = {
                        let a4415 = v1;
                        let a4416 = 32u8;
                        fixed_u8::bit_and(a4415, a4416)
                    };
                    let a4418 = 32u8;
                    if equal(a4417, a4418) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4424 = {
                    let a4421 = {
                        let a4419 = v1;
                        let a4420 = 16u8;
                        fixed_u8::bit_and(a4419, a4420)
                    };
                    let a4422 = 16u8;
                    if equal(a4421, a4422) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4423, a4424)?
            };
            nat_add(a4425, a4426)?
        };
        let a4450 = {
            let a4447 = {
                let a4435 = {
                    let a4429 = {
                        let a4427 = v1;
                        let a4428 = 8u8;
                        fixed_u8::bit_and(a4427, a4428)
                    };
                    let a4430 = 8u8;
                    if equal(a4429, a4430) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4436 = {
                    let a4433 = {
                        let a4431 = v1;
                        let a4432 = 4u8;
                        fixed_u8::bit_and(a4431, a4432)
                    };
                    let a4434 = 4u8;
                    if equal(a4433, a4434) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4435, a4436)?
            };
            let a4448 = {
                let a4445 = {
                    let a4439 = {
                        let a4437 = v1;
                        let a4438 = 2u8;
                        fixed_u8::bit_and(a4437, a4438)
                    };
                    let a4440 = 2u8;
                    if equal(a4439, a4440) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4446 = {
                    let a4443 = {
                        let a4441 = v1;
                        let a4442 = 1u8;
                        fixed_u8::bit_and(a4441, a4442)
                    };
                    let a4444 = 1u8;
                    if equal(a4443, a4444) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4445, a4446)?
            };
            nat_add(a4447, a4448)?
        };
        nat_add(a4449, a4450)?
    };
    let a4451 = v2;
    let a4452 = 57u64;
    if nat_le(a4451, a4452) {
        let a4453 = v2;
        let a4454 = 48u64;
        Ok(nat_sub(a4453, a4454))
    } else {
        let a4455 = v2;
        let a4456 = 87u64;
        Ok(nat_sub(a4455, a4456))
    }
}

pub fn f609(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if if if if f385(v2) {
        true
    } else {
        f386(v2)
    } {
        let a4457 = v2;
        let a4458 = 117u64;
        if !nat_eq(a4457, a4458) {
            let a4459 = v2;
            let a4460 = 85u64;
            !nat_eq(a4459, a4460)
        } else {
            false
        }
    } else {
        false
    } {
        true
    } else {
        if f387(v2) {
            let a4461 = 0u64;
            let a4462 = v1;
            nat_lt(a4461, a4462)
        } else {
            false
        }
    } {
        f32(v0, v1, 1u64)
    } else {
        let a4463 = v2;
        let a4464 = 65536u64;
        if nat_lt(a4463, a4464) {
            let a4465 = f8(Str::lit("u"));
            let a4466 = f610(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4465, a4466)
        } else {
            let a4467 = f8(Str::lit("U"));
            let a4468 = f610(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4467, a4468)
        }
    }
}

pub fn f610(p4485: u64, p4486: u64, p4487: u64, p4488: Bytes) -> Bytes {
    let m4484 = (p4485, (p4486, (p4487, p4488)));
    {
        let mut m4484 = m4484;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m4484;
                let m4469 = v0;
                if m4469 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m4469 - 1;
                    let a4470 = v1;
                    let a4471 = 0u64;
                    if if nat_eq(a4470, a4471) {
                        let a4472 = v2;
                        let a4473 = 0u64;
                        nat_eq(a4472, a4473)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a4474 = v1;
                            let a4475 = 16u64;
                            let a4476 = 0u64;
                            nat_quot(a4474, a4475, a4476)
                        }, ({
                            let a4477 = v2;
                            let a4478 = 1u64;
                            nat_sub(a4477, a4478)
                        }, {
                            let a4482 = f617({
                                let a4479 = v1;
                                let a4480 = 16u64;
                                let a4481 = 0u64;
                                nat_rem(a4479, a4480, a4481)
                            });
                            let a4483 = v3;
                            append_bytes(a4482, a4483)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m4484 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f611() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f612(v0: Option<Adt19>) -> Option<u64> {
    let m4489 = v0;
    match m4489 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m4490 = v1;
            match m4490 {
                Adt19::C0(_) => {
                    None::<u64>
                }
                Adt19::C1(v5, v6) => {
                    if v5 {
                        None::<u64>
                    } else {
                        Some(v6)
                    }
                }
                Adt19::C2(_, _, _) => {
                    None::<u64>
                }
                Adt19::C3(_) => {
                    None::<u64>
                }
                Adt19::C4(_) => {
                    None::<u64>
                }
                Adt19::C5(_) => {
                    None::<u64>
                }
                Adt19::C6 => {
                    None::<u64>
                }
                Adt19::C7(_, _) => {
                    None::<u64>
                }
                Adt19::C8(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f613(v0: u64, v1: Adt19, v2: Bytes) -> R<Result<(Bytes, List<Adt25>), Bytes>> {
    let m4491 = v0;
    if m4491 == 0 {
        Ok(f504(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m4491 - 1;
        let m4492 = v1.clone();
        match m4492 {
            Adt19::C0(v4) => {
                if v4 {
                    Ok(f558())
                } else {
                    Ok(f504(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt19::C1(_, _) => {
                Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C2(_, _, _) => {
                Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C3(_) => {
                Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C4(_) => {
                Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C5(_) => {
                let m4493 = f246(f148(v1.clone(), f8(Str::lit("$ref"))));
                match m4493 {
                    None => {
                        if if f513(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f513(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f513(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f505(v2.clone(), List::cons(Adt25::C0(v2, v1), List::<Adt25>::nil())))
                        } else {
                            let v20: Bytes = f151(v1.clone(), f8(Str::lit("type")));
                            if if f36(v20.clone(), Bytes::lit(&[])) {
                                !f598(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f558())
                            } else {
                                if f36(v20.clone(), f8(Str::lit("string"))) {
                                    Ok(f505(f8(Str::lit("Bytes")), List::<Adt25>::nil()))
                                } else {
                                    if f36(v20.clone(), f8(Str::lit("integer"))) {
                                        Ok(f505(f599(v1), List::<Adt25>::nil()))
                                    } else {
                                        if f36(v20.clone(), f8(Str::lit("number"))) {
                                            Ok(f505(f600(), List::<Adt25>::nil()))
                                        } else {
                                            if f36(v20.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f505(f8(Str::lit("Bool")), List::<Adt25>::nil()))
                                            } else {
                                                if f36(v20.clone(), f8(Str::lit("array"))) {
                                                    let m4495 = f148(v1, f8(Str::lit("items")));
                                                    match m4495 {
                                                        None => {
                                                            Ok(f556(f8(Str::lit("List")), f558()))
                                                        }
                                                        Some(v21) => {
                                                            Ok(f556(f8(Str::lit("List")), f613(v3, v21, {
                                                                let a4496 = v2;
                                                                let a4497 = f8(Str::lit("Item"));
                                                                append_bytes(a4496, a4497)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f36(v20.clone(), f8(Str::lit("object"))) {
                                                        let m4498 = f601(v1);
                                                        match m4498 {
                                                            None => {
                                                                Ok(f556(f8(Str::lit("List")), f602(f558())))
                                                            }
                                                            Some(v22) => {
                                                                Ok(f556(f8(Str::lit("List")), f602(f613(v3, v22, {
                                                                    let a4499 = v2;
                                                                    let a4500 = f8(Str::lit("Value"));
                                                                    append_bytes(a4499, a4500)
                                                                })?)))
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f504(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v20, List::<Bytes>::nil()))), v2))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(v18) => {
                        let m4494 = f502(v18)?;
                        match m4494 {
                            None => {
                                Ok(f504(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v19) => {
                                Ok(f505(v19, List::<Adt25>::nil()))
                            }
                        }
                    }
                }
            }
            Adt19::C6 => {
                Ok(f558())
            }
            Adt19::C7(_, _) => {
                Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt19::C8(_, _, _) => {
                Ok(f504(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f614(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f399(Fn10::F618, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f615(v0: Fn25, p4505: u64, p4506: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4504 = (p4505, p4506);
    {
        let mut m4504 = m4504;
        loop {
            let step = {
                let (v1, v2) = m4504;
                let m4501 = v1;
                if m4501 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4501 - 1;
                    let m4503 = {
                        let c4502: Fn25 = v0.clone();
                        c4502.apply(v2.clone())
                    };
                    match m4503 {
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
                Ok(next) => { m4504 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f616(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f619(v0)
}

pub fn f617(v0: u64) -> Bytes {
    f32({
        let a4507 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4507)
    }, v0, 1u64)
}

pub fn f618(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f619(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4508, _) = v0.clone();
        h4508
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4509, _) = {
            let (_, h4510) = v0.clone();
            h4510
        };
        h4509
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4511) = {
            let (_, h4512) = v0;
            h4512
        };
        h4511
    };
    let m4513 = v1.clone();
    match m4513.uncons() {
        None => {
            let m4514 = v2;
            match m4514.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4515 = v2.clone();
            match m4515.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f605({
                        let (h4516, _) = v8.clone();
                        h4516
                    }, {
                        let (h4517, _) = v6.clone();
                        h4517
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
