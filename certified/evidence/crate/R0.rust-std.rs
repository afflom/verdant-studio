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
    C0(u64, Adt18),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(u64, Option<u64>),
    C1(u64),
    C2(Adt19, List<u64>, u64),
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt20 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Bytes, Adt14),
    C1(Bytes),
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
    F114(u8, Bytes),
    F156(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F114(k0, k1) => f114(*k0, k1.clone(), p0),
            Fn0::F156(k0) => f156(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F132,
    F172,
    F183,
}

impl Fn1 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn1::F132 => f132(p0, p1),
            Fn1::F172 => f172(p0, p1),
            Fn1::F183 => f183(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F152(Bytes),
}

impl Fn2 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn2::F152(k0) => f152(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F199(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<(u64, Adt20)> {
        match self {
            Fn3::F199(k0, k1, k2) => f199(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F231,
}

impl Fn4 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn4::F231 => f231(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F234,
}

impl Fn5 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn5::F234 => f234(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F236,
}

impl Fn6 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn6::F236 => f236(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F239,
}

impl Fn7 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn7::F239 => f239(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F246(Bytes),
    F247(u64, Bytes),
    F303(Bytes),
    F347(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn8::F246(k0) => Ok(f246(k0.clone(), p0)),
            Fn8::F247(k0, k1) => f247(*k0, k1.clone(), p0),
            Fn8::F303(k0) => f303(k0.clone(), p0),
            Fn8::F347(k0) => f347(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F263(Bytes),
    F311(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn9::F263(k0) => f263(k0.clone(), p0),
            Fn9::F311(k0, k1) => Ok(f311(*k0, k1.clone(), p0)),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F265,
    F379,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F265 => f265(p0, p1),
            Fn10::F379 => f379(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F268,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F268 => f268(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F270,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F270 => f270(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F273,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F273 => f273(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F275,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt23) -> R<Adt23> {
        match self {
            Fn14::F275 => f275(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F280(Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
        match self {
            Fn15::F280(k0) => f280(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F306(Bytes),
}

impl Fn16 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn16::F306(k0) => f306(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F315,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn17::F315 => f315(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F317,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn18::F317 => f317(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F319,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F319 => f319(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F327,
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn20::F327 => f327(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F339,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F339 => f339(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F361,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F361 => f361(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F363,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F363 => f363(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F365(Bytes),
}

impl Fn24 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn24::F365(k0) => f365(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F378,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F378 => f378(p0),
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
                if f26(v2.clone(), f8(Str::lit("next"))) {
                    f28(v1, v3)
                } else {
                    if f26(v2.clone(), f8(Str::lit("build"))) {
                        Ok(f29(v1))
                    } else {
                        if f26(v2.clone(), f8(Str::lit("publish"))) {
                            f30(v1)
                        } else {
                            if f26(v2.clone(), f8(Str::lit("agenda"))) {
                                f31(v1)
                            } else {
                                let m128 = v3;
                                match m128.uncons() {
                                    None => {
                                        Ok(Adt11::C4(f25()))
                                    }
                                    Some((v4, v5)) => {
                                        if f26(v2.clone(), f8(Str::lit("ingest-index"))) {
                                            f32(v1, v4)
                                        } else {
                                            if f26(v2.clone(), f8(Str::lit("stage-bindings"))) {
                                                f33(v1, v4)
                                            } else {
                                                if f26(v2.clone(), f8(Str::lit("admit"))) {
                                                    f34(v1, v4)
                                                } else {
                                                    if f26(v2.clone(), f8(Str::lit("stage-encodings"))) {
                                                        f35(v1, v4)
                                                    } else {
                                                        if f26(v2.clone(), f8(Str::lit("admit-encoding"))) {
                                                            f36(v1, v4)
                                                        } else {
                                                            if f26(v2.clone(), f8(Str::lit("record-decision"))) {
                                                                f37(v1, v4)
                                                            } else {
                                                                if f26(v2.clone(), f8(Str::lit("merge-gate"))) {
                                                                    f38(v1, v4)
                                                                } else {
                                                                    if f26(v2.clone(), f8(Str::lit("ingest-authority"))) {
                                                                        f39(v1, v4)
                                                                    } else {
                                                                        if f26(v2, f8(Str::lit("propose-encoding"))) {
                                                                            let m129 = v5;
                                                                            match m129.uncons() {
                                                                                None => {
                                                                                    Ok(Adt11::C4(f25()))
                                                                                }
                                                                                Some((v6, v7)) => {
                                                                                    f40(v1, v4, v6, v7)
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
    let v1: List<Bytes> = f41(v0.clone(), 47u8)?;
    let v2: Bytes = f42(v1);
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
        Ok(f43(v0.clone(), 0u64, {
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
    f8(Str::lit("usage: verdant_studio check | next [SKIP...] | agenda | stage-bindings JSONL | admit JSONL | record-decision JSONL | stage-encodings JSONL | admit-encoding STAGED | merge-gate RULES | ingest-index CSV | ingest-authority BINDING | propose-encoding SCHEMA NAME CITATION... | build | publish\u{a}"))
}

pub fn f26(v0: Bytes, v1: Bytes) -> bool {
    let a152 = v0;
    let a153 = v1;
    equal(a152, a153)
}

pub fn f27(v0: Adt1) -> R<Adt11> {
    let m154 = f44(v0.clone(), f45());
    match m154.uncons() {
        None => {
            let m155 = f46(v0.clone(), f45())?;
            match m155.uncons() {
                None => {
                    let m158 = f47(match v0.clone() {
                        Adt1::C0(_, h156, _) => {
                            h156
                        }
                    }, f48(f49(), f50(match v0.clone() {
                        Adt1::C0(_, h157, _) => {
                            h157
                        }
                    }, false)));
                    match m158.uncons() {
                        None => {
                            let v7: (bool, Bytes) = f51(Adt13::C0(Adt12::C0(f52(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))))?, f54(v0.clone())?, f55(f56(v0.clone(), f8(Str::lit("model/requirements.json")))?, f8(Str::lit("id"))), f52(f53(v0.clone(), f8(Str::lit("model/ids.toml"))))?, f55(f56(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")))), f56(v0.clone(), f8(Str::lit("model/admissions.json")))?, f56(v0.clone(), f8(Str::lit("model/strata.json")))?, f57(f56(v0.clone(), f8(Str::lit("model/register-lock.json")))?), f50(match v0 {
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

pub fn f28(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f58(f59(match v0.clone() {
        Adt1::C0(_, h163, _) => {
            h163
        }
    }, f60())) {
        Ok(Adt11::C1(List::cons(f60(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f61(f62(f60(), f63(match v0.clone() {
            Adt1::C0(_, h164, _) => {
                h164
            }
        }, f60())))?;
        let m165 = f44(v0.clone(), List::cons(f8(Str::lit("model/authorities.toml")), v2.clone()));
        match m165.uncons() {
            None => {
                let m166 = f64(v0.clone(), f65(v2, v1.clone()))?;
                match m166 {
                    None => {
                        f66(v0, v1)
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

pub fn f29(v0: Adt1) -> Adt11 {
    if f58(f67(f8(Str::lit("prism-code lock")), match v0.clone() {
        Adt1::C0(_, _, h167) => {
            h167
        }
    })) {
        Adt11::C2(f8(Str::lit("npx")), f68(Str::lit("lock")), f8(Str::lit("prism-code lock")))
    } else {
        if f58(f67(f8(Str::lit("prism-code build")), match v0 {
            Adt1::C0(_, _, h168) => {
                h168
            }
        })) {
            Adt11::C2(f8(Str::lit("npx")), f68(Str::lit("build")), f8(Str::lit("prism-code build")))
        } else {
            Adt11::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("built: prism-code lock, then build (which checks), passed\u{a}")))
        }
    }
}

pub fn f30(v0: Adt1) -> R<Adt11> {
    let m169 = f44(v0.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f69(), List::<Bytes>::nil()))));
    match m169.uncons() {
        None => {
            Ok(Adt11::C3(f70(f71(v0.clone())?, List::cons((f72(), f73()), List::cons((f8(Str::lit(".github/workflows/operate.yml")), f74()), List::cons((f8(Str::lit(".github/CODEOWNERS")), f75()), List::cons((f8(Str::lit("docs/model.md")), f76(v0)?), List::cons((f8(Str::lit(".github/secret_scanning.yml")), f77()), List::<(Bytes, Bytes)>::nil())))))), f8(Str::lit("published the workflows, CODEOWNERS, secret scanning, docs/model.md and docs/owner-decisions.md\u{a}"))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f31(v0: Adt1) -> R<Adt11> {
    let m170 = f44(v0.clone(), List::cons(f78(), List::cons(f8(Str::lit("model/admissions.json")), List::<Bytes>::nil())));
    match m170.uncons() {
        None => {
            Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(f79(f80(f81(f82(f53(v0.clone(), f78()))?)), f83(f56(v0, f8(Str::lit("model/admissions.json")))?)))))
        }
        Some((v1, v2)) => {
            Ok(Adt11::C0(List::cons(v1, v2)))
        }
    }
}

pub fn f32(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m171 = f44(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m171.uncons() {
        None => {
            let v4: Bytes = f53(v0.clone(), v1);
            let v5: List<List<Bytes>> = f82(v4.clone())?;
            if f58(f84(f85(v5.clone()), f86())) {
                Ok(Adt11::C4(f8(Str::lit("the index header is not id,domain,name,owner,phases,scope,adoption,adoption_code,source\u{a}"))))
            } else {
                let v6: Bytes = f8(Str::lit("FATELINES-STANDARDS-INDEX"));
                let v7: Bytes = f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f87(Str::lit("id"), v6.clone())?, List::cons(f87(Str::lit("statement"), f8(Str::lit("The Fatelines game lifecycle standards index: one row per standard, with its domain, owner, phases, scope and adoption. Its rows are citations, not claims.")))?, List::cons(f87(Str::lit("artifact"), f78())?, List::cons(f87(Str::lit("checksum"), f88(v4.clone())?)?, List::cons(f8(Str::lit("realized_by = []\u{a}")), List::<Bytes>::nil())))))));
                Ok(Adt11::C3(List::cons((f78(), v4), List::cons((f8(Str::lit("model/authorities.toml")), f89(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v6.clone(), v7)?), List::cons(f90(v0, List::cons(v6, List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested the standards index: ")), List::cons(f91(f80(f81(v5)), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f33(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m172 = f44(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m172.uncons() {
        None => {
            let m173 = f92(f93(f53(v0, v1))?)?;
            match m173 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be one prismpm/authority-binding/1 binding with an id\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f91(f94(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f34(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m174 = f44(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil())))));
    match m174.uncons() {
        None => {
            let v4: List<Bytes> = f93(f53(v0.clone(), v1))?;
            let m175 = f95(v4.clone(), f8(Str::lit("requirement")))?;
            match m175 {
                None => {
                    Ok(Adt11::C4(f96()))
                }
                Some(v5) => {
                    let m176 = f95(v4, f8(Str::lit("admission")))?;
                    match m176 {
                        None => {
                            Ok(Adt11::C4(f96()))
                        }
                        Some(v6) => {
                            let v7: List<Adt14> = f56(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt14> = f56(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f48(f55(v5.clone(), f8(Str::lit("id"))), f55(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f61(f48(f55(v7.clone(), f8(Str::lit("id"))), f55(v8.clone(), f8(Str::lit("construct")))))?;
                            let a178 = {
                                let a177 = v9.clone();
                                length_list(a177)
                            };
                            let a179 = 0u64;
                            if nat_eq(a178, a179) {
                                Ok(Adt11::C4(f96()))
                            } else {
                                if f58({
                                    let a182 = {
                                        let a180 = f61(f48(v9.clone(), v10.clone()))?;
                                        length_list(a180)
                                    };
                                    let a183 = {
                                        let a181 = f48(v9.clone(), v10);
                                        length_list(a181)
                                    };
                                    nat_eq(a182, a183)
                                }) {
                                    Ok(Adt11::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                } else {
                                    Ok(Adt11::C3(List::cons(f97(f8(Str::lit("model/requirements.json")), f98(v7, v5))?, List::cons(f97(f8(Str::lit("model/admissions.json")), f98(v8, v6))?, List::cons(f90(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f91(v9, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f35(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m184 = f44(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m184.uncons() {
        None => {
            let m185 = f99(f93(f53(v0, v1))?)?;
            match m185 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an encoding: name, schema, and a citation list (req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...)\u{a}"))))
                }
                Some(v4) => {
                    Ok(Adt11::C3(v4.clone(), f7(List::cons(f8(Str::lit("staged ")), List::cons(f91(f94(v4), f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f36(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m186 = f44(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m186.uncons() {
        None => {
            let m187 = f100(v0.clone(), v1.clone())?;
            match m187 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m188 = f101(v4.clone(), f8(Str::lit("schema")));
                    match m188 {
                        None => {
                            Ok(Adt11::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f102(v0, f103(v5.clone())?, v5, f104(v4.clone(), f8(Str::lit("name"))), f57(f105(f101(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
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

pub fn f37(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m189 = f44(v0.clone(), List::cons(v1.clone(), List::cons(f69(), List::<Bytes>::nil())));
    match m189.uncons() {
        None => {
            let m190 = f106(f93(f53(v0.clone(), v1))?)?;
            match m190 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("every line must be an owner decision: agent, at, the verbatim quote, and requirements\u{a}"))))
                }
                Some(v4) => {
                    let v5: List<Adt14> = f56(v0, f69())?;
                    let v6: List<Bytes> = f55(v4.clone(), f8(Str::lit("at")));
                    let v7: List<Bytes> = f61(f55(v5.clone(), f8(Str::lit("at"))))?;
                    let a192 = {
                        let a191 = v6.clone();
                        length_list(a191)
                    };
                    let a193 = 0u64;
                    if if nat_eq(a192, a193) {
                        true
                    } else {
                        f58({
                            let a196 = {
                                let a194 = f61(f48(v6.clone(), v7.clone()))?;
                                length_list(a194)
                            };
                            let a197 = {
                                let a195 = f48(v6.clone(), v7);
                                length_list(a195)
                            };
                            nat_eq(a196, a197)
                        })
                    } {
                        Ok(Adt11::C4(f8(Str::lit("a decision is repeated, or is already recorded\u{a}"))))
                    } else {
                        Ok(Adt11::C3(List::cons(f97(f69(), f98(v5, v4))?, List::<(Bytes, Bytes)>::nil()), f7(List::cons(f8(Str::lit("recorded the owner's decisions of ")), List::cons(f91(v6, f8(Str::lit(" "))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f38(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m198 = f44(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m198.uncons() {
        None => {
            let m199 = f100(v0, v1)?;
            match m199 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the branch rules are not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f107(f105(Some(v4))) {
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

pub fn f39(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m200 = f44(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/register-lock.json")), List::<Bytes>::nil()))));
    match m200.uncons() {
        None => {
            let m201 = f100(v0.clone(), v1.clone())?;
            match m201 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the binding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    if f58(f26(f104(v4.clone(), f8(Str::lit("schema"))), f108())) {
                        Ok(Adt11::C4(f8(Str::lit("the binding is not prismpm/authority-binding/1\u{a}"))))
                    } else {
                        let v5: Bytes = f104(v4.clone(), f8(Str::lit("id")));
                        let v7: Adt14 = {
                            let m202 = f101(v4.clone(), f8(Str::lit("source")));
                            match m202 {
                                None => {
                                    Adt14::C5
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        let v8: Option<Bytes> = f109(f101(v7.clone(), f8(Str::lit("sha256"))));
                        let v9: Bytes = f110(f8(Str::lit("authorities")), v5.clone());
                        let v10: (Bytes, Bytes) = (f110(v9.clone(), f8(Str::lit("binding.json"))), f53(v0.clone(), v1));
                        let m203 = v8;
                        match m203 {
                            None => {
                                f111(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                            }
                            Some(v11) => {
                                if f58(f26(f104(v7.clone(), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable")))) {
                                    f111(v0, v4, v5, v10, None::<(Bytes, Bytes)>)
                                } else {
                                    let v12: Bytes = f110(v9, f112(f104(v7.clone(), f8(Str::lit("url"))))?);
                                    if f58(f67(v12.clone(), match v0.clone() {
                                        Adt1::C0(_, _, h204) => {
                                            h204
                                        }
                                    })) {
                                        Ok(Adt11::C2(f8(Str::lit("curl")), List::cons(f8(Str::lit("-fsSL")), List::cons(f8(Str::lit("--create-dirs")), List::cons(f8(Str::lit("-o")), List::cons(v12.clone(), List::cons(f104(v7, f8(Str::lit("url"))), List::<Bytes>::nil()))))), v12))
                                    } else {
                                        let m205 = f44(v0.clone(), List::cons(v12.clone(), List::<Bytes>::nil()));
                                        match m205.uncons() {
                                            None => {
                                                if f26(f88(f53(v0.clone(), v12.clone()))?, v11.clone()) {
                                                    f111(v0, v4, v5, v10, Some((v12, v11)))
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

pub fn f40(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt11> {
    let m206 = f44(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m206.uncons() {
        None => {
            let m207 = f100(v0.clone(), v1.clone())?;
            match m207 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f102(v0.clone(), f53(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt11::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f41(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a208 = v0.clone();
        length_bytes(a208)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h209, _) = f113(Fn0::F114(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h209
    };
    let v4: u64 = {
        let (h210, _) = {
            let (_, h211) = v3.clone();
            h211
        };
        h210
    };
    Ok(f115(List::cons(f43(v0, v4, {
        let a212 = v2;
        let a213 = v4;
        nat_sub(a212, a213)
    }), {
        let (_, h214) = {
            let (_, h215) = v3;
            h215
        };
        h214
    })))
}

pub fn f42(p219: List<Bytes>) -> Bytes {
    let m218 = p219;
    {
        let mut m218 = m218;
        loop {
            let step = {
                let v0 = m218;
                let m216 = v0;
                match m216.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m217 = v2.clone();
                        match m217.uncons() {
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
                Ok(next) => { m218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m223 = {
        let a220 = v0;
        let a221 = v1;
        let a222 = v2;
        slice_bytes(a220, a221, a222)
    };
    match m223 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f44(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m224 = v1;
    match m224.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m226 = f116(match v0.clone() {
                Adt1::C0(h225, _, _) => {
                    h225
                }
            }, v2.clone());
            match m226 {
                None => {
                    List::cons(v2, f44(v0, v3))
                }
                Some(_) => {
                    f44(v0, v3)
                }
            }
        }
    }
}

pub fn f45() -> List<Bytes> {
    List::cons(f8(Str::lit("model/authorities.toml")), List::cons(f8(Str::lit("model/ids.toml")), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f78(), List::<Bytes>::nil())))))))
}

pub fn f46(v0: Adt1, v1: List<Bytes>) -> R<List<Bytes>> {
    let m227 = v1;
    match m227.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if if f117(v0.clone(), v2.clone()) {
                f118(v2.clone(), f8(Str::lit(".json")))
            } else {
                false
            } {
                let m228 = f100(v0.clone(), v2.clone())?;
                match m228 {
                    None => {
                        Ok(List::cons(v2, f46(v0, v3)?))
                    }
                    Some(_) => {
                        f46(v0, v3)
                    }
                }
            } else {
                f46(v0, v3)
            }
        }
    }
}

pub fn f47(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>) -> List<Bytes> {
    let m229 = v1;
    match m229.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f59(v0.clone(), v2.clone()) {
                f47(v0, v3)
            } else {
                List::cons(v2, f47(v0, v3))
            }
        }
    }
}

pub fn f48(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m230 = v0;
    match m230.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f48(v3, v1))
        }
    }
}

pub fn f49() -> List<Bytes> {
    List::cons(f8(Str::lit("app/src")), List::cons(f8(Str::lit("studio/src")), List::cons(f8(Str::lit("stdlib/src")), List::<Bytes>::nil())))
}

pub fn f50(v0: List<(Bytes, List<Bytes>)>, v1: bool) -> List<Bytes> {
    let m231 = v0;
    match m231.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            f48(f119({
                let (h232, _) = v2;
                h232
            }, {
                let (_, h233) = v2;
                h233
            }, v1), f50(v3, v1))
        }
    }
}

pub fn f51(v0: Adt13) -> R<(bool, Bytes)> {
    let v1: Adt12 = match v0.clone() {
        Adt13::C0(h234, _, _, _, _) => {
            h234
        }
    };
    let v2: Adt15 = f120(v1.clone())?;
    let v3: List<Bytes> = f121(v1.clone());
    let v4: List<Bytes> = f55(match v0.clone() {
        Adt13::C0(_, _, h235, _, _) => {
            h235
        }
    }, f8(Str::lit("path")));
    let m237 = f122(v2, List::<u64>::nil(), f123(v1.clone(), match v0.clone() {
        Adt13::C0(_, h236, _, _, _) => {
            h236
        }
    }, 0u64)?);
    match m237 {
        None => {
            if f125(f126(match v0.clone() {
                Adt13::C0(_, _, _, _, h239) => {
                    h239
                }
            }, f48(match v0.clone() {
                Adt13::C0(_, _, _, _, h240) => {
                    h240
                }
            }, v4.clone()))?, f126(v4.clone(), f48(match v0.clone() {
                Adt13::C0(_, _, _, _, h241) => {
                    h241
                }
            }, v4))?, f127(match v0.clone() {
                Adt13::C0(_, _, h242, _, _) => {
                    h242
                }
            })) {
                let m246 = f128(f126(match v0.clone() {
                    Adt13::C0(_, _, _, h243, _) => {
                        h243
                    }
                }, f48(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h244, _) => {
                        h244
                    }
                }))?, f126(v3.clone(), f48(v3.clone(), match v0.clone() {
                    Adt13::C0(_, _, _, h245, _) => {
                        h245
                    }
                }))?);
                match m246 {
                    None => {
                        Ok((true, f7(List::cons(f8(Str::lit("admitted: ")), List::cons(f91(List::cons(f8(Str::lit("constructs")), List::<Bytes>::nil()), Bytes::lit(&[])), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok((false, f7(List::cons(f8(Str::lit("register entry dropped: ")), List::cons(f124(f48(v3, match v0 {
                            Adt13::C0(_, _, _, h247, _) => {
                                h247
                            }
                        }), v6), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                Ok((false, f8(Str::lit("ungrounded module: every module needs one stratum row and every row one module\u{a}"))))
            }
        }
        Some(v5) => {
            Ok((false, f7(List::cons(f8(Str::lit("unresolved citation: ")), List::cons(f124(match v1 {
                Adt12::C0(_, _, _, _, h238) => {
                    h238
                }
            }, v5), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f52(v0: Bytes) -> R<List<Bytes>> {
    f129(f93(v0)?, f8(Str::lit("id = \"")))
}

pub fn f53(v0: Adt1, v1: Bytes) -> Bytes {
    let m249 = f116(match v0 {
        Adt1::C0(h248, _, _) => {
            h248
        }
    }, v1);
    match m249 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m250 = v2;
            match m250 {
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

pub fn f54(v0: Adt1) -> R<List<Bytes>> {
    let m251 = f82(f53(v0, f78()))?;
    match m251.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((_, v2)) => {
            Ok(f80(v2))
        }
    }
}

pub fn f55(v0: List<Adt14>, v1: Bytes) -> List<Bytes> {
    let m252 = v0;
    match m252.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f104(v2, v1.clone()), f55(v3, v1))
        }
    }
}

pub fn f56(v0: Adt1, v1: Bytes) -> R<List<Adt14>> {
    Ok(f105(f100(v0, v1)?))
}

pub fn f57(v0: List<Adt14>) -> List<Bytes> {
    let m253 = v0;
    match m253.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m254 = f109(Some(v1));
            match m254 {
                None => {
                    f57(v2)
                }
                Some(v3) => {
                    List::cons(v3, f57(v2))
                }
            }
        }
    }
}

pub fn f58(v0: bool) -> bool {
    !v0
}

pub fn f59(p258: List<(Bytes, List<Bytes>)>, v1: Bytes) -> bool {
    let m257 = p258;
    {
        let mut m257 = m257;
        loop {
            let step = {
                let v0 = m257;
                let m255 = v0;
                match m255.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h256, _) = v2;
                            h256
                        }, v1.clone()) {
                            Err::<List<(Bytes, List<Bytes>)>, bool>(true)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, bool>(v3)
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

pub fn f60() -> Bytes {
    f8(Str::lit("model/bindings"))
}

pub fn f61(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f130(f131(Fn1::F132, List::<Bytes>::nil(), f133(v0)?)))
}

pub fn f62(v0: Bytes, v1: List<Bytes>) -> List<Bytes> {
    let m259 = v1;
    match m259.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f118(v2.clone(), f8(Str::lit(".json"))) {
                List::cons(f110(v0.clone(), v2), f62(v0, v3))
            } else {
                f62(v0, v3)
            }
        }
    }
}

pub fn f63(p264: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m263 = p264;
    {
        let mut m263 = m263;
        loop {
            let step = {
                let v0 = m263;
                let m260 = v0;
                match m260.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h261, _) = v2;
                            h261
                        }, v1.clone()) {
                            let (_, h262) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h262)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m263 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Adt1, p267: List<Bytes>) -> R<Option<Bytes>> {
    let m266 = p267;
    {
        let mut m266 = m266;
        loop {
            let step = {
                let v1 = m266;
                let m265 = v1;
                match m265.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        if f134(v0.clone(), v2.clone())? {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v2)))
                        } else {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m268 = v0;
    match m268.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f67(v2.clone(), v1.clone()) {
                f65(v3, v1)
            } else {
                List::cons(v2, f65(v3, v1))
            }
        }
    }
}

pub fn f66(v0: Adt1, v1: List<Bytes>) -> R<Adt11> {
    if f58(f59(match v0.clone() {
        Adt1::C0(_, h269, _) => {
            h269
        }
    }, f135())) {
        Ok(Adt11::C1(List::cons(f135(), List::<Bytes>::nil())))
    } else {
        let v2: List<Bytes> = f65(f61(f62(f135(), f63(match v0.clone() {
            Adt1::C0(_, h270, _) => {
                h270
            }
        }, f135())))?, v1);
        let m271 = f44(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), v2.clone()));
        match m271.uncons() {
            None => {
                let m272 = f136(v0.clone(), v2, f55(f56(v0, f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct"))))?;
                match m272 {
                    None => {
                        Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[])))
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

pub fn f67(v0: Bytes, p275: List<Bytes>) -> bool {
    let m274 = p275;
    {
        let mut m274 = m274;
        loop {
            let step = {
                let v1 = m274;
                let m273 = v1;
                match m273.uncons() {
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
                Ok(next) => { m274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Str) -> List<Bytes> {
    List::cons(f8(Str::lit("--yes")), List::cons(f8(Str::lit("prism-code@0.11.1")), List::cons(f8(v0), List::cons(f8(Str::lit("--project")), List::cons(f8(Str::lit("app")), List::cons(f8(Str::lit(".")), List::<Bytes>::nil()))))))
}

pub fn f69() -> Bytes {
    f8(Str::lit("model/owner-decisions.json"))
}

pub fn f70(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m276 = v0;
    match m276.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f70(v3, v1))
        }
    }
}

pub fn f71(v0: Adt1) -> R<List<(Bytes, Bytes)>> {
    let m277 = f56(v0.clone(), f69())?;
    match m277.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((_, _)) => {
            Ok(List::cons((f8(Str::lit("docs/owner-decisions.md")), f137(v0)?), List::<(Bytes, Bytes)>::nil()))
        }
    }
}

pub fn f72() -> Bytes {
    f8(Str::lit(".github/workflows/pages.yml"))
}

pub fn f73() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}name: pages\u{a}\u{a}on:\u{a}  pull_request:\u{a}  push:\u{a}    branches: [main]\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}concurrency:\u{a}  group: pages-${{ github.ref }}\u{a}  cancel-in-progress: true\u{a}\u{a}jobs:\u{a}  studio:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    permissions:\u{a}      contents: read\u{a}      pull-requests: read\u{a}    steps:\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          persist-credentials: false\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: check, build and publish through the API\u{a}        run: |\u{a}          set -euo pipefail\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}          if ! git diff --quiet; then echo \"::error::publish changed $(git diff --name-only | tr '\\n' ' '): run every operation through operate, which ends with check, build and publish\"; exit 1; fi\u{a}      - name: accept only pull requests made through the API (last, so every seal step reports first)\u{a}        if: github.event_name == 'pull_request'\u{a}        env:\u{a}          GH_TOKEN: ${{ github.token }}\u{a}          PR: ${{ github.event.pull_request.number }}\u{a}          AUTHOR: ${{ github.event.pull_request.user.login }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          app='afflom-verdant-studio[bot]'\u{a}          if [ \"$AUTHOR\" != \"$app\" ]; then\u{a}            echo \"::error title=provenance::$AUTHOR opened this pull request, not $app through operate. Every seal step above passed; only the provenance gate refuses it.\"\u{a}            exit 1\u{a}          fi\u{a}          others=$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${PR}/commits\" --paginate --jq \".[] | select(.author.login != \\\"$app\\\") | .sha\")\u{a}          if [ -n \"$others\" ]; then\u{a}            echo \"::error::these commits were not made by $app: $others\"\u{a}            exit 1\u{a}          fi\u{a}      - if: github.event_name == 'push'\u{a}        uses: actions/upload-pages-artifact@56afc609e74202658d3ffba0e8f6dda462b719fa # v3.0.1\u{a}        with:\u{a}          path: app/.prism/site\u{a}\u{a}  pages:\u{a}    if: github.event_name == 'push'\u{a}    needs: studio\u{a}    runs-on: ubuntu-24.04\u{a}    permissions:\u{a}      pages: write\u{a}      id-token: write\u{a}    environment:\u{a}      name: github-pages\u{a}      url: ${{ steps.deployment.outputs.page_url }}\u{a}    steps:\u{a}      - id: deployment\u{a}        uses: actions/deploy-pages@d6db90164ac5ed86f2b6aed7e0febac5b3c0c03e # v4.0.5\u{a}"))
}

pub fn f74() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# The one way an agent changes this repository: dispatch this workflow with an\u{a}# operation of the API. It runs the certified command, and opens a pull request\u{a}# as the Verdant Studio GitHub App for the owner to review.\u{a}name: operate\u{a}\u{a}on:\u{a}  push:\u{a}    branches: [main]\u{a}    paths: ['model/bindings/**', 'model/encodings/**', 'model/authorities.toml']\u{a}  workflow_dispatch:\u{a}    inputs:\u{a}      args:\u{a}        description: 'The operation and its arguments, e.g. \"stage-bindings target/input/bindings.jsonl\", or \"advance\" to run every operation that next derives'\u{a}        required: true\u{a}        default: 'advance'\u{a}        type: string\u{a}      input:\u{a}        description: 'Optional content, written to target/input/<input_name> before the operation runs'\u{a}        required: false\u{a}        type: string\u{a}      input_name:\u{a}        description: 'File name for input'\u{a}        required: false\u{a}        type: string\u{a}        default: 'input'\u{a}\u{a}permissions:\u{a}  contents: read\u{a}\u{a}# Only operations that write the registers share one group. GitHub keeps one\u{a}# pending run per group, so a staging or propose-command run gets its own.\u{a}concurrency:\u{a}  group: ${{ (startsWith(inputs.args, 'stage-') || startsWith(inputs.args, 'propose-command')) && format('operate-{0}', github.run_id) || 'operate' }}\u{a}  cancel-in-progress: false\u{a}\u{a}jobs:\u{a}  operate:\u{a}    runs-on: ubuntu-24.04\u{a}    timeout-minutes: 30\u{a}    steps:\u{a}      - id: app\u{a}        uses: actions/create-github-app-token@5d869da34e18e7287c1daad50e0b8ea0f506ce69 # v1.11.0\u{a}        with:\u{a}          app-id: ${{ vars.VERDANT_STUDIO_APP_ID }}\u{a}          private-key: ${{ secrets.VERDANT_STUDIO_APP_KEY }}\u{a}      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\u{a}        with:\u{a}          token: ${{ steps.app.outputs.token }}\u{a}      - uses: actions/setup-node@249970729cb0ef3589644e2896645e5dc5ba9c38\u{a}        with:\u{a}          node-version: '24'\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.elan\u{a}          key: lean-4.32.2-elan-4.1.2\u{a}      - if: inputs.args == 'propose-command'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: apply a change to the API's own sources, then export it again with Lean (propose-command only)\u{a}        if: inputs.args == 'propose-command'\u{a}        env:\u{a}          INPUT: ${{ inputs.input }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          mkdir -p target/input\u{a}          cat > target/input/change.patch <<< \"$INPUT\"\u{a}          outside=$(git apply --numstat target/input/change.patch | cut -f3 | grep -v -e '^studio/src/' -e '^README.md$' || true)\u{a}          if [ -n \"$outside\" ]; then echo \"::error::propose-command changes only studio/src and README.md, not: $outside\"; exit 1; fi\u{a}          git apply target/input/change.patch\u{a}          if [ ! -x \"$HOME/.elan/bin/elan\" ]; then\u{a}            curl -sSfL -o target/elan.tgz https://github.com/leanprover/elan/releases/download/v4.1.2/elan-x86_64-unknown-linux-gnu.tar.gz\u{a}            echo \"f81c2e48c1588d4612cd2c8851947898a45ac8d72748a07dff3a5694f1cf589b  target/elan.tgz\" | sha256sum -c -\u{a}            tar -xzf target/elan.tgz -C target\u{a}            target/elan-init -y --no-modify-path --default-toolchain leanprover/lean4:v4.32.2\u{a}          fi\u{a}          export PATH=\"$HOME/.elan/bin:$PATH\"\u{a}          test -d \"$HOME/.elan/toolchains/leanprover--lean4---v4.32.2\" || elan toolchain install leanprover/lean4:v4.32.2\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          ll=\"$PWD/target/lexlean/bin/lexlean\"\u{a}          if ! (cd studio && \"$ll\" lock && \"$ll\" check && \"$ll\" production build --request production-request.json) > target/export.log 2>&1; then\u{a}            grep -m 20 'error' target/export.log | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          rm -rf certified\u{a}          cp -r studio/.lexlean/production/*/ certified\u{a}          rm -f certified/packages/rust-std/verdant_studio/Cargo.lock\u{a}          { echo \"proposed a change to the API's sources, exported again as $(basename studio/.lexlean/production/*/)\"; git diff --stat -- studio/src README.md; } > target/operation.txt\u{a}      - id: command\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/release/verdant_studio\u{a}          key: verdant-studio-${{ hashFiles('certified/**') }}\u{a}      - uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: ~/.npm\u{a}          key: npm-prism-code-0.11.1\u{a}      - if: steps.command.outputs.cache-hit != 'true'\u{a}        uses: actions/cache@5a3ec84eff668545956fd18022155c47e93e2684 # v4.2.3\u{a}        with:\u{a}          path: target/lexlean\u{a}          key: lexlean-0.4.3\u{a}      - name: replay the certified bundle, then build its command (only when certified/ changed)\u{a}        if: steps.command.outputs.cache-hit != 'true'\u{a}        run: |\u{a}          set -euo pipefail\u{a}          test -x target/lexlean/bin/lexlean || cargo install --locked --version 0.4.3 lexlean --root target/lexlean\u{a}          target/lexlean/bin/lexlean production verify-artifact certified\u{a}          cargo build --release --manifest-path certified/packages/rust-std/verdant_studio/Cargo.toml --target-dir target\u{a}      - name: run the operation, then check, build and publish through the API, as the studio check does\u{a}        env:\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}          INPUT: ${{ inputs.input }}\u{a}          INPUT_NAME: ${{ inputs.input_name }}\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}        run: |\u{a}          set -euo pipefail -f\u{a}          if [ \"$ARGS\" = advance ]; then\u{a}            for pr in $(gh api \"repos/${GITHUB_REPOSITORY}/pulls?state=open&per_page=100\" --jq '.[] | select(.title == \"verdant_studio advance\") | .number'); do\u{a}              if [ \"$(gh api \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" --jq .mergeable_state)\" = dirty ]; then\u{a}                gh api --silent -X PATCH \"repos/${GITHUB_REPOSITORY}/pulls/${pr}\" -f state=closed\u{a}                echo \"::notice::closed advance pull request #${pr}: it conflicts with main, and this run supersedes it\"\u{a}              else\u{a}                echo \"::notice::advance pull request #${pr} is open; its merge changes model/authorities.toml, which runs advance again\"\u{a}                exit 0\u{a}              fi\u{a}            done\u{a}          fi\u{a}          if [ -n \"$INPUT\" ] && [ \"$ARGS\" != propose-command ]; then\u{a}            case \"$INPUT_NAME\" in */*|.*|'') echo \"input_name must be a plain file name\" >&2; exit 1;; esac\u{a}            mkdir -p target/input; cat > \"target/input/$INPUT_NAME\" <<< \"$INPUT\"\u{a}          fi\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            cat target/operation.txt\u{a}          elif [ \"$ARGS\" = advance ]; then\u{a}            : > target/operation.txt\u{a}            last=''\u{a}            skipped=''\u{a}            while op=$(target/release/verdant_studio next $skipped) && [ -n \"$op\" ]; do\u{a}              if [ \"$op\" = \"$last\" ]; then echo \"::error::$op did not settle\"; exit 1; fi\u{a}              if target/release/verdant_studio $op | tee -a target/operation.txt; then\u{a}                last=$op\u{a}              else\u{a}                binding=${op#* }\u{a}                echo \"::warning::$op was refused; the other bindings continue\"\u{a}                echo \"refused: $op\" >> target/operation.txt\u{a}                git clean -fdq -- \"authorities/$(basename \"$binding\" .json)\"\u{a}                skipped=\"$skipped $binding\"\u{a}              fi\u{a}            done\u{a}          else\u{a}            target/release/verdant_studio $ARGS | tee target/operation.txt\u{a}          fi\u{a}          target/release/verdant_studio check\u{a}          target/release/verdant_studio build\u{a}          target/release/verdant_studio publish\u{a}      - name: open a pull request as the Verdant Studio app\u{a}        env:\u{a}          GH_TOKEN: ${{ steps.app.outputs.token }}\u{a}          SLUG: ${{ steps.app.outputs.app-slug }}\u{a}          ARGS: ${{ inputs.args || 'advance' }}\u{a}        run: |\u{a}          set -euo pipefail\u{a}          git add -A\u{a}          if git diff --cached --quiet; then echo \"no change\"; exit 0; fi\u{a}          id=$(gh api \"users/${SLUG}[bot]\" --jq .id)\u{a}          git -c user.name=\"${SLUG}[bot]\" -c user.email=\"${id}+${SLUG}[bot]@users.noreply.github.com\" commit -q -F - <<MSG\u{a}          verdant_studio $ARGS\u{a}\u{a}          $(cat target/operation.txt)\u{a}          MSG\u{a}          branch=\"studio/${GITHUB_RUN_ID}\"\u{a}          if ! git push origin \"HEAD:refs/heads/${branch}\" 2> target/push.err; then\u{a}            grep -E 'GH013|error|rejected|path:' target/push.err | head -20 | sed 's/^/::error::/'\u{a}            exit 1\u{a}          fi\u{a}          gh pr create --head \"$branch\" --base main --title \"verdant_studio $ARGS\" --body-file target/operation.txt\u{a}          gh api \"repos/${GITHUB_REPOSITORY}/rules/branches/main\" > target/rules.json\u{a}          if [ \"$ARGS\" = propose-command ]; then\u{a}            echo \"::notice::a change to the API itself waits for the owner's review\"\u{a}          elif [ \"$(target/release/verdant_studio merge-gate target/rules.json)\" = merge ]; then\u{a}            gh pr merge \"$branch\" --auto --squash --delete-branch\u{a}          else\u{a}            echo \"::notice::main does not require the studio check, so this pull request waits for the owner's merge\"\u{a}          fi\u{a}"))
}

pub fn f75() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish`. Every change needs the owner's review.\u{a}* @afflom\u{a}"))
}

pub fn f76(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# The Verdant Studio model\u{a}\u{a}Generated by `verdant_studio publish` from model/requirements.json and model/admissions.json. Do not edit.\u{a}\u{a}## Requirements\u{a}\u{a}")), List::cons(f7(f138(f56(v0.clone(), f8(Str::lit("model/requirements.json")))?)), List::cons(f8(Str::lit("\u{a}## Admissions\u{a}\u{a}Each construct and the citation that admits it.\u{a}\u{a}")), List::cons(f7(f139(f56(v0, f8(Str::lit("model/admissions.json")))?)), List::<Bytes>::nil()))))))
}

pub fn f77() -> Bytes {
    f8(Str::lit("# Generated by `verdant-studio publish` from studio/src/Studio/Ops.lex.tex. Do not edit.\u{a}# authorities/ holds official corpora, fetched from their publishers and pinned\u{a}# by sha256. Their text can match a secret pattern by chance (UK GDPR XML matches\u{a}# the Mailgun key pattern), which would block every ingest.\u{a}paths-ignore:\u{a}  - \"authorities/**\"\u{a}"))
}

pub fn f78() -> Bytes {
    f8(Str::lit("catalog/standards-index.csv"))
}

pub fn f79(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m278 = v0;
    match m278.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f67(v2.clone(), v1.clone()) {
                f79(v3, v1)
            } else {
                List::cons(f7(List::cons(f8(Str::lit("uncited-row ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), f79(v3, v1))
            }
        }
    }
}

pub fn f80(v0: List<List<Bytes>>) -> List<Bytes> {
    let m279 = v0;
    match m279.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m280 = v1;
            match m280.uncons() {
                None => {
                    f80(v2)
                }
                Some((v3, _)) => {
                    List::cons(v3, f80(v2))
                }
            }
        }
    }
}

pub fn f81(v0: List<List<Bytes>>) -> List<List<Bytes>> {
    let m281 = v0;
    match m281.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f82(v0: Bytes) -> R<List<List<Bytes>>> {
    let v1: Adt16 = f140(v0.clone(), 0u64, {
        let a282 = v0;
        length_bytes(a282)
    }, Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::<List<Bytes>>::nil()))?;
    let v4: Adt16 = {
        let a285 = {
            let a284 = match v1.clone() {
                Adt16::C0(_, _, h283, _, _) => {
                    h283
                }
            };
            length_bytes(a284)
        };
        let a286 = 0u64;
        if if nat_eq(a285, a286) {
            let m288 = match v1.clone() {
                Adt16::C0(_, _, _, h287, _) => {
                    h287
                }
            };
            match m288.uncons() {
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
            f141(v1)
        }
    };
    Ok(f142(match v4 {
        Adt16::C0(_, _, _, _, h289) => {
            h289
        }
    }, List::<List<Bytes>>::nil()))
}

pub fn f83(v0: List<Adt14>) -> List<Bytes> {
    let m290 = v0;
    match m290.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m291 = f101(v1, f8(Str::lit("std")));
            match m291 {
                None => {
                    f83(v2)
                }
                Some(v3) => {
                    List::cons(f104(v3, f8(Str::lit("row"))), f83(v2))
                }
            }
        }
    }
}

pub fn f84(p296: List<Bytes>, p297: List<Bytes>) -> bool {
    let m295 = (p296, p297);
    {
        let mut m295 = m295;
        loop {
            let step = {
                let (v0, v1) = m295;
                let m292 = v0;
                match m292.uncons() {
                    None => {
                        let m293 = v1;
                        match m293.uncons() {
                            None => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<Bytes>, List<Bytes>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m294 = v1;
                        match m294.uncons() {
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
                Ok(next) => { m295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: List<List<Bytes>>) -> List<Bytes> {
    let m298 = v0;
    match m298.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, _)) => {
            v1
        }
    }
}

pub fn f86() -> List<Bytes> {
    List::cons(f8(Str::lit("id")), List::cons(f8(Str::lit("domain")), List::cons(f8(Str::lit("name")), List::cons(f8(Str::lit("owner")), List::cons(f8(Str::lit("phases")), List::cons(f8(Str::lit("scope")), List::cons(f8(Str::lit("adoption")), List::cons(f8(Str::lit("adoption_code")), List::cons(f8(Str::lit("source")), List::<Bytes>::nil())))))))))
}

pub fn f87(v0: Str, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(v0), List::cons(f8(Str::lit(" = ")), List::cons(f143(v1)?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f88(v0: Bytes) -> R<Bytes> {
    Ok(f144(f145(v0)?))
}

pub fn f89(v0: Bytes, v1: Bytes, v2: Bytes) -> R<Bytes> {
    let v3: List<Bytes> = f146(f147(f93(v0)?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), v1, List::<Bytes>::nil())?;
    Ok(f7(List::cons(f148(f149(v3, Bytes::lit(&[0x0a])))?, List::cons(f8(Str::lit("\u{a}\u{a}")), List::cons(v2, List::<Bytes>::nil())))))
}

pub fn f90(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f103(Adt14::C3(std::rc::Rc::new(f150(f61(f48(v1, f57(f56(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f91(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m299 = f151(Fn2::F152(v1), None::<Bytes>, v0);
    match m299 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f92(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m300 = v0;
    match m300.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f153(v1)?;
            let a302 = {
                let a301 = v3.clone();
                length_bytes(a301)
            };
            let a303 = 0u64;
            if nat_eq(a302, a303) {
                f92(v2)
            } else {
                let m304 = f154(v3.clone())?;
                match m304 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        let v5: Bytes = f104(v4.clone(), f8(Str::lit("id")));
                        if if f26(f104(v4, f8(Str::lit("schema"))), f108()) {
                            f58({
                                let a306 = {
                                    let a305 = v5.clone();
                                    length_bytes(a305)
                                };
                                let a307 = 0u64;
                                nat_eq(a306, a307)
                            })
                        } else {
                            false
                        } {
                            let m308 = f92(v2)?;
                            match m308 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v6) => {
                                    Ok(Some(List::cons((f155(v5), {
                                        let a309 = v3;
                                        let a310 = Bytes::lit(&[0x0a]);
                                        append_bytes(a309, a310)
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

pub fn f93(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a311 = v0.clone();
        length_bytes(a311)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h312, _) = f113(Fn0::F156(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h312
    };
    let v3: u64 = {
        let (h313, _) = {
            let (_, h314) = v2.clone();
            h314
        };
        h313
    };
    let v4: List<Bytes> = {
        let (_, h315) = {
            let (_, h316) = v2;
            h316
        };
        h315
    };
    let a317 = v3;
    let a318 = v1;
    if nat_lt(a317, a318) {
        Ok(f115(List::cons(f43(v0, v3, {
            let a319 = v1;
            let a320 = v3;
            nat_sub(a319, a320)
        }), v4)))
    } else {
        Ok(f115(v4))
    }
}

pub fn f94(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m321 = v0;
    match m321.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h322, _) = v1;
                h322
            }, f94(v2))
        }
    }
}

pub fn f95(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt14>>> {
    let m323 = v0;
    match m323.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f153(v2)?;
            let a325 = {
                let a324 = v4.clone();
                length_bytes(a324)
            };
            let a326 = 0u64;
            if nat_eq(a325, a326) {
                f95(v3, v1)
            } else {
                let m327 = f154(v4)?;
                match m327 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f157(v5.clone());
                        let a329 = {
                            let a328 = v6.clone();
                            length_bytes(a328)
                        };
                        let a330 = 0u64;
                        if nat_eq(a329, a330) {
                            Ok(None::<List<Adt14>>)
                        } else {
                            let m331 = f95(v3, v1.clone())?;
                            match m331 {
                                None => {
                                    Ok(None::<List<Adt14>>)
                                }
                                Some(v7) => {
                                    if f26(v6, v1) {
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

pub fn f96() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f97(v0: Bytes, v1: List<Adt14>) -> R<(Bytes, Bytes)> {
    Ok((v0, f103(Adt14::C3(std::rc::Rc::new(f158(v1))))?))
}

pub fn f98(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m332 = v0;
    match m332.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f98(v3, v1))
        }
    }
}

pub fn f99(v0: List<Bytes>) -> R<Option<List<(Bytes, Bytes)>>> {
    let m333 = v0;
    match m333.uncons() {
        None => {
            Ok(Some(List::<(Bytes, Bytes)>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f153(v1)?;
            let a335 = {
                let a334 = v3.clone();
                length_bytes(a334)
            };
            let a336 = 0u64;
            if nat_eq(a335, a336) {
                f99(v2)
            } else {
                let m337 = f154(v3.clone())?;
                match m337 {
                    None => {
                        Ok(None::<List<(Bytes, Bytes)>>)
                    }
                    Some(v4) => {
                        if f159(v4.clone()) {
                            let m338 = f99(v2)?;
                            match m338 {
                                None => {
                                    Ok(None::<List<(Bytes, Bytes)>>)
                                }
                                Some(v5) => {
                                    Ok(Some(List::cons((f7(List::cons(f135(), List::cons(f8(Str::lit("/")), List::cons(f160(f104(v4, f8(Str::lit("name"))))?, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))), {
                                        let a339 = v3;
                                        let a340 = Bytes::lit(&[0x0a]);
                                        append_bytes(a339, a340)
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

pub fn f100(v0: Adt1, v1: Bytes) -> R<Option<Adt14>> {
    f154(f53(v0, v1))
}

pub fn f101(p346: Adt14, v1: Bytes) -> Option<Adt14> {
    let m345 = p346;
    {
        let mut m345 = m345;
        loop {
            let step = {
                let v0 = m345;
                let m341 = v0;
                match m341 {
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
                    Adt14::C4(r342) => {
                        let v5 = (*r342).clone();
                        Ok::<Adt14, Option<Adt14>>(v5)
                    }
                    Adt14::C5 => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C6(_, _) => {
                        Err::<Adt14, Option<Adt14>>(None::<Adt14>)
                    }
                    Adt14::C7(v2, r343, r344) => {
                        let v3 = (*r343).clone();
                        let v4 = (*r344).clone();
                        if f26(v2, v1.clone()) {
                            Err::<Adt14, Option<Adt14>>(Some(v3))
                        } else {
                            Ok::<Adt14, Option<Adt14>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f102(v0: Adt1, v1: Bytes, v2: Adt14, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt11> {
    let m347 = f44(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m347.uncons() {
        None => {
            let m348 = f161(v4);
            match m348 {
                None => {
                    Ok(Adt11::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a349 = f8(Str::lit("Catalog."));
                        let a350 = f160(v3.clone())?;
                        append_bytes(a349, a350)
                    };
                    let v10: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f160(v3.clone())?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
                    let m351 = f162(v2, v9.clone(), f160(v3.clone())?)?;
                    match m351 {
                        Ok(v12) => {
                            Ok(Adt11::C3(List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f160(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::cons((v10.clone(), v12), List::cons((f8(Str::lit("model/admissions.json")), f103(Adt14::C3(std::rc::Rc::new(f158(f163(f56(v0.clone(), f8(Str::lit("model/admissions.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("construct")), std::rc::Rc::new(Adt14::C2(v9.clone())), std::rc::Rc::new(v8)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f103(Adt14::C3(std::rc::Rc::new(f158(f163(f56(v0.clone(), f8(Str::lit("model/strata.json")))?, Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("path")), std::rc::Rc::new(Adt14::C2(v10.clone())), std::rc::Rc::new(Adt14::C7(f8(Str::lit("role")), std::rc::Rc::new(Adt14::C2(f8(Str::lit("Encoding entered through propose-encoding.")))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("stratum")), std::rc::Rc::new(Adt14::C1(false, 0u64)), std::rc::Rc::new(Adt14::C5)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
                                let a355 = f164(f53(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f43(v10.clone(), 4u64, {
                                    let a353 = {
                                        let a352 = v10;
                                        length_bytes(a352)
                                    };
                                    let a354 = 4u64;
                                    nat_sub(a353, a354)
                                }))?;
                                let a356 = Bytes::lit(&[0x0a]);
                                append_bytes(a355, a356)
                            }), List::cons(f90(v0, List::cons(v9.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v9, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
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

pub fn f103(v0: Adt14) -> R<Bytes> {
    let a357 = f165(v0)?;
    let a358 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a357, a358))
}

pub fn f104(v0: Adt14, v1: Bytes) -> Bytes {
    let m359 = f109(f101(v0, v1));
    match m359 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f105(v0: Option<Adt14>) -> List<Adt14> {
    let m360 = v0;
    match m360 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v1) => {
            f166(v1)
        }
    }
}

pub fn f106(v0: List<Bytes>) -> R<Option<List<Adt14>>> {
    let m361 = v0;
    match m361.uncons() {
        None => {
            Ok(Some(List::<Adt14>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f153(v1)?;
            let a363 = {
                let a362 = v3.clone();
                length_bytes(a362)
            };
            let a364 = 0u64;
            if nat_eq(a363, a364) {
                f106(v2)
            } else {
                let m365 = f154(v3)?;
                match m365 {
                    None => {
                        Ok(None::<List<Adt14>>)
                    }
                    Some(v4) => {
                        if if f167(v4.clone(), f8(Str::lit("agent"))) {
                            if f167(v4.clone(), f8(Str::lit("at"))) {
                                if f167(v4.clone(), f8(Str::lit("quote"))) {
                                    f168(v4.clone(), f8(Str::lit("requirements")))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m366 = f106(v2)?;
                            match m366 {
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

pub fn f107(p369: List<Adt14>) -> bool {
    let m368 = p369;
    {
        let mut m368 = m368;
        loop {
            let step = {
                let v0 = m368;
                let m367 = v0;
                match m367.uncons() {
                    None => {
                        Err::<List<Adt14>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        if if f26(f104(v1.clone(), f8(Str::lit("type"))), f8(Str::lit("required_status_checks"))) {
                            f67(f8(Str::lit("studio")), f55(f169(v1, f8(Str::lit("parameters")), f8(Str::lit("required_status_checks"))), f8(Str::lit("context"))))
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
                Ok(next) => { m368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108() -> Bytes {
    f8(Str::lit("prismpm/authority-binding/1"))
}

pub fn f109(v0: Option<Adt14>) -> Option<Bytes> {
    let m370 = v0;
    match m370 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m371 = v1;
            match m371 {
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

pub fn f110(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f111(v0: Adt1, v1: Adt14, v2: Bytes, v3: (Bytes, Bytes), v4: Option<(Bytes, Bytes)>) -> R<Adt11> {
    Ok(Adt11::C3(List::cons(v3, List::cons((f8(Str::lit("model/authorities.toml")), f89(f53(v0.clone(), f8(Str::lit("model/authorities.toml"))), v2.clone(), f170(v1, v4)?)?), List::cons(f90(v0, List::cons(v2.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("ingested authority ")), List::cons(v2, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f112(v0: Bytes) -> R<Bytes> {
    Ok(f42(f41(v0, 47u8)?))
}

pub fn f113(v0: Fn0, p376: u64, p377: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m375 = (p376, p377);
    {
        let mut m375 = m375;
        loop {
            let step = {
                let (v1, v2) = m375;
                let m372 = v1;
                if m372 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m372 - 1;
                    let m374 = {
                        let c373: Fn0 = v0.clone();
                        c373.apply(v2.clone())?
                    };
                    match m374 {
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
                Ok(next) => { m375 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f171(v1, v0, v2)
}

pub fn f115(v0: List<Bytes>) -> List<Bytes> {
    f131(Fn1::F172, List::<Bytes>::nil(), v0)
}

pub fn f116(p382: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m381 = p382;
    {
        let mut m381 = m381;
        loop {
            let step = {
                let v0 = m381;
                let m378 = v0;
                match m378.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f26({
                            let (h379, _) = v2;
                            h379
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h380) = v2;
                                h380
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
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

pub fn f117(v0: Adt1, v1: Bytes) -> bool {
    let m384 = f116(match v0 {
        Adt1::C0(h383, _, _) => {
            h383
        }
    }, v1);
    match m384 {
        None => {
            false
        }
        Some(v2) => {
            let m385 = v2;
            match m385 {
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

pub fn f118(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a386 = v0.clone();
        length_bytes(a386)
    };
    let v3: u64 = {
        let a387 = v1.clone();
        length_bytes(a387)
    };
    let a388 = v3;
    let a389 = v2;
    if nat_le(a388, a389) {
        let m395 = {
            let a392 = v0;
            let a393 = {
                let a390 = v2;
                let a391 = v3;
                nat_sub(a390, a391)
            };
            let a394 = v3;
            slice_bytes(a392, a393, a394)
        };
        match m395 {
            None => {
                false
            }
            Some(v4) => {
                let a396 = v4;
                let a397 = v1;
                equal(a396, a397)
            }
        }
    } else {
        false
    }
}

pub fn f119(v0: Bytes, v1: List<Bytes>, v2: bool) -> List<Bytes> {
    let m398 = v1;
    match m398.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if v2 {
                f173(v3.clone())
            } else {
                f58(f173(v3.clone()))
            } {
                List::cons(f110(v0.clone(), v3), f119(v0, v4, v2))
            } else {
                f119(v0, v4, v2)
            }
        }
    }
}

pub fn f120(v0: Adt12) -> R<Adt15> {
    Ok(Adt15::C0(f174(match v0.clone() {
        Adt12::C0(h399, _, _, _, _) => {
            h399
        }
    }, 0u64)?, f174(match v0.clone() {
        Adt12::C0(_, h400, _, _, _) => {
            h400
        }
    }, 0u64)?, f174(match v0.clone() {
        Adt12::C0(_, _, h401, _, _) => {
            h401
        }
    }, 0u64)?, f174(match v0 {
        Adt12::C0(_, _, _, h402, _) => {
            h402
        }
    }, 0u64)?))
}

pub fn f121(v0: Adt12) -> List<Bytes> {
    f48(match v0.clone() {
        Adt12::C0(h403, _, _, _, _) => {
            h403
        }
    }, f48(match v0.clone() {
        Adt12::C0(_, _, h404, _, _) => {
            h404
        }
    }, f48(match v0.clone() {
        Adt12::C0(_, _, _, h405, _) => {
            h405
        }
    }, match v0 {
        Adt12::C0(_, _, _, _, h406) => {
            h406
        }
    })))
}

pub fn f122(v0: Adt15, p412: List<u64>, p413: List<Adt17>) -> Option<u64> {
    let m411 = (p412, p413);
    {
        let mut m411 = m411;
        loop {
            let step = {
                let (v1, v2) = m411;
                let m407 = v2;
                match m407.uncons() {
                    None => {
                        Err::<(List<u64>, List<Adt17>), Option<u64>>(None::<u64>)
                    }
                    Some((v3, v4)) => {
                        if f175(v0.clone(), v1.clone(), match v3.clone() {
                            Adt17::C0(_, h408) => {
                                h408
                            }
                        }) {
                            Ok::<(List<u64>, List<Adt17>), Option<u64>>((List::cons(match v3 {
                                Adt17::C0(h409, _) => {
                                    h409
                                }
                            }, v1), v4))
                        } else {
                            Err::<(List<u64>, List<Adt17>), Option<u64>>(Some(match v3 {
                                Adt17::C0(h410, _) => {
                                    h410
                                }
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: Adt12, v1: List<Adt14>, v2: u64) -> R<List<Adt17>> {
    let m414 = v1;
    match m414.uncons() {
        None => {
            Ok(List::<Adt17>::nil())
        }
        Some((v3, v4)) => {
            Ok(List::cons(Adt17::C0(v2, f176(v0.clone(), v3)?), f123(v0, v4, {
                let a415 = v2;
                let a416 = 1u64;
                nat_add(a415, a416)?
            })?))
        }
    }
}

pub fn f124(p420: List<Bytes>, p421: u64) -> Bytes {
    let m419 = (p420, p421);
    {
        let mut m419 = m419;
        loop {
            let step = {
                let (v0, v1) = m419;
                let m417 = v0;
                match m417.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m418 = v1;
                        if m418 == 0 {
                            Err::<(List<Bytes>, u64), Bytes>(v2)
                        } else {
                            let v4 = m418 - 1;
                            Ok::<(List<Bytes>, u64), Bytes>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m419 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: List<u64>, v1: List<u64>, v2: List<u64>) -> bool {
    if f177(v0.clone(), v1.clone()) {
        if f177(v1, v0) {
            f178(v2)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f126(v0: List<Bytes>, v1: List<Bytes>) -> R<List<u64>> {
    let m422 = v0;
    match m422.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((v2, v3)) => {
            Ok(List::cons(f179(v2, v1.clone())?, f126(v3, v1)?))
        }
    }
}

pub fn f127(v0: List<Adt14>) -> List<u64> {
    let m423 = v0;
    match m423.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f180(v1), f127(v2))
        }
    }
}

pub fn f128(p426: List<u64>, v1: List<u64>) -> Option<u64> {
    let m425 = p426;
    {
        let mut m425 = m425;
        loop {
            let step = {
                let v0 = m425;
                let m424 = v0;
                match m424.uncons() {
                    None => {
                        Err::<List<u64>, Option<u64>>(None::<u64>)
                    }
                    Some((v2, v3)) => {
                        if f181(v2, v1.clone()) {
                            Ok::<List<u64>, Option<u64>>(v3)
                        } else {
                            Err::<List<u64>, Option<u64>>(Some(v2))
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

pub fn f129(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m427 = v0;
    match m427.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m428 = f182(v2, v1.clone())?;
            match m428 {
                None => {
                    f129(v3, v1)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f129(v3, v1)?))
                }
            }
        }
    }
}

pub fn f130(v0: List<Bytes>) -> List<Bytes> {
    f131(Fn1::F183, List::<Bytes>::nil(), v0)
}

pub fn f131(v0: Fn1, p432: List<Bytes>, p433: List<Bytes>) -> List<Bytes> {
    let m431 = (p432, p433);
    {
        let mut m431 = m431;
        loop {
            let step = {
                let (v1, v2) = m431;
                let m429 = v2;
                match m429.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c430: Fn1 = v0.clone();
                            c430.apply(v1, v3)
                        }, v4))
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

pub fn f132(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f184(v0, v1)
}

pub fn f133(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f185(f186(f187(v0))?))
}

pub fn f134(v0: Adt1, v1: Bytes) -> R<bool> {
    let m434 = f100(v0.clone(), v1)?;
    match m434 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let v3: List<Bytes> = f188(f147(f93(f53(v0, f8(Str::lit("model/authorities.toml"))))?, List::<Bytes>::nil(), List::<List<Bytes>>::nil()), f104(v2.clone(), f8(Str::lit("id"))))?;
            let m435 = v3.clone();
            match m435.uncons() {
                None => {
                    Ok(true)
                }
                Some((_, _)) => {
                    let m437 = f109(f101({
                        let m436 = f101(v2.clone(), f8(Str::lit("source")));
                        match m436 {
                            None => {
                                Adt14::C5
                            }
                            Some(v6) => {
                                v6
                            }
                        }
                    }, f8(Str::lit("sha256"))));
                    match m437 {
                        None => {
                            Ok(false)
                        }
                        Some(v7) => {
                            Ok(f58(f67(f7(List::cons(f8(Str::lit("checksum = ")), List::cons(f143(f189(v2, v7))?, List::<Bytes>::nil()))), v3)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f135() -> Bytes {
    f8(Str::lit("model/encodings"))
}

pub fn f136(v0: Adt1, p441: List<Bytes>, v2: List<Bytes>) -> R<Option<Bytes>> {
    let m440 = p441;
    {
        let mut m440 = m440;
        loop {
            let step = {
                let v1 = m440;
                let m438 = v1;
                match m438.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v3, v4)) => {
                        if f67(f190(f104({
                            let m439 = f100(v0.clone(), v3.clone())?;
                            match m439 {
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
                Ok(next) => { m440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: Adt1) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("# Owner decisions after plan v4\u{a}\u{a}Generated by `verdant_studio publish` from model/owner-decisions.json. Do not edit.\u{a}Decisions the repository owner made after `docs/verdant-studio-plan.md`, quoted\u{a}as written. Each is the evidence of a requirement in `model/requirements.json`.\u{a}")), f191(f56(v0, f69())?)?)))
}

pub fn f138(v0: List<Adt14>) -> List<Bytes> {
    let m442 = v0;
    match m442.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- **")), List::cons(f104(v1.clone(), f8(Str::lit("id"))), List::cons(f8(Str::lit("** (")), List::cons(f104(v1.clone(), f8(Str::lit("stakeholder"))), List::cons(f8(Str::lit("): ")), List::cons(f104(v1, f8(Str::lit("statement"))), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))), f138(v2))
        }
    }
}

pub fn f139(v0: List<Adt14>) -> List<Bytes> {
    let m443 = v0;
    match m443.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("- `")), List::cons(f104(v1.clone(), f8(Str::lit("construct"))), List::cons(f8(Str::lit("`: ")), List::cons(f192(v1), List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))), f139(v2))
        }
    }
}

pub fn f140(v0: Bytes, p501: u64, p502: u64, p503: Adt16) -> R<Adt16> {
    let m500 = (p501, (p502, p503));
    {
        let mut m500 = m500;
        loop {
            let step = {
                let (v1, (v2, v3)) = m500;
                let m444 = v2;
                if m444 == 0 {
                    Err::<(u64, (u64, Adt16)), R<Adt16>>(Ok(v3))
                } else {
                    let v4 = m444 - 1;
                    let v5: u64 = f193(v0.clone(), v1)?;
                    if match v3.clone() {
                        Adt16::C0(_, h445, _, _, _) => {
                            h445
                        }
                    } {
                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                            let a446 = v1;
                            let a447 = 1u64;
                            nat_add(a446, a447)?
                        }, (v4, Adt16::C0(true, false, match v3.clone() {
                            Adt16::C0(_, _, h448, _, _) => {
                                h448
                            }
                        }, match v3.clone() {
                            Adt16::C0(_, _, _, h449, _) => {
                                h449
                            }
                        }, match v3 {
                            Adt16::C0(_, _, _, _, h450) => {
                                h450
                            }
                        }))))
                    } else {
                        if match v3.clone() {
                            Adt16::C0(h451, _, _, _, _) => {
                                h451
                            }
                        } {
                            let a452 = v5;
                            let a453 = 34u64;
                            if nat_eq(a452, a453) {
                                let a456 = f193(v0.clone(), {
                                    let a454 = v1;
                                    let a455 = 1u64;
                                    nat_add(a454, a455)?
                                })?;
                                let a457 = 34u64;
                                if nat_eq(a456, a457) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a458 = v1;
                                        let a459 = 1u64;
                                        nat_add(a458, a459)?
                                    }, (v4, Adt16::C0(true, true, f194(match v3.clone() {
                                        Adt16::C0(_, _, h460, _, _) => {
                                            h460
                                        }
                                    }, v0.clone(), v1), match v3.clone() {
                                        Adt16::C0(_, _, _, h461, _) => {
                                            h461
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h462) => {
                                            h462
                                        }
                                    }))))
                                } else {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a463 = v1;
                                        let a464 = 1u64;
                                        nat_add(a463, a464)?
                                    }, (v4, Adt16::C0(false, false, match v3.clone() {
                                        Adt16::C0(_, _, h465, _, _) => {
                                            h465
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h466, _) => {
                                            h466
                                        }
                                    }, match v3 {
                                        Adt16::C0(_, _, _, _, h467) => {
                                            h467
                                        }
                                    }))))
                                }
                            } else {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a468 = v1;
                                    let a469 = 1u64;
                                    nat_add(a468, a469)?
                                }, (v4, Adt16::C0(true, false, f194(match v3.clone() {
                                    Adt16::C0(_, _, h470, _, _) => {
                                        h470
                                    }
                                }, v0.clone(), v1), match v3.clone() {
                                    Adt16::C0(_, _, _, h471, _) => {
                                        h471
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h472) => {
                                        h472
                                    }
                                }))))
                            }
                        } else {
                            let a473 = v5;
                            let a474 = 34u64;
                            if nat_eq(a473, a474) {
                                Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                    let a475 = v1;
                                    let a476 = 1u64;
                                    nat_add(a475, a476)?
                                }, (v4, Adt16::C0(true, false, match v3.clone() {
                                    Adt16::C0(_, _, h477, _, _) => {
                                        h477
                                    }
                                }, match v3.clone() {
                                    Adt16::C0(_, _, _, h478, _) => {
                                        h478
                                    }
                                }, match v3 {
                                    Adt16::C0(_, _, _, _, h479) => {
                                        h479
                                    }
                                }))))
                            } else {
                                let a480 = v5;
                                let a481 = 44u64;
                                if nat_eq(a480, a481) {
                                    Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                        let a482 = v1;
                                        let a483 = 1u64;
                                        nat_add(a482, a483)?
                                    }, (v4, Adt16::C0(false, false, Bytes::lit(&[]), List::cons(match v3.clone() {
                                        Adt16::C0(_, _, h484, _, _) => {
                                            h484
                                        }
                                    }, match v3.clone() {
                                        Adt16::C0(_, _, _, h485, _) => {
                                            h485
                                        }
                                    }), match v3 {
                                        Adt16::C0(_, _, _, _, h486) => {
                                            h486
                                        }
                                    }))))
                                } else {
                                    let a487 = v5;
                                    let a488 = 13u64;
                                    if nat_eq(a487, a488) {
                                        Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                            let a489 = v1;
                                            let a490 = 1u64;
                                            nat_add(a489, a490)?
                                        }, (v4, v3)))
                                    } else {
                                        let a491 = v5;
                                        let a492 = 10u64;
                                        if nat_eq(a491, a492) {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a493 = v1;
                                                let a494 = 1u64;
                                                nat_add(a493, a494)?
                                            }, (v4, f141(v3))))
                                        } else {
                                            Ok::<(u64, (u64, Adt16)), R<Adt16>>(({
                                                let a495 = v1;
                                                let a496 = 1u64;
                                                nat_add(a495, a496)?
                                            }, (v4, Adt16::C0(false, false, f194(match v3.clone() {
                                                Adt16::C0(_, _, h497, _, _) => {
                                                    h497
                                                }
                                            }, v0.clone(), v1), match v3.clone() {
                                                Adt16::C0(_, _, _, h498, _) => {
                                                    h498
                                                }
                                            }, match v3 {
                                                Adt16::C0(_, _, _, _, h499) => {
                                                    h499
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
                Ok(next) => { m500 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Adt16) -> Adt16 {
    Adt16::C0(false, false, Bytes::lit(&[]), List::<Bytes>::nil(), List::cons(f195(List::cons(match v0.clone() {
        Adt16::C0(_, _, h504, _, _) => {
            h504
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, h505, _) => {
            h505
        }
    }), List::<Bytes>::nil()), match v0 {
        Adt16::C0(_, _, _, _, h506) => {
            h506
        }
    }))
}

pub fn f142(p509: List<List<Bytes>>, p510: List<List<Bytes>>) -> List<List<Bytes>> {
    let m508 = (p509, p510);
    {
        let mut m508 = m508;
        loop {
            let step = {
                let (v0, v1) = m508;
                let m507 = v0;
                match m507.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<Bytes>>, List<List<Bytes>>), List<List<Bytes>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(Bytes::lit(&[0x22]), List::cons(f196(v0.clone(), 0u64, {
        let a511 = v0;
        length_bytes(a511)
    }, Bytes::lit(&[]))?, List::cons(Bytes::lit(&[0x22]), List::<Bytes>::nil())))))
}

pub fn f144(v0: Adt20) -> Bytes {
    let a532 = f197(match v0.clone() {
        Adt20::C0(h512, _, _, _, _, _, _, _) => {
            h512
        }
    });
    let a533 = {
        let a530 = f197(match v0.clone() {
            Adt20::C0(_, h513, _, _, _, _, _, _) => {
                h513
            }
        });
        let a531 = {
            let a528 = f197(match v0.clone() {
                Adt20::C0(_, _, h514, _, _, _, _, _) => {
                    h514
                }
            });
            let a529 = {
                let a526 = f197(match v0.clone() {
                    Adt20::C0(_, _, _, h515, _, _, _, _) => {
                        h515
                    }
                });
                let a527 = {
                    let a524 = f197(match v0.clone() {
                        Adt20::C0(_, _, _, _, h516, _, _, _) => {
                            h516
                        }
                    });
                    let a525 = {
                        let a522 = f197(match v0.clone() {
                            Adt20::C0(_, _, _, _, _, h517, _, _) => {
                                h517
                            }
                        });
                        let a523 = {
                            let a520 = f197(match v0.clone() {
                                Adt20::C0(_, _, _, _, _, _, h518, _) => {
                                    h518
                                }
                            });
                            let a521 = f197(match v0 {
                                Adt20::C0(_, _, _, _, _, _, _, h519) => {
                                    h519
                                }
                            });
                            append_bytes(a520, a521)
                        };
                        append_bytes(a522, a523)
                    };
                    append_bytes(a524, a525)
                };
                append_bytes(a526, a527)
            };
            append_bytes(a528, a529)
        };
        append_bytes(a530, a531)
    };
    append_bytes(a532, a533)
}

pub fn f145(v0: Bytes) -> R<Adt20> {
    let v1: u64 = {
        let a534 = v0.clone();
        length_bytes(a534)
    };
    let v2: u64 = {
        let a540 = {
            let a537 = {
                let a535 = v1;
                let a536 = 72u64;
                nat_add(a535, a536)?
            };
            let a538 = 64u64;
            let a539 = 0u64;
            nat_quot(a537, a538, a539)
        };
        let a541 = 64u64;
        nat_mul(a540, a541)?
    };
    let (_, h542) = f198(Fn3::F199(v0, v1, v2), {
        let a543 = v2;
        let a544 = 64u64;
        let a545 = 0u64;
        nat_quot(a543, a544, a545)
    }, (0u64, f200()))?;
    Ok(h542)
}

pub fn f146(p548: List<List<Bytes>>, v1: Bytes, p549: List<Bytes>) -> R<List<Bytes>> {
    let m547 = (p548, p549);
    {
        let mut m547 = m547;
        loop {
            let step = {
                let (v0, v2) = m547;
                let m546 = v0;
                match m546.uncons() {
                    None => {
                        Err::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f201(v3.clone(), v1.clone())? {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, v2))
                        } else {
                            Ok::<(List<List<Bytes>>, List<Bytes>), R<List<Bytes>>>((v4, List::cons(f149(f202(v3), Bytes::lit(&[0x0a])), v2)))
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

pub fn f147(p552: List<Bytes>, p553: List<Bytes>, p554: List<List<Bytes>>) -> List<List<Bytes>> {
    let m551 = (p552, (p553, p554));
    {
        let mut m551 = m551;
        loop {
            let step = {
                let (v0, (v1, v2)) = m551;
                let m550 = v0;
                match m550.uncons() {
                    None => {
                        Err::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>(List::cons(f203(v1, List::<Bytes>::nil()), v2))
                    }
                    Some((v3, v4)) => {
                        if f26(v3.clone(), f8(Str::lit("[[authority]]"))) {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, List::<Bytes>::nil()), List::cons(f203(v1, List::<Bytes>::nil()), v2))))
                        } else {
                            Ok::<(List<Bytes>, (List<Bytes>, List<List<Bytes>>)), List<List<Bytes>>>((v4, (List::cons(v3, v1), v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m551 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Bytes) -> R<Bytes> {
    Ok(f43(v0.clone(), 0u64, f204(v0)?))
}

pub fn f149(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m555 = v0;
    match m555.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m556 = v3.clone();
            match m556.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a559 = v2;
                    let a560 = {
                        let a557 = v1.clone();
                        let a558 = f149(v3, v1);
                        append_bytes(a557, a558)
                    };
                    append_bytes(a559, a560)
                }
            }
        }
    }
}

pub fn f150(v0: List<Bytes>) -> Adt14 {
    let m561 = v0;
    match m561.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(Adt14::C2(v1)), std::rc::Rc::new(f150(v2)))
        }
    }
}

pub fn f151(v0: Fn2, p565: Option<Bytes>, p566: List<Bytes>) -> Option<Bytes> {
    let m564 = (p565, p566);
    {
        let mut m564 = m564;
        loop {
            let step = {
                let (v1, v2) = m564;
                let m562 = v2;
                match m562.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c563: Fn2 = v0.clone();
                            c563.apply(v1, v3)
                        }, v4))
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

pub fn f152(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m567 = v1;
    match m567 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a570 = {
                    let a568 = v3;
                    let a569 = v0;
                    append_bytes(a568, a569)
                };
                let a571 = v2;
                append_bytes(a570, a571)
            })
        }
    }
}

pub fn f153(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f204(v0.clone())?;
    let v2: u64 = f205(v0.clone(), v1)?;
    Ok(f43(v0, v2, {
        let a572 = v1;
        let a573 = v2;
        nat_sub(a572, a573)
    }))
}

pub fn f154(v0: Bytes) -> R<Option<Adt14>> {
    let v1: Bytes = f206(v0)?;
    let m577 = f207(v1.clone(), 0u64, {
        let a575 = {
            let a574 = v1.clone();
            length_bytes(a574)
        };
        let a576 = 2u64;
        nat_add(a575, a576)?
    }, 0u64)?;
    match m577 {
        None => {
            Ok(None::<Adt14>)
        }
        Some(v2) => {
            let a580 = {
                let (_, h578) = v2;
                h578
            };
            let a581 = {
                let a579 = v1;
                length_bytes(a579)
            };
            if nat_eq(a580, a581) {
                Ok(Some({
                    let (h582, _) = v2;
                    h582
                }))
            } else {
                Ok(None::<Adt14>)
            }
        }
    }
}

pub fn f155(v0: Bytes) -> Bytes {
    f7(List::cons(f60(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil())))))
}

pub fn f156(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f208(v0, v1)
}

pub fn f157(v0: Adt14) -> Bytes {
    if f167(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f167(v0.clone(), f8(Str::lit("id"))) {
            if f167(v0.clone(), f8(Str::lit("statement"))) {
                if f167(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f168(v0, f8(Str::lit("evidence")))
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

pub fn f158(v0: List<Adt14>) -> Adt14 {
    let m583 = v0;
    match m583.uncons() {
        None => {
            Adt14::C5
        }
        Some((v1, v2)) => {
            Adt14::C6(std::rc::Rc::new(v1), std::rc::Rc::new(f158(v2)))
        }
    }
}

pub fn f159(v0: Adt14) -> bool {
    if f167(v0.clone(), f8(Str::lit("name"))) {
        if f168(v0.clone(), f8(Str::lit("schema"))) {
            let m584 = f161(f57(f105(f101(v0, f8(Str::lit("citation"))))));
            match m584 {
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

pub fn f160(v0: Bytes) -> R<Bytes> {
    f209(v0.clone(), 0u64, {
        let a585 = v0;
        length_bytes(a585)
    }, true, Bytes::lit(&[]))
}

pub fn f161(v0: List<Bytes>) -> Option<Adt14> {
    let m586 = v0;
    match m586.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            if f26(v1.clone(), f8(Str::lit("req"))) {
                let m587 = v2;
                match m587.uncons() {
                    None => {
                        None::<Adt14>
                    }
                    Some((v3, _)) => {
                        Some(Adt14::C7(f8(Str::lit("req")), std::rc::Rc::new(Adt14::C2(v3)), std::rc::Rc::new(Adt14::C5)))
                    }
                }
            } else {
                if f26(v1.clone(), f8(Str::lit("std"))) {
                    let m588 = v2;
                    match m588.uncons() {
                        None => {
                            None::<Adt14>
                        }
                        Some((v5, v6)) => {
                            Some(Adt14::C7(f8(Str::lit("std")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("authority")), std::rc::Rc::new(Adt14::C2(v5)), std::rc::Rc::new({
                                let m589 = v6;
                                match m589.uncons() {
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
                        let m590 = v2;
                        match m590.uncons() {
                            None => {
                                None::<Adt14>
                            }
                            Some((v9, v10)) => {
                                let m591 = v10;
                                match m591.uncons() {
                                    None => {
                                        None::<Adt14>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt14::C7(f8(Str::lit("der")), std::rc::Rc::new(Adt14::C4(std::rc::Rc::new(Adt14::C7(f8(Str::lit("check")), std::rc::Rc::new(Adt14::C2(v11)), std::rc::Rc::new(Adt14::C7(f8(Str::lit("inputs")), std::rc::Rc::new(Adt14::C3(std::rc::Rc::new(f150(v12)))), std::rc::Rc::new(Adt14::C7(f8(Str::lit("rule")), std::rc::Rc::new(Adt14::C2(v9)), std::rc::Rc::new(Adt14::C5))))))))), std::rc::Rc::new(Adt14::C5)))
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

pub fn f162(v0: Adt14, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let m592 = f210(4096u64, f211(f212(f213(f101(v0.clone(), f8(Str::lit("$defs")))))?, List::cons(Adt21::C0(v2, v0), List::<Adt21>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
    match m592 {
        Ok(v4) => {
            Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f149(f203(v4, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
        }
        Err(v3) => {
            Ok(Err::<Bytes, Bytes>(v3))
        }
    }
}

pub fn f163(v0: List<Adt14>, v1: Adt14) -> List<Adt14> {
    let m593 = v0;
    match m593.uncons() {
        None => {
            List::cons(v1, List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            List::cons(v2, f163(v3, v1))
        }
    }
}

pub fn f164(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f149(f214(f93(v0)?, v1)?, Bytes::lit(&[0x0a])))
}

pub fn f165(v0: Adt14) -> R<Bytes> {
    let m594 = v0;
    match m594 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f215(v2, v3)
        }
        Adt14::C2(v4) => {
            f216(v4)
        }
        Adt14::C3(r595) => {
            let v5 = (*r595).clone();
            Ok(f217(f218(v5)?))
        }
        Adt14::C4(r596) => {
            let v6 = (*r596).clone();
            f219(f220(v6)?)
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

pub fn f166(v0: Adt14) -> List<Adt14> {
    let m597 = v0;
    match m597 {
        Adt14::C0(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C1(_, _) => {
            List::<Adt14>::nil()
        }
        Adt14::C2(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C3(r598) => {
            let v3 = (*r598).clone();
            f166(v3)
        }
        Adt14::C4(_) => {
            List::<Adt14>::nil()
        }
        Adt14::C5 => {
            List::<Adt14>::nil()
        }
        Adt14::C6(r599, r600) => {
            let v1 = (*r599).clone();
            let v2 = (*r600).clone();
            List::cons(v1, f166(v2))
        }
        Adt14::C7(_, _, _) => {
            List::<Adt14>::nil()
        }
    }
}

pub fn f167(v0: Adt14, v1: Bytes) -> bool {
    f58({
        let a602 = {
            let a601 = f104(v0, v1);
            length_bytes(a601)
        };
        let a603 = 0u64;
        nat_eq(a602, a603)
    })
}

pub fn f168(v0: Adt14, v1: Bytes) -> bool {
    let m604 = f101(v0, v1);
    match m604 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f169(v0: Adt14, v1: Bytes, v2: Bytes) -> List<Adt14> {
    let m605 = f101(v0, v1);
    match m605 {
        None => {
            List::<Adt14>::nil()
        }
        Some(v3) => {
            f105(f101(v3, v2))
        }
    }
}

pub fn f170(v0: Adt14, v1: Option<(Bytes, Bytes)>) -> R<Bytes> {
    let v2: Option<Adt14> = f101(v0.clone(), f8(Str::lit("source")));
    Ok(f7(List::cons(f8(Str::lit("[[authority]]\u{a}")), List::cons(f87(Str::lit("id"), f104(v0.clone(), f8(Str::lit("id"))))?, List::cons(f87(Str::lit("name"), f104(v0.clone(), f8(Str::lit("name"))))?, List::cons(f87(Str::lit("issuer"), f104(v0.clone(), f8(Str::lit("issuer"))))?, List::cons(f87(Str::lit("edition"), f104(v0.clone(), f8(Str::lit("edition"))))?, List::cons(f87(Str::lit("canonical_id"), f104(v0.clone(), f8(Str::lit("canonical_id"))))?, List::cons(f87(Str::lit("source_role"), f104(v0.clone(), f8(Str::lit("source_role"))))?, List::cons(f87(Str::lit("retrieval_date"), f104(v0.clone(), f8(Str::lit("retrieval_date"))))?, List::cons(f87(Str::lit("url"), f221(v2.clone(), f8(Str::lit("url"))))?, List::cons(f87(Str::lit("media_type"), f221(v2.clone(), f8(Str::lit("media_type"))))?, List::cons(f87(Str::lit("license"), f221(v2.clone(), f8(Str::lit("license"))))?, List::cons(f87(Str::lit("redistribution"), f221(v2, f8(Str::lit("redistribution"))))?, List::cons(f87(Str::lit("statement"), f104(v0.clone(), f8(Str::lit("statement"))))?, List::cons({
        let m606 = v1;
        match m606 {
            None => {
                f7(List::cons(f87(Str::lit("checksum"), f8(Str::lit("none")))?, List::cons(f87(Str::lit("checksum_reason"), f8(Str::lit("The authority is citation-only, so no artifact is committed.")))?, List::<Bytes>::nil())))
            }
            Some(v3) => {
                f7(List::cons(f87(Str::lit("artifact"), {
                    let (h607, _) = v3;
                    h607
                })?, List::cons(f87(Str::lit("checksum"), {
                    let (_, h608) = v3;
                    h608
                })?, List::<Bytes>::nil())))
            }
        }
    }, List::cons(f7(List::cons(f8(Str::lit("realized_by = ")), List::cons(f222(f57(f105(f101(v0, f8(Str::lit("realized_by"))))))?, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))), List::<Bytes>::nil())))))))))))))))))
}

pub fn f171(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h609, _) = v2.clone();
        h609
    };
    let v4: u64 = {
        let (h610, _) = {
            let (_, h611) = v2.clone();
            h611
        };
        h610
    };
    let v5: List<Bytes> = {
        let (_, h612) = {
            let (_, h613) = v2;
            h613
        };
        h612
    };
    let m614 = f223(v0.clone(), v3);
    match m614 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a615 = v6;
            let a616 = v1;
            if equal(a615, a616) {
                Ok(Some(({
                    let a617 = v3;
                    let a618 = 1u64;
                    nat_add(a617, a618)?
                }, ({
                    let a619 = v3;
                    let a620 = 1u64;
                    nat_add(a619, a620)?
                }, List::cons(f43(v0, v4, {
                    let a621 = v3;
                    let a622 = v4;
                    nat_sub(a621, a622)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a623 = v3;
                    let a624 = 1u64;
                    nat_add(a623, a624)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f172(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f173(v0: Bytes) -> bool {
    f118(v0, f8(Str::lit(".lex.tex")))
}

pub fn f174(v0: List<Bytes>, v1: u64) -> R<List<u64>> {
    let m625 = v0;
    match m625.uncons() {
        None => {
            Ok(List::<u64>::nil())
        }
        Some((_, v3)) => {
            Ok(List::cons(v1, f174(v3, {
                let a626 = v1;
                let a627 = 1u64;
                nat_add(a626, a627)?
            })?))
        }
    }
}

pub fn f175(v0: Adt15, v1: List<u64>, v2: Adt18) -> bool {
    let m628 = v2;
    match m628 {
        Adt18::C0(v3, v4) => {
            if f181(v3, match v0.clone() {
                Adt15::C0(h629, _, _, _) => {
                    h629
                }
            }) {
                f224(v0, v4)
            } else {
                false
            }
        }
        Adt18::C1(v5) => {
            f181(v5, match v0 {
                Adt15::C0(_, _, h630, _) => {
                    h630
                }
            })
        }
        Adt18::C2(_, v7, v8) => {
            if f177(v7, v1) {
                f181(v8, match v0 {
                    Adt15::C0(_, _, _, h631) => {
                        h631
                    }
                })
            } else {
                false
            }
        }
    }
}

pub fn f176(v0: Adt12, v1: Adt14) -> R<Adt18> {
    let m632 = f101(v1.clone(), f8(Str::lit("std")));
    match m632 {
        None => {
            let m637 = f109(f101(v1.clone(), f8(Str::lit("req"))));
            match m637 {
                None => {
                    let m639 = f101(v1, f8(Str::lit("der")));
                    match m639 {
                        None => {
                            Ok(f225(v0))
                        }
                        Some(v5) => {
                            let m640 = f226(f104(v5.clone(), f8(Str::lit("rule"))));
                            match m640 {
                                None => {
                                    Ok(f225(v0))
                                }
                                Some(v6) => {
                                    Ok(Adt18::C2(v6, f126(f57(f105(f101(v5.clone(), f8(Str::lit("inputs"))))), match v0.clone() {
                                        Adt12::C0(_, _, _, _, h641) => {
                                            h641
                                        }
                                    })?, f179(f104(v5, f8(Str::lit("check"))), match v0 {
                                        Adt12::C0(_, _, _, h642, _) => {
                                            h642
                                        }
                                    })?))
                                }
                            }
                        }
                    }
                }
                Some(v4) => {
                    Ok(Adt18::C1(f179(v4, match v0 {
                        Adt12::C0(_, _, h638, _, _) => {
                            h638
                        }
                    })?))
                }
            }
        }
        Some(v2) => {
            let m633 = f109(f101(v2.clone(), f8(Str::lit("row"))));
            match m633 {
                None => {
                    Ok(Adt18::C0(f179(f104(v2, f8(Str::lit("authority"))), match v0 {
                        Adt12::C0(h634, _, _, _, _) => {
                            h634
                        }
                    })?, None::<u64>))
                }
                Some(v3) => {
                    Ok(Adt18::C0(f179(f104(v2, f8(Str::lit("authority"))), match v0.clone() {
                        Adt12::C0(h635, _, _, _, _) => {
                            h635
                        }
                    })?, Some(f179(v3, match v0 {
                        Adt12::C0(_, h636, _, _, _) => {
                            h636
                        }
                    })?)))
                }
            }
        }
    }
}

pub fn f177(p645: List<u64>, v1: List<u64>) -> bool {
    let m644 = p645;
    {
        let mut m644 = m644;
        loop {
            let step = {
                let v0 = m644;
                let m643 = v0;
                match m643.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f181(v2, v1.clone()) {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f178(p650: List<u64>) -> bool {
    let m649 = p650;
    {
        let mut m649 = m649;
        loop {
            let step = {
                let v0 = m649;
                let m646 = v0;
                match m646.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a647 = v1;
                        let a648 = 7u64;
                        if nat_le(a647, a648) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f179(v0: Bytes, v1: List<Bytes>) -> R<u64> {
    let m651 = f227(v0, v1.clone(), 0u64)?;
    match m651 {
        None => {
            let a652 = v1;
            Ok(length_list(a652))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f180(v0: Adt14) -> u64 {
    let m653 = f228(f101(v0, f8(Str::lit("stratum"))));
    match m653 {
        None => {
            8u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f181(v0: u64, p658: List<u64>) -> bool {
    let m657 = p658;
    {
        let mut m657 = m657;
        loop {
            let step = {
                let v1 = m657;
                let m654 = v1;
                match m654.uncons() {
                    None => {
                        Err::<List<u64>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a655 = v2;
                        let a656 = v0;
                        if nat_eq(a655, a656) {
                            Err::<List<u64>, bool>(true)
                        } else {
                            Ok::<List<u64>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m657 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f229(v0.clone(), v1) {
        let m659 = f41(v0, 34u8)?;
        match m659.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m660 = v3;
                match m660.uncons() {
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

pub fn f183(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f184(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m661 = v0.clone();
    match m661.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a662 = v2;
            let a663 = v1.clone();
            if equal(a662, a663) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f185(v0: List<(Bytes, ())>) -> List<Bytes> {
    f130(f230(Fn4::F231, List::<Bytes>::nil(), v0))
}

pub fn f186(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f232(f233(Fn5::F234, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m668 = {
        let (h664, _) = f235(Fn6::F236, {
            let a666 = {
                let a665 = v0;
                length_list(a665)
            };
            let a667 = 1u64;
            nat_add(a666, a667)?
        }, v1)?;
        h664
    };
    match m668.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f187(v0: List<Bytes>) -> List<(Bytes, ())> {
    f237(f238(Fn7::F239, List::<(Bytes, ())>::nil(), v0))
}

pub fn f188(p671: List<List<Bytes>>, v1: Bytes) -> R<List<Bytes>> {
    let m670 = p671;
    {
        let mut m670 = m670;
        loop {
            let step = {
                let v0 = m670;
                let m669 = v0;
                match m669.uncons() {
                    None => {
                        Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        if f201(v2.clone(), v1.clone())? {
                            Err::<List<List<Bytes>>, R<List<Bytes>>>(Ok(v2))
                        } else {
                            Ok::<List<List<Bytes>>, R<List<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: Adt14, v1: Bytes) -> Bytes {
    if f26(f221(f101(v0, f8(Str::lit("source"))), f8(Str::lit("redistribution"))), f8(Str::lit("redistributable"))) {
        v1
    } else {
        f8(Str::lit("none"))
    }
}

pub fn f190(v0: Bytes) -> R<Bytes> {
    let a672 = f8(Str::lit("Catalog."));
    let a673 = f160(v0)?;
    Ok(append_bytes(a672, a673))
}

pub fn f191(v0: List<Adt14>) -> R<List<Bytes>> {
    let m674 = v0;
    match m674.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f7(List::cons(f8(Str::lit("\u{a}## ")), List::cons(f91(f57(f105(f101(v1.clone(), f8(Str::lit("requirements"))))), f8(Str::lit(", "))), List::cons(f8(Str::lit(" (")), List::cons(f104(v1.clone(), f8(Str::lit("agent"))), List::cons(f8(Str::lit(", ")), List::cons(f104(v1.clone(), f8(Str::lit("at"))), List::cons(f8(Str::lit(")\u{a}\u{a}")), List::cons(f7(f240(f93(f104(v1, f8(Str::lit("quote"))))?)), List::<Bytes>::nil()))))))))), f191(v2)?))
        }
    }
}

pub fn f192(v0: Adt14) -> Bytes {
    if f167(v0.clone(), f8(Str::lit("req"))) {
        let a675 = f8(Str::lit("req "));
        let a676 = f104(v0, f8(Str::lit("req")));
        append_bytes(a675, a676)
    } else {
        let m677 = f101(v0.clone(), f8(Str::lit("std")));
        match m677 {
            None => {
                let m680 = f101(v0, f8(Str::lit("der")));
                match m680 {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some(v2) => {
                        f7(List::cons(f8(Str::lit("der ")), List::cons(f104(v2.clone(), f8(Str::lit("rule"))), List::cons(f8(Str::lit(" from ")), List::cons(f91(f57(f105(f101(v2, f8(Str::lit("inputs"))))), f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                }
            }
            Some(v1) => {
                f7(List::cons(f8(Str::lit("std ")), List::cons(f104(v1.clone(), f8(Str::lit("authority"))), List::cons(if f167(v1.clone(), f8(Str::lit("row"))) {
                    let a678 = f8(Str::lit(" row "));
                    let a679 = f104(v1, f8(Str::lit("row")));
                    append_bytes(a678, a679)
                } else {
                    Bytes::lit(&[])
                }, List::<Bytes>::nil()))))
            }
        }
    }
}

pub fn f193(v0: Bytes, v1: u64) -> R<u64> {
    let m683 = {
        let a681 = v0;
        let a682 = v1;
        index_bytes(a681, a682)
    };
    match m683 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a728 = {
                let a704 = {
                    let a692 = {
                        let a686 = {
                            let a684 = v3;
                            let a685 = 128u8;
                            fixed_u8::bit_and(a684, a685)
                        };
                        let a687 = 128u8;
                        if equal(a686, a687) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a693 = {
                        let a690 = {
                            let a688 = v3;
                            let a689 = 64u8;
                            fixed_u8::bit_and(a688, a689)
                        };
                        let a691 = 64u8;
                        if equal(a690, a691) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a692, a693)?
                };
                let a705 = {
                    let a702 = {
                        let a696 = {
                            let a694 = v3;
                            let a695 = 32u8;
                            fixed_u8::bit_and(a694, a695)
                        };
                        let a697 = 32u8;
                        if equal(a696, a697) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a703 = {
                        let a700 = {
                            let a698 = v3;
                            let a699 = 16u8;
                            fixed_u8::bit_and(a698, a699)
                        };
                        let a701 = 16u8;
                        if equal(a700, a701) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a702, a703)?
                };
                nat_add(a704, a705)?
            };
            let a729 = {
                let a726 = {
                    let a714 = {
                        let a708 = {
                            let a706 = v3;
                            let a707 = 8u8;
                            fixed_u8::bit_and(a706, a707)
                        };
                        let a709 = 8u8;
                        if equal(a708, a709) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a715 = {
                        let a712 = {
                            let a710 = v3;
                            let a711 = 4u8;
                            fixed_u8::bit_and(a710, a711)
                        };
                        let a713 = 4u8;
                        if equal(a712, a713) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a714, a715)?
                };
                let a727 = {
                    let a724 = {
                        let a718 = {
                            let a716 = v3;
                            let a717 = 2u8;
                            fixed_u8::bit_and(a716, a717)
                        };
                        let a719 = 2u8;
                        if equal(a718, a719) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a725 = {
                        let a722 = {
                            let a720 = v3;
                            let a721 = 1u8;
                            fixed_u8::bit_and(a720, a721)
                        };
                        let a723 = 1u8;
                        if equal(a722, a723) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a724, a725)?
                };
                nat_add(a726, a727)?
            };
            nat_add(a728, a729)
        }
    }
}

pub fn f194(v0: Bytes, v1: Bytes, v2: u64) -> Bytes {
    let m733 = {
        let a730 = v1;
        let a731 = v2;
        let a732 = 1u64;
        slice_bytes(a730, a731, a732)
    };
    match m733 {
        None => {
            v0
        }
        Some(v3) => {
            let a734 = v0;
            let a735 = v3;
            append_bytes(a734, a735)
        }
    }
}

pub fn f195(p738: List<Bytes>, p739: List<Bytes>) -> List<Bytes> {
    let m737 = (p738, p739);
    {
        let mut m737 = m737;
        loop {
            let step = {
                let (v0, v1) = m737;
                let m736 = v0;
                match m736.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m737 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Bytes, p811: u64, p812: u64, p813: Bytes) -> R<Bytes> {
    let m810 = (p811, (p812, p813));
    {
        let mut m810 = m810;
        loop {
            let step = {
                let (v1, (v2, v3)) = m810;
                let m740 = v2;
                if m740 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m740 - 1;
                    let m743 = {
                        let a741 = v0.clone();
                        let a742 = v1;
                        index_bytes(a741, a742)
                    };
                    match m743 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v7: u64 = {
                                let v6: u8 = v5;
                                let a788 = {
                                    let a764 = {
                                        let a752 = {
                                            let a746 = {
                                                let a744 = v6;
                                                let a745 = 128u8;
                                                fixed_u8::bit_and(a744, a745)
                                            };
                                            let a747 = 128u8;
                                            if equal(a746, a747) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a753 = {
                                            let a750 = {
                                                let a748 = v6;
                                                let a749 = 64u8;
                                                fixed_u8::bit_and(a748, a749)
                                            };
                                            let a751 = 64u8;
                                            if equal(a750, a751) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a752, a753)?
                                    };
                                    let a765 = {
                                        let a762 = {
                                            let a756 = {
                                                let a754 = v6;
                                                let a755 = 32u8;
                                                fixed_u8::bit_and(a754, a755)
                                            };
                                            let a757 = 32u8;
                                            if equal(a756, a757) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a763 = {
                                            let a760 = {
                                                let a758 = v6;
                                                let a759 = 16u8;
                                                fixed_u8::bit_and(a758, a759)
                                            };
                                            let a761 = 16u8;
                                            if equal(a760, a761) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a762, a763)?
                                    };
                                    nat_add(a764, a765)?
                                };
                                let a789 = {
                                    let a786 = {
                                        let a774 = {
                                            let a768 = {
                                                let a766 = v6;
                                                let a767 = 8u8;
                                                fixed_u8::bit_and(a766, a767)
                                            };
                                            let a769 = 8u8;
                                            if equal(a768, a769) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a775 = {
                                            let a772 = {
                                                let a770 = v6;
                                                let a771 = 4u8;
                                                fixed_u8::bit_and(a770, a771)
                                            };
                                            let a773 = 4u8;
                                            if equal(a772, a773) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a774, a775)?
                                    };
                                    let a787 = {
                                        let a784 = {
                                            let a778 = {
                                                let a776 = v6;
                                                let a777 = 2u8;
                                                fixed_u8::bit_and(a776, a777)
                                            };
                                            let a779 = 2u8;
                                            if equal(a778, a779) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a785 = {
                                            let a782 = {
                                                let a780 = v6;
                                                let a781 = 1u8;
                                                fixed_u8::bit_and(a780, a781)
                                            };
                                            let a783 = 1u8;
                                            if equal(a782, a783) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a784, a785)?
                                    };
                                    nat_add(a786, a787)?
                                };
                                nat_add(a788, a789)?
                            };
                            let a790 = v7;
                            let a791 = 34u64;
                            if if nat_eq(a790, a791) {
                                true
                            } else {
                                let a792 = v7;
                                let a793 = 92u64;
                                nat_eq(a792, a793)
                            } {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a794 = v1;
                                    let a795 = 1u64;
                                    nat_add(a794, a795)?
                                }, (v4, {
                                    let a798 = v3;
                                    let a799 = {
                                        let a796 = Bytes::lit(&[0x5c]);
                                        let a797 = f43(v0.clone(), v1, 1u64);
                                        append_bytes(a796, a797)
                                    };
                                    append_bytes(a798, a799)
                                })))
                            } else {
                                let a800 = v7;
                                let a801 = 10u64;
                                if nat_eq(a800, a801) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a802 = v1;
                                        let a803 = 1u64;
                                        nat_add(a802, a803)?
                                    }, (v4, {
                                        let a804 = v3;
                                        let a805 = Bytes::lit(&[0x5c, 0x6e]);
                                        append_bytes(a804, a805)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a806 = v1;
                                        let a807 = 1u64;
                                        nat_add(a806, a807)?
                                    }, (v4, {
                                        let a808 = v3;
                                        let a809 = f43(v0.clone(), v1, 1u64);
                                        append_bytes(a808, a809)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m810 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: u32) -> Bytes {
    let a842 = f241({
        let a814 = f242(v0, 28u32);
        let a815 = 15u32;
        fixed_u32::bit_and(a814, a815)
    });
    let a843 = {
        let a840 = f241({
            let a816 = f242(v0, 24u32);
            let a817 = 15u32;
            fixed_u32::bit_and(a816, a817)
        });
        let a841 = {
            let a838 = f241({
                let a818 = f242(v0, 20u32);
                let a819 = 15u32;
                fixed_u32::bit_and(a818, a819)
            });
            let a839 = {
                let a836 = f241({
                    let a820 = f242(v0, 16u32);
                    let a821 = 15u32;
                    fixed_u32::bit_and(a820, a821)
                });
                let a837 = {
                    let a834 = f241({
                        let a822 = f242(v0, 12u32);
                        let a823 = 15u32;
                        fixed_u32::bit_and(a822, a823)
                    });
                    let a835 = {
                        let a832 = f241({
                            let a824 = f242(v0, 8u32);
                            let a825 = 15u32;
                            fixed_u32::bit_and(a824, a825)
                        });
                        let a833 = {
                            let a830 = f241({
                                let a826 = f242(v0, 4u32);
                                let a827 = 15u32;
                                fixed_u32::bit_and(a826, a827)
                            });
                            let a831 = f241({
                                let a828 = v0;
                                let a829 = 15u32;
                                fixed_u32::bit_and(a828, a829)
                            });
                            append_bytes(a830, a831)
                        };
                        append_bytes(a832, a833)
                    };
                    append_bytes(a834, a835)
                };
                append_bytes(a836, a837)
            };
            append_bytes(a838, a839)
        };
        append_bytes(a840, a841)
    };
    append_bytes(a842, a843)
}

pub fn f198(v0: Fn3, p847: u64, p848: (u64, Adt20)) -> R<(u64, Adt20)> {
    let m846 = (p847, p848);
    {
        let mut m846 = m846;
        loop {
            let step = {
                let (v1, v2) = m846;
                let m844 = v1;
                if m844 == 0 {
                    Err::<(u64, (u64, Adt20)), R<(u64, Adt20)>>(Ok(v2))
                } else {
                    let v3 = m844 - 1;
                    Ok::<(u64, (u64, Adt20)), R<(u64, Adt20)>>((v3, {
                        let c845: Fn3 = v0.clone();
                        c845.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m846 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt20)) -> R<(u64, Adt20)> {
    Ok(({
        let a850 = {
            let (h849, _) = v3.clone();
            h849
        };
        let a851 = 64u64;
        nat_add(a850, a851)?
    }, f243({
        let (_, h852) = v3;
        h852
    }, f244(v0, v1, v2, {
        let (h853, _) = v3;
        h853
    })?)?))
}

pub fn f200() -> Adt20 {
    Adt20::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f201(v0: List<Bytes>, v1: Bytes) -> R<bool> {
    Ok(f67(v1, f52(f149(v0, Bytes::lit(&[0x0a])))?))
}

pub fn f202(v0: List<Bytes>) -> List<Bytes> {
    let m854 = v0;
    match m854.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f26(v1.clone(), f8(Str::lit("authority = []"))) {
                f202(v2)
            } else {
                List::cons(v1, f202(v2))
            }
        }
    }
}

pub fn f203(p857: List<Bytes>, p858: List<Bytes>) -> List<Bytes> {
    let m856 = (p857, p858);
    {
        let mut m856 = m856;
        loop {
            let step = {
                let (v0, v1) = m856;
                let m855 = v0;
                match m855.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m856 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Bytes) -> R<u64> {
    let (h859, _) = f245(Fn8::F246(v0.clone()), {
        let a860 = v0.clone();
        length_bytes(a860)
    }, {
        let a861 = v0;
        length_bytes(a861)
    })?;
    Ok(h859)
}

pub fn f205(v0: Bytes, v1: u64) -> R<u64> {
    let (h862, _) = f245(Fn8::F247(v1, v0), v1, 0u64)?;
    Ok(h862)
}

pub fn f206(v0: Bytes) -> R<Bytes> {
    f248(v0.clone(), 0u64, {
        let a863 = v0;
        length_bytes(a863)
    }, false, false, Bytes::lit(&[]))
}

pub fn f207(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt14, u64)>> {
    let m864 = v2;
    if m864 == 0 {
        Ok(None::<(Adt14, u64)>)
    } else {
        let v4 = m864 - 1;
        let v5: u64 = f249(v0.clone(), v1)?;
        let a865 = v3;
        let a866 = 1u64;
        if nat_eq(a865, a866) {
            let m867 = f207(v0.clone(), v1, v4, 0u64)?;
            match m867 {
                None => {
                    Ok(None::<(Adt14, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f249(v0.clone(), {
                        let (_, h868) = v6.clone();
                        h868
                    })?;
                    let a869 = v7;
                    let a870 = 44u64;
                    if nat_eq(a869, a870) {
                        let m874 = f207(v0, {
                            let a872 = {
                                let (_, h871) = v6;
                                h871
                            };
                            let a873 = 1u64;
                            nat_add(a872, a873)?
                        }, v4, 1u64)?;
                        match m874 {
                            None => {
                                Ok(None::<(Adt14, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt14::C6(std::rc::Rc::new({
                                    let (h875, _) = v6;
                                    h875
                                }), std::rc::Rc::new({
                                    let (h876, _) = v8;
                                    h876
                                })), {
                                    let (_, h877) = v8;
                                    h877
                                })))
                            }
                        }
                    } else {
                        let a878 = v7;
                        let a879 = 93u64;
                        if nat_eq(a878, a879) {
                            Ok(Some((Adt14::C6(std::rc::Rc::new({
                                let (h880, _) = v6;
                                h880
                            }), std::rc::Rc::new(Adt14::C5)), {
                                let a882 = {
                                    let (_, h881) = v6;
                                    h881
                                };
                                let a883 = 1u64;
                                nat_add(a882, a883)?
                            })))
                        } else {
                            Ok(None::<(Adt14, u64)>)
                        }
                    }
                }
            }
        } else {
            let a884 = v3;
            let a885 = 2u64;
            if nat_eq(a884, a885) {
                let m888 = f250(v0.clone(), {
                    let a886 = v1;
                    let a887 = 1u64;
                    nat_add(a886, a887)?
                })?;
                match m888 {
                    None => {
                        Ok(None::<(Adt14, u64)>)
                    }
                    Some(v9) => {
                        let a889 = v5;
                        let a890 = 34u64;
                        if if nat_eq(a889, a890) {
                            let a892 = f249(v0.clone(), {
                                let (h891, _) = v9.clone();
                                h891
                            })?;
                            let a893 = 58u64;
                            nat_eq(a892, a893)
                        } else {
                            false
                        } {
                            let m897 = f207(v0.clone(), {
                                let a895 = {
                                    let (h894, _) = v9;
                                    h894
                                };
                                let a896 = 1u64;
                                nat_add(a895, a896)?
                            }, v4, 0u64)?;
                            match m897 {
                                None => {
                                    Ok(None::<(Adt14, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f249(v0.clone(), {
                                        let (_, h898) = v10.clone();
                                        h898
                                    })?;
                                    let a899 = v11;
                                    let a900 = 44u64;
                                    if nat_eq(a899, a900) {
                                        let m904 = f207(v0, {
                                            let a902 = {
                                                let (_, h901) = v10;
                                                h901
                                            };
                                            let a903 = 1u64;
                                            nat_add(a902, a903)?
                                        }, v4, 2u64)?;
                                        match m904 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt14::C7({
                                                    let (_, h905) = v9;
                                                    h905
                                                }, std::rc::Rc::new({
                                                    let (h906, _) = v10;
                                                    h906
                                                }), std::rc::Rc::new({
                                                    let (h907, _) = v12;
                                                    h907
                                                })), {
                                                    let (_, h908) = v12;
                                                    h908
                                                })))
                                            }
                                        }
                                    } else {
                                        let a909 = v11;
                                        let a910 = 125u64;
                                        if nat_eq(a909, a910) {
                                            Ok(Some((Adt14::C7({
                                                let (_, h911) = v9;
                                                h911
                                            }, std::rc::Rc::new({
                                                let (h912, _) = v10;
                                                h912
                                            }), std::rc::Rc::new(Adt14::C5)), {
                                                let a914 = {
                                                    let (_, h913) = v10;
                                                    h913
                                                };
                                                let a915 = 1u64;
                                                nat_add(a914, a915)?
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
                let a916 = v5;
                let a917 = 34u64;
                if nat_eq(a916, a917) {
                    let m920 = f250(v0, {
                        let a918 = v1;
                        let a919 = 1u64;
                        nat_add(a918, a919)?
                    })?;
                    match m920 {
                        None => {
                            Ok(None::<(Adt14, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt14::C2({
                                let (_, h921) = v13;
                                h921
                            }), {
                                let (h922, _) = v13;
                                h922
                            })))
                        }
                    }
                } else {
                    if f26(f43(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                        Ok(Some((Adt14::C0(true), {
                            let a923 = v1;
                            let a924 = 4u64;
                            nat_add(a923, a924)?
                        })))
                    } else {
                        if f26(f43(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                            Ok(Some((Adt14::C0(false), {
                                let a925 = v1;
                                let a926 = 5u64;
                                nat_add(a925, a926)?
                            })))
                        } else {
                            let a927 = v5;
                            let a928 = 91u64;
                            if nat_eq(a927, a928) {
                                let a931 = f249(v0.clone(), {
                                    let a929 = v1;
                                    let a930 = 1u64;
                                    nat_add(a929, a930)?
                                })?;
                                let a932 = 93u64;
                                if nat_eq(a931, a932) {
                                    Ok(Some((Adt14::C3(std::rc::Rc::new(Adt14::C5)), {
                                        let a933 = v1;
                                        let a934 = 2u64;
                                        nat_add(a933, a934)?
                                    })))
                                } else {
                                    let m937 = f207(v0, {
                                        let a935 = v1;
                                        let a936 = 1u64;
                                        nat_add(a935, a936)?
                                    }, v4, 1u64)?;
                                    match m937 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v14) => {
                                            Ok(Some((Adt14::C3(std::rc::Rc::new({
                                                let (h938, _) = v14;
                                                h938
                                            })), {
                                                let (_, h939) = v14;
                                                h939
                                            })))
                                        }
                                    }
                                }
                            } else {
                                let a940 = v5;
                                let a941 = 123u64;
                                if nat_eq(a940, a941) {
                                    let a944 = f249(v0.clone(), {
                                        let a942 = v1;
                                        let a943 = 1u64;
                                        nat_add(a942, a943)?
                                    })?;
                                    let a945 = 125u64;
                                    if nat_eq(a944, a945) {
                                        Ok(Some((Adt14::C4(std::rc::Rc::new(Adt14::C5)), {
                                            let a946 = v1;
                                            let a947 = 2u64;
                                            nat_add(a946, a947)?
                                        })))
                                    } else {
                                        let m950 = f207(v0, {
                                            let a948 = v1;
                                            let a949 = 1u64;
                                            nat_add(a948, a949)?
                                        }, v4, 2u64)?;
                                        match m950 {
                                            None => {
                                                Ok(None::<(Adt14, u64)>)
                                            }
                                            Some(v15) => {
                                                Ok(Some((Adt14::C4(std::rc::Rc::new({
                                                    let (h951, _) = v15;
                                                    h951
                                                })), {
                                                    let (_, h952) = v15;
                                                    h952
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let m953 = f251(v0, v1)?;
                                    match m953 {
                                        None => {
                                            Ok(None::<(Adt14, u64)>)
                                        }
                                        Some(v16) => {
                                            Ok(Some(({
                                                let (_, h954) = v16;
                                                h954
                                            }, {
                                                let (h955, _) = v16;
                                                h955
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

pub fn f208(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h956, _) = v1.clone();
        h956
    };
    let v3: u64 = {
        let (h957, _) = {
            let (_, h958) = v1.clone();
            h958
        };
        h957
    };
    let v4: List<Bytes> = {
        let (_, h959) = {
            let (_, h960) = v1;
            h960
        };
        h959
    };
    let m961 = f223(v0.clone(), v2);
    match m961 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a962 = v5;
            let a963 = 10u8;
            if equal(a962, a963) {
                Ok(Some(({
                    let a964 = v2;
                    let a965 = 1u64;
                    nat_add(a964, a965)?
                }, ({
                    let a966 = v2;
                    let a967 = 1u64;
                    nat_add(a966, a967)?
                }, List::cons(f252(f43(v0, v3, {
                    let a968 = v2;
                    let a969 = v3;
                    nat_sub(a968, a969)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a970 = v2;
                    let a971 = 1u64;
                    nat_add(a970, a971)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f209(v0: Bytes, p1038: u64, p1039: u64, p1040: bool, p1041: Bytes) -> R<Bytes> {
    let m1037 = (p1038, (p1039, (p1040, p1041)));
    {
        let mut m1037 = m1037;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1037;
                let m972 = v2;
                if m972 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m972 - 1;
                    let m975 = {
                        let a973 = v0.clone();
                        let a974 = v1;
                        index_bytes(a973, a974)
                    };
                    match m975 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v8: u64 = {
                                let v7: u8 = v6;
                                let a1020 = {
                                    let a996 = {
                                        let a984 = {
                                            let a978 = {
                                                let a976 = v7;
                                                let a977 = 128u8;
                                                fixed_u8::bit_and(a976, a977)
                                            };
                                            let a979 = 128u8;
                                            if equal(a978, a979) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a985 = {
                                            let a982 = {
                                                let a980 = v7;
                                                let a981 = 64u8;
                                                fixed_u8::bit_and(a980, a981)
                                            };
                                            let a983 = 64u8;
                                            if equal(a982, a983) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a984, a985)?
                                    };
                                    let a997 = {
                                        let a994 = {
                                            let a988 = {
                                                let a986 = v7;
                                                let a987 = 32u8;
                                                fixed_u8::bit_and(a986, a987)
                                            };
                                            let a989 = 32u8;
                                            if equal(a988, a989) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a995 = {
                                            let a992 = {
                                                let a990 = v7;
                                                let a991 = 16u8;
                                                fixed_u8::bit_and(a990, a991)
                                            };
                                            let a993 = 16u8;
                                            if equal(a992, a993) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a994, a995)?
                                    };
                                    nat_add(a996, a997)?
                                };
                                let a1021 = {
                                    let a1018 = {
                                        let a1006 = {
                                            let a1000 = {
                                                let a998 = v7;
                                                let a999 = 8u8;
                                                fixed_u8::bit_and(a998, a999)
                                            };
                                            let a1001 = 8u8;
                                            if equal(a1000, a1001) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1007 = {
                                            let a1004 = {
                                                let a1002 = v7;
                                                let a1003 = 4u8;
                                                fixed_u8::bit_and(a1002, a1003)
                                            };
                                            let a1005 = 4u8;
                                            if equal(a1004, a1005) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1006, a1007)?
                                    };
                                    let a1019 = {
                                        let a1016 = {
                                            let a1010 = {
                                                let a1008 = v7;
                                                let a1009 = 2u8;
                                                fixed_u8::bit_and(a1008, a1009)
                                            };
                                            let a1011 = 2u8;
                                            if equal(a1010, a1011) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1017 = {
                                            let a1014 = {
                                                let a1012 = v7;
                                                let a1013 = 1u8;
                                                fixed_u8::bit_and(a1012, a1013)
                                            };
                                            let a1015 = 1u8;
                                            if equal(a1014, a1015) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1016, a1017)?
                                    };
                                    nat_add(a1018, a1019)?
                                };
                                nat_add(a1020, a1021)?
                            };
                            if f253(v8) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1022 = v1;
                                    let a1023 = 1u64;
                                    nat_add(a1022, a1023)?
                                }, (v5, (true, v4))))
                            } else {
                                if if f254(v8) {
                                    v3
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1024 = v1;
                                        let a1025 = 1u64;
                                        nat_add(a1024, a1025)?
                                    }, (v5, (false, {
                                        let a1029 = v4;
                                        let a1030 = f43({
                                            let a1026 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1026)
                                        }, {
                                            let a1027 = v8;
                                            let a1028 = 97u64;
                                            nat_sub(a1027, a1028)
                                        }, 1u64);
                                        append_bytes(a1029, a1030)
                                    }))))
                                } else {
                                    if if f254(v8) {
                                        true
                                    } else {
                                        if f255(v8) {
                                            true
                                        } else {
                                            f256(v8)
                                        }
                                    } {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1031 = v1;
                                            let a1032 = 1u64;
                                            nat_add(a1031, a1032)?
                                        }, (v5, (false, {
                                            let a1033 = v4;
                                            let a1034 = f43(v0.clone(), v1, 1u64);
                                            append_bytes(a1033, a1034)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1035 = v1;
                                            let a1036 = 1u64;
                                            nat_add(a1035, a1036)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f210(p1049: u64, p1050: List<Adt21>, p1051: List<Bytes>, p1052: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1048 = (p1049, (p1050, (p1051, p1052)));
    {
        let mut m1048 = m1048;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1048;
                let m1042 = v0;
                if m1042 == 0 {
                    Err::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1042 - 1;
                    let m1043 = v1;
                    match m1043.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1044 = v5;
                            match m1044 {
                                Adt21::C0(v8, v9) => {
                                    if f67(v8.clone(), v2.clone()) {
                                        Ok::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        let m1045 = f257(v8.clone(), v9)?;
                                        match m1045 {
                                            Ok(v11) => {
                                                Ok::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f211({
                                                    let (_, h1046) = v11;
                                                    h1046
                                                }, List::cons(Adt21::C1({
                                                    let (h1047, _) = v11;
                                                    h1047
                                                }), v6)), (List::cons(v8, v2), v3))))
                                            }
                                            Err(v10) => {
                                                Err::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                            }
                                        }
                                    }
                                }
                                Adt21::C1(v7) => {
                                    Ok::<(u64, (List<Adt21>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1048 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: List<Adt21>, v1: List<Adt21>) -> List<Adt21> {
    let m1053 = v0;
    match m1053.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f211(v3, v1))
        }
    }
}

pub fn f212(v0: List<(Bytes, Adt14)>) -> R<List<Adt21>> {
    let m1054 = v0;
    match m1054.uncons() {
        None => {
            Ok(List::<Adt21>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt21::C0(f160({
                let (h1055, _) = v1;
                h1055
            })?, {
                let (_, h1056) = v1;
                h1056
            }), f212(v2)?))
        }
    }
}

pub fn f213(v0: Option<Adt14>) -> List<(Bytes, Adt14)> {
    let m1057 = v0;
    match m1057 {
        None => {
            List::<(Bytes, Adt14)>::nil()
        }
        Some(v1) => {
            f258(v1)
        }
    }
}

pub fn f214(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1058 = v0;
    match m1058.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f229(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f149(f259(f61(List::cons(v1, f260(f41(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f214(v3, v1)?))
            }
        }
    }
}

pub fn f215(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1059 = 0u64;
        let a1060 = v1;
        nat_lt(a1059, a1060)
    } else {
        false
    } {
        let a1061 = Bytes::lit(&[0x2d]);
        let a1062 = f261(v1)?;
        Ok(append_bytes(a1061, a1062))
    } else {
        f261(v1)
    }
}

pub fn f216(v0: Bytes) -> R<Bytes> {
    let a1068 = {
        let a1066 = Bytes::lit(&[0x22]);
        let a1067 = {
            let (_, h1063) = {
                let (h1064, _) = f262(Fn9::F263(v0.clone()), {
                    let a1065 = v0;
                    length_bytes(a1065)
                }, (0u64, Bytes::lit(&[])))?;
                h1064
            };
            h1063
        };
        append_bytes(a1066, a1067)
    };
    let a1069 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1068, a1069))
}

pub fn f217(v0: List<Bytes>) -> Bytes {
    let a1072 = {
        let a1070 = Bytes::lit(&[0x5b]);
        let a1071 = f91(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1070, a1071)
    };
    let a1073 = Bytes::lit(&[0x5d]);
    append_bytes(a1072, a1073)
}

pub fn f218(v0: Adt14) -> R<List<Bytes>> {
    let m1074 = v0;
    match m1074 {
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
        Adt14::C6(r1075, r1076) => {
            let v7 = (*r1075).clone();
            let v8 = (*r1076).clone();
            Ok(List::cons(f165(v7)?, f218(v8)?))
        }
        Adt14::C7(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f219(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f264(Fn10::F265, List::<(Bytes, Bytes)>::nil(), f266(v0)?);
    let v2: List<Bytes> = f267(Fn11::F268, List::<Bytes>::nil(), v1)?;
    let a1079 = {
        let a1077 = Bytes::lit(&[0x7b]);
        let a1078 = f91(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1077, a1078)
    };
    let a1080 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1079, a1080))
}

pub fn f220(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1081 = v0;
    match m1081 {
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
        Adt14::C7(v9, r1082, r1083) => {
            let v10 = (*r1082).clone();
            let v11 = (*r1083).clone();
            Ok(List::cons((v9, f165(v10)?), f220(v11)?))
        }
    }
}

pub fn f221(v0: Option<Adt14>, v1: Bytes) -> Bytes {
    let m1084 = v0;
    match m1084 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            f104(v2, v1)
        }
    }
}

pub fn f222(v0: List<Bytes>) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("[")), List::cons(f149(f259(v0)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))))
}

pub fn f223(v0: Bytes, v1: u64) -> Option<u8> {
    let a1085 = v0;
    let a1086 = v1;
    index_bytes(a1085, a1086)
}

pub fn f224(v0: Adt15, v1: Option<u64>) -> bool {
    let m1087 = v1;
    match m1087 {
        None => {
            true
        }
        Some(v2) => {
            f181(v2, match v0 {
                Adt15::C0(_, h1088, _, _) => {
                    h1088
                }
            })
        }
    }
}

pub fn f225(v0: Adt12) -> Adt18 {
    Adt18::C1({
        let a1090 = match v0 {
            Adt12::C0(_, _, h1089, _, _) => {
                h1089
            }
        };
        length_list(a1090)
    })
}

pub fn f226(v0: Bytes) -> Option<Adt19> {
    if f26(v0.clone(), f8(Str::lit("standard"))) {
        Some(Adt19::C0)
    } else {
        if f26(v0.clone(), f8(Str::lit("pattern"))) {
            Some(Adt19::C1)
        } else {
            if f26(v0, f8(Str::lit("extrapolated"))) {
                Some(Adt19::C2)
            } else {
                None::<Adt19>
            }
        }
    }
}

pub fn f227(v0: Bytes, p1095: List<Bytes>, p1096: u64) -> R<Option<u64>> {
    let m1094 = (p1095, p1096);
    {
        let mut m1094 = m1094;
        loop {
            let step = {
                let (v1, v2) = m1094;
                let m1091 = v1;
                match m1091.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(None::<u64>))
                    }
                    Some((v3, v4)) => {
                        if f26(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<Option<u64>>>(Ok(Some(v2)))
                        } else {
                            Ok::<(List<Bytes>, u64), R<Option<u64>>>((v4, {
                                let a1092 = v2;
                                let a1093 = 1u64;
                                nat_add(a1092, a1093)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1094 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f228(v0: Option<Adt14>) -> Option<u64> {
    let m1097 = v0;
    match m1097 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m1098 = v1;
            match m1098 {
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

pub fn f229(v0: Bytes, v1: Bytes) -> bool {
    let m1103 = {
        let a1100 = v0;
        let a1101 = 0u64;
        let a1102 = {
            let a1099 = v1.clone();
            length_bytes(a1099)
        };
        slice_bytes(a1100, a1101, a1102)
    };
    match m1103 {
        None => {
            false
        }
        Some(v2) => {
            let a1104 = v2;
            let a1105 = v1;
            equal(a1104, a1105)
        }
    }
}

pub fn f230(v0: Fn4, p1109: List<Bytes>, p1110: List<(Bytes, ())>) -> List<Bytes> {
    let m1108 = (p1109, p1110);
    {
        let mut m1108 = m1108;
        loop {
            let step = {
                let (v1, v2) = m1108;
                let m1106 = v2;
                match m1106.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1107: Fn4 = v0.clone();
                            c1107.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1111, _) = v1;
        h1111
    }, v0)
}

pub fn f232(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f269(Fn12::F270, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f233(v0: Fn5, p1115: List<List<(Bytes, ())>>, p1116: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1114 = (p1115, p1116);
    {
        let mut m1114 = m1114;
        loop {
            let step = {
                let (v1, v2) = m1114;
                let m1112 = v2;
                match m1112.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1113: Fn5 = v0.clone();
                            c1113.apply(v1, v3)
                        }, v4))
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

pub fn f234(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f235(v0: Fn6, p1121: u64, p1122: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1120 = (p1121, p1122);
    {
        let mut m1120 = m1120;
        loop {
            let step = {
                let (v1, v2) = m1120;
                let m1117 = v1;
                if m1117 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1117 - 1;
                    let m1119 = {
                        let c1118: Fn6 = v0.clone();
                        c1118.apply(v2.clone())?
                    };
                    match m1119 {
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
                Ok(next) => { m1120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f271(v0)
}

pub fn f237(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f272(Fn13::F273, List::<(Bytes, ())>::nil(), v0)
}

pub fn f238(v0: Fn7, p1126: List<(Bytes, ())>, p1127: List<Bytes>) -> List<(Bytes, ())> {
    let m1125 = (p1126, p1127);
    {
        let mut m1125 = m1125;
        loop {
            let step = {
                let (v1, v2) = m1125;
                let m1123 = v2;
                match m1123.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1124: Fn7 = v0.clone();
                            c1124.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f240(v0: List<Bytes>) -> List<Bytes> {
    let m1128 = v0;
    match m1128.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let a1130 = {
                    let a1129 = v1.clone();
                    length_bytes(a1129)
                };
                let a1131 = 0u64;
                if nat_eq(a1130, a1131) {
                    f8(Str::lit(">\u{a}"))
                } else {
                    f7(List::cons(f8(Str::lit("> ")), List::cons(v1, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))
                }
            }, f240(v2))
        }
    }
}

pub fn f241(v0: u32) -> Bytes {
    let a1134 = {
        let a1132 = v0;
        let a1133 = 8u32;
        fixed_u32::bit_and(a1132, a1133)
    };
    let a1135 = 0u32;
    if equal(a1134, a1135) {
        let a1138 = {
            let a1136 = v0;
            let a1137 = 4u32;
            fixed_u32::bit_and(a1136, a1137)
        };
        let a1139 = 0u32;
        if equal(a1138, a1139) {
            let a1142 = {
                let a1140 = v0;
                let a1141 = 2u32;
                fixed_u32::bit_and(a1140, a1141)
            };
            let a1143 = 0u32;
            if equal(a1142, a1143) {
                let a1146 = {
                    let a1144 = v0;
                    let a1145 = 1u32;
                    fixed_u32::bit_and(a1144, a1145)
                };
                let a1147 = 0u32;
                if equal(a1146, a1147) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1150 = {
                    let a1148 = v0;
                    let a1149 = 1u32;
                    fixed_u32::bit_and(a1148, a1149)
                };
                let a1151 = 0u32;
                if equal(a1150, a1151) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1154 = {
                let a1152 = v0;
                let a1153 = 2u32;
                fixed_u32::bit_and(a1152, a1153)
            };
            let a1155 = 0u32;
            if equal(a1154, a1155) {
                let a1158 = {
                    let a1156 = v0;
                    let a1157 = 1u32;
                    fixed_u32::bit_and(a1156, a1157)
                };
                let a1159 = 0u32;
                if equal(a1158, a1159) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1162 = {
                    let a1160 = v0;
                    let a1161 = 1u32;
                    fixed_u32::bit_and(a1160, a1161)
                };
                let a1163 = 0u32;
                if equal(a1162, a1163) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1166 = {
            let a1164 = v0;
            let a1165 = 4u32;
            fixed_u32::bit_and(a1164, a1165)
        };
        let a1167 = 0u32;
        if equal(a1166, a1167) {
            let a1170 = {
                let a1168 = v0;
                let a1169 = 2u32;
                fixed_u32::bit_and(a1168, a1169)
            };
            let a1171 = 0u32;
            if equal(a1170, a1171) {
                let a1174 = {
                    let a1172 = v0;
                    let a1173 = 1u32;
                    fixed_u32::bit_and(a1172, a1173)
                };
                let a1175 = 0u32;
                if equal(a1174, a1175) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1178 = {
                    let a1176 = v0;
                    let a1177 = 1u32;
                    fixed_u32::bit_and(a1176, a1177)
                };
                let a1179 = 0u32;
                if equal(a1178, a1179) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1182 = {
                let a1180 = v0;
                let a1181 = 2u32;
                fixed_u32::bit_and(a1180, a1181)
            };
            let a1183 = 0u32;
            if equal(a1182, a1183) {
                let a1186 = {
                    let a1184 = v0;
                    let a1185 = 1u32;
                    fixed_u32::bit_and(a1184, a1185)
                };
                let a1187 = 0u32;
                if equal(a1186, a1187) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1190 = {
                    let a1188 = v0;
                    let a1189 = 1u32;
                    fixed_u32::bit_and(a1188, a1189)
                };
                let a1191 = 0u32;
                if equal(a1190, a1191) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f242(v0: u32, v1: u32) -> u32 {
    let m1194 = {
        let a1192 = v0;
        let a1193 = v1;
        fixed_u32::shift_right(a1192, a1193)
    };
    match m1194 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f243(v0: Adt20, v1: Adt22) -> R<Adt20> {
    let v2: Adt23 = f274(Fn14::F275, 64u64, Adt23::C0(match v0.clone() {
        Adt20::C0(h1195, _, _, _, _, _, _, _) => {
            h1195
        }
    }, match v0.clone() {
        Adt20::C0(_, h1196, _, _, _, _, _, _) => {
            h1196
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h1197, _, _, _, _, _) => {
            h1197
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1198, _, _, _, _) => {
            h1198
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, h1199, _, _, _) => {
            h1199
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1200, _, _) => {
            h1200
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1201, _) => {
            h1201
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, _, h1202) => {
            h1202
        }
    }, v1, 0u64))?;
    Ok(Adt20::C0(f276(match v0.clone() {
        Adt20::C0(h1203, _, _, _, _, _, _, _) => {
            h1203
        }
    }, match v2.clone() {
        Adt23::C0(h1204, _, _, _, _, _, _, _, _, _) => {
            h1204
        }
    }), f276(match v0.clone() {
        Adt20::C0(_, h1205, _, _, _, _, _, _) => {
            h1205
        }
    }, match v2.clone() {
        Adt23::C0(_, h1206, _, _, _, _, _, _, _, _) => {
            h1206
        }
    }), f276(match v0.clone() {
        Adt20::C0(_, _, h1207, _, _, _, _, _) => {
            h1207
        }
    }, match v2.clone() {
        Adt23::C0(_, _, h1208, _, _, _, _, _, _, _) => {
            h1208
        }
    }), f276(match v0.clone() {
        Adt20::C0(_, _, _, h1209, _, _, _, _) => {
            h1209
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, h1210, _, _, _, _, _, _) => {
            h1210
        }
    }), f276(match v0.clone() {
        Adt20::C0(_, _, _, _, h1211, _, _, _) => {
            h1211
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, h1212, _, _, _, _, _) => {
            h1212
        }
    }), f276(match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1213, _, _) => {
            h1213
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, h1214, _, _, _, _) => {
            h1214
        }
    }), f276(match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1215, _) => {
            h1215
        }
    }, match v2.clone() {
        Adt23::C0(_, _, _, _, _, _, h1216, _, _, _) => {
            h1216
        }
    }), f276(match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h1217) => {
            h1217
        }
    }, match v2 {
        Adt23::C0(_, _, _, _, _, _, _, h1218, _, _) => {
            h1218
        }
    })))
}

pub fn f244(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    Ok(Adt22::C0(f277(v0.clone(), v1, v2, v3)?, f277(v0.clone(), v1, v2, {
        let a1219 = v3;
        let a1220 = 4u64;
        nat_add(a1219, a1220)?
    })?, f277(v0.clone(), v1, v2, {
        let a1221 = v3;
        let a1222 = 8u64;
        nat_add(a1221, a1222)?
    })?, f277(v0.clone(), v1, v2, {
        let a1223 = v3;
        let a1224 = 12u64;
        nat_add(a1223, a1224)?
    })?, f277(v0.clone(), v1, v2, {
        let a1225 = v3;
        let a1226 = 16u64;
        nat_add(a1225, a1226)?
    })?, f277(v0.clone(), v1, v2, {
        let a1227 = v3;
        let a1228 = 20u64;
        nat_add(a1227, a1228)?
    })?, f277(v0.clone(), v1, v2, {
        let a1229 = v3;
        let a1230 = 24u64;
        nat_add(a1229, a1230)?
    })?, f277(v0.clone(), v1, v2, {
        let a1231 = v3;
        let a1232 = 28u64;
        nat_add(a1231, a1232)?
    })?, f277(v0.clone(), v1, v2, {
        let a1233 = v3;
        let a1234 = 32u64;
        nat_add(a1233, a1234)?
    })?, f277(v0.clone(), v1, v2, {
        let a1235 = v3;
        let a1236 = 36u64;
        nat_add(a1235, a1236)?
    })?, f277(v0.clone(), v1, v2, {
        let a1237 = v3;
        let a1238 = 40u64;
        nat_add(a1237, a1238)?
    })?, f277(v0.clone(), v1, v2, {
        let a1239 = v3;
        let a1240 = 44u64;
        nat_add(a1239, a1240)?
    })?, f277(v0.clone(), v1, v2, {
        let a1241 = v3;
        let a1242 = 48u64;
        nat_add(a1241, a1242)?
    })?, f277(v0.clone(), v1, v2, {
        let a1243 = v3;
        let a1244 = 52u64;
        nat_add(a1243, a1244)?
    })?, f277(v0.clone(), v1, v2, {
        let a1245 = v3;
        let a1246 = 56u64;
        nat_add(a1245, a1246)?
    })?, f277(v0, v1, v2, {
        let a1247 = v3;
        let a1248 = 60u64;
        nat_add(a1247, a1248)?
    })?))
}

pub fn f245(v0: Fn8, p1253: u64, p1254: u64) -> R<(u64, bool)> {
    let m1252 = (p1253, p1254);
    {
        let mut m1252 = m1252;
        loop {
            let step = {
                let (v1, v2) = m1252;
                let m1249 = v1;
                if m1249 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1249 - 1;
                    let m1251 = {
                        let c1250: Fn8 = v0.clone();
                        c1250.apply(v2)?
                    };
                    match m1251 {
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
                Ok(next) => { m1252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: Bytes, v1: u64) -> Option<u64> {
    let a1255 = 0u64;
    let a1256 = v1;
    if nat_lt(a1255, a1256) {
        if f278(v0, {
            let a1257 = v1;
            let a1258 = 1u64;
            nat_sub(a1257, a1258)
        }) {
            Some({
                let a1259 = v1;
                let a1260 = 1u64;
                nat_sub(a1259, a1260)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f247(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1261 = v2;
    let a1262 = v0;
    if nat_lt(a1261, a1262) {
        if f278(v1, v2) {
            Ok(Some({
                let a1263 = v2;
                let a1264 = 1u64;
                nat_add(a1263, a1264)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f248(v0: Bytes, p1344: u64, p1345: u64, p1346: bool, p1347: bool, p1348: Bytes) -> R<Bytes> {
    let m1343 = (p1344, (p1345, (p1346, (p1347, p1348))));
    {
        let mut m1343 = m1343;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, v5)))) = m1343;
                let m1265 = v2;
                if m1265 == 0 {
                    Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                } else {
                    let v6 = m1265 - 1;
                    let m1268 = {
                        let a1266 = v0.clone();
                        let a1267 = v1;
                        index_bytes(a1266, a1267)
                    };
                    match m1268 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(Ok(v5))
                        }
                        Some(v7) => {
                            let v9: u64 = {
                                let v8: u8 = v7;
                                let a1313 = {
                                    let a1289 = {
                                        let a1277 = {
                                            let a1271 = {
                                                let a1269 = v8;
                                                let a1270 = 128u8;
                                                fixed_u8::bit_and(a1269, a1270)
                                            };
                                            let a1272 = 128u8;
                                            if equal(a1271, a1272) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1278 = {
                                            let a1275 = {
                                                let a1273 = v8;
                                                let a1274 = 64u8;
                                                fixed_u8::bit_and(a1273, a1274)
                                            };
                                            let a1276 = 64u8;
                                            if equal(a1275, a1276) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1277, a1278)?
                                    };
                                    let a1290 = {
                                        let a1287 = {
                                            let a1281 = {
                                                let a1279 = v8;
                                                let a1280 = 32u8;
                                                fixed_u8::bit_and(a1279, a1280)
                                            };
                                            let a1282 = 32u8;
                                            if equal(a1281, a1282) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1288 = {
                                            let a1285 = {
                                                let a1283 = v8;
                                                let a1284 = 16u8;
                                                fixed_u8::bit_and(a1283, a1284)
                                            };
                                            let a1286 = 16u8;
                                            if equal(a1285, a1286) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1287, a1288)?
                                    };
                                    nat_add(a1289, a1290)?
                                };
                                let a1314 = {
                                    let a1311 = {
                                        let a1299 = {
                                            let a1293 = {
                                                let a1291 = v8;
                                                let a1292 = 8u8;
                                                fixed_u8::bit_and(a1291, a1292)
                                            };
                                            let a1294 = 8u8;
                                            if equal(a1293, a1294) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1300 = {
                                            let a1297 = {
                                                let a1295 = v8;
                                                let a1296 = 4u8;
                                                fixed_u8::bit_and(a1295, a1296)
                                            };
                                            let a1298 = 4u8;
                                            if equal(a1297, a1298) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1299, a1300)?
                                    };
                                    let a1312 = {
                                        let a1309 = {
                                            let a1303 = {
                                                let a1301 = v8;
                                                let a1302 = 2u8;
                                                fixed_u8::bit_and(a1301, a1302)
                                            };
                                            let a1304 = 2u8;
                                            if equal(a1303, a1304) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a1310 = {
                                            let a1307 = {
                                                let a1305 = v8;
                                                let a1306 = 1u8;
                                                fixed_u8::bit_and(a1305, a1306)
                                            };
                                            let a1308 = 1u8;
                                            if equal(a1307, a1308) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a1309, a1310)?
                                    };
                                    nat_add(a1311, a1312)?
                                };
                                nat_add(a1313, a1314)?
                            };
                            let v10: Bytes = f43(v0.clone(), v1, 1u64);
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                    let a1315 = v1;
                                    let a1316 = 1u64;
                                    nat_add(a1315, a1316)?
                                }, (v6, (if v4 {
                                    true
                                } else {
                                    let a1317 = v9;
                                    let a1318 = 34u64;
                                    !nat_eq(a1317, a1318)
                                }, (if v4 {
                                    false
                                } else {
                                    let a1319 = v9;
                                    let a1320 = 92u64;
                                    nat_eq(a1319, a1320)
                                }, {
                                    let a1321 = v5;
                                    let a1322 = v10;
                                    append_bytes(a1321, a1322)
                                })))))
                            } else {
                                let a1323 = v9;
                                let a1324 = 32u64;
                                if if if if nat_eq(a1323, a1324) {
                                    true
                                } else {
                                    let a1325 = v9;
                                    let a1326 = 9u64;
                                    nat_eq(a1325, a1326)
                                } {
                                    true
                                } else {
                                    let a1327 = v9;
                                    let a1328 = 10u64;
                                    nat_eq(a1327, a1328)
                                } {
                                    true
                                } else {
                                    let a1329 = v9;
                                    let a1330 = 13u64;
                                    nat_eq(a1329, a1330)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                        let a1331 = v1;
                                        let a1332 = 1u64;
                                        nat_add(a1331, a1332)?
                                    }, (v6, (false, (false, v5)))))
                                } else {
                                    if f26(f43(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1333 = v1;
                                            let a1334 = 4u64;
                                            nat_add(a1333, a1334)?
                                        }, (v6, (false, (false, {
                                            let a1335 = v5;
                                            let a1336 = f8(Str::lit("[]"));
                                            append_bytes(a1335, a1336)
                                        })))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, (bool, Bytes)))), R<Bytes>>(({
                                            let a1337 = v1;
                                            let a1338 = 1u64;
                                            nat_add(a1337, a1338)?
                                        }, (v6, ({
                                            let a1339 = v9;
                                            let a1340 = 34u64;
                                            nat_eq(a1339, a1340)
                                        }, (false, {
                                            let a1341 = v5;
                                            let a1342 = v10;
                                            append_bytes(a1341, a1342)
                                        })))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f249(v0: Bytes, v1: u64) -> R<u64> {
    let m1351 = {
        let a1349 = v0;
        let a1350 = v1;
        index_bytes(a1349, a1350)
    };
    match m1351 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a1396 = {
                let a1372 = {
                    let a1360 = {
                        let a1354 = {
                            let a1352 = v3;
                            let a1353 = 128u8;
                            fixed_u8::bit_and(a1352, a1353)
                        };
                        let a1355 = 128u8;
                        if equal(a1354, a1355) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a1361 = {
                        let a1358 = {
                            let a1356 = v3;
                            let a1357 = 64u8;
                            fixed_u8::bit_and(a1356, a1357)
                        };
                        let a1359 = 64u8;
                        if equal(a1358, a1359) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1360, a1361)?
                };
                let a1373 = {
                    let a1370 = {
                        let a1364 = {
                            let a1362 = v3;
                            let a1363 = 32u8;
                            fixed_u8::bit_and(a1362, a1363)
                        };
                        let a1365 = 32u8;
                        if equal(a1364, a1365) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a1371 = {
                        let a1368 = {
                            let a1366 = v3;
                            let a1367 = 16u8;
                            fixed_u8::bit_and(a1366, a1367)
                        };
                        let a1369 = 16u8;
                        if equal(a1368, a1369) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1370, a1371)?
                };
                nat_add(a1372, a1373)?
            };
            let a1397 = {
                let a1394 = {
                    let a1382 = {
                        let a1376 = {
                            let a1374 = v3;
                            let a1375 = 8u8;
                            fixed_u8::bit_and(a1374, a1375)
                        };
                        let a1377 = 8u8;
                        if equal(a1376, a1377) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a1383 = {
                        let a1380 = {
                            let a1378 = v3;
                            let a1379 = 4u8;
                            fixed_u8::bit_and(a1378, a1379)
                        };
                        let a1381 = 4u8;
                        if equal(a1380, a1381) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1382, a1383)?
                };
                let a1395 = {
                    let a1392 = {
                        let a1386 = {
                            let a1384 = v3;
                            let a1385 = 2u8;
                            fixed_u8::bit_and(a1384, a1385)
                        };
                        let a1387 = 2u8;
                        if equal(a1386, a1387) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a1393 = {
                        let a1390 = {
                            let a1388 = v3;
                            let a1389 = 1u8;
                            fixed_u8::bit_and(a1388, a1389)
                        };
                        let a1391 = 1u8;
                        if equal(a1390, a1391) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a1392, a1393)?
                };
                nat_add(a1394, a1395)?
            };
            nat_add(a1396, a1397)
        }
    }
}

pub fn f250(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: ((u64, (Bytes, u64)), bool) = f279(Fn15::F280(v0.clone()), {
        let a1399 = {
            let a1398 = v0;
            length_bytes(a1398)
        };
        let a1400 = 1u64;
        nat_add(a1399, a1400)?
    }, (v1, (Bytes::lit(&[]), 0u64)))?;
    let a1404 = {
        let (_, h1401) = {
            let (_, h1402) = {
                let (h1403, _) = v2.clone();
                h1403
            };
            h1402
        };
        h1401
    };
    let a1405 = 1u64;
    if nat_eq(a1404, a1405) {
        Ok(Some(({
            let (h1406, _) = {
                let (h1407, _) = v2.clone();
                h1407
            };
            h1406
        }, {
            let (h1408, _) = {
                let (_, h1409) = {
                    let (h1410, _) = v2;
                    h1410
                };
                h1409
            };
            h1408
        })))
    } else {
        Ok(None::<(u64, Bytes)>)
    }
}

pub fn f251(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a1411 = f249(v0.clone(), v1)?;
        let a1412 = 45u64;
        nat_eq(a1411, a1412)
    };
    let v3: u64 = if v2 {
        let a1413 = v1;
        let a1414 = 1u64;
        nat_add(a1413, a1414)?
    } else {
        v1
    };
    let v4: u64 = f281(v0.clone(), v3)?;
    let a1417 = 19u64;
    let a1418 = {
        let a1415 = v4;
        let a1416 = v3;
        nat_sub(a1415, a1416)
    };
    if nat_lt(a1417, a1418) {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m1421 = f282(f43(v0, v3, {
            let a1419 = v4;
            let a1420 = v3;
            nat_sub(a1419, a1420)
        }))?;
        match m1421 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v5) => {
                if f283(v2, v5) {
                    Ok(Some((v4, Adt14::C1(v2, v5))))
                } else {
                    Ok(None::<(u64, Adt14)>)
                }
            }
        }
    }
}

pub fn f252(v0: Bytes) -> Bytes {
    if f118(v0.clone(), Bytes::lit(&[0x0d])) {
        f43(v0.clone(), 0u64, {
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

pub fn f253(v0: u64) -> bool {
    let a1425 = v0;
    let a1426 = 95u64;
    if nat_eq(a1425, a1426) {
        true
    } else {
        let a1427 = v0;
        let a1428 = 45u64;
        if nat_eq(a1427, a1428) {
            true
        } else {
            let a1429 = v0;
            let a1430 = 32u64;
            if nat_eq(a1429, a1430) {
                true
            } else {
                let a1431 = v0;
                let a1432 = 46u64;
                if nat_eq(a1431, a1432) {
                    true
                } else {
                    let a1433 = v0;
                    let a1434 = 47u64;
                    nat_eq(a1433, a1434)
                }
            }
        }
    }
}

pub fn f254(v0: u64) -> bool {
    let a1435 = 97u64;
    let a1436 = v0;
    if nat_le(a1435, a1436) {
        let a1437 = v0;
        let a1438 = 122u64;
        nat_le(a1437, a1438)
    } else {
        false
    }
}

pub fn f255(v0: u64) -> bool {
    let a1439 = 65u64;
    let a1440 = v0;
    if nat_le(a1439, a1440) {
        let a1441 = v0;
        let a1442 = 90u64;
        nat_le(a1441, a1442)
    } else {
        false
    }
}

pub fn f256(v0: u64) -> bool {
    let a1443 = 48u64;
    let a1444 = v0;
    if nat_le(a1443, a1444) {
        let a1445 = v0;
        let a1446 = 57u64;
        nat_le(a1445, a1446)
    } else {
        false
    }
}

pub fn f257(v0: Bytes, v1: Adt14) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m1447 = f101(v1.clone(), f8(Str::lit("enum")));
    match m1447 {
        None => {
            let m1449 = f287(v0.clone(), f213(f101(v1.clone(), f8(Str::lit("properties")))), f57(f105(f101(v1, f8(Str::lit("required"))))))?;
            match m1449 {
                Ok(v6) => {
                    Ok(f286(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                        let m1451 = {
                            let (h1450, _) = v6.clone();
                            h1450
                        };
                        match m1451.uncons() {
                            None => {
                                Bytes::lit(&[])
                            }
                            Some((_, _)) => {
                                f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f149({
                                    let (h1452, _) = v6;
                                    h1452
                                }, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                            }
                        }
                    }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), {
                        let (_, h1453) = v6;
                        h1453
                    }))
                }
                Err(v5) => {
                    Ok(Err::<(Bytes, List<Adt21>), Bytes>(v5))
                }
            }
        }
        Some(v2) => {
            let m1448 = f284(f57(f166(v2)))?;
            match m1448 {
                Ok(v4) => {
                    Ok(f286(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f149(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt21>::nil()))
                }
                Err(v3) => {
                    Ok(f285(v3, v0))
                }
            }
        }
    }
}

pub fn f258(v0: Adt14) -> List<(Bytes, Adt14)> {
    let m1454 = v0;
    match m1454 {
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
        Adt14::C4(r1455) => {
            let v4 = (*r1455).clone();
            f258(v4)
        }
        Adt14::C5 => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C6(_, _) => {
            List::<(Bytes, Adt14)>::nil()
        }
        Adt14::C7(v1, r1456, r1457) => {
            let v2 = (*r1456).clone();
            let v3 = (*r1457).clone();
            List::cons((v1, v2), f258(v3))
        }
    }
}

pub fn f259(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1458 = v0;
    match m1458.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f143(v1)?, f259(v2)?))
        }
    }
}

pub fn f260(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1459 = v0;
    match m1459.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f260(v3, false))
            } else {
                f260(v3, true)
            }
        }
    }
}

pub fn f261(v0: u64) -> R<Bytes> {
    f288(f289(), 10u64, v0)
}

pub fn f262(v0: Fn9, p1464: u64, p1465: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1463 = (p1464, p1465);
    {
        let mut m1463 = m1463;
        loop {
            let step = {
                let (v1, v2) = m1463;
                let m1460 = v1;
                if m1460 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1460 - 1;
                    let m1462 = {
                        let c1461: Fn9 = v0.clone();
                        c1461.apply(v2.clone())?
                    };
                    match m1462 {
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
                Ok(next) => { m1463 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f263(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f290(v0, v1)
}

pub fn f264(v0: Fn10, p1469: List<(Bytes, Bytes)>, p1470: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1468 = (p1469, p1470);
    {
        let mut m1468 = m1468;
        loop {
            let step = {
                let (v1, v2) = m1468;
                let m1466 = v2;
                match m1466.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1467: Fn10 = v0.clone();
                            c1467.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f265(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f291(v0, v1)
}

pub fn f266(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f292(v0)
}

pub fn f267(v0: Fn11, p1474: List<Bytes>, p1475: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1473 = (p1474, p1475);
    {
        let mut m1473 = m1473;
        loop {
            let step = {
                let (v1, v2) = m1473;
                let m1471 = v2;
                match m1471.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1472: Fn11 = v0.clone();
                            c1472.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1473 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f268(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f293(v1)?, v0))
}

pub fn f269(v0: Fn12, p1479: List<List<(Bytes, ())>>, p1480: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1478 = (p1479, p1480);
    {
        let mut m1478 = m1478;
        loop {
            let step = {
                let (v1, v2) = m1478;
                let m1476 = v2;
                match m1476.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1477: Fn12 = v0.clone();
                            c1477.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1478 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f271(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1482 = {
        let a1481 = v0.clone();
        length_list(a1481)
    };
    let a1483 = 1u64;
    if nat_le(a1482, a1483) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f294(v0)?))
    }
}

pub fn f272(v0: Fn13, p1487: List<(Bytes, ())>, p1488: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1486 = (p1487, p1488);
    {
        let mut m1486 = m1486;
        loop {
            let step = {
                let (v1, v2) = m1486;
                let m1484 = v2;
                match m1484.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1485: Fn13 = v0.clone();
                            c1485.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1486 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f273(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f274(v0: Fn14, p1492: u64, p1493: Adt23) -> R<Adt23> {
    let m1491 = (p1492, p1493);
    {
        let mut m1491 = m1491;
        loop {
            let step = {
                let (v1, v2) = m1491;
                let m1489 = v1;
                if m1489 == 0 {
                    Err::<(u64, Adt23), R<Adt23>>(Ok(v2))
                } else {
                    let v3 = m1489 - 1;
                    Ok::<(u64, Adt23), R<Adt23>>((v3, {
                        let c1490: Fn14 = v0.clone();
                        c1490.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Adt23) -> R<Adt23> {
    f295(v0)
}

pub fn f276(v0: u32, v1: u32) -> u32 {
    let m1496 = {
        let a1494 = f296(v0);
        let a1495 = f296(v1);
        fixed_u64::checked_add(a1494, a1495)
    };
    match m1496 {
        None => {
            0u32
        }
        Some(v2) => {
            f297(v2)
        }
    }
}

pub fn f277(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1499 = {
        let a1497 = v3;
        let a1498 = 8u64;
        nat_add(a1497, a1498)?
    };
    let a1500 = v2;
    if nat_eq(a1499, a1500) {
        Ok(f298({
            let a1503 = {
                let a1501 = v1;
                let a1502 = 8u64;
                nat_mul(a1501, a1502)?
            };
            let a1504 = 4294967296u64;
            let a1505 = 0u64;
            nat_quot(a1503, a1504, a1505)
        }))
    } else {
        let a1508 = {
            let a1506 = v3;
            let a1507 = 4u64;
            nat_add(a1506, a1507)?
        };
        let a1509 = v2;
        if nat_eq(a1508, a1509) {
            Ok(f298({
                let a1512 = {
                    let a1510 = v1;
                    let a1511 = 8u64;
                    nat_mul(a1510, a1511)?
                };
                let a1513 = 4294967296u64;
                let a1514 = 0u64;
                nat_rem(a1512, a1513, a1514)
            }))
        } else {
            Ok(f299(f300(v0.clone(), v1, v3), f300(v0.clone(), v1, {
                let a1515 = v3;
                let a1516 = 1u64;
                nat_add(a1515, a1516)?
            }), f300(v0.clone(), v1, {
                let a1517 = v3;
                let a1518 = 2u64;
                nat_add(a1517, a1518)?
            }), f300(v0, v1, {
                let a1519 = v3;
                let a1520 = 3u64;
                nat_add(a1519, a1520)?
            })))
        }
    }
}

pub fn f278(v0: Bytes, v1: u64) -> bool {
    let m1521 = f223(v0, v1);
    match m1521 {
        None => {
            false
        }
        Some(v2) => {
            f301(v2)
        }
    }
}

pub fn f279(v0: Fn15, p1526: u64, p1527: (u64, (Bytes, u64))) -> R<((u64, (Bytes, u64)), bool)> {
    let m1525 = (p1526, p1527);
    {
        let mut m1525 = m1525;
        loop {
            let step = {
                let (v1, v2) = m1525;
                let m1522 = v1;
                if m1522 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<((u64, (Bytes, u64)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1522 - 1;
                    let m1524 = {
                        let c1523: Fn15 = v0.clone();
                        c1523.apply(v2.clone())?
                    };
                    match m1524 {
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
                Ok(next) => { m1525 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    f302(v0, v1)
}

pub fn f281(v0: Bytes, v1: u64) -> R<u64> {
    let (h1528, _) = f245(Fn8::F303(v0.clone()), {
        let a1530 = {
            let a1529 = v0;
            length_bytes(a1529)
        };
        let a1531 = 1u64;
        nat_add(a1530, a1531)?
    }, v1)?;
    Ok(h1528)
}

pub fn f282(v0: Bytes) -> R<Option<u64>> {
    if f304(v0.clone())? {
        Ok(Some({
            let (_, h1532) = {
                let (h1533, _) = f305(Fn16::F306(v0.clone()), {
                    let a1534 = v0;
                    length_bytes(a1534)
                }, (0u64, 0u64))?;
                h1533
            };
            h1532
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f283(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1535 = 0u64;
        let a1536 = v1;
        if nat_lt(a1535, a1536) {
            let a1537 = v1;
            let a1538 = 9223372036854775808u64;
            nat_le(a1537, a1538)
        } else {
            false
        }
    } else {
        let a1539 = v1;
        let a1540 = 9223372036854775807u64;
        nat_le(a1539, a1540)
    }
}

pub fn f284(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1541 = v0;
    match m1541.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v1, v2)) => {
            let v3: Bytes = f307(v1.clone())?;
            if f308(v3.clone())? {
                let m1542 = f284(v2)?;
                match m1542 {
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

pub fn f285(v0: Bytes, v1: Bytes) -> Result<(Bytes, List<Adt21>), Bytes> {
    Err::<(Bytes, List<Adt21>), Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f286(v0: Bytes, v1: List<Adt21>) -> Result<(Bytes, List<Adt21>), Bytes> {
    Ok::<(Bytes, List<Adt21>), Bytes>((v0, v1))
}

pub fn f287(v0: Bytes, v1: List<(Bytes, Adt14)>, v2: List<Bytes>) -> R<Result<(List<Bytes>, List<Adt21>), Bytes>> {
    let m1543 = v1;
    match m1543.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Adt21>), Bytes>((List::<Bytes>::nil(), List::<Adt21>::nil())))
        }
        Some((v3, v4)) => {
            let v5: Bytes = f307({
                let (h1544, _) = v3.clone();
                h1544
            })?;
            if f308(v5.clone())? {
                let m1549 = f309({
                    let (_, h1545) = v3.clone();
                    h1545
                }, {
                    let a1547 = v0.clone();
                    let a1548 = f160({
                        let (h1546, _) = v3.clone();
                        h1546
                    })?;
                    append_bytes(a1547, a1548)
                })?;
                match m1549 {
                    Ok(v7) => {
                        let v8: Bytes = if f67({
                            let (h1550, _) = v3;
                            h1550
                        }, v2.clone()) {
                            let (h1551, _) = v7;
                            h1551
                        } else {
                            f7(List::cons(f8(Str::lit("(Option ")), List::cons({
                                let (h1552, _) = v7;
                                h1552
                            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                        };
                        let m1553 = f287(v0, v4, v2)?;
                        match m1553 {
                            Ok(v10) => {
                                Ok(Ok::<(List<Bytes>, List<Adt21>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons(v8, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                    let (h1554, _) = v10;
                                    h1554
                                }), f211({
                                    let (_, h1555) = v7;
                                    h1555
                                }, {
                                    let (_, h1556) = v10;
                                    h1556
                                }))))
                            }
                            Err(v9) => {
                                Ok(Err::<(List<Bytes>, List<Adt21>), Bytes>(v9))
                            }
                        }
                    }
                    Err(v6) => {
                        Ok(Err::<(List<Bytes>, List<Adt21>), Bytes>(v6))
                    }
                }
            } else {
                Ok(Err::<(List<Bytes>, List<Adt21>), Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                    let (h1557, _) = v3;
                    h1557
                }, List::<Bytes>::nil())))))
            }
        }
    }
}

pub fn f288(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a1558 = v2;
    let a1559 = 0u64;
    if nat_eq(a1558, a1559) {
        Ok(f310(v0, 0u64))
    } else {
        let (_, h1560) = {
            let (h1561, _) = f262(Fn9::F311(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h1561
        };
        Ok(h1560)
    }
}

pub fn f289() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f290(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m1565 = {
        let a1563 = v0.clone();
        let a1564 = {
            let (h1562, _) = v1.clone();
            h1562
        };
        index_bytes(a1563, a1564)
    };
    match m1565 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1567 = {
                    let (h1566, _) = v1.clone();
                    h1566
                };
                let a1568 = 1u64;
                nat_add(a1567, a1568)?
            }, {
                let a1571 = {
                    let (_, h1569) = v1;
                    h1569
                };
                let a1572 = f312(v0, {
                    let (h1570, _) = v1;
                    h1570
                }, v2)?;
                append_bytes(a1571, a1572)
            })))
        }
    }
}

pub fn f291(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1573 = v0.clone();
    match m1573.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1576 = {
                let (h1574, _) = v2;
                h1574
            };
            let a1577 = {
                let (h1575, _) = v1.clone();
                h1575
            };
            if equal(a1576, a1577) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f292(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f313(f314(Fn17::F315, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1582 = {
        let (h1578, _) = f316(Fn18::F317, {
            let a1580 = {
                let a1579 = v0;
                length_list(a1579)
            };
            let a1581 = 1u64;
            nat_add(a1580, a1581)?
        }, v1)?;
        h1578
    };
    match m1582.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f293(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1587 = {
        let a1584 = f216({
            let (h1583, _) = v0;
            h1583
        })?;
        let a1585 = Bytes::lit(&[0x3a]);
        append_bytes(a1584, a1585)
    };
    let a1588 = {
        let (_, h1586) = v0;
        h1586
    };
    Ok(append_bytes(a1587, a1588))
}

pub fn f294(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f232({
        let (_, h1589) = {
            let (h1590, _) = f318(Fn19::F319, {
                let a1592 = {
                    let a1591 = v0.clone();
                    length_list(a1591)
                };
                let a1593 = 1u64;
                nat_add(a1592, a1593)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1590
        };
        h1589
    }))
}

pub fn f295(v0: Adt23) -> R<Adt23> {
    let v1: u32 = f320(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, _, h1594) => {
            h1594
        }
    });
    let v2: u32 = f276(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, h1595, _, _) => {
            h1595
        }
    }, f276(f321(match v0.clone() {
        Adt23::C0(_, _, _, _, h1596, _, _, _, _, _) => {
            h1596
        }
    }), f276(f322(match v0.clone() {
        Adt23::C0(_, _, _, _, h1597, _, _, _, _, _) => {
            h1597
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1598, _, _, _, _) => {
            h1598
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1599, _, _, _) => {
            h1599
        }
    }), f276(v1, {
        let m1602 = match v0.clone() {
            Adt23::C0(_, _, _, _, _, _, _, _, h1601, _) => {
                h1601
            }
        };
        match m1602 {
            Adt22::C0(h1600, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1600
            }
        }
    }))));
    let v3: u32 = f276(f323(match v0.clone() {
        Adt23::C0(h1603, _, _, _, _, _, _, _, _, _) => {
            h1603
        }
    }), f324(match v0.clone() {
        Adt23::C0(h1604, _, _, _, _, _, _, _, _, _) => {
            h1604
        }
    }, match v0.clone() {
        Adt23::C0(_, h1605, _, _, _, _, _, _, _, _) => {
            h1605
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1606, _, _, _, _, _, _, _) => {
            h1606
        }
    }));
    Ok(Adt23::C0(f276(v2, v3), match v0.clone() {
        Adt23::C0(h1607, _, _, _, _, _, _, _, _, _) => {
            h1607
        }
    }, match v0.clone() {
        Adt23::C0(_, h1608, _, _, _, _, _, _, _, _) => {
            h1608
        }
    }, match v0.clone() {
        Adt23::C0(_, _, h1609, _, _, _, _, _, _, _) => {
            h1609
        }
    }, f276(match v0.clone() {
        Adt23::C0(_, _, _, h1610, _, _, _, _, _, _) => {
            h1610
        }
    }, v2), match v0.clone() {
        Adt23::C0(_, _, _, _, h1611, _, _, _, _, _) => {
            h1611
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, h1612, _, _, _, _) => {
            h1612
        }
    }, match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, h1613, _, _, _) => {
            h1613
        }
    }, f325(match v0.clone() {
        Adt23::C0(_, _, _, _, _, _, _, _, h1614, _) => {
            h1614
        }
    }), {
        let a1616 = match v0 {
            Adt23::C0(_, _, _, _, _, _, _, _, _, h1615) => {
                h1615
            }
        };
        let a1617 = 1u64;
        nat_add(a1616, a1617)?
    }))
}

pub fn f296(v0: u32) -> u64 {
    let m1619 = {
        let a1618 = v0;
        fixed_u64::convert(i128::from(a1618))
    };
    match m1619 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f297(v0: u64) -> u32 {
    let m1623 = {
        let a1622 = {
            let a1620 = v0;
            let a1621 = 4294967295u64;
            fixed_u64::bit_and(a1620, a1621)
        };
        fixed_u32::convert(i128::from(a1622))
    };
    match m1623 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f298(v0: u64) -> u32 {
    let (_, h1624) = {
        let (_, h1625) = f326(Fn20::F327, 32u64, (v0, (1u32, 0u32)));
        h1625
    };
    h1624
}

pub fn f299(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a1630 = f328(v0, 24u32);
    let a1631 = {
        let a1628 = f328(v1, 16u32);
        let a1629 = {
            let a1626 = f328(v2, 8u32);
            let a1627 = v3;
            fixed_u32::bit_or(a1626, a1627)
        };
        fixed_u32::bit_or(a1628, a1629)
    };
    fixed_u32::bit_or(a1630, a1631)
}

pub fn f300(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a1632 = v2;
    let a1633 = v1;
    if nat_lt(a1632, a1633) {
        let m1636 = {
            let a1634 = v0;
            let a1635 = v2;
            index_bytes(a1634, a1635)
        };
        match m1636 {
            None => {
                0u32
            }
            Some(v3) => {
                let m1638 = {
                    let a1637 = v3;
                    fixed_u32::convert(i128::from(a1637))
                };
                match m1638 {
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
        let a1639 = v2;
        let a1640 = v1;
        if nat_eq(a1639, a1640) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f301(v0: u8) -> bool {
    let a1641 = v0;
    let a1642 = 32u8;
    if if if equal(a1641, a1642) {
        true
    } else {
        let a1643 = v0;
        let a1644 = 9u8;
        equal(a1643, a1644)
    } {
        true
    } else {
        let a1645 = v0;
        let a1646 = 10u8;
        equal(a1645, a1646)
    } {
        true
    } else {
        let a1647 = v0;
        let a1648 = 12u8;
        if equal(a1647, a1648) {
            true
        } else {
            let a1649 = v0;
            let a1650 = 13u8;
            equal(a1649, a1650)
        }
    }
}

pub fn f302(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<Option<(u64, (Bytes, u64))>> {
    let v2: u64 = {
        let (h1651, _) = v1.clone();
        h1651
    };
    let v3: Bytes = {
        let (h1652, _) = {
            let (_, h1653) = v1.clone();
            h1653
        };
        h1652
    };
    let a1656 = {
        let (_, h1654) = {
            let (_, h1655) = v1;
            h1655
        };
        h1654
    };
    let a1657 = 0u64;
    if nat_eq(a1656, a1657) {
        let v4: u64 = f249(v0.clone(), v2)?;
        let a1658 = v4;
        let a1659 = 34u64;
        if nat_eq(a1658, a1659) {
            Ok(Some(({
                let a1660 = v2;
                let a1661 = 1u64;
                nat_add(a1660, a1661)?
            }, (v3, 1u64))))
        } else {
            let a1662 = v4;
            let a1663 = 92u64;
            if nat_eq(a1662, a1663) {
                f329(v0, v2, v3)
            } else {
                let a1664 = v4;
                let a1665 = 32u64;
                if if nat_lt(a1664, a1665) {
                    true
                } else {
                    let a1666 = v4;
                    let a1667 = 256u64;
                    nat_eq(a1666, a1667)
                } {
                    Ok(Some((v2, (v3, 2u64))))
                } else {
                    Ok(Some(({
                        let a1668 = v2;
                        let a1669 = 1u64;
                        nat_add(a1668, a1669)?
                    }, ({
                        let a1670 = v3;
                        let a1671 = f43(v0, v2, 1u64);
                        append_bytes(a1670, a1671)
                    }, 0u64))))
                }
            }
        }
    } else {
        Ok(None::<(u64, (Bytes, u64))>)
    }
}

pub fn f303(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f330(v0, v1)
}

pub fn f304(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1672 = v0.clone();
        length_bytes(a1672)
    };
    let a1673 = 0u64;
    let a1674 = v1;
    if nat_lt(a1673, a1674) {
        let a1675 = f331(v0.clone())?;
        let a1676 = v1;
        if nat_eq(a1675, a1676) {
            let a1677 = 1u64;
            let a1678 = v1;
            if nat_lt(a1677, a1678) {
                let m1681 = {
                    let a1679 = v0;
                    let a1680 = 0u64;
                    index_bytes(a1679, a1680)
                };
                match m1681 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1682 = v2;
                        let a1683 = 48u8;
                        Ok(!equal(a1682, a1683))
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

pub fn f305(v0: Fn16, p1688: u64, p1689: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1687 = (p1688, p1689);
    {
        let mut m1687 = m1687;
        loop {
            let step = {
                let (v1, v2) = m1687;
                let m1684 = v1;
                if m1684 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1684 - 1;
                    let m1686 = {
                        let c1685: Fn16 = v0.clone();
                        c1685.apply(v2)?
                    };
                    match m1686 {
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
                Ok(next) => { m1687 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f332(v0, v1)
}

pub fn f307(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f209(v0.clone(), 0u64, {
        let a1690 = v0;
        length_bytes(a1690)
    }, false, Bytes::lit(&[]))?;
    let m1693 = {
        let a1691 = v1.clone();
        let a1692 = 0u64;
        index_bytes(a1691, a1692)
    };
    match m1693 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v4: u64 = {
                let v3: u8 = v2;
                let a1738 = {
                    let a1714 = {
                        let a1702 = {
                            let a1696 = {
                                let a1694 = v3;
                                let a1695 = 128u8;
                                fixed_u8::bit_and(a1694, a1695)
                            };
                            let a1697 = 128u8;
                            if equal(a1696, a1697) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1703 = {
                            let a1700 = {
                                let a1698 = v3;
                                let a1699 = 64u8;
                                fixed_u8::bit_and(a1698, a1699)
                            };
                            let a1701 = 64u8;
                            if equal(a1700, a1701) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1702, a1703)?
                    };
                    let a1715 = {
                        let a1712 = {
                            let a1706 = {
                                let a1704 = v3;
                                let a1705 = 32u8;
                                fixed_u8::bit_and(a1704, a1705)
                            };
                            let a1707 = 32u8;
                            if equal(a1706, a1707) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1713 = {
                            let a1710 = {
                                let a1708 = v3;
                                let a1709 = 16u8;
                                fixed_u8::bit_and(a1708, a1709)
                            };
                            let a1711 = 16u8;
                            if equal(a1710, a1711) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1712, a1713)?
                    };
                    nat_add(a1714, a1715)?
                };
                let a1739 = {
                    let a1736 = {
                        let a1724 = {
                            let a1718 = {
                                let a1716 = v3;
                                let a1717 = 8u8;
                                fixed_u8::bit_and(a1716, a1717)
                            };
                            let a1719 = 8u8;
                            if equal(a1718, a1719) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1725 = {
                            let a1722 = {
                                let a1720 = v3;
                                let a1721 = 4u8;
                                fixed_u8::bit_and(a1720, a1721)
                            };
                            let a1723 = 4u8;
                            if equal(a1722, a1723) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1724, a1725)?
                    };
                    let a1737 = {
                        let a1734 = {
                            let a1728 = {
                                let a1726 = v3;
                                let a1727 = 2u8;
                                fixed_u8::bit_and(a1726, a1727)
                            };
                            let a1729 = 2u8;
                            if equal(a1728, a1729) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1735 = {
                            let a1732 = {
                                let a1730 = v3;
                                let a1731 = 1u8;
                                fixed_u8::bit_and(a1730, a1731)
                            };
                            let a1733 = 1u8;
                            if equal(a1732, a1733) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1734, a1735)?
                    };
                    nat_add(a1736, a1737)?
                };
                nat_add(a1738, a1739)?
            };
            if f255(v4) {
                let a1746 = f43({
                    let a1740 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a1740)
                }, {
                    let a1741 = v4;
                    let a1742 = 65u64;
                    nat_sub(a1741, a1742)
                }, 1u64);
                let a1747 = f43(v1.clone(), 1u64, {
                    let a1744 = {
                        let a1743 = v1;
                        length_bytes(a1743)
                    };
                    let a1745 = 1u64;
                    nat_sub(a1744, a1745)
                });
                Ok(append_bytes(a1746, a1747))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f308(v0: Bytes) -> R<bool> {
    let m1750 = {
        let a1748 = v0;
        let a1749 = 0u64;
        index_bytes(a1748, a1749)
    };
    match m1750 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            if f254({
                let v2: u8 = v1;
                let a1795 = {
                    let a1771 = {
                        let a1759 = {
                            let a1753 = {
                                let a1751 = v2;
                                let a1752 = 128u8;
                                fixed_u8::bit_and(a1751, a1752)
                            };
                            let a1754 = 128u8;
                            if equal(a1753, a1754) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a1760 = {
                            let a1757 = {
                                let a1755 = v2;
                                let a1756 = 64u8;
                                fixed_u8::bit_and(a1755, a1756)
                            };
                            let a1758 = 64u8;
                            if equal(a1757, a1758) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1759, a1760)?
                    };
                    let a1772 = {
                        let a1769 = {
                            let a1763 = {
                                let a1761 = v2;
                                let a1762 = 32u8;
                                fixed_u8::bit_and(a1761, a1762)
                            };
                            let a1764 = 32u8;
                            if equal(a1763, a1764) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a1770 = {
                            let a1767 = {
                                let a1765 = v2;
                                let a1766 = 16u8;
                                fixed_u8::bit_and(a1765, a1766)
                            };
                            let a1768 = 16u8;
                            if equal(a1767, a1768) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1769, a1770)?
                    };
                    nat_add(a1771, a1772)?
                };
                let a1796 = {
                    let a1793 = {
                        let a1781 = {
                            let a1775 = {
                                let a1773 = v2;
                                let a1774 = 8u8;
                                fixed_u8::bit_and(a1773, a1774)
                            };
                            let a1776 = 8u8;
                            if equal(a1775, a1776) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a1782 = {
                            let a1779 = {
                                let a1777 = v2;
                                let a1778 = 4u8;
                                fixed_u8::bit_and(a1777, a1778)
                            };
                            let a1780 = 4u8;
                            if equal(a1779, a1780) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1781, a1782)?
                    };
                    let a1794 = {
                        let a1791 = {
                            let a1785 = {
                                let a1783 = v2;
                                let a1784 = 2u8;
                                fixed_u8::bit_and(a1783, a1784)
                            };
                            let a1786 = 2u8;
                            if equal(a1785, a1786) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a1792 = {
                            let a1789 = {
                                let a1787 = v2;
                                let a1788 = 1u8;
                                fixed_u8::bit_and(a1787, a1788)
                            };
                            let a1790 = 1u8;
                            if equal(a1789, a1790) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a1791, a1792)?
                    };
                    nat_add(a1793, a1794)?
                };
                nat_add(a1795, a1796)?
            }) {
                Ok(true)
            } else {
                Ok(f255({
                    let v3: u8 = v1;
                    let a1841 = {
                        let a1817 = {
                            let a1805 = {
                                let a1799 = {
                                    let a1797 = v3;
                                    let a1798 = 128u8;
                                    fixed_u8::bit_and(a1797, a1798)
                                };
                                let a1800 = 128u8;
                                if equal(a1799, a1800) {
                                    128u64
                                } else {
                                    0u64
                                }
                            };
                            let a1806 = {
                                let a1803 = {
                                    let a1801 = v3;
                                    let a1802 = 64u8;
                                    fixed_u8::bit_and(a1801, a1802)
                                };
                                let a1804 = 64u8;
                                if equal(a1803, a1804) {
                                    64u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1805, a1806)?
                        };
                        let a1818 = {
                            let a1815 = {
                                let a1809 = {
                                    let a1807 = v3;
                                    let a1808 = 32u8;
                                    fixed_u8::bit_and(a1807, a1808)
                                };
                                let a1810 = 32u8;
                                if equal(a1809, a1810) {
                                    32u64
                                } else {
                                    0u64
                                }
                            };
                            let a1816 = {
                                let a1813 = {
                                    let a1811 = v3;
                                    let a1812 = 16u8;
                                    fixed_u8::bit_and(a1811, a1812)
                                };
                                let a1814 = 16u8;
                                if equal(a1813, a1814) {
                                    16u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1815, a1816)?
                        };
                        nat_add(a1817, a1818)?
                    };
                    let a1842 = {
                        let a1839 = {
                            let a1827 = {
                                let a1821 = {
                                    let a1819 = v3;
                                    let a1820 = 8u8;
                                    fixed_u8::bit_and(a1819, a1820)
                                };
                                let a1822 = 8u8;
                                if equal(a1821, a1822) {
                                    8u64
                                } else {
                                    0u64
                                }
                            };
                            let a1828 = {
                                let a1825 = {
                                    let a1823 = v3;
                                    let a1824 = 4u8;
                                    fixed_u8::bit_and(a1823, a1824)
                                };
                                let a1826 = 4u8;
                                if equal(a1825, a1826) {
                                    4u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1827, a1828)?
                        };
                        let a1840 = {
                            let a1837 = {
                                let a1831 = {
                                    let a1829 = v3;
                                    let a1830 = 2u8;
                                    fixed_u8::bit_and(a1829, a1830)
                                };
                                let a1832 = 2u8;
                                if equal(a1831, a1832) {
                                    2u64
                                } else {
                                    0u64
                                }
                            };
                            let a1838 = {
                                let a1835 = {
                                    let a1833 = v3;
                                    let a1834 = 1u8;
                                    fixed_u8::bit_and(a1833, a1834)
                                };
                                let a1836 = 1u8;
                                if equal(a1835, a1836) {
                                    1u64
                                } else {
                                    0u64
                                }
                            };
                            nat_add(a1837, a1838)?
                        };
                        nat_add(a1839, a1840)?
                    };
                    nat_add(a1841, a1842)?
                }))
            }
        }
    }
}

pub fn f309(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m1843 = f333(v0.clone());
    match m1843 {
        None => {
            if f26(f104(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m1844 = f101(v0, f8(Str::lit("items")));
                match m1844 {
                    None => {
                        Ok(f285(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f334(f8(Str::lit("List")), f335(v3, {
                            let a1845 = v1;
                            let a1846 = f8(Str::lit("Item"));
                            append_bytes(a1845, a1846)
                        })?))
                    }
                }
            } else {
                f335(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f334(f8(Str::lit("Option")), f335(v2, v1)?))
        }
    }
}

pub fn f310(v0: Bytes, v1: u64) -> Bytes {
    let m1850 = {
        let a1847 = v0;
        let a1848 = v1;
        let a1849 = 1u64;
        slice_bytes(a1847, a1848, a1849)
    };
    match m1850 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f311(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f336(v1, v0, v2)
}

pub fn f312(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a1895 = {
            let a1871 = {
                let a1859 = {
                    let a1853 = {
                        let a1851 = v3;
                        let a1852 = 128u8;
                        fixed_u8::bit_and(a1851, a1852)
                    };
                    let a1854 = 128u8;
                    if equal(a1853, a1854) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1860 = {
                    let a1857 = {
                        let a1855 = v3;
                        let a1856 = 64u8;
                        fixed_u8::bit_and(a1855, a1856)
                    };
                    let a1858 = 64u8;
                    if equal(a1857, a1858) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1859, a1860)?
            };
            let a1872 = {
                let a1869 = {
                    let a1863 = {
                        let a1861 = v3;
                        let a1862 = 32u8;
                        fixed_u8::bit_and(a1861, a1862)
                    };
                    let a1864 = 32u8;
                    if equal(a1863, a1864) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1870 = {
                    let a1867 = {
                        let a1865 = v3;
                        let a1866 = 16u8;
                        fixed_u8::bit_and(a1865, a1866)
                    };
                    let a1868 = 16u8;
                    if equal(a1867, a1868) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1869, a1870)?
            };
            nat_add(a1871, a1872)?
        };
        let a1896 = {
            let a1893 = {
                let a1881 = {
                    let a1875 = {
                        let a1873 = v3;
                        let a1874 = 8u8;
                        fixed_u8::bit_and(a1873, a1874)
                    };
                    let a1876 = 8u8;
                    if equal(a1875, a1876) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1882 = {
                    let a1879 = {
                        let a1877 = v3;
                        let a1878 = 4u8;
                        fixed_u8::bit_and(a1877, a1878)
                    };
                    let a1880 = 4u8;
                    if equal(a1879, a1880) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1881, a1882)?
            };
            let a1894 = {
                let a1891 = {
                    let a1885 = {
                        let a1883 = v3;
                        let a1884 = 2u8;
                        fixed_u8::bit_and(a1883, a1884)
                    };
                    let a1886 = 2u8;
                    if equal(a1885, a1886) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1892 = {
                    let a1889 = {
                        let a1887 = v3;
                        let a1888 = 1u8;
                        fixed_u8::bit_and(a1887, a1888)
                    };
                    let a1890 = 1u8;
                    if equal(a1889, a1890) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1891, a1892)?
            };
            nat_add(a1893, a1894)?
        };
        nat_add(a1895, a1896)?
    };
    let a1897 = v4;
    let a1898 = 34u64;
    if nat_eq(a1897, a1898) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a1899 = v4;
        let a1900 = 92u64;
        if nat_eq(a1899, a1900) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a1901 = v4;
            let a1902 = 8u64;
            if nat_eq(a1901, a1902) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a1903 = v4;
                let a1904 = 12u64;
                if nat_eq(a1903, a1904) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a1905 = v4;
                    let a1906 = 10u64;
                    if nat_eq(a1905, a1906) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a1907 = v4;
                        let a1908 = 13u64;
                        if nat_eq(a1907, a1908) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a1909 = v4;
                            let a1910 = 9u64;
                            if nat_eq(a1909, a1910) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a1911 = v4;
                                let a1912 = 32u64;
                                if nat_lt(a1911, a1912) {
                                    let a1913 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a1914 = f337(v2)?;
                                    Ok(append_bytes(a1913, a1914))
                                } else {
                                    Ok(f43(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f313(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f338(Fn21::F339, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f314(v0: Fn17, p1918: List<List<(Bytes, Bytes)>>, p1919: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m1917 = (p1918, p1919);
    {
        let mut m1917 = m1917;
        loop {
            let step = {
                let (v1, v2) = m1917;
                let m1915 = v2;
                match m1915.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c1916: Fn17 = v0.clone();
                            c1916.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1917 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f316(v0: Fn18, p1924: u64, p1925: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m1923 = (p1924, p1925);
    {
        let mut m1923 = m1923;
        loop {
            let step = {
                let (v1, v2) = m1923;
                let m1920 = v1;
                if m1920 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1920 - 1;
                    let m1922 = {
                        let c1921: Fn18 = v0.clone();
                        c1921.apply(v2.clone())?
                    };
                    match m1922 {
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
                Ok(next) => { m1923 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f340(v0)
}

pub fn f318(v0: Fn19, p1930: u64, p1931: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m1929 = (p1930, p1931);
    {
        let mut m1929 = m1929;
        loop {
            let step = {
                let (v1, v2) = m1929;
                let m1926 = v1;
                if m1926 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1926 - 1;
                    let m1928 = {
                        let c1927: Fn19 = v0.clone();
                        c1927.apply(v2.clone())?
                    };
                    match m1928 {
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
                Ok(next) => { m1929 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f341(v0)
}

pub fn f320(v0: u64) -> u32 {
    let a1932 = v0;
    let a1933 = 32u64;
    if nat_lt(a1932, a1933) {
        let a1934 = v0;
        let a1935 = 16u64;
        if nat_lt(a1934, a1935) {
            let a1936 = v0;
            let a1937 = 8u64;
            if nat_lt(a1936, a1937) {
                let a1938 = v0;
                let a1939 = 4u64;
                if nat_lt(a1938, a1939) {
                    let a1940 = v0;
                    let a1941 = 2u64;
                    if nat_lt(a1940, a1941) {
                        let a1942 = v0;
                        let a1943 = 1u64;
                        if nat_lt(a1942, a1943) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a1944 = v0;
                        let a1945 = 3u64;
                        if nat_lt(a1944, a1945) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a1946 = v0;
                    let a1947 = 6u64;
                    if nat_lt(a1946, a1947) {
                        let a1948 = v0;
                        let a1949 = 5u64;
                        if nat_lt(a1948, a1949) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a1950 = v0;
                        let a1951 = 7u64;
                        if nat_lt(a1950, a1951) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a1952 = v0;
                let a1953 = 12u64;
                if nat_lt(a1952, a1953) {
                    let a1954 = v0;
                    let a1955 = 10u64;
                    if nat_lt(a1954, a1955) {
                        let a1956 = v0;
                        let a1957 = 9u64;
                        if nat_lt(a1956, a1957) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a1958 = v0;
                        let a1959 = 11u64;
                        if nat_lt(a1958, a1959) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a1960 = v0;
                    let a1961 = 14u64;
                    if nat_lt(a1960, a1961) {
                        let a1962 = v0;
                        let a1963 = 13u64;
                        if nat_lt(a1962, a1963) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a1964 = v0;
                        let a1965 = 15u64;
                        if nat_lt(a1964, a1965) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a1966 = v0;
            let a1967 = 24u64;
            if nat_lt(a1966, a1967) {
                let a1968 = v0;
                let a1969 = 20u64;
                if nat_lt(a1968, a1969) {
                    let a1970 = v0;
                    let a1971 = 18u64;
                    if nat_lt(a1970, a1971) {
                        let a1972 = v0;
                        let a1973 = 17u64;
                        if nat_lt(a1972, a1973) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a1974 = v0;
                        let a1975 = 19u64;
                        if nat_lt(a1974, a1975) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a1976 = v0;
                    let a1977 = 22u64;
                    if nat_lt(a1976, a1977) {
                        let a1978 = v0;
                        let a1979 = 21u64;
                        if nat_lt(a1978, a1979) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a1980 = v0;
                        let a1981 = 23u64;
                        if nat_lt(a1980, a1981) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a1982 = v0;
                let a1983 = 28u64;
                if nat_lt(a1982, a1983) {
                    let a1984 = v0;
                    let a1985 = 26u64;
                    if nat_lt(a1984, a1985) {
                        let a1986 = v0;
                        let a1987 = 25u64;
                        if nat_lt(a1986, a1987) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a1988 = v0;
                        let a1989 = 27u64;
                        if nat_lt(a1988, a1989) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a1990 = v0;
                    let a1991 = 30u64;
                    if nat_lt(a1990, a1991) {
                        let a1992 = v0;
                        let a1993 = 29u64;
                        if nat_lt(a1992, a1993) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a1994 = v0;
                        let a1995 = 31u64;
                        if nat_lt(a1994, a1995) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a1996 = v0;
        let a1997 = 48u64;
        if nat_lt(a1996, a1997) {
            let a1998 = v0;
            let a1999 = 40u64;
            if nat_lt(a1998, a1999) {
                let a2000 = v0;
                let a2001 = 36u64;
                if nat_lt(a2000, a2001) {
                    let a2002 = v0;
                    let a2003 = 34u64;
                    if nat_lt(a2002, a2003) {
                        let a2004 = v0;
                        let a2005 = 33u64;
                        if nat_lt(a2004, a2005) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2006 = v0;
                        let a2007 = 35u64;
                        if nat_lt(a2006, a2007) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2008 = v0;
                    let a2009 = 38u64;
                    if nat_lt(a2008, a2009) {
                        let a2010 = v0;
                        let a2011 = 37u64;
                        if nat_lt(a2010, a2011) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2012 = v0;
                        let a2013 = 39u64;
                        if nat_lt(a2012, a2013) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2014 = v0;
                let a2015 = 44u64;
                if nat_lt(a2014, a2015) {
                    let a2016 = v0;
                    let a2017 = 42u64;
                    if nat_lt(a2016, a2017) {
                        let a2018 = v0;
                        let a2019 = 41u64;
                        if nat_lt(a2018, a2019) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2020 = v0;
                        let a2021 = 43u64;
                        if nat_lt(a2020, a2021) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2022 = v0;
                    let a2023 = 46u64;
                    if nat_lt(a2022, a2023) {
                        let a2024 = v0;
                        let a2025 = 45u64;
                        if nat_lt(a2024, a2025) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2026 = v0;
                        let a2027 = 47u64;
                        if nat_lt(a2026, a2027) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2028 = v0;
            let a2029 = 56u64;
            if nat_lt(a2028, a2029) {
                let a2030 = v0;
                let a2031 = 52u64;
                if nat_lt(a2030, a2031) {
                    let a2032 = v0;
                    let a2033 = 50u64;
                    if nat_lt(a2032, a2033) {
                        let a2034 = v0;
                        let a2035 = 49u64;
                        if nat_lt(a2034, a2035) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2036 = v0;
                        let a2037 = 51u64;
                        if nat_lt(a2036, a2037) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2038 = v0;
                    let a2039 = 54u64;
                    if nat_lt(a2038, a2039) {
                        let a2040 = v0;
                        let a2041 = 53u64;
                        if nat_lt(a2040, a2041) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2042 = v0;
                        let a2043 = 55u64;
                        if nat_lt(a2042, a2043) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2044 = v0;
                let a2045 = 60u64;
                if nat_lt(a2044, a2045) {
                    let a2046 = v0;
                    let a2047 = 58u64;
                    if nat_lt(a2046, a2047) {
                        let a2048 = v0;
                        let a2049 = 57u64;
                        if nat_lt(a2048, a2049) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2050 = v0;
                        let a2051 = 59u64;
                        if nat_lt(a2050, a2051) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2052 = v0;
                    let a2053 = 62u64;
                    if nat_lt(a2052, a2053) {
                        let a2054 = v0;
                        let a2055 = 61u64;
                        if nat_lt(a2054, a2055) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2056 = v0;
                        let a2057 = 63u64;
                        if nat_lt(a2056, a2057) {
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

pub fn f321(v0: u32) -> u32 {
    let a2060 = {
        let a2058 = f342(v0, 6u32, 26u32);
        let a2059 = f342(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2058, a2059)
    };
    let a2061 = f342(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2060, a2061)
}

pub fn f322(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2067 = {
        let a2062 = v0;
        let a2063 = v1;
        fixed_u32::bit_and(a2062, a2063)
    };
    let a2068 = {
        let a2065 = {
            let a2064 = v0;
            fixed_u32::bit_not(a2064)
        };
        let a2066 = v2;
        fixed_u32::bit_and(a2065, a2066)
    };
    fixed_u32::bit_xor(a2067, a2068)
}

pub fn f323(v0: u32) -> u32 {
    let a2071 = {
        let a2069 = f342(v0, 2u32, 30u32);
        let a2070 = f342(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2069, a2070)
    };
    let a2072 = f342(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2071, a2072)
}

pub fn f324(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2081 = {
        let a2077 = {
            let a2073 = v0;
            let a2074 = v1;
            fixed_u32::bit_and(a2073, a2074)
        };
        let a2078 = {
            let a2075 = v0;
            let a2076 = v2;
            fixed_u32::bit_and(a2075, a2076)
        };
        fixed_u32::bit_xor(a2077, a2078)
    };
    let a2082 = {
        let a2079 = v1;
        let a2080 = v2;
        fixed_u32::bit_and(a2079, a2080)
    };
    fixed_u32::bit_xor(a2081, a2082)
}

pub fn f325(v0: Adt22) -> Adt22 {
    Adt22::C0(match v0.clone() {
        Adt22::C0(_, h2083, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2083
        }
    }, match v0.clone() {
        Adt22::C0(_, _, h2084, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2084
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, h2085, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2085
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, h2086, _, _, _, _, _, _, _, _, _, _, _) => {
            h2086
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, h2087, _, _, _, _, _, _, _, _, _, _) => {
            h2087
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, h2088, _, _, _, _, _, _, _, _, _) => {
            h2088
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, h2089, _, _, _, _, _, _, _, _) => {
            h2089
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, h2090, _, _, _, _, _, _, _) => {
            h2090
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2091, _, _, _, _, _, _) => {
            h2091
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, h2092, _, _, _, _, _) => {
            h2092
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, h2093, _, _, _, _) => {
            h2093
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2094, _, _, _) => {
            h2094
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2095, _, _) => {
            h2095
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2096, _) => {
            h2096
        }
    }, match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2097) => {
            h2097
        }
    }, f276(f343(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2098, _) => {
            h2098
        }
    }), f276(match v0.clone() {
        Adt22::C0(_, _, _, _, _, _, _, _, _, h2099, _, _, _, _, _, _) => {
            h2099
        }
    }, f276(f344(match v0.clone() {
        Adt22::C0(_, h2100, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2100
        }
    }), match v0 {
        Adt22::C0(h2101, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2101
        }
    }))))
}

pub fn f326(v0: Fn20, p2105: u64, p2106: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2104 = (p2105, p2106);
    {
        let mut m2104 = m2104;
        loop {
            let step = {
                let (v1, v2) = m2104;
                let m2102 = v1;
                if m2102 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2102 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2103: Fn20 = v0.clone();
                        c2103.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2104 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2107, _) = v0;
        h2107
    };
    let v2: u32 = {
        let (h2108, _) = {
            let (_, h2109) = v0;
            h2109
        };
        h2108
    };
    let v3: u32 = {
        let (_, h2110) = {
            let (_, h2111) = v0;
            h2111
        };
        h2110
    };
    ({
        let a2112 = v1;
        let a2113 = 2u64;
        let a2114 = 0u64;
        nat_quot(a2112, a2113, a2114)
    }, (f328(v2, 1u32), {
        let a2118 = {
            let a2115 = v1;
            let a2116 = 2u64;
            let a2117 = 0u64;
            nat_rem(a2115, a2116, a2117)
        };
        let a2119 = 1u64;
        if nat_eq(a2118, a2119) {
            let a2120 = v3;
            let a2121 = v2;
            fixed_u32::bit_or(a2120, a2121)
        } else {
            v3
        }
    }))
}

pub fn f328(v0: u32, v1: u32) -> u32 {
    let m2124 = {
        let a2122 = v0;
        let a2123 = v1;
        fixed_u32::shift_left(a2122, a2123)
    };
    match m2124 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f329(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f249(v0.clone(), {
        let a2125 = v1;
        let a2126 = 1u64;
        nat_add(a2125, a2126)?
    })?;
    let v4: Bytes = {
        let a2127 = v3;
        let a2128 = 117u64;
        if nat_eq(a2127, a2128) {
            f345(v0, v1)?
        } else {
            f346(v3)
        }
    };
    let a2129 = v4.clone();
    let a2130 = Bytes::lit(&[]);
    if equal(a2129, a2130) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a2131 = v3;
            let a2132 = 117u64;
            if nat_eq(a2131, a2132) {
                let a2133 = v1;
                let a2134 = 6u64;
                nat_add(a2133, a2134)?
            } else {
                let a2135 = v1;
                let a2136 = 2u64;
                nat_add(a2135, a2136)?
            }
        }, ({
            let a2137 = v2;
            let a2138 = v4;
            append_bytes(a2137, a2138)
        }, 0u64))))
    }
}

pub fn f330(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f249(v0, v1)?;
    let a2139 = 47u64;
    let a2140 = v2;
    if if nat_lt(a2139, a2140) {
        let a2141 = v2;
        let a2142 = 58u64;
        nat_lt(a2141, a2142)
    } else {
        false
    } {
        Ok(Some({
            let a2143 = v1;
            let a2144 = 1u64;
            nat_add(a2143, a2144)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f331(v0: Bytes) -> R<u64> {
    let (h2145, _) = f245(Fn8::F347(v0.clone()), {
        let a2146 = v0;
        length_bytes(a2146)
    }, 0u64)?;
    Ok(h2145)
}

pub fn f332(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2150 = {
        let a2148 = v0;
        let a2149 = {
            let (h2147, _) = v1;
            h2147
        };
        index_bytes(a2148, a2149)
    };
    match m2150 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2152 = {
                    let (h2151, _) = v1;
                    h2151
                };
                let a2153 = 1u64;
                nat_add(a2152, a2153)?
            }, {
                let a2157 = {
                    let a2155 = {
                        let (_, h2154) = v1;
                        h2154
                    };
                    let a2156 = 10u64;
                    nat_mul(a2155, a2156)?
                };
                let a2158 = f348(v2)?;
                nat_add(a2157, a2158)?
            })))
        }
    }
}

pub fn f333(v0: Adt14) -> Option<Adt14> {
    let m2159 = f105(f101(v0, f8(Str::lit("oneOf"))));
    match m2159.uncons() {
        None => {
            None::<Adt14>
        }
        Some((v1, v2)) => {
            let m2160 = v2;
            match m2160.uncons() {
                None => {
                    None::<Adt14>
                }
                Some((v3, v4)) => {
                    let m2161 = v4;
                    match m2161.uncons() {
                        None => {
                            if f349(v3.clone()) {
                                Some(v1)
                            } else {
                                if f349(v1) {
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

pub fn f334(v0: Bytes, v1: Result<(Bytes, List<Adt21>), Bytes>) -> Result<(Bytes, List<Adt21>), Bytes> {
    let m2162 = v1;
    match m2162 {
        Ok(v3) => {
            f286(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons({
                let (h2163, _) = v3;
                h2163
            }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), {
                let (_, h2164) = v3;
                h2164
            })
        }
        Err(v2) => {
            Err::<(Bytes, List<Adt21>), Bytes>(v2)
        }
    }
}

pub fn f335(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m2165 = f333(v0.clone());
    match m2165 {
        None => {
            if f26(f104(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2166 = f101(v0, f8(Str::lit("items")));
                match m2166 {
                    None => {
                        Ok(f285(f8(Str::lit("array without items")), v1))
                    }
                    Some(v3) => {
                        Ok(f334(f8(Str::lit("List")), f350(v3, {
                            let a2167 = v1;
                            let a2168 = f8(Str::lit("Item"));
                            append_bytes(a2167, a2168)
                        })?))
                    }
                }
            } else {
                f350(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f334(f8(Str::lit("Option")), f350(v2, v1)?))
        }
    }
}

pub fn f336(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2169, _) = v2;
        h2169
    };
    let a2170 = v3;
    let a2171 = 0u64;
    if nat_eq(a2170, a2171) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2172 = v3;
            let a2173 = v1;
            let a2174 = 0u64;
            nat_quot(a2172, a2173, a2174)
        }, {
            let a2179 = f310(v0, {
                let a2175 = v3;
                let a2176 = v1;
                let a2177 = 0u64;
                nat_rem(a2175, a2176, a2177)
            });
            let a2180 = {
                let (_, h2178) = v2;
                h2178
            };
            append_bytes(a2179, a2180)
        }))
    }
}

pub fn f337(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2225 = {
            let a2201 = {
                let a2189 = {
                    let a2183 = {
                        let a2181 = v1;
                        let a2182 = 128u8;
                        fixed_u8::bit_and(a2181, a2182)
                    };
                    let a2184 = 128u8;
                    if equal(a2183, a2184) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2190 = {
                    let a2187 = {
                        let a2185 = v1;
                        let a2186 = 64u8;
                        fixed_u8::bit_and(a2185, a2186)
                    };
                    let a2188 = 64u8;
                    if equal(a2187, a2188) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2189, a2190)?
            };
            let a2202 = {
                let a2199 = {
                    let a2193 = {
                        let a2191 = v1;
                        let a2192 = 32u8;
                        fixed_u8::bit_and(a2191, a2192)
                    };
                    let a2194 = 32u8;
                    if equal(a2193, a2194) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2200 = {
                    let a2197 = {
                        let a2195 = v1;
                        let a2196 = 16u8;
                        fixed_u8::bit_and(a2195, a2196)
                    };
                    let a2198 = 16u8;
                    if equal(a2197, a2198) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2199, a2200)?
            };
            nat_add(a2201, a2202)?
        };
        let a2226 = {
            let a2223 = {
                let a2211 = {
                    let a2205 = {
                        let a2203 = v1;
                        let a2204 = 8u8;
                        fixed_u8::bit_and(a2203, a2204)
                    };
                    let a2206 = 8u8;
                    if equal(a2205, a2206) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2212 = {
                    let a2209 = {
                        let a2207 = v1;
                        let a2208 = 4u8;
                        fixed_u8::bit_and(a2207, a2208)
                    };
                    let a2210 = 4u8;
                    if equal(a2209, a2210) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2211, a2212)?
            };
            let a2224 = {
                let a2221 = {
                    let a2215 = {
                        let a2213 = v1;
                        let a2214 = 2u8;
                        fixed_u8::bit_and(a2213, a2214)
                    };
                    let a2216 = 2u8;
                    if equal(a2215, a2216) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2222 = {
                    let a2219 = {
                        let a2217 = v1;
                        let a2218 = 1u8;
                        fixed_u8::bit_and(a2217, a2218)
                    };
                    let a2220 = 1u8;
                    if equal(a2219, a2220) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2221, a2222)?
            };
            nat_add(a2223, a2224)?
        };
        nat_add(a2225, a2226)?
    };
    let a2233 = f310(f351(), {
        let a2227 = v2;
        let a2228 = 16u64;
        let a2229 = 0u64;
        nat_quot(a2227, a2228, a2229)
    });
    let a2234 = f310(f351(), {
        let a2230 = v2;
        let a2231 = 16u64;
        let a2232 = 0u64;
        nat_rem(a2230, a2231, a2232)
    });
    Ok(append_bytes(a2233, a2234))
}

pub fn f338(v0: Fn21, p2238: List<List<(Bytes, Bytes)>>, p2239: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2237 = (p2238, p2239);
    {
        let mut m2237 = m2237;
        loop {
            let step = {
                let (v1, v2) = m2237;
                let m2235 = v2;
                match m2235.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2236: Fn21 = v0.clone();
                            c2236.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f340(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2241 = {
        let a2240 = v0.clone();
        length_list(a2240)
    };
    let a2242 = 1u64;
    if nat_le(a2241, a2242) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f352(v0)?))
    }
}

pub fn f341(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2243, _) = v0;
        h2243
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2244) = v0;
        h2244
    };
    let m2245 = v1;
    match m2245.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2246 = v4;
            match m2246.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f353(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f342(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2247 = f242(v0, v1);
    let a2248 = f328(v0, v2);
    fixed_u32::bit_or(a2247, a2248)
}

pub fn f343(v0: u32) -> u32 {
    let a2251 = {
        let a2249 = f342(v0, 17u32, 15u32);
        let a2250 = f342(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2249, a2250)
    };
    let a2252 = f242(v0, 10u32);
    fixed_u32::bit_xor(a2251, a2252)
}

pub fn f344(v0: u32) -> u32 {
    let a2255 = {
        let a2253 = f342(v0, 7u32, 25u32);
        let a2254 = f342(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2253, a2254)
    };
    let a2256 = f242(v0, 3u32);
    fixed_u32::bit_xor(a2255, a2256)
}

pub fn f345(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2259 = f43(v0.clone(), {
        let a2257 = v1;
        let a2258 = 2u64;
        nat_add(a2257, a2258)?
    }, 2u64);
    let a2260 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2259, a2260) {
        let m2263 = f354(f43(v0, {
            let a2261 = v1;
            let a2262 = 4u64;
            nat_add(a2261, a2262)?
        }, 2u64))?;
        match m2263 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f249(v2.clone(), 0u64)?;
                let a2264 = v3;
                let a2265 = 32u64;
                if nat_lt(a2264, a2265) {
                    if f355(v3) {
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

pub fn f346(v0: u64) -> Bytes {
    let a2266 = v0;
    let a2267 = 34u64;
    if nat_eq(a2266, a2267) {
        Bytes::lit(&[0x22])
    } else {
        let a2268 = v0;
        let a2269 = 92u64;
        if nat_eq(a2268, a2269) {
            Bytes::lit(&[0x5c])
        } else {
            let a2270 = v0;
            let a2271 = 98u64;
            if nat_eq(a2270, a2271) {
                Bytes::lit(&[0x08])
            } else {
                let a2272 = v0;
                let a2273 = 102u64;
                if nat_eq(a2272, a2273) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2274 = v0;
                    let a2275 = 110u64;
                    if nat_eq(a2274, a2275) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2276 = v0;
                        let a2277 = 114u64;
                        if nat_eq(a2276, a2277) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2278 = v0;
                            let a2279 = 116u64;
                            if nat_eq(a2278, a2279) {
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

pub fn f347(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2282 = {
        let a2280 = v0;
        let a2281 = v1;
        index_bytes(a2280, a2281)
    };
    match m2282 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f356(v2)? {
                Ok(Some({
                    let a2283 = v1;
                    let a2284 = 1u64;
                    nat_add(a2283, a2284)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f348(v0: u8) -> R<u64> {
    let a2331 = {
        let v1: u8 = v0;
        let a2329 = {
            let a2305 = {
                let a2293 = {
                    let a2287 = {
                        let a2285 = v1;
                        let a2286 = 128u8;
                        fixed_u8::bit_and(a2285, a2286)
                    };
                    let a2288 = 128u8;
                    if equal(a2287, a2288) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2294 = {
                    let a2291 = {
                        let a2289 = v1;
                        let a2290 = 64u8;
                        fixed_u8::bit_and(a2289, a2290)
                    };
                    let a2292 = 64u8;
                    if equal(a2291, a2292) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2293, a2294)?
            };
            let a2306 = {
                let a2303 = {
                    let a2297 = {
                        let a2295 = v1;
                        let a2296 = 32u8;
                        fixed_u8::bit_and(a2295, a2296)
                    };
                    let a2298 = 32u8;
                    if equal(a2297, a2298) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2304 = {
                    let a2301 = {
                        let a2299 = v1;
                        let a2300 = 16u8;
                        fixed_u8::bit_and(a2299, a2300)
                    };
                    let a2302 = 16u8;
                    if equal(a2301, a2302) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2303, a2304)?
            };
            nat_add(a2305, a2306)?
        };
        let a2330 = {
            let a2327 = {
                let a2315 = {
                    let a2309 = {
                        let a2307 = v1;
                        let a2308 = 8u8;
                        fixed_u8::bit_and(a2307, a2308)
                    };
                    let a2310 = 8u8;
                    if equal(a2309, a2310) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2316 = {
                    let a2313 = {
                        let a2311 = v1;
                        let a2312 = 4u8;
                        fixed_u8::bit_and(a2311, a2312)
                    };
                    let a2314 = 4u8;
                    if equal(a2313, a2314) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2315, a2316)?
            };
            let a2328 = {
                let a2325 = {
                    let a2319 = {
                        let a2317 = v1;
                        let a2318 = 2u8;
                        fixed_u8::bit_and(a2317, a2318)
                    };
                    let a2320 = 2u8;
                    if equal(a2319, a2320) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2326 = {
                    let a2323 = {
                        let a2321 = v1;
                        let a2322 = 1u8;
                        fixed_u8::bit_and(a2321, a2322)
                    };
                    let a2324 = 1u8;
                    if equal(a2323, a2324) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2325, a2326)?
            };
            nat_add(a2327, a2328)?
        };
        nat_add(a2329, a2330)?
    };
    let a2332 = 48u64;
    Ok(nat_sub(a2331, a2332))
}

pub fn f349(v0: Adt14) -> bool {
    f26(f104(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f350(v0: Adt14, v1: Bytes) -> R<Result<(Bytes, List<Adt21>), Bytes>> {
    let m2333 = f357(f258(v0.clone()));
    match m2333 {
        None => {
            let m2334 = f109(f101(v0.clone(), f8(Str::lit("$ref"))));
            match m2334 {
                None => {
                    let m2336 = f101(v0.clone(), f8(Str::lit("enum")));
                    match m2336 {
                        None => {
                            let m2337 = f101(v0.clone(), f8(Str::lit("const")));
                            match m2337 {
                                None => {
                                    let v8: Bytes = f104(v0.clone(), f8(Str::lit("type")));
                                    if f26(v8.clone(), f8(Str::lit("string"))) {
                                        Ok(f286(f8(Str::lit("Bytes")), List::<Adt21>::nil()))
                                    } else {
                                        if f26(v8.clone(), f8(Str::lit("integer"))) {
                                            Ok(f286(f359(v0), List::<Adt21>::nil()))
                                        } else {
                                            if f26(v8.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f286(f8(Str::lit("Bool")), List::<Adt21>::nil()))
                                            } else {
                                                if f26(v8.clone(), f8(Str::lit("object"))) {
                                                    Ok(f286(v1.clone(), List::cons(Adt21::C0(v1, v0), List::<Adt21>::nil())))
                                                } else {
                                                    Ok(f285(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v8, List::<Bytes>::nil()))), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v6) => {
                                    let m2338 = f109(Some(v6));
                                    match m2338 {
                                        None => {
                                            Ok(f285(f8(Str::lit("unsupported const")), v1))
                                        }
                                        Some(_) => {
                                            Ok(f286(f8(Str::lit("Bytes")), List::<Adt21>::nil()))
                                        }
                                    }
                                }
                            }
                        }
                        Some(_) => {
                            Ok(f286(v1.clone(), List::cons(Adt21::C0(v1, v0), List::<Adt21>::nil())))
                        }
                    }
                }
                Some(v3) => {
                    let m2335 = f358(v3)?;
                    match m2335 {
                        None => {
                            Ok(f285(f8(Str::lit("unsupported reference")), v1))
                        }
                        Some(v4) => {
                            Ok(f286(v4, List::<Adt21>::nil()))
                        }
                    }
                }
            }
        }
        Some(v2) => {
            Ok(f285(f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v2, List::<Bytes>::nil()))), v1))
        }
    }
}

pub fn f351() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f352(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f313({
        let (_, h2339) = {
            let (h2340, _) = f360(Fn22::F361, {
                let a2342 = {
                    let a2341 = v0.clone();
                    length_list(a2341)
                };
                let a2343 = 1u64;
                nat_add(a2342, a2343)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2340
        };
        h2339
    }))
}

pub fn f353(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a2348 = {
            let a2346 = {
                let a2344 = v0.clone();
                length_list(a2344)
            };
            let a2347 = {
                let a2345 = v1.clone();
                length_list(a2345)
            };
            nat_add(a2346, a2347)?
        };
        let a2349 = 1u64;
        nat_add(a2348, a2349)?
    };
    Ok(f237({
        let (_, h2350) = {
            let (_, h2351) = {
                let (h2352, _) = f362(Fn23::F363, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h2352
            };
            h2351
        };
        h2350
    }))
}

pub fn f354(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2353 = v0.clone();
        length_bytes(a2353)
    };
    let a2357 = {
        let a2354 = v1;
        let a2355 = 2u64;
        let a2356 = 0u64;
        nat_rem(a2354, a2355, a2356)
    };
    let a2358 = 0u64;
    if nat_eq(a2357, a2358) {
        let (_, h2359) = {
            let (h2360, _) = f364(Fn24::F365(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2360
        };
        Ok(h2359)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f355(v0: u64) -> bool {
    let a2361 = v0;
    let a2362 = 8u64;
    if nat_eq(a2361, a2362) {
        true
    } else {
        let a2363 = v0;
        let a2364 = 9u64;
        if nat_eq(a2363, a2364) {
            true
        } else {
            let a2365 = v0;
            let a2366 = 10u64;
            if nat_eq(a2365, a2366) {
                true
            } else {
                let a2367 = v0;
                let a2368 = 12u64;
                if nat_eq(a2367, a2368) {
                    true
                } else {
                    let a2369 = v0;
                    let a2370 = 13u64;
                    nat_eq(a2369, a2370)
                }
            }
        }
    }
}

pub fn f356(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2415 = {
            let a2391 = {
                let a2379 = {
                    let a2373 = {
                        let a2371 = v1;
                        let a2372 = 128u8;
                        fixed_u8::bit_and(a2371, a2372)
                    };
                    let a2374 = 128u8;
                    if equal(a2373, a2374) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2380 = {
                    let a2377 = {
                        let a2375 = v1;
                        let a2376 = 64u8;
                        fixed_u8::bit_and(a2375, a2376)
                    };
                    let a2378 = 64u8;
                    if equal(a2377, a2378) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2379, a2380)?
            };
            let a2392 = {
                let a2389 = {
                    let a2383 = {
                        let a2381 = v1;
                        let a2382 = 32u8;
                        fixed_u8::bit_and(a2381, a2382)
                    };
                    let a2384 = 32u8;
                    if equal(a2383, a2384) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2390 = {
                    let a2387 = {
                        let a2385 = v1;
                        let a2386 = 16u8;
                        fixed_u8::bit_and(a2385, a2386)
                    };
                    let a2388 = 16u8;
                    if equal(a2387, a2388) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2389, a2390)?
            };
            nat_add(a2391, a2392)?
        };
        let a2416 = {
            let a2413 = {
                let a2401 = {
                    let a2395 = {
                        let a2393 = v1;
                        let a2394 = 8u8;
                        fixed_u8::bit_and(a2393, a2394)
                    };
                    let a2396 = 8u8;
                    if equal(a2395, a2396) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2402 = {
                    let a2399 = {
                        let a2397 = v1;
                        let a2398 = 4u8;
                        fixed_u8::bit_and(a2397, a2398)
                    };
                    let a2400 = 4u8;
                    if equal(a2399, a2400) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2401, a2402)?
            };
            let a2414 = {
                let a2411 = {
                    let a2405 = {
                        let a2403 = v1;
                        let a2404 = 2u8;
                        fixed_u8::bit_and(a2403, a2404)
                    };
                    let a2406 = 2u8;
                    if equal(a2405, a2406) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2412 = {
                    let a2409 = {
                        let a2407 = v1;
                        let a2408 = 1u8;
                        fixed_u8::bit_and(a2407, a2408)
                    };
                    let a2410 = 1u8;
                    if equal(a2409, a2410) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2411, a2412)?
            };
            nat_add(a2413, a2414)?
        };
        nat_add(a2415, a2416)?
    };
    let a2417 = 48u64;
    let a2418 = v2;
    if nat_le(a2417, a2418) {
        let a2419 = v2;
        let a2420 = 57u64;
        Ok(nat_le(a2419, a2420))
    } else {
        Ok(false)
    }
}

pub fn f357(p2426: List<(Bytes, Adt14)>) -> Option<Bytes> {
    let m2425 = p2426;
    {
        let mut m2425 = m2425;
        loop {
            let step = {
                let v0 = m2425;
                let m2421 = v0;
                match m2421.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if if f67({
                            let (h2422, _) = v1.clone();
                            h2422
                        }, f366()) {
                            true
                        } else {
                            f67({
                                let (h2423, _) = v1.clone();
                                h2423
                            }, f367())
                        } {
                            Ok::<List<(Bytes, Adt14)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt14)>, Option<Bytes>>(Some({
                                let (h2424, _) = v1;
                                h2424
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2425 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: Bytes) -> R<Option<Bytes>> {
    if f229(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f160(f43(v0.clone(), 8u64, {
            let a2428 = {
                let a2427 = v0;
                length_bytes(a2427)
            };
            let a2429 = 8u64;
            nat_sub(a2428, a2429)
        }))?))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f359(v0: Adt14) -> Bytes {
    let m2430 = f228(f101(v0, f8(Str::lit("minimum"))));
    match m2430 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f360(v0: Fn22, p2435: u64, p2436: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m2434 = (p2435, p2436);
    {
        let mut m2434 = m2434;
        loop {
            let step = {
                let (v1, v2) = m2434;
                let m2431 = v1;
                if m2431 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2431 - 1;
                    let m2433 = {
                        let c2432: Fn22 = v0.clone();
                        c2432.apply(v2.clone())?
                    };
                    match m2433 {
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
                Ok(next) => { m2434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f368(v0)
}

pub fn f362(v0: Fn23, p2441: u64, p2442: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m2440 = (p2441, p2442);
    {
        let mut m2440 = m2440;
        loop {
            let step = {
                let (v1, v2) = m2440;
                let m2437 = v1;
                if m2437 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m2437 - 1;
                    let m2439 = {
                        let c2438: Fn23 = v0.clone();
                        c2438.apply(v2.clone())
                    };
                    match m2439 {
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
                Ok(next) => { m2440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f369(v0)
}

pub fn f364(v0: Fn24, p2447: u64, p2448: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2446 = (p2447, p2448);
    {
        let mut m2446 = m2446;
        loop {
            let step = {
                let (v1, v2) = m2446;
                let m2443 = v1;
                if m2443 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2443 - 1;
                    let m2445 = {
                        let c2444: Fn24 = v0.clone();
                        c2444.apply(v2.clone())?
                    };
                    match m2445 {
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
                Ok(next) => { m2446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f365(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f370(v0, v1)
}

pub fn f366() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::<Bytes>::nil()))))))))))))))
}

pub fn f367() -> List<Bytes> {
    List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("uniqueItems")), List::cons(f8(Str::lit("format")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("exclusiveMinimum")), List::cons(f8(Str::lit("exclusiveMaximum")), List::cons(f8(Str::lit("multipleOf")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::<Bytes>::nil())))))))))))))))
}

pub fn f368(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h2449, _) = v0;
        h2449
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h2450) = v0;
        h2450
    };
    let m2451 = v1;
    match m2451.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m2452 = v4;
            match m2452.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f371(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f369(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h2453, _) = v0.clone();
        h2453
    };
    let v2: List<(Bytes, ())> = {
        let (h2454, _) = {
            let (_, h2455) = v0.clone();
            h2455
        };
        h2454
    };
    let v3: List<(Bytes, ())> = {
        let (_, h2456) = {
            let (_, h2457) = v0;
            h2457
        };
        h2456
    };
    let m2458 = v1.clone();
    match m2458.uncons() {
        None => {
            let m2459 = v2;
            match m2459.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2460 = v2.clone();
            match m2460.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f372({
                        let (h2461, _) = v8.clone();
                        h2461
                    }, {
                        let (h2462, _) = v6.clone();
                        h2462
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

pub fn f370(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2463, _) = v1;
        h2463
    };
    let m2465 = {
        let (_, h2464) = v1;
        h2464
    };
    match m2465 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2468 = {
                let a2466 = v0.clone();
                let a2467 = v2;
                index_bytes(a2466, a2467)
            };
            match m2468 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2473 = {
                        let a2471 = v0;
                        let a2472 = {
                            let a2469 = v2;
                            let a2470 = 1u64;
                            nat_add(a2469, a2470)?
                        };
                        index_bytes(a2471, a2472)
                    };
                    match m2473 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f373(v4)? {
                                f373(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2474 = v2;
                                    let a2475 = 2u64;
                                    nat_add(a2474, a2475)?
                                }, Some({
                                    let a2480 = v3;
                                    let a2481 = f310(f374(), {
                                        let a2478 = {
                                            let a2476 = f375(v4)?;
                                            let a2477 = 16u64;
                                            nat_mul(a2476, a2477)?
                                        };
                                        let a2479 = f375(v5)?;
                                        nat_add(a2478, a2479)?
                                    });
                                    append_bytes(a2480, a2481)
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

pub fn f371(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a2486 = {
            let a2484 = {
                let a2482 = v0.clone();
                length_list(a2482)
            };
            let a2485 = {
                let a2483 = v1.clone();
                length_list(a2483)
            };
            nat_add(a2484, a2485)?
        };
        let a2487 = 1u64;
        nat_add(a2486, a2487)?
    };
    Ok(f376({
        let (_, h2488) = {
            let (_, h2489) = {
                let (h2490, _) = f377(Fn25::F378, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h2490
            };
            h2489
        };
        h2488
    }))
}

pub fn f372(v0: Bytes, v1: Bytes) -> bool {
    let a2495 = {
        let a2491 = v0;
        let a2492 = v1;
        compare_bytes(a2491, a2492)
    };
    let a2496 = {
        let a2493 = Bytes::lit(&[]);
        let a2494 = Bytes::lit(&[0x01]);
        compare_bytes(a2493, a2494)
    };
    equal(a2495, a2496)
}

pub fn f373(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2541 = {
            let a2517 = {
                let a2505 = {
                    let a2499 = {
                        let a2497 = v1;
                        let a2498 = 128u8;
                        fixed_u8::bit_and(a2497, a2498)
                    };
                    let a2500 = 128u8;
                    if equal(a2499, a2500) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2506 = {
                    let a2503 = {
                        let a2501 = v1;
                        let a2502 = 64u8;
                        fixed_u8::bit_and(a2501, a2502)
                    };
                    let a2504 = 64u8;
                    if equal(a2503, a2504) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2505, a2506)?
            };
            let a2518 = {
                let a2515 = {
                    let a2509 = {
                        let a2507 = v1;
                        let a2508 = 32u8;
                        fixed_u8::bit_and(a2507, a2508)
                    };
                    let a2510 = 32u8;
                    if equal(a2509, a2510) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2516 = {
                    let a2513 = {
                        let a2511 = v1;
                        let a2512 = 16u8;
                        fixed_u8::bit_and(a2511, a2512)
                    };
                    let a2514 = 16u8;
                    if equal(a2513, a2514) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2515, a2516)?
            };
            nat_add(a2517, a2518)?
        };
        let a2542 = {
            let a2539 = {
                let a2527 = {
                    let a2521 = {
                        let a2519 = v1;
                        let a2520 = 8u8;
                        fixed_u8::bit_and(a2519, a2520)
                    };
                    let a2522 = 8u8;
                    if equal(a2521, a2522) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2528 = {
                    let a2525 = {
                        let a2523 = v1;
                        let a2524 = 4u8;
                        fixed_u8::bit_and(a2523, a2524)
                    };
                    let a2526 = 4u8;
                    if equal(a2525, a2526) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2527, a2528)?
            };
            let a2540 = {
                let a2537 = {
                    let a2531 = {
                        let a2529 = v1;
                        let a2530 = 2u8;
                        fixed_u8::bit_and(a2529, a2530)
                    };
                    let a2532 = 2u8;
                    if equal(a2531, a2532) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2538 = {
                    let a2535 = {
                        let a2533 = v1;
                        let a2534 = 1u8;
                        fixed_u8::bit_and(a2533, a2534)
                    };
                    let a2536 = 1u8;
                    if equal(a2535, a2536) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2537, a2538)?
            };
            nat_add(a2539, a2540)?
        };
        nat_add(a2541, a2542)?
    };
    let a2543 = 48u64;
    let a2544 = v2;
    if if nat_le(a2543, a2544) {
        let a2545 = v2;
        let a2546 = 57u64;
        nat_le(a2545, a2546)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2547 = 97u64;
        let a2548 = v2;
        if nat_le(a2547, a2548) {
            let a2549 = v2;
            let a2550 = 102u64;
            Ok(nat_le(a2549, a2550))
        } else {
            Ok(false)
        }
    }
}

pub fn f374() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f375(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2595 = {
            let a2571 = {
                let a2559 = {
                    let a2553 = {
                        let a2551 = v1;
                        let a2552 = 128u8;
                        fixed_u8::bit_and(a2551, a2552)
                    };
                    let a2554 = 128u8;
                    if equal(a2553, a2554) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2560 = {
                    let a2557 = {
                        let a2555 = v1;
                        let a2556 = 64u8;
                        fixed_u8::bit_and(a2555, a2556)
                    };
                    let a2558 = 64u8;
                    if equal(a2557, a2558) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2559, a2560)?
            };
            let a2572 = {
                let a2569 = {
                    let a2563 = {
                        let a2561 = v1;
                        let a2562 = 32u8;
                        fixed_u8::bit_and(a2561, a2562)
                    };
                    let a2564 = 32u8;
                    if equal(a2563, a2564) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2570 = {
                    let a2567 = {
                        let a2565 = v1;
                        let a2566 = 16u8;
                        fixed_u8::bit_and(a2565, a2566)
                    };
                    let a2568 = 16u8;
                    if equal(a2567, a2568) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2569, a2570)?
            };
            nat_add(a2571, a2572)?
        };
        let a2596 = {
            let a2593 = {
                let a2581 = {
                    let a2575 = {
                        let a2573 = v1;
                        let a2574 = 8u8;
                        fixed_u8::bit_and(a2573, a2574)
                    };
                    let a2576 = 8u8;
                    if equal(a2575, a2576) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2582 = {
                    let a2579 = {
                        let a2577 = v1;
                        let a2578 = 4u8;
                        fixed_u8::bit_and(a2577, a2578)
                    };
                    let a2580 = 4u8;
                    if equal(a2579, a2580) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2581, a2582)?
            };
            let a2594 = {
                let a2591 = {
                    let a2585 = {
                        let a2583 = v1;
                        let a2584 = 2u8;
                        fixed_u8::bit_and(a2583, a2584)
                    };
                    let a2586 = 2u8;
                    if equal(a2585, a2586) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2592 = {
                    let a2589 = {
                        let a2587 = v1;
                        let a2588 = 1u8;
                        fixed_u8::bit_and(a2587, a2588)
                    };
                    let a2590 = 1u8;
                    if equal(a2589, a2590) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2591, a2592)?
            };
            nat_add(a2593, a2594)?
        };
        nat_add(a2595, a2596)?
    };
    let a2597 = v2;
    let a2598 = 57u64;
    if nat_le(a2597, a2598) {
        let a2599 = v2;
        let a2600 = 48u64;
        Ok(nat_sub(a2599, a2600))
    } else {
        let a2601 = v2;
        let a2602 = 87u64;
        Ok(nat_sub(a2601, a2602))
    }
}

pub fn f376(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f264(Fn10::F379, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f377(v0: Fn25, p2607: u64, p2608: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m2606 = (p2607, p2608);
    {
        let mut m2606 = m2606;
        loop {
            let step = {
                let (v1, v2) = m2606;
                let m2603 = v1;
                if m2603 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m2603 - 1;
                    let m2605 = {
                        let c2604: Fn25 = v0.clone();
                        c2604.apply(v2.clone())
                    };
                    match m2605 {
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
                Ok(next) => { m2606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f380(v0)
}

pub fn f379(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f380(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h2609, _) = v0.clone();
        h2609
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h2610, _) = {
            let (_, h2611) = v0.clone();
            h2611
        };
        h2610
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h2612) = {
            let (_, h2613) = v0;
            h2613
        };
        h2612
    };
    let m2614 = v1.clone();
    match m2614.uncons() {
        None => {
            let m2615 = v2;
            match m2615.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m2616 = v2.clone();
            match m2616.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f372({
                        let (h2617, _) = v8.clone();
                        h2617
                    }, {
                        let (h2618, _) = v6.clone();
                        h2618
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
